use crate::Ty;
use petgraph::{graph::IndexType, unionfind::UnionFind};

pub struct UnionFindSolver {
    unionfind: UnionFind<Ty>,
}

impl UnionFindSolver {
    pub fn new() -> UnionFindSolver {
        UnionFindSolver {
            unionfind: UnionFind::new(10),
        }
    }

    pub fn unify(&mut self, t1: Ty, t2: Ty) {
        if self.unionfind.union(t1, t2) {
            return;
        }

        match (t1, t2) {
            // (Ty)
            _ => unreachable!(),
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
