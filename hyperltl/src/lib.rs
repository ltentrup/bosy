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
    /// The temporal operator `R` for release
    Release,
    /// The temporal operator `W` for the weak variant of until
    WeakUntil,
    /// Literal `true`
    True,
    /// Literal `false`
    False,
}

impl Op {
    /// returns the arity if operator has a fixed one and `None` otherwise
    fn arity(&self) -> Option<usize> {
        use self::Op::*;
        match self {
            Negation => Some(1),
            Next => Some(1),
            Finally => Some(1),
            Globally => Some(1),
            Conjunction => None,
            Disjunction => None,
            Implication => Some(2),
            Exclusion => Some(2),
            Equivalence => Some(2),
            Until => Some(2),
            Release => Some(2),
            WeakUntil => Some(2),
            True => Some(0),
            False => Some(0),
        }
    }

    fn is_safety(&self) -> bool {
        use self::Op::*;
        match self {
            Negation => true,
            Next => true,
            Finally => false,
            Globally => true,
            Conjunction => true,
            Disjunction => true,
            Implication => true,
            Exclusion => true,
            Equivalence => true,
            Until => false,
            Release => true,
            WeakUntil => true,
            True => true,
            False => true,
        }
    }

    fn is_propositional(&self) -> bool {
        use self::Op::*;
        match self {
            Negation => true,
            Next => false,
            Finally => false,
            Globally => false,
            Conjunction => true,
            Disjunction => true,
            Implication => true,
            Exclusion => true,
            Equivalence => true,
            Until => false,
            Release => false,
            WeakUntil => false,
            True => true,
            False => true,
        }
    }

    fn is_temporal(&self) -> bool {
        !self.is_propositional()
    }

    fn is_chainable(&self) -> bool {
        use self::Op::*;
        match self {
            Negation => false,
            Next => false,
            Finally => false,
            Globally => false,
            Conjunction => true,
            Disjunction => true,
            Implication => false,
            Exclusion => false,
            Equivalence => false,
            Until => false,
            Release => false,
            WeakUntil => false,
            True => false,
            False => false,
        }
    }
}

impl HyperLTL {
    pub fn new_unary(op: Op, inner: Self) -> Self {
        HyperLTL::Appl(op, vec![inner])
    }

    pub fn new_binary(op: Op, lhs: Self, rhs: Self) -> Self {
        HyperLTL::Appl(op, vec![lhs, rhs])
    }

    fn constant_false() -> HyperLTL { HyperLTL::Appl(Op::False, Vec::new()) }
    fn constant_true() -> HyperLTL {HyperLTL::Appl(Op::True, Vec::new())}
}
