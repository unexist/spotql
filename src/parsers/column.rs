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
use nom::{IResult, branch::alt, bytes::tag, character::complete::{
    alphanumeric1, multispace0
}, combinator::{complete, map, map_res, opt}, multi::separated_list0, sequence::{delimited, preceded}};
use nom::Parser;

#[derive(Debug)]
pub struct Column<'a> {
    pub name: &'a str,
    pub alias: Option<&'a str>,
}

//
// Column parser
//

pub(crate) fn column_name_parser(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        delimited(
            multispace0,
            alt(
                (
                    tag("*"),
                    alphanumeric1
                )
            ),
            multispace0
        ), str::from_utf8
    ).parse(input)
}

pub(crate) fn column_parser(input: &[u8]) -> IResult<&[u8], Column<'_>> {
    map(
        (
            column_name_parser,
            opt(
                complete(
                    preceded(
                        delimited(
                            multispace0,
                            tag("as"),
                            multispace0,
                        ),
                        column_name_parser
                    ),
                )
            )
        ),
        |(name, alias)| Column {
            name,
            alias,
        }
    ).parse(input)
}

pub(crate) fn column_list_parser(input: &[u8]) -> IResult<&[u8], Vec<Column<'_>>> {
    complete(
        separated_list0(
            complete(
                delimited(
                    multispace0,
                    tag(","),
                    multispace0,
                )
            ), column_parser
    )).parse(input)
}
