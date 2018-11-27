# Steps to reproduce

Install toolchains:
```sh
rustup toolchain install nightly-2018-11-25
rustup toolchain install nightly-2018-11-26
```

Build with `nightly-2018-11-25`:
```sh
$ cargo +nightly-2018-11-25 build --release
    Finished release [optimized] target(s) in 0.91s
```

Build with `nightly-2018-11-26`:
```sh
$ cargo +nightly-2018-11-26 build --release
    Finished release [optimized] target(s) in 8.24s
```
