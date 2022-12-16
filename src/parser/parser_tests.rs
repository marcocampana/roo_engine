use crate::parser;

#[test]
fn load_config_should_load_rules_in_file() {
    let rules = parser::load_config(&String::from("rules.toml"));

    assert!(rules.len() > 0);
}
