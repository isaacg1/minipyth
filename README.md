# Minipyth

Minipyth is a new programming language,
inspired by [Pyth](https://github.com/isaacg1/pyth).
Minipyth is intended to be
to program from my phone.
As a result, the only characters used in Minipyth
are the characters on the home screen of my phone's default layout,
namely lowercase letters,
comma, period, space, and newline.
Additionally, Minipyth is a golfy language:
Each character is an independent atom of the language.
Ideally, anything can be programmed using a short program.

Because the language has only 30 characters,
it is possible to give a encoding of Minipyth using only 5 bits per byte.
This mode will is enabled by a flag.

For ease of debugging, there is also a flag for
using the character mnemonics rather than the characters themselves.

# High-level semantics

There are three types of characters in the language:

* Basic functions

* Higher-order functions

* Binders

## Basic functions

A basic function takes 1 argument and returns 1 output.
Functions are applied to the input by default.
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

A higher-order function takes a basic function as input, and returns a basic function,
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

    mhhb

**b** is the bind-1 binder.
It packages **hh** into a single function.
Therefore, this program increases each input by 2.
If the input is the list `[4, 5]`, this program returns `[6, 7]`.

# Object types

An object can be any of the following types:

* Number. Numbers can be internally represented as ints, rational or float.

* Char. A letter of a string

* List. Lists can hold arbitrary objects.

# Character reference

| chars | mnemonic | Type | Function |
| ----- | -------- | ---- | -------- |
| b | bind-1 | binder | Binds back to the first higher order function not already bound to. |
| h | head | basic | Num: +1. List: first element |
| t | tail | basic | List: All but first element |
| s | sum | basic | List: sum |
| p | product | basic | List: product |
| y | power-set | basic | List: power-set |
| l | length | basic | List: length |
| f | filter | higher | List: filter func over list |
| i | inverse | higher | Invert. Defined case-by-case. |
| m | map | higher | List: map func over list |
| o | order | higher | List: order by func |
| u | fixed-point | higher | Apply until result repeats or errors. Return all results. |
