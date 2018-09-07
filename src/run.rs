#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate embedded_hal as hal;
use hal::blocking::i2c::{Write, WriteRead};
use super::{DS1307, Error, Register, BitFlags};

impl<I2C, E> DS1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
     /// Read if the clock is running.
    pub fn is_running(&mut self) -> Result<bool, Error<E>> {
        let data = self.read_register(Register::SECONDS)?;
        Ok(data & BitFlags::CH != 0)
    }

    /// Set the clock to run (default on power-on).
    /// (Does not alter the device register if already running).
    pub fn set_running(&mut self) -> Result<(), Error<E>> {
        // needs to keep the seconds so we read it first
        let data = self.read_register(Register::SECONDS)?;
        if (data & BitFlags::CH) == 0 {
            self.write_register(Register::SECONDS, data | BitFlags::CH)
        }
        else {
            Ok(())
        }
    }

    /// Halt the clock.
    /// (Does not alter the device register if already halted).
    pub fn halt(&mut self) -> Result<(), Error<E>> {
        // needs to keep the seconds so we read it first
        let data = self.read_register(Register::SECONDS)?;
        if (data & BitFlags::CH) != 0 {
            self.write_register(Register::SECONDS, data & !BitFlags::CH)
        }
        else {
            Ok(())
        }
    }
}
