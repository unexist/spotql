///
/// @package Spotql
///
/// @file Spotql auth parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::query::{ Query, query_parser };
use crate::parsers::statement::Verb;
use crate::parsers::parser_error::ParserError;

fn parse_query(input: &[u8]) -> Result<Query, ParserError> {
    match query_parser(input) {
        Ok((_, query)) => Ok(query),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}
//
// Auth message
//

static MESSAGE: &'static str = "Q\u{0}\u{0}\u{0}\u{19}select * from songs;\u{0}";

#[test]
fn test_parse_query() {
    match parse_query(MESSAGE.as_bytes()) {
        Ok(query) => {
            assert_eq!(query.tag, 'Q');
            assert!(query.statement.is_some());

            let stmnt = query.statement.unwrap();

            assert_eq!(stmnt.verb, Verb::SELECT);
            assert_eq!(stmnt.columns, Some(vec!["*"]));
            assert_eq!(stmnt.table, Some("songs"));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}