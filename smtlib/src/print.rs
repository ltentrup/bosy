use super::*;
use std::fmt::{Display, Formatter, Result};

impl Display for Instance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for sort in &self.sorts {
            write!(f, "{}\n", sort)?;
        }
        for decl in &self.declarations {
            write!(f, "{}\n", decl)?;
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
                    write!(f, "(declare-const {} {})", name, ret)
                } else {
                    let formatted: Vec<String> =
                        params.into_iter().map(|sort| format!("{}", sort)).collect();
                    write!(
                        f,
                        "(declare-fun {} ({}) {})",
                        name,
                        formatted.join(" "),
                        ret
                    )
                }
            }
        }
    }
}

impl IdentDecl {
    fn name(&self) -> &str {
        match &self {
            IdentDecl::Func(name, _, _) => name,
        }
    }
}

impl Display for SortDecl {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            SortDecl::Sort(name, arity) => write!(f, "(declare-sort {} {})", name, arity),
            /*SortDecl::Enum(name, values) => {
                let formatted: Vec<String> = values
                    .into_iter()
                    .map(|ident| format!("({})", ident))
                    .collect();
                format!("(declare-datatype {} ( {} ))", name, formatted.join(" "),)
            }*/
            _ => unimplemented!(),
        }
    }
}

impl SortDecl {
    fn name(&self) -> &str {
        match &self {
            SortDecl::Sort(name, _) => name,
        }
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.kind {
            SortKind::Bool => write!(f, "Bool"),
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
                let formatted: Vec<String> =
                    terms.into_iter().map(|sort| format!("{}", sort)).collect();
                write!(f, "({} {})", ident, formatted.join(" "))
            }
            _ => unimplemented!(),
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

impl Display for BoolFun {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BoolFun::And => write!(f, "and"),
            _ => unimplemented!(),
        }
    }
}
