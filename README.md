# Rust DS1307 Driver

This is a platform agnostic Rust driver for the DS1307 real-time clock,
based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

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

## Status

- [ ] Read date/time
- [ ] Set date/time in 24-hour format
- [ ] Set date/time in 12-hour format
- [ ] Enable/disable clock
- [ ] Access to user RAM.
- [ ] Control square-wave output

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

