use lalrpop_util::lalrpop_mod;

pub mod ast;
mod lexer;

#[cfg(test)]
mod parser_tests;

lalrpop_mod!(pub parser);
