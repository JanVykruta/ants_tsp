To install Rust I suggest [rustup](https://rustup.rs/). Although, a system package should work fine. For code viewing I suggest `vs-code` with `rust-analyzer` extension. It annotates all inferred types (works like magic). To use `rust-analyzer` you have to use rustup for installation of Rust, it does not work with a system package.

To run use:
```shell
cargo run --release <PROGRAM_ARGS>
```

Possible `PROGRAM_ARGS` can be obtained using (note the `--` before `-h`):
```shell
cargo run --release -- -h
```

Please note that the input files have to be unarchived beforehand.

Examples:
```shell
cargo run --release data/bays29.tsp data/bays29.opt.tour --iter 1000
```