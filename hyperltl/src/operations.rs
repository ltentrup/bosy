use super::HyperLTL::*;
use super::Op::*;
use super::QuantKind::*;
use super::*;
use std::collections::HashSet;

impl HyperLTL {
    /// Checks if a formula contains no quantifier, i.e., is LTL
    pub fn is_quantifier_free(&self) -> bool {
        match self {
            Quant(_, _, _) => false,
            Appl(_, inner) => inner.iter().all(|ele| ele.is_quantifier_free()),
            Prop(_, _) => true,
        }
    }

    /// Checks if a formula contains a quantifier prefix, followed by LTL body
    pub fn is_hyperltl(&self) -> bool {
        match self {
            Quant(_, _, scope) => scope.is_hyperltl() || scope.is_quantifier_free(),
            _ => false,
        }
    }

    /// Returns the set of Props contained in the formula
    pub fn get_propositions(&self) -> HashSet<&str> {
        match self {
            Quant(_, _, inner) => inner.get_propositions(),
            Appl(_, inner) => inner.iter().fold(HashSet::new(), |set, ele| {
                set.union(&ele.get_propositions()).map(|e| *e).collect()
            }),
            Prop(prop, _) => {
                let mut singleton = HashSet::new();
                singleton.insert(prop.as_ref());
                singleton
            }
        }
    }

    pub fn get_occurrences(&self) -> HashSet<String> {
        match self {
            Quant(_, _, inner) => inner.get_occurrences(),
            Appl(_, inner) => inner.iter().fold(HashSet::new(), |set, ele| {
                set.union(&ele.get_occurrences())
                    .map(|e| e.clone())
                    .collect()
            }),
            Prop(_, _) => {
                let mut singleton = HashSet::new();
                singleton.insert(format!("{}", self));
                singleton
            }
        }
    }

    pub fn get_body(&self) -> &HyperLTL {
        match self {
            Quant(_, _, inner) => inner.get_body(),
            _ => self,
        }
    }

    pub fn get_quantifier(&self) -> Vec<(QuantKind, Vec<String>)> {
        let mut res = Vec::new();
        self.quantifier(&mut res);
        res
    }

    fn quantifier(&self, quant: &mut Vec<(QuantKind, Vec<String>)>) {
        match self {
            Quant(kind, param, inner) => {
                quant.push((*kind, param.clone()));
                inner.quantifier(quant);
            }
            _ => (),
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
            Appl(Negation, expr) => unimplemented!(),
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

impl Op {
    fn negate(&mut self) {
        *self = match self {
            Next => Next,
            Finally => Globally,
            Globally => Finally,
            Conjunction => Disjunction,
            Disjunction => Conjunction,
            Exclusion => Equivalence,
            Equivalence => Exclusion,
            Until => Release,
            Release => Until,
            True => False,
            False => True,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_binary() {
        let expr = Appl(
            Disjunction,
            vec![
                Appl(Negation, vec![Prop("a".into(), None)]),
                Prop("b".into(), None),
            ],
        );
        assert!(expr.is_quantifier_free());
        assert!(!expr.is_hyperltl());
    }

    #[test]
    fn example_unary() {
        let expr = Appl(Negation, vec![Appl(Globally, vec![Prop("a".into(), None)])]);
        assert!(expr.is_quantifier_free());
        assert!(!expr.is_hyperltl());
    }

    #[test]
    fn example_quantifier() {
        let expr = Quant(
            Forall,
            vec!["pi".into()],
            Box::new(Prop("a".into(), Some("pi".into()))),
        );
        assert!(!expr.is_quantifier_free());
        assert!(expr.is_hyperltl());
    }

    #[test]
    fn example_get_propositionns() {
        let expr = Appl(
            Conjunction,
            vec![
                Appl(Negation, vec![Prop("a".into(), None)]),
                Appl(
                    Disjunction,
                    vec![Prop("b".into(), None), Prop("c".into(), None)],
                ),
            ],
        );
        let Props = expr.get_propositions();
        assert!(Props.contains("a"));
        assert!(Props.contains("b"));
        assert!(Props.contains("c"));
    }
}
