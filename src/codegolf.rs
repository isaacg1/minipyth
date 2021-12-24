use crate::test_helpers::*;
use crate::Object::*;
use crate::{lex, parse};

#[test]
fn make_2014() {
    // https://codegolf.stackexchange.com/questions/17005
    let program = "ttsmzyhhyhh";
    let output = run_prog(program, int_to_obj(0));
    assert_eq!(int_to_obj(2014), output);
}

#[test]
fn primality() {
    // https://codegolf.stackexchange.com/questions/57617
    let programs = vec![
        "stlfsmqiphzbihlqtnwttmh",
        "sttlfsiphzbihlqxtmh",
        "stlfspipbihlqtxtmh",
        "iphzbpmptmbq",
        "ip",
    ];
    for program in programs {
        let func = parse(lex(program));
        for i in 1..30 {
            let output = func.execute(int_to_obj(i));
            let is_prime = (2..i).all(|div| i % div != 0) && i > 1;
            let desired_output = int_to_obj(if is_prime { 1 } else { 0 });
            assert_eq!(desired_output, output, "Input: {}", i);
        }
    }
}

#[test]
fn fibonacci() {
    // https://codegolf.stackexchange.com/questions/85
    let programs = vec![
        "ihhhzxbthzqbshihqbzbhhzhm",
        "ihhzrbshbzbhhzhm",
        "ihsrbshnbms",
    ];
    for program in programs {
        let func = parse(lex(program));
        let mut fib_pair = (0, 1);
        for i in 1..10 {
            let output = func.execute(int_to_obj(i));
            let desired_output = int_to_obj(fib_pair.1);
            assert_eq!(desired_output, output, "Input: {}, Program: {}", i, program);
            fib_pair = (fib_pair.1, fib_pair.0 + fib_pair.1);
        }
    }
}

#[test]
fn nesting() {
    // https://codegolf.stackexchange.com/questions/239867
    let program = "htnrbhqbht";
    let output = run_prog(program, list_int_to_obj(vec![5, -1]));
    let n1 = int_to_obj(-1);
    let desired_output = List(vec![
        n1.clone(),
        List(vec![
            n1.clone(),
            List(vec![
                n1.clone(),
                List(vec![n1.clone(), List(vec![n1.clone()])]),
            ]),
        ]),
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn reshaped() {
    // https://codegolf.stackexchange.com/questions/239788
    let program = "ttfecbxitxt";
    let output = run_prog(program, list_int_to_obj(vec![0, 1, 2, 3, 4, 0, 1, 2]));
    let desired_output = List(vec![lli_to_obj(vec![vec![], vec![]])]);
    assert_eq!(desired_output, output);
}

#[test]
fn anagram() {
    // https://codegolf.stackexchange.com/questions/1294
    let program = "emo";
    let output = run_prog(
        program,
        lli_to_obj(vec![vec![0, 1, 2, 3], vec![2, 3, 1, 0]]),
    );
    let desired_output = int_to_obj(1);
    assert_eq!(desired_output, output);
}

#[test]
fn sqrt() {
    // OEIS: A196
    let programs = vec!["smeboqcbmqpbhhqr", "lfeboqabmqpbhhq"];
    for program in programs {
        let func = parse(lex(program));
        for i in 0..=10 {
            let input = int_to_obj(i);
            let output = func.execute(input);
            let desired_output = int_to_obj((i as f64).sqrt() as i64);
            assert_eq!(desired_output, output, "Input: {}, Program: {}", i, program);
        }
    }
}

#[test]
fn group_by_displacement() {
    // https://codegolf.stackexchange.com/questions/239999
    let program = "mmihgqhhonsqcbmnhzlz";
    let output = run_prog(program, list_int_to_obj(vec![5, 6, 8, 3, 9, 4, 2, 1, 7]));
    let desired_output = lli_to_obj(vec![
        vec![3],
        vec![4, 7],
        vec![5, 6, 9],
        vec![8, 2],
        vec![1],
    ]);
    assert_eq!(desired_output, output);
}

#[test]
fn even_subarrays() {
    let program = "smsihlsqfzsmxitxt";
    let input = list_int_to_obj(vec![0, 6, 6, 5, 3, 3, 8]);
    let output = run_prog(program, input);
    let desired_output = int_to_obj(13);
    assert_eq!(desired_output, output);
}
