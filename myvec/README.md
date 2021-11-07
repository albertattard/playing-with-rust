## Implementing Rust's Vec From Scratch

**Do not use this code for production!!  This is a simple exercise using pointers and memory.**

Based on [Implementing Rust's Vec From Scratch](https://www.youtube.com/watch?v=3OL95gZgPWA)

Install [valgrind](https://valgrind.org/) following the
instructions [here](https://github.com/LouisBrunner/valgrind-macos).

```shell
$ brew tap LouisBrunner/valgrind
$ brew install --HEAD LouisBrunner/valgrind/valgrind
```

Build and analyse the program

```shell
$ cargo build --release
$ valgrind ./target/release/myvec
```
