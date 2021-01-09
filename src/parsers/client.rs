///
/// @package Spotql
///
/// @file Spotql client parser
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
pub struct Client<'a> {
    pub protocol_version: i32,
    pub username: Option<&'a str>,
    pub database: Option<&'a str>,
}

/* Startup package: int32 len | int32 protocol | str name | \0 str value | ... | \0 */
named!(client_parser<&[u8], Client>,
    do_parse!(
        len: le_i32 >>
        version: le_i32 >>
        values: many0!(
            terminated!(take_while!(|b: u8| b != 0), tag!([0]))
        ) >>
        (Client {
            protocol_version: version,
            username: None,
            database: None,
        })
    )
);

pub fn parse_client(input: &[u8]) -> Result<Client, ParserError> {
    match client_parser(input) {
        Ok((_, client)) => Ok(client),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}