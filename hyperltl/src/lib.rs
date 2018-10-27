#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod operations;
mod parse;
mod print;
mod serialize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HyperLTL {
    /// A quantifier, e.g. `forall pi`
    Quant(QuantKind, Vec<String>, Box<HyperLTL>),
    /// A unary operation, e.g., `G x` and `! a`
    Unary(UnOp, Box<HyperLTL>),
    /// A binary operation, e.g., `a U b` and `c && d`
    Binary(BinOp, Box<HyperLTL>, Box<HyperLTL>),
    /// A proposition
    Proposition(String, Option<String>),
    /// A literal, i.e., `true` or `false`
    Literal(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantKind {
    /// The existential path quantifier
    Exists,
    /// The universal path quantifier
    Forall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    /// The Boolean operator `!` for logical inversion
    Negation,
    /// The temporal operator `X` for next
    Next,
    /// The temporal operator `G` for globally
    Globally,
    /// The temporal operator `F` for finally
    Finally,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// The Boolean operator `&&` for logical conjunction
    Conjunction,
    /// The Boolean operator `||` for logical disjunction
    Disjunction,
    /// The Boolean operator `->` for logical implication
    Implication,
    /// The Boolean operator `xor` for logical exclusion
    Exclusion,
    /// The Boolean operator `<->` for logical equivalence
    Equivalence,
    /// The temporal operator `U` for until
    Until,
    /// The temporal operator `W` for the weak variant of until
    WeakUntil,
    /// The temporal operator `R` for release
    Release,
}
