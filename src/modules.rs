#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{error, results::Results};
pub(crate) use pyo3::{
    prelude::{PyModule, PyResult, Python},
    types::PyTuple,
};
use serde_derive::{Deserialize, Serialize};
use std::fs;

/// Module structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub file_path: String,
    pub name: String,
    pub desc: String,
    pub author: String,
    pub target_types: Vec<String>,
}

impl Module {
    /// Create an empty module
    pub fn create(&self, name: &str, desc: &str, author: &str, target_types: Vec<String>) {
        let code = format!(
            r"
import json


class ModuleInfos:
    name = '{}'
    desc = '{}'
    author = '{}'
    target_types = {:?}

    def to_json(self) -> str:
        return json.dumps({{
            'name': self.name,
            'desc': self.desc,
            'author': self.author,
            'target_types': self.target_types
        }})


def execute(target: str):
    ...


if __name__ == '__main__':
    import sys
    execute(sys.argv[1])

        ",
            name, desc, author, target_types
        );
    }

    /// Run the module
    pub fn execute(&self, target: &str) {
        if Python::with_gil(|py| -> PyResult<()> {
            let content = fs::read_to_string(format!("{}.py", self.file_path))?;
            let module = PyModule::from_code(
                py,
                &content,
                &format!("{}.py", self.file_path),
                &self.file_path,
            )?;

            let output = module
                .getattr("execute")? // Grab the execute() function
                .call1(PyTuple::new(py, &[target]))? // Call it with the target as arguments
                .to_string(); // Grab the output JSON data as string

            println!("{}", Results::load(output).render(&self.name)); // Parse the output and display it

            Ok(())
        })
        .is_err()
        {
            error!(format!("Failed to run module: {}", self.name));
        }
    }
}

/// List of modules paths
#[derive(Debug, Serialize, Deserialize)]
struct ModulesList {
    modules: Vec<String>,
}

impl ModulesList {
    /// Load modules list from 'modules.json'
    pub fn load() -> Self {
        let content = fs::read_to_string("modules.json").unwrap();
        let data: ModulesList = serde_json::from_str(&content).unwrap();

        data
    }
}

/// Modules registery
#[derive(Debug, Serialize, Deserialize)]
pub struct Modules {
    modules: Vec<Module>,
}

impl Modules {
    /// Load all the modules from './modules'
    pub fn load() -> Self {
        let mut modules: Vec<Module> = Vec::new();

        for file_path in ModulesList::load().modules {
            let module = Python::with_gil(|py| -> PyResult<Module> {
                let content = fs::read_to_string(format!("{}.py", file_path))?;

                let module =
                    PyModule::from_code(py, &content, &format!("{}.py", file_path), &file_path)?;

                let module_infos_json = module
                    .getattr("ModuleInfos")? // Grab the ModuleInfos class
                    .call0()? // Call the __init__() class method
                    .call_method0("to_json")? // Call the to_json() method
                    .to_string(); // Convert the json data to string

                // Serialize the json data into a Module structure
                let module: Module = serde_json::from_str(&module_infos_json).unwrap();

                Ok(module)
            });

            if module.is_err() {
                error!(format!(
                    "Failed to load module: {} ({})",
                    file_path,
                    module.err().unwrap()
                ));
                continue;
            }

            modules.push(module.unwrap())
        }

        Modules { modules: modules }
    }

    /// Get a module by specifing the type in the scope
    pub fn get_modules_by_type(&self, t_type: &str) -> Vec<&Module> {
        let mut results = Vec::new();
        for module in &self.modules {
            if module.target_types.contains(&t_type.to_string()) {
                results.push(module)
            }
        }
        results
    }
}
