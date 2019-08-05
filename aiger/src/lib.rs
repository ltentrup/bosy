#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include bindings g(* right_val).try_into().unwrap() `rust-bindgen`
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::convert::TryInto;
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub struct Aiger {
    aiger: *mut aiger,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct AigerLit(u32);

#[derive(Debug)]
pub struct AigerSymbol<'a> {
    symbol: &'a aiger_symbol,
}

#[derive(Debug)]
pub struct AigerAnd<'a> {
    and: &'a aiger_and,
}

impl Aiger {
    pub fn new() -> Aiger {
        let aiger = unsafe { aiger_init() };
        assert!(!aiger.is_null());
        Aiger { aiger }
    }

    pub fn from_str(content: &str) -> Result<Self, String> {
        let aiger = Self::new();

        let c_str = CString::new(content).expect("CString::new failed");
        let ptr = c_str.as_ptr();

        let result = unsafe { aiger_read_from_string(aiger.aiger, ptr) };
        if !result.is_null() {
            return Err(aiger.error());
        }

        Ok(aiger)
    }

    pub fn error(&self) -> String {
        let err = unsafe { aiger_error(self.aiger) };
        unsafe { CStr::from_ptr(err) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn is_reencoded(&self) -> bool {
        unsafe { aiger_is_reencoded(self.aiger) > 0 }
    }

    pub fn reencode(&mut self) {
        unsafe { aiger_reencode(self.aiger) }
    }

    pub fn inputs(&self) -> impl Iterator<Item = AigerSymbol> {
        let inputs = unsafe {
            std::slice::from_raw_parts(
                (*self.aiger).inputs,
                (*self.aiger).num_inputs.try_into().unwrap(),
            )
        };
        inputs.iter().map(|symbol| AigerSymbol { symbol })
    }

    pub fn latches(&self) -> impl Iterator<Item = AigerSymbol> {
        let latches = unsafe {
            std::slice::from_raw_parts(
                (*self.aiger).latches,
                (*self.aiger).num_latches.try_into().unwrap(),
            )
        };
        latches.iter().map(|symbol| AigerSymbol { symbol })
    }

    pub fn outputs(&self) -> impl Iterator<Item = AigerSymbol> {
        let outputs = unsafe {
            std::slice::from_raw_parts(
                (*self.aiger).outputs,
                (*self.aiger).num_outputs.try_into().unwrap(),
            )
        };
        outputs.iter().map(|symbol| AigerSymbol { symbol })
    }

    pub fn ands(&self) -> impl Iterator<Item = AigerAnd> {
        let ands = unsafe {
            std::slice::from_raw_parts(
                (*self.aiger).ands,
                (*self.aiger).num_ands.try_into().unwrap(),
            )
        };
        ands.iter().map(|and| AigerAnd { and })
    }
}

impl Drop for Aiger {
    fn drop(&mut self) {
        unsafe { aiger_reset(self.aiger) }
    }
}

impl AigerLit {
    pub const FALSE: Self = AigerLit(0);

    pub fn normalize(&self) -> (bool, Self) {
        (self.0 & 1 == 1, AigerLit(self.0 & !1))
    }
}

impl std::fmt::Display for AigerLit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Not for AigerLit {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0 ^ 1)
    }
}

impl<'a> AigerSymbol<'a> {
    pub fn lit(&self) -> AigerLit {
        AigerLit(self.symbol.lit)
    }

    pub fn next(&self) -> AigerLit {
        AigerLit(self.symbol.next)
    }

    pub fn reset(&self) -> u32 {
        self.symbol.reset
    }

    pub fn name(&self) -> Option<&str> {
        if self.symbol.name.is_null() {
            return None;
        }
        unsafe { CStr::from_ptr(self.symbol.name) }.to_str().ok()
    }
}

impl<'a> AigerAnd<'a> {
    pub fn lhs(&self) -> AigerLit {
        AigerLit(self.and.lhs)
    }

    pub fn rhs0(&self) -> AigerLit {
        AigerLit(self.and.rhs0)
    }

    pub fn rhs1(&self) -> AigerLit {
        AigerLit(self.and.rhs1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_creation() {
        let _aiger = Aiger::new();
    }
}
