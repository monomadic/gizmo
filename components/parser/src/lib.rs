//! This crate parses astryx source and emits an AST (abstract syntax tree).
//!
//! There are a few stages to this:
//! 1. Lexical analysis: breaks up the raw text into tokens
//! 2. Parsing: transforms tokens into the AST
//!
//! If you wanted to add or change the syntax (language) of astryx,
//! everything you need is in this crate.
//!
//! ## Usage
//! ```
//! use parser;
//!
//! let source = "page\n";
//! let ast = parser::parse(source).unwrap();
//!
//! ```

use nom::{
    branch::alt, character::complete::multispace1, IResult, sequence::tuple, Err,
};
use nom_locate::LocatedSpan;
use error::ParserErrorKind;
use linesplit::Line;
use models::Statement;

type Span<'a> = LocatedSpan<&'a str>;

pub mod error;
pub mod models;
pub mod parser;
pub mod variable;
pub use crate::error::{ParserError, ParserResult};
pub use crate::parser::Token;
mod eof;
mod linesplit;

// /// returns a nom combinator version of the parser
// pub fn _run(i: &str) -> IResult<Span, Vec<Token>> {
//     // pub fn run<'a, S: Into<&'a str>>(i: S) -> IResult<Span<'a>, Vec<Token>> {
//     tuple((
//         nom::multi::many0(parser::node),
//         alt((eof::eof, multispace1)),
//     ))(Span::new(i))
//     .map(|(r, (a, _))| (r, a))
// }

// pub fn __run(i: &str) -> ParserResult<Vec<Token>> {
//     tuple((
//         nom::multi::many0(parser::node),
//         alt((eof::eof, multispace1)),
//     ))(Span::new(i))
//     .map(|(_, (a, _))| a)
//     .map_err(|e| e.into())
// }

pub fn parse_line<'a>(i: &'a str) -> Result<Statement<'a>, ParserError<Span>> {
    parser::statement(Span::new(i))
        .map(|(r, result)| result)
        .map_err(|e| match e {
            Err::Error(e) | Err::Failure(e) => e,
            Err::Incomplete(_) => unreachable!(),
        })
}

#[test]
fn test_parse_line() {
    assert!(parse_line("func()").is_ok());
    assert!(parse_line("func()").is_ok());
}

// pub fn run(i: &str) -> ParserResult<Span, Span> {
//     linesplit::take_lines(i) // break document up by whitespace indentation
//         .unwrap().1
//         // .map(|(_, lines)| lines) // rem will always be empty as we use cut()
//         // .collect::<Vec<Line>>()
//         .into_iter().map(|line:Line| parse_line(&line.content.to_string()))
//         .collect()
// }

// pub fn run(i: &str) -> ParserResult<Span, ParserError<Span>> {
//     // linesplit::take_lines(i) // break document up by whitespace indentation
//     //     // .map(|(_, lines)| lines) // rem will always be empty as we use cut()
//     //     .map(|(_, lines)| {
//     //         // println!("sending {:?}", line.content);
//     //         lines.iter().map(|line| {
//     //             parser::statement(line.content) // parse a line into a statement
//     //             .map(|(_, token)| token)
//     //             .map_err(|e:Err<_>| e.into())
//     //             // .map_err(|e| Err::convert::<ParserError>(e))
//     //             // .unwrap_err()
//     //             // .map_err(ParserError::from)
                
//     //         }).collect()
//     //     })
//     //     .map_err(|e| e.into())
//     //     // .collect()

// }

// #[test]
// fn test_run() {
//     assert!(run("").is_ok());
//     // assert!(run("page").is_ok());
//     assert!(run("page\n").is_ok());
//     assert!(run("page\n\tdiv\n").is_ok());
//     // assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 1);

//     let result = run("hello\n@@@\n");
//     println!("{:?}", result);

//     // assert!(run("44").is_err());
// }

// pub fn run(i: &str) -> ParserResult<Vec<Token>> {
//     linesplit::take_lines(i) // break document up by whitespace indentation
//         .map(|(_, lines)| lines)? // rem will always be empty as we use cut()
//         .into_iter()
//         .map(|line| {
//             println!("sending {:?}", line.content);
//             parser::node(&line.content.to_string()) // parse a line into a statement
//                 .map(|(_, token)| token)
//                 // .map_err(|e:Err<_>| match e {
//                 //     Err::Error(e) | Err::Failure(e) => e.convert(),
//                 //     Err::Incomplete(_) => Err::convert
//                 // })
//                 // .map_err(|e| Err::convert::<ParserError>(e))
//                 // .unwrap_err()
//                 // .map_err(ParserError::from)
//         })
//         .collect()
// }

// #[test]
// fn test_run() {
//     assert!(run("").is_ok());
//     // assert!(run("page").is_ok());
//     assert!(run("page\n").is_ok());
//     assert!(run("page\n\tdiv\n").is_ok());
//     // assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 1);

//     let result = run("hello\n@@@\n");
//     println!("{:?}", result);

//     // assert!(run("44").is_err());
// }
