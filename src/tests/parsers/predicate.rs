///
/// @package Spotql
///
/// @file Spotql predicate parser tests
/// @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::predicate::{ Operator, Predicate, Combinator, predicate_parser };
use crate::parsers::parser_error::ParserError;

fn parse_predicate(input: &str) -> Result<Predicate, ParserError> {
    match predicate_parser(input.as_bytes()) {
        Ok((_, predicate)) => Ok(predicate),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}

#[test]
fn test_parse_simple_equal_predicate() {
    match parse_predicate("a = b") {
        Ok(pred) => {
            assert_eq!(pred.left_hand, "a");
            assert_eq!(pred.op, Operator::EQUAL);
            assert_eq!(pred.right_hand, "b");
            assert!(pred.combinator.is_none());
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_greater_predicate() {
    match parse_predicate("playcount > 25") {
        Ok(pred) => {
            assert_eq!(pred.left_hand, "playcount");
            assert_eq!(pred.op, Operator::GREATER);
            assert_eq!(pred.right_hand, "25");
            assert!(pred.combinator.is_none());
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_combi_greater_predicate() {
    match parse_predicate("playcount > 25 and a = b") {
        Ok(pred) => {
            assert_eq!(pred.left_hand, "playcount");
            assert_eq!(pred.op, Operator::GREATER);
            assert_eq!(pred.right_hand, "25");
            assert_eq!(pred.combinator, Some(Combinator::AND));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}