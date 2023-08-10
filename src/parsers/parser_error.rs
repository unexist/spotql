///
/// @package Spotql
///
/// @file Spotql parser error
/// @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

#[derive(Debug)]
pub struct ParserError {
    pub message: String
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.message)
    }
}