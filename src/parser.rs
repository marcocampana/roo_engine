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
            "^g (.+)$",
            "https://www.google.com/search?q={}"
        ),
        (
            "^a (.+)$",
            "https://www.amazon.com/s?k={}"
        ),
        (
            "^cal$",
            "https://calendar.google.com/"
        ),
        (
            "^newmail$",
            "https://mail.google.com/mail/?view=cm"
        ),
        (
            "^newdoc$",
            "https://docs.google.com/document/u/0/create"
        ),
        (
            "^newsheet$",
            "https://docs.google.com/spreadsheets/u/0/create"
        ),
        (
            "^y$",
            "https://www.youtube.com"
        ),
        (
            "^y (.+)$",
            "https://www.youtube.com/results?search_query={}"

        ),
        (
            "^maps$",
            "https://www.google.com/maps"
        ),
        (
            "^maps (.+)$",
            "https://www.google.com/maps/search/{}"
        ),
        (
            "^drive$",
            "https://drive.google.com/drive/u/0/my-drive"
        ),
        (
            "^drive (.+)$",
            "https://drive.google.com/drive/u/0/search?q={}"
        ),
        (
            "^gh$",
            "https://github.com/"
        ),
        (
            "^gh (.+)$",
            "https://github.com/search?q={}"
        ),
        (
            "^ghc (.+)$",
            "https://github.com/search?q={}&type=code"
        ),
        (
            "^ghi (.+)$",
            "https://github.com/search?q={}&type=issues"
        ),
        (
            "^tw @(.+)$",
            "https://twitter.com/{}"
        ),
        (
            "^tw ([^@].+)$",
            "https://twitter.com/search?q={}"
        ),
        (
            "^redd (.+)$",
            "https://www.reddit.com/search/?q={}"
        ),
        (
            "^pin (.+)$",
            "https://www.pinterest.com/search/pins/?q={}"
        ),
        (
            "^(i-.+)$",
            "https://us-east-1.console.aws.amazon.com/ec2/home?region=us-east-1#InstanceDetails:instanceId={}"
        ),
        (
            "^(vpc-.+)$",
            "https://us-east-1.console.aws.amazon.com/vpc/home?region=us-east-1#VpcDetails:VpcId={}"
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