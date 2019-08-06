use super::HyperLTL::*;

impl std::fmt::Display for super::HyperLTL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Quant(kind, param, scope) => {
                let params: Vec<String> = param.into_iter().map(|p| format!("{}", p)).collect();
                write!(f, "{} {}: {}", kind, params.join(","), scope)
            }
            Appl(op, inner) => match inner.len() {
                0 => write!(f, "{}", op),
                1 => write!(f, "{}{}", op, inner.iter().next().unwrap()),
                _ => {
                    let operands: Vec<String> =
                        inner.iter().map(|ele| format!("{}", ele)).collect();
                    write!(f, "({})", operands.join(&format!(" {} ", op)))
                }
            },
            Prop(name, index) => match index {
                Some(index) => write!(f, "{}[{}]", name, index),
                None => write!(f, "{}", name),
            },
        }
    }
}

use super::Op::*;
impl std::fmt::Display for super::Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Negation => write!(f, "¬"),
            Next => write!(f, "○"),
            Finally => write!(f, "◇"),
            Globally => write!(f, "□"),
            Conjunction => write!(f, "∧"),
            Disjunction => write!(f, "∨"),
            Implication => write!(f, "→"),
            Exclusion => write!(f, "⊕"),
            Equivalence => write!(f, "↔"),
            Until => write!(f, "U"),
            Release => write!(f, "R"),
            WeakUntil => write!(f, "W"),
            True => write!(f, "⊤"),
            False => write!(f, "⊥"),
        }
    }
}

use super::QuantKind::*;
impl std::fmt::Display for super::QuantKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Exists => write!(f, "∃"),
            Forall => write!(f, "∀"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn print_binary() {
        let expr = Appl(
            Disjunction,
            vec![
                Appl(Negation, vec![Prop("a".into(), None)]),
                Prop("b".into(), None),
            ],
        );
        assert_eq!(format!("{}", expr), "(¬a ∨ b)")
    }

    #[test]
    fn print_unary() {
        let expr = Appl(Negation, vec![Appl(Globally, vec![Prop("a".into(), None)])]);
        assert_eq!(format!("{}", expr), "¬□a")
    }

    #[test]
    fn print_quantifier() {
        let expr = Quant(
            Forall,
            vec!["pi".into()],
            Box::new(Prop("a".into(), Some("pi".into()))),
        );
        assert_eq!(format!("{}", expr), "∀ pi: a[pi]")
    }
}
