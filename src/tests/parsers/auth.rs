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

use crate::parsers::auth::{ Auth, auth_parser };
use crate::parsers::parser_error::ParserError;

fn parse_auth(input: &[u8]) -> Result<Auth, ParserError> {
    match auth_parser(input) {
        Ok((_, query)) => Ok(query),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}
//
// Auth message
//

static MESSAGE: &'static str = "p\u{0}\u{0}\u{0}\u{e}test\u{0}";

#[test]
fn test_parse_auth() {
    match parse_auth(MESSAGE.as_bytes()) {
        Ok(auth) => {
            assert_eq!(auth.tag, 'p');
            assert_eq!(auth.payload, Some("test"));
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}