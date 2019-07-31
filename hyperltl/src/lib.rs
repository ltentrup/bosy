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
mod spot;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HyperLTL {
    /// A quantifier, e.g. `forall pi`
    Quant(QuantKind, Vec<String>, Box<HyperLTL>),
    /// An operation, e.g., `G x`, `! a`, `a U b`, or `c && d`
    Appl(Op, Vec<HyperLTL>),
    /// A proposition
    Prop(String, Option<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantKind {
    /// The existential path quantifier
    Exists,
    /// The universal path quantifier
    Forall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    /// The Boolean operator `!` for logical inversion
    Negation,
    /// The temporal operator `X` for next
    Next,
    /// The temporal operator `G` for globally
    Globally,
    /// The temporal operator `F` for finally
    Finally,
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
    /// Literal `true`
    True,
    /// Literal `false`
    False
}
