#[macro_use] extern crate rocket;

use clap::{arg, command, ArgMatches};
use regex::Regex;
use roo_engine::{server, parser};
use rocket::{response::Redirect, State};

#[launch]
fn start_server() -> _ {
    let matches = parse_arguments();
    
    let rules = match matches.get_one::<String>("rules_path") {
        Some(config_path) => parser::load_config(config_path),
        None => parser::get_default_rules()
    };

    let address = matches.get_one::<String>("address").expect("Address argument is required.");
    let port = matches.get_one::<String>("port").unwrap().parse::<u16>().unwrap();
    
    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", address));

    rocket::custom(figment)
        .manage(rules)
        .mount("/", routes![index])
}

#[get("/?<q>")]
fn index(q: &str, rules: &State<Vec<(Regex, String)>>) -> Redirect {
    let output = server::handle_input(q, &rules);
    
    return Redirect::to(output);
}

fn parse_arguments() -> ArgMatches {
    let matches = command!()
        .arg(arg!(--address <VALUE>).required(false).default_value("127.0.0.1"))
        .arg(arg!(--port <VALUE>).required(false).default_value("3030"))
        .arg(arg!(--rules_path <VALUE>).required(false))
        .get_matches();

    return matches;
}