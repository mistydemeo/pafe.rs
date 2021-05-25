# pafe for Rust

This crate provides bindings for [libpafe](https://github.com/rfujita/libpafe), a C library for interacting with Sony [PaSoRi](https://ja.wikipedia.org/wiki/PaSoRi) USB devices. These devices are USB readers for [FeliCa IC cards](https://en.wikipedia.org/wiki/FeliCa), a kind of contactless smart card which is widely used in Japan - particularly for Japanese transit cards like [Suica](https://en.wikipedia.org/wiki/Suica) and the [Edy](https://en.wikipedia.org/wiki/Edy) electronic wallet.

There are two crates in this repo: `pafe-sys`, which provides low-level bindings to `libpafe`, and `pafe`, which provides high-level Rustic bindings. `pafe-sys` covers roughly 100% of libpafe's functionality, while `pafe` is still very incomplete.

## Usage

You need to have a copy of `libpafe` installed to build this crate. If you use Homebrew on Mac, you can get it by running:

```sh
brew install mistydemeo/formulae/libpafe
```

## I want to help!

Thank you! I'm still in the early stages of working on this, so the project structure may change frequently. PRs that add more bindings to the lower-level functions.

## License

GPL 2.0, matching libpafe itself.
