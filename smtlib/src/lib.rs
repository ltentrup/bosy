#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;

mod operator;
pub mod parse;
mod print;

#[derive(Debug)]
pub struct Instance {
    declarations: Vec<Rc<IdentDecl>>,
    sorts: Vec<Rc<SortDecl>>,
    assertions: Vec<Term>,
}

impl Instance {
    pub fn new() -> Self {
        Instance {
            declarations: Vec::new(),
            sorts: Vec::new(),
            assertions: Vec::new(),
        }
    }

    pub fn declare_sort(&mut self, name: &str, arity: usize) -> Sort {
        let sort = Rc::new(SortDecl::Sort(name.to_string(), arity));
        self.sorts.push(sort.clone());
        Sort {
            kind: SortKind::Custom(sort),
        }
    }

    pub fn declare_enum(&mut self, name: &str, values: &[&str]) -> Sort {
        unimplemented!();
    }

    pub fn declare_const(&mut self, name: &str, sort: &Sort) -> Identifier {
        let func = Rc::new(IdentDecl::Func(name.to_string(), Vec::new(), sort.clone()));
        self.declarations.push(func.clone());
        Identifier {
            kind: IdentKind::Custom(func),
        }
    }

    pub fn declare_fun(&mut self, name: &str, param: &[&Sort], ret: &Sort) -> Identifier {
        let func = Rc::new(IdentDecl::Func(
            name.to_string(),
            param.iter().map(|&s| s.clone()).collect(),
            ret.clone(),
        ));
        self.declarations.push(func.clone());
        Identifier {
            kind: IdentKind::Custom(func),
        }
    }

    pub fn assert(&mut self, expr: Term) {
        self.assertions.push(expr);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Term {
    kind: TermKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TermKind {
    Lit(Literal),
    Ident(Identifier),
    Appl(Identifier, Vec<Box<Term>>),
    //Let(Vec<(Symbol, Box<Term>)>, Box<Term>),
    //Quant(QuantKind, Vec<(Symbol, Sort)>, Box<Term>),
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Literal {
    Numeral(i128),
    //Decimal(f64),
    //Hexadecimal(Symbol),
    //Binary(Symbol),
    //String(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SortDecl {
    /// A sort declaration
    Sort(String, usize),
    // An enum declaration
    //Enum(String, Vec<IdentDecl>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sort {
    kind: SortKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SortKind {
    Bool,
    Int,
    Real,
    Custom(Rc<SortDecl>),
}

impl Sort {
    pub const BOOL: &'static Sort = &Sort {
        kind: SortKind::Bool,
    };
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Identifier {
    kind: IdentKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum IdentKind {
    //Simple(Symbol),
    //Indexed(Symbol, Vec<Index>),
    BooleanFun(BoolFun),
    Custom(Rc<IdentDecl>),
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// enum Index {
//     Numeral(usize),
//     Symbol(Symbol),
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum QuantKind {
    Exists,
    Forall,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdentDecl {
    /// A function declaration
    Func(String, Vec<Sort>, Sort),
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
