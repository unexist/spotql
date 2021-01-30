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

#[derive(Debug, PartialEq, Clone)]
pub enum Verb {
    SELECT,
    UPDATE,
}

#[derive(Debug)]
pub struct Statement<'a> {
    pub verb: Verb,
    pub columns: Option<Vec<&'a str>>,
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

named!(pub column_name_parser<&[u8], &str>,
    map_res!(
        delimited!(
            multispace0,
            alt!(
                tag!("*")
                | alphanumeric1
            ),
            multispace0
        ), str::from_utf8
    )
);

named!(column_parser<&[u8], Vec<&str>>,
    complete!(
        dbg_dmp!(
            separated_list0!(
                complete!(
                    delimited!(
                        multispace0,
                        tag!(","),
                        multispace0
                    )
                ),
                column_name_parser
            )
        )
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
            dbg_dmp!(
                many1!(
                    complete!(predicate_parser)
                )
            )
        )
    )
);

named!(pub statement_parser<&[u8], Statement>,
    do_parse!(
        verb: verb_parser >>
        columns: opt!(column_parser) >>
        table: opt!(table_parser) >>
        predicates: opt!(predicate_list_parser) >>
        (Statement {
            verb: verb,
            columns: columns,
            table: table,
            predicates: predicates,
        })
    )
);