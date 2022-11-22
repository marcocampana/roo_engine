use std::collections::HashMap;
use regex::Regex;
use warp::{
    Filter, hyper::Uri, Rejection, Reply,
};

#[tokio::main]
pub async fn start(port: &u16, rules: Vec<(Regex, String)>) {
    let app = warp::get().and(filter_query(rules));

    warp::serve(app)
        .run(([127,0,0,1], *port))
        .await
}

pub fn filter_query(rules: Vec<(Regex, String)>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    return warp::query::<HashMap<String, String>>()
        .map(move |p: HashMap<String, String>| match p.get("q") {
            Some(q) => warp::redirect::temporary(handle_input(q, &rules)),
            None => warp::redirect::temporary(Uri::from_static("/help.html")),
        });
}

pub fn handle_input(q: &String, rules: &Vec<(Regex, String)>) -> Uri {    
    for (input, output) in rules {
        if input.is_match(q) {

            let mut s = String::from(output);
            for cap in input.captures_iter(q) {
                for i in 1..cap.len() {
                    s = s.replacen("{}", &cap[i], 1);
                }
            }

            return s.replace(' ', "%20").parse::<Uri>().unwrap();
        }
    }

    // TODO handle default is there is not match
    return "/help.html".parse().unwrap();
} 