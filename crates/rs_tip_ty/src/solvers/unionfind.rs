use crate::{TermKind, Ty, TyCtxt};
use anyhow::{anyhow, Result};
use std::{fmt, hash::Hash};

#[derive(Debug, Clone)]
struct UnionFind<K> {
    parent: Vec<K>,
}

#[inline]
unsafe fn get_unchecked<K>(xs: &[K], index: usize) -> &K {
    debug_assert!(index < xs.len());
    xs.get_unchecked(index)
}

#[inline]
unsafe fn get_unchecked_mut<K>(xs: &mut [K], index: usize) -> &mut K {
    debug_assert!(index < xs.len());
    xs.get_unchecked_mut(index)
}

impl<K> UnionFind<K>
where
    K: IndexType,
{
    fn new(n: usize) -> Self {
        let parent = (0..n).map(K::new).collect::<Vec<K>>();

        UnionFind { parent }
    }

    fn find(&self, x: K) -> K {
        assert!(x.index() < self.parent.len());
        unsafe {
            let mut x = x;
            loop {
                // Use unchecked indexing because we can trust the internal set ids.
                let xparent = *get_unchecked(&self.parent, x.index());
                if xparent == x {
                    break;
                }
                x = xparent;
            }
            x
        }
    }

    fn find_mut(&mut self, x: K) -> K {
        assert!(x.index() < self.parent.len());
        unsafe { self.find_mut_recursive(x) }
    }

    unsafe fn find_mut_recursive(&mut self, mut x: K) -> K {
        let mut parent = *get_unchecked(&self.parent, x.index());
        while parent != x {
            let grandparent = *get_unchecked(&self.parent, parent.index());
            *get_unchecked_mut(&mut self.parent, x.index()) = grandparent;
            x = parent;
            parent = grandparent;
        }
        x
    }

    fn equiv(&self, x: K, y: K) -> bool {
        self.find(x) == self.find(y)
    }

    fn union(&mut self, x: K, y: K) -> bool {
        if x == y {
            return false;
        }
        let xrep = self.find_mut(x);
        let yrep = self.find_mut(y);

        if xrep == yrep {
            return false;
        }
        self.parent[xrep.index()] = yrep;
        true
    }
}

unsafe trait IndexType: Copy + Default + Hash + Ord + fmt::Debug + 'static {
    fn new(x: usize) -> Self;
    fn index(&self) -> usize;
    fn max() -> Self;
}

unsafe impl IndexType for u32 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u32
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u32::MAX
    }
}

pub struct UnionFindSolver {
    unionfind: UnionFind<Ty>,
}

impl UnionFindSolver {
    pub fn unify(&mut self, interner: TyCtxt, t1: Ty, t2: Ty) -> Result<()> {
        if self.unionfind.equiv(t1, t2) {
            Ok(())
        } else {
            let t1_kind = t1.kind(interner);
            let t2_kind = t2.kind(interner);
            match (t1_kind.term_kind(), t2_kind.term_kind()) {
                (TermKind::Var, TermKind::Var) | (TermKind::Var, _) => {
                    self.unionfind.union(t1, t2);
                }
                (_, TermKind::Var) => {
                    self.unionfind.union(t2, t1);
                }
                (TermKind::Cons, TermKind::Cons) if t1_kind.matches(&t2_kind) => {
                    self.unionfind.union(t1, t2);
                }
                _ => return Err(anyhow!("cannot unify {:?} and {:?}", t1_kind, t2_kind)),
            };
            Ok(())
        }
    }
}

impl Default for UnionFindSolver {
    fn default() -> UnionFindSolver {
        Self {
            unionfind: UnionFind::new(10),
        }
    }
}

unsafe impl IndexType for Ty {
    fn new(x: usize) -> Ty {
        Ty {
            interned: IndexType::new(x),
        }
    }

    fn index(&self) -> usize {
        IndexType::index(&self.interned)
    }

    fn max() -> Ty {
        Ty {
            interned: IndexType::max(),
        }
    }
}
