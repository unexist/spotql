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
use nom::error::Error;
use nom::sequence::delimited;
use nom::character::complete::multispace0;

/* https://docs.rs/nom/latest/nom/recipes/index.html#whitespace */
pub fn ws<'a, O, F>(
    inner: F,
) -> impl Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>
where
    F: Parser<&'a [u8], Output = O, Error = Error<&'a [u8]>>,
{
    delimited(multispace0, inner, multispace0)
}
