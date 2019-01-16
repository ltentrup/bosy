#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::convert::From;
use std::rc::{Rc, Weak};

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

    pub fn declare_enum(&mut self, name: &str, values: &[String]) -> (Sort, Vec<Identifier>) {
        let sort = Rc::new(SortDecl::Enum(
            name.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        ));
        self.sorts.push(sort.clone());
        let idents: Vec<Identifier> = values
            .iter()
            .map(|val| {
                let case = Rc::new(IdentDecl::Case(
                    val.to_string(),
                    Sort {
                        kind: SortKind::Custom(sort.clone()),
                    },
                ));
                self.declarations.push(case.clone());
                Identifier {
                    kind: IdentKind::Custom(case),
                }
            })
            .collect();
        (
            Sort {
                kind: SortKind::Custom(sort),
            },
            idents,
        )
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Term {
    pub kind: TermKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TermKind {
    Lit(Literal),
    Ident(Identifier),
    Appl(Identifier, Vec<Term>),
    //Let(Vec<(Symbol, Box<Term>)>, Box<Term>),
    Quant(QuantKind, Vec<Identifier>, Box<Term>),
}

impl Term {
    pub fn new_ident(ident: &Identifier) -> Term {
        Term {
            kind: TermKind::Ident(ident.clone()),
        }
    }

    pub fn new_appl(ident: Identifier, param: Vec<Term>) -> Term {
        Term {
            kind: TermKind::Appl(ident, param),
        }
    }

    pub fn new_quant<F>(kind: QuantKind, binding: &[(String, &Sort)], scope: F) -> Term
    where
        F: Fn(&[Identifier]) -> Term,
    {
        let identifier: Vec<Identifier> = binding
            .iter()
            .map(|(name, sort)| {
                let decl = Rc::new(IdentDecl::Func(
                    name.to_string(),
                    Vec::new(),
                    (*sort).clone(),
                ));
                Identifier {
                    kind: IdentKind::Custom(decl),
                }
            })
            .collect();
        let inner = scope(&identifier);
        Term {
            kind: TermKind::Quant(kind, identifier, Box::new(inner)),
        }
    }

    pub const TRUE: Term = Term {
        kind: TermKind::Ident(Identifier::TRUE),
    };

    pub const FALSE: Term = Term {
        kind: TermKind::Ident(Identifier::FALSE),
    };

    /// Transfers term from one instance to another using the lookup table
    pub fn transfer(&self, lookup: &HashMap<Identifier, Identifier>) -> Term {
        match &self.kind {
            TermKind::Lit(l) => Term {
                kind: TermKind::Lit(*l),
            },
            TermKind::Ident(i) => {
                if let Some(t) = lookup.get(i) {
                    Term::new_ident(t)
                } else {
                    Term::new_ident(i)
                }
            }
            TermKind::Appl(i, param) => Term::new_appl(
                lookup.get(i).unwrap_or(i).clone(),
                param.iter().map(|t| t.transfer(lookup)).collect(),
            ),
            _ => unimplemented!(),
        }
    }

    pub fn replace(&self, lookup: &HashMap<Term, Term>) -> Term {
        if let Some(new) = lookup.get(self) {
            return new.clone();
        }
        match &self.kind {
            TermKind::Lit(l) => Term {
                kind: TermKind::Lit(*l),
            },
            TermKind::Ident(i) => Term::new_ident(i),
            TermKind::Appl(i, param) => {
                Term::new_appl(i.clone(), param.iter().map(|t| t.replace(lookup)).collect())
            }
            _ => unimplemented!(),
        }
    }

    pub fn convert<F>(&self, replace: &F) -> Term
    where
        F: Fn(&Term) -> Option<Term>,
    {
        if let Some(new) = replace(&self) {
            return new;
        }
        match &self.kind {
            TermKind::Lit(l) => Term {
                kind: TermKind::Lit(*l),
            },
            TermKind::Ident(i) => Term::new_ident(i),
            TermKind::Appl(i, param) => Term::new_appl(
                i.clone(),
                param.iter().map(|t| t.convert(replace)).collect(),
            ),
            _ => unimplemented!(),
        }
    }
}

impl From<&Identifier> for Term {
    fn from(ident: &Identifier) -> Self {
        Term::new_ident(&ident)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Literal {
    Numeral(i128),
    //Decimal(f64),
    //Hexadecimal(Symbol),
    //Binary(Symbol),
    //String(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SortDecl {
    /// A sort declaration
    Sort(String, usize),
    // An enum declaration
    Enum(String, Vec<String>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Sort {
    kind: SortKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    pub const INT: &'static Sort = &Sort {
        kind: SortKind::Int,
    };
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Identifier {
    pub kind: IdentKind,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum IdentKind {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BoolFun {
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
    /// Less than `<`
    Lt,
    /// Less or equal `<=`
    Le,
    // Equal `=`
    Eq,
}

impl Identifier {
    pub const TRUE: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::True),
    };

    pub const FALSE: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::False),
    };

    pub const NOT: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Not),
    };

    pub const AND: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::And),
    };

    pub const OR: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Or),
    };

    pub const IMPL: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Impl),
    };

    pub const XOR: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Xor),
    };

    pub const EQUIV: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Equiv),
    };

    pub const LT: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Lt),
    };

    pub const LE: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Le),
    };

    pub const EQ: Identifier = Identifier {
        kind: IdentKind::BooleanFun(BoolFun::Eq),
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum QuantKind {
    Exists,
    Forall,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum IdentDecl {
    /// A function declaration
    Func(String, Vec<Sort>, Sort),
    /// An enum case
    Case(String, Sort),
}

impl IdentDecl {
    fn sort(&self) -> &Sort {
        match self {
            IdentDecl::Func(_, _, s) => &s,
            IdentDecl::Case(_, s) => &s,
        }
    }
}

/*enum Sort2 {
    Static(&'static SortDecl),
    Dynamic(Rc<SortDecl>),
}

enum Ident2<'a> {
    Static(&'a StaticFuncDecl<'a>),
    Dynamic(Rc<DynamicFuncDecl>)
}

struct DynamicFuncDecl {
    name: String,
    param: Vec<Sort>,
    ret: Sort,
}

struct StaticFuncDecl<'a> {
    name: &'a str,
    param: &'a[&'a Sort],
    ret: &'a Sort,
    // attributes
}

impl<'a> StaticFuncDecl<'a> {
    const TRUE: &'static StaticFuncDecl<'static> = &StaticFuncDecl {
        name: "true",
        param: &[],
        ret: Sort::BOOL,
    };

    const AND: &'static StaticFuncDecl<'static> = &StaticFuncDecl {
        name: "and",
        param: &[Sort::BOOL, Sort::BOOL],
        ret: Sort::BOOL,
    };
}*/

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn print_simple_script() {
        let mut instance = Instance::new();
        let a = &instance.declare_const("a", Sort::BOOL);
        let b = &instance.declare_const("b", Sort::BOOL);
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
