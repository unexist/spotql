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
use nom::character::complete::{
    alphanumeric1,
};
use nom::number::Endianness;

use crate::parsers::parser_error::ParserError;

#[derive(Debug)]
pub struct Startup<'a> {
    pub protocol_version: i32,
    pub len: i32,
    pub payload: Option<Vec<&'a str>>,
}

/* Startup package: int32 len | int32 protocol version | key \0 | value \0 | \0 */
named!(startup_parser<&[u8], Startup>,
    dbg_dmp!(
        do_parse!(
            len: i32!(Endianness::Big) >>
            version: i32!(Endianness::Big) >>
            payload: opt!(
                terminated!(
                    separated_list0!(
                        tag!([0]),
                        alphanumeric1
                    ),
                    tag!([0])
                )
             ) >>
            (Startup {
                len: len,
                protocol_version: version,
                payload: if let Some(list) = payload {
                    println!("{:?}", list);

                    Some(list.into_iter().map(|s| from_utf8(s).unwrap()).collect())
                } else {
                    None
                },
            })
        )
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