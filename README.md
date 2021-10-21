rgb2ansi256
===========
[![crate][crates-io-badge]][crates-io]
[![CI][ci-badge]][ci]

[rgb2ansi256][] is a small Rust library to convert RGB 24-bit colors into ANSI 256 (8-bit) color codes with zero dependencies
and `const fn`. This crate was ported from [ansi_colours][] C library v1.0.4.

By porting the library, we have the following two benefits:

- No unsafety
- Compile time calculation (no runtime overhead)

```rust
use rgb2ansi256::rgb_to_ansi256;

const MEDIUM_SPRING_GREEN: u8 = rgb_to_ansi256(0, 255, 175);

assert_eq!(MEDIUM_SPRING_GREEN, 49);
```

[The documentation](https://docs.rs/rgb2ansi256)

## Installation

Add this crate to your `Cargo.toml`.

```toml
[dependencies]
rgb2ansi = "0.1"
```

## License

Inheriting from the original C library, this library is distributed under [LGPL-3.0 License](./LICENSE).

[rgb2ansi256]: https://github.com/rhysd/rgb2ansi256
[ansi_colours]: https://github.com/mina86/ansi_colours
[crates-io]: https://crates.io/crates/rgb2ansi256
[crates-io-badge]: https://img.shields.io/crates/v/rgb2ansi256.svg
[ci-badge]: https://github.com/rhysd/rgb2ansi256/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/rhysd/rgb2ansi256/actions/workflows/ci.yml
