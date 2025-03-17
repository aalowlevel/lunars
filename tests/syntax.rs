use std::fs::read_to_string;

use lunars::parser::LunarsParser;
use lunars::parser::Rule;
use lunars::parser::ast::Program;
use pest::Parser;

#[test]
fn test_control_flow() {
    let code = read_to_string("tests/control_flow.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse control_flow.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}

#[test]
fn test_data_structures() {
    let code = read_to_string("tests/data_structures.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse data_structures.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}

#[test]
fn test_functions() {
    let code = read_to_string("tests/functions.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse functions.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}

#[test]
fn test_modules() {
    let code = read_to_string("tests/modules.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse modules.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}

#[test]
fn test_pattern_matching() {
    let code = read_to_string("tests/pattern_matching.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse pattern_matching.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}

#[test]
fn test_returns() {
    let code = read_to_string("tests/returns.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse returns.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}

#[test]
fn test_rust_interop() {
    let code = read_to_string("tests/rust_interop.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse rust_interop.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}

#[test]
fn test_variables() {
    let code = read_to_string("tests/variables.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse variables.lnrs");

    let program = Program::try_from(pairs.unwrap());
    dbg!(&program);
    assert!(program.is_ok(), "Failed to map to AST variables.lnrs");
}

#[test]
fn test_visibility() {
    let code = read_to_string("tests/visibility.lnrs").unwrap();
    let pairs = LunarsParser::parse(Rule::program, &code);
    assert!(pairs.is_ok(), "Failed to parse visibility.lnrs");

    // Uncomment to print parsed results
    // println!("{:#?}", pairs);
}
