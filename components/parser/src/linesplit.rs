use crate::Span;
use nom::{
    bytes::complete::take_until,
    character::complete::{newline, one_of},
    combinator::cut,
    multi::many0,
    sequence::tuple,
    IResult,
};

/// whitespace significant linesplit
/// see: map_while

#[derive(Debug)]
pub struct Line<'a> {
    pub content: Span<'a>,
    pub children: Vec<Line<'a>>,
}

pub(crate) fn take_lines(i: &str) -> IResult<Span, Vec<Line>> {
    cut(many0(take_children))(Span::new(i))
}

#[test]
fn test_take_lines() {
    assert!(take_lines("").is_ok());
    // let (r, lines) = take_lines("").unwrap();
    // println!("{:#?}", take_lines("a\n\tb\nc\n\td\n"));
    // assert_eq!(take_children(Span::new(""))?.1.content, Span::new(""));
}

fn take_children(i: Span) -> IResult<Span, Line> {
    let (mut r, (indent, line)) = line(i)?;
    let mut children: Vec<Line> = Vec::new();

    // see map_while
    while line_indent(r.fragment()) > indent {
        let (rem, child) = take_children(r)?;
        children.push(child);
        r = rem;
    }

    Ok((
        r,
        Line {
            content: line,
            children,
        },
    ))
}

/// take a single line in the format (indent, content) and chomp newline
fn line(i: Span) -> IResult<Span, (usize, Span)> {
    tuple((
        nom::multi::many0_count(one_of(" \t")),
        take_until("\n"),
        newline,
    ))(i)
    .map(|(r, (indent, line, _))| (r, (indent, line)))
}

fn take_line(i: Span) -> IResult<Span, (usize, Span)> {
    take_until("\n")(i).map(|(r, line)| (r, (line_indent(line.fragment()), line)))
}

/// returns the position of the first non-whitespace character,
/// or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

fn line_indent(i: &str) -> usize {
    indentation_level(i).map(|(_r, indent)| indent).unwrap_or(0)
}