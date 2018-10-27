#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::convert::From;
use symboltable::{Symbol, SymbolTable};

mod operator;
pub mod parse;
mod print;

#[derive(Debug)]
pub struct Instance {
    declarations: Vec<Declaration>,
    assertions: Vec<Term>,
    symboltable: SymbolTable,
}

impl Instance {
    pub fn new() -> Self {
        Instance {
            declarations: Vec::new(),
            assertions: Vec::new(),
            symboltable: SymbolTable::new(),
        }
    }

    fn declare_enum(name: &str, values: &[&str]) {
        unimplemented!()
    }

    fn declare_const(&mut self, name: &str, sort: Sort) -> Identifier {
        let symbol = self.symboltable.get_symbol_for(name);
        self.declarations
            .push(Declaration::new_function(symbol, Vec::new(), sort));
        Identifier::new_simple(symbol)
    }

    fn declare_fun(name: &str, param: &[Sort], ret: Sort) -> Identifier {
        unimplemented!();
    }

    fn assert(&mut self, expr: Term) {
        self.assertions.push(expr);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Term {
    kind: TermKind,
}

#[derive(Debug, PartialEq, Eq)]
enum TermKind {
    Lit(Literal),
    Ident(Identifier),
    Appl(Identifier, Vec<Box<Term>>),
    Let(Vec<(Symbol, Box<Term>)>, Box<Term>),
    Quant(QuantKind, Vec<(Symbol, Sort)>, Box<Term>),
}

impl Term {
    fn new_ident(ident: Identifier) -> Term {
        Term {
            kind: TermKind::Ident(ident),
        }
    }

    fn new_appl(ident: Identifier, param: Vec<Box<Term>>) -> Term {
        Term {
            kind: TermKind::Appl(ident, param),
        }
    }

    pub const TRUE: Term = Term {
        kind: TermKind::Ident(Identifier::TRUE),
    };

    pub const FALSE: Term = Term {
        kind: TermKind::Ident(Identifier::FALSE),
    };
}

impl From<Identifier> for Term {
    fn from(ident: Identifier) -> Self {
        Term::new_ident(ident)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Literal {
    Numeral(Symbol),
    Decimal(Symbol),
    Hexadecimal(Symbol),
    Binary(Symbol),
    String(Symbol),
}

#[derive(Debug, PartialEq, Eq)]
struct Sort {
    kind: SortKind,
}

#[derive(Debug, PartialEq, Eq)]
enum SortKind {
    Bool,
    Int,
    Real,
    Simple(Identifier),
    Parameterized(Identifier, Vec<Box<Sort>>),
}

impl Sort {
    const BOOL: Sort = Sort {
        kind: SortKind::Bool,
    };
}

#[derive(Debug, PartialEq, Eq)]
struct Identifier {
    kind: IdentKind,
}

#[derive(Debug, PartialEq, Eq)]
enum IdentKind {
    Simple(Symbol),
    Indexed(Symbol, Vec<usize>),
    BooleanFun(BoolFun),
}

#[derive(Debug, PartialEq, Eq)]
enum BoolFun {
    True,
    False,
    Not,
    Impl,
    And,
    Or,
    Xor,
    Equiv,
    Distinct,
    Ite,
}

impl Identifier {
    fn new_simple(symbol: Symbol) -> Identifier {
        Identifier {
            kind: IdentKind::Simple(symbol),
        }
    }

    const TRUE: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::True),
    };

    const FALSE: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::False),
    };

    const NOT: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Not),
    };

    const AND: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::And),
    };

    const OR: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Or),
    };

    const IMPL: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Impl),
    };

    const XOR: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Xor),
    };

    const EQUIV: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Equiv),
    };
}

#[derive(Debug, PartialEq, Eq)]
enum QuantKind {
    Exists,
    Forall,
}

#[derive(Debug)]
struct Declaration {
    kind: DeclKind,
}

#[derive(Debug)]
enum DeclKind {
    /// A function declaration
    Func(Symbol, Vec<Sort>, Sort),
    /// A sort declaration
    Sort(Symbol, usize),
}

impl Declaration {
    fn new_function(name: Symbol, param: Vec<Sort>, ret: Sort) -> Declaration {
        Declaration {
            kind: DeclKind::Func(name, param, ret),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn print_simple_script() {
        let mut instance = Instance::new();
        let a = instance.declare_const("a", Sort::BOOL);
        let b = instance.declare_const("b", Sort::BOOL);
        instance.assert(Term::new_appl(
            Identifier::AND,
            vec![Box::new(a.into()), Box::new(b.into())],
        ));
        let script = format!("{}", instance);
        assert_eq!(
            script,
            "(declare-const a Bool)\n(declare-const b Bool)\n(assert (and a b))\n"
        )
    }
}
