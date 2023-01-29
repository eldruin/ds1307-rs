# Rust DS1307 Real-Time Clock Driver

[![crates.io](https://img.shields.io/crates/v/ds1307.svg)](https://crates.io/crates/ds1307)
[![Docs](https://docs.rs/ds1307/badge.svg)](https://docs.rs/ds1307)
![MSRV](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/ds1307-rs/workflows/Build/badge.svg)](https://github.com/eldruin/ds1307-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/eldruin/ds1307-rs/badge.svg?branch=master)](https://coveralls.io/r/eldruin/ds1307-rs?branch=master)

This is a platform agnostic Rust driver for the DS1307 real-time clock,
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Read and set date and time in 12-hour and 24-hour format. See: `datetime`
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

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use ds1307::{DateTimeAccess, Ds1307, NaiveDate};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds1307::new(dev);
    let datetime = NaiveDate::from_ymd_opt(2022, 1, 2)
        .unwrap()
        .and_hms_opt(19, 59, 58)
        .unwrap();
    rtc.set_datetime(&datetime).unwrap();
    // ...
    let datetime = rtc.datetime().unwrap();
    println!("{datetime}");
    // This will print something like: 2022-01-02 19:59:58
}
```


## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.62 and up. It *might*
compile with older versions but that may change in any new patch release.

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/ds1307-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
