//!
//! @package Spotql
//!
//! @file Spotql parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use std::str;
use nom::{AsChar, IResult, branch::alt, bytes::{complete::take_while1}, combinator::map_res, sequence::delimited
};
use nom::Parser;

use crate::parsers::incoming::common::{btag, ws};

#[inline]
fn is_sql_identifier(chr: u8) -> bool {
    AsChar::is_alphanum(chr) || '_' as u8 == chr || '@' as u8 == chr
}

pub(crate) fn identifier_parser(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        ws(
            alt((
                btag("*"),
                delimited(btag("`"), take_while1(is_sql_identifier), btag("`")),
                delimited(btag("["), take_while1(is_sql_identifier), btag("]")),
                take_while1(is_sql_identifier),
            ))
        ), str::from_utf8
    ).parse(input)
}
