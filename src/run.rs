use crate::{BitFlags, Ds1307, Error, Register};
use embedded_hal::i2c::I2c;

impl<I2C, E> Ds1307<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Read if the clock is running.
    pub fn running(&mut self) -> Result<bool, Error<E>> {
        Ok(!self.register_bit_flag_high(Register::SECONDS, BitFlags::CH)?)
    }

    /// Set the clock to run (default on power-on).
    /// (Does not alter the device register if already running).
    pub fn set_running(&mut self) -> Result<(), Error<E>> {
        // Clock Halt (CH) bit should be cleared for oscillator to work.
        self.clear_register_bit_flag(Register::SECONDS, BitFlags::CH)
    }

    /// Halt the clock.
    /// (Does not alter the device register if already halted).
    pub fn halt(&mut self) -> Result<(), Error<E>> {
        // Clock Halt (CH) bit should be set for oscillator to stop.
        self.set_register_bit_flag(Register::SECONDS, BitFlags::CH)
    }
}
