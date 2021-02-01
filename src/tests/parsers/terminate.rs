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

use crate::parsers::terminate::{ Terminate, terminate_parser };
use crate::parsers::parser_error::ParserError;

fn parse_terminate(input: &[u8]) -> Result<Terminate, ParserError> {
    match terminate_parser(input) {
        Ok((_, term)) => Ok(term),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}
//
// Terminate message
//

static MESSAGE: &'static str = "X\u{0}\u{0}\u{0}\u{0}";

#[test]
fn test_parse_terminate() {
    match parse_terminate(MESSAGE.as_bytes()) {
        Ok(term) => {
            assert_eq!(term.tag, 'X');
            assert_eq!(term.len, 0);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}