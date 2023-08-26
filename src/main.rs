use std::env;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use std::string::String;

use serde::Deserialize;
use toml::{Table,Value};
use toml::map::Map;


#[derive(Deserialize, Debug)]
struct Config {
    preview: String,
    confirm: Option<bool>,
    execute: Vec<ExecuteMap>,
}

#[derive(Deserialize, Debug)]
struct ExecuteMap{
    key: char,
    command: String,
}

fn print_usage() {
    let usage = r#"
Usage: inmap CONFIG
Preview and execute commands on data interactively
Example: ls | inmap config.toml
config.toml contains instructions on how to preview and execute commands.

Example config.toml:

preview = "firefox {}"
confirm = false
execute = [
	{ key = "h", command = "rm {}" },
	{ key = "j", command = "cp {} /backup" },
	{ key = "k", command = "echo {}" },
	{ key = "l", command = "grep {} | xargs rm" },
]

"#;
    print!("{}", usage);
}

fn check_config(config: &Config) {
}

/// Reads the passed argument as a toml file and returns it after validating it has required keys
/// Halts program execution if there are more or less than 1 additional arg.
fn process_config_args(arg_list: Vec<String>) -> Result<Config, Box<dyn Error>> {
    match arg_list.len() {
	2 =>  {
	    let config_file = PathBuf::from(arg_list.last().unwrap());
	    let contents = std::fs::read_to_string(config_file)?; // &mut contents)?;
//	    let table = contents.parse::<Table>()?;
	    let config: Config = toml::from_str(&contents)?;
	    check_config(&config);
	    Ok(config)
	},
	_ => {
	    print_usage();
	    exit(1);
	}
    }
}

fn run_line(line: String, config: &Config) {
    // we need the templating, to do the string replacement.
    // do I need real templating, or is there a string function that can do substring replacement?
    // we should probably do a quick validation of the config before it gets here, namely that the
    // commands (preview and execute) are valid (i.e. have no more than one instance of "{}" which
    // is our replacement string.
    println!("this is some input: {}", line);
}


fn main() {
    let config = process_config_args(env::args().collect());
    let lines = io::stdin().lines();
    match config {
	Ok(config) => {
	    println!("ok config!");
	    for line in lines {
		run_line(line.unwrap(), &config);
	    }
	},
	Err(e) => {
	    eprintln!("{}", e);
	    exit(1);
	}
    }
}
