///
/// @package Spotql
///
/// @file Spotql condition parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use nom::character::complete::{
    multispace0,
};

use crate::parsers::statement::{ column_name_parser };

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

named!(op_parser<&[u8], Operator>,
    delimited!(
        multispace0,
        alt!(
            value!(Operator::GREATER, tag!(">"))
            | value!(Operator::EQUAL, tag!("="))
            | value!(Operator::SMALLER, tag!("<"))
        ),
        multispace0
    )
);

named!(combinator_parser<&[u8], Combinator>,
    complete!(
        delimited!(
            multispace0,
            alt!(
                value!(Combinator::AND, tag!("and"))
                | value!(Combinator::OR, tag!("or"))
            ),
            multispace0
        )
    )
);

named!(pub predicate_parser<&[u8], Predicate>,
    dbg_dmp!(
        do_parse!(
            left_hand: column_name_parser >>
            op: op_parser >>
            right_hand: column_name_parser >>
            combinator: opt!(combinator_parser) >>
            (Predicate {
                left_hand: left_hand,
                op: op,
                right_hand: right_hand,
                combinator: combinator,
            })
        )
    )
);