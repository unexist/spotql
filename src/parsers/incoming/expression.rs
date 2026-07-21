//!
//! @package Spotql
//!
//! @file Spotql expression parser
//! @copyright (c) 2026-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use nom::{IResult, character::complete::{
    alphanumeric1, multispace0
}, combinator::map_res, sequence::delimited};
use nom::Parser;

pub(crate) fn expression_parser(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        delimited(
            multispace0,
            alphanumeric1,
            multispace0,
        ), str::from_utf8
    ).parse(input)
}
