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

use std::result::Result;
use nom::character::complete::{
    multispace0,
    alphanumeric1,
};

use crate::parsers::parser_error::ParserError;

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
    pub conditions: Option<Vec<&'a str>>,
}

//
// Statement parser
//

named!(verb_parser<&str, Verb>,
    delimited!(
        multispace0,
        alt!(
            value!(Verb::SELECT, tag!("select"))
            | value!(Verb::UPDATE, tag!("update"))
        ),
        multispace0
    )
);

named!(column_name_parser<&str, &str>,
    delimited!(
        multispace0,
        alt!(
            tag!("*")
            | alphanumeric1
        ),
        multispace0
    )
);

named!(column_parser<&str, Vec<&str>>,
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

named!(table_parser<&str, &str>,
    complete!(
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
        )
    )
);

named!(statement_parser<&str, Statement>,
    do_parse!(
        verb: verb_parser >>
        columns: opt!(column_parser) >>
        table: opt!(table_parser) >>
        (Statement {
            verb: verb,
            columns: columns,
            table: table,
            conditions: None,
        })
    )
);

pub fn parse_statement(input: &str) -> Result<Statement, ParserError> {
    match statement_parser(input) {
        Ok((_, stmt)) => Ok(stmt),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}