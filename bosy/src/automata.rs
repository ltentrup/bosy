use crate::logic::Logic;
use pathfinding::directed::strongly_connected_components::strongly_connected_components_from;
use std::collections::HashMap;

pub mod conversion;
mod dot;

#[derive(Debug, Eq, Hash)]
pub struct State<L: Logic> {
    pub id: StateId,
    pub name: Option<String>,
    pub initial: bool,
    pub rejecting: bool,
    pub safety: Option<L>,
}

impl<L: Logic> State<L> {
    fn new(id: StateId) -> Self {
        State {
            id,
            name: None,
            initial: false,
            rejecting: false,
            safety: None,
        }
    }
}

impl<L: Logic> std::cmp::PartialEq for State<L> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

type StateId = usize;

#[derive(Debug)]
pub struct CoBuchiAutomaton<L: Logic> {
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
        self.states.push(State::new(id));
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

    pub fn states(&self) -> &[State<L>] {
        &self.states
    }

    pub fn outgoing(&self, state: &State<L>) -> impl Iterator<Item = (&State<L>, &L)> {
        self.transitions[&state.id]
            .iter()
            .map(move |(&k, v)| (&self.states[k], v))
    }

    fn initial(&self) -> &State<L> {
        self.states.iter().find(|s| s.initial).unwrap()
    }

    pub fn sccs(&self) -> Vec<Vec<&State<L>>> {
        let successors = |s: &&State<L>| self.outgoing(s).filter(|(_,guard)| {
            !guard.is_false()
        }).map(|(s,_)| s);
        strongly_connected_components_from(&self.initial(), successors)
    }
}
