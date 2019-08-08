use crate::logic::Logic;
use pathfinding::directed::strongly_connected_components::strongly_connected_components_from;
use std::collections::HashMap;

pub mod conversion;
mod dot;
pub mod reduction;

#[derive(Debug, Eq, Hash, Clone)]
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

#[derive(Debug, Clone)]
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
        let successors = |s: &&State<L>| {
            self.outgoing(s)
                .filter(|(_, guard)| !guard.is_false())
                .map(|(s, _)| s)
        };
        strongly_connected_components_from(&self.initial(), successors)
    }

    pub fn remove_rejecting_sinks(&mut self) {
        let rejecting_sink = match self
            .states()
            .iter()
            .filter(|s| {
                if !s.rejecting || s.initial {
                    return false;
                }
                let mut outgoing = self.outgoing(s);
                if let Some((succ, guard)) = outgoing.next() {
                    if outgoing.next().is_some() {
                        return false;
                    }
                    if succ != *s {
                        return false;
                    }
                    if !guard.is_true() {
                        return false;
                    }
                    true
                } else {
                    false
                }
            })
            .map(|s| s.id)
            .next()
        {
            Some(sink) => sink,
            None => return,
        };
        //println!("sink {}", rejecting_sink);

        // remove edges to rejecting state

        let transitions = &mut self.transitions;
        let states = &mut self.states;

        transitions.retain(|&source, outgoing| {
            if source == rejecting_sink {
                return false;
            }
            outgoing.retain(|&target, guard| {
                if target != rejecting_sink {
                    return true;
                }
                states[source].safety = Some(guard.negated());
                false
            });
            true
        });

        // remove rejecting sink from states
        states.remove(rejecting_sink);

        // rename states
        states.iter_mut().for_each(|state| {
            if state.id > rejecting_sink {
                state.id -= 1;
            }
        });

        let old = std::mem::replace(&mut self.transitions, HashMap::new());

        self.transitions = old
            .into_iter()
            .map(|(source, outgoing)| {
                let new_source = if source > rejecting_sink {
                    source - 1
                } else {
                    source
                };
                let new_outgoing = outgoing
                    .into_iter()
                    .map(|(target, guard)| {
                        let new_target = if target > rejecting_sink {
                            target - 1
                        } else {
                            target
                        };
                        (new_target, guard)
                    })
                    .collect();
                (new_source, new_outgoing)
            })
            .collect();
    }
}
