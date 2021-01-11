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
use nom::number::complete::{
    le_i32
};

use crate::parsers::parser_error::ParserError;

#[derive(Debug)]
pub struct Startup<'a> {
    pub protocol_version: i32,
    pub len: i32,
    pub payload: Option<&'a str>,
}

/* Startup package: int32 len | int32 protocol | payload */
named!(startup_parser<&[u8], Startup>,
    do_parse!(
        len: le_i32 >>
        version: le_i32 >>
        (Startup {
            len: len,
            protocol_version: version,
            payload: None,
        })
    )
);

pub fn parse_startup(input: &[u8]) -> Result<Startup, ParserError> {
    match startup_parser(input) {
        Ok((_, startup)) => Ok(startup),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}