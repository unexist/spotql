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
use std::str::from_utf8;
use nom::number::complete::{
    le_i32
};
use nom::character::complete::{
    anychar
};

use crate::parsers::parser_error::ParserError;

#[derive(Debug)]
pub struct Regular<'a> {
    pub tag: char,
    pub len: i32,
    pub payload: Option<&'a str>,
}

/* Regular package: char tag | int32 len | payload | \0 */
named!(regular_parser<&[u8], Regular>,
    do_parse!(
        tag: anychar >>
        len: le_i32 >>
        payload: opt!(map_res!(
            terminated!(take_while!(|b: u8| b != 0), tag!([0])), from_utf8
        )) >>
        (Regular {
            tag: tag,
            len: len,
            payload: payload,
        })
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