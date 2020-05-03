//! This is a platform agnostic Rust driver for the DS1307 real-time clock,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! This driver allows you to:
//! - Read and set date and time in 12-hour and 24-hour format. See: [`get_datetime()`].
//! - Enable and disable the real-time clock. See: [`set_running()`].
//! - Read and write user RAM. See: [`read_ram()`].
//! - Control square-wave output. See: [`enable_square_wave_output()`].
//!
//! [`get_datetime()`]: struct.DS1307.html#method.get_datetime
//! [`set_running()`]: struct.DS1307.html#method.set_running
//! [`read_ram()`]: struct.DS1307.html#method.read_ram
//! [`enable_square_wave_output()`]: struct.DS1307.html#method.enable_square_wave_output
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
//! extern crate linux_embedded_hal as hal;
//! extern crate ds1307;
//! use ds1307::DS1307;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = DS1307::new(dev);
//! let year = rtc.get_year().unwrap();
//! println!("Year: {}", year);
//! # }
//! ```
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ### Set the year
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds1307;
//! use ds1307::DS1307;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = DS1307::new(dev);
//! rtc.set_year(2018).unwrap();
//! # }
//! ```
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ### Set the current date and time at once
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds1307;
//! use ds1307::{DS1307, DateTime, Hours};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = DS1307::new(dev);
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
//! extern crate linux_embedded_hal as hal;
//! extern crate ds1307;
//! use ds1307::{DS1307, Hours};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = DS1307::new(dev);
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
//! extern crate linux_embedded_hal as hal;
//! extern crate ds1307;
//! use ds1307::DS1307;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = DS1307::new(dev);
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
//! extern crate linux_embedded_hal as hal;
//! extern crate ds1307;
//! use ds1307::{DS1307, SQWOUTRateBits};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = DS1307::new(dev);
//!
//! rtc.enable_square_wave_output().unwrap();
//! let rate_bits = SQWOUTRateBits {
//!     rs0: true,
//!     rs1: false
//! };
//! rtc.set_square_wave_output_rate(rate_bits).unwrap();
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data.
    InvalidInputData,
}

struct Register;

impl Register {
    const SECONDS: u8 = 0x00;
    const MINUTES: u8 = 0x01;
    const HOURS: u8 = 0x02;
    const DOW: u8 = 0x03;
    const DOM: u8 = 0x04;
    const MONTH: u8 = 0x05;
    const YEAR: u8 = 0x06;
    const SQWOUT: u8 = 0x07;
    const RAM_BEGIN: u8 = 0x08;
    const RAM_END: u8 = 0x3F;
}

struct BitFlags;

impl BitFlags {
    const H24_H12: u8 = 0b0100_0000;
    const AM_PM: u8 = 0b0010_0000;
    const CH: u8 = 0b1000_0000;
    const SQWE: u8 = 0b0001_0000;
    const OUTLEVEL: u8 = 0b1000_0000;
    const OUTRATERS0: u8 = 0b0000_0001;
    const OUTRATERS1: u8 = 0b0000_0010;
}

const DEVICE_ADDRESS: u8 = 0b110_1000;

/// DS1307 driver
#[derive(Debug, Default)]
pub struct DS1307<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}

mod datetime;
pub use datetime::{DateTime, Hours};
mod ram;
mod run;
mod square_wave;
pub use square_wave::SQWOUTRateBits;

impl<I2C, E> DS1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance.
    pub fn new(i2c: I2C) -> Self {
        DS1307 { i2c }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    fn is_register_bit_flag_high(&mut self, address: u8, bitmask: u8) -> Result<bool, Error<E>> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }

    fn set_register_bit_flag(&mut self, address: u8, bitmask: u8) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) == 0 {
            self.write_register(address, data | bitmask)
        } else {
            Ok(())
        }
    }

    fn clear_register_bit_flag(&mut self, address: u8, bitmask: u8) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) != 0 {
            self.write_register(address, data & !bitmask)
        } else {
            Ok(())
        }
    }

    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2C)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}
