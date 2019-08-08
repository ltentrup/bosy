use super::{CoBuchiAutomaton, State, StateId};
use crate::logic::Logic;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

impl<L: Logic> CoBuchiAutomaton<L> {
    pub fn reduce_to_safety(self, bound: u32) -> Self {
        let sccs = self.sccs();

        let mut mapping: HashMap<(StateId, u32), StateId> = HashMap::new();
        let mut processed: HashSet<(StateId, u32)> = HashSet::new();

        let mut queue: VecDeque<(StateId, u32)> = self
            .states
            .iter()
            .filter(|s| s.initial)
            .map(|s| (s.id, 0))
            .collect();

        let mut states: Vec<State<L>> = Vec::new();
        let mut transitions: HashMap<StateId, HashMap<StateId, L>> = HashMap::new();

        fn insert<L: Logic>(
            states: &mut Vec<State<L>>,
            old: &Vec<State<L>>,
            transitions: &mut HashMap<StateId, HashMap<StateId, L>>,
            mapping: &mut HashMap<(StateId, u32), StateId>,
            state_id: StateId,
            k: u32,
        ) -> StateId {
            if let Some(new_id) = mapping.get(&(state_id, k)) {
                return *new_id;
            }
            let mut copy = old[state_id].clone();
            copy.name = Some(format!("{}_{}", copy.name.as_ref().unwrap(), k));
            copy.rejecting = false;
            let new_id = states.len();
            copy.id = new_id;
            states.push(copy);
            mapping.insert((state_id, k), new_id);
            let empty = transitions.insert(new_id, HashMap::new());
            assert!(empty.is_none());
            new_id
        }

        // add initial states
        for &(state_id, k) in &queue {
            insert(
                &mut states,
                &self.states,
                &mut transitions,
                &mut mapping,
                state_id,
                k,
            );
        }

        while let Some((state_id, k)) = queue.pop_front() {
            if processed.contains(&(state_id, k)) {
                continue;
            }

            // build transition
            for (&next_state_id, guard) in &self.transitions[&state_id] {
                let next_state = &self.states[next_state_id];
                let mut counter_reset: bool = true;
                // counter can be reset if the states are in different SCCs or the SCC does not contain a rejecting state
                for scc in &sccs {
                    if scc.contains(&&self.states[state_id])
                        && scc.contains(&next_state)
                        && scc.iter().any(|s| s.rejecting)
                    {
                        counter_reset = false;
                        break;
                    }
                }
                let next_counter = if counter_reset {
                    0
                } else if next_state.rejecting {
                    k + 1
                } else {
                    k
                };
                let new_state_id = mapping[&(state_id, k)];
                if next_counter > bound {
                    assert_eq!(next_counter, bound + 1);
                    // exceeded bound, transform into safety condition
                    if let Some(safety) = states[new_state_id].safety.as_mut() {
                        *safety = safety.and(guard.negated());
                    } else {
                        states[new_state_id].safety = Some(guard.negated());
                    }
                } else {
                    queue.push_back((next_state_id, next_counter));
                    let new_next_state_id = insert(
                        &mut states,
                        &self.states,
                        &mut transitions,
                        &mut mapping,
                        next_state_id,
                        next_counter,
                    );

                    let outgoing = transitions.entry(new_state_id).or_insert_with(HashMap::new);
                    outgoing.insert(new_next_state_id, guard.clone());
                }
            }

            processed.insert((state_id, k));
        }

        Self {
            manager: self.manager,
            states,
            transitions,
        }
    }
}
