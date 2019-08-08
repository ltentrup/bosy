use super::HyperLTL::*;
use super::Op::*;
use super::QuantKind::*;
use super::*;
use std::collections::HashSet;

impl HyperLTL {
    fn check_arity(&self) {
        match self {
            Quant(_, _, scope) => scope.check_arity(),
            Appl(op, inner) => {
                inner.iter().for_each(|ele| ele.check_arity());
                match op.arity() {
                    Some(arity) => assert_eq!(arity, inner.len()),
                    None => {}
                }
            }
            Prop(_, _) => {}
        }
    }

    pub fn is_ltl(&self) -> bool {
        self.is_quantifier_free()
    }

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
    pub fn normalize(mut self) -> Self {
        self.check_arity();
        self.remove_derived();
        self.push_next().to_nnf(false).simplify().flatten()
    }

    /// Removes all operators that do not have a dual operation, i.e., `Implication` and `WeakUntil`
    fn remove_derived(&mut self) {
        match self {
            Quant(kind, vars, scope) => scope.remove_derived(),
            Appl(op, inner) => {
                inner.iter_mut().for_each(|subf| subf.remove_derived());
                match op {
                    Implication => {
                        // euivalent to `!lhs || rhs`
                        let lhs = inner.remove(0);
                        inner.insert(0, Appl(Negation, vec![lhs]));
                        *op = Disjunction;
                    }
                    WeakUntil => {
                        // equivelant to `G lhs || lhs U rhs`
                        let lhs = inner[0].clone();
                        let dummy = Appl(Op::True, vec![]);
                        *op = Op::Until;
                        let old = std::mem::replace(self, dummy);
                        *self = Appl(Op::Disjunction, vec![Appl(Op::Globally, vec![lhs]), old])
                    }
                    _ => {}
                }
            }
            Prop(_, _) => {}
        }
    }

    // pushes next operator over other temporal operators
    fn push_next(self) -> Self {
        match self {
            Appl(Op::Next, mut inner) => {
                let inner = inner.pop().unwrap();
                match inner {
                    Appl(Op::Globally, inner) => Appl(
                        Op::Globally,
                        inner
                            .into_iter()
                            .map(|subf| Appl(Op::Next, vec![subf]).push_next())
                            .collect(),
                    ),
                    Appl(Op::Finally, inner) => Appl(
                        Op::Finally,
                        inner
                            .into_iter()
                            .map(|subf| Appl(Op::Next, vec![subf]).push_next())
                            .collect(),
                    ),
                    t => Appl(Op::Next, vec![t]),
                }
            }
            Appl(op, inner) => Appl(op, inner.into_iter().map(|subf| subf.push_next()).collect()),
            Prop(_, _) => self,
            _ => unreachable!(),
        }
    }

    fn to_nnf(self, negated: bool) -> HyperLTL {
        match self {
            Quant(mut qtype, params, scope) => {
                if negated {
                    qtype.negate()
                }
                Quant(qtype, params, scope.to_nnf(negated).into())
            }
            Appl(Negation, expr) => {
                assert_eq!(expr.len(), 1);
                expr.into_iter().next().unwrap().to_nnf(!negated)
            }
            Appl(mut op, mut inner) => {
                if negated {
                    op.negate();
                }
                if op != Op::Equivalence && op != Op::Exclusion {
                    inner = inner.into_iter().map(|subf| subf.to_nnf(negated)).collect();
                }
                Appl(op, inner)
            }
            Prop(name, path) => {
                if negated {
                    Appl(Negation, vec![Prop(name, path)])
                } else {
                    Prop(name, path)
                }
            }
        }
    }

    fn flatten(self) -> HyperLTL {
        match self {
            Quant(qtype, mut params, mut scope) => {
                scope = match *scope {
                    Quant(other_qtype, other_params, other_scope) => {
                        if qtype == other_qtype {
                            // quantifiers can be collapsed
                            params.extend(other_params);
                            let new_quant = Quant(qtype, params, other_scope);
                            return new_quant.flatten();
                        } else {
                            Quant(other_qtype, other_params, other_scope)
                                .flatten()
                                .into()
                        }
                    }
                    _ => scope.flatten().into(),
                };
                Quant(qtype, params, scope)
            }
            Appl(op, mut inner) => {
                inner = inner.into_iter().map(|subf| subf.flatten()).collect();
                let mut new_inner = Vec::new();
                for subf in inner.into_iter() {
                    match subf {
                        Appl(other_op, other_inner) => {
                            if other_op == op && op.is_chainable() {
                                new_inner.extend(other_inner)
                            } else {
                                new_inner.push(Appl(other_op, other_inner))
                            }
                        }
                        subf => new_inner.push(subf),
                    }
                }
                Appl(op, new_inner)
            }
            Prop(name, path) => Prop(name, path),
        }
    }

    // remove true and false
    fn simplify(self) -> HyperLTL {
        assert!(self.is_ltl());
        match self {
            Appl(op, mut inner) => {
                inner = inner.into_iter().map(|subf| subf.simplify()).collect();
                match op {
                    Op::Conjunction => {
                        if inner.contains(&HyperLTL::constant_false()) {
                            return HyperLTL::constant_false();
                        }
                        inner.retain(|subf| subf != &HyperLTL::constant_true());
                        if inner.is_empty() {
                            return HyperLTL::constant_true();
                        } else if inner.len() == 1 {
                            return inner.pop().unwrap();
                        }
                        HyperLTL::Appl(Op::Conjunction, inner)
                    }
                    Op::Disjunction => {
                        if inner.contains(&HyperLTL::constant_true()) {
                            return HyperLTL::constant_true();
                        }
                        inner.retain(|subf| subf != &HyperLTL::constant_false());
                        if inner.is_empty() {
                            return HyperLTL::constant_false();
                        } else if inner.len() == 1 {
                            return inner.pop().unwrap();
                        }
                        HyperLTL::Appl(Op::Disjunction, inner)
                    }
                    op => HyperLTL::Appl(op, inner),
                }
            }
            Prop(name, path) => Prop(name, path),
            _ => unreachable!(),
        }
    }

    /// checks whether an LTL formula is in the syntactic safety fragment
    fn is_syntactic_safe(&self) -> bool {
        assert!(self.is_ltl());

        match self {
            Appl(op, inner) => op.is_safety() && inner.iter().all(|subf| subf.is_syntactic_safe()),
            Prop(_, _) => true,
            _ => unreachable!(),
        }
    }

    /// checks whether an LTL formula is an invariant, i.e., of the form `G (propositional)`
    fn is_invariant(&self) -> bool {
        assert!(self.is_ltl());

        match self {
            Appl(Globally, inner) => inner[0].is_propositional(),
            _ => false,
        }
    }

    /// checks whether an LTL formula is propositional, i.e., does not contain temporal operators
    fn is_propositional(&self) -> bool {
        assert!(self.is_ltl());

        match self {
            Appl(op, inner) => {
                op.is_propositional() && inner.iter().all(|subf| subf.is_propositional())
            }
            Prop(_, _) => true,
            _ => unreachable!(),
        }
    }

    /// checks whether an LTL formula is invariant up to one Next operator, e.g., `G (a <-> X a)`
    fn is_prime_invariant(&self) -> bool {
        assert!(self.is_ltl());

        match self {
            Appl(Globally, inner) => inner[0].is_nearly_propositional(false),
            _ => false,
        }
    }

    /// checks whether an LTL formula is nearly propositional, i.e., the only temporal operator is `X` with nesting depth 1
    fn is_nearly_propositional(&self, mut seen_x: bool) -> bool {
        assert!(self.is_ltl());

        match self {
            Appl(op, inner) => {
                if *op == Next {
                    if seen_x {
                        return false;
                    }
                    seen_x = true;
                } else if !op.is_propositional() {
                    return false;
                }
                inner
                    .iter()
                    .all(|subf| subf.is_nearly_propositional(seen_x))
            }
            Prop(_, _) => true,
            _ => unreachable!(),
        }
    }

    /// checks whether an LTL formula is an invariant, i.e., of the form `G (propositional)`
    fn is_reccurrence(&self) -> bool {
        assert!(self.is_ltl());

        match self {
            Appl(Globally, inner) => match &inner[0] {
                Appl(Finally, inner) => inner[0].is_propositional(),
                _ => false,
            },
            _ => false,
        }
    }

    pub fn partition(self) -> LTLPartitioning {
        assert!(self.is_ltl());

        let mut assumptions: Vec<HyperLTL> = Vec::new();
        let mut guarantee: Option<HyperLTL> = None;

        //println!("partition {}", &self);

        match self {
            Appl(Disjunction, inner) => {
                // may be of form assumptions => guarantees
                for subf in inner.into_iter() {
                    match subf {
                        Appl(Conjunction, inner) => {
                            match guarantee {
                                None => guarantee = Some(Appl(Conjunction, inner)),
                                Some(Appl(Conjunction, other)) => {
                                    // check which one has more conjuncts
                                    if inner.len() > other.len() {
                                        // switch
                                        guarantee = Some(Appl(Conjunction, inner));
                                        assumptions.push(Appl(Conjunction, other));
                                    } else {
                                        guarantee = Some(Appl(Conjunction, other));
                                        assumptions.push(Appl(Conjunction, inner));
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => {
                            assumptions.push(subf);
                        }
                    }
                }
            }
            Appl(Conjunction, inner) => {
                // no (global) assumptions
                guarantee = Some(Appl(Conjunction, inner));
            }
            Quant(_, _, _) => unreachable!(),
            f => guarantee = Some(f),
        }

        assert!(guarantee.is_some() || !assumptions.is_empty());

        let guarantees = if let Some(guarantee) = guarantee {
            if let Appl(Conjunction, inner) = guarantee {
                inner
            } else {
                return LTLPartitioning {
                    preset_assumptions: Vec::new(),
                    preset_guarantees: Vec::new(),
                    invariant_assumptions: Vec::new(),
                    invariant_guarantees: Vec::new(),
                    prime_invariant_assumptions: Vec::new(),
                    prime_invariant_guarantees: Vec::new(),
                    safety_assumptions: Vec::new(),
                    safety_guarantees: Vec::new(),
                    reccurrence_assumptions: Vec::new(),
                    reccurrence_guarantees: Vec::new(),
                    liveness_assumptions: Vec::new(),
                    liveness_guarantees: vec![guarantee],
                };
            }
        } else {
            return LTLPartitioning {
                preset_assumptions: Vec::new(),
                preset_guarantees: Vec::new(),
                invariant_assumptions: Vec::new(),
                invariant_guarantees: Vec::new(),
                prime_invariant_assumptions: Vec::new(),
                prime_invariant_guarantees: Vec::new(),
                safety_assumptions: Vec::new(),
                safety_guarantees: Vec::new(),
                reccurrence_assumptions: Vec::new(),
                reccurrence_guarantees: Vec::new(),
                liveness_assumptions: Vec::new(),
                liveness_guarantees: vec![assumptions
                    .into_iter()
                    .fold(HyperLTL::constant_false(), |val, ass| {
                        HyperLTL::new_binary(Op::Disjunction, val, ass)
                    })],
            };
        };

        // negate assumptions
        assumptions = assumptions
            .into_iter()
            .map(|subf| subf.to_nnf(true))
            .collect();

        //println!("assumptions {:?}", assumptions);
        //println!("guarantees {:?}", guarantees);

        // filter
        let (safety_assumptions, liveness_assumptions): (Vec<HyperLTL>, Vec<HyperLTL>) =
            assumptions
                .into_iter()
                .partition(|subf| subf.is_syntactic_safe());
        let (invariant_assumptions, safety_assumptions): (Vec<HyperLTL>, Vec<HyperLTL>) =
            safety_assumptions
                .into_iter()
                .partition(|subf| subf.is_invariant());
        let (preset_assumptions, safety_assumptions): (Vec<HyperLTL>, Vec<HyperLTL>) =
            safety_assumptions
                .into_iter()
                .partition(|subf| subf.is_propositional());
        let (prime_invariant_assumptions, safety_assumptions): (Vec<HyperLTL>, Vec<HyperLTL>) =
            safety_assumptions
                .into_iter()
                .partition(|subf| subf.is_prime_invariant());
        let (reccurrence_assumptions, liveness_assumptions): (Vec<HyperLTL>, Vec<HyperLTL>) =
            liveness_assumptions
                .into_iter()
                .partition(|subf| subf.is_reccurrence());

        let (safety_guarantees, liveness_guarantees): (Vec<HyperLTL>, Vec<HyperLTL>) = guarantees
            .into_iter()
            .partition(|subf| subf.is_syntactic_safe());
        let (invariant_guarantees, safety_guarantees): (Vec<HyperLTL>, Vec<HyperLTL>) =
            safety_guarantees
                .into_iter()
                .partition(|subf| subf.is_invariant());
        let (preset_guarantees, safety_guarantees): (Vec<HyperLTL>, Vec<HyperLTL>) =
            safety_guarantees
                .into_iter()
                .partition(|subf| subf.is_propositional());
        let (prime_invariant_guarantees, safety_guarantees): (Vec<HyperLTL>, Vec<HyperLTL>) =
            safety_guarantees
                .into_iter()
                .partition(|subf| subf.is_prime_invariant());
        let (reccurrence_guarantees, liveness_guarantees): (Vec<HyperLTL>, Vec<HyperLTL>) =
            liveness_guarantees
                .into_iter()
                .partition(|subf| subf.is_reccurrence());

        LTLPartitioning {
            preset_assumptions,
            preset_guarantees,
            invariant_assumptions,
            invariant_guarantees,
            prime_invariant_assumptions,
            prime_invariant_guarantees,
            safety_assumptions,
            safety_guarantees,
            reccurrence_assumptions,
            reccurrence_guarantees,
            liveness_assumptions,
            liveness_guarantees,
        }
    }
}

#[derive(Debug, Default)]
pub struct LTLPartitioning {
    pub preset_assumptions: Vec<HyperLTL>,
    pub preset_guarantees: Vec<HyperLTL>,
    pub invariant_assumptions: Vec<HyperLTL>,
    pub invariant_guarantees: Vec<HyperLTL>,
    pub prime_invariant_assumptions: Vec<HyperLTL>,
    pub prime_invariant_guarantees: Vec<HyperLTL>,
    pub safety_assumptions: Vec<HyperLTL>,
    pub safety_guarantees: Vec<HyperLTL>,
    pub reccurrence_assumptions: Vec<HyperLTL>,
    pub reccurrence_guarantees: Vec<HyperLTL>,
    pub liveness_assumptions: Vec<HyperLTL>,
    pub liveness_guarantees: Vec<HyperLTL>,
}
impl std::fmt::Display for LTLPartitioning {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "preset assumptions ({}):", self.preset_assumptions.len())?;
        for subf in &self.preset_assumptions {
            writeln!(f, "\t{}", subf)?;
        }
        writeln!(f, "preset guarantees ({}):", self.preset_guarantees.len())?;
        for subf in &self.preset_guarantees {
            writeln!(f, "\t{}", subf)?;
        }

        writeln!(
            f,
            "invariant assumptions ({}):",
            self.invariant_assumptions.len()
        )?;
        for subf in &self.invariant_assumptions {
            writeln!(f, "\t{}", subf)?;
        }
        writeln!(
            f,
            "invariant guarantees ({}):",
            self.invariant_guarantees.len()
        )?;
        for subf in &self.invariant_guarantees {
            writeln!(f, "\t{}", subf)?;
        }

        writeln!(
            f,
            "prime invariant assumptions ({}):",
            self.prime_invariant_assumptions.len()
        )?;
        for subf in &self.prime_invariant_assumptions {
            writeln!(f, "\t{}", subf)?;
        }
        writeln!(
            f,
            "prime invariant guarantees ({}):",
            self.prime_invariant_guarantees.len()
        )?;
        for subf in &self.prime_invariant_guarantees {
            writeln!(f, "\t{}", subf)?;
        }

        writeln!(f, "safety assumptions ({}):", self.safety_assumptions.len())?;
        for subf in &self.safety_assumptions {
            writeln!(f, "\t{}", subf)?;
        }
        writeln!(f, "safety guarantees ({}):", self.safety_guarantees.len())?;
        for subf in &self.safety_guarantees {
            writeln!(f, "\t{}", subf)?;
        }

        writeln!(
            f,
            "reccurrence assumptions ({}):",
            self.reccurrence_assumptions.len()
        )?;
        for subf in &self.reccurrence_assumptions {
            writeln!(f, "\t{}", subf)?;
        }
        writeln!(
            f,
            "reccurrence guarantees ({}):",
            self.reccurrence_guarantees.len()
        )?;
        for subf in &self.reccurrence_guarantees {
            writeln!(f, "\t{}", subf)?;
        }

        writeln!(
            f,
            "liveness assumptions ({}):",
            self.liveness_assumptions.len()
        )?;
        for subf in &self.liveness_assumptions {
            writeln!(f, "\t{}", subf)?;
        }
        writeln!(
            f,
            "liveness guarantees ({}):",
            self.liveness_guarantees.len()
        )?;
        for subf in &self.liveness_guarantees {
            writeln!(f, "\t{}", subf)?;
        }
        Ok(())
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
        let props = expr.get_propositions();
        assert!(props.contains("a"));
        assert!(props.contains("b"));
        assert!(props.contains("c"));
    }

    #[test]
    fn test_removed_derived_implication() {
        let mut before = Appl(
            Implication,
            vec![Prop("a".into(), None), Prop("b".into(), None)],
        );
        let after = Appl(
            Disjunction,
            vec![
                Appl(Negation, vec![Prop("a".into(), None)]),
                Prop("b".into(), None),
            ],
        );
        before.remove_derived();

        assert_eq!(before, after);
    }

    #[test]
    fn test_removed_derived_weak_until() {
        let mut before = Appl(
            WeakUntil,
            vec![Prop("a".into(), None), Prop("b".into(), None)],
        );
        let after = Appl(
            Disjunction,
            vec![
                Appl(Globally, vec![Prop("a".into(), None)]),
                Appl(Until, vec![Prop("a".into(), None), Prop("b".into(), None)]),
            ],
        );
        before.remove_derived();

        assert_eq!(before, after);
    }

    #[test]
    fn test_nnf_transformation() {
        let a = Prop("a".into(), None);
        let b = Prop("b".into(), None);
        let before = Appl(
            Negation,
            vec![Appl(
                Disjunction,
                vec![
                    Appl(Globally, vec![a.clone()]),
                    Appl(Until, vec![a.clone(), b.clone()]),
                ],
            )],
        );
        let after = HyperLTL::new_binary(
            Conjunction,
            HyperLTL::new_unary(Finally, HyperLTL::new_unary(Negation, a.clone())),
            HyperLTL::new_binary(
                Release,
                HyperLTL::new_unary(Negation, a.clone()),
                HyperLTL::new_unary(Negation, b.clone()),
            ),
        );

        assert_eq!(before.to_nnf(false), after);
    }

    #[test]
    fn test_flatten() {
        let a = Prop("a".into(), None);
        let b = Prop("b".into(), None);
        let c = Prop("c".into(), None);

        let before = HyperLTL::new_binary(
            Conjunction,
            HyperLTL::new_binary(
                Conjunction,
                a.clone(),
                HyperLTL::new_binary(Conjunction, b.clone(), c.clone()),
            ),
            HyperLTL::new_binary(
                Disjunction,
                HyperLTL::new_binary(Disjunction, a.clone(), b.clone()),
                c.clone(),
            ),
        );
        let after = Appl(
            Conjunction,
            vec![
                a.clone(),
                b.clone(),
                c.clone(),
                Appl(Disjunction, vec![a.clone(), b.clone(), c.clone()]),
            ],
        );

        assert_eq!(before.flatten(), after);
    }
}
