use crate::{ast::Spanned, lexer::Lexer, parser::ProgParser};
use expect_test::{expect, Expect};

fn check(input: &str, expect: Expect) {
    let lexer = Lexer::new(input);
    let Spanned { node: prog, .. } = ProgParser::new().parse(lexer).unwrap();
    expect.assert_eq(&format!("{:?}", prog));
}

#[test]
fn smoke_test() {
    check(
        "foo (a, b) { var x, y; x = null; return null; }",
        expect![[r#"Prog { funs: [Spanned { start: 0, end: 47, node: Fun { name: Spanned { start: 0, end: 3, node: "foo" }, params: [Spanned { start: 5, end: 6, node: "a" }, Spanned { start: 8, end: 9, node: "b" }], vars: [Spanned { start: 17, end: 18, node: "x" }, Spanned { start: 20, end: 21, node: "y" }], stms: [Spanned { start: 23, end: 32, node: IdentifierAssign(Spanned { start: 23, end: 24, node: "x" }, Spanned { start: 27, end: 31, node: Null }) }], return_: Spanned { start: 0, end: 0, node: Null } } }] }"#]],
    )
}
