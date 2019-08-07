use std::cmp::Eq;
use std::fmt::Display;
use std::hash::Hash;

pub trait Logic: Hash + Eq + Display + Clone {
    type Manager;

    fn is_false(&self) -> bool;
    fn is_true(&self) -> bool;
    fn negated(&self) -> Self;
    fn and(&self, other: Self) -> Self;
}

use smtlib;

impl Logic for smtlib::Term {
    type Manager = smtlib::Instance;

    fn is_false(&self) -> bool {
        self.is_false()
    }

    fn is_true(&self) -> bool {
        self.is_true()
    }

    fn negated(&self) -> Self {
        self.negated()
    }

    fn and(&self, other: Self) -> Self {
        smtlib::Term::new_appl(smtlib::Identifier::AND, vec![self.clone(), other])
    }
}
