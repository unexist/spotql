//!
//! @package Spotql
//!
//! @file Spotql startup parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use std::collections::HashMap;
use std::str::from_utf8;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::is_a;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::number::complete::be_i32;

#[derive(Debug)]
pub struct Startup<'a> {
    pub len: i32,
    pub protocol_version: i32,
    pub parameters: Option<HashMap<&'a str, &'a str>>,
}

/* Startup message: int32 len | int32 protocol version | key \0 | value \0 | \0 */
pub(crate) fn startup_parser(input: &[u8]) -> IResult<&[u8], Startup<'_>> {
    map(
        (
            be_i32,
            be_i32,
            opt(
                separated_list0(
                    char('\0'),
                    is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_01234567890"),
                )
            )
        ),
        |(len, protocol_version, payload)| Startup {
            len,
            protocol_version,
            parameters: if let Some(list) = payload {
                let mut params: HashMap<&str, &str> = HashMap::new();
                let mut iter = list.iter();

                while let Some(key) = iter.next() {
                    if let Some(value) = iter.next() {
                        params.insert(
                            from_utf8(*key).expect("Conversion from u8 failed"),
                            from_utf8(*value).expect("Conversion from u8 failed"));
                    }
                }

                Some(params)
            } else {
                None
            },
        }
    ).parse(input)
}
