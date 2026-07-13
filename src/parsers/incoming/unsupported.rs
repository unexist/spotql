//!
//! @package Spotql
//!
//! @file Spotql query parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use std::collections::HashMap;
use nom::bytes::tag;
use nom::combinator::map;
use nom::number::complete::be_i32;
use nom::{IResult, combinator::opt};
use nom::character::complete::anychar;
use nom::Parser;

use crate::parsers::incoming::predicate::Predicate;

/* Case statement: case expr when expr then expr end */
pub(crate) fn unsupported_case_parser(input: &[u8]) -> IResult<&[u8], bool> {
    complete(
        (
            tag("case"),
            anychar,
            opt(statement_parser),
        ),
    ).parse(input)
}

pub(crate) fn unsupported_parser(input: &[u8]) -> IResult<&[u8], bool> {
    map(
        (
            anychar,
            be_i32,
        ),
        |(tag, len)| Terminate {
            tag,
            len
        }
    ).parse(input)
}
