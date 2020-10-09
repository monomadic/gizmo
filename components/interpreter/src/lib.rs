//! This crate takes an ast and emits a series of document trees.
//! 
//! It is responsible for:
//! - reading referenced files
//! - resolving variables and references into values
//! - executing functions
//!

use arguments::{NamedArguments, TypeGetters};
use parser::{parser::Attribute, Token};
use rctree::Node;
use state::State;
use std::collections::HashMap;
use value::{Document, Value};
use error::{InterpreterError, InterpreterResult};
use html::{new_node_with_text, HTMLNode, HTMLElement};

mod arguments;
mod state;
mod value;
mod error;
pub mod html;
mod highlighter;
mod frontmatter;
mod markdown;

/// run the interpreter on an AST tree and return a HTMLNode tree for each page
pub fn run(tokens: &Vec<Token>, pwd: Option<&str>) -> InterpreterResult<HashMap<String, Node<HTMLNode>>> {
    let state = &mut State::new(pwd);

    for token in tokens {
        _run(token, state, &mut None)?;
    }

    Ok(state.pages.clone())
}

/// recurse each token, resolve variables
fn _run(token: &Token, state: &mut State, parent: &mut Option<Node<HTMLNode>>) -> InterpreterResult<()> {
    match token {
        Token::Comment(_) => {}
        Token::Element(e) => {
            let mut el = HTMLElement::new_from_html_tag(&e.ident)?;

            for attr in &e.attributes.clone() {
                match attr {
                    // class attribute eg .blah
                    Attribute::Class(class) => el.add_class(class),
                    // symbol eg. centered align.center
                    Attribute::Symbol(_) => {
                        // state.imports.modify_element(&modifier, None, &mut el)?;
                        // ()
                        unimplemented!();
                    }
                    // named attribute eg. href="/index.html"
                    Attribute::NamedAttribute { ident, variable } => {
                        let value = state.resolve(variable)?;
                        // wtf is this
                        // state.imports.modify_element(
                        //     &ident,
                        //     Some(&String::from(value)),
                        //     &mut el,
                        // )?;
                    }
                    // anonymous attribute eg disabled
                    Attribute::Decorator(_) => panic!("decorators deprecated"),
                }
            }

            let mut node = Some(Node::new(HTMLNode::Element(el)));

            // interpret children
            for token in &e.children {
                _run(token, state, &mut node)?;
            }

            if let Some(parent) = parent {
                parent.append(node.unwrap());
            } else {
                // tag was found that isn't actually in any structure
                return Err(InterpreterError::OrphanTag);
            }
        }
        Token::ForLoop(f) => {
            let path: Value = state.resolve(&f.iterable)?;

            if let Value::Path(path) = path {
                let project_relative_path = state.get_state_relative_path(&path);
                // this should eventually return an array type, not a vec<document>
                let documents = Document::read_from_glob(&project_relative_path)?;

                if documents.len() == 0 {
                    return Err(InterpreterError::EmptyFileGlob);
                }

                // for loops should not assume documents in future...
                for document in documents {
                    // create a new local state to pass down the tree
                    let mut new_state = state.clone();

                    new_state.insert(&f.index, &Value::Document(document));

                    for token in &f.children {
                        _run(token, &mut new_state, parent)?;
                    }

                    state.pages = new_state.pages;
                }
            } else {
                return Err(InterpreterError::InvalidDocuments);
            }
        }
        Token::Text(t) => {
            if let Some(parent) = parent {
                // let buffer = crate::interpolator::interpolate(t, &state.local_variables)?;
                parent.append(Node::new(HTMLNode::Text(state.interpolate_string(t)?)));
            }
        }
        Token::CodeBlock(_) => {}
        Token::FunctionCall(f) => {
            // resolve any variables in function arguments
            let arguments: NamedArguments = f
                .arguments
                .iter()
                .flat_map(|(ident, v)| state.resolve(v).map(|v| (ident.clone(), v.clone())))
                .collect();

            match f.ident.as_str() {
                "page" => {
                    // let el = functions::page(
                    //     arguments.get_required_string("route")?,
                    //     arguments.get_string("title")
                    // )?;

                    let route = arguments.get_required_string("route")?;
                    let stylesheet = arguments.get_string("stylesheet");

                    // make a fresh node tree
                    let mut node = Node::new(HTMLNode::new_element("html"));

                    // <title>
                    if let Some(title) = arguments.get_string("title") {
                        node.append(new_node_with_text("title", &title)?);
                    }

                    // <link rel="stylesheet">
                    if let Some(stylesheet) = stylesheet {
                        node.append(Node::new(HTMLNode::new_stylesheet_element(format!(
                            "/{}",
                            stylesheet
                        ))));
                    }

                    let mut body = Some(Node::new(HTMLNode::new_element("body")));

                    for token in &f.children {
                        _run(token, state, &mut body)?;
                    }

                    node.append(body.unwrap()); // unwrap is ok cause I just made it Some... rethink this though

                    state.pages.insert(route, node.clone().root());
                }
                "embed" => {
                    if let Some(parent) = parent {
                        let path: String = arguments.get_required_path("path")?;
                        let project_relative_path = state.get_state_relative_path(&path);
                        let file_content: String = std::fs::read_to_string(&project_relative_path).unwrap(); // Todo: From
                        let node: Node<HTMLNode> = Node::new(HTMLNode::Text(file_content));

                        parent.append(node);
                    } else {
                        return Err(InterpreterError::UnexpectedFunction);
                    }
                }
                "exec" => unimplemented!(),
                _ => unimplemented!(),
            }
        }
    }

    Ok(())
}