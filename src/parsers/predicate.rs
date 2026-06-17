//!
//! @package Spotql
//!
//! @file Spotql condition parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use nom::combinator::{map, opt};
use nom::{IResult, branch::alt, bytes::tag, character::complete::multispace0, combinator::value, sequence::delimited};
use nom::Parser;

use crate::parsers::column::{column_name_parser};

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    GREATER,
    EQUAL,
    SMALLER,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Combinator {
    AND,
    OR,
}

#[derive(Debug)]
pub struct Predicate<'a> {
    pub left_hand: &'a str,
    pub op: Operator,
    pub right_hand: &'a str,
    pub combinator: Option<Combinator>,
}

//
// Condition parser
//

pub(crate) fn op_parser(input: &[u8]) -> IResult<&[u8], Operator> {
    delimited(
        multispace0,
        alt(
            (
                value(Operator::GREATER, tag(">")),
                value(Operator::EQUAL, tag("=")),
                value(Operator::SMALLER, tag("<")),
            )
        ),
        multispace0
    ).parse(input)
}

pub(crate) fn combinator_parser(input: &[u8]) -> IResult<&[u8], Combinator> {
    delimited(
        multispace0,
        alt(
            (
                value(Combinator::AND, tag("and")),
                value(Combinator::OR, tag("or")),
            )
        ),
        multispace0
    ).parse(input)
}

pub(crate) fn predicate_parser(input: &[u8]) -> IResult<&[u8], Predicate> {
    map(
        (
            column_name_parser,
            op_parser,
            column_name_parser,
            opt(combinator_parser),
        ),
        |(left_hand, op, right_hand, combinator)| Predicate {
            left_hand,
            op,
            right_hand,
            combinator,
        }
    ).parse(input)
}
