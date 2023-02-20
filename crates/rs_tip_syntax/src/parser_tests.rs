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
        expect![[
            r#"Prog { funs: [Spanned { start: 0, end: 47, node: Fun { name: Spanned { start: 0, end: 3, node: "foo" }, params: [Spanned { start: 5, end: 6, node: "a" }, Spanned { start: 8, end: 9, node: "b" }], vars: [Spanned { start: 17, end: 18, node: "x" }, Spanned { start: 20, end: 21, node: "y" }], stms: [Spanned { start: 23, end: 32, node: IdentifierAssign(Spanned { start: 23, end: 24, node: "x" }, Spanned { start: 27, end: 31, node: Null }) }], return_: Spanned { start: 40, end: 44, node: Null } } }] }"#
        ]],
    );
}

#[test]
fn test_complicated() {
    check(
        r#"
foo(p,x) {
    var f,q;
    if (*p==0) { f=1; }
    else {
        q = alloc 0;
        *q = (*p)-1;
        f=(*p)*(x(q,x));
    }
    return f;
}

main() {
    var n;
    n = input;
    return foo(&n,foo);
}
"#,
        expect![[
            r#"Prog { funs: [Spanned { start: 1, end: 148, node: Fun { name: Spanned { start: 1, end: 4, node: "foo" }, params: [Spanned { start: 5, end: 6, node: "p" }, Spanned { start: 7, end: 8, node: "x" }], vars: [Spanned { start: 20, end: 21, node: "f" }, Spanned { start: 22, end: 23, node: "q" }], stms: [Spanned { start: 29, end: 132, node: If(Spanned { start: 33, end: 38, node: Binary(Spanned { start: 33, end: 35, node: Dereference(Spanned { start: 34, end: 35, node: Identifier(Spanned { start: 34, end: 35, node: "p" }) }) }, Equal, Spanned { start: 37, end: 38, node: Int(Spanned { start: 37, end: 38, node: 0 }) }) }, [Spanned { start: 42, end: 46, node: IdentifierAssign(Spanned { start: 42, end: 43, node: "f" }, Spanned { start: 44, end: 45, node: Int(Spanned { start: 44, end: 45, node: 1 }) }) }], Some([Spanned { start: 68, end: 80, node: IdentifierAssign(Spanned { start: 68, end: 69, node: "q" }, Spanned { start: 72, end: 79, node: Alloc(Spanned { start: 78, end: 79, node: Int(Spanned { start: 78, end: 79, node: 0 }) }) }) }, Spanned { start: 89, end: 101, node: PointerAssign(Spanned { start: 90, end: 91, node: Identifier(Spanned { start: 90, end: 91, node: "q" }) }, Spanned { start: 94, end: 100, node: Binary(Spanned { start: 94, end: 98, node: Paren(Spanned { start: 95, end: 97, node: Dereference(Spanned { start: 96, end: 97, node: Identifier(Spanned { start: 96, end: 97, node: "p" }) }) }) }, Subtract, Spanned { start: 99, end: 100, node: Int(Spanned { start: 99, end: 100, node: 1 }) }) }) }, Spanned { start: 110, end: 126, node: IdentifierAssign(Spanned { start: 110, end: 111, node: "f" }, Spanned { start: 112, end: 125, node: Binary(Spanned { start: 112, end: 116, node: Paren(Spanned { start: 113, end: 115, node: Dereference(Spanned { start: 114, end: 115, node: Identifier(Spanned { start: 114, end: 115, node: "p" }) }) }) }, Multiply, Spanned { start: 117, end: 125, node: Paren(Spanned { start: 118, end: 124, node: Call(Spanned { start: 118, end: 119, node: Identifier(Spanned { start: 118, end: 119, node: "x" }) }, [Spanned { start: 120, end: 121, node: Identifier(Spanned { start: 120, end: 121, node: "q" }) }, Spanned { start: 122, end: 123, node: Identifier(Spanned { start: 122, end: 123, node: "x" }) }]) }) }) }) }])) }], return_: Spanned { start: 144, end: 145, node: Identifier(Spanned { start: 144, end: 145, node: "f" }) } } }, Spanned { start: 150, end: 210, node: Fun { name: Spanned { start: 150, end: 154, node: "main" }, params: [], vars: [Spanned { start: 167, end: 168, node: "n" }], stms: [Spanned { start: 174, end: 184, node: IdentifierAssign(Spanned { start: 174, end: 175, node: "n" }, Spanned { start: 178, end: 183, node: Input }) }], return_: Spanned { start: 196, end: 207, node: Call(Spanned { start: 196, end: 199, node: Identifier(Spanned { start: 196, end: 199, node: "foo" }) }, [Spanned { start: 200, end: 202, node: Pointer(Spanned { start: 201, end: 202, node: "n" }) }, Spanned { start: 203, end: 206, node: Identifier(Spanned { start: 203, end: 206, node: "foo" }) }]) } } }] }"#
        ]],
    );
}
