mod abc;
mod modules;
mod results;
mod tui;
mod types;

use modules::{Module, Modules};
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

    println!(""); // add a new line before printing modules content

    let modules = Modules::load();

    let mut fmodules: Vec<&Module> = Vec::new();

    for t_type in &compatible_types {
        fmodules.append(&mut modules.get_modules_by_type(t_type.rname()));
    }

    if fmodules.len() == 0 {
        error!("Found not even one module to run.");
        return Ok(());
    }

    info!("Running modules:");

    fmodules
        .iter()
        .for_each(|t| println!(" - {} ({})", t.name, t.desc));

    println!(""); // add a new line before printing modules content

    fmodules.iter().for_each(|t| t.execute(&target));

    ok!("done!");

    Ok(())
}
