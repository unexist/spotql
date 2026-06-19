//!
//! @package Spotql
//!
//! @file Spotql auth parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use nom::IResult;
use nom::combinator::{map, map_res, opt};
use nom::character::complete::{anychar, alphanumeric1};
use nom::number::complete::be_i32;
use nom::Parser;

#[derive(Debug)]
pub struct Auth<'a> {
    pub tag: char,
    pub len: i32,
    pub payload: Option<&'a str>,
}

/* Auth message: char tag | int32 len | payload | \0 */
pub(crate) fn auth_parser(input: &[u8]) -> IResult<&[u8], Auth> {
    map(
        (
            anychar,
            be_i32,
            opt(map_res(alphanumeric1, str::from_utf8)),
        ),
        |(tag, len, payload)| Auth {
            tag,
            len,
            payload,
        }
    ).parse(input)
}
