use crate::safety::SafetyGame;
use bosy::automata::conversion::LTL2Automaton;
use bosy::specification::Specification;
use cudd::{CuddManager, CuddNode};
use hyperltl::{HyperLTL, Op};
use log::info;
use std::ops::Not;

impl<'a> SafetyGame<'a> {
    #[allow(clippy::cognitive_complexity)]
    pub fn from_bosy(spec: &Specification, manager: &'a CuddManager, bound: u32) -> Self {
        info!("Build safety game from BoSy spec with bound {}", bound);
        assert!(spec.hyper().is_none());

        let ltl = spec.ltl();
        info!("LTL specification is {}", ltl);

        let partitioned = ltl.normalize().partition().unwrap();

        if !partitioned.liveness_assumptions.is_empty()
            || !partitioned.reccurrence_assumptions.is_empty()
        {
            //panic!("not applicable");
            println!("detected liveness assumptions");
        }

        info!("partitioned\n\n{}", partitioned);

        // build a safety game with a single livness counter of bound `bound`

        // (1) create BDD nodes for inputs and outputs
        let mut controllables: Vec<CuddNode> = Vec::new();
        let mut uncontrollables: Vec<CuddNode> = Vec::new();
        let mut compose: Vec<CuddNode> = Vec::new();

        let mut controllable_names: Vec<String> = Vec::new();
        let mut uncontrollable_names: Vec<String> = Vec::new();

        for input in spec.inputs() {
            let mut node = manager.new_var();
            node.set_primary_input();
            uncontrollables.push(node.clone());
            compose.push(node);
            uncontrollable_names.push(input.clone());
        }

        for output in spec.outputs() {
            let node = manager.new_var();
            controllables.push(node.clone());
            compose.push(node);
            controllable_names.push(output.clone());
        }

        assert_eq!(controllables.len(), controllable_names.len());
        assert_eq!(uncontrollables.len(), uncontrollable_names.len());

        let mut latches: Vec<CuddNode> = Vec::new();
        let mut latch_names: Vec<String> = Vec::new();
        let mut initial_condition: Vec<CuddNode> = Vec::new();

        // (2.0) create latch that is only initially true (and otherwise always false)
        let initial_node = manager.new_var();
        latches.push(initial_node.clone());
        latch_names.push(String::from("reg_init"));
        compose.push(manager.zero());
        initial_condition.push(initial_node.clone());

        // (2.1) create copy for values of input and output of previous step
        for (node, name) in uncontrollables
            .iter()
            .chain(&controllables)
            .zip(uncontrollable_names.iter().chain(&controllable_names))
        {
            let next_node = manager.new_var();
            latches.push(next_node.clone());
            latch_names.push(String::from("reg_") + name);

            compose.push(node.clone());
            initial_condition.push(!next_node);
        }

        let mut env_safe: Vec<CuddNode> = Vec::new();
        let mut sys_safe: Vec<CuddNode> = Vec::new();

        // (2.2) safety automata
        for safety_assumption in &partitioned.safety_assumptions {
            let LivenessAutomatonEncoding {
                state_nodes,
                incoming,
                state_names,
                initial_state,
                fair,
            } = build_liveness_automaton(
                safety_assumption,
                &manager,
                &uncontrollables,
                &uncontrollable_names,
                &controllables,
                &controllable_names,
            );

            latches.extend(state_nodes);
            latch_names.extend(state_names);
            compose.extend(incoming);
            initial_condition.extend(initial_state);
            env_safe.extend(fair);
        }
        for safety_guarantee in &partitioned.safety_guarantees {
            let LivenessAutomatonEncoding {
                state_nodes,
                incoming,
                state_names,
                initial_state,
                fair,
            } = build_liveness_automaton(
                safety_guarantee,
                &manager,
                &uncontrollables,
                &uncontrollable_names,
                &controllables,
                &controllable_names,
            );

            latches.extend(state_nodes);
            latch_names.extend(state_names);
            compose.extend(incoming);
            initial_condition.extend(initial_state);
            sys_safe.extend(fair);
        }

        let mut sys_fair: Vec<CuddNode> = Vec::new();
        let mut env_fair: Vec<CuddNode> = Vec::new();

        // (2.3) liveness automata
        for liveness_assumption in &partitioned.liveness_assumptions {
            let LivenessAutomatonEncoding {
                state_nodes,
                incoming,
                state_names,
                initial_state,
                fair,
            } = build_liveness_automaton(
                liveness_assumption,
                &manager,
                &uncontrollables,
                &uncontrollable_names,
                &controllables,
                &controllable_names,
            );

            latches.extend(state_nodes);
            latch_names.extend(state_names);
            compose.extend(incoming);
            initial_condition.extend(initial_state);
            env_fair.extend(fair);
        }
        for liveness_guarantee in &partitioned.liveness_guarantees {
            let LivenessAutomatonEncoding {
                state_nodes,
                incoming,
                state_names,
                initial_state,
                fair,
            } = build_liveness_automaton(
                liveness_guarantee,
                &manager,
                &uncontrollables,
                &uncontrollable_names,
                &controllables,
                &controllable_names,
            );

            /*incoming.iter().enumerate().for_each(|(i, s)| {
                println!("<incoming {}>", i);
                s.print_minterms();
                assert_ne!(s, &manager.zero());
                println!("</incoming {}>", i);
            });
            initial_state.iter().enumerate().for_each(|(i, s)| {
                println!("<initial {}>", i);
                s.print_minterms();
                assert_ne!(s, &manager.zero());
                println!("</initial {}>", i);
            });

            println!("<sys_fair>");
            fair.print_minterms();
            println!("</sys_fair>");*/

            latches.extend(state_nodes);
            latch_names.extend(state_names);
            compose.extend(incoming);
            initial_condition.extend(initial_state);
            sys_fair.extend(fair);
        }

        // (3) build constraints from LTL

        // (3.1) preset
        for preset_assumption in &partitioned.preset_assumptions {
            env_safe.push(initial_node.implies(&translate(
                preset_assumption,
                true,
                manager,
                &uncontrollables,
                &uncontrollable_names,
                &controllables,
                &controllable_names,
                &latches,
            )));
        }
        for preset_guarantee in &partitioned.preset_guarantees {
            sys_safe.push(initial_node.implies(&translate(
                preset_guarantee,
                true,
                manager,
                &uncontrollables,
                &uncontrollable_names,
                &controllables,
                &controllable_names,
                &latches,
            )));
        }

        // (3.2) invariants
        for invariant_assumption in &partitioned.invariant_assumptions {
            if let HyperLTL::Appl(Op::Globally, inner) = &invariant_assumption {
                env_safe.push(translate(
                    &inner[0],
                    true,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                ));
            } else {
                panic!("{} not an invariant", invariant_assumption);
            }
        }
        for invariant_guarantee in &partitioned.invariant_guarantees {
            if let HyperLTL::Appl(Op::Globally, inner) = &invariant_guarantee {
                sys_safe.push(translate(
                    &inner[0],
                    true,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                ));
            } else {
                panic!("{} not an invariant", invariant_guarantee);
            }
        }

        // (3.3) primed invariants
        for prime_invariant_assumption in &partitioned.prime_invariant_assumptions {
            if let HyperLTL::Appl(Op::Globally, inner) = &prime_invariant_assumption {
                env_safe.push((!&initial_node).implies(&translate(
                    &inner[0],
                    false,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                )));
            } else {
                panic!("{} not a prime invariant", prime_invariant_assumption);
            }
        }
        for prime_invariant_guarantee in &partitioned.prime_invariant_guarantees {
            if let HyperLTL::Appl(Op::Globally, inner) = &prime_invariant_guarantee {
                sys_safe.push((!&initial_node).implies(&translate(
                    &inner[0],
                    false,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                )));
            } else {
                panic!("{} not a prime invariant", prime_invariant_guarantee);
            }
        }

        // (3.4) reccurrence
        for reccurrence_assumption in &partitioned.reccurrence_assumptions {
            if let HyperLTL::Appl(Op::Globally, inner) = &reccurrence_assumption {
                if let HyperLTL::Appl(Op::Finally, inner) = &inner[0] {
                    env_fair.push(translate(
                        &inner[0],
                        false,
                        manager,
                        &uncontrollables,
                        &uncontrollable_names,
                        &controllables,
                        &controllable_names,
                        &latches,
                    ));
                /*println!("<env_fair>");
                env_fair.last().unwrap().print_minterms();
                println!("</env_fair>");*/
                } else {
                    panic!("{} not an reccurrence", reccurrence_assumption);
                }
            } else {
                panic!("{} not an reccurrence", reccurrence_assumption);
            }
        }
        for reccurrence_guarantee in &partitioned.reccurrence_guarantees {
            if let HyperLTL::Appl(Op::Globally, inner) = &reccurrence_guarantee {
                if let HyperLTL::Appl(Op::Finally, inner) = &inner[0] {
                    sys_fair.push(translate(
                        &inner[0],
                        false,
                        manager,
                        &uncontrollables,
                        &uncontrollable_names,
                        &controllables,
                        &controllable_names,
                        &latches,
                    ));
                } else {
                    panic!("{} not an reccurrence", reccurrence_guarantee);
                }
            } else {
                panic!("{} not an reccurrence", reccurrence_guarantee);
            }
        }

        let env_safe_err = env_safe
            .into_iter()
            .fold(manager.zero(), |val, safe| val.or(&!safe));
        let sys_safe_err = sys_safe
            .into_iter()
            .fold(manager.zero(), |val, safe| val.or(&!safe));

        /*println!("<sys_safe_err>");
        sys_safe_err.print_minterms();
        println!("</sys_safe_err>");*/

        // we have to remember if env_safe error happened
        let env_safe_err_happened = manager.new_var();
        latches.push(env_safe_err_happened.clone());
        latch_names.push(String::from("env_safe_err_happened"));
        compose.push(env_safe_err_happened.or(&env_safe_err.clone()));
        initial_condition.push(!env_safe_err_happened.clone());

        // we have to remember every sys_fair..
        let sys_fair_done: Vec<CuddNode> = sys_fair.iter().map(|_| manager.new_var()).collect();

        let all_sys_fair_fulfilled = sys_fair_done
            .iter()
            .zip(&sys_fair)
            .fold(manager.one(), |val, (done, fair)| val.and(&done.or(&fair)));

        let progress_in_sys_fair = sys_fair_done
            .iter()
            .zip(&sys_fair)
            .fold(manager.zero(), |val, (done, fair)| {
                val.or(&done.not().and(&fair))
            });

        /*println!("<all_sys_fair_fulfilled>");
        all_sys_fair_fulfilled.print_minterms();
        println!("</all_sys_fair_fulfilled>");

        println!("<progress_in_sys_fair>");
        progress_in_sys_fair.print_minterms();
        println!("</progress_in_sys_fair>");

        println!("<!all_sys_fair_fulfilled>");
        all_sys_fair_fulfilled.not().print_minterms();
        println!("</!all_sys_fair_fulfilled>");

        println!("<!all_sys_fair_fulfilled & !progress_in_sys_fair>");
        (all_sys_fair_fulfilled.not())
            .and(&!progress_in_sys_fair.clone())
            .print_minterms();
        println!("</!all_sys_fair_fulfilled & !progress_in_sys_fair>");*/

        // ..and env_fair
        let env_fair_done: Vec<CuddNode> = env_fair.iter().map(|_| manager.new_var()).collect();
        let all_env_fair_fulfilled = env_fair_done
            .iter()
            .zip(&env_fair)
            .fold(manager.one(), |val, (done, fair)| val.and(&done.or(&fair)));

        /*println!("<all_env_fair_fulfilled>");
        all_env_fair_fulfilled.print_minterms();
        println!("</all_env_fair_fulfilled>");*/

        // build counter constraint, initial zero
        let counter_latches: Vec<CuddNode> = (0..bound).map(|_| manager.new_var()).collect();
        initial_condition.extend(counter_latches.iter().map(|n| !n));

        let env_fair_done_compose: Vec<CuddNode> = env_fair_done
            .iter()
            .zip(&env_fair)
            .map(|(done, fair)| {
                (!all_sys_fair_fulfilled.clone())
                    .and(&!progress_in_sys_fair.clone())
                    .and(&!all_env_fair_fulfilled.clone())
                    .and(&done.or(fair))
            })
            .collect();

        /*for c in &env_fair_done_compose {
            println!("<env_fair_done_compose>");
            c.print_minterms();
            //assert_ne!(c, &manager.zero());
            println!("</env_fair_done_compose>");
        }*/

        let sys_fair_done_compose: Vec<CuddNode> = sys_fair_done
            .iter()
            .zip(&sys_fair)
            .map(|(done, fair)| {
                all_sys_fair_fulfilled.ite(
                    &manager.zero(),
                    &progress_in_sys_fair.ite(&(done.or(fair)), done),
                )
            })
            .collect();

        /*for c in &sys_fair_done_compose {
            println!("<sys_fair_done_compose>");
            c.print_minterms();
            //assert_ne!(c, &manager.zero());
            println!("</sys_fair_done_compose>");
        }*/

        let counter_compose: Vec<CuddNode> = counter_latches
            .iter()
            .enumerate()
            .map(|(i, counter)| {
                all_sys_fair_fulfilled.ite(
                    &manager.zero(),
                    &progress_in_sys_fair.ite(
                        &manager.zero(),
                        &all_env_fair_fulfilled.ite(
                            &(if i == 0 {
                                counter.xor(&manager.one())
                            } else {
                                (0..i)
                                    .map(|j| &counter_latches[j])
                                    .fold(manager.one(), |val, count| val.and(count))
                                    .ite(&counter.xor(&manager.one()), &counter)
                            }),
                            &counter,
                        ),
                    ),
                )
            })
            .collect();

        /*for c in &counter_compose {
            println!("<counter_compose>");
            c.print_minterms();
            //assert_ne!(c, &manager.zero());
            println!("</counter_compose>");
        }*/

        // add latches for sys_fair_done and env_fair_done
        initial_condition.extend(sys_fair_done.iter().map(|n| !n));
        latch_names.extend(
            sys_fair_done
                .iter()
                .enumerate()
                .map(|(i, _)| format!("sys_fair_{}", i)),
        );
        latches.extend(sys_fair_done);
        compose.extend(sys_fair_done_compose);

        initial_condition.extend(env_fair_done.iter().map(|n| !n));
        latch_names.extend(
            env_fair_done
                .iter()
                .enumerate()
                .map(|(i, _)| format!("env_fair_{}", i)),
        );
        latches.extend(env_fair_done);
        compose.extend(env_fair_done_compose);

        // add counter latches
        latches.extend(counter_latches.clone());
        latch_names.extend((0..bound).map(|i| format!("counter_{}", i)));
        compose.extend(counter_compose);

        let fair_err = counter_latches
            .iter()
            .fold(manager.one(), |val, bit| val.and(bit));

        /*println!("<fair_err>");
        fair_err.print_minterms();
        println!("</fair_err>");*/

        // build safety condition
        let safety_condition = (!env_safe_err)
            .and(&!env_safe_err_happened)
            .implies(&(!sys_safe_err).and(&!fair_err));
        //assign o_err = ~env_safe_err & ~env_safe_err_happened & (sys_safe_err | fair_err);

        //println!("<safety_condition>");
        //safety_condition.print_minterms();
        //println!("</safety_condition>");

        let initial_condition = initial_condition
            .into_iter()
            .fold(manager.one(), |val, init| val.and(&init));

        /*println!("<initial_condition>");
        initial_condition.print_minterms();
        println!("</initial_condition>");

        println!("{:?}", latch_names);*/

        assert_eq!(latches.len(), latch_names.len());

        SafetyGame {
            manager,
            controllables,
            uncontrollables,
            latches,
            compose,
            initial_condition,
            safety_condition,

            controllable_names,
            uncontrollable_names,
            latch_names,
        }
    }
}

/// translates a guard given as `smtlib::Term` into BDD
fn translate_guard<'a>(
    term: &smtlib::Term,
    manager: &'a CuddManager,
    inputs: &[CuddNode<'a>],
    input_names: &[String],
    outputs: &[CuddNode<'a>],
    output_names: &[String],
) -> CuddNode<'a> {
    use smtlib::{BoolFun, IdentDecl, IdentKind, TermKind};
    match &term.kind {
        TermKind::Ident(ident) => match &ident.kind {
            IdentKind::BooleanFun(BoolFun::True) => manager.one(),
            IdentKind::BooleanFun(BoolFun::False) => manager.zero(),
            IdentKind::Custom(ident_decl) => match ident_decl.as_ref() {
                IdentDecl::Func(name, _, _) => {
                    if let Some(i) = input_names.iter().position(|n| n == name) {
                        inputs[i].clone()
                    } else if let Some(o) = output_names.iter().position(|n| n == name) {
                        outputs[o].clone()
                    } else {
                        unreachable!("Proposition `{}` not found", name);
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        TermKind::Appl(ident, inner) => {
            let inner: Vec<CuddNode> = inner
                .iter()
                .map(|f| translate_guard(f, manager, inputs, input_names, outputs, output_names))
                .collect();
            match &ident.kind {
                IdentKind::BooleanFun(BoolFun::Not) => !inner[0].clone(),
                IdentKind::BooleanFun(BoolFun::And) => {
                    inner.into_iter().fold(manager.one(), |val, g| val.and(&g))
                }
                IdentKind::BooleanFun(BoolFun::Or) => {
                    inner.into_iter().fold(manager.zero(), |val, g| val.or(&g))
                }
                f => unreachable!("unknown fun {:?}", f),
            }
        }
        t => unreachable!("unexepected term {:?}", t),
    }
}

#[derive(Debug)]
struct LivenessAutomatonEncoding<'a> {
    state_nodes: Vec<CuddNode<'a>>,
    incoming: Vec<CuddNode<'a>>,
    state_names: Vec<String>,
    initial_state: Vec<CuddNode<'a>>,
    fair: Vec<CuddNode<'a>>,
}

fn build_liveness_automaton<'a>(
    ltl: &HyperLTL,
    manager: &'a CuddManager,
    inputs: &[CuddNode<'a>],
    input_names: &[String],
    outputs: &[CuddNode<'a>],
    output_names: &[String],
) -> LivenessAutomatonEncoding<'a> {
    let automaton =
        match LTL2Automaton::Spot.to_ucw(&HyperLTL::new_unary(Op::Negation, ltl.clone())) {
            Err(err) => {
                eprintln!("failed to convert LTL to automaton");
                eprintln!("{}", err);
                panic!();
            }
            Ok(automaton) => automaton,
        };
    let mut state_nodes: Vec<CuddNode> = Vec::new();
    let mut state_names: Vec<String> = Vec::new();
    let mut initial_state: Vec<CuddNode> = Vec::new();
    let mut fair: Vec<CuddNode> = Vec::new();

    for state in automaton.states() {
        let node = manager.new_var();
        if state.initial {
            initial_state.push(node.clone());
        } else {
            initial_state.push(node.clone().not());
        }
        /*if state.rejecting {
            // only count real rejecting runs, i.e., self loops
            let guard = automaton
                .outgoing(state)
                .filter(|(s, _)| s.id == state.id)
                .map(|(_, guard)| guard)
                .next()
                .unwrap();
            let translated_guard =
                translate_guard(guard, manager, inputs, input_names, outputs, output_names);
            fair.push(!(node.and(&translated_guard)));
        }*/
        state_nodes.push(node);
        state_names.push(state.name.as_ref().unwrap().clone());
    }

    // rejecting states
    for scc in &automaton.sccs() {
        if !scc.iter().any(|s| s.rejecting) {
            // no rejecting state in SCC
            continue;
        }
        let mut condition = manager.zero();
        for state in scc.iter().filter(|s| s.rejecting) {
            let node = &state_nodes[state.id];
            // only count real rejecting runs, i.e., self loops
            let forbidden = automaton
                .outgoing(state)
                .filter(|(s, _)| scc.contains(s) && s.rejecting)
                .map(|(_, guard)| translate_guard(guard, manager, inputs, input_names, outputs, output_names))
                .fold(manager.one(), |val, guard| val.and(&guard));
            condition = condition.or(&!(node.and(&forbidden)));
        }
        fair.push(condition);
    }

    // build transition function
    let mut incoming: Vec<CuddNode> = state_nodes.iter().map(|_| manager.zero()).collect();
    for state in automaton.states() {
        for (target, guard) in automaton.outgoing(state) {
            //println!("{} -{}-> {}", state.id, guard, target.id);
            let translated_guard =
                translate_guard(guard, manager, inputs, input_names, outputs, output_names);
            incoming[target.id] = incoming[target.id]
                .clone()
                .or(&(state_nodes[state.id].clone() & translated_guard));
        }
    }
    LivenessAutomatonEncoding {
        state_nodes,
        incoming,
        state_names,
        initial_state,
        fair,
    }
}

/// Translates an LTL formula to CUDD
fn translate<'a>(
    ltl: &HyperLTL,
    primed: bool,
    manager: &'a CuddManager,
    inputs: &[CuddNode<'a>],
    input_names: &[String],
    outputs: &[CuddNode<'a>],
    output_names: &[String],
    latches: &[CuddNode<'a>],
) -> CuddNode<'a> {
    match ltl {
        HyperLTL::Appl(op, inner) => match op {
            Op::Next => {
                assert!(!primed);
                translate(
                    &inner[0],
                    true,
                    manager,
                    inputs,
                    input_names,
                    outputs,
                    output_names,
                    latches,
                )
            }
            Op::True => manager.one(),
            Op::False => manager.zero(),
            Op::Negation => !translate(
                &inner[0],
                primed,
                manager,
                inputs,
                input_names,
                outputs,
                output_names,
                latches,
            ),
            Op::Disjunction => inner.iter().fold(manager.zero(), |val, inner| {
                val.or(&translate(
                    inner,
                    primed,
                    manager,
                    inputs,
                    input_names,
                    outputs,
                    output_names,
                    latches,
                ))
            }),
            Op::Conjunction => inner.iter().fold(manager.one(), |val, inner| {
                val.and(&translate(
                    inner,
                    primed,
                    manager,
                    inputs,
                    input_names,
                    outputs,
                    output_names,
                    latches,
                ))
            }),
            Op::Equivalence => {
                let lhs = translate(
                    &inner[0],
                    primed,
                    manager,
                    inputs,
                    input_names,
                    outputs,
                    output_names,
                    latches,
                );
                let rhs = translate(
                    &inner[1],
                    primed,
                    manager,
                    inputs,
                    input_names,
                    outputs,
                    output_names,
                    latches,
                );
                lhs.xnor(&rhs)
            }
            Op::Exclusion => {
                let lhs = translate(
                    &inner[0],
                    primed,
                    manager,
                    inputs,
                    input_names,
                    outputs,
                    output_names,
                    latches,
                );
                let rhs = translate(
                    &inner[1],
                    primed,
                    manager,
                    inputs,
                    input_names,
                    outputs,
                    output_names,
                    latches,
                );
                lhs.xor(&rhs)
            }
            op => unreachable!("`{}` not allowed in this context", op),
        },
        HyperLTL::Prop(name, None) => {
            if let Some(i) = input_names.iter().position(|n| n == name) {
                if !primed {
                    latches[i + 1].clone()
                } else {
                    inputs[i].clone()
                }
            } else if let Some(o) = output_names.iter().position(|n| n == name) {
                if !primed {
                    latches[inputs.len() + o + 1].clone()
                } else {
                    outputs[o].clone()
                }
            } else {
                unreachable!("Proposition `{}` not found", name);
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::safety::SafetyGameSolver;
    use bosy::specification::Semantics;

    #[test]
    fn test_unrealizable_invariant() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["r_0", "r_1"],
    "outputs": ["g_0", "g_1"],
    "assumptions": [],
    "guarantees": [
        "G(g_0 & !g_0)"
    ]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 1);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_none());
    }

    #[test]
    fn test_unrealizable_preset() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["r_0", "r_1"],
    "outputs": ["g_0", "g_1"],
    "assumptions": [],
    "guarantees": [
        "g_0 & !g_0"
    ]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 1);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_none());
    }

    #[test]
    fn test_unrealizable_preset_invariant() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["r_0", "r_1"],
    "outputs": ["g_0", "g_1"],
    "assumptions": [],
    "guarantees": [
        "g_0",
        "G !g_0"
    ]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 1);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_none());
    }

    #[test]
    fn test_unrealizable_liveness() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["r_0", "r_1"],
    "outputs": ["g_0", "g_1"],
    "assumptions": [],
    "guarantees": [
        "G !g_0",
        "F g_0"
    ]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 1);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_none());
    }

    #[test]
    fn test_unrealizable_recurrence() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["r_0", "r_1"],
    "outputs": ["g_0", "g_1"],
    "assumptions": [],
    "guarantees": [
        "G !g_0",
        "GF g_0"
    ]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 1);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_none());
    }

    #[test]
    fn test_unrealizable_arbiter() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["r_0", "r_1"],
    "outputs": ["g_0", "g_1"],
    "assumptions": [],
    "guarantees": [
        "G (! g_0 || ! g_1)",
        "G ( ( r_0 && X r_1) -> F (g_0 && g_1 ))",
        "G ( r_0 -> F g_0)",
        "G ( r_1 -> F g_1)"
    ]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 2);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_none());
    }

    #[test]
    fn test_reccurrence_assumption() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["r_0"],
    "outputs": ["g_0"],
    "assumptions": [
        "GF r_0"
    ],
    "guarantees": [
        "G (g_0 <-> r_0)",
        "GF g_0"
    ]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 1);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_some());
    }

    #[test]
    fn test_rejecting_self_loop() {
        let spec: Specification = serde_json::from_str(
            r#"
{
    "semantics": "mealy",
    "inputs": ["p_0", "p_1", "p_2", "p_3"],
    "outputs": ["acc"],
    "assumptions": [],
    "guarantees": ["G (F p_0 && F p_1) <-> GF acc"]
}
        "#,
        )
        .expect("Specification is not valid");
        assert!(spec.check().is_ok());

        let manager = CuddManager::new();
        let safety_game = SafetyGame::from_bosy(&spec, &manager, 1);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        assert!(solver.solve().is_some());
    }
}
