# cppwatch
`cppwatch`is small, personal toy project to make `cargo watch` for cpp project in rust. WORK IN PROGRESS.

## How to use

```shell
Usage: cppwatch --src <PATH> --execute <CMD>

Options:
  -s, --src <PATH>     Src code location to watch
  -x, --execute <CMD>  Command to execute when src changed
  -h, --help           Print help information
  -V, --version        Print version information
```

```shell
cargo run -- -s ../../poseidonos/ -x "make -j12"
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/cppwatch -s ../../poseidonos/ -x 'make -j12'`
Watch ../../poseidonos/
```