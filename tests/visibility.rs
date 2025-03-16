use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_visibility() {
    let code = std::fs::read_to_string("tests/visibility.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse visibility.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
