///
/// @package Spotql
///
/// @file Spotql predicate parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::predicate::{ Operator, Predicate, predicate_parser };
use crate::parsers::parser_error::ParserError;

fn parse_predicate(input: &[u8]) -> Result<Predicate, ParserError> {
    match predicate_parser(input) {
        Ok((_, predicate)) => Ok(predicate),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}
//
// Predicate message
//

static MESSAGE: &'static str = "playcount > 25";

#[test]
fn test_parse_simple_predicate() {
    match parse_predicate(MESSAGE.as_bytes()) {
        Ok(pred) => {
            assert_eq!(pred.left_hand, "playcount");
            assert_eq!(pred.op, Operator::GREATER);
            assert_eq!(pred.right_hand, "25");
            assert!(pred.combinator.is_none());
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}