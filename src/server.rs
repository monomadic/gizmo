use astryx::{
    error::*,
    interpreter::{self, State},
};
use simple_server::{Server, StatusCode};
use std::{collections::HashMap, path::PathBuf};
use astryx::error::AstryxResult;

pub(crate) fn start(file: PathBuf, port: u32) -> AstryxResult<()> {
    let host = "127.0.0.1";
    let port = port.to_string();

    let mut server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let path = request.uri().path();
        let pages = render_pages(file.clone());

        println!("{} {}", request.method(), path);

        if path.contains("svg")  {
            response.header("content-type", "image/svg+xml");
            // return Ok(response.body(svgfile.as_bytes().to_vec())?);
        }

        match pages {
            Ok(pages) => {
                match pages.get(path) {
                    Some(page) => Ok(response.body(page.as_bytes().to_vec())?),
                    None => {
                        response.status(StatusCode::NOT_FOUND);
                        Ok(response.body(format!("<h1>404</h1><p>Path not found: {}<p>", path).as_bytes().to_vec())?)
                    }
                }
            }
            Err(e) => {
                response.status(StatusCode::INTERNAL_SERVER_ERROR);
                println!("ERROR: {:#?}", e);

                let mut highlighter = astryx::highlighter::SyntaxHighlighter::new();
                highlighter.set_syntax_by_file_extension("yaml");
                let highlighted_msg = highlighter.highlight(&format!("{}", e.msg));
                let highlighted_state = highlighter.highlight(&format!("{:?}", e.state));

                Ok(response.body(
                    format!("<html style='background-color: black;color: white;'><body><h1>Error :(</h1><pre>{}</pre>\n\n<pre>{:#?}</pre></body></html>", highlighted_msg, highlighted_state)
                        .as_bytes()
                        .to_vec(),
                )?)
            }
        }

        // request.method() -> &Method::GET
    });

    server.set_static_directory("examples/public");
    server.listen(host, &port);
}

fn render_pages(file: PathBuf) -> AstryxResult<HashMap<String, String>> {
    let state = &mut State::new();
    let file = astryx::filesystem::read_file(file.clone())?;
    let nodes = astryx::parser::parse(&file)?;
    // let _ = interpreter::run(&nodes, state)?;

    let _ = interpreter::__run(&nodes, state)?;

    let mut pages = state.render_pages()?;

    pages.insert("/state".into(), format!("{:#?}", state));
    pages.insert("/nodes".into(), format!("{:#?}", nodes));

    Ok(pages)
}
