# binaryno
Rust binary running on bare metal arm cortex m4.

Getting my feet wet with osdev in rust; building a (uni) kernel, based 
on [Embedded Systems Architecture](https://www.packtpub.com/application-development/embedded-systems-architecture) 
the tutorials at [https://os.phil-opp.com](https://os.phil-opp.com).

# Install deps & build

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
