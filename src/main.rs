use clap::{App, Arg};
use serde::{Deserialize, Serialize};

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Func {
    Basic(BasicFunc),
    Higher(HigherFunc, Box<Func>),
    Double(DoubleFunc, Box<Func>, Box<Func>),
    Bound(Vec<Func>),
}
impl Func {
    fn execute(&self, arg: Object) -> Object {
        use Func::*;
        match self {
            Basic(basic) => basic.execute(arg),
            Higher(higher_func, func) => higher_func.execute(func, arg),
            Double(double_func, func1, func2) => double_func.execute(func1, func2, arg),
            Bound(funcs) => {
                let mut working_obj = arg;
                for func in funcs.iter().rev() {
                    working_obj = func.execute(working_obj);
                }
                working_obj
            }
        }
    }
    fn inverse_execute(&self, arg: Object) -> Object {
        use Func::*;
        match self {
            Basic(basic) => basic.inverse_execute(arg),
            Higher(higher_func, func) => higher_func.inverse_execute(func, arg),
            Double(double_func, func1, func2) => double_func.inverse_execute(func1, func2, arg),
            Bound(funcs) => {
                let mut working_obj = arg;
                for func in funcs {
                    working_obj = func.inverse_execute(working_obj);
                }
                working_obj
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
// Todo: Bigint
enum Object {
    Int(i64),
    List(Vec<Object>),
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct SortKey(bool, i64, Vec<SortKey>);

impl Object {
    fn to_key(&self) -> SortKey {
        use Object::*;
        match self {
            Int(i) => SortKey(false, *i, vec![]),
            List(l) => SortKey(true, 0, l.iter().map(|obj| obj.to_key()).collect()),
        }
    }
    fn is_truthy(&self) -> bool {
        use Object::*;
        match self {
            Int(i) => *i != 0,
            List(l) => !l.is_empty(),
        }
    }
}

#[derive(Debug)]
enum Token {
    Basic(BasicFunc),
    Higher(HigherFunc),
    Double(DoubleFunc),
    Bound(BoundToken),
}

#[derive(Debug, PartialEq, Eq)]
enum BasicFunc {
    Head,
    Tail,
    Sum,
    Product,
    PowerSet,
    Length,
    Negate,
}

impl BasicFunc {
    fn execute(&self, arg: Object) -> Object {
        use BasicFunc::*;
        use Object::*;
        match (self, arg) {
            (Head, Int(i)) => Int(i + 1),
            (Head, List(mut l)) => l.remove(0),
            (Tail, Int(i)) => Int(i - 1),
            (Tail, List(mut l)) => {
                l.remove(0);
                List(l)
            }
            (Sum, Int(i)) => if i == 0 { Int(1) } else { Int(0)},
            (Sum, List(l)) => {
                if l.iter().all(|elem| matches!(elem, Int(_))) {
                    let total = l
                        .iter()
                        .map(|elem| if let Int(i) = elem { i } else { unreachable!() })
                        .sum();
                    Int(total)
                } else {
                    let mut output = vec![];
                    for elem in l {
                        match elem {
                            Int(_) => output.push(elem),
                            List(l) => output.extend(l),
                        }
                    }
                    List(output)
                }
            }
            (Product, List(l)) => {
                if l.iter().all(|elem| matches!(elem, Int(_))) {
                    let total = l
                        .iter()
                        .map(|elem| if let Int(i) = elem { i } else { unreachable!() })
                        .product();
                    Int(total)
                } else {
                    panic!("Product of list of list not implemented: {:?}", List(l));
                }
            }
            (PowerSet, Int(i)) => {
                if i < 0 {
                    panic!("Negative exponent in power set not implemented: {}", i);
                }
                Int(2i64.pow(i as u32))
            }
            (Length, List(l)) => Int(l.len() as i64),
            (Negate, Int(i)) => Int(-i),
            (Negate, List(mut l)) => {
                l.reverse();
                List(l)
            }
            (s, a) => panic!("Basic func unimplemented: {:?}, {:?}", s, a),
        }
    }
    fn inverse_execute(&self, arg: Object) -> Object {
        use BasicFunc::*;
        use Object::*;
        match (self, arg) {
            (Head, Int(i)) => Int(i - 1),
            (Head, List(mut l)) => l.remove(l.len() - 1),
            (Tail, List(mut l)) => {
                l.pop();
                List(l)
            }
            (Product, List(l)) if l.len() == 2 => {
                if let Int(num) = l[0] {
                    if let Int(den) = l[1] {
                        return List(vec![Int(num / den), Int(num % den)]);
                    }
                }
                panic!("Unimplemented inverse product: {:?} {:?}", self, List(l));
            }
            (s, a) => panic!("Basic inverse func unimplemented: {:?}, {:?}", s, a),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HigherFunc {
    Map,
    Filter,
    Order,
    FixedPoint,
    Inverse,
}
impl HigherFunc {
    fn to_list(arg: Object) -> Vec<Object> {
        use Object::*;
        match arg {
            Int(i) if i < 0 => (0..-i).rev().map(Int).collect(),
            Int(i) => (0..i).map(Int).collect(),
            List(l) => l,
        }
    }
    fn execute(&self, func: &Func, arg: Object) -> Object {
        use HigherFunc::*;
        use Object::*;
        match self {
            Map => {
                let list = HigherFunc::to_list(arg);
                List(list.into_iter().map(|obj| func.execute(obj)).collect())
            }
            Filter => {
                let mut list = HigherFunc::to_list(arg);
                list.retain(|obj| func.execute(obj.clone()).is_truthy());
                List(list)
            }
            Order => {
                let mut list = HigherFunc::to_list(arg);
                list.sort_by_key(|obj| {
                    let new_obj = func.execute(obj.clone());
                    new_obj.to_key()
                });
                List(list)
            }
            FixedPoint => {
                let mut seen = HashSet::new();
                let mut sequence = vec![];
                let mut current = arg;
                while !seen.contains(&current) {
                    seen.insert(current.clone());
                    sequence.push(current.clone());
                    current = func.execute(current);
                }
                List(sequence)
            }
            Inverse => func.inverse_execute(arg),
        }
    }
    fn inverse_execute(&self, func: &Func, arg: Object) -> Object {
        use HigherFunc::*;
        use Object::*;
        match self {
            Map => {
                let list = HigherFunc::to_list(arg);
                List(
                    list.into_iter()
                        .map(|obj| func.inverse_execute(obj))
                        .collect(),
                )
            }
            Filter => {
                let mut list = HigherFunc::to_list(arg);
                list.retain(|obj| func.inverse_execute(obj.clone()).is_truthy());
                List(list)
            }
            // Order: Inverse permutation, original func
            Inverse => func.execute(arg),
            s => panic!(
                "Higher inverse func unimplemented: {:?}, {:?}, {:?}",
                s, func, arg
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DoubleFunc {
    While,
    Bifurcate,
}

impl DoubleFunc {
    fn execute(&self, func1: &Func, func2: &Func, arg: Object) -> Object {
        use DoubleFunc::*;
        use Object::*;
        match self {
            While => {
                let mut working_arg = arg.clone();
                let mut sequence = vec![];
                loop {
                    sequence.push(working_arg.clone());
                    let test = func1.execute(working_arg.clone());
                    if !test.is_truthy() {
                        break;
                    }
                    working_arg = func2.execute(working_arg);
                }
                List(sequence)
            }
            Bifurcate => {
                let ret1 = func1.execute(arg.clone());
                let ret2 = func2.execute(arg);
                List(vec![ret1, ret2])
            }
        }
    }
    fn inverse_execute(&self, func1: &Func, func2: &Func, arg: Object) -> Object {
        use DoubleFunc::*;
        use Object::*;
        match self {
            // Bifurcate: Invariant
            Bifurcate => {
                let res1 = func1.execute(arg.clone());
                let res2 = func2.execute(arg);
                if res1 == res2 {
                    Int(1)
                } else {
                    Int(0)
                }
            }
            While => panic!(
                "Double func inverse unimplemented: {:?} {:?} {:?} {:?}",
                self, func1, func2, arg
            ),
        }
    }
}

#[derive(Debug)]
enum BoundToken {
    Bound1,
    BoundQuote,
}
#[derive(Debug)]
enum HOF {
    Higher(HigherFunc),
    Double(DoubleFunc),
    DoubleHalf(DoubleFunc, Func),
    Func(Func),
    Quote,
}

fn parse(mut tokens: Vec<Token>) -> Func {
    // This is unsatisfying - should really be the first unbound.
    let num_quotes = tokens
        .iter()
        .filter(|elem| matches! {elem, Token::Bound(BoundToken::BoundQuote)})
        .count();
    if num_quotes % 2 == 1 {
        let maybe_index = tokens
            .iter()
            .position(|elem| matches! {elem,  Token::Higher(_)});
        if let Some(index) = maybe_index {
            tokens.insert(index + 1, Token::Bound(BoundToken::BoundQuote));
        } else {
            panic!("Odd quotes, no higher order funcs");
        }
    }
    let mut state: Vec<HOF> = vec![];
    for token in tokens {
        match token {
            Token::Basic(basic_func) => state.push(HOF::Func(Func::Basic(basic_func))),
            Token::Higher(higher_func) => state.push(HOF::Higher(higher_func)),
            Token::Double(double_func) => state.push(HOF::Double(double_func)),
            Token::Bound(BoundToken::Bound1) => {
                let mut rev_bind_group = vec![];
                loop {
                    let last = state.pop();
                    match last {
                        None => panic!("Bind reached front"),
                        Some(HOF::Higher(higher_func)) => {
                            rev_bind_group.reverse();
                            let bound_func = Func::Bound(rev_bind_group);
                            let new_func = Func::Higher(higher_func, Box::new(bound_func));
                            state.push(HOF::Func(new_func));
                            break;
                        }
                        Some(HOF::Double(double_func)) => {
                            rev_bind_group.reverse();
                            let bound_func = Func::Bound(rev_bind_group);
                            let new_token = HOF::DoubleHalf(double_func, bound_func);
                            state.push(new_token);
                            break;
                        }
                        Some(HOF::DoubleHalf(double_func, bound_func)) => {
                            rev_bind_group.reverse();
                            let bound_func2 = Func::Bound(rev_bind_group);
                            let new_func = Func::Double(
                                double_func,
                                Box::new(bound_func),
                                Box::new(bound_func2),
                            );
                            state.push(HOF::Func(new_func));
                            break;
                        }
                        Some(HOF::Func(func)) => rev_bind_group.push(func),
                        Some(HOF::Quote) => panic!("Bind reached quote"),
                    }
                }
            }
            Token::Bound(BoundToken::BoundQuote) => {
                let quote_count = state.iter().filter(|hof| matches!(hof, HOF::Quote)).count();
                assert!(quote_count <= 1);
                if quote_count == 0 {
                    state.push(HOF::Quote);
                } else {
                    let mut rev_bind_group = vec![];
                    loop {
                        let last = state.pop();
                        match last {
                            None => unreachable!("Didn't find paired quote"),
                            Some(HOF::Func(func)) => rev_bind_group.push(func),
                            Some(HOF::Quote) => {
                                rev_bind_group.reverse();
                                let bound_func = Func::Bound(rev_bind_group);
                                let last_state = state.pop();
                                match last_state {
                                    Some(HOF::Higher(higher_func)) => {
                                        let new_func =
                                            Func::Higher(higher_func, Box::new(bound_func));
                                        state.push(HOF::Func(new_func));
                                        break;
                                    }
                                    Some(HOF::Double(double_func)) => {
                                        state.push(HOF::DoubleHalf(double_func, bound_func));
                                        break;
                                    }
                                    Some(HOF::DoubleHalf(double_func, old_func)) => {
                                        let new_func = Func::Double(
                                            double_func,
                                            Box::new(old_func),
                                            Box::new(bound_func),
                                        );
                                        state.push(HOF::Func(new_func));
                                        break;
                                    }
                                    _ => panic!("Paired quote not before higher or double func"),
                                }
                            }
                            Some(HOF::Higher(higher_func)) => {
                                let prev_func = rev_bind_group.pop();
                                match prev_func {
                                    None => rev_bind_group.push(Func::Higher(
                                        higher_func,
                                        Box::new(Func::Bound(vec![])),
                                    )),
                                    Some(prev) => rev_bind_group
                                        .push(Func::Higher(higher_func, Box::new(prev))),
                                }
                            }
                            Some(HOF::Double(double_func)) => {
                                let prev_func = rev_bind_group.pop();
                                match prev_func {
                                    None => rev_bind_group.push(Func::Double(
                                        double_func,
                                        Box::new(Func::Bound(vec![])),
                                        Box::new(Func::Bound(vec![])),
                                    )),
                                    Some(prev) => state.push(HOF::DoubleHalf(double_func, prev)),
                                }
                            }
                            Some(HOF::DoubleHalf(double_func, old_func)) => {
                                let prev_func = rev_bind_group.pop();
                                match prev_func {
                                    None => rev_bind_group.push(Func::Double(
                                        double_func,
                                        Box::new(old_func),
                                        Box::new(Func::Bound(vec![])),
                                    )),
                                    Some(prev) => rev_bind_group.push(Func::Double(
                                        double_func,
                                        Box::new(old_func),
                                        Box::new(prev),
                                    )),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut funcs = vec![];
    enum HD {
        H(HigherFunc),
        D(DoubleFunc),
        D2(DoubleFunc, Func),
    }
    let mut open_higher: Vec<HD> = vec![];
    for hof in state {
        match hof {
            HOF::Func(func) => {
                let mut working_func = func;
                loop {
                    match open_higher.pop() {
                        Some(HD::H(higher_func)) => {
                            working_func = Func::Higher(higher_func, Box::new(working_func))
                        }
                        None => {
                            funcs.push(working_func);
                            break;
                        }
                        Some(HD::D(double_func)) => {
                            open_higher.push(HD::D2(double_func, working_func));
                            break;
                        }
                        Some(HD::D2(double_func, old_func)) => {
                            working_func = Func::Double(
                                double_func,
                                Box::new(old_func),
                                Box::new(working_func),
                            )
                        }
                    }
                }
            }
            HOF::Higher(higher_func) => open_higher.push(HD::H(higher_func)),
            HOF::Double(double_func) => open_higher.push(HD::D(double_func)),
            HOF::DoubleHalf(double_func, func) => open_higher.push(HD::D2(double_func, func)),
            HOF::Quote => unreachable!("All quotes paired at start of parse"),
        }
    }
    if !open_higher.is_empty() {
        let mut working_func = Func::Bound(vec![]);
        loop {
            working_func = match open_higher.pop() {
                Some(HD::H(higher_func)) => Func::Higher(higher_func, Box::new(working_func)),
                Some(HD::D(double_func)) => Func::Double(
                    double_func,
                    Box::new(working_func),
                    Box::new(Func::Bound(vec![])),
                ),
                Some(HD::D2(double_func, old_func)) => {
                    Func::Double(double_func, Box::new(old_func), Box::new(working_func))
                }
                None => break,
            };
        }
        funcs.push(working_func);
    }
    Func::Bound(funcs)
}
fn lex(code: &str) -> Vec<Token> {
    code.chars()
        .map(|c| match c {
            'b' => Token::Double(DoubleFunc::Bifurcate),
            'f' => Token::Higher(HigherFunc::Filter),
            'h' => Token::Basic(BasicFunc::Head),
            'i' => Token::Higher(HigherFunc::Inverse),
            'l' => Token::Basic(BasicFunc::Length),
            'm' => Token::Higher(HigherFunc::Map),
            'n' => Token::Basic(BasicFunc::Negate),
            'o' => Token::Higher(HigherFunc::Order),
            'p' => Token::Basic(BasicFunc::Product),
            'q' => Token::Bound(BoundToken::BoundQuote),
            's' => Token::Basic(BasicFunc::Sum),
            't' => Token::Basic(BasicFunc::Tail),
            'w' => Token::Double(DoubleFunc::While),
            'x' => Token::Higher(HigherFunc::FixedPoint),
            'y' => Token::Basic(BasicFunc::PowerSet),
            'z' => Token::Bound(BoundToken::Bound1),
            _ => unimplemented!(),
        })
        .collect()
}

fn run(program: &str, maybe_input: Option<&str>, debug: bool) -> String {
    let tokens = lex(program);
    let func = parse(tokens);
    if debug {
        println!("{:#?}", func);
    }
    let input = maybe_input.unwrap_or("0");
    let parsed_input: Object = ron::from_str(input).expect("Invalid input");
    let output = func.execute(parsed_input);
    ron::to_string(&output).expect("Output serialized")
}

fn main() {
    let matches = App::new("Minipyth")
        .version("1.0")
        .author("Isaac Grosof")
        .about("Implements the Minipyth programming language")
        .arg(
            Arg::with_name("PROGRAM")
                .help("The program to run")
                .required(true),
        )
        .arg(Arg::with_name("INPUT").help("The input to provide"))
        .arg(
            Arg::with_name("DEBUG")
                .short("d")
                .long("debug")
                .help("Prints parse tree"),
        )
        .get_matches();
    let program = matches.value_of("PROGRAM").unwrap();
    let debug = matches.is_present("DEBUG");
    let input = matches.value_of("INPUT");
    let result = run(program, input, debug);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::*;
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
}

#[cfg(test)]
mod codegolf;
