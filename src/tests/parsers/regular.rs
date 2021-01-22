///
/// @package Spotql
///
/// @file Spotql regular parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::regular::{ Regular, regular_parser };
use crate::parsers::parser_error::ParserError;

fn parse_regular(input: &[u8]) -> Result<Regular, ParserError> {
    match regular_parser(input) {
        Ok((_, regular)) => Ok(regular),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}
//
// Startup packet
//

static MESSAGE: &'static str = "N\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}";

#[test]
fn test_parse_regular() {
    match parse_regular(MESSAGE.as_bytes()) {
        Ok(regular) => {
            assert_eq!(regular.tag, 'N');
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}