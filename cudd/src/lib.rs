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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CuddManager {
    ptr: *mut DdManager,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CuddNode<'a> {
    manager: &'a CuddManager,
    node: *mut DdNode,
}

#[derive(Debug, Copy, Clone)]
pub enum CuddReordering {
    Same,
    None,
    Random,
    Sift,
    GroupSift,
    LazySift,
}

impl CuddManager {
    pub fn new() -> Self {
        let ptr = unsafe { Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0) };
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn one(&self) -> CuddNode {
        CuddNode::new(self, unsafe { Cudd_ReadOne(self.ptr) })
    }

    pub fn zero(&self) -> CuddNode {
        CuddNode::new(self, unsafe { Cudd_Not(Cudd_ReadOne(self.ptr)) })
    }

    pub fn new_var(&self) -> CuddNode {
        CuddNode::new(self, unsafe { Cudd_bddNewVar(self.ptr) })
    }

    pub fn set_auto_dyn(&self, reordering: CuddReordering) {
        unsafe { Cudd_AutodynEnable(self.ptr, reordering.c_repr()) }
    }
}

impl Drop for CuddManager {
    fn drop(&mut self) {
        unsafe {
            Cudd_Quit(self.ptr);
        }
    }
}

impl<'a> CuddNode<'a> {
    fn new(manager: &'a CuddManager, node: *mut DdNode) -> Self {
        unsafe { Cudd_Ref(node) };
        Self { manager, node }
    }

    fn check_return_value(&self, val: *const DdNode) {
        if !val.is_null() {
            return;
        }
        match unsafe { Cudd_ReadErrorCode(self.manager.ptr) } {
            Cudd_ErrorType_CUDD_MEMORY_OUT => panic!("CUDD: Out of memory"),
            Cudd_ErrorType_CUDD_TOO_MANY_NODES => panic!("CUDD: Too many nodes"),
            Cudd_ErrorType_CUDD_MAX_MEM_EXCEEDED => panic!("CUDD: Maximum memory exceeded"),
            Cudd_ErrorType_CUDD_TIMEOUT_EXPIRED => {
                let lag = unsafe {
                    Cudd_ReadElapsedTime(self.manager.ptr) - Cudd_ReadTimeLimit(self.manager.ptr)
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
        let result =
            unsafe { Cudd_bddSetPiVar(self.manager.ptr, self.index().try_into().unwrap()) };
        assert!(result > 0);
    }

    pub fn set_present_state(&mut self) {
        let result =
            unsafe { Cudd_bddSetPsVar(self.manager.ptr, self.index().try_into().unwrap()) };
        assert!(result > 0);
    }

    pub fn and_abstract(self, and: &CuddNode, cube: &CuddNode) -> Self {
        assert_eq!(self.manager, cube.manager);
        let result =
            unsafe { Cudd_bddAndAbstract(self.manager.ptr, self.node, and.node, cube.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn exist_abstract(self, cube: &CuddNode) -> Self {
        assert_eq!(self.manager, cube.manager);
        let result = unsafe { Cudd_bddExistAbstract(self.manager.ptr, self.node, cube.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn univ_abstract(self, cube: &CuddNode) -> Self {
        assert_eq!(self.manager, cube.manager);
        let result = unsafe { Cudd_bddUnivAbstract(self.manager.ptr, self.node, cube.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn or(self, other: &CuddNode) -> Self {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddOr(self.manager.ptr, self.node, other.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn and(self, other: &CuddNode) -> Self {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddAnd(self.manager.ptr, self.node, other.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn and_assign(&mut self, other: &CuddNode) {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddAnd(self.manager.ptr, self.node, other.node) };
        self.check_return_value(result);
        *self = CuddNode::new(&self.manager, result)
    }

    pub fn implies(self, other: &CuddNode) -> Self {
        assert_eq!(self.manager, other.manager);
        // !self || other
        let result = unsafe { Cudd_bddOr(self.manager.ptr, Cudd_Not(self.node), other.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn xnor(self, other: &CuddNode) -> Self {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddXnor(self.manager.ptr, self.node, other.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn xor(&self, other: &CuddNode) -> Self {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddXor(self.manager.ptr, self.node, other.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn ite(&self, lhs: &CuddNode, rhs: &CuddNode) -> Self {
        assert_eq!(self.manager, lhs.manager);
        assert_eq!(self.manager, rhs.manager);
        let result = unsafe { Cudd_bddIte(self.manager.ptr, self.node, lhs.node, rhs.node) };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn vector_compose(self, vector: &[CuddNode]) -> Self {
        assert_eq!(vector.len(), unsafe { Cudd_ReadSize(self.manager.ptr) }
            as usize);
        let result = {
            let mut vector: Vec<*mut DdNode> = vector.iter().map(|node| node.node).collect();
            unsafe { Cudd_bddVectorCompose(self.manager.ptr, self.node, vector.as_mut_ptr()) }
        };
        self.check_return_value(result);
        CuddNode::new(&self.manager, result)
    }

    pub fn leq(&self, other: &CuddNode) -> bool {
        assert_eq!(self.manager, other.manager);
        let result = unsafe { Cudd_bddLeq(self.manager.ptr, self.node, other.node) };
        result > 0
    }

    pub fn print_minterms(&self) {
        unsafe { Cudd_PrintMinterm(self.manager.ptr, self.node) };
    }
}

impl<'a> Clone for CuddNode<'a> {
    fn clone(&self) -> Self {
        Self::new(&self.manager, self.node)
    }
}

impl<'a> Drop for CuddNode<'a> {
    fn drop(&mut self) {
        unsafe {
            Cudd_RecursiveDeref(self.manager.ptr, self.node);
        }
    }
}

impl<'a> std::ops::Not for CuddNode<'a> {
    type Output = Self;

    fn not(self) -> Self {
        CuddNode::new(&self.manager, Cudd_Not(self.node))
    }
}

impl<'a> std::ops::Not for &CuddNode<'a> {
    type Output = CuddNode<'a>;

    fn not(self) -> Self::Output {
        CuddNode::new(&self.manager, Cudd_Not(self.node))
    }
}

impl<'a> std::ops::BitAnd for CuddNode<'a> {
    type Output = Self;

    fn bitand(self, rhs: CuddNode) -> Self {
        assert_eq!(self.manager, rhs.manager);
        let result = unsafe { Cudd_bddAnd(self.manager.ptr, self.node, rhs.node) };
        CuddNode::new(&self.manager, result)
    }
}

impl<'a> std::ops::BitOr for CuddNode<'a> {
    type Output = Self;

    fn bitor(self, rhs: CuddNode) -> Self {
        assert_eq!(self.manager, rhs.manager);
        let result = unsafe { Cudd_bddOr(self.manager.ptr, self.node, rhs.node) };
        CuddNode::new(&self.manager, result)
    }
}

impl CuddReordering {
    fn c_repr(self) -> Cudd_ReorderingType {
        use CuddReordering::*;
        match self {
            Same => Cudd_ReorderingType_CUDD_REORDER_SAME,
            None => Cudd_ReorderingType_CUDD_REORDER_NONE,
            Random => Cudd_ReorderingType_CUDD_REORDER_RANDOM,
            Sift => Cudd_ReorderingType_CUDD_REORDER_SIFT,
            GroupSift => Cudd_ReorderingType_CUDD_REORDER_GROUP_SIFT,
            LazySift => Cudd_ReorderingType_CUDD_REORDER_LAZY_SIFT,
        }
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
