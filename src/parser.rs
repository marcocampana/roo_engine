use std::fs;
use regex::Regex;
use serde::Deserialize;

#[cfg(test)]
mod parser_tests;

#[derive(Deserialize)]
struct Command {
    input: String,
    output: String,
}

#[derive(Deserialize)]
struct Config {
    command: Vec<Command>,
}

pub fn load_config(path: &String) -> Vec<(Regex, String)> {
    let contents = fs::read_to_string(path)
        .expect(&format!("error: could not find {}", path));
    
    let config: Config = toml::from_str(&contents).unwrap();

    let mut rules: Vec<(Regex, String)>  = vec![];

    for command in config.command {
        let input = Regex::new(&command.input).unwrap();
        let output = command.output.to_string();
        
        rules.push((input, output));
    }

    return rules;
}