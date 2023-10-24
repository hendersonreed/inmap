use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub preview: String,
    pub confirm: Option<bool>,
    pub execute: Vec<ExecuteMap>,
}

#[derive(Deserialize, Debug)]
pub struct ExecuteMap{
    pub key: char,
    pub command: String,
}

pub fn sub_line_in_commands(line: String, config: &Config) -> String {
    "unimplemented".to_string() 
}
