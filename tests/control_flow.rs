use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_control_flow() {
    let code = std::fs::read_to_string("tests/control_flow.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse control_flow.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
