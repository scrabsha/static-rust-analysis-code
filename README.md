# What is this all about?

This repository contains the code for "Building a Rust static analysis tool in
a weekend", a blogpost that summarize what I learned while developping [cargo-breaking].

More specifically, there are three "compilers" in this repository:
* `simple_rustc_wrapper`, which explains how Cargo interacts with Rustc,
* `bundled_rustc_wrapper`, which explains how to use Rustc as a library,
* `analysis_compiler`, which explains how to actually perform static analysis.

# Compiling

Both `bundled_rustc_wrapper` and `analysis_compiler` require the nightly
toolchain and a few other components to be installed. In order to guarantee
future compatibility, we will use `nightly-2021-08-10` toolchain:

```bash
$ rustup install nightly-2021-08-10
$ rustup component add --toolchain nightly-2021-08-10 rust-src rustc-dev llvm-tools-preview
```

Everything can be built with the following command:

```bash
$ cargo build
```

# Testing a compiler

*In the following commands, substitute `<COMPILER>` with one of the compilers
included in this repository.*

First the compiler must be installed:

```bash
$ cargo install --path <COMPILER>
```

The compiler can then be called on any Rust project by setting the
`RUSTC_WRAPPER` environment variable:

```bash
$ RUSTC_WRAPPER=<COMPILER> cargo +nightly-2021-08-10 check
```

Note that the `analysis_compiler` also requires the `TARGET_CRATE` environment
variable to be set to the crate on which `cargo` is currently called. See the
blogpost itself for more information.

[cargo-breaking]: https://github.com/iomentum/cargo-breaking