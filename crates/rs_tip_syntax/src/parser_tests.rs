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
        "foo ()",
        expect![[
            r#"Prog { funs: [Spanned { start: 0, end: 6, node: Fun { name: Spanned { start: 0, end: 3, node: "foo" }, params: [] } }] }"#
        ]],
    )
}
