use clap::{App, Arg};
use num_bigint::{BigInt, ToBigInt};
use num_traits::cast::ToPrimitive;
use num_traits::{One, Signed, Zero};

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Object {
    Int(BigInt),
    List(Vec<Object>),
    Error(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Object::*;
        match self {
            Int(i) => write!(f, "{}", i),
            List(l) => {
                write!(f, "[")?;
                for (index, elem) in l.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?
                    }
                    write!(f, "{}", elem)?
                }
                write!(f, "]")
            }
            Error(e) => write!(f, "Error: {}", e),
        }
    }
}

impl Object {
    fn from_str(string: &str) -> Object {
        use Object::*;
        if string.is_empty() {
            return List(vec![]);
        }
        if !string.contains('[') && !string.contains(',') {
            let integer = string.parse().expect("Nonlist should be int");
            return Int(integer);
        }
        let sub_string = if string.chars().nth(0).expect("Nonempty") == '[' {
            assert!(
                string.chars().rev().nth(0).expect("Nonempty") == ']',
                "Object string should have matched brackets: {:?}",
                string
            );
            &string[1..string.len() - 1]
        } else {
            string
        };
        if !sub_string.contains('[') {
            let mut sub_vec = vec![];
            for element_string in sub_string.split(',') {
                let trimmed = element_string.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let sub_elem = Object::from_str(trimmed);
                sub_vec.push(sub_elem);
            }
            List(sub_vec)
        } else {
            let mut cursor = 0;
            let mut sub_vec = vec![];
            loop {
                let next_bracket = sub_string
                    .chars()
                    .enumerate()
                    .skip(cursor)
                    .filter(|(_, c)| *c == '[')
                    .next();
                if let Some((bracket_index, _)) = next_bracket {
                    let inner = sub_string[cursor..bracket_index].trim();
                    let inner_obj = Object::from_str(inner);
                    if let List(list) = inner_obj {
                        sub_vec.extend(list);
                    } else {
                        panic!("Inner is list: {:?}", inner_obj)
                    }
                    cursor = bracket_index;
                } else {
                    let inner = sub_string[cursor..].trim();
                    let inner_obj = Object::from_str(inner);
                    if let List(list) = inner_obj {
                        sub_vec.extend(list);
                    } else {
                        panic!("Inner is list: {:?}", inner_obj)
                    }
                    break;
                }
                let next_close = sub_string
                    .chars()
                    .enumerate()
                    .skip(cursor)
                    .filter(|(_, c)| *c == ']')
                    .next()
                    .expect("Open has close")
                    .0;
                let inner = sub_string[cursor..=next_close].trim();
                let inner_obj = Object::from_str(inner);
                sub_vec.push(inner_obj);
                cursor = next_close + 1;
            }
            List(sub_vec)
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct SortKey(bool, BigInt, Vec<SortKey>);

impl Object {
    fn to_key(&self) -> SortKey {
        use Object::*;
        match self {
            Int(i) => SortKey(false, i.clone(), vec![]),
            List(l) => SortKey(
                true,
                Zero::zero(),
                l.iter().map(|obj| obj.to_key()).collect(),
            ),
            Error(_) => SortKey(true, One::one(), vec![]),
        }
    }
    fn is_truthy(&self) -> bool {
        use Object::*;
        match self {
            Int(i) => *i != Zero::zero(),
            List(l) => !l.is_empty(),
            Error(_) => false,
        }
    }
    fn to_list(self) -> Vec<Object> {
        use Object::*;
        match self {
            Int(i) if i < Zero::zero() => {
                let mut nums = vec![];
                let mut j: BigInt = Zero::zero();
                let target = -i;
                loop {
                    if &j == &target {
                        break;
                    }
                    nums.push(Int(j.clone()));
                    j += 1;
                }
                nums.reverse();
                nums
            }
            Int(i) => {
                let mut nums = vec![];
                let mut j: BigInt = Zero::zero();
                loop {
                    if &j == &i {
                        break;
                    }
                    nums.push(Int(j.clone()));
                    j += 1;
                }
                nums
            }
            List(l) => l,
            a @ Error(_) => panic!("to_list called on {:?}", a),
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum BasicFunc {
    Head,
    Tail,
    Sum,
    Product,
    PowerSet,
    Length,
    Negate,
    Equal,
    Combine,
    AllPair,
}

impl BasicFunc {
    fn execute(&self, arg: Object) -> Object {
        use BasicFunc::*;
        use Object::*;
        match (self, arg) {
            (Head, Int(i)) => Int(i + 1),
            (Head, List(mut l)) => {
                if l.is_empty() {
                    Error("Head of empty list".to_string())
                } else {
                    l.remove(0)
                }
            }
            (Tail, Int(i)) => Int(i - 1),
            (Tail, List(mut l)) => {
                if l.is_empty() {
                    Error("Tail of empty list".to_string())
                } else {
                    l.remove(0);
                    List(l)
                }
            }
            (Sum, Int(i)) => {
                if i == Zero::zero() {
                    Int(One::one())
                } else {
                    Int(Zero::zero())
                }
            }
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
                            Int(_) | Error(_) => output.push(elem),
                            List(l) => output.extend(l),
                        }
                    }
                    List(output)
                }
            }
            (Product, Int(i)) => {
                let a = i.abs();
                if a < 2.to_bigint().unwrap() {
                    List(vec![])
                } else {
                    let mut factors = vec![];
                    let mut j = 2.to_bigint().unwrap();
                    let mut work = a;
                    while &j * &j <= work {
                        if &work % &j == Zero::zero() {
                            work /= &j;
                            factors.push(j.to_bigint().unwrap());
                        } else {
                            j += 1;
                        }
                    }
                    if work > One::one() {
                        factors.push(work)
                    }
                    List(factors.into_iter().map(Int).collect())
                }
            }
            (Product, List(l)) => {
                if l.iter().all(|elem| matches!(elem, Int(_))) {
                    let total = l
                        .iter()
                        .map(|elem| if let Int(i) = elem { i } else { unreachable!() })
                        .product();
                    Int(total)
                } else if l.iter().any(|elem| matches!(elem, Error(_))) {
                    panic!("Product has error in list: {:?}", l);
                } else {
                    let list_of_lists: Vec<Vec<Object>> =
                        l.into_iter().map(|elem| elem.to_list()).collect();
                    let mut staged_lists = vec![vec![]];
                    for sub_list in list_of_lists {
                        let mut next_stage = vec![];
                        for old_list in staged_lists {
                            for elem in sub_list.clone() {
                                let mut new_list = old_list.clone();
                                new_list.push(elem.clone());
                                next_stage.push(new_list);
                            }
                        }
                        staged_lists = next_stage;
                    }
                    List(staged_lists.into_iter().map(List).collect())
                }
            }
            (Combine, List(l)) => {
                if let Some(first_error) = l.iter().filter(|elem| matches!(elem, Error(_))).next() {
                    first_error.clone()
                } else {
                    let longest = l
                        .iter()
                        .map(|elem| match elem {
                            Int(_) => 1,
                            List(inner) => inner.len(),
                            Error(_) => unreachable!("No errors"),
                        })
                        .max()
                        .expect("Empty -> 1");
                    let mut output = vec![];
                    for index in 0..longest {
                        let mut row = vec![];
                        for elem in &l {
                            let maybe_to_push = match elem {
                                a @ Int(_) => {
                                    if index == 0 {
                                        Some(a.clone())
                                    } else {
                                        None
                                    }
                                }
                                List(inner) => inner.get(index).cloned(),
                                Error(_) => unreachable!("No errors"),
                            };
                            if let Some(to_push) = maybe_to_push {
                                row.push(to_push)
                            };
                        }
                        output.push(List(row))
                    }
                    List(output)
                }
            }
            (PowerSet, Int(i)) => {
                if i < Zero::zero() {
                    // Rationals
                    Error("Negative exponent in power set".to_string())
                } else {
                    Int(2
                        .to_bigint()
                        .unwrap()
                        .pow(i.to_u64().expect("Exponent small") as u32))
                }
            }
            (PowerSet, List(l)) => {
                let num_subsets = 2u64.pow(l.len() as u32);
                let mut output = vec![];
                for i in 0..num_subsets {
                    let mut subset = vec![];
                    for (index, elem) in l.iter().enumerate() {
                        let mask = 1 << index;
                        if i & mask > 0 {
                            subset.push(elem.clone());
                        }
                    }
                    output.push(List(subset))
                }
                List(output)
            }
            (Length, List(l)) => Int(l.len().to_bigint().unwrap()),
            (Length, Int(i)) => {
                let (_sign, bits) = i.to_radix_be(2);
                List(
                    bits.iter()
                        .map(|&b| Int((b as i64).to_bigint().unwrap()))
                        .collect(),
                )
            }
            (Negate, Int(i)) => Int(-i),
            (Negate, List(mut l)) => {
                l.reverse();
                List(l)
            }
            (Equal, List(mut l)) => {
                if let Some(last) = l.pop() {
                    let same = l.iter().all(|elem| elem == &last);
                    if same {
                        Int(One::one())
                    } else {
                        Int(Zero::zero())
                    }
                } else {
                    Int(One::one())
                }
            }
            (AllPair, List(l)) => {
                if l.len() >= 2 && l.iter().skip(1).any(|elem| matches!(elem, List(_))) {
                    let (first, rest) = l.split_first().expect("Checked 2");
                    let rest_lists: Vec<Vec<Object>> = rest
                        .into_iter()
                        .map(|elem| elem.clone().to_list())
                        .collect();
                    let out: Vec<Object> = rest_lists
                        .into_iter()
                        .map(|list| {
                            let paired = list
                                .into_iter()
                                .map(|elem| List(vec![first.clone(), elem]))
                                .collect();
                            List(paired)
                        })
                        .collect();
                    if out.len() == 1 {
                        out[0].clone()
                    } else {
                        List(out)
                    }
                } else if l.len() >= 2 && matches!(l[0], List(_)) {
                    let mut rest = l.clone();
                    let second = rest.remove(1);
                    let rest_lists: Vec<Vec<Object>> = rest
                        .into_iter()
                        .map(|elem| elem.clone().to_list())
                        .collect();
                    let out: Vec<Object> = rest_lists
                        .into_iter()
                        .map(|list| {
                            let paired = list
                                .into_iter()
                                .map(|elem| List(vec![elem, second.clone()]))
                                .collect();
                            List(paired)
                        })
                        .collect();
                    if out.len() == 1 {
                        out[0].clone()
                    } else {
                        List(out)
                    }
                } else {
                    List(
                        l.iter()
                            .map(|elem| List(vec![List(l.clone()), elem.clone()]))
                            .collect(),
                    )
                }
            }
            (AllPair, arg @ Int(_)) => {
                let list = arg.clone().to_list();
                List(
                    list.into_iter()
                        .map(|elem| List(vec![arg.clone(), elem]))
                        .collect(),
                )
            }
            (_, a @ Error(_)) => a,
            (s, a) => panic!("Basic func unimplemented: {:?}, {:?}", s, a),
        }
    }
    fn inverse_execute(&self, arg: Object) -> Object {
        use BasicFunc::*;
        use Object::*;
        match (self, arg) {
            (Head, Int(i)) => Int(i - 1),
            (Head, List(mut l)) => {
                if l.is_empty() {
                    Error("End (inverse head) of empty list".to_string())
                } else {
                    l.remove(l.len() - 1)
                }
            }
            (Tail, List(mut l)) => {
                if l.is_empty() {
                    Error("Inverse tail of empty list".to_string())
                } else {
                    l.pop();
                    List(l)
                }
            }
            (Product, List(l)) if l.len() == 2 => {
                if let Int(num) = &l[0] {
                    if let Int(den) = &l[1] {
                        let zero: BigInt = Zero::zero();
                        if den == &zero {
                            return Error("Divide by zero".to_string());
                        } else {
                            return List(vec![Int(num / den), Int(num % den)]);
                        }
                    }
                }
                panic!("Unimplemented inverse product: {:?} {:?}", self, List(l));
            }
            (Product, Int(i)) => {
                if i <= One::one() {
                    Int(Zero::zero())
                } else {
                    let mut div = 2;
                    let mut is_prime = true;
                    while &(div * div).to_bigint().unwrap() <= &i {
                        if &i % div == Zero::zero() {
                            is_prime = false;
                            break;
                        }
                        div += 1;
                    }
                    if is_prime {
                        Int(One::one())
                    } else {
                        Int(Zero::zero())
                    }
                }
            }
            (Length, List(l)) => {
                if l.iter().all(|elem| matches!(elem, Int(_))) {
                    let mut total: BigInt = Zero::zero();
                    for bit in l {
                        if let Int(b) = bit {
                            total *= 2;
                            total += b
                        }
                    }
                    Int(total)
                } else {
                    panic!("Unimplemented inverse l: {:?} {:?}", self, List(l));
                }
            }
            (Sum, arg) => List(vec![arg]),
            (_, a @ Error(_)) => a,
            (s, a) => panic!("Basic inverse func unimplemented: {:?}, {:?}", s, a),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum HigherFunc {
    Map,
    Filter,
    Order,
    FixedPoint,
    Inverse,
    Repeat,
}
impl HigherFunc {
    fn first_error(mut arg: Vec<Object>) -> Object {
        let maybe_index = arg.iter().position(|elem| matches!(elem, Object::Error(_)));
        if let Some(index) = maybe_index {
            arg.remove(index)
        } else {
            Object::List(arg)
        }
    }
    fn execute(&self, func: &Func, arg: Object) -> Object {
        use HigherFunc::*;
        use Object::*;
        match self {
            Map => {
                let list = arg.to_list();
                let out_list = list.into_iter().map(|obj| func.execute(obj)).collect();
                HigherFunc::first_error(out_list)
            }
            Filter => {
                let mut list = arg.to_list();
                list.retain(|obj| func.execute(obj.clone()).is_truthy());
                List(list)
            }
            Order => {
                let mut list = arg.to_list();
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
                while !seen.contains(&current) && !matches!(current, Error(_)) {
                    seen.insert(current.clone());
                    sequence.push(current.clone());
                    current = func.execute(current);
                }
                List(sequence)
            }
            Inverse => func.inverse_execute(arg),
            Repeat => {
                let (times, start) = match arg {
                    List(mut l) => {
                        if l.is_empty() {
                            (List(l.clone()), List(l))
                        } else if l.len() == 1 {
                            (l[0].clone(), l[0].clone())
                        } else {
                            let first = l.remove(0);
                            let second = l.remove(0);
                            (first, second)
                        }
                    }
                    Int(_) | Error(_) => (arg.clone(), arg.clone()),
                };
                match times {
                    List(l) => {
                        let mut output = vec![start.clone()];
                        let mut current = start;
                        for _ in 0..l.len() {
                            current = func.execute(current);
                            output.push(current.clone());
                        }
                        List(output)
                    }
                    Int(i) => {
                        if i < Zero::zero() {
                            List(vec![])
                        } else {
                            let mut output = vec![];
                            let mut current = start;
                            let mut j: BigInt = Zero::zero();
                            while j < i {
                                current = func.execute(current);
                                output.push(current.clone());
                                j += 1;
                            }
                            List(output)
                        }
                    }
                    Error(_) => List(vec![]),
                }
            }
        }
    }
    fn inverse_execute(&self, func: &Func, arg: Object) -> Object {
        use HigherFunc::*;
        use Object::*;
        match self {
            Order => {
                let list = arg.to_list();
                let mut indices: Vec<usize> = (0..list.len()).collect();
                indices.sort_by_key(|&i| func.execute(list[i].clone()).to_key());
                let mut inverse_indices: Vec<Option<usize>> = vec![None; list.len()];
                for (index, &perm) in indices.iter().enumerate() {
                    inverse_indices[perm] = Some(index);
                }
                let reordered = inverse_indices
                    .iter()
                    .map(|i| list[i.unwrap()].clone())
                    .collect();
                List(reordered)
            }
            Inverse => func.execute(arg),
            _ => {
                let inv = Func::Higher(HigherFunc::Inverse, Box::new(func.clone()));
                self.execute(&inv, arg)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
                let mut working_arg = arg;
                let mut sequence = vec![];
                loop {
                    if matches!(working_arg, Error(_)) {
                        break;
                    }
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
                if matches! {ret1, Error(_)} {
                    ret1
                } else if matches! {ret2, Error(_)} {
                    ret2
                } else {
                    List(vec![ret1, ret2])
                }
            }
        }
    }
    fn inverse_execute(&self, func1: &Func, func2: &Func, arg: Object) -> Object {
        match self {
            _ => {
                let inv1 = Func::Higher(HigherFunc::Inverse, Box::new(func1.clone()));
                let inv2 = Func::Higher(HigherFunc::Inverse, Box::new(func2.clone()));
                self.execute(&inv1, &inv2, arg)
            }
        }
    }
}

#[derive(Debug)]
enum BoundToken {
    Bound1,
    BoundQuote,
    SoloQuote,
}
#[derive(Debug)]
enum HOF {
    Higher(HigherFunc),
    Double(DoubleFunc),
    DoubleHalf(DoubleFunc, Func),
    Func(Func),
    Quote,
}

fn parse(tokens: Vec<Token>) -> Func {
    let mut state: Vec<HOF> = vec![];
    for token in tokens {
        if let Token::Bound(BoundToken::SoloQuote) = &token {
            assert!(state.iter().all(|elem| !matches!(elem, HOF::Quote)));
            let maybe_first_unbound_index =
                state.iter().position(|elem| !matches!(elem, HOF::Func(_)));
            if let Some(first_unbound_index) = maybe_first_unbound_index {
                state.insert(first_unbound_index + 1, HOF::Quote)
            } else {
                panic!("SoloQuote has no preceeding unbound: {:?}", state);
            }
        }
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
            Token::Bound(BoundToken::BoundQuote | BoundToken::SoloQuote) => {
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
                                    Some(HOF::Func(func)) => {
                                        let second_last_state = state.pop();
                                        match second_last_state {
                                            Some(HOF::Double(double_func)) => {
                                                let new_func = Func::Double(
                                                    double_func,
                                                    Box::new(func),
                                                    Box::new(bound_func),
                                                );
                                                state.push(HOF::Func(new_func));
                                                break;
                                            }
                                            _ => panic!("Paired quote in front of func, not in legal position: {:?} {:?} {:?}", state, func, second_last_state),
                                        }
                                    }
                                    _ => panic!(
                                        "Paired quote not in legal position: {:?} {:?}",
                                        state, last_state
                                    ),
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
    let mut tokens: Vec<Token> = code
        .chars()
        .map(|c| match c {
            'a' => Token::Basic(BasicFunc::AllPair),
            'b' => Token::Double(DoubleFunc::Bifurcate),
            'c' => Token::Basic(BasicFunc::Combine),
            'e' => Token::Basic(BasicFunc::Equal),
            'f' => Token::Higher(HigherFunc::Filter),
            'h' => Token::Basic(BasicFunc::Head),
            'i' => Token::Higher(HigherFunc::Inverse),
            'l' => Token::Basic(BasicFunc::Length),
            'm' => Token::Higher(HigherFunc::Map),
            'n' => Token::Basic(BasicFunc::Negate),
            'o' => Token::Higher(HigherFunc::Order),
            'p' => Token::Basic(BasicFunc::Product),
            'q' => Token::Bound(BoundToken::BoundQuote),
            'r' => Token::Higher(HigherFunc::Repeat),
            's' => Token::Basic(BasicFunc::Sum),
            't' => Token::Basic(BasicFunc::Tail),
            'w' => Token::Double(DoubleFunc::While),
            'x' => Token::Higher(HigherFunc::FixedPoint),
            'y' => Token::Basic(BasicFunc::PowerSet),
            'z' => Token::Bound(BoundToken::Bound1),
            _ => unimplemented!("Lex {}", c),
        })
        .collect();
    let num_quote = tokens
        .iter()
        .filter(|elem| matches!(elem, Token::Bound(BoundToken::BoundQuote)))
        .count();
    if num_quote % 2 == 1 {
        let solo_index = tokens
            .iter()
            .position(|elem| matches!(elem, Token::Bound(BoundToken::BoundQuote)))
            .expect("Odd means at least one");
        tokens[solo_index] = Token::Bound(BoundToken::SoloQuote);
    }
    tokens
}

fn run(program: &str, maybe_input: Option<&str>, debug: bool) -> String {
    let tokens = lex(program);
    let func = parse(tokens);
    if debug {
        println!("{:#?}", func);
    }
    let input = maybe_input.unwrap_or("0");
    let parsed_input: Object = Object::from_str(input);
    let output = func.execute(parsed_input);
    format!("{}", output)
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
mod test_helpers {
    use crate::Object::*;
    use crate::{lex, parse, Object};
    use num_bigint::ToBigInt;

    pub fn run_prog(program: &str, input: Object) -> Object {
        let tokens = lex(program);
        let func = parse(tokens);
        func.execute(input)
    }

    pub fn int_to_obj(int: i64) -> Object {
        Int(int.to_bigint().unwrap())
    }

    pub fn list_int_to_obj(ints: Vec<i64>) -> Object {
        List(ints.into_iter().map(int_to_obj).collect())
    }

    pub fn lli_to_obj(intss: Vec<Vec<i64>>) -> Object {
        List(intss.into_iter().map(list_int_to_obj).collect())
    }
}

#[cfg(test)]
mod codegolf;

#[cfg(test)]
mod coverage_code;

#[cfg(test)]
mod coverage_parse;
