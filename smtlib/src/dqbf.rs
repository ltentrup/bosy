use super::*;

struct DQBFTransformer {
    instance: Instance,
}

impl DQBFTransformer {
    fn new(instance: Instance) -> DQBFTransformer {
        DQBFTransformer { instance }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn print_simple_script() {
        let mut instance = Instance::new();
        let x = instance.declare_fun("x", &[Sort::BOOL], Sort::BOOL);
        instance.assert(Term::new_quant(
            QuantKind::Forall,
            &[("a".to_string(), Sort::BOOL), ("b".to_string(), Sort::BOOL)],
            |idents| {
                Term::new_appl(
                    Identifier::AND,
                    vec![
                        Term::new_appl(x.clone(), vec![Term::new_ident(&idents[0])]),
                        Term::new_appl(x.clone(), vec![Term::new_ident(&idents[1])]),
                    ],
                )
            },
        ));
        let script = format!("{}", instance);
        assert_eq!(
            script,
            "(declare-fun x (Bool) Bool)\n(assert (forall ((a Bool) (b Bool)) (and (x a ) (x b ) )))\n"
        )
    }
}
