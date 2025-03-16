use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_functions() {
    let code = std::fs::read_to_string("tests/functions.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse functions.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
