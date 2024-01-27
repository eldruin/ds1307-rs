use crate::{BitFlags, Ds1307, Error, Register, ADDR};
use embedded_hal::i2c::I2c;
pub use rtcc::{
    DateTimeAccess, Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Rtcc, Timelike,
};

impl<I2C, E> DateTimeAccess for Ds1307<I2C>
where
    I2C: I2c<Error = E>,
{
    type Error = Error<E>;

    fn datetime(&mut self) -> Result<NaiveDateTime, Self::Error> {
        let mut data = [0; 7];
        self.i2c
            .write_read(ADDR, &[0x00], &mut data)
            .map_err(Error::I2C)?;
        let year = 2000 + u16::from(packed_bcd_to_decimal(data[Register::YEAR as usize]));
        let month = packed_bcd_to_decimal(data[Register::MONTH as usize]);
        let day = packed_bcd_to_decimal(data[Register::DOM as usize]);
        let hour = self.get_hours_from_register(data[Register::HOURS as usize])?;
        let minute = packed_bcd_to_decimal(data[Register::MINUTES as usize]);
        let second = packed_bcd_to_decimal(remove_ch_bit(data[Register::SECONDS as usize]));
        let date = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
            .ok_or(Error::InvalidInputData)?;
        date.and_hms_opt(get_h24(hour).into(), minute.into(), second.into())
            .ok_or(Error::InvalidInputData)
    }

    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error> {
        if datetime.year() < 2000 || datetime.year() > 2099 {
            return Err(Error::InvalidInputData);
        }
        let hour = self.get_hours_register_value(Hours::H24(datetime.hour() as u8))?;
        let ch_flag = self.read_register(Register::SECONDS)? & BitFlags::CH;
        let payload = [
            Register::SECONDS,
            decimal_to_packed_bcd(datetime.second() as u8) | ch_flag,
            decimal_to_packed_bcd(datetime.minute() as u8),
            hour,
            datetime.weekday().number_from_sunday() as u8,
            decimal_to_packed_bcd(datetime.day() as u8),
            decimal_to_packed_bcd(datetime.month() as u8),
            decimal_to_packed_bcd((datetime.year() - 2000) as u8),
        ];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }
}

#[allow(clippy::manual_range_contains)]
impl<I2C, E> Rtcc for Ds1307<I2C>
where
    I2C: I2c<Error = E>,
{
    fn seconds(&mut self) -> Result<u8, Self::Error> {
        let data = self.read_register(Register::SECONDS)?;
        Ok(packed_bcd_to_decimal(remove_ch_bit(data)))
    }

    fn minutes(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::MINUTES)
    }

    fn hours(&mut self) -> Result<Hours, Self::Error> {
        let data = self.read_register(Register::HOURS)?;
        self.get_hours_from_register(data)
    }

    fn weekday(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::DOW)
    }

    fn day(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::DOM)
    }

    fn month(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::MONTH)
    }

    fn year(&mut self) -> Result<u16, Self::Error> {
        let year = self.read_register_decimal(Register::YEAR)?;
        Ok(2000 + u16::from(year))
    }

    fn date(&mut self) -> Result<NaiveDate, Self::Error> {
        let mut data = [0; 3];
        self.i2c
            .write_read(ADDR, &[Register::DOM], &mut data)
            .map_err(Error::I2C)?;
        let year = 2000 + u16::from(packed_bcd_to_decimal(data[2]));
        let month = packed_bcd_to_decimal(data[1]);
        let day = packed_bcd_to_decimal(data[0]);
        NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
            .ok_or(Error::InvalidInputData)
    }

    fn time(&mut self) -> Result<NaiveTime, Self::Error> {
        let mut data = [0; 3];
        self.i2c
            .write_read(ADDR, &[Register::SECONDS], &mut data)
            .map_err(Error::I2C)?;
        let hour = self.get_hours_from_register(data[Register::HOURS as usize])?;
        let minute = packed_bcd_to_decimal(data[Register::MINUTES as usize]);
        let second = packed_bcd_to_decimal(remove_ch_bit(data[Register::SECONDS as usize]));
        NaiveTime::from_hms_opt(get_h24(hour).into(), minute.into(), second.into())
            .ok_or(Error::InvalidInputData)
    }

    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error> {
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

    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::MINUTES, minutes)
    }

    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error> {
        let value = self.get_hours_register_value(hours)?;
        self.write_register(Register::HOURS, value)
    }

    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error> {
        if weekday < 1 || weekday > 7 {
            return Err(Error::InvalidInputData);
        }
        self.write_register(Register::DOW, weekday)
    }

    fn set_day(&mut self, day: u8) -> Result<(), Self::Error> {
        if day < 1 || day > 31 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::DOM, day)
    }

    fn set_month(&mut self, month: u8) -> Result<(), Self::Error> {
        if month < 1 || month > 12 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::MONTH, month)
    }

    fn set_year(&mut self, year: u16) -> Result<(), Self::Error> {
        if year < 2000 || year > 2099 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::YEAR, (year - 2000) as u8)
    }

    fn set_date(&mut self, date: &NaiveDate) -> Result<(), Self::Error> {
        if date.year() < 2000 || date.year() > 2099 {
            return Err(Error::InvalidInputData);
        }
        let payload = [
            Register::DOW,
            date.weekday().number_from_sunday() as u8,
            decimal_to_packed_bcd(date.day() as u8),
            decimal_to_packed_bcd(date.month() as u8),
            decimal_to_packed_bcd((date.year() - 2000) as u8),
        ];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }

    fn set_time(&mut self, time: &NaiveTime) -> Result<(), Self::Error> {
        let hour = self.get_hours_register_value(Hours::H24(time.hour() as u8))?;
        let ch_flag = self.read_register(Register::SECONDS)? & BitFlags::CH;
        let payload = [
            Register::SECONDS,
            decimal_to_packed_bcd(time.second() as u8) | ch_flag,
            decimal_to_packed_bcd(time.minute() as u8),
            hour,
        ];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }
}

#[allow(clippy::manual_range_contains)]
impl<I2C, E> Ds1307<I2C>
where
    I2C: I2c<Error = E>,
{
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

fn get_h24(hour: Hours) -> u8 {
    match hour {
        Hours::H24(h) => h,
        Hours::AM(h) => h,
        Hours::PM(h) => h + 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_to_h24() {
        assert_eq!(0, get_h24(Hours::H24(0)));
        assert_eq!(0, get_h24(Hours::AM(0)));
        assert_eq!(12, get_h24(Hours::PM(0)));

        assert_eq!(1, get_h24(Hours::H24(1)));
        assert_eq!(1, get_h24(Hours::AM(1)));
        assert_eq!(13, get_h24(Hours::PM(1)));

        assert_eq!(23, get_h24(Hours::H24(23)));
        assert_eq!(12, get_h24(Hours::AM(12)));
        assert_eq!(23, get_h24(Hours::PM(11)));
    }

    #[test]
    fn can_convert_packed_bcd_to_decimal() {
        assert_eq!(0, packed_bcd_to_decimal(0b0000_0000));
        assert_eq!(1, packed_bcd_to_decimal(0b0000_0001));
        assert_eq!(9, packed_bcd_to_decimal(0b0000_1001));
        assert_eq!(10, packed_bcd_to_decimal(0b0001_0000));
        assert_eq!(11, packed_bcd_to_decimal(0b0001_0001));
        assert_eq!(19, packed_bcd_to_decimal(0b0001_1001));
        assert_eq!(20, packed_bcd_to_decimal(0b0010_0000));
        assert_eq!(21, packed_bcd_to_decimal(0b0010_0001));
        assert_eq!(59, packed_bcd_to_decimal(0b0101_1001));
    }

    #[test]
    fn can_convert_decimal_to_packed_bcd() {
        assert_eq!(0b0000_0000, decimal_to_packed_bcd(0));
        assert_eq!(0b0000_0001, decimal_to_packed_bcd(1));
        assert_eq!(0b0000_1001, decimal_to_packed_bcd(9));
        assert_eq!(0b0001_0000, decimal_to_packed_bcd(10));
        assert_eq!(0b0001_0001, decimal_to_packed_bcd(11));
        assert_eq!(0b0001_1001, decimal_to_packed_bcd(19));
        assert_eq!(0b0010_0000, decimal_to_packed_bcd(20));
        assert_eq!(0b0010_0001, decimal_to_packed_bcd(21));
        assert_eq!(0b0101_1001, decimal_to_packed_bcd(59));
    }
}
