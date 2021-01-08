///
/// @package Spotql
///
/// @file Spotql parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parser::{ Verb, parse };

///
/// Simple statements
///

#[test]
fn test_parse_simple_select_statement() {
    match parse("select") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_update_statement() {
    match parse("update") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::UPDATE);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_select_statement_with_column() {
    match parse("select *") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert_eq!(stmt.columns, Some(vec!["*"]));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_select_statement_with_column_and_table() {
    match parse("select * from table") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert_eq!(stmt.columns, Some(vec!["*"]));
            assert_eq!(stmt.table, Some("table"));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}