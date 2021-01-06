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

use nom::IResult;

use std::str::from_utf8;
use std::fmt;
use std::result::Result;

#[derive(Debug)]
pub struct ParserError {
    data: String
}

#[derive(Debug)]
pub struct Statement<'a> {
    pub verb: &'a str,
    pub columns: Vec<&'a str>,
    pub table: &'a str,
    pub conditions: Vec<&'a str>
}

named!(word_parser<&str, &str>, take_until!(" "));

named!(statement_parser<&str, Statement>,
    do_parse!(
        verb: word_parser >>
        (Statement {
            verb: verb,
            columns: vec![],
            table: "",
            conditions: vec![]
        })
    )
);

pub fn parse(input: &str) -> Result<Statement, ParserError> {
    match statement_parser(input) {
        Ok((_, stmt)) => Ok(stmt),
        Err(e) => Err(ParserError {
            data: format!("Incomplete {:?}", e)
        })
    }
}