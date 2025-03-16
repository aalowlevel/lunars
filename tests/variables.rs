use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_variables() {
    let code = std::fs::read_to_string("tests/variables.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse variables.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
