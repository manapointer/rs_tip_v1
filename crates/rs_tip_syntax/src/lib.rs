use lalrpop_util::lalrpop_mod;

mod ast;
mod lexer;

#[cfg(test)]
mod parser_tests;

lalrpop_mod!(pub parser);
