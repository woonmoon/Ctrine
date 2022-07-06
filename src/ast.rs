// use std::fmt::{Debug, Error, Formatter};

// more to come
pub enum TypeSpec {
    Int,
    Error, 
}

pub enum DeclSpecs {
    Type(TypeSpec),
    Error,
}

pub enum DirDeclarator {
    Id(String),
    Error,
}

pub enum Declarator {
    ValDecl(DirDeclarator), // not a pointer
    Error,
}

pub enum InitDecl {
    Decl(Declarator),
    // DeclAss(Declarator, "=", )
    Error,
}

pub enum InitDeclList {
    InitDeclL(InitDecl),
    Error,
}

// pub struct Declaration {
//     strspec: Option<StorageSpec>,
//     tspec: TypeSpec,
//     name: String,
//     val: Option<Any>,
// }

pub enum Decl {
    Decl(Box<DeclSpecs>),
    DeclList(Box<DeclSpecs>, Box<InitDeclList>),
    Error,
}