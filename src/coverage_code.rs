use crate::Object::*;
use crate::{lex, parse, Object};
use num_bigint::ToBigInt;

// The goal of this module is coverage of all nontrivial behavior of the execute functions

fn run_prog(program: &str, input: Object) -> Object {
    let tokens = lex(program);
    let func = parse(tokens);
    func.execute(input)
}

fn int_to_obj(int: i64) -> Object {
    Int(int.to_bigint().unwrap())
}

fn list_int_to_obj(ints: Vec<i64>) -> Object {
    List(ints.into_iter().map(int_to_obj).collect())
}

fn lli_to_obj(intss: Vec<Vec<i64>>) -> Object {
    List(intss.into_iter().map(list_int_to_obj).collect())
}

#[test]
fn inverse_map() {
    let program = "imh";
    let input = int_to_obj(10);
    let desired_output = list_int_to_obj(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn inverse_bifurcate() {
    let program = "mibhy";
    let input = int_to_obj(3);
    let desired_output = list_int_to_obj(vec![1, 1, 0]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn order() {
    let program = "oihxttzmq";
    let input = int_to_obj(10);
    let desired_output = list_int_to_obj(vec![0, 2, 4, 6, 8, 1, 3, 5, 7, 9]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn head_empty() {
    let program = "fhmmh";
    let input = int_to_obj(3);
    let desired_output = lli_to_obj(vec![vec![1], vec![1, 2]]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn sum_list_of_lists() {
    let program = "smm";
    let input = int_to_obj(4);
    let desired_output = list_int_to_obj(vec![0, 0, 1, 0, 1, 2]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn inverse_tail() {
    let program = "xitm";
    let input = int_to_obj(4);
    let desired_output = lli_to_obj(vec![
        vec![0, 1, 2, 3],
        vec![0, 1, 2],
        vec![0, 1],
        vec![0],
        vec![],
    ]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn inverse_error_prop() {
    let program = "ittzm";
    let input = int_to_obj(0);
    let output = run_prog(program, input);
    assert!(matches!(output, Error(_)));
}

#[test]
fn negative_range() {
    let program = "mzn";
    let input = int_to_obj(5);
    let desired_output = list_int_to_obj(vec![4, 3, 2, 1, 0]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn map_error() {
    let program = "mtmm";
    let input = int_to_obj(4);
    let output = run_prog(program, input);
    assert!(matches!(output, Error(_)));
}

#[test]
fn inverse_filter() {
    let program = "ifhm";
    let input = int_to_obj(3);
    let desired_output = list_int_to_obj(vec![0, 2]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn double_inverse() {
    let program = "iih";
    let input = int_to_obj(1);
    let desired_output = int_to_obj(2);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn while_arg_error() {
    let program = "wytm";
    let input = int_to_obj(4);
    let desired_output = lli_to_obj(vec![
        vec![0, 1, 2, 3],
        vec![1, 2, 3],
        vec![2, 3],
        vec![3],
        vec![],
    ]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn bifurcate_second_error() {
    let program = "bltm";
    let input = int_to_obj(0);
    let output = run_prog(program, input);
    assert!(matches!(output, Error(_)));
}

#[test]
fn transpose() {
    let program = "pmm";
    let input = int_to_obj(5);
    let desired_output = lli_to_obj(vec![vec![0, 0, 0, 0], vec![1, 1, 1], vec![2, 2], vec![3]]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn transpose_mixed() {
    let program = "pxm";
    let input = int_to_obj(5);
    let desired_output = lli_to_obj(vec![vec![5, 0], vec![1], vec![2], vec![3], vec![4]]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn obj_roundtrip() {
    let input = "[1, 2, [-1, 0, 2], 91, -312370917097070709709620963505826096106016061]";
    let object = Object::from_str(input);
    let output = format!("{}", object);
    assert_eq!(input, &output);
}

#[test]
fn obj_error() {
    let program = "tm";
    let input = int_to_obj(0);
    let object = run_prog(program, input);
    let output = format!("{}", object);
    let desired_output = "Error: Tail of empty list";
    assert_eq!(desired_output, output);
}
