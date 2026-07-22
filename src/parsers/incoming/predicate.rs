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

use nom::bytes::tag_no_case;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::{IResult, branch::alt, bytes::tag, combinator::value, combinator::complete};
use nom::Parser;
use strum::AsRefStr;

use crate::parsers::incoming::expression::expression_parser;
use crate::parsers::incoming::ws::ws;

#[derive(Debug, PartialEq, Clone, AsRefStr)]
pub enum Operator {
    GREATER,
    EQUAL,
    SMALLER,
    UNEQUAL,
    UNLIKE,
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
    ws(
        alt(
            (
                value(Operator::GREATER, tag(">")),
                value(Operator::EQUAL, tag("=")),
                value(Operator::UNEQUAL, tag("<>")),
                value(Operator::SMALLER, tag("<")),
                value(Operator::UNLIKE, tag("!~")),
            )
        )
    ).parse(input)
}

pub(crate) fn combinator_parser(input: &[u8]) -> IResult<&[u8], Combinator> {
    complete(
        ws(
            alt(
                (
                    value(Combinator::AND, tag_no_case("and")),
                    value(Combinator::OR, tag_no_case("or")),
                )
            )
        )
    ).parse(input)
}

pub(crate) fn predicate_parser(input: &[u8]) -> IResult<&[u8], Predicate<'_>> {
    map(
        (
            expression_parser,
            op_parser,
            expression_parser,
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

pub(crate) fn predicate_list_parser(input: &[u8]) -> IResult<&[u8], Vec<Predicate<'_>>> {
    complete(
        many0(
            complete(
                ws(predicate_parser)
            )
        )
    ).parse(input)
}
