///
/// @package Spotql
///
/// @file Spotql auth parser tests
/// @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
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
// Query message
//

static MESSAGE: &'static str = "Q\u{0}\u{0}\u{0}\u{19}select * from songs;\u{0}";

#[test]
fn test_parse_query() {
    match parse_query(MESSAGE.as_bytes()) {
        Ok(query) => {
            assert_eq!(query.tag, 'Q');
            assert!(query.statement.is_some());

            let stmt = query.statement.unwrap();

            assert_eq!(stmt.verb, Verb::SELECT);
            assert!(stmt.columns.is_some());

            match stmt.columns {
                Some(cols) => {
                    assert_eq!(cols.len(), 1);

                    let col = cols.get(0).unwrap();

                    assert_eq!(col.name, "*");
                    assert!(col.alias.is_none());
                },
                None => unreachable!(),
            }

            assert_eq!(stmt.table, Some("songs"));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}