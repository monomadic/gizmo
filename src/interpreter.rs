use crate::error::*;
use crate::models::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Site {
    // pub styles: HashMap<String, Style>,
    pub pages: HashMap<String, String>,
}

// #[derive(Debug, Clone)]
// pub enum Style {
//     // todo: separate only into valid styles eg TextStyle
//     BackgroundColor(String), // todo: Color
//     Custom(String),          // custom css eg "border: 1px solid red" etc
// }

#[derive(Debug, Clone)]
pub struct State {
    pub page_buffers: HashMap<String, String>,
    variables_in_scope: HashMap<String, Variable>,
    current_page_buffer: Option<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables_in_scope: HashMap::new(),
            page_buffers: HashMap::new(),
            current_page_buffer: None, // TODO should be current_page, it's not the buffer.
        }
    }

    pub fn get_required_variable(&self, i: &str) -> ParseResult<&Variable> {
        self.variables_in_scope
            .get(i)
            .ok_or(AstryxError::ParseError(format!(
                "variable not found: {}\nvariables in scope: {:?}",
                i, self.variables_in_scope
            )))
    }

    pub fn get_current_page_buffer(&mut self) -> ParseResult<&mut String> {
        if let Some(current_page) = self.current_page_buffer.clone() {
            if let Some(current_page_buffer) = self.page_buffers.get_mut(&current_page) {
                return Ok(current_page_buffer);
            }
        }
        // TODO return error
        panic!("oop");
    }

    // TODO extract this out into a multibuffer design pattern
    pub fn create_buffer(&mut self, key: String) -> ParseResult<()> {
        self.page_buffers.insert(key.clone(), String::new()); // FIXME check for collisions!
        self.current_page_buffer = Some(key);
        Ok(())
    }

    pub fn write_to_current_buffer(&mut self, string: &str) -> ParseResult<()> {
        Ok(self.get_current_page_buffer()?.push_str(string))
    }
}

pub fn html_tag(ident: &str, attributes: Vec<(&str, String)>) -> String {
    let attribs = if !attributes.is_empty() {
        format!(
            " {}",
            attributes
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<String>>()
                .join(" ")
        )
    } else {
        String::new()
    };

    format!("<{}{}>", ident, attribs)
}

/// run the interpreter over a series of nodes
pub fn run(nodes: &Vec<Node>, state: &mut State) -> ParseResult<()> {
    for node in nodes {
        match node {
            Node::Element(e) => {
                let arguments = collect_named_attributes(&e.attributes)?;

                match e.ident.as_str() {
                    // TODO make elements scriptable / programmable
                    // suggestion: nodes can 'resolve' to other nodes, ending in tag
                    "page" => {
                        // keep note of current page
                        let current_page = state.current_page_buffer.clone();
                        let path = crate::interpolation::stringify_variable(
                            &get_required_argument("path", &arguments)?,
                            &state.variables_in_scope,
                        )?;

                        state.create_buffer(path)?;
                        state.write_to_current_buffer("<html><head>")?;
                        if let Some(title) = get_optional_variable("title", &arguments) {
                            let title = crate::interpolation::stringify_variable(
                                &title,
                                &state.variables_in_scope,
                            )?;

                            state.write_to_current_buffer(&format!("<title>{}</title>", title))?;
                        };
                        state.write_to_current_buffer("<body>")?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</body></html>")?;

                        // surrender page buffer after use to previous page buffer
                        state.current_page_buffer = current_page;
                    }
                    "row" | "column" => {
                        state.write_to_current_buffer(&format!("<div class=\"{}\">", e.ident))?;
                        run(&e.children, state)?;
                        state.write_to_current_buffer("</div>")?;
                    }
                    "image" | "img" | "i" => {
                        // let path =
                        //     stringify_variable(&get_required_argument("path", &arguments)?, state)?;

                        let path = crate::interpolation::stringify_variable(
                            &get_required_argument("path", &arguments)?,
                            &state.variables_in_scope,
                        )?;

                        state.write_to_current_buffer(&html_tag("img", vec![("src", path)]))?;
                    }
                    _ => {
                        // panic!("");
                    }
                }
            }
            Node::Text(t) => {
                let buffer = crate::interpolation::interpolate(t, &state.variables_in_scope)?;
                state.write_to_current_buffer(&buffer)?;
            }
            Node::ForLoop(f) => {
                // FIXME: throw errors in error conditions, don't just fall through
                // FIXME: give a variable which can be interpolated

                let files = crate::filesystem::read_content_metadata(&f.iterable)?;
                for file in files {
                    // create a new local state to pass down the tree
                    let mut new_state = state.clone();

                    new_state
                        .variables_in_scope
                        .insert(f.index.clone(), Variable::TemplateFile(file));

                    println!("state: {:#?}", new_state.variables_in_scope);

                    run(&f.children, &mut new_state)?;
                    state.page_buffers = new_state.page_buffers; // kind of a dirty hack
                }
            }
            Node::CodeBlock(cb) => {
                state
                    .page_buffers
                    .insert(cb.ident.clone(), cb.content.clone());
            }
        }
    }

    Ok(())
}

pub fn collect_named_attributes(
    attributes: &Vec<Attribute>,
) -> ParseResult<HashMap<&String, &Variable>> {
    let mut named_attributes: HashMap<&String, &Variable> = HashMap::new();

    for attribute in attributes {
        match attribute {
            Attribute::Assignment { ident, variable } => {
                let _ = named_attributes.insert(ident, variable);

                // .ok_or(CassetteError::ParseError(format!(
                //     "duplicate assignment: {}",
                //     ident
                // )))?;
            }
            _ => (),
        }
    }
    Ok(named_attributes)
}

pub fn get_optional_variable(i: &str, locals: &HashMap<&String, &Variable>) -> Option<Variable> {
    locals
        .get(&String::from(i.clone()))
        .map(|v| v.clone().clone())
}

pub fn get_required_argument(
    i: &str,
    arguments: &HashMap<&String, &Variable>,
) -> ParseResult<Variable> {
    arguments
        .get(&i.to_string())
        .map(|v| v.clone().clone())
        .ok_or(AstryxError::ParseError(format!(
            "argument not found: {}. arguments: {:?}",
            i, arguments
        )))
}
