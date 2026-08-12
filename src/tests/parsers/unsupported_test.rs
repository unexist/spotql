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

 use crate::parsers::incoming::unsupported::{unsupported_parser};
 use crate::parsers::parser_error::ParserError;

 fn parse_unsupported(input: &[u8]) -> Result<bool, ParserError> {
     match unsupported_parser(input) {
         Ok((_, valid_but_unsupported)) => Ok(valid_but_unsupported),
         Err(e) => Err(ParserError {
             message: e.to_string()
         })
     }
 }

 //
 // Case message
 //

static MESSAGE1: &'static str = "CASE c.relkind END";

 #[test]
 fn should_parse_unsupported_case() {
     match parse_unsupported(MESSAGE1.as_bytes()) {
         Ok(valid_but_unsupported) => {
             assert!(valid_but_unsupported);
         },
         Err(e) => panic!("Error: {}", e),
     }
 }


 //
 // Join message
 //

static MESSAGE2: &'static str = "LEFT JOIN album.name ON album.id = track.album_id";

 #[test]
 fn should_parse_unsupported_join() {
     match parse_unsupported(MESSAGE2.as_bytes()) {
         Ok(valid_but_unsupported) => {
             assert!(valid_but_unsupported);
         },
         Err(e) => panic!("Error: {}", e),
     }
 }
