use crate::logic::Logic;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

pub(crate) mod conversion;

#[derive(Debug)]
pub(crate) struct State<L: Logic> {
    name: Option<String>,
    initial: bool,
    rejecting: bool,
    safety: Option<L>,
}

impl<L: Logic> State<L> {
    fn new() -> Self {
        State {
            name: None,
            initial: false,
            rejecting: false,
            safety: None,
        }
    }
}

type StateId = usize;

#[derive(Debug)]
pub(crate) struct CoBuchiAutomaton<L: Logic> {
    manager: L::Manager,
    states: Vec<State<L>>,
    transitions: HashMap<StateId, HashMap<StateId, L>>,
}

impl<L: Logic> CoBuchiAutomaton<L> {
    fn new(manager: L::Manager) -> Self {
        CoBuchiAutomaton {
            manager,
            states: Vec::new(),
            transitions: HashMap::new(),
        }
    }

    fn new_state(&mut self) -> StateId {
        let id = self.states.len();
        self.states.push(State::new());
        id
    }

    fn get_state(&self, id: StateId) -> &State<L> {
        &self.states[id]
    }

    fn get_state_mut(&mut self, id: StateId) -> &mut State<L> {
        &mut self.states[id]
    }

    fn add_transition(&mut self, source: StateId, target: StateId, guard: L) {
        let outgoing = self
            .transitions
            .entry(source)
            .or_insert_with(|| HashMap::new());
        outgoing.insert(target, guard);
    }
}
