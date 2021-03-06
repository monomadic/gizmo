use crate::{statement::expression, Expression, ParserError, Span, StringToken};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    character::complete::multispace0,
    combinator::map,
    IResult,
};

pub(crate) fn piped_string<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<StringToken>, ParserError<Span<'a>>> {
    nom::sequence::tuple((tag("| "), tokenised_string))(i).map(|(r, (_, value))| (r, value))
}

#[test]
fn test_piped_string() {
    assert!(piped_string(Span::new_extra("", "")).is_err());
    assert!(piped_string(Span::new_extra("| ", "")).is_ok());
    assert!(piped_string(Span::new_extra("| hi", "")).is_ok());
}

pub(crate) fn tokenised_string<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<StringToken>, ParserError<Span<'a>>> {
    nom::multi::many0(alt((
        map(interpolated_expression, |expr| {
            StringToken::Expression(expr)
        }),
        map(raw_text, |s| StringToken::Text(s)),
    )))(i)
}

fn raw_text<'a>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>, ParserError<Span<'a>>> {
    is_not("\n")(i)
}

fn interpolated_expression<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, Expression<'a>, ParserError<Span<'a>>> {
    nom::sequence::tuple((
        multispace0,
        tag("${"),
        multispace0,
        expression,
        multispace0,
        char('}'),
    ))(i)
    .map(|(r, (_, _, _, var, _, _))| (r, var))
    // .map_err(|e| {
    //     e.map(|(s, _k)| ParserError {
    //         context: i, // we need to reset the context to the whole line
    //         kind: ParserErrorKind::UnexpectedToken(s.fragment().to_string()),
    //         pos: s,
    //     })
    // })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpolated_expression() {
        assert!(interpolated_expression(Span::new_extra("", "")).is_err());
    }
}
