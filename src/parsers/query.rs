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

use nom::combinator::map;
use nom::{IResult, combinator::opt};
use nom::character::complete::anychar;
use nom::Parser;

use crate::parsers::statement::{Statement, statement_parser};

#[derive(Debug)]
pub struct Query<'a> {
    pub tag: char,
    pub len: i32,
    pub statement: Option<Statement<'a>>,
}

/* Auth message: char tag | int32 len | payload | \0 */
pub(crate) fn query_parser(input: &[u8]) -> IResult<&[u8], Query<'_>> {
    map(
        (
            anychar,
            nom::character::complete::i32,
            opt(statement_parser),
        ),
        |(tag, len, statement)| Query {
            tag,
            len,
            statement,
        }
    ).parse(input)
}
