// use std::error::Error;
// use std::fmt;

use crate::Span;
use nom::{
    error::{ErrorKind, ParseError},
};

#[derive(Debug)]
pub struct ParserError<I> {
    pub kind: ParserErrorKind<I>,
    pub pos: I,
    pub context: I,
}

// #[derive(Debug)]
// pub struct Position {
//     pub line: u32,
//     pub column: usize,
//     pub offset: usize,
// }

impl <'a>ParseError<Span<'a>> for ParserError<Span<'a>> {
    fn from_error_kind(input: Span<'a>, _kind: ErrorKind) -> Self {
        // panic!("incoming: {:?}", input);
        ParserError {
            kind: ParserErrorKind::Unhandled,
            pos: input,
            context: input,
        }
    }

    fn append(_i: Span<'a>, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

// impl<I:std::fmt::Debug> ParseError<I> for ParserError<I> {
//     fn from_error_kind(input: I, _kind: ErrorKind) -> Self {
//         panic!("incoming: {:?}", input);
//         ParserError {
//             kind: ParserErrorKind::Unhandled,
//             pos: Position {
//                 column: 0,
//                 line: 0,
//                 offset: 0,
//             },
//             context: input,
//         }
//         // panic!("unhandled error {:?} {:?}", input, kind);
//     }

//     fn append(_i: I, _kind: ErrorKind, other: Self) -> Self {
//         other
//     }

//     fn from_char(input: I, _: char) -> Self {
//         // panic!("from_char");
//         Self::from_error_kind(input, ErrorKind::Char)
//     }

//     fn or(self, other: Self) -> Self {
//         // panic!("or");
//         other
//     }
// }

// impl<I:std::fmt::Display> ContextError<I> for ParserError<I> {
//     fn add_context(input: I, ctx: &'static str, other: Self) -> Self {
//         panic!("add_context: {}{}", ctx, input);
//         other
//     }
// }

// this is selfish and a perf hit, but I don't want to expose Span
// it's not that bad as these aren't heap allocated
// impl<'a> From<Span<'a>> for Position {
//     fn from(span: Span) -> Position {
//         Position {
//             line: span.location_line(),
//             offset: span.location_offset(),
//             column: span.get_column(),
//         }
//     }
// }

// impl From<Span<'_>> for Position {
//     fn from(span: Span) -> Position {
//         Position {
//             line: span.location_line(),
//             offset: span.location_offset(),
//             column: span.get_column(),
//         }
//     }
// }

// used by the main api interface, run()
// impl <I>From<I> for ParserError<I> {
//     fn from(err: Err<(I, nom::error::ErrorKind)>) -> ParserError<I> {
//         // println!("err: {:?}", err);
//         match err {
//             Err::Error((span, _kind)) | Err::Failure((span, _kind)) => ParserError {
//                 context: "span",
//                 kind: ParserErrorKind::SyntaxError,
//                 pos: span.into(),
//             },
//             nom::Err::Incomplete(_) => unreachable!(),
//         }
//     }
// }

// impl From<(nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)> for ParserError<&str> {
//     fn from(err: (nom_locate::LocatedSpan<&str>, nom::error::ErrorKind)) -> ParserError<&str> {
//         let (span, kind) = err;
//         ParserError {
//             context: span.to_string(),
//             kind: ParserErrorKind::SyntaxError,
//             pos: span.into(),
//         }
//     }
// }

// impl ParserError {
//     pub fn from_nom_error(err: nom::Err))
//     pub fn new(_msg: &str) -> Self {
//         unimplemented!()
//     }
// }

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind<I> {
    SyntaxError,
    FunctionArgumentError,
    Unhandled,
    UnexpectedToken(String),
    ExpectedValue,
    Nom(I),
}

// impl Error for ParseError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }
