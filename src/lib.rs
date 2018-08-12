//! This is a platform agnostic Rust driver for the DS1307 real-time clock,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

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
}

const DEVICE_ADDRESS: u8 = 0b110_1000;

/// DS1307 driver
#[derive(Debug, Default)]
pub struct DS1307<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}

impl<I2C, E> DS1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new(i2c: I2C) -> Self {
        DS1307 {
            i2c,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Reads the seconds
    pub fn get_seconds(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[0x00], &mut data)
            .map_err(Error::I2C).and(Ok(packed_bcd_to_decimal(remove_ch_bit(data[0]))))
    }

    /// Reads the minutes
    pub fn get_minutes(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[0x01], &mut data)
            .map_err(Error::I2C).and(Ok(packed_bcd_to_decimal(data[0])))
    }
}

fn remove_ch_bit(value: u8) -> u8 {
    value & 0x7F
}

/// Transforms a number in packed BCD format to decimal
fn packed_bcd_to_decimal(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0xF)
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;

    use super::*;

    fn setup<'a>(read_data: &'a [u8]) -> DS1307<hal::I2cMock<'a>> {
        let mut dev = hal::I2cMock::new();
        dev.set_read_data(&read_data);
        DS1307::new(dev)
    }

    fn check_sent_data(rtc: DS1307<hal::I2cMock>, data: &[u8]) {
        let dev = rtc.destroy();
        assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
        assert_eq!(dev.get_write_data(), &data[..]);
    }

    #[test]
    fn sends_correct_data_for_seconds_read() {
        let mut rtc = setup(&[0]);
        rtc.get_seconds().unwrap();
        check_sent_data(rtc, &[0]);
    }

    #[test]
    fn can_read_seconds() {
        let mut rtc = setup(&[0b0101_1001]);
        assert_eq!(59, rtc.get_seconds().unwrap());
    }

    #[test]
    fn ch_bit_is_ignored() {
        let mut rtc = setup(&[0b1101_1001]);
        assert_eq!(59, rtc.get_seconds().unwrap());
    }
    
    #[test]
    fn can_read_minutes() {
        let mut rtc = setup(&[0b0101_1001]);
        assert_eq!(59, rtc.get_minutes().unwrap());
        check_sent_data(rtc, &[0x01]);
    }

    #[test]
    fn can_convert_packed_bcd_to_decimal() {
        assert_eq!(0,  packed_bcd_to_decimal(0b0000_0000));
        assert_eq!(1,  packed_bcd_to_decimal(0b0000_0001));
        assert_eq!(9,  packed_bcd_to_decimal(0b0000_1001));
        assert_eq!(10, packed_bcd_to_decimal(0b0001_0000));
        assert_eq!(11, packed_bcd_to_decimal(0b0001_0001));
        assert_eq!(19, packed_bcd_to_decimal(0b0001_1001));
        assert_eq!(20, packed_bcd_to_decimal(0b0010_0000));
        assert_eq!(21, packed_bcd_to_decimal(0b0010_0001));
        assert_eq!(59, packed_bcd_to_decimal(0b0101_1001));
    }
}

