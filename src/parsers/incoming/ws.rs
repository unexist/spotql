//!
//! @package Spotql
//!
//! @file Spotql query parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use nom::Parser;
use nom::bytes::complete::tag_no_case;
use nom::error::{Error, ParseError};
use nom::sequence::delimited;
use nom::character::complete::multispace0;

pub fn btag<'a, E>(s: &'static str) -> impl Parser<&'a [u8], Output = &'a [u8], Error = E>
where
    E: ParseError<&'a [u8]>,
{
    tag_no_case(s.as_bytes())
}

/* https://docs.rs/nom/latest/nom/recipes/index.html#whitespace */
pub fn ws<'a, O, F>(inner: F) -> impl Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>
where
    F: Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>,
{
    delimited(multispace0, inner, multispace0)
}
