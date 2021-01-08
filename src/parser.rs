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
    alphanumeric1
};

#[derive(Debug)]
pub struct ParserError {
    message: String
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.message)
    }
}

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

named!(verb_parser<&str, Verb>,
    alt!(
        value!(Verb::SELECT, tag!("select"))
        | value!(Verb::UPDATE, tag!("update"))
    )
);

named!(column_name<&str, &str>,
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
        delimited!(
            multispace0,
            alt!(
                do_parse!(
                    single_col: column_name >>
                    (vec![single_col])
                )
                | separated_list0!(
                    tag!(","),
                    column_name
                )
            ),
            multispace0
        )
    )
);

named!(table_parser<&str, &str>,
    complete!(
        delimited!(
            multispace0,
            do_parse!(
                tag!("from") >>
                multispace0 >>
                table: alphanumeric1 >>
                (table)
            ),
            multispace0
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

pub fn parse(input: &str) -> Result<Statement, ParserError> {
    match statement_parser(input) {
        Ok((_, stmt)) => Ok(stmt),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}