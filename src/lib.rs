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
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::Ds1307;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//! let year = rtc.get_year().unwrap();
//! println!("Year: {}", year);
//! # }
//! ```
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ### Set the year
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::Ds1307;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//! rtc.set_year(2018).unwrap();
//! # }
//! ```
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ### Set the current date and time at once
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::{Ds1307, DateTime, Hours};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//! let datetime = DateTime {
//!                           year: 2018,
//!                           month: 08,
//!                           day: 15,
//!                           weekday: 4,
//!                           hour: Hours::H24(19),
//!                           minute: 59,
//!                           second: 58
//!                };
//! rtc.set_datetime(&datetime).unwrap();
//! # }
//! ```
//!
//! ### Get the current date and time at once
//!
//! ```no_run
//! use linux_embedded_hal as hal;
//! use ds1307::{Ds1307, Hours};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds1307::new(dev);
//!
//! let datetime = rtc.get_datetime().unwrap();
//!
//! // The hours depend on the RTC running mode
//! match datetime.hour {
//!     Hours::H24(h) => println!("{}-{}-{}, {} {}:{}:{}", datetime.year,
//!                               datetime.month, datetime.day, datetime.weekday,
//!                               h, datetime.minute, datetime.second),
//!     Hours::AM(h) => println!("{}-{}-{}, {} {}:{}:{} AM", datetime.year,
//!                               datetime.month, datetime.day, datetime.weekday,
//!                               h, datetime.minute, datetime.second),
//!     Hours::PM(h) => println!("{}-{}-{}, {} {}:{}:{} PM", datetime.year,
//!                               datetime.month, datetime.day, datetime.weekday,
//!                               h, datetime.minute, datetime.second),
//! }
//! // This will print something like: 2018-08-15, 4 19:59:58
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
pub use crate::datetime::{DateTime, Hours};
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
