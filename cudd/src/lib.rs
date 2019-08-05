#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include bindings g(* right_val).try_into().unwrap() `rust-bindgen`
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::convert::TryInto;

/// was C macro `#define Cudd_Not(node) ((DdNode *)((uintptr_t)(node) ^ (uintptr_t) 01))`
fn Cudd_Not(node: *mut DdNode) -> *mut DdNode {
    unsafe {
        let val = std::mem::transmute::<*mut DdNode, usize>(node) ^ 1;
        std::mem::transmute::<usize, *mut DdNode>(val)
    }
}

#[derive(Debug)]
pub struct CuddManager {
    manager: *mut DdManager,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CuddNode {
    manager: *mut DdManager,
    node: *mut DdNode,
}

impl CuddManager {
    pub fn new() -> Self {
        let manager = unsafe { Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0) };
        assert!(!manager.is_null());
        Self { manager }
    }

    pub fn one(&self) -> CuddNode {
        CuddNode::new(self.manager, unsafe { Cudd_ReadOne(self.manager) })
    }

    pub fn zero(&self) -> CuddNode {
        CuddNode::new(self.manager, unsafe {
            Cudd_Not(Cudd_ReadOne(self.manager))
        })
    }

    pub fn new_var(&self) -> CuddNode {
        CuddNode::new(self.manager, unsafe { Cudd_bddNewVar(self.manager) })
    }
}

impl Drop for CuddManager {
    fn drop(&mut self) {
        unsafe {
            Cudd_Quit(self.manager);
        }
    }
}

impl CuddNode {
    fn new(manager: *mut DdManager, node: *mut DdNode) -> Self {
        unsafe { Cudd_Ref(node) };
        Self { manager, node }
    }

    fn check_return_value(&self, val: *const DdNode) {
        if !val.is_null() {
            return;
        }
        match unsafe { Cudd_ReadErrorCode(self.manager) } {
            Cudd_ErrorType_CUDD_MEMORY_OUT => panic!("CUDD: Out of memory"),
            Cudd_ErrorType_CUDD_TOO_MANY_NODES => panic!("CUDD: Too many nodes"),
            Cudd_ErrorType_CUDD_MAX_MEM_EXCEEDED => panic!("CUDD: Maximum memory exceeded"),
            Cudd_ErrorType_CUDD_TIMEOUT_EXPIRED => {
                let lag = unsafe {
                    Cudd_ReadElapsedTime(self.manager) - Cudd_ReadTimeLimit(self.manager)
                };
                panic!("CUDD: Timeout expired.  Lag = {} ms", lag);
            }
            Cudd_ErrorType_CUDD_TERMINATION => panic!("CUDD: Terminated"),
            Cudd_ErrorType_CUDD_INVALID_ARG => panic!("CUDD: Invalid argument"),
            Cudd_ErrorType_CUDD_INTERNAL_ERROR => panic!("CUDD: Internal error"),
            Cudd_ErrorType_CUDD_NO_ERROR => panic!("Unexpected error"),
            _ => unreachable!(),
        }
    }

    pub fn index(&self) -> u32 {
        unsafe { Cudd_NodeReadIndex(self.node) }
    }

    pub fn set_primary_input(&mut self) {
        let result = unsafe { Cudd_bddSetPiVar(self.manager, self.index().try_into().unwrap()) };
        assert!(result > 0);
    }

    pub fn set_present_state(&mut self) {
        let result = unsafe { Cudd_bddSetPsVar(self.manager, self.index().try_into().unwrap()) };
        assert!(result > 0);
    }

    pub fn and_abstract(self, and: &CuddNode, cube: &CuddNode) -> CuddNode {
        assert_eq!(self.manager, cube.manager);
        let result = unsafe { Cudd_bddAndAbstract(self.manager, self.node, and.node, cube.node) };
        self.check_return_value(result);
        CuddNode::new(self.manager, result)
    }

    pub fn exist_abstract(self, cube: &CuddNode) -> CuddNode {
        assert_eq!(self.manager, cube.manager);
        let result = unsafe { Cudd_bddExistAbstract(self.manager, self.node, cube.node) };
        self.check_return_value(result);
        CuddNode::new(self.manager, result)
    }

    pub fn univ_abstract(self, cube: &CuddNode) -> CuddNode {
        assert_eq!(self.manager, cube.manager);
        let result = unsafe { Cudd_bddUnivAbstract(self.manager, self.node, cube.node) };
        self.check_return_value(result);
        CuddNode::new(self.manager, result)
    }

    pub fn or(self, other: &CuddNode) -> CuddNode {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddOr(self.manager, self.node, other.node) };
        self.check_return_value(result);
        CuddNode::new(self.manager, result)
    }

    pub fn and(self, other: &CuddNode) -> CuddNode {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddAnd(self.manager, self.node, other.node) };
        self.check_return_value(result);
        CuddNode::new(self.manager, result)
    }

    pub fn and_assign(&mut self, other: &CuddNode) {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddAnd(self.manager, self.node, other.node) };
        self.check_return_value(result);
        *self = CuddNode::new(self.manager, result)
    }

    pub fn vector_compose(self, vector: &[CuddNode]) -> CuddNode {
        assert_eq!(vector.len(), unsafe { Cudd_ReadSize(self.manager) }
            as usize);
        let result = {
            let mut vector: Vec<*mut DdNode> = vector.iter().map(|node| node.node).collect();
            unsafe { Cudd_bddVectorCompose(self.manager, self.node, vector.as_mut_ptr()) }
        };
        self.check_return_value(result);
        CuddNode::new(self.manager, result)
    }

    pub fn leq(&self, other: &CuddNode) -> bool {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddLeq(self.manager, self.node, other.node) };
        result > 0
    }
}

impl Clone for CuddNode {
    fn clone(&self) -> Self {
        Self::new(self.manager, self.node)
    }
}

impl Drop for CuddNode {
    fn drop(&mut self) {
        unsafe {
            Cudd_RecursiveDeref(self.manager, self.node);
        }
    }
}

impl std::ops::Not for CuddNode {
    type Output = CuddNode;

    fn not(self) -> Self {
        CuddNode::new(self.manager, Cudd_Not(self.node))
    }
}

impl std::ops::Not for &CuddNode {
    type Output = CuddNode;

    fn not(self) -> CuddNode {
        CuddNode::new(self.manager, Cudd_Not(self.node))
    }
}

impl std::ops::BitAnd for CuddNode {
    type Output = CuddNode;

    fn bitand(self, rhs: CuddNode) -> Self {
        assert_eq!(self.manager, rhs.manager);
        let result = unsafe { Cudd_bddAnd(self.manager, self.node, rhs.node) };
        CuddNode::new(self.manager, result)
    }
}

impl std::ops::BitOr for CuddNode {
    type Output = CuddNode;

    fn bitor(self, rhs: CuddNode) -> Self {
        assert_eq!(self.manager, rhs.manager);
        let result = unsafe { Cudd_bddOr(self.manager, self.node, rhs.node) };
        CuddNode::new(self.manager, result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_creation() {
        let _manager = CuddManager::new();
    }

    #[test]
    fn test_simple() {
        let manager = CuddManager::new();
        let one = manager.one();
        let zero = manager.zero();
        let var = manager.new_var();
        let var2 = manager.new_var();
        let one2 = manager.one();

        assert_ne!(&one, &zero);
        assert_ne!(one.clone(), var.clone());
        assert_ne!(one.clone(), var2.clone());
        assert_ne!(var.clone(), var2.clone());
        assert_eq!(one.clone(), one2.clone());
        assert_eq!(one.clone(), !zero.clone());
        assert_eq!(zero.clone() & one.clone(), zero);
        assert_eq!(zero.clone() | one.clone(), one);
        assert_eq!(one.clone() & var.clone(), var);
        assert_eq!(var.clone() & var2.clone(), var2.clone() & var.clone());
    }
}
