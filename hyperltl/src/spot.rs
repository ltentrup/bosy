use super::HyperLTL::*;
use super::{HyperLTL, Op};
use std::io::{Error, Write};

impl HyperLTL {
    pub fn to_spot(&self) -> String {
        match self {
            Appl(op, inner) => {
                match op {
                    Op::True => return format!("1"),
                    Op::False => return format!("0"),
                    _ => {}
                }
                match inner.len() {
                    0 => format!("{}", op),
                    1 => format!("{}{}", op, inner.iter().next().unwrap()),
                    _ => {
                        let operands: Vec<String> = inner.iter().map(|ele| ele.to_spot()).collect();
                        format!("({})", operands.join(&format!("{}", op)))
                    }
                }
            }
            Prop(name, index) => match index {
                Some(index) => format!("\"{}[{}]\"", name, index),
                None => format!("\"{}\"", name),
            },
            _ => unreachable!(),
        }
    }
}
