use super::BinOp::*;
use super::HyperLTL::*;
use super::QuantKind::*;
use super::UnOp::*;
use super::*;
use std::collections::HashSet;

impl HyperLTL {
    /// Checks if a formula contains no quantifier, i.e., is LTL
    pub fn is_quantifier_free(&self) -> bool {
        match self {
            Quant(_, _, _) => false,
            Unary(_, inner) => inner.is_quantifier_free(),
            Binary(_, left, right) => left.is_quantifier_free() && right.is_quantifier_free(),
            Proposition(_, _) => true,
            Literal(_) => true,
        }
    }

    /// Checks if a formula contains a quantifier prefix, followed by LTL body
    pub fn is_hyperltl(&self) -> bool {
        match self {
            Quant(_, _, scope) => scope.is_hyperltl() || scope.is_quantifier_free(),
            _ => false,
        }
    }

    /// Returns the set of propositions contained in the formula
    pub fn get_propositions(&self) -> HashSet<&str> {
        match self {
            Quant(_, _, inner) => inner.get_propositions(),
            Unary(_, inner) => inner.get_propositions(),
            Binary(_, left, right) => left
                .get_propositions()
                .union(&right.get_propositions())
                .map(|e| *e)
                .collect(),
            Proposition(prop, _) => {
                let mut singleton = HashSet::new();
                singleton.insert(prop.as_ref());
                singleton
            }
            Literal(_) => HashSet::new(),
        }
    }

    /// Brings formula to negation normal form (NNF) and collapses consecutive quantifier of the same type
    fn normalize(self) -> Self {
        self.collapse_quantifier().to_nnf(false)
    }

    fn collapse_quantifier(self) -> Self {
        unimplemented!();
    }

    fn to_nnf(self, negated: bool) -> HyperLTL {
        match self {
            Quant(mut qtype, params, scope) => {
                if negated {
                    qtype.negate()
                }
                Quant(qtype, params, scope.to_nnf(negated).into())
            }
            Unary(Negation, expr) => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl QuantKind {
    fn negate(&mut self) {
        *self = match self {
            Exists => Forall,
            Forall => Exists,
        }
    }
}

impl BinOp {
    fn negate(&mut self) {
        *self = match self {
            Conjunction => Disjunction,
            Disjunction => Conjunction,
            Exclusion => Equivalence,
            Equivalence => Exclusion,
            Until => Release,
            Release => Until,
            _ => unreachable!(),
        }
    }
}

impl UnOp {
    fn negate(&mut self) {
        *self = match self {
            Next => Next,
            Finally => Globally,
            Globally => Finally,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_binary() {
        let expr = Binary(
            Disjunction,
            Box::new(Unary(Negation, Box::new(Proposition("a".into(), None)))),
            Box::new(Proposition("b".into(), None)),
        );
        assert!(expr.is_quantifier_free());
        assert!(!expr.is_hyperltl());
    }

    #[test]
    fn example_unary() {
        let expr = Unary(
            Negation,
            Box::new(Unary(Globally, Box::new(Proposition("a".into(), None)))),
        );
        assert!(expr.is_quantifier_free());
        assert!(!expr.is_hyperltl());
    }

    #[test]
    fn example_quantifier() {
        let expr = Quant(
            Forall,
            vec!["pi".into()],
            Box::new(Proposition("a".into(), Some("pi".into()))),
        );
        assert!(!expr.is_quantifier_free());
        assert!(expr.is_hyperltl());
    }

    #[test]
    fn example_get_propositions() {
        let expr = Binary(
            Conjunction,
            Box::new(Unary(Negation, Box::new(Proposition("a".into(), None)))),
            Box::new(Binary(
                Disjunction,
                Box::new(Proposition("b".into(), None)),
                Box::new(Proposition("c".into(), None)),
            )),
        );
        let propositions = expr.get_propositions();
        assert!(propositions.contains("a"));
        assert!(propositions.contains("b"));
        assert!(propositions.contains("c"));
    }
}
