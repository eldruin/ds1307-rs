//! This is a platform agnostic Rust driver for the DS1307 real-time clock,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! This driver allows you to:
//! - Read and set date and time in 12-hour and 24-hour format. See: [`get_datetime()`].
//! - Enable and disable the real-time clock. See: [`set_running()`].
//! - Read and write user RAM. See: [`read_ram()`].
//! - Control square-wave output. See: [`enable_square_wave_output()`].
//!
//! [`get_datetime()`]: struct.Ds1307.html#method.get_datetime
//! [`set_running()`]: struct.Ds1307.html#method.set_running
//! [`read_ram()`]: struct.Ds1307.html#method.read_ram
//! [`enable_square_wave_output()`]: struct.Ds1307.html#method.enable_square_wave_output
//!
//! ## The device
//!
//! The DS1307 serial real-time clock (RTC) is a low-power, full binary-coded
//! decimal (BCD) clock/calendar plus 56 bytes of NV SRAM. Address and data are
//! transferred serially through an I2C, bidirectional bus.
//!
//! The clock/calendar provides seconds, minutes, hours, day, date, month, and
//! year information. The end of the month date is automatically adjusted for months
//! with fewer than 31 days, including corrections for leap year. The clock
//! operates in either the 24-hour or 12-hour format with AM/PM indicator.
//!
//! The DS1307 has a built-in power-sense circuit that detects power failures and
//! automatically switches to the backup supply.
//! Timekeeping operation continues while the part operates from the backup supply.
//!
//! Datasheet: [DS1307](https://datasheets.maximintegrated.com/en/ds/DS1307.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Get the year
//!
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::{Ds1307, Rtcc};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//! let year = rtc.get_year().unwrap();
//! println!("Year: {}", year);
//! # }
//! ```
//!
//! ### Set the year
//!
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::{Ds1307, Rtcc};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//! rtc.set_year(2018).unwrap();
//! # }
//! ```
//!
//! ### Set and get the current date and time at once
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::{Ds1307, NaiveDate, Rtcc};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//! let datetime = NaiveDate::from_ymd(2020, 5, 2).and_hms(19, 59, 58);
//! rtc.set_datetime(&datetime).unwrap();
//! // ...
//! let datetime = rtc.get_datetime().unwrap();
//! println!("{}", datetime);
//! // This will print something like: 2020-05-02 19:59:58
//! # }
//! ```
//!
//! ### Get the current date
//!
//! Similar methods exist for setting the date and for setting
//! getting the time.
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::{Ds1307, Rtcc};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//! let date = rtc.get_date().unwrap();
//! println!("{}", date);
//! # }
//! ```
//!
//! ### Read and write user RAM
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::Ds1307;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//!
//! let data = [171; 3];
//! rtc.write_ram(2, &data).unwrap();
//!
//! let mut data = [0; 3];
//! rtc.read_ram(2, &mut data).unwrap();
//!
//! println!("{}, {}, {}", data[0], data[1], data[2]);
//! // This will print: 171, 171, 171
//! # }
//! ```
//!
//! ### Enable square-wave output and select rate
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::{Ds1307, SqwOutRate};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//!
//! rtc.enable_square_wave_output().unwrap();
//! let rate = SqwOutRate::Khz32_768;
//! rtc.set_square_wave_output_rate(rate).unwrap();
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data.
    InvalidInputData,
}

/// DS1307 driver
#[derive(Debug, Default)]
pub struct Ds1307<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}

mod datetime;
pub use rtcc::{Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Rtcc, Timelike};
mod ram;
mod run;
mod square_wave;
pub use crate::square_wave::{SqwOutLevel, SqwOutRate};
mod register_access;
use register_access::{BitFlags, Register, ADDR};

impl<I2C, E> Ds1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance.
    pub fn new(i2c: I2C) -> Self {
        Ds1307 { i2c }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
