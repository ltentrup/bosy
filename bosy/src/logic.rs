use std::hash::Hash;
use  std::cmp::Eq;

pub trait Logic: Hash + Eq {
    type Manager;

    fn is_false(&self) -> bool;
}

use smtlib;

impl Logic for smtlib::Term {
    type Manager = smtlib::Instance;

    fn is_false(&self) -> bool {
        self.is_false()
    }
}
