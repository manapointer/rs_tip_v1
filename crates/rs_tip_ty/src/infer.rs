use std::collections::HashMap;

use crate::{hash, solvers::unionfind::UnionFindSolver, Ty, TyCtxt};
use rs_tip_syntax::ast;

pub struct InferenceResult {
    pub expr_to_ty: HashMap<u64, Ty>,
}

pub fn infer(prog: &ast::AstProg) -> InferenceResult {
    let cx = InferenceContext::default();

    InferenceResult {
        expr_to_ty: HashMap::default(),
    }
}

#[derive(Default)]
struct InferenceContext<'a> {
    tcx: TyCtxt<'a>,
    expr_to_ty: HashMap<u64, Ty>,
    solver: UnionFindSolver,
}

impl InferenceContext {
    fn infer_prog(&mut self, prog: &ast::AstProg) {
        prog.node.funs.iter().for_each(|fun| self.infer_fun(fun))
    }

    fn infer_fun(&mut self, fun: &ast::AstFun) {}

    fn infer_exp(&mut self, exp: &ast::AstExp) -> Ty {
        let exp = &exp.node;
        let ty = match exp {
            ast::Exp::Int(_) => self.record_exp_ty(exp),
            ast::Exp::Identifier(_) => todo!(),
            ast::Exp::Unary(_, _) => todo!(),
            ast::Exp::Binary(_, _, _) => todo!(),
            ast::Exp::Input => todo!(),
            ast::Exp::Call(_, _) => todo!(),
            ast::Exp::Alloc(_) => todo!(),
            ast::Exp::Pointer(_) => todo!(),
            ast::Exp::Dereference(_) => todo!(),
            ast::Exp::Null => todo!(),
            ast::Exp::Record(_) => todo!(),
            ast::Exp::Field(_, _) => todo!(),
        };

        todo!()
    }

    fn record_exp_ty(&mut self, exp: &ast::Exp, ty: Ty) {
        self.expr_to_ty.insert(hash::hash(exp), ty);
    }
}
