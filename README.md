# binaryno
Rust binary running on bare ARM CortexM hardware (`thumbv7em-none-eabihf`).

# Scope
This repo contains a template for cross compiling`thumbv7em-none-eabihf` binaries on MacOS and Linux
host. The target spec for this architecture (bare metal, no os) can be found in `thumbv7em-none-eabihf-noos.json`.

I'm getting my feet wet with osdev in rust; building a (uni) kernel, based 
on [Embedded Systems Architecture](https://www.packtpub.com/application-development/embedded-systems-architecture) 
the tutorials at [https://os.phil-opp.com](https://os.phil-opp.com).

# Install deps & build


Rust `nightly` and `thumbv7em-none-eabihf` target are required to build the binary.

Manual steps:
```
$ cargo install cargo-xbuild
$ rustup default nightly
$ rustup target add thumbv7em-none-eabihf
$ rustup component add rust-src
$ rustup component add llvm-tools-preview
$ cargo xbuild --release
```

# Run
TODO: needs a bootloader

# Resources
- https://os.phil-opp.com/freestanding-rust-binary/
- https://github.com/rust-embedded/cortex-m-quickstart
- https://www.packtpub.com/application-development/embedded-systems-architecture
