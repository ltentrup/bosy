use super::{CoBuchiAutomaton, StateId};
use hyperltl::HyperLTL;
use pest::Parser;
use smtlib;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Parser)]
#[grammar = "automata/neverclaim.pest"]
struct NeverClaimParser;

pub enum LTL2Automaton {
    Spot,
}

thread_local! {static cache: RefCell<HashMap<HyperLTL, CoBuchiAutomaton<smtlib::Term>>> = RefCell::new(HashMap::new())}

impl LTL2Automaton {
    pub fn to_ucw(&self, spec: &HyperLTL) -> Result<CoBuchiAutomaton<smtlib::Term>, Box<Error>> {
        assert!(spec.is_quantifier_free());

        // check if automaton is in cache
        if let Some(res) = cache.with(|cache_cell| -> Option<CoBuchiAutomaton<smtlib::Term>> {
            cache_cell.borrow().get(spec).cloned()
        }) {
            return Ok(res);
        }

        // make thr current process the leader of a new process group
        // thus, the child cwill be terminated on timeout as well
        unsafe { libc::setsid() };

        let output = Command::new("ltl2tgba")
            .env("PATH", "../external/bin:./external/bin")
            .arg("-f")
            .arg(spec.to_spot())
            .arg("--spin")
            .arg("--low")
            .output()?;
        //println!("{:?}", output);
        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr)?;
            panic!("automaton construction failed:\n{}", stderr);
        }
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout)?;
        let mut automaton = CoBuchiAutomaton::from(&stdout, spec.get_occurrences().into_iter())?;
        automaton.remove_rejecting_sinks();

        // insert into cache
        cache.with(|cache_cell|  {
            cache_cell.borrow_mut().insert(spec.clone(), automaton.clone())
        });

        Ok(automaton)
    }
}

impl CoBuchiAutomaton<smtlib::Term> {
    fn from<'a>(neverclaim: &str, props: impl Iterator<Item = String>) -> Result<Self, Box<Error>> {
        let pairs = NeverClaimParser::parse(Rule::neverclaim, neverclaim)?;

        let mut instance = smtlib::Instance::new();
        for prop in props {
            instance.declare_const(&prop, smtlib::Sort::BOOL);
        }

        let mut automaton = CoBuchiAutomaton::<smtlib::Term>::new(instance);

        let mut translation: HashMap<&str, StateId> = HashMap::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::state => (),
                Rule::EOI => break,
                _ => unreachable!(),
            }
            let mut state_decl = pair.into_inner();
            let name = state_decl.next().expect("mismatch in neverclaim parser");
            assert!(name.as_rule() == Rule::identifier);
            let name = name.as_str();
            let initial = name.ends_with("_init");
            let rejecting = name.starts_with("accept_");

            let entry = translation
                .entry(name)
                .or_insert_with(|| automaton.new_state());
            let id = *entry;

            let state = automaton.get_state_mut(id);
            state.name = Some(name.to_string());
            state.initial = initial;
            state.rejecting = rejecting;

            while let Some(transition) = state_decl.next() {
                if transition.as_rule() == Rule::skip {
                    automaton.add_transition(id, id, smtlib::Term::TRUE);
                    assert!(transition.into_inner().next() == None);
                    break;
                }
                let mut transition = transition.into_inner();
                let expression = transition
                    .next()
                    .expect("expected expression in transition");
                let term = smtlib::parse::propositional::parse(
                    &mut automaton.manager,
                    expression.as_str(),
                )?;
                let target = transition.next().expect("expect traget node in transition");
                let target_id = *(translation
                    .entry(target.as_str())
                    .or_insert_with(|| automaton.new_state()));
                automaton.add_transition(id, target_id, term);
            }
        }

        Ok(automaton)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_never_claim() {
        let claim = "never { /* G!b M X(!a | !b) */
T0_init:
  if
  :: (true) -> goto T0_S1
  :: (!(b)) -> goto accept_S2
  fi;
T0_S1:
  if
  :: ((!(a)) || (!(b))) -> goto T0_S1
  :: (!(b)) -> goto accept_S2
  fi;
accept_S2:
  if
  :: (!(b)) -> goto accept_S2
  fi;
}";
        NeverClaimParser::parse(Rule::neverclaim, claim).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn convert_spot() -> Result<(), Box<Error>> {
        use hyperltl::Op::{Finally, Globally};
        let ltl = HyperLTL::Appl(
            Globally,
            vec![HyperLTL::Appl(
                Finally,
                vec![HyperLTL::Prop("a".into(), None)],
            )],
        );
        let converter = LTL2Automaton::Spot;
        let automaton = converter.to_ucw(&ltl)?;
        assert!(automaton.states.len() == 2);
        Ok(())
    }
}
