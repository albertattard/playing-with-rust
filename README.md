# Playing with Rust

This repository contains Rust examples from tutorials found online. Most of the examples found here either already exist
in the standard library or implemented inefficiently and not production ready.

## References

- [Jon Gjengset's YouTube Channel](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ)
- [Ryan Levick's YouTube Channel](https://www.youtube.com/c/RyanLevicksVideos)

## Tooling

- [rustup](https://rustup.rs/)

  ```shell
  $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- [cargo edit](https://github.com/killercup/cargo-edit)

  ```shell
  $ cargo install cargo-edit
  ```

- [cargo expand](https://github.com/dtolnay/cargo-expand)

  ```shell
  $ cargo install cargo-expand
  ```

- [valgrind](https://valgrind.org/) following the instructions [here](https://github.com/LouisBrunner/valgrind-macos)

  ```shell
  $ brew tap LouisBrunner/valgrind
  $ brew install --HEAD LouisBrunner/valgrind/valgrind
  ```

## Examples

- Lifetime
    - [Crust of Rust: Lifetime Annotations](str-split/README.md)
    - [Understanding Rust Lifetimes](lifetime/README.md)

- Macros
    - [Crust of Rust: Declarative Macros](vec-mac/README.md)

- Pointers
    - [Implementing Rust's Vec From Scratch](myvec/README.md)

- Small Applications
    - [Key Value Store](kvstore/README.md)
    - [A Singly Linked List in Rust](linked-list/README.md)

- Traits
    - [Dynamic vs Static Dispatch in Rust](traits/README.md)
