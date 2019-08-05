use hyperltl::{HyperLTL, Op};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specification {
    pub(crate) semantics: Semantics,
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,
    assumptions: Vec<HyperLTL>,
    guarantees: Vec<HyperLTL>,
    pub(crate) hyper: Option<Vec<HyperLTL>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Semantics {
    #[serde(rename(serialize = "mealy", deserialize = "mealy"))]
    Mealy,
    #[serde(rename(serialize = "moore", deserialize = "moore"))]
    Moore,
}

impl Specification {
    /// Checks the given specification for problems and reports them
    pub fn check(&self) -> Result<(), Vec<String>> {
        let mut failures = Vec::new();
        let mut propositions: HashSet<&str> = HashSet::new();

        for input in &self.inputs {
            if self.outputs.contains(input) {
                failures.push(format!(
                    "input proposition `{}` contained in outputs",
                    input
                ));
            }
            propositions.insert(input.as_ref());
        }
        for output in &self.outputs {
            if self.inputs.contains(output) {
                failures.push(format!(
                    "output proposition `{}` contained in inputs",
                    output
                ));
            }
            propositions.insert(output.as_ref());
        }

        for assumption in &self.assumptions {
            if !assumption.is_quantifier_free() {
                failures.push(format!(
                    "assumptions have to be in LTL, i.e., contain no quantifiers\ngiven formula was `{}`",
                    assumption
                ));
            }
            let not_bounded: HashSet<&str> = assumption
                .get_propositions()
                .difference(&propositions)
                .map(|p| *p)
                .collect();
            if !not_bounded.is_empty() {
                failures.push(format!(
                    "unknown propositions {:?} found in assumption\ngiven formula was `{}`",
                    not_bounded, assumption
                ));
            }
        }

        for guarantee in &self.guarantees {
            if !guarantee.is_quantifier_free() {
                failures.push(format!(
                    "guarantees have to be in LTL, i.e., contain no quantifiers\ngiven formula was `{}`",
                    guarantee
                ));
            }
            let not_bounded: HashSet<&str> = guarantee
                .get_propositions()
                .difference(&propositions)
                .map(|p| *p)
                .collect();
            if !not_bounded.is_empty() {
                failures.push(format!(
                    "unknown propositions {:?} found in guarantee\ngiven formula was `{}`",
                    not_bounded, guarantee
                ));
            }
        }

        if let Some(hyper) = self.hyper.as_ref() {
            for formula in hyper {
                if !formula.is_hyperltl() {
                    failures.push(format!(
                    "LTL formula found in HyperLTL part, move to `guarantees` instead\ngiven formula was `{}`",
                    formula));
                }
                let not_bounded: HashSet<&str> = formula
                    .get_propositions()
                    .difference(&propositions)
                    .map(|p| *p)
                    .collect();
                if !not_bounded.is_empty() {
                    failures.push(format!(
                    "unknown propositions {:?} found in HyperLTL formula\ngiven formula was `{}`",
                    not_bounded, formula
                ));
                }
            }
        }

        if failures.len() == 0 {
            Ok(())
        } else {
            Err(failures)
        }
    }

    /// Returns the combination of assumptions and guarantees as a single LTL formula
    pub fn ltl(&self) -> HyperLTL {
        assert!(self.check().is_ok());
        let assumptions = self
            .assumptions
            .iter()
            .fold(HyperLTL::Appl(Op::True, vec![]), |val, ele| {
                HyperLTL::Appl(Op::Conjunction, vec![val, ele.clone()])
            });
        let guarantees = self
            .guarantees
            .iter()
            .fold(HyperLTL::Appl(Op::True, vec![]), |val, ele| {
                HyperLTL::Appl(Op::Conjunction, vec![val, ele.clone()])
            });
        HyperLTL::Appl(Op::Implication, vec![assumptions, guarantees])
    }

    pub fn hyper(&self) -> &Option<Vec<HyperLTL>> {
        &self.hyper
    }

    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    pub fn inputs(&self) -> &[String] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[String] {
        &self.outputs
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::error::Error;

    #[test]
    fn check_specs() -> Result<(), Box<Error>> {
        let spec: Specification = serde_json::from_str("{\"semantics\": \"mealy\",\"inputs\": [\"r_0\", \"r_1\", \"g_0\"],\"outputs\": [\"g_0\", \"g_1\"],\"assumptions\": [\"F a\"],\"guarantees\": [\"G (!g_0 || ! g_1)\",\"G (r_0 -> F g_0)\",\"G (r_1 -> F g_1)\",\"forall pi : G r_0[pi]\"],\"hyper\": [\"GF g_0\"]}")?;
        assert!(spec.check().is_err());
        Ok(())
    }

}
