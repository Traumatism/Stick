use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Row {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    name: String,
    rows: Vec<Row>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
    nodes: Vec<Node>,
}

impl Results {
    pub fn load(data: String) -> Self {
        let results: Results = serde_json::from_str(&data).unwrap();
        results
    }

    pub fn render(&self, module_name: &str) -> String {
        let mut rendered = String::new();

        let s = &format!("\nResults for module: {}", module_name);

        rendered.push_str(s);
        rendered.push('\n');

        rendered.push_str(&"=".repeat(s.len()));
        rendered.push('\n');

        for node in &self.nodes {
            rendered.push('\n');

            rendered.push_str(&format!(
                "    {}\n    {}\n",
                node.name,
                "-".repeat(node.name.len())
            ));

            for row in &node.rows {
                rendered.push_str(&format!("        {}: {}\n", row.key, row.value));
            }
        }

        rendered
    }
}
