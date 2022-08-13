mod common;
mod modules;
mod results;
mod tui;
mod types;

use modules::Modules;
use types::match_types;

use clap::Parser;
use std::io::Result;

#[derive(Parser, Debug)]
struct Cli {
    target: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    print_banner!();
    let args = Cli::parse();

    let target = args.target;

    let compatible_types = match_types(target.clone());

    if compatible_types.len() == 0 {
        error!(format!("Found target {} was compatible no types", &target));
        return Ok(());
    }

    info!(format!(
        "Found target {} was compatible with types:",
        &target
    ));

    compatible_types
        .iter()
        .for_each(|t| println!(" - {}", t.hname()));

    let modules = Modules::load();

    for t_type in &compatible_types {
        modules
            .get_modules_by_type(t_type.rname())
            .iter()
            .for_each(|module| module.execute(&target));
    }

    Ok(())
}
