use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_data_structures() {
    let code = std::fs::read_to_string("tests/data_structures.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse data_structures.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
