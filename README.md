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

Here is a comparison before/after conversion.

<img src="https://github.com/rhysd/ss/raw/master/rgb2ansi256/compare.png" alt="comparison" width=533 height=181/>

The left window is iTerm2 which supports 24-bit colors (true colors). The right window is Terminal.app which supports only
8-bit colors. Conversion could reproduce 24-bit colors in 8-bit colors well.

## Installation

Add this crate to your `Cargo.toml`.

```toml
[dependencies]
rgb2ansi = "0.1"
```

## Benchmark

This is the result of [the micro benchmark suite](./bench/benches/bench.rs). rgb2ansi256 is slightly faster than ansi_colours.

```
rgb2ansi256             time:   [150.58 us 151.06 us 151.55 us]
                        change: [-1.2657% -0.9774% -0.6991%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) low mild
  4 (4.00%) high mild

ansi_colours            time:   [158.07 us 158.83 us 159.64 us]
                        change: [-1.3717% -0.7338% -0.1909%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low mild
  8 (8.00%) high mild
  1 (1.00%) high severe
```

- Machine: iMac (Retina 5K, 27-inch, 2020)
- ansi_colours: v1.0.4
- rustc: 1.56 (stable)

## License

Inheriting from the original C library, this library is distributed under [LGPL-3.0 License](./LICENSE).

[rgb2ansi256]: https://github.com/rhysd/rgb2ansi256
[ansi_colours]: https://github.com/mina86/ansi_colours
[crates-io]: https://crates.io/crates/rgb2ansi256
[crates-io-badge]: https://img.shields.io/crates/v/rgb2ansi256.svg
[ci-badge]: https://github.com/rhysd/rgb2ansi256/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/rhysd/rgb2ansi256/actions/workflows/ci.yml
