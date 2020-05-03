# Rust DS1307 Real-Time Clock Driver

[![crates.io](https://img.shields.io/crates/v/ds1307.svg)](https://crates.io/crates/ds1307)
[![Docs](https://docs.rs/ds1307/badge.svg)](https://docs.rs/ds1307)
[![Build Status](https://travis-ci.org/eldruin/ds1307-rs.svg?branch=master)](https://travis-ci.org/eldruin/ds1307-rs)
[![Coverage Status](https://coveralls.io/repos/eldruin/ds1307-rs/badge.svg?branch=master)](https://coveralls.io/r/eldruin/ds1307-rs?branch=master)

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

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use ds1307::{DateTime, Hours, DS1307};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = DS1307::new(dev);
    let datetime = DateTime {
        year: 2020,
        month: 5,
        day: 2,
        weekday: 6,
        hour: Hours::H24(19),
        minute: 59,
        second: 58,
    };
    rtc.set_datetime(&datetime).unwrap();

    let datetime = rtc.get_datetime().unwrap();

    // The hours depend on the RTC running mode.
    match datetime.hour {
        Hours::H24(h) => println!(
            "{}-{}-{}, {} {}:{}:{}",
            datetime.year,
            datetime.month,
            datetime.day,
            datetime.weekday,
            h,
            datetime.minute,
            datetime.second
        ),
        Hours::AM(h) => println!(
            "{}-{}-{}, {} {}:{}:{} AM",
            datetime.year,
            datetime.month,
            datetime.day,
            datetime.weekday,
            h,
            datetime.minute,
            datetime.second
        ),
        Hours::PM(h) => println!(
            "{}-{}-{}, {} {}:{}:{} PM",
            datetime.year,
            datetime.month,
            datetime.day,
            datetime.weekday,
            h,
            datetime.minute,
            datetime.second
        ),
    }
    // This will print something like: 2020-05-02, 6 19:59:58
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/ds1307-rs/issues).

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

