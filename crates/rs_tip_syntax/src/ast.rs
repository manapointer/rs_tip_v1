#[derive(Clone, Debug)]
pub(crate) struct Spanned<T> {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) node: T,
}

pub(crate) type AstInt = Spanned<i32>;
pub(crate) type AstString = Spanned<String>;
pub(crate) type AstExpr = Spanned<Expr>;
pub(crate) type AstField = Spanned<Field>;
pub(crate) type AstStmt = Spanned<Stmt>;
pub(crate) type AstFun = Spanned<Fun>;
pub(crate) type AstProg = Spanned<Prog>;

#[derive(Debug)]
pub(crate) struct Prog {
    pub(crate) funs: Vec<AstFun>,
}

#[derive(Debug)]
pub(crate) struct Fun {
    pub(crate) name: AstString,
    pub(crate) params: Vec<AstString>,
    // variables: Vec<AstString>,
    // statements: Vec<AstStmt>,
    // return_: AstExpr,
}

#[derive(Debug)]
pub(crate) enum Stmt {
    IdentifierAssign(AstString, AstExpr),
    PointerAssign(AstExpr, AstExpr),
    FieldAssign(AstString, AstString, AstExpr),
    DereferenceFieldAssign(AstExpr, AstString, AstExpr),
    Output(AstExpr),
    If(AstExpr, Box<AstStmt>, Option<Box<AstStmt>>),
    While(AstExpr, Box<AstStmt>),
}

#[derive(Debug)]
pub(crate) enum Expr {
    Int(AstInt),
    Identifier(AstString),
    Unary(UnOp, Box<AstExpr>),
    Binary(Box<AstExpr>, BinOp, Box<AstExpr>),
    Input,
    Call(Box<AstExpr>, Vec<AstExpr>),
    Alloc(Box<AstExpr>),
    Pointer(AstString),
    Dereference(Box<AstExpr>),
    Null,
    Record(Vec<AstField>),
    Field(Box<AstExpr>, AstString),
}

#[derive(Debug)]
pub(crate) struct Field {
    name: AstString,
    value: Box<AstExpr>,
}

#[derive(Debug)]
pub(crate) enum UnOp {
    Negative,
}

#[derive(Debug)]
pub(crate) enum BinOp {
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
