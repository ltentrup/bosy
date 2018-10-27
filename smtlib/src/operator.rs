//! Implements operator overloading to improve usability.

use crate::{Identifier, Term};
use std::ops::{BitAnd, BitOr, Not};

impl BitOr for Term {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Term::new_appl(Identifier::OR, vec![self.into(), rhs.into()])
    }
}

impl BitAnd for Term {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Term::new_appl(Identifier::AND, vec![self.into(), rhs.into()])
    }
}

impl Not for Term {
    type Output = Term;

    fn not(self) -> Term {
        Term::new_appl(Identifier::NOT, vec![self.into()])
    }
}
