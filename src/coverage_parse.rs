use crate::*;

// This module's goal is complete coverage of the parse function

#[test]
fn basic() {
    let program = "hss";
    let funcs = parse(lex(program));
    let desired_funcs = vec![
        Func::Basic(BasicFunc::Head),
        Func::Basic(BasicFunc::Sum),
        Func::Basic(BasicFunc::Sum),
    ];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn higher() {
    let program = "mhhm";
    let funcs = parse(lex(program));
    let desired_funcs = vec![
        Func::Higher(HigherFunc::Map, Box::new(Func::Basic(BasicFunc::Head))),
        Func::Basic(BasicFunc::Head),
        Func::Higher(HigherFunc::Map, Box::new(Func::Bound(vec![]))),
    ];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn bind() {
    let program = "mhmmzz";
    let funcs = parse(lex(program));
    let desired_funcs = vec![
        Func::Higher(HigherFunc::Map, Box::new(Func::Basic(BasicFunc::Head))),
        Func::Higher(
            HigherFunc::Map,
            Box::new(Func::Bound(vec![Func::Higher(
                HigherFunc::Map,
                Box::new(Func::Bound(vec![])),
            )])),
        ),
    ];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn open_higher() {
    let program = "mmm";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Higher(
        HigherFunc::Map,
        Box::new(Func::Higher(
            HigherFunc::Map,
            Box::new(Func::Higher(HigherFunc::Map, Box::new(Func::Bound(vec![])))),
        )),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn quote() {
    let program = "ihmhmhmhzhzhq";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Higher(
        HigherFunc::Inverse,
        Box::new(Func::Bound(vec![
            Func::Basic(BasicFunc::Head),
            Func::Higher(HigherFunc::Map, Box::new(Func::Basic(BasicFunc::Head))),
            Func::Higher(
                HigherFunc::Map,
                Box::new(Func::Bound(vec![
                    Func::Basic(BasicFunc::Head),
                    Func::Higher(
                        HigherFunc::Map,
                        Box::new(Func::Bound(vec![Func::Basic(BasicFunc::Head)])),
                    ),
                    Func::Basic(BasicFunc::Head),
                ])),
            ),
            Func::Basic(BasicFunc::Head),
        ])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn double() {
    let program = "bhhzhhz";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Double(
        DoubleFunc::Bifurcate,
        Box::new(Func::Bound(vec![
            Func::Basic(BasicFunc::Head),
            Func::Basic(BasicFunc::Head),
        ])),
        Box::new(Func::Bound(vec![
            Func::Basic(BasicFunc::Head),
            Func::Basic(BasicFunc::Head),
        ])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn double_quote() {
    let program = "bqhhqhhz";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Double(
        DoubleFunc::Bifurcate,
        Box::new(Func::Bound(vec![
            Func::Basic(BasicFunc::Head),
            Func::Basic(BasicFunc::Head),
        ])),
        Box::new(Func::Bound(vec![
            Func::Basic(BasicFunc::Head),
            Func::Basic(BasicFunc::Head),
        ])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn double_skip() {
    let program = "mbq";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Higher(
        HigherFunc::Map,
        Box::new(Func::Bound(vec![Func::Double(
            DoubleFunc::Bifurcate,
            Box::new(Func::Bound(vec![])),
            Box::new(Func::Bound(vec![])),
        )])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn double_half_skip() {
    let program = "mbhq";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Higher(
        HigherFunc::Map,
        Box::new(Func::Bound(vec![Func::Double(
            DoubleFunc::Bifurcate,
            Box::new(Func::Basic(BasicFunc::Head)),
            Box::new(Func::Bound(vec![])),
        )])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn double_end() {
    let program = "b";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Double(
        DoubleFunc::Bifurcate,
        Box::new(Func::Bound(vec![])),
        Box::new(Func::Bound(vec![])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
#[test]
fn double_half_end() {
    let program = "bh";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Double(
        DoubleFunc::Bifurcate,
        Box::new(Func::Basic(BasicFunc::Head)),
        Box::new(Func::Bound(vec![])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}

#[test]
fn double_half_quote() {
    let program = "bhqhhq";
    let funcs = parse(lex(program));
    let desired_funcs = vec![Func::Double(
        DoubleFunc::Bifurcate,
        Box::new(Func::Basic(BasicFunc::Head)),
        Box::new(Func::Bound(vec![
            Func::Basic(BasicFunc::Head),
            Func::Basic(BasicFunc::Head),
        ])),
    )];
    assert_eq!(funcs, Func::Bound(desired_funcs));
}
