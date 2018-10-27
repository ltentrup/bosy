use super::*;

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for decl in &self.declarations {
            write!(f, "{}\n", decl.to_string(&self.symboltable))?;
        }
        for assertion in &self.assertions {
            write!(f, "(assert {})\n", assertion.to_string(&self.symboltable))?;
        }
        Ok(())
    }
}

impl Declaration {
    fn to_string(&self, s: &SymbolTable) -> String {
        match &self.kind {
            DeclKind::Func(name, params, ret) => {
                if params.is_empty() {
                    format!(
                        "(declare-const {} {})",
                        s.get_string(*name),
                        ret.to_string(s)
                    )
                } else {
                    let formatted: Vec<String> =
                        params.into_iter().map(|sort| sort.to_string(s)).collect();
                    format!(
                        "(declare-fun {} ({}) {})",
                        s.get_string(*name),
                        formatted.join(" "),
                        ret.to_string(s)
                    )
                }
            }
            DeclKind::Sort(_, _) => unimplemented!(),
        }
    }
}

impl Sort {
    fn to_string(&self, s: &SymbolTable) -> String {
        match &self.kind {
            SortKind::Bool => "Bool".into(),
            _ => unimplemented!(),
        }
    }
}

impl Term {
    fn to_string(&self, s: &SymbolTable) -> String {
        match &self.kind {
            TermKind::Ident(ident) => ident.to_string(s),
            TermKind::Appl(ident, terms) => {
                let formatted: Vec<String> =
                    terms.into_iter().map(|sort| sort.to_string(s)).collect();
                format!("({} {})", ident.to_string(s), formatted.join(" "))
            }
            _ => unimplemented!(),
        }
    }
}

impl Identifier {
    fn to_string(&self, s: &SymbolTable) -> String {
        match &self.kind {
            IdentKind::Simple(symbol) => s.get_string(*symbol).into(),
            IdentKind::BooleanFun(fun) => format!("{}", fun),
            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Display for BoolFun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BoolFun::And => write!(f, "and"),
            _ => unimplemented!(),
        }
    }
}
