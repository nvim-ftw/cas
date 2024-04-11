use untwine::{parser, parser_repl};

fn main() {
    println!("Welcome to a (simple) Computer Algebra System.");
    parser_repl(expression);
}

#[derive(Debug)]
enum Func {
    Root(u32, Term),
    Log(u32, Term),
    Power(f64, Term),
    JustX,
}

#[derive(Debug)]
enum Term {
    Constant(f64),
    FnOnly(Box<Func>),
    Both(f64, Box<Func>),
}

#[derive(Debug)]
enum Connection {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exp,
}

/// A binary tree structure to represent a polynomial expression
#[derive(Debug)]
enum ExpressionNode {
    Term(Term),
    Pair(Box<ExpressionNode>, Connection, Box<ExpressionNode>),
}

impl TryFrom<(ExpressionNode, char, ExpressionNode)> for ExpressionNode {
    type Error = ();
    fn try_from(from: (ExpressionNode, char, ExpressionNode)) -> Result<ExpressionNode, ()> {
        Ok(ExpressionNode::Pair(
            Box::new(from.0),
            match from.1 {
                '+' => Connection::Add,
                '-' => Connection::Subtract,
                '*' => Connection::Multiply,
                '/' => Connection::Divide,
                '^' => Connection::Exp,
                _ => return Err(()),
            },
            Box::new(from.2),
        ))
    }
}

impl From<Term> for ExpressionNode {
    fn from(from: Term) -> ExpressionNode {
        ExpressionNode::Term(from)
    }
}

struct Equation {
    left: ExpressionNode,
    right: ExpressionNode,
}

// For reference while building the new one
parser! {
    sep = #{char::is_ascii_whitespace}*;
    num: num=<"-"? {char::is_ascii_digit}+ ("." {char::is_ascii_digit}+)?>
        -> f64 { num.parse().unwrap() }
    int: num=<"-"? {char::is_ascii_digit}+> -> u32 { num.parse().unwrap() }
    variable: {char::is_ascii_alphabetic} -> Func { Func::JustX }

    log: "log_" base=int "(" arg=term ")" -> Func {
        Func::Log(base, arg)
    }
    root: "rt_" degree=int "(" arg=term ")" -> Func {
        Func::Root(degree, arg)
    }
    pow: "pow(" t=term "," sep degree=num ")" -> Func {
        Func::Power(degree, t)
    }
    function = (log | root | pow | variable) -> Func;
    term: coefficient=num? func=function? -> Term {
        match (coefficient, func) {
            (Some(c), Some(f)) => Term::Both(c, Box::new(f)),
            (Some(c), None) => Term::Constant(c),
            (None, Some(f)) => Term::FnOnly(Box::new(f)),
            _ => panic!("Empty terms not allowed"),
        }
    }
    add: first=mul sep ops=(["+-"] sep mul)* -> ExpressionNode {
        ops.into_iter().fold(
            first,
            |acc, (op, term)|
            (acc, op, term).try_into().unwrap()
        )
    }
    mul: first=parens sep ops=(["*/"] sep parens)* -> ExpressionNode {
        ops.into_iter().fold(
            first,
            |acc, (op, term)|
            (acc, op, term).try_into().unwrap()
        )
    }
    nodeterm: t=term -> ExpressionNode { t.into() }
    parens = ("(" add ")" | nodeterm) -> ExpressionNode;
    pub expression = add -> ExpressionNode;
}
