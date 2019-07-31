use crate::automata::conversion::LTL2Automaton;
use crate::automata::{CoBuchiAutomaton, State};
use crate::specification::Specification;
use hyperltl::{HyperLTL, Op};
use smtlib::{IdentKind, Identifier, Instance, QuantKind, Sort, Term, TermKind};
use std::collections::HashMap;
use std::process;

pub(crate) struct BoSyEncoding<'a> {
    specification: &'a Specification,
    instance: Instance,
}

impl<'a> BoSyEncoding<'a> {
    pub(crate) fn new(specification: &'a Specification) -> Self {
        BoSyEncoding {
            specification,
            instance: Instance::new(),
        }
    }

    pub(crate) fn solve(&mut self, bound: usize, bounds: &[usize], file_name: &str) {
        let linear = HyperLTL::Appl(Op::Negation, vec![self.specification.ltl()]);

        println!("build automaton");

        let automaton = match LTL2Automaton::Spot.to_ucw(linear) {
            Err(err) => {
                eprintln!("failed to convert LTL to automaton");
                eprintln!("{}", err);
                process::exit(1);
            }
            Ok(automaton) => automaton,
        };

        //println!("{:?}", automaton);

        println!("create encoding");

        let mut constraints = Instance::new();

        // Representation of the transition
        //let states = constraints.declare_sort("S", 0);
        let (states, state_values) = constraints.declare_enum(
            "S",
            &(0..bound)
                .into_iter()
                .map(|(i)| format!("s_{}", i))
                .collect::<Vec<String>>(),
        );
        //let initial = constraints.declare_const("s_0", &states);
        let tau = constraints.declare_fun(
            "tau",
            &vec![
                vec![&states],
                vec![Sort::BOOL; self.specification.inputs.len()],
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<&Sort>>(),
            &states,
        );
        let labels: Vec<Identifier> = self
            .specification
            .outputs
            .iter()
            .map(|o| {
                constraints.declare_fun(
                    o,
                    &vec![
                        vec![&states],
                        vec![Sort::BOOL; self.specification.inputs.len()],
                    ]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<&Sort>>(),
                    Sort::BOOL,
                )
            })
            .collect();

        // Representation of the automaton
        let (aut_state, aut_states) = constraints.declare_enum(
            "Q",
            &automaton
                .states()
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    s.name
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("q{}", i))
                })
                .collect::<Vec<String>>(),
        );

        // representation of run graph
        let lambda = constraints.declare_fun("lambda_", &vec![&states, &aut_state], Sort::BOOL);
        let lambda_sharp =
            constraints.declare_fun("lambda_sharp", &vec![&states, &aut_state], Sort::INT);

        for (state, ident) in automaton.states().iter().zip(&aut_states) {
            if state.initial {
                constraints.assert(Term::new_appl(
                    lambda.clone(),
                    vec![Term::new_ident(&state_values[0]), Term::new_ident(&ident)],
                ))
            }
            for (target, term) in automaton.outgoing(state) {
                constraints.assert(self.build_transitions(
                    &states,
                    &lambda,
                    &lambda_sharp,
                    target,
                    term,
                    &ident,
                    &aut_states,
                    &labels,
                    &tau,
                    None,
                    None,
                    None,
                    &vec![],
                    &vec![],
                ))
            }
        }

        if let Some(spec) = self.specification.hyper.as_ref() {
            self.encode_hyper(
                spec,
                &mut constraints,
                &states,
                &state_values,
                &labels,
                &tau,
                bound,
                bounds,
            );
        }
        use std::io::BufWriter;
        use std::io::Write;

        println!("write encoding to file");

        let mut file = std::fs::File::create(file_name).expect("file creation failed");
        let mut buf_writer = BufWriter::new(file);
        writeln!(
            buf_writer,
            "(set-option :smt.ematching false)\n(set-option :smt.mbqi true)\n{}\n(check-sat)\n",
            constraints
        );

        //println!("{}\n(check-sat)\n", constraints);

        //unimplemented!();
    }

    fn build_transitions(
        &self,
        states: &Sort,
        lambda: &Identifier,
        lambda_sharp: &Identifier,
        target: &State<Term>,
        term: &Term,
        ident: &Identifier,
        aut_states: &[Identifier],
        labels: &[Identifier],
        tau: &Identifier,
        path_vars: Option<&[&String]>,
        univ_path_vars: Option<&[&String]>,
        strat_states: Option<&[Sort]>,
        strat_labels: &[(Vec<Identifier>, String, usize)],
        strat_taus: &[(Identifier, usize)],
    ) -> Term {
        //println!("target {:?}", target);

        let mut forall_quant: Vec<(String, &Sort)> = Vec::new();
        if let Some(path_vars) = path_vars {
            let univ_path_vars = univ_path_vars.unwrap();

            //println!("paths {:?} {:?}", path_vars, univ_path_vars);

            // S and S'
            forall_quant.extend(path_vars.iter().map(|pvar| (format!("s_{}", pvar), states)));
            forall_quant.extend(
                path_vars
                    .iter()
                    .map(|pvar| (format!("s_p_{}", pvar), states)),
            );

            // strat and strat'
            let strat_states = strat_states.unwrap();
            forall_quant.extend(
                strat_states
                    .iter()
                    .enumerate()
                    .map(|(i, sort)| (format!("s_exists_{}", i), sort)),
            );
            forall_quant.extend(
                strat_states
                    .iter()
                    .enumerate()
                    .map(|(i, sort)| (format!("s_exists_p_{}", i), sort)),
            );

            for path_var in univ_path_vars {
                forall_quant.extend(
                    self.specification
                        .inputs
                        .iter()
                        .map(|i| (format!("{}_{}", i, path_var), Sort::BOOL)),
                );
            }
        } else {
            forall_quant.push(("s".to_string(), states));
            forall_quant.push(("s_p".to_string(), states));
            forall_quant.extend(
                self.specification
                    .inputs
                    .iter()
                    .map(|i| (i.clone(), Sort::BOOL)),
            );
        }

        let t = Term::new_quant(QuantKind::Forall, &forall_quant, |identifier| {
            // Extract identifier for S, S', and inputs from closure argument
            let mut in_out_map: HashMap<String, Term> = HashMap::new();
            // `s`, `s_prime`, and `ins` are vectors corresponding to path variables
            let s: Vec<Identifier>;
            let s_prime: Vec<Identifier>;
            let mut ins: Vec<Vec<Identifier>>;
            let ins_appl: Vec<Vec<Term>>; // inputs used in the transition function of tau
            let mut strat_s: Vec<Identifier> = Vec::new();
            let mut strat_s_prime: Vec<Identifier> = Vec::new();
            let mut strat_tau_equal: Term = Term::TRUE;

            let mut identifier = Vec::from(identifier);
            if let Some(path_vars) = path_vars {
                let univ_path_vars = univ_path_vars.unwrap();
                let strat_states = strat_states.unwrap();
                let mut other: Vec<Identifier>;
                // S
                other = identifier.split_off(path_vars.len());
                s = identifier;
                identifier = other;

                //  S'
                other = identifier.split_off(path_vars.len());
                s_prime = identifier;
                identifier = other;

                // S-exists
                other = identifier.split_off(strat_states.len());
                strat_s = identifier;
                identifier = other;

                // S-exists-prime
                other = identifier.split_off(strat_states.len());
                strat_s_prime = identifier;
                identifier = other;

                // inputs
                let global_ins = identifier.clone(); // stores all inputs in-order, used to apply to strat_tau and strat_label using slicing
                ins = Vec::new();
                assert!(identifier.len() >= self.specification.inputs.len());
                while identifier.len() > self.specification.inputs.len() {
                    other = identifier.split_off(self.specification.inputs.len());
                    ins.push(identifier);
                    identifier = other;
                }
                ins.push(identifier);
                assert_eq!(ins.len(), univ_path_vars.len());

                // used to build `ins_appl`
                let mut ins_appl_builder: HashMap<String, Vec<Term>> = HashMap::new();

                // universally controlled path variables
                for (path_var, inputs) in univ_path_vars.iter().zip(&ins) {
                    // build parameter
                    let path_ins_appl: Vec<Term> =
                        inputs.iter().map(|i| Term::new_ident(i)).collect();
                    ins_appl_builder.insert(path_var.to_string(), path_ins_appl);

                    // build replacer for transition function
                    for (i, ident) in self.specification.inputs.iter().zip(inputs) {
                        in_out_map.insert(
                            format!(
                                "{}",
                                HyperLTL::Prop(i.to_string(), Some(path_var.to_string()))
                            ),
                            Term::new_ident(&ident),
                        );
                    }
                }

                // existentially controlled path variables
                assert_eq!(strat_s.len(), strat_labels.len());
                for ((in_labels, path_var, slice_length), curr_strat_s) in
                    strat_labels.iter().zip(&strat_s)
                {
                    assert_eq!(in_labels.len(), self.specification.inputs.len());

                    let mut path_ins_appl: Vec<Term> = Vec::new();

                    for (i, in_label) in self.specification.inputs.iter().zip(in_labels) {
                        let mut in_label_args: Vec<Term> = vec![Term::new_ident(curr_strat_s)];
                        in_label_args.extend(
                            global_ins
                                .split_at(*slice_length)
                                .0
                                .iter()
                                .map(|ele| Term::new_ident(ele)),
                        );
                        in_out_map.insert(
                            format!("{}", HyperLTL::Prop(i.to_string(), Some(path_var.clone()))),
                            Term::new_appl(in_label.clone(), in_label_args.clone()),
                        );

                        path_ins_appl.push(Term::new_appl(in_label.clone(), in_label_args));
                    }

                    ins_appl_builder.insert(path_var.to_string(), path_ins_appl);
                }

                ins_appl = path_vars
                    .iter()
                    .map(|pvar| ins_appl_builder[*pvar].clone())
                    .collect();

                assert_eq!(path_vars.len(), ins_appl.len());
                assert_eq!(path_vars.len(), s.len());

                // define labels of transition system
                for ((path_var, inputs), s) in path_vars.iter().zip(&ins_appl).zip(&s) {
                    for (o, ident) in self.specification.outputs.iter().zip(labels) {
                        let mut label_appl: Vec<Term> = vec![Term::new_ident(&s)];
                        label_appl.extend(inputs.clone());
                        in_out_map.insert(
                            format!(
                                "{}",
                                HyperLTL::Prop(o.to_string(), Some(path_var.to_string()))
                            ),
                            Term::new_appl(ident.clone(), label_appl),
                        );
                    }
                }

                // build the transition equality for existential path strategy
                assert_eq!(strat_s.len(), strat_s_prime.len());
                assert_eq!(strat_s.len(), strat_taus.len());
                for ((current_strat_s, current_strat_s_p), (tau, slice_length)) in
                    strat_s.iter().zip(&strat_s_prime).zip(strat_taus)
                {
                    let mut tau_args: Vec<Term> = vec![Term::new_ident(current_strat_s)];
                    tau_args.extend(
                        global_ins
                            .split_at(*slice_length)
                            .0
                            .iter()
                            .map(|ele| Term::new_ident(ele)),
                    );
                    let tau_next = Term::new_appl(tau.clone(), tau_args);
                    let tau_next_equal = Term::new_appl(
                        Identifier::EQ,
                        vec![tau_next, Term::new_ident(current_strat_s_p)],
                    );
                    strat_tau_equal = strat_tau_equal & tau_next_equal;
                }
            } else {
                let mut other: Vec<Identifier>;
                // S
                other = identifier.split_off(1);
                s = identifier;
                identifier = other;

                // S'
                other = identifier.split_off(1);
                s_prime = identifier;
                identifier = other;

                // inputs
                ins_appl = vec![identifier.iter().map(|i| Term::new_ident(i)).collect()];
                ins = vec![identifier];

                for (o, ident) in self.specification.outputs.iter().zip(labels) {
                    let mut label_appl: Vec<Term> = vec![Term::new_ident(&s[0])];
                    label_appl.extend(ins[0].iter().map(|i| Term::new_ident(&i)));
                    in_out_map.insert(o.to_string(), Term::new_appl(ident.clone(), label_appl));
                }
                for (i, ident) in self.specification.inputs.iter().zip(&ins[0]) {
                    in_out_map.insert(i.to_string(), Term::new_ident(&ident));
                }
            }

            let transformed = term.convert(&|t| match &t.kind {
                TermKind::Ident(i) => match &i.kind {
                    IdentKind::Custom(decl) => {
                        if let Some(term) = in_out_map.get(decl.name()) {
                            Some(term.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            });
            //println!("{}", transformed);

            assert_eq!(s.len(), s_prime.len());
            assert_eq!(s.len(), ins_appl.len());

            let mut tau_next_constraint = Term::TRUE;
            for ((current_s, current_s_p), inputs) in s.iter().zip(&s_prime).zip(&ins_appl) {
                assert_eq!(inputs.len(), self.specification.inputs.len());
                let mut tau_appl: Vec<Term> = vec![Term::new_ident(current_s)];
                tau_appl.extend(inputs.iter().map(|i| i.clone()));
                let tau_next = Term::new_appl(tau.clone(), tau_appl);
                let tau_next_equal =
                    Term::new_appl(Identifier::EQ, vec![tau_next, Term::new_ident(current_s_p)]);
                tau_next_constraint = tau_next_constraint & tau_next_equal;
            }

            let mut lambda_appl: Vec<Term> = s.iter().map(|s| Term::new_ident(&s)).collect();
            lambda_appl.push(Term::new_ident(ident));
            lambda_appl.extend(strat_s.iter().map(|s| Term::new_ident(&s)));
            let lambda_current = Term::new_appl(lambda.clone(), lambda_appl);

            Term::new_appl(
                Identifier::IMPL,
                vec![
                    lambda_current & transformed & tau_next_constraint & strat_tau_equal,
                    self.next_state(
                        lambda,
                        lambda_sharp,
                        &s,
                        &s_prime,
                        &strat_s,
                        &strat_s_prime,
                        ident,
                        &aut_states[target.id],
                        target.rejecting,
                    ),
                ],
            )
        });
        t
    }

    fn next_state(
        &self,
        lambda: &Identifier,
        lambda_sharp: &Identifier,
        s: &[Identifier],
        s_prime: &[Identifier],
        strat_s: &[Identifier],
        strat_s_prime: &[Identifier],
        source: &Identifier,
        target: &Identifier,
        rejecting: bool,
    ) -> Term {
        let mut lambda_appl: Vec<Term> = s_prime.iter().map(|s| Term::new_ident(&s)).collect();
        lambda_appl.push(Term::new_ident(target));
        lambda_appl.extend(strat_s_prime.iter().map(|s| Term::new_ident(&s)));

        let lambda_next_appl = lambda_appl.clone();

        let l = Term::new_appl(lambda.clone(), lambda_appl);
        let fun = if rejecting {
            Identifier::LT
        } else {
            Identifier::LE
        };

        let mut lambda_curr_appl: Vec<Term> = s.iter().map(|s| Term::new_ident(&s)).collect();
        lambda_curr_appl.push(Term::new_ident(source));
        lambda_curr_appl.extend(strat_s.iter().map(|s| Term::new_ident(&s)));

        let greater = Term::new_appl(
            fun,
            vec![
                Term::new_appl(lambda_sharp.clone(), lambda_curr_appl),
                Term::new_appl(lambda_sharp.clone(), lambda_next_appl),
            ],
        );
        l & greater
    }

    fn encode_hyper(
        &self,
        spec: &[HyperLTL],
        constraints: &mut Instance,
        states: &Sort,
        state_values: &[Identifier],
        labels: &[Identifier],
        tau: &Identifier,
        bound: usize,
        bounds: &[usize],
    ) {
        assert_eq!(
            spec.len(),
            bounds.len(),
            "The bounds have to match the number of HyperLTL specifications"
        );
        for (i, hyper) in spec.iter().enumerate() {
            //println!("{}", hyper);
            //println!("{}", hyper.get_body());

            let negated_hyper = HyperLTL::Appl(Op::Negation, vec![hyper.get_body().clone()]);

            let automaton = match LTL2Automaton::Spot.to_ucw(negated_hyper) {
                Err(err) => {
                    eprintln!("failed to convert LTL to automaton");
                    eprintln!("{}", err);
                    process::exit(1);
                }
                Ok(automaton) => automaton,
            };

            // Representation of the automaton
            let (aut_state, aut_states) = constraints.declare_enum(
                &format!("Q_{}", i),
                &automaton
                    .states()
                    .iter()
                    .enumerate()
                    .map(|(j, s)| {
                        s.name
                            .as_ref()
                            .map(|s| format!("q_{}_{}", i, s))
                            .unwrap_or_else(|| format!("q_{}_j", j))
                    })
                    .collect::<Vec<String>>(),
            );

            let quantifier = hyper.get_quantifier();
            let path_vars: Vec<&String> = quantifier
                .iter()
                .map(|(_, params)| params)
                .flatten()
                .collect();
            let num_quant = path_vars.len();
            /*println!(
                "num-quant {},  paths-vars {:?}, quant {:?}",
                num_quant, path_vars, quantifier
            );*/

            // build the strategy for existential path quantifier
            let mut universal_pvars: Vec<&String> = Vec::new(); // stores the dependent path quantifier, i.e., earlier bound
            let mut strat_sorts: Vec<Sort> = Vec::new(); // stores the states of the strategies
            let mut strat_initial: Vec<Identifier> = Vec::new(); // stores the initial states
            let mut strat_labels: Vec<(Vec<Identifier>, String, usize)> = Vec::new(); // stores the labels representing inputs, second parameter is path_variable, third parameter number of univ path vars * inputs
            let mut strat_taus: Vec<(Identifier, usize)> = Vec::new(); // stores the transition functions, second parameter number of univ path vars * inputs
            for (j, (kind, param)) in quantifier.iter().enumerate() {
                match kind {
                    hyperltl::QuantKind::Forall => universal_pvars.extend(param),
                    hyperltl::QuantKind::Exists => {
                        let (strat_sort, strat_states) = constraints.declare_enum(
                            &format!("S_{}_{}", i, j),
                            &(0..bounds[i])
                                .into_iter()
                                .map(|k| format!("s_{}_{}_{}", i, j, k))
                                .collect::<Vec<String>>(),
                        );
                        let tau = constraints.declare_fun(
                            &format!("tau_{}_{}", i, j),
                            &vec![
                                vec![&strat_sort],
                                vec![
                                    Sort::BOOL;
                                    self.specification.inputs.len() * universal_pvars.len()
                                ],
                            ]
                            .into_iter()
                            .flatten()
                            .collect::<Vec<&Sort>>(),
                            &strat_sort,
                        );
                        strat_taus
                            .push((tau, self.specification.inputs.len() * universal_pvars.len()));
                        for pvar in param {
                            let labels: Vec<Identifier> = self
                                .specification
                                .inputs
                                .iter()
                                .map(|inp| {
                                    constraints.declare_fun(
                                        &format!("out_{}_{}_{}_{}_", i, j, inp, pvar),
                                        &vec![
                                            vec![&strat_sort],
                                            vec![
                                                Sort::BOOL;
                                                self.specification.inputs.len()
                                                    * universal_pvars.len()
                                            ],
                                        ]
                                        .into_iter()
                                        .flatten()
                                        .collect::<Vec<&Sort>>(),
                                        Sort::BOOL,
                                    )
                                })
                                .collect();
                            strat_labels.push((
                                labels,
                                pvar.clone(),
                                self.specification.inputs.len() * universal_pvars.len(),
                            ));
                        }
                        strat_sorts.push(strat_sort);
                        strat_initial.push(strat_states[0].clone());
                    }
                }
            }

            let mut lambda_args: Vec<&Sort> = vec![&states; num_quant];
            lambda_args.push(&aut_state);
            lambda_args.extend(&strat_sorts);

            let lambda =
                constraints.declare_fun(&format!("lambda_{}", i), &lambda_args, Sort::BOOL);
            let lambda_sharp =
                constraints.declare_fun(&format!("lambda_{}_sharp", i), &lambda_args, Sort::INT);

            for (state, ident) in automaton.states().iter().zip(&aut_states) {
                if state.initial {
                    let mut lambda_init_args: Vec<Term> =
                        vec![Term::new_ident(&state_values[0]); num_quant];
                    lambda_init_args.push(Term::new_ident(&ident));
                    lambda_init_args.extend(strat_initial.iter().map(|s| Term::new_ident(s)));
                    constraints.assert(Term::new_appl(lambda.clone(), lambda_init_args))
                }
                for (target, term) in automaton.outgoing(state) {
                    constraints.assert(self.build_transitions(
                        &states,
                        &lambda,
                        &lambda_sharp,
                        target,
                        term,
                        &ident,
                        &aut_states,
                        &labels,
                        &tau,
                        Some(&path_vars),
                        Some(&universal_pvars),
                        Some(&strat_sorts),
                        &strat_labels,
                        &strat_taus,
                    ))
                }
            }
        }
    }
}
