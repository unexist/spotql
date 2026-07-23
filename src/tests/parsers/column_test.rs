//!
//! @package Spotql
//!
//! @file Spotql column parser tests
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use crate::parsers::incoming::column::{Column, column_list_parser, column_parser};
use crate::parsers::parser_error::ParserError;

fn parse_column(input: &str) -> Result<Column<'_>, ParserError> {
    match column_parser(input.as_bytes()) {
        Ok((_, col)) => Ok(col),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}

fn parse_columns(input: &str) -> Result<Vec<Column<'_>>, ParserError> {
    match column_list_parser(input.as_bytes()) {
        Ok((_, cols)) => Ok(cols),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}

///
/// Simple columns
///

#[test]
fn should_parse_simple_column() {
    match parse_column("songs") {
        Ok(col) => {
            assert!(col.table.is_none());
            assert_eq!(col.name, "songs");
            assert!(col.alias.is_none());
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_simple_column_with_table() {
    match parse_column("spotify.songs") {
        Ok(col) => {
            assert_eq!(col.table, Some("spotify"));
            assert_eq!(col.name, "songs");
            assert!(col.alias.is_none());
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_simple_column_with_alias() {
    match parse_column("songs as foo") {
        Ok(col) => {
            assert!(col.table.is_none());
            assert_eq!(col.name, "songs");
            assert_eq!(col.alias, Some("foo"));
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_simple_column_with_table_and_alias() {
    match parse_column("spotify.songs as foo") {
        Ok(col) => {
            assert_eq!(col.table, Some("spotify"));
            assert_eq!(col.name, "songs");
            assert_eq!(col.alias, Some("foo"));
        },
        Err(e) => panic!("Error: {}", e),
    }
}

///
/// Multiple columns
///

#[test]
fn should_parse_multi_columns() {
    match parse_columns("songs, tracks") {
        Ok(cols) => {
            assert!(cols[0].table.is_none());
            assert_eq!(cols[0].name, "songs");
            assert!(cols[0].alias.is_none());

            assert!(cols[1].table.is_none());
            assert_eq!(cols[1].name, "tracks");
            assert!(cols[1].alias.is_none());
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_multi_columns_with_table() {
    match parse_columns("spotify.songs, spotify.tracks") {
        Ok(cols) => {
            assert_eq!(cols[0].table, Some("spotify"));
            assert_eq!(cols[0].name, "songs");
            assert!(cols[0].alias.is_none());

            assert_eq!(cols[1].table, Some("spotify"));
            assert_eq!(cols[1].name, "tracks");
            assert!(cols[1].alias.is_none());
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_multi_columns_with_tables_and_aliases() {
    match parse_columns("songs as foo, tracks as bar") {
        Ok(cols) => {
            assert!(cols[0].table.is_none());
            assert_eq!(cols[0].name, "songs");
            assert_eq!(cols[0].alias, Some("foo"));

            assert!(cols[1].table.is_none());
            assert_eq!(cols[1].name, "tracks");
            assert_eq!(cols[1].alias, Some("bar"));
        },
        Err(e) => panic!("Error: {}", e),
    }
}

#[test]
fn should_parse_multi_columns_with_aliases() {
    match parse_columns("spotify.songs as foo, spotify.tracks as bar") {
        Ok(cols) => {
            assert_eq!(cols[0].table, Some("spotify"));
            assert_eq!(cols[0].name, "songs");
            assert_eq!(cols[0].alias, Some("foo"));

            assert_eq!(cols[1].table, Some("spotify"));
            assert_eq!(cols[1].name, "tracks");
            assert_eq!(cols[1].alias, Some("bar"));
        },
        Err(e) => panic!("Error: {}", e),
    }
}
