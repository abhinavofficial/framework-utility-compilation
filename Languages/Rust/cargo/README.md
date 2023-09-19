# Cargo

Cargo is a package manager for Rust. You can use Cargo to install and manage packages that you want to use, Cargo is also the build system, no more make files, Cargo is also the test Runner, and the documentation generator, etc. Cargo is like all the good parts of npm and pip and bundler and make.

cargo fmt and clippy are another two useful tools to keep your code optimized and clean. You can also set pre-commit hooks, so cargo fmt and clippy are invoked before the code is checked in.

```shell
$ vi .git/hooks/pre-commit

cargo fmt
exec cargo clippy -- -D warnings
```