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

use crate::parsers::statement::{ Verb, parse_statement };

///
/// Simple verbs
///

#[test]
fn test_parse_simple_select_statement() {
    match parse_statement("select") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_update_statement() {
    match parse_statement("update") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::UPDATE);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

///
/// Columns
///

#[test]
fn test_parse_simple_select_statement_with_single_column() {
    match parse_statement("select *") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert_eq!(stmt.columns, Some(vec!["*"]));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_select_statement_with_single_column_with_as() {
    match parse_statement("select * as all") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert_eq!(stmt.columns, Some(vec!["*"]));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}


#[test]
fn test_parse_simple_select_statement_with_multi_column() {
    match parse_statement("select a, b") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert_eq!(stmt.columns, Some(vec!["a", "b"]));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

///
/// Tables
///

#[test]
fn test_parse_simple_select_statement_with_single_column_and_table() {
    match parse_statement("select * from table") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, Verb::SELECT);
            assert_eq!(stmt.columns, Some(vec!["*"]));
            assert_eq!(stmt.table, Some("table"));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}