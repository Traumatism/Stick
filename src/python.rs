use crate::results::Results;
pub(crate) use pyo3::{
    prelude::{PyModule, PyResult, Python},
    types::PyTuple,
};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct ModulesList {
    modules: Vec<String>,
}

impl ModulesList {
    pub fn load() -> Self {
        let content = fs::read_to_string("modules.json").unwrap();
        let data: ModulesList = serde_json::from_str(&content).unwrap();

        data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub file_path: String,
    pub name: String,
    pub desc: String,
    pub author: String,
    pub target_types: Vec<String>,
}

impl Module {
    pub fn execute(&self, target: &str) {
        let _ = Python::with_gil(|py| -> PyResult<()> {
            let content = fs::read_to_string(format!("{}.py", self.file_path))?;
            let module = PyModule::from_code(
                py,
                &content,
                &format!("{}.py", self.file_path),
                &self.file_path,
            )?;

            let output = module
                .getattr("execute")?
                .call1(PyTuple::new(py, &[target]))?
                .to_string();

            println!("{}", Results::load(output).render(&self.name));

            Ok(())
        })
        .expect(&format!("Failed to run module: {}", self.name));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modules {
    modules: Vec<Module>,
}

impl Modules {
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
            })
            .unwrap();

            modules.push(module)
        }

        Modules { modules: modules }
    }

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
