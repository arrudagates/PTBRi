# PTBR interpreter

## What is PTBR?

PTBR is a language being developed for easy understanding with it's main goal to be a simple way to introduce Portuguese speakers without any technical background to programming

Here's an example:

```
a é 5
b é 2
mostre a vezes b
```

## Where are we in the development?

PTBRi started as an interpreter forked from rust-lci, a LOLCODE interpreter, following that logic, the language itself started as LOLCODE. That old codebase can be found at [old-PTBRi](https://github.com/arrudagates/old-PTBRi)

This is a rewrite from scratch with a complete rewrite of everything from the language specification to the parser and interpreter.

The current code is far from production ready, there's so much to be done, the language is far from being 100% implemented and both the parser and interpreter are prototype level code.

## How to use

Currently we have a basic prototype, it can be tested by running the code with a ptbr file as argument:

```
cargo run tests/basic.ptbr
// or compiled
ptbri tests/basic.ptbr
```

## What's already implemented?

You can check the already implemented syntax [here](ptbr_definitions.md)

