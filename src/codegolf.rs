use crate::Object::*;
use crate::{lex, parse, Object};

fn run_prog(program: &str, input: Object) -> Object {
    let tokens = lex(program);
    let func = parse(tokens);
    func.execute(input)
}

#[test]
fn make_2014() {
    // https://codegolf.stackexchange.com/questions/17005
    let program = "ttsmzpbzyyyh";
    let output = run_prog(program, Int(0));
    assert_eq!(output, Int(2014));
}

#[test]
fn primality() {
    // https://codegolf.stackexchange.com/questions/57617

    // Wilson's theorem: ibhiphznbqptoqz
    // Fails with overflow on i = 22
    // Return once implement bigint

    let program = "stlfsmqiphzbihlqtnwttmh";
    let func = parse(lex(program));
    for i in 1..256 {
        let output = func.execute(Int(i));
        let is_prime = (2..i).all(|div| i % div != 0) && i > 1;
        let desired_output = Int(if is_prime { 1 } else { 0 });
        assert_eq!(desired_output, output, "Input: {}", i);
    }
}
