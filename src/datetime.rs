use super::{BitFlags, Error, Register, DEVICE_ADDRESS, DS1307};
use hal::blocking::i2c::{Write, WriteRead};

/// Hours in either 12-hour (AM/PM) or 24-hour format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hours {
    /// AM [1-12]
    AM(u8),
    /// PM [1-12]
    PM(u8),
    /// 24H format [0-23]
    H24(u8),
}

/// Date and time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime {
    /// Year [2000-2099]
    pub year: u16,
    /// Month [1-12]
    pub month: u8,
    /// Day [1-31]
    pub day: u8,
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour in 24h/12h format
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8,
}

impl<I2C, E> DS1307<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Read the seconds.
    pub fn get_seconds(&mut self) -> Result<u8, Error<E>> {
        let data = self.read_register(Register::SECONDS)?;
        Ok(packed_bcd_to_decimal(remove_ch_bit(data)))
    }

    /// Read the minutes.
    pub fn get_minutes(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::MINUTES)
    }

    /// Read the hours.
    pub fn get_hours(&mut self) -> Result<Hours, Error<E>> {
        let data = self.read_register(Register::HOURS)?;
        self.get_hours_from_register(data)
    }

    fn get_hours_from_register(&self, data: u8) -> Result<Hours, Error<E>> {
        if is_24h_format(data) {
            Ok(Hours::H24(packed_bcd_to_decimal(data & !BitFlags::H24_H12)))
        } else if is_am(data) {
            Ok(Hours::AM(packed_bcd_to_decimal(
                data & !(BitFlags::H24_H12 | BitFlags::AM_PM),
            )))
        } else {
            Ok(Hours::PM(packed_bcd_to_decimal(
                data & !(BitFlags::H24_H12 | BitFlags::AM_PM),
            )))
        }
    }

    /// Read the day of the week [1-7].
    pub fn get_weekday(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::DOW)
    }

    /// Read the day of the month [1-31].
    pub fn get_day(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::DOM)
    }

    /// Read the month [1-12].
    pub fn get_month(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::MONTH)
    }

    /// Read the year [2000-2099].
    pub fn get_year(&mut self) -> Result<u16, Error<E>> {
        let year = self.read_register_decimal(Register::YEAR)?;
        Ok(2000 + u16::from(year))
    }

    /// Read the date and time.
    pub fn get_datetime(&mut self) -> Result<DateTime, Error<E>> {
        let mut data = [0; 7];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[0x00], &mut data)
            .map_err(Error::I2C)?;
        Ok(DateTime {
            year: 2000 + u16::from(packed_bcd_to_decimal(data[Register::YEAR as usize])),
            month: packed_bcd_to_decimal(data[Register::MONTH as usize]),
            day: packed_bcd_to_decimal(data[Register::DOM as usize]),
            weekday: packed_bcd_to_decimal(data[Register::DOW as usize]),
            hour: self.get_hours_from_register(data[Register::HOURS as usize])?,
            minute: packed_bcd_to_decimal(data[Register::MINUTES as usize]),
            second: packed_bcd_to_decimal(remove_ch_bit(data[Register::SECONDS as usize])),
        })
    }

    /// Set the seconds [0-59].
    ///
    /// Will return an `Error::InvalidInputData` if the seconds are out of range.
    pub fn set_seconds(&mut self, seconds: u8) -> Result<(), Error<E>> {
        if seconds > 59 {
            return Err(Error::InvalidInputData);
        }
        // needs to keep the CH bit status so we read it first
        let data = self.read_register(Register::SECONDS)?;
        self.write_register(
            Register::SECONDS,
            data & BitFlags::CH | decimal_to_packed_bcd(seconds),
        )
    }

    /// Set the minutes [0-59].
    ///
    /// Will return an `Error::InvalidInputData` if the minutes are out of range.
    pub fn set_minutes(&mut self, minutes: u8) -> Result<(), Error<E>> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::MINUTES, minutes)
    }

    /// Set the hours.
    ///
    /// Changes the operating mode to 12h/24h depending on the parameter.
    ///
    /// Will return an `Error::InvalidInputData` if the hours are out of range.
    pub fn set_hours(&mut self, hours: Hours) -> Result<(), Error<E>> {
        let value = self.get_hours_register_value(hours)?;
        self.write_register(Register::HOURS, value)
    }

    fn get_hours_register_value(&mut self, hours: Hours) -> Result<u8, Error<E>> {
        match hours {
            Hours::H24(h) if h > 23 => Err(Error::InvalidInputData),
            Hours::H24(h) => Ok(decimal_to_packed_bcd(h)),
            Hours::AM(h) if h < 1 || h > 12 => Err(Error::InvalidInputData),
            Hours::AM(h) => Ok(BitFlags::H24_H12 | decimal_to_packed_bcd(h)),
            Hours::PM(h) if h < 1 || h > 12 => Err(Error::InvalidInputData),
            Hours::PM(h) => Ok(BitFlags::H24_H12 | BitFlags::AM_PM | decimal_to_packed_bcd(h)),
        }
    }

    /// Set the day of week [1-7].
    ///
    /// Will return an `Error::InvalidInputData` if the day is out of range.
    pub fn set_weekday(&mut self, weekday: u8) -> Result<(), Error<E>> {
        if weekday < 1 || weekday > 7 {
            return Err(Error::InvalidInputData);
        }
        self.write_register(Register::DOW, weekday)
    }

    /// Set the day of month [1-31].
    ///
    /// Will return an `Error::InvalidInputData` if the day is out of range.
    pub fn set_day(&mut self, day: u8) -> Result<(), Error<E>> {
        if day < 1 || day > 31 {
            return Err(Error::InvalidInputData);
        }
        self.write_register(Register::DOM, day)
    }

    /// Set the month [1-12].
    ///
    /// Will return an `Error::InvalidInputData` if the month is out of range.
    pub fn set_month(&mut self, month: u8) -> Result<(), Error<E>> {
        if month < 1 || month > 12 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::MONTH, month)
    }

    /// Set the year [2000-2099].
    ///
    /// Will return an `Error::InvalidInputData` if the year is out of range.
    pub fn set_year(&mut self, year: u16) -> Result<(), Error<E>> {
        if year < 2000 || year > 2099 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::YEAR, (year - 2000) as u8)
    }

    /// Set the date and time.
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    pub fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Error<E>> {
        if datetime.year < 2000 || datetime.year > 2099 ||
           datetime.month < 1   || datetime.month > 12  ||
           datetime.day < 1     || datetime.day > 31    ||
           datetime.weekday < 1 || datetime.weekday > 7 ||
           datetime.minute > 59 ||
           datetime.second > 59 {
            return Err(Error::InvalidInputData);
        }
        let ch_flag = self.read_register(Register::SECONDS)? & BitFlags::CH;
        let payload = [
            decimal_to_packed_bcd(datetime.second) | ch_flag,
            decimal_to_packed_bcd(datetime.minute),
            self.get_hours_register_value(datetime.hour)?,
            decimal_to_packed_bcd(datetime.weekday),
            decimal_to_packed_bcd(datetime.day),
            decimal_to_packed_bcd(datetime.month),
            decimal_to_packed_bcd((datetime.year - 2000) as u8),
        ];
        self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2C)
    }

    fn read_register_decimal(&mut self, register: u8) -> Result<u8, Error<E>> {
        let data = self.read_register(register)?;
        Ok(packed_bcd_to_decimal(data))
    }

    fn write_register_decimal(&mut self, register: u8, decimal_number: u8) -> Result<(), Error<E>> {
        self.write_register(register, decimal_to_packed_bcd(decimal_number))
    }
}

fn is_24h_format(hours_data: u8) -> bool {
    hours_data & BitFlags::H24_H12 == 0
}

fn is_am(hours_data: u8) -> bool {
    hours_data & BitFlags::AM_PM == 0
}

fn remove_ch_bit(value: u8) -> u8 {
    value & !BitFlags::CH
}

/// Transforms a number in packed BCD format to decimal
fn packed_bcd_to_decimal(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0xF)
}

/// Transforms a decimal number to packed BCD format
fn decimal_to_packed_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;

    use super::*;

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

    #[test]
    fn can_convert_decimal_to_packed_bcd() {
        assert_eq!(0b0000_0000, decimal_to_packed_bcd( 0));
        assert_eq!(0b0000_0001, decimal_to_packed_bcd( 1));
        assert_eq!(0b0000_1001, decimal_to_packed_bcd( 9));
        assert_eq!(0b0001_0000, decimal_to_packed_bcd(10));
        assert_eq!(0b0001_0001, decimal_to_packed_bcd(11));
        assert_eq!(0b0001_1001, decimal_to_packed_bcd(19));
        assert_eq!(0b0010_0000, decimal_to_packed_bcd(20));
        assert_eq!(0b0010_0001, decimal_to_packed_bcd(21));
        assert_eq!(0b0101_1001, decimal_to_packed_bcd(59));
    }
}
