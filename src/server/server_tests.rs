use crate::server;
use crate::parser;

#[test]
fn handle_input_returns_uri_for_the_given_input() {
    let rules = parser::load_config(&String::from("rules.toml"));

    let uri = server::handle_input(&String::from("g hello world!"), &rules);
    assert_eq!(uri, "https://www.google.com/search?q=hello%20world!");

    let uri = server::handle_input(&String::from("a hello world!"), &rules);
    assert_eq!(uri, "https://www.amazon.com/s?k=hello%20world!");

    let uri = server::handle_input(&String::from("gh hello world!"), &rules);
    assert_eq!(uri, "https://github.com/search?q=hello%20world!");

    let uri = server::handle_input(&String::from("ghi hello world!"), &rules);
    assert_eq!(uri, "https://github.com/search?q=hello%20world!&type=issues");

    let uri = server::handle_input(&String::from("ghc hello world!"), &rules);
    assert_eq!(uri, "https://github.com/search?q=hello%20world!&type=code");

    let uri = server::handle_input(&String::from("newmail"), &rules);
    assert_eq!(uri, "https://mail.google.com/mail/?view=cm");

    let uri = server::handle_input(&String::from("newdoc"), &rules);
    assert_eq!(uri, "https://docs.google.com/document/u/0/create");

    let uri = server::handle_input(&String::from("tw @marcocampana"), &rules);
    assert_eq!(uri, "https://twitter.com/marcocampana");

    let uri = server::handle_input(&String::from("tw rust language"), &rules);
    assert_eq!(uri, "https://twitter.com/search?q=rust%20language");

    let uri = server::handle_input(&String::from("i-1234567890abcdef0"), &rules);
    assert_eq!(uri, "https://us-east-1.console.aws.amazon.com/ec2/home?region=us-east-1#InstanceDetails:instanceId=i-1234567890abcdef0");

    let uri = server::handle_input(&String::from("vpc-12345678"), &rules);
    assert_eq!(uri, "https://us-east-1.console.aws.amazon.com/vpc/home?region=us-east-1#VpcDetails:VpcId=vpc-12345678");
}

#[test]
fn handle_input_when_command_is_unknown_returns_default_uri() {
    let rules = parser::load_config(&String::from("rules.toml"));

    let uri = server::handle_input(&String::from("unknown command"), &rules);
    assert_eq!(uri.to_string(), "https://www.google.com/search?q=unknown%20command");
}