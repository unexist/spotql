///
/// @package Spotql
///
/// @file Spotql startup parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::startup::{ Startup, startup_parser };
use crate::parsers::parser_error::ParserError;

fn parse_startup(input: &[u8]) -> Result<Startup, ParserError> {
    match startup_parser(input) {
        Ok((_, startup)) => Ok(startup),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}
//
// Startup message
//

static MESSAGE: &'static str = "\u{0}\u{0}\u{0}N\u{0}\u{3}\u{0}\u{0}\
    user\u{0}unexist\u{0}database\u{0}foo\u{0}application_name\u{0}\
    psql\u{0}client_encoding\u{0}UTF8\u{0}\u{0}";

static PROTOCOL_VERSION: i32 = 196608;

#[test]
fn test_parse_startup() {
    match parse_startup(MESSAGE.as_bytes()) {
        Ok(startup) => {
            assert_eq!(startup.len, 78);
            assert_eq!(startup.protocol_version, PROTOCOL_VERSION);
            assert!(startup.payload.is_some());

            match startup.payload {
                Some(list) => {
                    assert_eq!(list,
                        vec!["user", "unexist", "database",
                             "foo", "application_name",
                             "psql", "client_encoding", "UTF8",
                        ]
                    );
                    assert_eq!(list.len(), 8);
                },
                None => unreachable!()
            }
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}