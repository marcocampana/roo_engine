use regex::Regex;

#[cfg(test)]
mod server_tests;

pub fn handle_input(q: &str, rules: &Vec<(Regex, String)>) -> String {    
    for (input, output) in rules {
        if input.is_match(q) {
            let mut s = String::from(output);
            for cap in input.captures_iter(q) {
                for i in 1..cap.len() {
                    s = s.replacen("{}", &cap[i], 1);
                }
            }

            return encode_url(&s);
        }
    }

    return get_default_uri(q);
} 

fn get_default_uri(q: &str) -> String {
    let default_uri = "https://www.google.com/search?q={}".replace("{}", q);

    return encode_url(&default_uri);
}

fn encode_url(s: &String) -> String {
    return s.replace(" ", "%20");
}
