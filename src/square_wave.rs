use crate::{BitFlags, Ds1307, Error, Register};
use embedded_hal::blocking::i2c::{Write, WriteRead};

/// Square-wave output rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SqwOutRate {
    /// 1 Hz
    Hz1,
    /// 4.096 kHz
    Khz4_096,
    /// 8.192 kHz
    Khz8_192,
    /// 32.768 kHz
    Khz32_768,
}

/// Square-wave output level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SqwOutLevel {
    /// Low
    Low,
    /// High
    High,
}

impl<I2C, E> Ds1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Read whether the square-wave output is enabled.
    pub fn square_wave_output_enabled(&mut self) -> Result<bool, Error<E>> {
        self.register_bit_flag_high(Register::SQWOUT, BitFlags::SQWE)
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
    pub fn get_square_wave_output_level(&mut self) -> Result<SqwOutLevel, Error<E>> {
        if self.register_bit_flag_high(Register::SQWOUT, BitFlags::OUTLEVEL)? {
            Ok(SqwOutLevel::High)
        } else {
            Ok(SqwOutLevel::Low)
        }
    }

    /// Set square-wave output level.
    /// (Does not alter the device register if same level is already configured).
    pub fn set_square_wave_output_level(&mut self, level: SqwOutLevel) -> Result<(), Error<E>> {
        match level {
            SqwOutLevel::Low => self.clear_register_bit_flag(Register::SQWOUT, BitFlags::OUTLEVEL),
            SqwOutLevel::High => self.set_register_bit_flag(Register::SQWOUT, BitFlags::OUTLEVEL),
        }
    }

    /// Read square-wave output rate control bits.
    pub fn get_square_wave_output_rate(&mut self) -> Result<SqwOutRate, Error<E>> {
        let data = self.read_register(Register::SQWOUT)?;
        let rs1 = (data & BitFlags::OUTRATERS1) != 0;
        let rs0 = (data & BitFlags::OUTRATERS0) != 0;
        match (rs1, rs0) {
            (false, false) => Ok(SqwOutRate::Hz1),
            (false, true) => Ok(SqwOutRate::Khz4_096),
            (true, false) => Ok(SqwOutRate::Khz8_192),
            (true, true) => Ok(SqwOutRate::Khz32_768),
        }
    }

    /// Set square-wave output rate.
    pub fn set_square_wave_output_rate(&mut self, rate: SqwOutRate) -> Result<(), Error<E>> {
        let data = self.read_register(Register::SQWOUT)?;
        let data = data & !BitFlags::OUTRATERS1 & !BitFlags::OUTRATERS0;
        let sqwout = match rate {
            SqwOutRate::Hz1 => data,
            SqwOutRate::Khz4_096 => data | BitFlags::OUTRATERS0,
            SqwOutRate::Khz8_192 => data | BitFlags::OUTRATERS1,
            SqwOutRate::Khz32_768 => data | BitFlags::OUTRATERS1 | BitFlags::OUTRATERS0,
        };
        self.write_register(Register::SQWOUT, sqwout)
    }
}
