use crate::error::*;
use interpreter::State;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn build<'a>(file: &'a str) -> AstryxResult<'a, ()> {
    let state = Rc::new(RefCell::new(State::new()));

    parser::run(file)
        .map_err(AstryxError::from)
        .and_then(|nodes| interpreter::run(&nodes, state).map_err(AstryxError::from))
        .map(program::render_project)
        .map(|_| ())
}
