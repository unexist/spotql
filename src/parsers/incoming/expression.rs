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
    alphanumeric1
}, combinator::map_res};
use nom::Parser;

use crate::parsers::incoming::ws::ws;

pub(crate) fn expression_parser(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        ws(alphanumeric1), str::from_utf8
    ).parse(input)
}
