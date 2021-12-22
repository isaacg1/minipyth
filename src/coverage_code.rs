use crate::test_helpers::*;
use crate::Object;
use crate::Object::*;

// The goal of this module is coverage of all nontrivial behavior of the execute functions

#[test]
fn inverse_map() {
    let program = "imh";
    let input = int_to_obj(10);
    let desired_output = list_int_to_obj(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8]);
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
    let program = "cmm";
    let input = int_to_obj(5);
    let desired_output = lli_to_obj(vec![vec![0, 0, 0, 0], vec![1, 1, 1], vec![2, 2], vec![3]]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn transpose_mixed() {
    let program = "cxm";
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

#[test]
fn obj_nested_first() {
    let input = "[[1, 2, 3, 4], [2, 3, 4, 1]]";
    let object = Object::from_str(input);
    let output = format!("{}", object);
    assert_eq!(input, &output);
}

#[test]
fn sum_mixed() {
    let program = "sxm";
    let input = int_to_obj(5);
    let desired_output = list_int_to_obj(vec![5, 0, 1, 2, 3, 4]);
    let output = run_prog(program, input);
    assert_eq!(desired_output, output);
}

#[test]
fn divide_by_zero() {
    let program = "ipm";
    let input = int_to_obj(-2);
    let output = run_prog(program, input);
    assert!(matches!(output, Error(_)));
}

#[test]
fn repeat_list() {
    let program = "rybtzzm";
    let input = int_to_obj(2);
    let output = run_prog(program, input);
    let desired_output = List(vec![
        list_int_to_obj(vec![0, 1]),
        lli_to_obj(vec![vec![], vec![0], vec![1], vec![0, 1]]),
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn inverse_double() {
    let program = "irmh";
    let input = int_to_obj(3);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![vec![-1, 0, 1], vec![-2, -1, 0], vec![-3, -2, -1]]);
    assert_eq!(desired_output, output);
}

#[test]
fn order_error() {
    let program = "ozrtmqbh";
    let input = int_to_obj(2);
    let output = run_prog(program, input);
    match output {
        List(list) => {
            assert_eq!(3, list.len());
            assert_eq!(list_int_to_obj(vec![]), list[0]);
            assert_eq!(list_int_to_obj(vec![1]), list[1]);
            assert!(matches!(list[2], Error(_)));
        }
        _ => panic!("{:?}", output),
    }
}

#[test]
fn repeat_empty() {
    let program = "rtm";
    let input = int_to_obj(0);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![vec![]]);
    assert_eq!(desired_output, output);
}

#[test]
fn repeat_one() {
    let program = "rhmhhhz";
    let input = int_to_obj(1);
    let output = run_prog(program, input);
    let desired_output = list_int_to_obj(vec![4, 5, 6]);
    assert_eq!(desired_output, output);
}

#[test]
fn inverse_while() {
    let program = "iwhh";
    let input = int_to_obj(5);
    let output = run_prog(program, input);
    let desired_output = list_int_to_obj(vec![5, 4, 3, 2, 1]);
    assert_eq!(desired_output, output);
}

#[test]
fn equal_empty() {
    let program = "mextm";
    let input = int_to_obj(5);
    let output = run_prog(program, input);
    let desired_output = list_int_to_obj(vec![0, 0, 0, 0, 1, 1]);
    assert_eq!(desired_output, output);
}

#[test]
fn inverse_order() {
    let program = "ios";
    let input = int_to_obj(5);
    let output = run_prog(program, input);
    let desired_output = list_int_to_obj(vec![4, 0, 1, 2, 3]);
    assert_eq!(desired_output, output);
}

#[test]
fn prime_factors() {
    let program = "mp";
    let input = list_int_to_obj(vec![0, 12, 25]);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![vec![], vec![2, 2, 3], vec![5, 5]]);
    assert_eq!(desired_output, output);
}

#[test]
fn combine_error() {
    let program = "cist";
    let input = list_int_to_obj(vec![]);
    let output = run_prog(program, input);
    assert!(matches!(output, Error(_)));
}

#[test]
fn to_binary() {
    let program = "mltz";
    let input = int_to_obj(6);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![
        vec![1],
        vec![0],
        vec![1],
        vec![1, 0],
        vec![1, 1],
        vec![1, 0, 0],
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn powerset_error() {
    let program = "y";
    let input = int_to_obj(-1);
    let output = run_prog(program, input);
    assert!(matches!(output, Error(_)));
}

#[test]
fn all_pairs() {
    let program = "abzmh";
    let input = int_to_obj(5);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![
        vec![5, 1],
        vec![5, 2],
        vec![5, 3],
        vec![5, 4],
        vec![5, 5],
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn all_pairs_second() {
    let program = "abmh";
    let input = int_to_obj(5);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![
        vec![1, 5],
        vec![2, 5],
        vec![3, 5],
        vec![4, 5],
        vec![5, 5],
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn all_pairs_multi() {
    let program = "amm";
    let input = int_to_obj(3);
    let output = run_prog(program, input);
    let desired_output = List(vec![
        List(vec![List(vec![list_int_to_obj(vec![]), int_to_obj(0)])]),
        List(vec![
            List(vec![list_int_to_obj(vec![]), int_to_obj(0)]),
            List(vec![list_int_to_obj(vec![]), int_to_obj(1)]),
        ]),
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn all_pairs_multi_int() {
    let program = "asbzmm";
    let input = int_to_obj(3);
    let output = run_prog(program, input);
    let desired_output = List(vec![
        list_int_to_obj(vec![]),
        lli_to_obj(vec![vec![3, 0]]),
        lli_to_obj(vec![vec![3, 0], vec![3, 1]]),
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn all_pairs_multi_second() {
    let program = "asbbm";
    let input = int_to_obj(3);
    let output = run_prog(program, input);
    let desired_output = List(vec![
        lli_to_obj(vec![vec![0, 3], vec![1, 3], vec![2, 3]]),
        lli_to_obj(vec![vec![0, 3], vec![1, 3], vec![2, 3]]),
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn all_pairs_self() {
    let program = "am";
    let input = int_to_obj(2);
    let output = run_prog(program, input);
    let desired_output = List(vec![
        List(vec![list_int_to_obj(vec![0, 1]), int_to_obj(0)]),
        List(vec![list_int_to_obj(vec![0, 1]), int_to_obj(1)]),
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn all_pairs_int() {
    let program = "a";
    let input = int_to_obj(4);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3]]);
    assert_eq!(desired_output, output);
}

#[test]
fn from_binary() {
    let program = "ilm";
    let input = int_to_obj(11);
    let output = run_prog(program, input);
    let desired_output = int_to_obj(2036);
    assert_eq!(desired_output, output);
}

#[test]
fn cartesian_product() {
    let program = "pbmhm";
    let input = int_to_obj(2);
    let output = run_prog(program, input);
    let desired_output = lli_to_obj(vec![vec![1, 0], vec![1, 1], vec![2, 0], vec![2, 1]]);
    assert_eq!(desired_output, output);
}
