use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    hash::Hash,
};

pub mod solvers;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreshVarId(u32);

pub struct TyCtxt<'tcx> {
    inner: &'tcx TyCtxtInner,
}

pub struct TyCtxtInner {
    data: RefCell<InternerData>,
}

pub struct InternerData {
    counter: u32,

    ty_kinds_to_interned: HashMap<TyKind, u32>,
    interned_to_ty_kinds: HashMap<u32, TyKind>,

    term_kinds_to_interned: HashMap<TermKind, u32>,
    interned_to_term_kinds: HashMap<u32, TermKind>,
}

impl TyCtxtInner {
    fn intern_ty_kind(&self, kind: TyKind) -> u32 {
        let repr = self.data.borrow().ty_kinds_to_interned.get(&kind).cloned();
        match repr {
            Some(repr) => repr,
            None => {
                let mut data = self.data.borrow_mut();
                data.counter += 1;
                let counter = data.counter;
                data.ty_kinds_to_interned.insert(kind.clone(), counter);
                data.interned_to_ty_kinds.insert(counter, kind);
                counter
            }
        }
    }

    fn intern_term_kind(&self, kind: TermKind) -> u32 {
        let repr = self
            .data
            .borrow()
            .term_kinds_to_interned
            .get(&kind)
            .cloned();
        match repr {
            Some(repr) => repr,
            None => {
                let mut data = self.data.borrow_mut();
                data.counter += 1;
                let counter = data.counter;
                data.term_kinds_to_interned.insert(kind.clone(), counter);
                data.interned_to_term_kinds.insert(counter, kind);
                counter
            }
        }
    }

    fn ty_kind(&self, repr: u32) -> &TyKind {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TermKind {
    Var(Ty),
    Cons(Ty),
    Mu(Ty),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Term {
    interned: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TyKind {
    Int,
    Function(Vec<Term>),
    Pointer(Term),
    Record(Vec<Term>),
    AbsentField,
    Var(VarId),
    FreshVar(FreshVarId),
    Recursive(TermKind, TermKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ty {
    interned: u32,
}
