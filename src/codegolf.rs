use crate::Object::*;
use crate::{lex, parse, Object};
use num_bigint::ToBigInt;

fn run_prog(program: &str, input: Object) -> Object {
    let tokens = lex(program);
    let func = parse(tokens);
    func.execute(input)
}

fn int_to_obj(int: i64) -> Object {
    Int(int.to_bigint().unwrap())
}

#[test]
fn make_2014() {
    // https://codegolf.stackexchange.com/questions/17005
    let program = "ttsmzpbzyyyh";
    let output = run_prog(program, int_to_obj(0));
    assert_eq!(int_to_obj(2014), output);
}

#[test]
fn primality() {
    // https://codegolf.stackexchange.com/questions/57617

    let programs = vec![
        "stlfsmqiphzbihlqtnwttmh",
        "sttlfsiphzbihlqxtmh",
        "ibstthzihipbhptmzz",
        "stlfspipbihlqtxtmh",
        "iphzbpmptmbq",
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

    let program = "ihhhzxbthzqbshihqbzbhhzhm";
    let func = parse(lex(program));
    let mut fib_pair = (0, 1);
    for i in 1..10 {
        let output = func.execute(int_to_obj(i));
        let desired_output = int_to_obj(fib_pair.1);
        assert_eq!(desired_output, output, "Input: {}", i);
        fib_pair = (fib_pair.1, fib_pair.0 + fib_pair.1);
    }
}
