use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to project `src`
pub struct ChalkParser;

pub fn parse(source: &str) -> String {
    let mut result = String::new();
    let pairs = ChalkParser::parse(Rule::library, source).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        result.push_str(&format!("{:#?}", pair));
    }
    result
}
