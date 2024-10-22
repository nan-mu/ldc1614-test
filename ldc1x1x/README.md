[![crates.io](https://img.shields.io/crates/v/ldc1x1x?logo=rust) ![](https://img.shields.io/crates/d/ldc1x1x)](https://crates.io/crates/ldc1x1x)
[![API Docs](https://docs.rs/ldc1x1x/badge.svg)](https://docs.rs/ldc1x1x/)
[![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](https://unlicense.org)
[![Support me on Patreon](https://img.shields.io/badge/dynamic/json?logo=patreon&color=%23e85b46&label=support%20me%20on%20patreon&query=data.attributes.patron_count&suffix=%20patrons&url=https%3A%2F%2Fwww.patreon.com%2Fapi%2Fcampaigns%2F9395291)](https://www.patreon.com/valpackett)

# ldc1x1x

Rust [`embedded-hal`] 1.x driver for Texas Instruments (TI) I²C inductance-to-digital converters (LDC): [LDC1312/LDC1314], [LDC1612/LDC1614].

Includes `const fn` clock divider calculation (and currently requires nightly Rust because of this).

[`embedded-hal`]: https://docs.rs/embedded-hal
[LDC1312/LDC1314]: https://www.ti.com/lit/ds/symlink/ldc1314.pdf
[LDC1612/LDC1614]: https://www.ti.com/lit/ds/symlink/ldc1614.pdf

## License

This is free and unencumbered software released into the public domain.  
For more information, please refer to the `UNLICENSE` file or [unlicense.org](https://unlicense.org).
