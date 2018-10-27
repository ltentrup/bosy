use crate::automata::conversion::LTL2Automaton;
use crate::specification::Specification;
use hyperltl::{HyperLTL, UnOp};
use smtlib::Instance;
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

        unimplemented!();
    }
}
