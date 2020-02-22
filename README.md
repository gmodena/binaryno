# binaryno
Rust binary running on bare metal arm cortex m4.

# Install deps & build

```
$ cargo install cargo-xbuild
$ cargo install cargo-binutils
$ rustup default nightly
$ rustup target add thumbv7em-none-eabihf
$ rustup component add rust-src
$ rustup component add llvm-tools-preview
$ cargo xbuild --release

```

# Run

# Resources

- https://os.phil-opp.com/freestanding-rust-binary/
- https://github.com/rust-embedded/cortex-m-quickstart
