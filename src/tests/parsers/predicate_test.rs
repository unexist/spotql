//!
//! @package Spotql
//!
//! @file Spotql predicate parser tests
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use crate::parsers::incoming::predicate::{Operator, Predicate, Combinator, predicate_parser, predicate_list_parser};
use crate::parsers::parser_error::ParserError;

fn parse_predicate(input: &str) -> Result<Predicate<'_>, ParserError> {
    match predicate_parser(input.as_bytes()) {
        Ok((_, pred)) => Ok(pred),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}

fn parse_predicates(input: &str) -> Result<Vec<Predicate<'_>>, ParserError> {
    match predicate_list_parser(input.as_bytes()) {
        Ok((_, preds)) => Ok(preds),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}

///
/// Simple predicates
///

#[test]
fn should_parse_simple_equal_predicate() {
    match parse_predicate("a = b") {
        Ok(pred) => {
            assert_eq!(pred.left_hand, "a");
            assert_eq!(pred.op, Operator::EQUAL);
            assert_eq!(pred.right_hand, "b");
            assert!(pred.combinator.is_none());
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_simple_greater_predicate() {
    match parse_predicate("playcount > 25") {
        Ok(pred) => {
            assert_eq!(pred.left_hand, "playcount");
            assert_eq!(pred.op, Operator::GREATER);
            assert_eq!(pred.right_hand, "25");
            assert!(pred.combinator.is_none());
        },
        Err(e) => panic!("Error: {}", e),
    }
}

///
/// Multiple predicates
///

#[test]
fn should_parse_combi_greater_and_equal_predicates() {
    match parse_predicates("playcount > 25 and a = b") {
        Ok(preds) => {
            assert_eq!(preds.len(), 2);

            assert_eq!(preds[0].left_hand, "playcount");
            assert_eq!(preds[0].op, Operator::GREATER);
            assert_eq!(preds[0].right_hand, "25");
            assert_eq!(preds[0].combinator, Some(Combinator::AND));

            assert_eq!(preds[1].left_hand, "a");
            assert_eq!(preds[1].op, Operator::EQUAL);
            assert_eq!(preds[1].right_hand, "b");
            assert_eq!(preds[1].combinator, None);
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_combi_unequal_and_unlike_predicates() {
    match parse_predicates("playcount <> 25 and a !~ b") {
        Ok(preds) => {
            assert_eq!(preds.len(), 2);

            assert_eq!(preds[0].left_hand, "playcount");
            assert_eq!(preds[0].op, Operator::UNEQUAL);
            assert_eq!(preds[0].right_hand, "25");
            assert_eq!(preds[0].combinator, Some(Combinator::AND));

            assert_eq!(preds[1].left_hand, "a");
            assert_eq!(preds[1].op, Operator::UNLIKE);
            assert_eq!(preds[1].right_hand, "b");
            assert_eq!(preds[1].combinator, None);
        },
        Err(e) => panic!("Error: {}", e),
    }
}
