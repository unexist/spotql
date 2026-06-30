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
use nom::branch::alt;
use nom::bytes::tag;
use nom::character::complete::{
    multispace0,
    alphanumeric1,
};
use nom::combinator::{complete, map, map_res, opt, value};
use nom::multi::many1;
use nom::{IResult, Parser};
use nom::sequence::{delimited, preceded};

use crate::parsers::incoming::predicate::{Predicate, predicate_parser};
use crate::parsers::incoming::column::{Column, column_list_parser};

#[derive(Debug, PartialEq, Clone)]
pub enum Verb {
    SELECT,
    UPDATE,
}

#[derive(Debug)]
pub struct Statement<'a> {
    pub verb: Verb,
    pub columns: Option<Vec<Column<'a>>>,
    pub table: Option<&'a str>,
    pub predicates: Option<Vec<Predicate<'a>>>,
}

//
// Statement parser
//

pub(crate) fn verb_parser(input: &[u8]) -> IResult<&[u8], Verb> {
    delimited(
        multispace0,
        alt(
            (
                value(Verb::SELECT, tag("select")),
                value(Verb::UPDATE, tag("update")),
            )
        ),
        multispace0
    ).parse(input)
}

pub(crate) fn table_parser(input: &[u8]) -> IResult<&[u8], &str> {
    complete(
        map_res(
            preceded(
                delimited(
                    multispace0,
                    tag("from"),
                    multispace0
                ),
                delimited(
                    multispace0,
                    alphanumeric1,
                    multispace0
                )
            ), str::from_utf8
        )
    ).parse(input)
}

pub(crate) fn predicate_list_parser(input: &[u8]) -> IResult<&[u8], Vec<Predicate<'_>>> {
    complete(
        preceded(
            delimited(
                multispace0,
                tag("where"),
                multispace0
            ),
            many1(
                complete(predicate_parser)
            )
        )
    ).parse(input)
}

pub(crate) fn statement_parser(input: &[u8]) -> IResult<&[u8], Statement<'_>> {
    map(
        (
            verb_parser,
            opt(column_list_parser),
            opt(table_parser),
            opt(predicate_list_parser),
            tag(";")
        ),
        |(verb, columns, table, predicates, _)| Statement {
            verb,
            columns,
            table,
            predicates,
        }
    ).parse(input)
}
