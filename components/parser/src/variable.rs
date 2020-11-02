use crate::{error::ParserErrorKind, Literal, ParserError, Span};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::map,
    number::complete::double,
    sequence::{delimited, tuple},
    IResult,
};

// // should this actually be called a reference? probably....

// // pub(crate) fn variable<'a>(i: Span<'a>) -> IResult<Span<'a>, Variable<'a>, ParserError<Span<'a>>> {
// //     alt((
// //         // map(hash, JsonValue::Object),
// //         // map(array, JsonValue::Array),
// //         // map(quoted_string, |s: Span| Variable::QuotedString(s)),
// //         map(relative_path, |s: Span| Variable::RelativePath(s)),
// //         map(alphanumeric1, |s: Span| Variable::Reference(s)),
// //         // map(argument_idx,   |i| Property::ArgumentIndex(i.parse::<usize>().unwrap())),
// //         // map(double,         |f| Property::Float(f)),
// //         // map(digit1,         |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
// //         // map(boolean,        |b| Property::Boolean(b)),
// //         // map(dotted_symbol,  |s| Property::DottedSymbol(String::from(s))),
// //         // map(symbol,         |s| Property::Symbol(String::from(s))),
// //     ))(i)
// //     .map_err(|e| {
// //         e.map(|(s, _k)| ParserError {
// //             context: i, // we need to reset the context to the whole line
// //             kind: ParserErrorKind::UnexpectedToken("variable".into()),
// //             pos: s,
// //         })
// //     })
// // }

pub(crate) fn literal<'a>(i: Span<'a>) -> IResult<Span<'a>, Literal<'a>, ParserError<Span<'a>>> {
    alt((
        // map(hash, JsonValue::Object),
        // map(array, JsonValue::Array),
        map(quoted_string, |s: Span| Literal::String(s)),
        // map(relative_path, |s: Span| Variable::RelativePath(s)),
        // map(alphanumeric1, |s: Span| Variable::Reference(s)),
        // map(argument_idx,   |i| Property::ArgumentIndex(i.parse::<usize>().unwrap())),
        map(double, |f| Literal::Float(i, f)),
        // map(digit1,         |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
        // map(boolean,        |b| Property::Boolean(b)),
        // map(dotted_symbol,  |s| Property::DottedSymbol(String::from(s))),
        // map(symbol,         |s| Property::Symbol(String::from(s))),
    ))(i)
    .map_err(|e| {
        e.map(|(s, _k)| ParserError {
            context: i, // we need to reset the context to the whole line
            kind: ParserErrorKind::UnexpectedToken("variable".into()),
            pos: s,
        })
    })
}

fn quoted_string(i: Span) -> IResult<Span, Span> {
    delimited(char('\"'), is_not("\""), char('\"'))(i)
}

/// match relative paths eg: ./test.txt and ../../test.txt
pub fn relative_path<'a>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>, ParserError<Span<'a>>> {
    tuple((path_prefix, path_characters))(i)
        // .map(|(r, (prefix, pathname))| (r, Span::new(&format!("{}{}", prefix, pathname)))) // check this!
        .map(|(r, (_prefix, path))| (r, path)) // fix this so that prefix is included
        .map_err(|e| {
            e.map(|(s, _k)| ParserError {
                context: i, // we need to reset the context to the whole line
                kind: ParserErrorKind::UnexpectedToken("gg".into()),
                pos: s,
            })
        })
}

fn path_characters(i: Span) -> IResult<Span, Span> {
    nom::bytes::complete::is_a("./*-_abcdefghijklmnopqrstuvwxyz1234567890ABCDEF")(i)
}

// match path prefixes ./ or ../
fn path_prefix(i: Span) -> IResult<Span, Span> {
    alt((tag("./"), tag("../")))(i)
}

impl Literal<'_> {
    pub fn inspect(&self) -> String {
        match self {
            Literal::String(s) => format!("\"{}\"", s.fragment().to_string()),
            Literal::Float(_, f) => f.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_literal() {
        assert_eq!(
            literal(Span::new("4")).unwrap().1.inspect(),
            String::from("4")
        );
    }
}
