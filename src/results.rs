use serde_derive::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct Row {
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct Node {
    name: String,
    rows: Vec<Row>,
}

/// Module results tree
///
///
/// Results for module ...
/// ======================
///     
///     Node name 1
///     -----------
///         Key: value
///         Key: value
///
///     Node name 2
///     -----------
///         Key: value
///
///     Node name 3
///     -----------
///         No results.
///
///
#[derive(Deserialize)]
pub struct Results {
    nodes: Vec<Node>,
}

impl Results {
    /// Load from module execute() call output
    pub fn load(data: String) -> Self {
        let results: Results = serde_json::from_str(&data).unwrap();
        results
    }

    /// Render the results
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

            if &node.rows.len() == &0 {
                rendered.push_str("        No results.\n");
                continue;
            }

            for row in &node.rows {
                rendered.push_str(&format!("        {}: {}\n", row.key, row.value));
            }
        }

        rendered
    }
}
