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
use nom::{AsChar, IResult, branch::alt, bytes::{complete::take_while1, tag}, character::complete::alphanumeric1, combinator::{complete, map, map_res, opt}, multi::separated_list0, sequence::{delimited, preceded, terminated}
};
use nom::Parser;

use crate::parsers::incoming::common::{btag, ws};

#[derive(Debug)]
pub struct Column<'a> {
    pub table: Option<&'a str>,
    pub name: &'a str,
    pub alias: Option<&'a str>,
}

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

//
// Column parser
//

pub(crate) fn column_name_parser(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        ws(
            alt(
                (
                    btag("*"),
                    alphanumeric1
                )
            ),
        ), str::from_utf8
    ).parse(input)
}

pub(crate) fn column_parser(input: &[u8]) -> IResult<&[u8], Column<'_>> {
    map((
        opt(
            terminated(
                identifier_parser,
                tag("."),
            )
        ),
        identifier_parser,
        opt(
            complete(
                preceded(
                    ws(btag("as")),
                    identifier_parser
                ),
            )
        )),
        |(table, name, alias)| Column {
            table,
            name,
            alias,
        }
    ).parse(input)
}

pub(crate) fn column_list_parser(input: &[u8]) -> IResult<&[u8], Vec<Column<'_>>> {
    complete(
        separated_list0(
            ws(btag(",")),
            column_parser
    )).parse(input)
}
