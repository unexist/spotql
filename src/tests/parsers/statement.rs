///
/// @package Spotql
///
/// @file Spotql statement parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::statement::{ statement_parser, Statement, Verb };
use crate::parsers::predicate::{ Operator, Combinator };
use crate::parsers::parser_error::ParserError;

fn parse_statement(input: &str) -> Result<Statement, ParserError> {
    match statement_parser(input.as_bytes()) {
        Ok((_, stmt)) => Ok(stmt),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}

///
/// Simple verbs
///

#[test]
fn test_parse_simple_select_statement() {
    match parse_statement("select;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_update_statement() {
    match parse_statement("update;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::UPDATE);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

///
/// Columns
///

macro_rules! test_column {
    ($column:stmt, $name:expr, $alias:expr) => {
        match {{ $column }} {
            Some(col) => {
                assert_eq!(col.name, $name);
                assert_eq!(col.alias, $alias);
            },
            None => unreachable!()
        }
    };
}

#[test]
fn test_parse_simple_select_statement_with_single_column() {
    match parse_statement("select *;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert!(stmt.columns.is_some());

            match stmt.columns {
                Some(list) => {
                    assert_eq!(list.len(), 1);

                    test_column!(list.get(0), "*", None);
                },
                None => unreachable!(),
            }

        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_select_statement_with_single_column_with_as() {
    match parse_statement("select a as all;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert!(stmt.columns.is_some());

            match stmt.columns {
                Some(list) => {
                    assert_eq!(list.len(), 1);

                    test_column!(list.get(0), "a", Some("all"));
                },
                None => unreachable!(),
            }
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}


#[test]
fn test_parse_simple_select_statement_with_multi_column() {
    match parse_statement("select a, b;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert!(stmt.columns.is_some());

            match stmt.columns {
                Some(list) => {
                    assert_eq!(list.len(), 2);

                    test_column!(list.get(0), "a", None);
                    test_column!(list.get(1), "b", None);
                },
                None => unreachable!(),
            }
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

///
/// Tables
///

#[test]
fn test_parse_simple_select_statement_with_single_column_and_table() {
    match parse_statement("select * from table;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert!(stmt.columns.is_some());

            match stmt.columns {
                Some(list) => {
                    assert_eq!(list.len(), 1);

                    test_column!(list.get(0), "*", None);
                },
                None => unreachable!(),
            }

            assert_eq!(stmt.table, Some("table"));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

///
/// Predicates
///

macro_rules! test_predicate {
    ($predicate:stmt, $left:expr, $op:expr, $right:expr, $combi:expr) => {
        match {{ $predicate }} {
            Some(pred) => {
                assert_eq!(pred.left_hand, $left);
                assert_eq!(pred.op, $op);
                assert_eq!(pred.right_hand, $right);
                assert_eq!(pred.combinator, $combi);
            },
            None => unreachable!()
        }
    };
}

#[test]
fn test_parse_simple_select_statement_with_single_column_and_table_and_simple_predicate() {
    match parse_statement("select * from table where a = b;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert!(stmt.columns.is_some());

            match stmt.columns {
                Some(list) => {
                    assert_eq!(list.len(), 1);

                    test_column!(list.get(0), "*", None);
                },
                None => unreachable!(),
            }

            assert_eq!(stmt.table, Some("table"));
            assert!(stmt.predicates.is_some());

            match stmt.predicates {
                Some(list) => {
                    assert_eq!(list.len(), 1);

                    test_predicate!(list.get(0), "a", Operator::EQUAL, "b", None);
                },
                None => unreachable!(),
            }
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_select_statement_with_single_column_and_table_and_combi_predicate() {
    match parse_statement("select * from table where a = b and b = a;") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert!(stmt.columns.is_some());

            match stmt.columns {
                Some(list) => {
                    assert_eq!(list.len(), 1);

                    test_column!(list.get(0), "*", None);
                },
                None => unreachable!(),
            }

            assert_eq!(stmt.table, Some("table"));
            assert!(stmt.predicates.is_some());

            match stmt.predicates {
                Some(list) => {
                    assert_eq!(list.len(), 2);

                    test_predicate!(list.get(0), "a", Operator::EQUAL, "b", Some(Combinator::AND));
                    test_predicate!(list.get(1), "b", Operator::EQUAL, "a", None);
                },
                None => unreachable!(),
            }
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}