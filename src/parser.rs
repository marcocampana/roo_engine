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

pub fn get_default_rules() -> Vec<(Regex, String)> {
    let commands = vec![
        (
            "^g (.+)$".to_string(),
            "https://www.google.com/search?q={}".to_string()
        ),
        (
            "^a (.+)$".to_string(),
            "https://www.amazon.com/s?k={}".to_string()
        ),
        (
            "^cal$".to_string(),
            "https://calendar.google.com/".to_string()
        ),
        (
            "^newmail$".to_string(),
            "https://mail.google.com/mail/?view=cm".to_string()
        ),
        (
            "^newdoc$".to_string(),
            "https://docs.google.com/document/u/0/create".to_string()
        ),
        (
            "^newsheet$".to_string(),
            "https://docs.google.com/spreadsheets/u/0/create".to_string()
        ),
        (
            "^y$".to_string(),
            "https://www.youtube.com".to_string()
        ),
        (
            "^y (.+)$".to_string(),
            "https://www.youtube.com/results?search_query={}".to_string()

        ),
        (
            "^maps$".to_string(),
            "https://www.google.com/maps".to_string()
        ),
        (
            "^maps (.+)$".to_string(),
            "https://www.google.com/maps/search/{}".to_string()
        ),
        (
            "^drive$".to_string(),
            "https://drive.google.com/drive/u/0/my-drive".to_string()
        ),
        (
            "^drive (.+)$".to_string(),
            "https://drive.google.com/drive/u/0/search?q={}".to_string()
        ),
        (
            "^gh$".to_string(),
            "https://github.com/".to_string()
        ),
        (
            "^gh (.+)$".to_string(),
            "https://github.com/search?q={}".to_string()
        ),
        (
            "^ghc (.+)$".to_string(),
            "https://github.com/search?q={}&type=code".to_string()
        ),
        (
            "^ghi (.+)$".to_string(),
            "https://github.com/search?q={}&type=issues".to_string()
        ),
        (
            "^tw @(.+)$".to_string(),
            "https://twitter.com/{}".to_string()
        ),
        (
            "^tw ([^@].+)$".to_string(),
            "https://twitter.com/search?q={}".to_string()
        ),
        (
            "^(i-.+)$".to_string(),
            "https://us-east-1.console.aws.amazon.com/ec2/home?region=us-east-1#InstanceDetails:instanceId={}".to_string()
        ),
        (
            "^(vpc-.+)$".to_string(),
            "https://us-east-1.console.aws.amazon.com/vpc/home?region=us-east-1#VpcDetails:VpcId={}".to_string()
        ),
    ];

    let mut rules: Vec<(Regex, String)> = vec![];

    for rule in commands {
        let input  = Regex::new(&rule.0).unwrap();
        let output = rule.1.to_string();

        rules.push((input,output));
    }

    return rules;
}

pub fn load_config(path: &String) -> Vec<(Regex, String)> {
    let contents = fs::read_to_string(path)
        .expect(&format!("error: could not find {}", path));

    let config: Config = toml::from_str(&contents).unwrap();

    let mut rules: Vec<(Regex, String)> = vec![];

    for command in config.command {
        let input = Regex::new(&command.input).unwrap();
        let output = command.output.to_string();

        rules.push((input, output));
    }

    return rules;
}