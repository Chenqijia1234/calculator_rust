use crate::number_parser;

#[test]
fn test_number() {
    number_parser::arithmetic("  ( 443 + 24) * 434").unwrap();
}
