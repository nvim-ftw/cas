use untwine::{parser, parser_repl};

fn main() {
    println!("Welcome to a (simple) Computer Algebra System.");
    // parser_repl(old_parser);
}

struct XPower(f64);

enum Func {
    Root(u32, Term),
    Log(u32, Term),
}

enum Term {
    Constant(f64),
    FnOnly(Box<Func>),
    Both(f64, Box<Func>),
}

enum Connection {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// A binary tree structure to represent a polynomial expression
enum ExpressionNode {
    Term(Term),
    Pair(Box<ExpressionNode>, Box<ExpressionNode>),
}

struct Equation {
    left: ExpressionNode,
    right: ExpressionNode,
}

fn testfn() {}

// For reference while building the new one
parser! {
    sep = #{char::is_ascii_whitespace}*;
    num: num=<"-"? {char::is_ascii_digit}+ ("." {char::is_ascii_digit}+)?>
        -> f64 { num.parse().unwrap() }
    int: num=<"-"? {char::is_ascii_digit}+> -> u32 { num.parse().unwrap() }
    variable: var={char::is_ascii_alphabetic}? -> bool { var.is_some() }

    log: "log_" base=int "(" arg=term ")" -> Func {
        Func::Log(base, arg)
    }
    root: "rt_" degree=int "(" arg=term ")" -> Func {
        Func::Root(degree, arg)
    }
    function = (log | root) -> Func;
    term: coefficient=num? func=function? -> Term {
        match (coefficient, func) {
            (Some(c), Some(f)) => Term::Both(c, Box::new(f)),
            (Some(c), None) => Term::Constant(c),
            (None, Some(f)) => Term::FnOnly(Box::new(f)),
            _ => panic!("Empty terms not allowed"),
        }
    }
    add: first=mul sep ops=(["+-"] sep mul)* -> f64 {
        ops.into_iter().fold(first, |left, (op, right)| operate(left, op, right))
    }
    mul: first=pow sep ops=(["*/"] sep pow)* -> f64 {
        ops.into_iter().fold(first, |left, (op, right)| operate(left, op, right))
    }
    pow: first=term sep ops=("^" sep term)* -> f64 {
        ops.into_iter().fold(first, |left, right| left.powf(right))
    }
    pub old_parser = add -> f64;
}

// A term is composed at least one of the following:
// - A Coefficient
// - A function of x (including its identity/no-op)
