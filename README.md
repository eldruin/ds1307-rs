# Rust DS1307 Real-Time Clock Driver

[![crates.io](https://img.shields.io/crates/v/ds1307.svg)](https://crates.io/crates/ds1307)
[![Docs](https://docs.rs/ds1307/badge.svg)](https://docs.rs/ds1307)
[![Build Status](https://travis-ci.org/eldruin/ds1307-rs.svg?branch=master)](https://travis-ci.org/eldruin/ds1307-rs)
[![Coverage Status](https://coveralls.io/repos/eldruin/ds1307-rs/badge.svg?branch=master)](https://coveralls.io/r/eldruin/ds1307-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the DS1307 real-time clock,
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Read and set date and time in 12-hour and 24-hour format. See: `get_datetime`
- Enable and disable the real-time clock. See: `set_running`
- Read and write user RAM. See: `read_ram`
- Control square-wave output. See: `enable_square_wave_output`

[Introductory blog post](https://blog.eldruin.com/ds1307-real-time-clock-rtc-driver-in-rust/)

## The device

The DS1307 serial real-time clock (RTC) is a low-power, full binary-coded
decimal (BCD) clock/calendar plus 56 bytes of NV SRAM. Address and data are
transferred serially through an I2C, bidirectional bus.

The clock/calendar provides seconds, minutes, hours, day, date, month, and
year information. The end of the month date is automatically adjusted for months
with fewer than 31 days, including corrections for leap year. The clock
operates in either the 24-hour or 12-hour format with AM/PM indicator.

The DS1307 has a built-in power-sense circuit that detects power failures and
automatically switches to the backup supply.
Timekeeping operation continues while the part operates from the backup supply.

Datasheet: [DS1307](https://datasheets.maximintegrated.com/en/ds/DS1307.pdf)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

