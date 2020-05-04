use crate::{Ds1307, Error};
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub struct Register;
impl Register {
    pub const SECONDS: u8 = 0x00;
    pub const MINUTES: u8 = 0x01;
    pub const HOURS: u8 = 0x02;
    pub const DOW: u8 = 0x03;
    pub const DOM: u8 = 0x04;
    pub const MONTH: u8 = 0x05;
    pub const YEAR: u8 = 0x06;
    pub const SQWOUT: u8 = 0x07;
    pub const RAM_BEGIN: u8 = 0x08;
    pub const RAM_END: u8 = 0x3F;
}

pub struct BitFlags;
impl BitFlags {
    pub const H24_H12: u8 = 0b0100_0000;
    pub const AM_PM: u8 = 0b0010_0000;
    pub const CH: u8 = 0b1000_0000;
    pub const SQWE: u8 = 0b0001_0000;
    pub const OUTLEVEL: u8 = 0b1000_0000;
    pub const OUTRATERS0: u8 = 0b0000_0001;
    pub const OUTRATERS1: u8 = 0b0000_0010;
}

pub const ADDR: u8 = 0b110_1000;

impl<I2C, E> Ds1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub(crate) fn register_bit_flag_high(
        &mut self,
        address: u8,
        bitmask: u8,
    ) -> Result<bool, Error<E>> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }

    pub(crate) fn set_register_bit_flag(
        &mut self,
        address: u8,
        bitmask: u8,
    ) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) == 0 {
            self.write_register(address, data | bitmask)
        } else {
            Ok(())
        }
    }

    pub(crate) fn clear_register_bit_flag(
        &mut self,
        address: u8,
        bitmask: u8,
    ) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) != 0 {
            self.write_register(address, data & !bitmask)
        } else {
            Ok(())
        }
    }

    pub(crate) fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }

    pub(crate) fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(ADDR, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}
