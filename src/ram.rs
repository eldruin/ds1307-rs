use crate::{Error, Register, DEVICE_ADDRESS, Ds1307};
use embedded_hal::blocking::i2c::{Write, WriteRead};

const RAM_BYTE_COUNT: usize = (Register::RAM_END - Register::RAM_BEGIN + 1) as usize;

impl<I2C, E> Ds1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Read a data array from the user RAM starting at the given offset.
    ///
    /// There is a total of 56 bytes of user RAM available so the valid ranges for
    /// the parameters are: `address_offset`: [0-55] and `data` array length: [0-56].
    ///
    /// Will return an `Error::InvalidInputData` if attempting to access a position not
    /// available or if attempting to read too much data.
    pub fn read_ram(&mut self, address_offset: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        if data.is_empty() {
            return Ok(());
        }
        self.check_ram_parameters(address_offset, &data)?;
        self.i2c
            .write_read(
                DEVICE_ADDRESS,
                &[Register::RAM_BEGIN + address_offset],
                &mut data[..],
            )
            .map_err(Error::I2C)
    }

    /// Write a data array to the user RAM starting at the given offset.
    ///
    /// There is a total of 56 bytes of user RAM available so the valid ranges for
    /// the parameters are: `address_offset`: [0-55] and `data` array length: [0-56].
    ///
    /// Will return an `Error::InvalidInputData` if attempting to access a position not
    /// available or if attempting to write too much data.
    pub fn write_ram(&mut self, address_offset: u8, data: &[u8]) -> Result<(), Error<E>> {
        if data.is_empty() {
            return Ok(());
        }
        self.check_ram_parameters(address_offset, &data)?;
        let mut payload = [0; RAM_BYTE_COUNT + 1];
        payload[0] = Register::RAM_BEGIN + address_offset;
        payload[1..=data.len()].copy_from_slice(&data);
        self.i2c
            .write(DEVICE_ADDRESS, &payload[..=data.len()])
            .map_err(Error::I2C)
    }

    fn check_ram_parameters(&self, address_offset: u8, data: &[u8]) -> Result<(), Error<E>> {
        if address_offset >= RAM_BYTE_COUNT as u8
            || (address_offset as usize + data.len()) > RAM_BYTE_COUNT
        {
            return Err(Error::InvalidInputData);
        }
        Ok(())
    }
}
