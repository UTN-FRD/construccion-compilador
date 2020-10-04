# frd-lisp [![Build Status](https://travis-ci.org/UTN-FRD/construccion-compilador.svg?branch=master)](https://travis-ci.org/UTN-FRD/construccion-compilador)

Lisp written in Rust as part of a research project in UTN-FRD (Universidad Tecnologica Nacional - Facultad Regional Delta)

## Requirements

- [Rust (latest stable), via Rustup is recommended](https://www.rust-lang.org/tools/install)
- [rust fmt](https://github.com/rust-lang/rustfmt)
- [rust clippy](https://github.com/rust-lang/rust-clippy)
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)
- [nodejs](https://nodejs.org/es/) 12.x or newer
- git

## Running the interpreter

```
$ cargo run
```

## Running the web interpreter

First compile the Rust code to wasm

```
$ wasm-pack build
```

Install web dependencies

```
$ cd site/
$ npm install
```

Start the server

```
$ npm run start
```

Then go to http://localhost:8080/


## Running tests

```
$ cargo test
```

## Code quality

- PRs MUST have passing builds to be merged
- be sure to auto format your code with `cargo fmt`
- add tests for new features
- be sure to use `cargo clippy` to detect common problems in your code (we cannot include this in the CI process because of an outstanding issue between clippy and lalrpop https://github.com/lalrpop/lalrpop/pull/384)

# Mission, Vision, Values

## Mission

TODO

## Vision

TODO

## Values

TODO

## References

The two main reference points are

- [norvigs toy lisp impl](http://norvig.com/lispy.html): from which we got the basics
- [Racket lisp](https://docs.racket-lang.org/getting-started/index.html): the lisp implementation that we aim to (right now we are not very close but the ideal is to make something like Ractket syntax wise)
- [Racket lisp](https://stackoverflow.com/questions/41417892/what-is-the-difference-between-and-brackets-in-racket-lisp-programming) `()` vs `[]`
