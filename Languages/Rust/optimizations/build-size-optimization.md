# Optimize build size of rust application

* Build cargo in release mode ```cargo build --release```
* Strip symbols
* Optimize for binary size
* Enable link time optimization
* Set code generation units to 1

In cargo.toml
```shell
[profile.release]
strip = true # automatically strip symbols from binary
opt-level = "s" # optimize for size. Experiment what really work for the project
lto = true # Enable link time optimization
codegen-units = 1 # maximize size reduction optimizations
```

To further reduce binary size, you can install cargo-bloat

```shell
cargo install cargo-bloat --no-default-features
```

comment out strip = true from release profile and then run

```shell
[profile.release]
# strip = true # automatically strip symbols from binary
opt-level = "s" # optimize for size. Experiment what really work for the project
lto = true # Enable link time optimization
codegen-units = 1 # maximize size reduction optimizations
cargo bloat --release --crates
```

This would show which dependencies are taking the biggest size, which you can then change for optimization.

For more information, https://github.com/johnthagen/min-sized-rust
