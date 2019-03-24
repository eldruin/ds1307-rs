use super::{BitFlags, Error, Register, DS1307};
use hal::blocking::i2c::{Write, WriteRead};

/// Square-wave output rate bits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SQWOUTRateBits {
    /// Rate selection control bit 0.
    pub rs0: bool,
    /// Rate selection control bit 1.
    pub rs1: bool,
}

impl<I2C, E> DS1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Read whether the square-wave output is enabled.
    pub fn is_square_wave_output_enabled(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::SQWOUT, BitFlags::SQWE)
    }

    /// Enable the square-wave output.
    /// (Does not alter the device register if already enabled).
    pub fn enable_square_wave_output(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::SQWOUT, BitFlags::SQWE)
    }

    /// Disable the square-wave output.
    /// (Does not alter the device register if already disabled).
    pub fn disable_square_wave_output(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::SQWOUT, BitFlags::SQWE)
    }

    /// Read status of square-wave output level control bit.
    pub fn get_square_wave_output_level(&mut self) -> Result<bool, Error<E>> {
        self.is_register_bit_flag_high(Register::SQWOUT, BitFlags::OUTLEVEL)
    }

    /// Set square-wave output level high.
    /// (Does not alter the device register if level is already high).
    pub fn set_square_wave_output_level_high(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::SQWOUT, BitFlags::OUTLEVEL)
    }

    /// Set square-wave output level low.
    /// (Does not alter the device register if level is already low).
    pub fn set_square_wave_output_level_low(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::SQWOUT, BitFlags::OUTLEVEL)
    }

    /// Set square-wave output level.
    /// (Does not alter the device register if same level is already configured).
    pub fn set_square_wave_output_level(
        &mut self,
        should_level_be_high: bool,
    ) -> Result<(), Error<E>> {
        if should_level_be_high {
            self.set_square_wave_output_level_high()
        } else {
            self.set_square_wave_output_level_low()
        }
    }

    /// Read square-wave output rate control bits.
    pub fn get_square_wave_output_rate(&mut self) -> Result<SQWOUTRateBits, Error<E>> {
        let data = self.read_register(Register::SQWOUT)?;
        Ok(SQWOUTRateBits {
            rs0: (data & BitFlags::OUTRATERS0) != 0,
            rs1: (data & BitFlags::OUTRATERS1) != 0,
        })
    }

    /// Set square-wave output rate control bits.
    /// (Does not alter the device register if the same rate is already configured).
    pub fn set_square_wave_output_rate(
        &mut self,
        rate_bits: SQWOUTRateBits,
    ) -> Result<(), Error<E>> {
        let data = self.read_register(Register::SQWOUT)?;
        if rate_bits.rs0 != ((data & BitFlags::OUTRATERS0) != 0)
            || rate_bits.rs1 != ((data & BitFlags::OUTRATERS1) != 0)
        {
            let mut data = data & !(BitFlags::OUTRATERS0 | BitFlags::OUTRATERS1);
            if rate_bits.rs0 {
                data |= BitFlags::OUTRATERS0;
            }
            if rate_bits.rs1 {
                data |= BitFlags::OUTRATERS1;
            }
            self.write_register(Register::SQWOUT, data)
        } else {
            Ok(())
        }
    }
}
