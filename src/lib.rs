#![warn(clippy::todo)]
use untwine::parser_repl;
mod expressions;
use expressions::{equation, expression, ExpressionNode};

pub fn start_repl() {
    parser_repl(equation);
}

#[derive(Debug)]
pub struct Equation {
    left: ExpressionNode,
    right: ExpressionNode,
}

impl Equation {
    fn solve(mut self) -> Result<f64, ()> {
        todo!()
    }
}
