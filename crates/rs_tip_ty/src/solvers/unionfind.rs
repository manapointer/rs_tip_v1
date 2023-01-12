use crate::Term;
use petgraph::{graph::IndexType, unionfind::UnionFind};

pub struct UnionFindSolver {
    unionfind: UnionFind<Term>,
}

impl UnionFindSolver {
    pub fn new() -> UnionFindSolver {
        UnionFindSolver {
            unionfind: UnionFind::new(10),
        }
    }

    pub fn unify(&mut self, t1: Term, t2: Term) {
        if self.unionfind.union(t1, t2) {
            return;
        }

        match (t1, t2) {
            // (Term)
            _ => unreachable!(),
        }
    }
}

unsafe impl IndexType for Term {
    fn new(x: usize) -> Term {
        Term {
            interned: IndexType::new(x),
        }
    }

    fn index(&self) -> usize {
        IndexType::index(&self.interned)
    }

    fn max() -> Term {
        Term {
            interned: IndexType::max(),
        }
    }
}
