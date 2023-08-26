use std::env;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use std::string::String;

use serde::Deserialize;


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

/// checks that each line of the preview and execute commands only have one set of {}.
fn check_config(config: &Config) -> Result<(), String> {
    let preview_matches: Vec<_> = config.preview
	.matches("{}")
	.collect();
    if preview_matches.len() != 1 {
	return Err(
	    format!(
		"Configured preview command:
		\t{}
		has incorrect number of instances of {{}}. Please reformat to have exactly 1.", 
		config.preview)
	    .trim_start()
	    .into())
    }
    for each in &config.execute {
	let command_matches: Vec<_> = each.command.matches("{}").collect();
	if command_matches.len() != 1 {
	    return Err(
		format!(
		    "Configured execute command: 
		    \t{}: {}
		    has incorrect number of instances of {{}}. Please reformat to have exactly 1.", 
		    each.key, 
		    each.command)
	    .trim_start()
	    .into())
	}
    }
    Ok(())
}

/// Reads the passed argument as a toml file and returns it after validating it has required keys
/// Halts program execution if there are more or less than 1 additional arg.
fn process_config_args(arg_list: Vec<String>) -> Result<Config, Box<dyn Error>> {
    match arg_list.len() {
	2 =>  {
	    let config_file = PathBuf::from(arg_list.last().unwrap());
	    let contents = std::fs::read_to_string(config_file)?; // &mut contents)?;
	    let config: Config = toml::from_str(&contents)?;
	    let _ = check_config(&config)?;
	    Ok(config)
	},
	_ => {
	    print_usage();
	    exit(1);
	}
    }
}

fn run_line(line: String, config: &Config) {
    // clear the screen.
    // replace {} in preview, execute it.
    // replace {} in each execute line, display them, along with the key commands.
    // wait for input (and if confirm, then wait for a <CR> as well.)
    // execute matching execute line.
    // if no execute matches input, then print an error and require a <CR> to continue.
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
