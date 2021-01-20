///
/// @package Spotql
///
/// @file Spotql startup parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use std::result::Result;
use nom::number::Endianness;

use crate::parsers::parser_error::ParserError;

#[derive(Debug)]
pub struct Regular<'a> {
    pub tag: char,
    pub len: i32,
    pub payload: Option<&'a str>,
}

/* Regular package: char tag | int32 len | payload | \0 */
named!(regular_parser<&[u8], Regular>,
    dbg_dmp!(
        do_parse!(
            tag: i32!(Endianness::Big) >>
            len: i32!(Endianness::Big) >>
            (Regular {
                tag: 'N', //std::char::from_u32(tag as u32).unwrap(),
                len: len,
                payload: None,
            })
        )
    )
);

pub fn parse_regular(input: &[u8]) -> Result<Regular, ParserError> {
    match regular_parser(input) {
        Ok((_, regular)) => Ok(regular),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}