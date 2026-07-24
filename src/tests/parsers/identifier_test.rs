 //!
 //! @package Spotql
 //!
 //! @file Spotql case parser tests
 //! @copyright (c) 2026-present Christoph Kappel <christoph@unexist.dev>
 //! @version $Id$
 //!
 //! This program can be distributed under the terms of the GNU GPLv2.
 //! See the file LICENSE for details.
 //!

 use crate::parsers::{incoming::column::identifier_parser, parser_error::ParserError};

 fn parse_identifier(input: &[u8]) -> Result<&str, ParserError> {
     match identifier_parser(input) {
         Ok((_, expr)) => Ok(expr),
         Err(e) => Err(ParserError {
             message: e.to_string()
         })
     }
 }
 //
 // Expression message
 //

static MESSAGE: &'static str = "unexist";

 #[test]
 fn should_parse_identifier() {
     match parse_identifier(MESSAGE.as_bytes()) {
         Ok(_expr) => {
             assert!(true);
         },
         Err(e) => panic!("Error: {}", e),
     }
 }
