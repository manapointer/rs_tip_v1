pub(crate) struct Spanned<T> {
    start: usize,
    end: usize,
    node: T,
}

pub(crate) type AstInt = Spanned<i32>;
pub(crate) type AstString = Spanned<String>;
pub(crate) type AstExpr = Spanned<Expr>;
pub(crate) type AstStmt = Spanned<Stmt>;

pub(crate) enum Stmt {
    Assign(AstString, AstExpr),
    Output(AstExpr),
    If(AstExpr, Box<AstStmt>, Option<Box<AstStmt>>),
    While(AstExpr, Box<AstStmt>),
}

pub(crate) enum Expr {
    Int(AstInt),
    Identifier(AstString),
    Binary(Box<AstExpr>, BinOp, Box<AstExpr>),
    Input,
}

pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    Equal,
}

pub(crate) trait IntoSpanned: Sized {
    fn into_spanned(self, start: usize, end: usize) -> Spanned<Self> {
        Spanned {
            start,
            end,
            node: self,
        }
    }
}

impl<T: Sized> IntoSpanned for T {}
