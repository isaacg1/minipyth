# Minipyth

Minipyth is a minimal programming language,
inspired by [Pyth](https://github.com/isaacg1/pyth).
Minipyth is intended to be
programmed from my phone.
As a result, the only characters used in Minipyth
are lowercase letters.
Additionally, Minipyth is a golfy language:
Each character is an independent atom of the language.
Ideally, anything can be programmed using a short program.

To run a Minipyth program, you can simply run:

    cargo run <program> <input>

Input is optional, defaults to 0.

You'll need rust installed, here's the instructions to do so: https://www.rust-lang.org/learn/get-started

Minipyth is a highly minimalistic language,
and intentionally lacks many features that are taken for granted in other languages.
Minipyth lacks:

* Variables

* Closures

* Functions of arity other than 1.

* General control flow primitives.

* Strings, floats, etc. Minipyth only has ints and lists.

Now that we know what Minipyth lacks, let's talk about its features.

# High-level semantics

There are three types of characters in the language:

* Basic functions, which take one object as input and return an object as output.

* Higher-order functions, which take one or more functions as input
and create a function as output. The resulting function takes one object as input
and returns an object as output.

* Binders, which combine a sequence of functions into a single function.

## Basic functions

A basic function takes 1 argument and returns 1 output.
Functions are applied to the input by default,
in order from right to left.
For instance, consider the program

    hs

**s** is the sum function,
while **h** functions as the successor function in this context.
By default, the functions are applied to the input on STDIN.
Suppose that STDIN consists of two numbers, newline seaparated:

    4
    5

These arguments are internally converted to a list:

    [4, 5]

The program **hs** sums the list, and increments the result, returning 10.

## Higher-order functions

A higher-order function takes one or more functions as input,
and returns a function,
which is then applied as usual.
By default, the input basic function is the following character,
or the basic function returned by the following character.

For instance, consider the program

    mh

**m** is the map higher-order function.
It creates the function which maps its input function over a list.

Suppose that the input is the list `[4, 5]`. The program **mh** returns `[5, 6]`.

## Binding character

A binding character runs before the characters listed above, at parse time.
Binding characters look back through the program, find a certain string of functions,
and package them into one function, for use in higher-order functions.

For instance, consider the program

    mhhz

**z** is the bind-1 binder.
It packages **hh** into a single function.
Therefore, this program increases each input by 2.
If the input is the list `[4, 5]`, this program returns `[6, 7]`.

# Object types

An object can be any of the following types:

* Integer. Numbers are unbounded in size.

* List. Lists can hold arbitrary objects.

* Error. Errors mostly just propogate up to the top of the program,
but some functions can handle them gracefully.

Sometimes, functions need to interpret objects as truthy or falsy.
The falsy objects are 0, [], and all errors.
All other objects are falsy.

Sometimes, functions need to output a truth value.
1 represents true, 0 represents false.

# Language reference

Updated: 2021-12-23

| chars | mnemonic | Type | Function |
| ----- | -------- | ---- | -------- |
| a | all-pairs | basic | Given [x, [a, b, c, ...]], create [[x, a], [x, b], ...]. Given [[a, b, c, ...], int], create [[a, int], [b, int], ...]. Given list of ints l, same as [l, l]. Given int i, [[i, 0], [i, 1], ..., [i, i-1]]. |
| b | bifurcate | higher-2 | Given two funcs, apply each to input, make list. |
| c | combine | basic | Transponse list. Int: permutations. |
| d | deduplicate | basic | Remove all repeated elements. Int: partitions. |
| e | equal | basic | Given a list, check if all elements identical. Int: subset permutations. |
| f | filter | higher | to_list: filter func over list |
| h | head | basic | Int: x+1. List: first element |
| i | inverse | higher | Invert. Defined case-by-case. |
| j | deep-index | higher | Given list and func, apply func to list length, then deep index the result into the input. Int unimplemented. |
| k | constant | basic | 0 |
| l | length | basic | List: length. Int: To binary. |
| m | map | higher | to_list: map func over list. |
| n | negate | basic | Int: -x. List: reverse | 
| o | order | higher | to_list: order by key given by func |
| p | product | basic | Int: prime-factorization. List(Int): product. List(List): Cartesian product |
| q | quote | binder | Pair with next q, combine everything within into one function. If odd number, first q pairs with earliest eligible location in the program.
| r | repeat | higher | Apply func a number of times equal to input[0], starting with input[1]. Return all results. If input is length 1 or non-list, use input as both times and start.
| s | sum | basic | Int: logical negation. List(Int): integer sum. List(List): concatenate |
| t | tail | basic | Int: x-1. List: All but first element |
| u | update | higher | Given [i, list] and func, update list[i] using func. |
| v | reverse-lookup | higher | Find first input to func that produces arg as output. |
| w | while | higher-2 | Apply second func until first func returns falsy or error. Return starting value and all results.
| x | fixed-point | higher | Apply until result repeats or errors. Return all results. |
| y | power-set | basic | Int: 2^x. List: power-set |
| z | bind-eager | binder | Combine everything backwards until unbound higher-order function into one function.

Glossary:

* to_list: Cast to list. Int >= 0: range [0, i). Int < 0: reverse of to_list(-i). List unchanged.
