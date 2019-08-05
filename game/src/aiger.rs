use crate::safety::SafetyGame;
use aiger::{Aiger, AigerLit};
use cudd::{CuddManager, CuddNode};
use maplit::hashmap;
use std::collections::HashMap;

impl From<&Aiger> for SafetyGame {
    fn from(aiger: &Aiger) -> SafetyGame {
        assert!(aiger.is_reencoded());

        let mut controllable_lits: Vec<AigerLit> = Vec::new();
        let mut uncontrollable_lits: Vec<AigerLit> = Vec::new();

        let mut manager = CuddManager::new();
        let mut cache: HashMap<AigerLit, CuddNode> = hashmap![AigerLit::FALSE => manager.zero()];
        let mut controllables: Vec<CuddNode> = Vec::new();
        let mut uncontrollables: Vec<CuddNode> = Vec::new();
        let mut latches: Vec<CuddNode> = Vec::new();
        let mut compose: Vec<CuddNode> = Vec::new();

        for input in aiger.inputs() {
            // TODO: check if it makes if difference if one partitions CUDD nodes by controllable/uncontrollable instead of mix between them
            let mut node = manager.new_var();
            node.set_primary_input();
            cache.insert(input.lit(), node.clone());

            let controllable = if let Some(name) = input.name() {
                name.starts_with("controllable")
            } else {
                false
            };

            if controllable {
                controllable_lits.push(input.lit());
                controllables.push(node.clone());
            } else {
                uncontrollable_lits.push(input.lit());
                uncontrollables.push(node.clone());
            }
            compose.push(node);
        }

        let controllable_names: Vec<String> = controllable_lits
            .iter()
            .map(|lit| format!("{}", lit))
            .collect();
        let uncontrollable_names: Vec<String> = uncontrollable_lits
            .iter()
            .map(|lit| format!("{}", lit))
            .collect();
        let latch_names: Vec<String> = aiger
            .latches()
            .map(|symbol| format!("{}", symbol.lit()))
            .collect();

        assert_eq!(controllables.len(), controllable_lits.len());
        assert_eq!(uncontrollables.len(), uncontrollable_lits.len());

        for latch in aiger.latches() {
            let mut node = manager.new_var();
            node.set_present_state();
            cache.insert(latch.lit(), node.clone());
            latches.push(node);
        }

        assert_eq!(latches.len(), latch_names.len());

        fn lookup_literal(cache: &HashMap<AigerLit, CuddNode>, lit: &AigerLit) -> CuddNode {
            let (negated, normalized_lit) = lit.normalize();
            let bdd_node = cache[&normalized_lit].clone();
            if negated {
                !bdd_node
            } else {
                bdd_node
            }
        }

        for and in aiger.ands() {
            let node =
                lookup_literal(&cache, &and.rhs0()).and(&lookup_literal(&cache, &and.rhs1()));
            cache.insert(and.lhs(), node);
        }

        let initial =
            latches
                .iter()
                .zip(aiger.latches())
                .fold(manager.one(), |init, (node, latch)| match latch.reset() {
                    0 => init.and(&!node),
                    1 => init.and(node),
                    _ => unreachable!(
                        "error in AIGER input: only 0 and 1 is allowed for initial latch values"
                    ),
                });

        compose.extend(
            aiger
                .latches()
                .map(|latch| lookup_literal(&cache, &latch.next())),
        );

        let safety_condition = aiger.outputs().fold(manager.one(), |safe, output| {
            lookup_literal(&cache, &output.lit())
        });

        SafetyGame {
            manager,
            controllables,
            uncontrollables,
            latches,
            compose,
            initial,
            safety_condition,

            controllable_names,
            uncontrollable_names,
            latch_names,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::safety::SafetyGameSolver;
    use bosy::specification::Semantics;

    #[test]
    fn test_unrealizable() {
        let aiger = Aiger::from_str(
            "aag 4 2 1 1 1
2
4
6 2
9
8 6 5
i0 i_r
i1 controllable_g
l0 r-latch
o0 err
c
G(r & !Xg) = G(r) & G(!Xg)
unrealizable

",
        )
        .unwrap();

        let safety_game = SafetyGame::from(&aiger);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_none());
    }
}
