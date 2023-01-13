use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use hash::hash;

mod hash;
pub mod solvers;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VarId(u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FreshVarId(u32);

pub struct Interners {
    counter: u32,
    ty_kinds_to_interned: HashMap<u64, u32>,
    interned_to_ty_kinds: HashMap<u32, Rc<TyKind>>,
}

#[derive(Clone, Copy)]
pub struct TyCtxt<'tcx> {
    inner: &'tcx TyCtxtInner,
}

impl<'tcx> TyCtxt<'tcx> {
    pub fn intern_ty_kind(self, kind: TyKind) -> u32 {
        self.inner.intern_ty_kind(kind)
    }

    pub fn repr(self, kind: &TyKind) -> Option<u32> {
        self.inner.repr(kind)
    }

    pub fn ty_kind(self, repr: u32) -> Rc<TyKind> {
        self.inner.ty_kind(repr)
    }
}

pub struct TyCtxtInner {
    interners: RefCell<Interners>,
}

impl TyCtxtInner {
    fn intern_ty_kind(&self, kind: TyKind) -> u32 {
        let hash = hash(&kind);
        let repr = self
            .interners
            .borrow()
            .ty_kinds_to_interned
            .get(&hash)
            .cloned();
        match repr {
            Some(repr) => repr,
            None => {
                let mut interners = self.interners.borrow_mut();
                interners.counter += 1;
                let counter = interners.counter;
                interners.ty_kinds_to_interned.insert(hash, counter);
                interners
                    .interned_to_ty_kinds
                    .insert(counter, Rc::new(kind));
                counter
            }
        }
    }

    fn repr(&self, kind: &TyKind) -> Option<u32> {
        self.interners
            .borrow()
            .ty_kinds_to_interned
            .get(&hash(kind))
            .cloned()
    }

    fn ty_kind(&self, repr: u32) -> Rc<TyKind> {
        match self
            .interners
            .borrow()
            .interned_to_ty_kinds
            .get(&repr)
            .cloned()
        {
            Some(kind) => kind,
            None => panic!("invalid repr supplied to ty_kind"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TermKind {
    Var,
    Cons,
    Mu,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TyKind {
    Int,
    Function(Vec<Ty>, Ty),
    Pointer(Ty),
    Record(Vec<Ty>),
    AbsentField,
    Var(VarId),
    FreshVar(FreshVarId),
    Recursive(VarId, Ty),
}

impl TyKind {
    pub fn term_kind(&self) -> TermKind {
        match self {
            TyKind::Var(_) | TyKind::FreshVar(_) => TermKind::Var,
            TyKind::Int
            | TyKind::Function(_, _)
            | TyKind::Pointer(_)
            | TyKind::Record(_)
            | TyKind::AbsentField => TermKind::Cons,
            TyKind::Recursive(_, _) => TermKind::Mu,
        }
    }

    pub fn substitute(&self, interner: TyCtxt, from: VarId, to: Ty) -> Ty {
        match self {
            TyKind::Int => Ty::for_kind(interner, self),
            TyKind::Function(params, ret) => {
                let params = params
                    .iter()
                    .map(|param| param.substitute(interner, from, to))
                    .collect();
                Ty::intern(
                    interner,
                    TyKind::Function(params, ret.substitute(interner, from, to)),
                )
            }
            TyKind::Pointer(of) => {
                Ty::intern(interner, TyKind::Pointer(of.substitute(interner, from, to)))
            }
            TyKind::Record(args) => Ty::intern(
                interner,
                TyKind::Record(
                    args.iter()
                        .map(|arg| arg.substitute(interner, from, to))
                        .collect(),
                ),
            ),
            TyKind::AbsentField => Ty::for_kind(interner, self),
            TyKind::Var(v) => {
                if *v == from {
                    Ty::for_kind(interner, self)
                } else {
                    to
                }
            }
            TyKind::FreshVar(_) => todo!(),
            TyKind::Recursive(v, ty) => {
                if *v == from {
                    Ty::for_kind(interner, self)
                } else {
                    Ty::intern(
                        interner,
                        TyKind::Recursive(*v, ty.substitute(interner, from, to)),
                    )
                }
            }
        }
    }

    fn arity(&self) -> usize {
        match self {
            TyKind::Function(params, _) => params.len() + 1,
            TyKind::Pointer(_) => 1,
            _ => 0,
        }
    }

    fn matches(&self, other: &TyKind) -> bool {
        if self.term_kind() != TermKind::Cons || other.term_kind() != TermKind::Cons {
            return false;
        }
        match (self, other) {
            (TyKind::Int, TyKind::Int)
            | (TyKind::Function(_, _), TyKind::Function(_, _))
            | (TyKind::Pointer(_), TyKind::Pointer(_))
            | (TyKind::Record(_), TyKind::Record(_))
            | (TyKind::AbsentField, TyKind::AbsentField) => self.arity() == other.arity(),
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Ty {
    interned: u32,
}

impl Ty {
    fn intern(interner: TyCtxt, kind: TyKind) -> Ty {
        Ty {
            interned: interner.intern_ty_kind(kind),
        }
    }

    fn for_kind(interner: TyCtxt, kind: &TyKind) -> Ty {
        interner
            .repr(kind)
            .map(|repr| Ty { interned: repr })
            .unwrap()
    }

    fn substitute(self, interner: TyCtxt, from: VarId, to: Ty) -> Ty {
        let kind = self.kind(interner);
        kind.substitute(interner, from, to)
    }

    fn kind(self, interner: TyCtxt) -> Rc<TyKind> {
        interner.ty_kind(self.interned)
    }
}
