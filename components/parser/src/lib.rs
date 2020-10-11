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

use nom_locate::LocatedSpan;
use linesplit::Line;

use rctree::Node;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type ParserResult<T,I> = Result<T, ParserError<I>>;

pub mod error;
pub mod models;
pub mod statement;
// pub mod variable;
pub use crate::error::ParserError;
pub use crate::models::*;
mod linesplit;
mod element;
mod function;

// pub fn parse_line<'a>(i: Span<'a>) -> Result<Statement<'a>, ParserError<Span>> {
//     statement::statement(i)
//         .map(|(_r, result)| result)
//         .map_err(|e| match e {
//             Err::Error(e) | Err::Failure(e) => e,
//             Err::Incomplete(_) => unreachable!(),
//         })
// }

// #[test]
// fn test_parse_line() {
//     assert!(parse_line(Span::new("func()")).is_ok());
//     // assert!(parse_line(Span::new("func(a:1)")).is_ok());
//     assert!(parse_line(Span::new("func()\nfunc()")).is_err());
// }

pub fn run<'a>(i: &'a str) -> Result<Vec<Node<Statement<'a>>>, ParserError<Span<'a>>> {
    let (_, lines): (_, Vec<Line>) = linesplit::take_lines(i).expect("linesplit fail (fix)"); // break document up by whitespace indentation

    lines
        .into_iter()
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line<'a>(line: Line<'a>) -> Result<Node<Statement<'a>>, ParserError<Span<'a>>> {
    let (_, statement) = statement::statement(line.content).unwrap(); // fix this

    let mut node: Node<Statement> = Node::new(statement);

    for child in line.children {
        println!("line {:?}", &child);
        let (_, statement) = statement::statement(child.content).unwrap();
        println!("statement {:?}", &statement);
        node.append(Node::new(statement));
    }
    
    Ok(node)
}

#[test]
fn test_run() {
    assert!(run("").is_ok());
    // assert!(run("page").is_ok());
    assert!(run("page()\n").is_ok());
    assert!(run("page()\ndiv()\n").is_ok());
    // assert!(run("page()\n\tdiv\n").is_err()); // children
    // assert_eq!(run("page\n\n\n").unwrap().0.get_column(), 1);

    // let result = run("hello\n@@@\n");
    // println!("{:?}", result);

    // println!("--{:?}", run("func()\n"));

    // assert!(run("44").is_err());
}
