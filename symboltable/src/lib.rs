use std::collections::HashMap;

/// A symbol is a reference to an entry in SymbolTable
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Symbol(u32);

impl Symbol {
    pub fn new(name: u32) -> Symbol {
        Symbol(name)
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

/// A SymbolTable is a bi-directional mapping between strings and symbols
#[derive(Debug)]
pub struct SymbolTable {
    names: HashMap<Box<str>, Symbol>,
    strings: Vec<Box<str>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            names: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn get_symbol_for(&mut self, string: &str) -> Symbol {
        // check if already presents
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        // insert in symboltable
        let name = Symbol(self.strings.len() as u32);
        let copy = string.to_string().into_boxed_str();
        self.strings.push(copy.clone());
        self.names.insert(copy, name);

        name
    }

    pub fn get_string(&self, symbol: Symbol) -> &str {
        self.strings[symbol.to_usize()].as_ref()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn symbol_table() {
        let mut symboltable = SymbolTable::new();
        let sym_a = symboltable.get_symbol_for("a");
        let sym_b = symboltable.get_symbol_for("b");
        assert_ne!(sym_a, sym_b);
        assert_eq!(sym_a, symboltable.get_symbol_for("a"));
        assert_eq!(symboltable.get_string(sym_a), "a");
        assert_eq!(symboltable.get_string(sym_b), "b");
    }
}
