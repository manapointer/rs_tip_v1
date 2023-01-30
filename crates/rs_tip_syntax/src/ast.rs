#[derive(Clone, Debug)]
pub(crate) struct Spanned<T> {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) node: T,
}

pub(crate) type AstInt = Spanned<i32>;
pub(crate) type AstString = Spanned<String>;
pub(crate) type AstExp = Spanned<Exp>;
pub(crate) type AstField = Spanned<Field>;
pub(crate) type AstStm = Spanned<Stm>;
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
    pub(crate) vars: Vec<AstString>,
    pub(crate) stms: Vec<AstStm>,
    pub(crate) return_: AstExp,
}

#[derive(Debug)]
pub(crate) enum Stm {
    IdentifierAssign(AstString, AstExp),
    PointerAssign(AstExp, AstExp),
    FieldAssign(AstString, AstString, AstExp),
    DereferenceFieldAssign(AstExp, AstString, AstExp),
    Output(AstExp),
    If(AstExp, Vec<AstStm>, Option<Vec<AstStm>>),
    While(AstExp, Vec<AstStm>),
}

#[derive(Debug)]
pub(crate) enum Exp {
    Int(AstInt),
    Identifier(AstString),
    Unary(UnOp, Box<AstExp>),
    Binary(Box<AstExp>, BinOp, Box<AstExp>),
    Input,
    Call(Box<AstExp>, Vec<AstExp>),
    Alloc(Box<AstExp>),
    Pointer(AstString),
    Dereference(Box<AstExp>),
    Null,
    Record(Vec<AstField>),
    Field(Box<AstExp>, AstString),
}

#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) name: AstString,
    pub(crate) value: Box<AstExp>,
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
