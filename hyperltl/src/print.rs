use super::HyperLTL::*;

impl std::fmt::Display for super::HyperLTL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Quant(kind, param, scope) => {
                let params: Vec<String> = param.into_iter().map(|p| format!("{}", p)).collect();
                write!(f, "{} {}: {}", kind, params.join(","), scope)
            }
            Unary(op, expr) => write!(f, "{}{}", op, expr),
            Binary(op, left, right) => write!(f, "({} {} {})", left, op, right),
            Proposition(name, index) => match index {
                Some(index) => write!(f, "{}[{}]", name, index),
                None => write!(f, "{}", name),
            },
            Literal(val) => write!(f, "{}", if *val { "1" } else { "0" }),
        }
    }
}

use super::BinOp::*;
impl std::fmt::Display for super::BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Conjunction => write!(f, "∧"),
            Disjunction => write!(f, "∨"),
            Implication => write!(f, "→"),
            Exclusion => write!(f, "⊕"),
            Equivalence => write!(f, "↔"),
            Until => write!(f, "U"),
            WeakUntil => write!(f, "W"),
            Release => write!(f, "R"),
        }
    }
}

use super::UnOp::*;
impl std::fmt::Display for super::UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Negation => write!(f, "¬"),
            Next => write!(f, "○"),
            Finally => write!(f, "◇"),
            Globally => write!(f, "□"),
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
        let expr = Binary(
            Disjunction,
            Box::new(Unary(Negation, Box::new(Proposition("a".into(), None)))),
            Box::new(Proposition("b".into(), None)),
        );
        assert_eq!(format!("{}", expr), "(¬a ∨ b)")
    }

    #[test]
    fn print_unary() {
        let expr = Unary(
            Negation,
            Box::new(Unary(Globally, Box::new(Proposition("a".into(), None)))),
        );
        assert_eq!(format!("{}", expr), "¬□a")
    }

    #[test]
    fn print_quantifier() {
        let expr = Quant(
            Forall,
            vec!["pi".into()],
            Box::new(Proposition("a".into(), Some("pi".into()))),
        );
        assert_eq!(format!("{}", expr), "∀ pi: a[pi]")
    }
}
