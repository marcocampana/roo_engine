use std::collections::HashMap;
use regex::Regex;
use warp::{
    Filter, hyper::Uri, Rejection, Reply,
};

#[cfg(test)]
mod server_tests;

#[tokio::main]
pub async fn start(port: &u16, rules: Vec<(Regex, String)>) {
    let app = warp::get().and(filter_query(rules));

    warp::serve(app)
        .run(([127,0,0,1], *port))
        .await
}

fn filter_query(rules: Vec<(Regex, String)>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    return warp::query::<HashMap<String, String>>()
        .map(move |p: HashMap<String, String>| match p.get("q") {
            Some(q) => warp::redirect::temporary(handle_input(q, &rules)),
            None => warp::redirect::temporary(Uri::from_static("/help.html")),
        });
}

fn handle_input(q: &String, rules: &Vec<(Regex, String)>) -> Uri {    
    for (input, output) in rules {
        if input.is_match(q) {

            let mut s = String::from(output);
            for cap in input.captures_iter(q) {
                for i in 1..cap.len() {
                    s = s.replacen("{}", &cap[i], 1);
                }
            }

            return encode_url(&s).parse::<Uri>().unwrap();
        }
    }

    return get_default_uri(q);
} 

fn get_default_uri(q: &String) -> Uri {
    let default_uri = "https://www.google.com/search?q={}".replace("{}", &encode_url(q));

    return default_uri.parse::<Uri>().unwrap();
}

fn encode_url(s: &String) -> String {
    return s.replace(" ", "%20");
}
