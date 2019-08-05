use bosy::specification::Semantics;
use cudd::{CuddManager, CuddNode};

#[derive(Debug)]
pub struct SafetyGame {
    pub(crate) manager: CuddManager,
    pub(crate) controllables: Vec<CuddNode>,
    pub(crate) uncontrollables: Vec<CuddNode>,
    pub(crate) latches: Vec<CuddNode>,
    pub(crate) compose: Vec<CuddNode>,
    pub(crate) initial: CuddNode,
    pub(crate) safety_condition: CuddNode,

    pub(crate) controllable_names: Vec<String>,
    pub(crate) uncontrollable_names: Vec<String>,
    pub(crate) latch_names: Vec<String>,
}

#[derive(Debug)]
pub struct SafetyGameSolver {
    instance: SafetyGame,
    semantics: Semantics,

    exiscube: CuddNode,
    univcube: CuddNode,
}

impl SafetyGameSolver {
    pub fn new(instance: SafetyGame, semantics: Semantics) -> Self {
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

    fn pre_system(&mut self, states: CuddNode) -> CuddNode {
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

    pub fn solve(&mut self) -> Option<CuddNode> {
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
