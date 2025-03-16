use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use pest::Parser;

#[test]
fn test_rust_interop() {
    let code = std::fs::read_to_string("tests/rust_interop.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse rust_interop.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
