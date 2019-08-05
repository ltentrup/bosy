use bosy::specification::Semantics;
use cudd::{CuddManager, CuddNode};

#[derive(Debug)]
pub struct SafetyGame<'a> {
    pub(crate) manager: &'a CuddManager,
    pub(crate) controllables: Vec<CuddNode<'a>>,
    pub(crate) uncontrollables: Vec<CuddNode<'a>>,
    pub(crate) latches: Vec<CuddNode<'a>>,
    pub(crate) compose: Vec<CuddNode<'a>>,
    pub(crate) initial: CuddNode<'a>,
    pub(crate) safety_condition: CuddNode<'a>,

    pub(crate) controllable_names: Vec<String>,
    pub(crate) uncontrollable_names: Vec<String>,
    pub(crate) latch_names: Vec<String>,
}

#[derive(Debug)]
pub struct SafetyGameSolver<'a> {
    instance: SafetyGame<'a>,
    semantics: Semantics,

    exiscube: CuddNode<'a>,
    univcube: CuddNode<'a>,
}

impl<'a> SafetyGameSolver<'a> {
    pub fn new(instance: SafetyGame<'a>, semantics: Semantics) -> Self {
        let exiscube = instance
            .controllables
            .iter()
            .fold(instance.manager.one(), |f, node| f.and(node));
        let univcube = instance
            .uncontrollables
            .iter()
            .fold(instance.manager.one(), |f, node| f.and(node));
        Self {
            instance,
            semantics,
            exiscube,
            univcube,
        }
    }

    fn pre_system(&mut self, states: CuddNode<'a>) -> CuddNode<'a> {
        match self.semantics {
            Semantics::Mealy => states
                .vector_compose(&self.instance.compose)
                .and_abstract(&self.instance.safety_condition, &self.exiscube)
                .univ_abstract(&self.univcube),
            Semantics::Moore => states
                .vector_compose(&self.instance.compose)
                .and(&self.instance.safety_condition)
                .univ_abstract(&self.univcube)
                .exist_abstract(&self.exiscube),
        }
    }

    pub fn solve(&mut self) -> Option<CuddNode<'a>> {
        let mut fixpoint = self.instance.manager.zero();
        let mut safe_states = self.instance.manager.one();

        let mut rounds = 0;
        while safe_states != fixpoint {
            rounds += 1;
            println!("round {}", rounds);

            fixpoint = safe_states.clone();
            safe_states.and_assign(&self.pre_system(safe_states.clone()));
            if !self.instance.initial.leq(&safe_states) {
                // unrealizable
                return None;
            }
        }

        Some(fixpoint)
    }
}
