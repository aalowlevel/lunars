use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_pattern_matching() {
    let code = std::fs::read_to_string("tests/pattern_matching.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse pattern_matching.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
