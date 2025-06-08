use analyzer::Analyzer;
use anyhow::Result;
use better_panic::Settings;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::{Config, Rule};
use stderrlog::LogLevelNum;
use util::find;

mod analyzer;
mod cli;
mod config;
mod source_to_dom;
mod util;

fn main() -> Result<()> {
    set_panic_hook();

    let cli = Cli::parse();

    let config = if let Some(config_path) = cli.config {
        Config::load(config_path)
    } else {
        Config::new()
    };

    stderrlog::new()
        .module(module_path!())
        .verbosity(if cli.verbose {
            LogLevelNum::Debug
        } else {
            LogLevelNum::Warn
        })
        .init()
        .unwrap();

    match cli.command {
        Commands::Eval { path, xpath, json } => eval(&path, &xpath, json),
        Commands::Analyze { path, json } => analyze(&path, config, json),
    }
}

fn eval(path: &Vec<String>, xpath: &String, json: bool) -> Result<()> {
    for path in find(path) {
        let results = Analyzer::new()
            .analyze_file(
                &path,
                &vec![Rule {
                    name: "anonymous".to_string(),
                    xpath: xpath.to_string(),
                }],
            )
            .unwrap();
        print_result(results, json)
    }
    Ok(())
}

fn analyze(path: &Vec<String>, config: Config, json: bool) -> Result<()> {
    for path in find(path) {
        let results = Analyzer::new().analyze_file(&path, &config.rules).unwrap();
        print_result(results, json)
    }
    Ok(())
}
fn print_result(results: analyzer::FileResults, json: bool) {
    if results.len() > 0 {
        if json {
            println!("{}", serde_json::to_string(&results).unwrap());
        } else {
            println!("{}", results.path.yellow());
            for result in results.results {
                println!("[{}] {}", result.rule.cyan(), result.result);
            }
        }
    }
}
fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .create_panic_handler()(panic_info);
    }));
}
