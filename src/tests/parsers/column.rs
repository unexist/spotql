///
/// @package Spotql
///
/// @file Spotql column parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::column::{ Column, column_parser };
use crate::parsers::parser_error::ParserError;

fn parse_column(input: &str) -> Result<Column, ParserError> {
    match column_parser(input.as_bytes()) {
        Ok((_, col)) => Ok(col),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}

///
/// Simple columns
///

#[test]
fn test_parse_simple_column() {
    match parse_column("songs") {
        Ok(col) => {
            assert_eq!(col.name, "songs");
            assert!(col.alias.is_none());
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}

#[test]
fn test_parse_simple_column_with_alias() {
    match parse_column("songs as foo") {
        Ok(col) => {
            assert_eq!(col.name, "songs");
            assert_eq!(col.alias, Some("foo"));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}