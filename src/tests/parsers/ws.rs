//!
//! @package Spotql
//!
//! @file Spotql auth parser tests
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use nom::{Parser, bytes::complete::tag};

use crate::parsers::{incoming::ws::ws, parser_error::ParserError};

fn parse_whitespaced_tag(input: &[u8]) -> Result<&[u8], ParserError> {
    let mut parser = ws(tag(&b"test"[..]));

    match parser.parse(input) {
        Ok((_, tag)) => Ok(tag),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}
static MESSAGE: &'static str = " test ";

#[test]
fn should_parse_tag_with_leading_and_trailing_ws() {
    match parse_whitespaced_tag(MESSAGE.as_bytes()) {
        Ok(_tag) => {
            assert!(true);
        },
        Err(e) => panic!("Error: {}", e),
    }
}
