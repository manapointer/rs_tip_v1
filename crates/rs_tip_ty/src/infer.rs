use std::{collections::HashMap, fmt, ops::Deref};

use crate::{hash::RefMap, solvers::unionfind::UnionFindSolver, Ty, TyCtxt, TyKind};
use rs_tip_syntax::ast::{self, AstString};

#[derive(Debug)]
pub enum InferenceError {
    UndefinedVariable(String),
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InferenceError::UndefinedVariable(name) => write!(f, "undefined variable: {}", name),
        }
    }
}

pub type Result<T> = std::result::Result<T, InferenceError>;

#[derive(Default)]
struct Scope {
    name_to_ty: HashMap<String, Ty>,
}

impl Scope {
    fn add_names(&mut self, names: &[impl Deref<Target = String>], interner: TyCtxt) {
        for name in names.into_iter() {
            self.name_to_ty.insert(
                name.to_string(),
                TyKind::make_var(interner).intern(interner),
            );
        }
    }
}

pub struct InferenceResult {
    pub expr_to_ty: HashMap<u64, Ty>,
}

pub fn infer(tcx: TyCtxt<'_>, prog: &ast::AstProg) -> InferenceResult {
    let cx = InferenceContext::new(tcx);

    InferenceResult {
        expr_to_ty: HashMap::default(),
    }
}

struct InferenceContext<'a> {
    tcx: TyCtxt<'a>,
    expr_to_ty: RefMap<ast::AstExp, Ty>,
    solver: UnionFindSolver,
    scopes: Vec<Scope>,
}

impl<'a> InferenceContext<'a> {
    fn new(tcx: TyCtxt<'_>) -> InferenceContext {
        let global_scope = Scope::default();
        InferenceContext {
            tcx,
            expr_to_ty: RefMap::new(),
            solver: UnionFindSolver::default(),
            scopes: vec![global_scope],
        }
    }

    fn infer_prog(&mut self, prog: &ast::AstProg) -> Result<InferenceResult> {
        for fun in &prog.node.funs {
            self.infer_fun(fun)?;
        }
        todo!();
    }

    fn infer_fun(&mut self, fun: &ast::AstFun) -> Result<()> {
        self.scopes.push(Scope::default());
        self.add_names(&fun.params);
        self.add_names(&fun.vars);

        self.infer_stms(&fun.stms)?;

        self.scopes.pop();
        Ok(())
    }

    fn infer_stms(&mut self, stms: &Vec<ast::AstStm>) -> Result<()> {
        for stm in stms {
            self.infer_stm(stm)?;
        }
        Ok(())
    }

    fn infer_stm(&mut self, stm: &ast::AstStm) -> Result<()> {
        let node = &stm.node;
        match node {
            ast::Stm::IdentifierAssign(name, exp) => {
                let name_ty = self.lookup(name)?;
                let exp_ty = self.infer_exp(exp)?;
                // self.solver.unify(self.tcx, name_ty, exp_ty).unwrap();
                self.unify(name_ty, exp_ty);
            }
            ast::Stm::PointerAssign(target, exp) => {
                let target_ty = self.infer_exp(target)?;
                let pointer_ty = TyKind::Pointer(self.infer_exp(exp)?).intern(self.tcx);
                self.solver.unify(self.tcx, target_ty, pointer_ty).unwrap()
            }
            ast::Stm::FieldAssign(_, _, _) => todo!(),
            ast::Stm::DereferenceFieldAssign(_, _, _) => todo!(),
            ast::Stm::Output(exp) => {
                let exp_ty = self.infer_exp(exp)?;
                self.solver
                    .unify(self.tcx, exp_ty, self.tcx.common().int())
                    .unwrap();
            }
            ast::Stm::If(cond, then, else_) => {
                let exp_ty = self.infer_exp(cond)?;
                self.solver
                    .unify(self.tcx, exp_ty, self.tcx.common().int())
                    .unwrap();
                self.infer_stms(then)?;
                if let Some(else_) = else_ {
                    self.infer_stms(else_)?;
                }
            }
            ast::Stm::While(cond, body) => {
                let exp_ty = self.infer_exp(cond)?;
                self.solver
                    .unify(self.tcx, exp_ty, self.tcx.common().int())
                    .unwrap();
                self.infer_stms(body)?;
            }
        };
        Ok(())
    }

    fn infer_exp(&mut self, exp: &ast::AstExp) -> Result<Ty> {
        let node = &exp.node;

        if let ast::Exp::Identifier(name) = node {
            return self.lookup(name);
        }

        // Assign the expression a new type variable.
        let exp_ty = TyKind::make_var(self.tcx).intern(self.tcx);
        self.record_exp_ty(exp, exp_ty);

        match node {
            ast::Exp::Int(_) => {
                self.unify_int(exp_ty);
            }
            ast::Exp::Unary(_, operand) => {
                let operand_ty = self.infer_exp(operand)?;
                self.unify_int(operand_ty);
                self.unify_int(exp_ty);
            }
            ast::Exp::Binary(lhs, _, rhs) => {
                let lhs_ty = self.infer_exp(lhs)?;
                self.unify_int(lhs_ty);
                let rhs_ty = self.infer_exp(rhs)?;
                self.unify_int(rhs_ty);
                self.unify_int(exp_ty);
            }
            ast::Exp::Input => {
                self.unify_int(exp_ty);
            }
            ast::Exp::Call(callee, args) => {
                let args_tys: Vec<Ty> =
                    Result::from_iter(args.iter().map(|arg| self.infer_exp(arg)))?;
                let return_ty = TyKind::make_var(self.tcx).intern(self.tcx);
                let callee_ty = self.infer_exp(callee)?;
                let fun_ty = TyKind::Function(args_tys, return_ty).intern(self.tcx);
                self.unify(callee_ty, fun_ty);
                self.unify(exp_ty, return_ty);
            }
            ast::Exp::Alloc(alloc) => {
                let alloc_ty = self.infer_exp(alloc)?;
                self.unify(exp_ty, TyKind::Pointer(alloc_ty).intern(self.tcx));
            }
            ast::Exp::Pointer(name) => {
                self.unify(exp_ty, TyKind::Pointer(self.lookup(name)?).intern(self.tcx));
            }
            ast::Exp::Dereference(deref) => {
                let inner_ty = TyKind::make_var(self.tcx).intern(self.tcx);
                let pointer_ty = TyKind::Pointer(inner_ty).intern(self.tcx);
                let deref_ty = self.infer_exp(deref)?;
                self.unify(pointer_ty, deref_ty);
                self.unify(exp_ty, inner_ty);
            }
            ast::Exp::Null => {
                self.unify(exp_ty, TyKind::make_var(self.tcx).intern(self.tcx));
            }
            ast::Exp::Record(_) => todo!(),
            ast::Exp::Field(_, _) => todo!(),

            // Handled above.
            ast::Exp::Identifier(_) => unreachable!(),
        };
        Ok(exp_ty)
    }

    fn record_exp_ty(&mut self, exp: &ast::AstExp, ty: Ty) -> Ty {
        self.expr_to_ty.insert(exp, ty);
        ty
    }

    fn add_names(&mut self, names: &[AstString]) {
        let scope = self.scopes.last_mut().unwrap();
        scope.add_names(names, self.tcx);
    }

    fn lookup(&self, name: &str) -> Result<Ty> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.name_to_ty.get(name).cloned())
            .ok_or_else(|| InferenceError::UndefinedVariable(name.to_string()))
    }

    fn unify(&mut self, t1: Ty, t2: Ty) {
        self.solver.unify(self.tcx, t1, t2).unwrap();
    }

    fn unify_int(&mut self, ty: Ty) {
        self.unify(ty, self.tcx.common().int());
    }
}
