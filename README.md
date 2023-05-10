# Imgur link generator

<a href="https://asciinema.org/a/583804" target="_blank"><img src="https://asciinema.org/a/583804.svg" /></a>

## How to build?

[First install rust](https://rustup.rs/) then run
```
cargo build --release
```

```
Brute-forces imgur image links

Usage: imgur_link_generator [OPTIONS]

Options:
  -l, --length <LENGTH>  Generated imgur code length [default: 5] [default: 5]
  -a, --amount <AMOUNT>  How many codes to generate [default: 1] [default: 1]
  -s, --spinner          Enables progress spinner [default: true]
      --tries            Shows failed tries in spinner [default: false]
  -h, --help             Print help
  -V, --version          Print version
```
