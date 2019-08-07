use super::*;
use std::fmt::{Display, Formatter, Result};

impl Display for Instance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for sort in &self.sorts {
            write!(f, "{}", sort)?;
        }
        for decl in &self.declarations {
            write!(f, "{}", decl)?;
        }
        for assertion in &self.assertions {
            write!(f, "(assert {})\n", assertion)?;
        }
        Ok(())
    }
}

impl Display for IdentDecl {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            IdentDecl::Func(name, params, ret) => {
                if params.is_empty() {
                    write!(f, "(declare-const {} {})\n", name, ret)
                } else {
                    let formatted: Vec<String> =
                        params.into_iter().map(|sort| format!("{}", sort)).collect();
                    write!(
                        f,
                        "(declare-fun {} ({}) {})\n",
                        name,
                        formatted.join(" "),
                        ret
                    )
                }
            }
            // enum cases are not explicitly declared
            IdentDecl::Case(name, sort) => Ok(()),
        }
    }
}

impl IdentDecl {
    pub fn name(&self) -> &str {
        match &self {
            IdentDecl::Func(name, _, _) => name,
            IdentDecl::Case(name, _) => name,
        }
    }
}

impl Display for SortDecl {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            SortDecl::Sort(name, arity) => write!(f, "(declare-sort {} {})\n", name, arity),
            SortDecl::Enum(name, values) => {
                let formatted: Vec<String> =
                    values.into_iter().map(|n| format!("({})", n)).collect();
                write!(
                    f,
                    "(declare-datatype {} ( {} ))\n",
                    name,
                    formatted.join(" "),
                )
            }
        }
    }
}

impl SortDecl {
    fn name(&self) -> &str {
        match &self {
            SortDecl::Sort(name, _) => name,
            SortDecl::Enum(name, _) => name,
        }
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.kind {
            SortKind::Bool => write!(f, "Bool"),
            SortKind::Int => write!(f, "Int"),
            SortKind::Custom(decl) => write!(f, "{}", decl.name()),
            _ => unimplemented!(),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.kind {
            TermKind::Ident(ident) => write!(f, "{}", ident),
            TermKind::Appl(ident, terms) => {
                write!(f, "({} ", ident)?;
                for term in terms {
                    write!(f, "{} ", term)?;
                }
                write!(f, ")")
            }
            TermKind::Quant(kind, binding, inner) => {
                let formatted: Vec<String> = binding
                    .into_iter()
                    .map(|ident| format!("({} {})", ident, ident.sort()))
                    .collect();
                write!(f, "({} ({}) {})", kind, formatted.join(" "), inner)
            }
            _ => unimplemented!(),
        }
    }
}

impl Display for QuantKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            QuantKind::Forall => write!(f, "forall"),
            QuantKind::Exists => write!(f, "exists"),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.kind {
            IdentKind::BooleanFun(fun) => write!(f, "{}", fun),
            IdentKind::Custom(decl) => write!(f, "{}", decl.name()),
            _ => unimplemented!(),
        }
    }
}

impl Identifier {
    fn sort(&self) -> &Sort {
        match &self.kind {
            IdentKind::Custom(decl) => decl.sort(),
            _ => unreachable!(),
        }
    }
}

impl Display for BoolFun {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BoolFun::And => write!(f, "and"),
            BoolFun::Or => write!(f, "or"),
            BoolFun::Not => write!(f, "not"),
            BoolFun::Impl => write!(f, "=>"),
            BoolFun::False => write!(f, "false"),
            BoolFun::True => write!(f, "true"),
            BoolFun::Lt => write!(f, "<"),
            BoolFun::Le => write!(f, "<="),
            BoolFun::Eq => write!(f, "="),
            _ => unimplemented!(),
        }
    }
}
