use clap::{arg, command, ArgMatches};
use roo::{server, parser};

fn main() {
    let matches = parse_arguments();
    
    let config_path = matches.get_one::<String>("path").expect("Path argument is required.");
    // let host = matches.get_one::<String>("host").expect("Host argument is required.");
    let port = matches.get_one::<String>("port").unwrap().parse::<u16>().unwrap();
    let rules = parser::load_config(config_path);

    server::start(&port, rules);
}

fn parse_arguments() -> ArgMatches {
    let matches = command!()
        .arg(arg!(--host <VALUE>).required(false).default_value("127.0.0.1"))
        .arg(arg!(--port <VALUE>).required(false).default_value("3030"))
        .arg(arg!(--path <VALUE>).required(false).default_value("./rules.toml"))
        .get_matches();

    return matches;
}