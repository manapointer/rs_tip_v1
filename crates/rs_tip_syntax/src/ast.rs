use std::ops::Deref;

#[derive(Clone, Debug, Hash)]
pub struct Spanned<T> {
    pub start: usize,
    pub end: usize,
    pub node: T,
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

pub type AstInt = Spanned<i32>;
pub type AstString = Spanned<String>;
pub type AstExp = Spanned<Exp>;
pub type AstField = Spanned<Field>;
pub type AstStm = Spanned<Stm>;
pub type AstFun = Spanned<Fun>;
pub type AstProg = Spanned<Prog>;

#[derive(Debug, Hash)]
pub struct Prog {
    pub funs: Vec<AstFun>,
}

#[derive(Debug, Hash)]
pub struct Fun {
    pub name: AstString,
    pub params: Vec<AstString>,
    pub vars: Vec<AstString>,
    pub stms: Vec<AstStm>,
    pub return_: AstExp,
}

#[derive(Debug, Hash)]
pub enum Stm {
    IdentifierAssign(AstString, AstExp),
    PointerAssign(AstExp, AstExp),
    FieldAssign(AstString, AstString, AstExp),
    DereferenceFieldAssign(AstExp, AstString, AstExp),
    Output(AstExp),
    If(AstExp, Vec<AstStm>, Option<Vec<AstStm>>),
    While(AstExp, Vec<AstStm>),
}

#[derive(Debug, Hash)]
pub enum Exp {
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

#[derive(Debug, Hash)]
pub struct Field {
    pub name: AstString,
    pub value: Box<AstExp>,
}

#[derive(Debug, Hash)]
pub enum UnOp {
    Negative,
}

#[derive(Debug, Hash)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    Equal,
}

pub trait IntoSpanned: Sized {
    fn into_spanned(self, start: usize, end: usize) -> Spanned<Self> {
        Spanned {
            start,
            end,
            node: self,
        }
    }
}

impl<T: Sized> IntoSpanned for T {}
