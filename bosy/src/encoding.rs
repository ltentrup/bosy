use crate::automata::conversion::LTL2Automaton;
use crate::specification::Specification;
use hyperltl::{HyperLTL, UnOp};
use smtlib::{Identifier, Instance, Sort};
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

        println!("{:?}", automaton);

        let mut constraints = Instance::new();

        // Representation of the transition system
        let states: Vec<Identifier> = (0..bound)
            .map(|i| constraints.new_ident(&format!("s_{}", i)))
            .collect();
        let state = constraints.declare_enum("S", &states);
        let tau = constraints.declare_fun("tau", &vec![state.clone()], state.clone());

        println!("{}", constraints);

        unimplemented!();
    }
}
