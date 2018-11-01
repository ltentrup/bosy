use crate::automata::conversion::LTL2Automaton;
use crate::automata::{CoBuchiAutomaton, State};
use crate::specification::Specification;
use hyperltl::{HyperLTL, UnOp};
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

    pub(crate) fn solve(&mut self, bound: usize) {
        let linear = HyperLTL::Unary(UnOp::Negation, Box::new(self.specification.ltl()));

        let converter = LTL2Automaton::Spot;
        let automaton = match converter.to_ucw(linear) {
            Err(err) => {
                eprintln!("failed to convert LTL to automaton");
                eprintln!("{}", err);
                process::exit(1);
            }
            Ok(automaton) => automaton,
        };

        //println!("{:?}", automaton);

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
            ].into_iter()
            .flatten()
            .collect::<Vec<&Sort>>(),
            &states,
        );
        let labels: Vec<Identifier> = self
            .specification
            .outputs
            .iter()
            .map(|o| constraints.declare_fun(o, &vec![&states], Sort::BOOL))
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
                }).collect::<Vec<String>>(),
        );

        // representation of run graph
        let lambda = constraints.declare_fun("lambda", &vec![&states, &aut_state], Sort::BOOL);
        let lambda_sharp =
            constraints.declare_fun("lambda_sharp", &vec![&states, &aut_state], Sort::INT);

        for (state, ident) in automaton.states().iter().zip(&aut_states) {
            if state.initial {
                constraints.assert(Term::new_appl(
                    lambda.clone(),
                    vec![
                        Box::new(Term::new_ident(&state_values[0])),
                        Box::new(Term::new_ident(&ident)),
                    ],
                ))
            }
            for (target, term) in automaton.outgoing(state) {
                constraints.assert(self.build_transitions(
                    &automaton,
                    &states,
                    &lambda,
                    &lambda_sharp,
                    state,
                    target,
                    term,
                    &ident,
                    &aut_states,
                    &labels,
                    &tau,
                ))
            }
        }

        println!("{}\n(check-sat)\n", constraints);

        unimplemented!();
    }

    fn build_transitions(
        &self,
        automaton: &CoBuchiAutomaton<Term>,
        states: &Sort,
        lambda: &Identifier,
        lambda_sharp: &Identifier,
        state: &State<Term>,
        target: &State<Term>,
        term: &Term,
        ident: &Identifier,
        aut_states: &[Identifier],
        labels: &[Identifier],
        tau: &Identifier,
    ) -> Term {
        println!("{:?} {}", target, term);

        let t = Term::new_quant(
            QuantKind::Forall,
            &vec![
                vec![("s", states)],
                vec![("s_p", states)],
                self.specification
                    .inputs
                    .iter()
                    .map(|i| (i.as_ref(), Sort::BOOL))
                    .collect::<Vec<(&str, &Sort)>>(),
            ].into_iter()
            .flatten()
            .collect::<Vec<(&str, &Sort)>>(),
            |identifier| {
                let (s, s_prime_ins) = identifier.split_first().unwrap();
                let (s_prime, ins) = s_prime_ins.split_first().unwrap();

                let mut in_out_map: HashMap<&str, Term> = HashMap::new();
                for (o, ident) in self.specification.outputs.iter().zip(labels) {
                    in_out_map.insert(
                        o,
                        Term::new_appl(ident.clone(), vec![Box::new(Term::new_ident(&s))]),
                    );
                }
                for (i, ident) in self.specification.inputs.iter().zip(ins) {
                    in_out_map.insert(i, Term::new_ident(&ident));
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

                let tau_next = Term::new_appl(
                    tau.clone(),
                    vec![vec![s.clone()], Vec::from(ins)]
                        .into_iter()
                        .flatten()
                        .map(|e| Box::new(Term::new_ident(&e)))
                        .collect::<Vec<Box<Term>>>(),
                );
                let tau_next_equal = Term::new_appl(
                    Identifier::EQ,
                    vec![Box::new(tau_next), Box::new(Term::new_ident(s_prime))],
                );

                Term::new_appl(
                    Identifier::IMPL,
                    vec![
                        Term::new_appl(
                            lambda.clone(),
                            vec![
                                Box::new(Term::new_ident(s)),
                                Box::new(Term::new_ident(ident)),
                            ],
                        ).into(),
                        Term::new_appl(
                            Identifier::IMPL,
                            vec![
                                Box::new(transformed & tau_next_equal),
                                Box::new(self.next_state(
                                    lambda,
                                    lambda_sharp,
                                    s,
                                    &s_prime,
                                    ident,
                                    &aut_states[target.id],
                                    target.rejecting,
                                )),
                            ],
                        ).into(),
                    ],
                )
            },
        );
        t
    }

    fn next_state(
        &self,
        lambda: &Identifier,
        lambda_sharp: &Identifier,
        s: &Identifier,
        s_prime: &Identifier,
        source: &Identifier,
        target: &Identifier,
        rejecting: bool,
    ) -> Term {
        let l = Term::new_appl(
            lambda.clone(),
            vec![
                Box::new(Term::new_ident(s_prime)),
                Box::new(Term::new_ident(target)),
            ],
        );
        let fun = if rejecting {
            Identifier::LT
        } else {
            Identifier::LE
        };
        let greater = Term::new_appl(
            fun,
            vec![
                Box::new(Term::new_appl(
                    lambda_sharp.clone(),
                    vec![
                        Box::new(Term::new_ident(s)),
                        Box::new(Term::new_ident(source)),
                    ],
                )),
                Box::new(Term::new_appl(
                    lambda_sharp.clone(),
                    vec![
                        Box::new(Term::new_ident(s_prime)),
                        Box::new(Term::new_ident(target)),
                    ],
                )),
            ],
        );
        l & greater
    }
}
