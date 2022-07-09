use std::fmt::{Debug};

// more to come
#[derive(Debug)]
pub enum TypeSpec {
    Int,
    Error, 
}

#[derive(Debug)]
pub enum DeclSpecs {
    Type(TypeSpec),
    Error,
}

#[derive(Debug)]
pub enum DirDeclarator {
    Id(String),
    Error,
}

#[derive(Debug)]
pub enum Declarator {
    ValDecl(DirDeclarator),
    PointerDecl(DirDeclarator),
    Error,
}

#[derive(Debug)]
pub enum InitDecl {
    Decl(Declarator),
    // DeclAss(Declarator, "=", )
    Error,
}

// pub struct Declaration {
//     strspec: Option<StorageSpec>,
//     tspec: TypeSpec,
//     name: String,
//     val: Option<Any>,
// }

#[derive(Debug)]
pub enum Decl {
    Decl(DeclSpecs),
    DeclList(DeclSpecs, Vec<Box<InitDecl>>),
    Error,
}