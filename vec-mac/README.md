# Crust of Rust: Declarative Macros

Based
on [Crust of Rust: Declarative Macros](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2).
More information about macros can be found in
[The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html).

Install [cargo expand](https://github.com/dtolnay/cargo-expand).

```shell
$ cargo install cargo-expand
```

Expand all macros used, including those found in the tests

```shell
$ cargo expand --lib --tests
```

This didn't work for me and I got the following error

```shell
Checking vec-mac v0.1.0 (/Users/albertattard/Projects/albertattard/playing-with-rust/vec-mac)
error: the option `Z` is only accepted on the nightly compiler
error: could not compile `vec-mac`
```
