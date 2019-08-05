use crate::safety::SafetyGame;
use bosy::specification::Specification;
use cudd::{CuddManager, CuddNode};
use hyperltl::{HyperLTL, Op};
use log::info;

impl<'a> SafetyGame<'a> {
    pub fn from_bosy(spec: &Specification, manager: &'a CuddManager, bound: u32) -> Self {
        info!("Build safety game from BoSy spec with bound {}", bound);
        assert!(spec.hyper().is_none());

        let ltl = spec.ltl();
        info!("LTL specification is {}", ltl);

        let partitioned = ltl.normalize().partition().unwrap();

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
        latches.push(manager.new_var());
        latch_names.push(String::from("reg_init"));
        compose.push(manager.zero());
        initial_condition.push(manager.one());

        // (2.1) create copy for values of input and output of previous step
        for (node, name) in uncontrollables
            .iter()
            .chain(&controllables)
            .zip(uncontrollable_names.iter().chain(&controllable_names))
        {
            let next_node = manager.new_var();
            latches.push(next_node);
            latch_names.push(String::from("reg_") + name);

            compose.push(node.clone());
            initial_condition.push(manager.zero());
        }

        // (2.2) automata
        // TODO

        // (3) build constraints from LTL/Automata
        let mut env_safe: Vec<CuddNode> = Vec::new();
        let mut sys_safe: Vec<CuddNode> = Vec::new();

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
                            &inner[0],
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
                            &inner[0],
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
                        if primed {
                            latches[i + 1].clone()
                        } else {
                            inputs[i].clone()
                        }
                    } else if let Some(o) = output_names.iter().position(|n| n == name) {
                        if primed {
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

        // (3.1) preset
        let initial = &latches[0];
        for preset_assumption in &partitioned.preset_assumptions {
            env_safe.push(initial.clone().implies(&translate(
                preset_assumption,
                false,
                manager,
                &uncontrollables,
                &uncontrollable_names,
                &controllables,
                &controllable_names,
                &latches,
            )));
        }
        for preset_guarantee in &partitioned.preset_guarantees {
            sys_safe.push(initial.clone().implies(&translate(
                preset_guarantee,
                false,
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
                    false,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                ));
            }
        }
        for invariant_guarantee in &partitioned.invariant_guarantees {
            if let HyperLTL::Appl(Op::Globally, inner) = &invariant_guarantee {
                sys_safe.push(translate(
                    &inner[0],
                    false,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                ));
            }
        }

        // (3.3) primed invariants
        for prime_invariant_assumption in &partitioned.prime_invariant_assumptions {
            if let HyperLTL::Appl(Op::Globally, inner) = &prime_invariant_assumption {
                env_safe.push((!initial).implies(&translate(
                    &inner[0],
                    false,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                )));
            }
        }
        for prime_invariant_guarantee in &partitioned.prime_invariant_guarantees {
            if let HyperLTL::Appl(Op::Globally, inner) = &prime_invariant_guarantee {
                sys_safe.push((!initial).implies(&translate(
                    &inner[0],
                    false,
                    manager,
                    &uncontrollables,
                    &uncontrollable_names,
                    &controllables,
                    &controllable_names,
                    &latches,
                )));
            }
        }

        let env_safe_err = env_safe
            .into_iter()
            .fold(manager.zero(), |val, safe| val.or(&!safe));
        let sys_safe_err = sys_safe
            .into_iter()
            .fold(manager.zero(), |val, safe| val.or(&!safe));

        // we have to remember if env_safe error happened
        let env_safe_err_happened = manager.new_var();
        latches.push(env_safe_err_happened.clone());
        latch_names.push(String::from("env_safe_err_happened"));
        compose.push(env_safe_err_happened.clone().or(&env_safe_err.clone()));
        initial_condition.push(manager.zero());

        let safety_condition = (!env_safe_err)
            .and(&!env_safe_err_happened)
            .implies(&!sys_safe_err);
        //assign o_err = ~env_safe_err & ~env_safe_err_happened & (sys_safe_err | fair_err);

        let initial_condition = initial_condition
            .into_iter()
            .fold(manager.one(), |val, init| val.and(&init));

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
