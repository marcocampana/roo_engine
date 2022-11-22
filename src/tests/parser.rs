use crate::parser;

#[test]
fn load_config_should_load_default_rules() {
    let rules = parser::load_config(&String::from("rules.toml"));

    assert!(rules.len() > 0);
}
