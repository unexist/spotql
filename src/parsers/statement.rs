///
/// @package Spotql
///
/// @file Spotql parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use std::str;
use nom::character::complete::{
    multispace0,
    alphanumeric1,
};

use crate::parsers::predicate::{ Predicate, predicate_parser };
use crate::parsers::column::{ Column, column_list_parser };

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

named!(verb_parser<&[u8], Verb>,
    delimited!(
        multispace0,
        alt!(
            value!(Verb::SELECT, tag!("select"))
            | value!(Verb::UPDATE, tag!("update"))
        ),
        multispace0
    )
);

named!(table_parser<&[u8], &str>,
    complete!(
        map_res!(
            preceded!(
                delimited!(
                    multispace0,
                    tag!("from"),
                    multispace0
                ),
                delimited!(
                    multispace0,
                    alphanumeric1,
                    multispace0
                )
            ), str::from_utf8
        )
    )
);

named!(predicate_list_parser<&[u8], Vec<Predicate>>,
    complete!(
        preceded!(
            delimited!(
                multispace0,
                tag!("where"),
                multispace0
            ),
            many1!(
                complete!(predicate_parser)
            )
        )
    )
);

named!(pub statement_parser<&[u8], Statement>,
    do_parse!(
        verb: verb_parser >>
        columns: opt!(column_list_parser) >>
        table: opt!(table_parser) >>
        predicates: opt!(predicate_list_parser) >>
        tag!(";") >>
        (Statement {
            verb: verb,
            columns: columns,
            table: table,
            predicates: predicates,
        })
    )
);