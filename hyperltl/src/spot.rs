use super::HyperLTL;
use super::HyperLTL::*;
use std::io::{Error, Write};

impl HyperLTL {
    pub fn to_spot(&self) -> String {
        match self {
            Unary(op, expr) => format!("{}{}", op, expr.to_spot()),
            Binary(op, left, right) => format!("({} {} {})", left.to_spot(), op, right.to_spot()),
            Proposition(name, index) => match index {
                Some(index) => format!("\"{}[{}]\"", name, index),
                None => format!("\"{}\"", name),
            },
            Literal(val) => format!("{}", if *val { "1" } else { "0" }),
            _ => unreachable!(),
        }
    }
}
