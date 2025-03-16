use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_returns() {
    let code = std::fs::read_to_string("tests/returns.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse returns.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
