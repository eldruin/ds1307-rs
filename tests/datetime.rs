use ds1307::{Error, NaiveDate, NaiveDateTime, NaiveTime, Rtcc};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

fn get_valid_datetime() -> NaiveDateTime {
    NaiveDate::from_ymd(2018, 8, 13).and_hms(23, 59, 58)
}

#[test]
fn get_datetime() {
    let mut dev = new(&trans_read!(
        SECONDS,
        [
            0b1101_1000,
            0b0101_1001,
            0b0010_0011,
            0b0000_0010,
            0b0001_0011,
            0b0000_1000,
            0b0001_1000
        ]
    ));
    assert_eq!(get_valid_datetime(), dev.get_datetime().unwrap());
    destroy(dev);
}

#[test]
fn get_date() {
    let mut dev = new(&trans_read!(DOM, [0b0001_0011, 0b0000_1000, 0b0001_1000]));
    assert_eq!(NaiveDate::from_ymd(2018, 8, 13), dev.get_date().unwrap());
    destroy(dev);
}

#[test]
fn get_time() {
    let mut dev = new(&trans_read!(
        SECONDS,
        [0b1101_1000, 0b0101_1001, 0b0010_0011]
    ));
    assert_eq!(NaiveTime::from_hms(23, 59, 58), dev.get_time().unwrap());
    destroy(dev);
}

set_invalid_test!(
    year_too_small,
    set_datetime,
    &NaiveDate::from_ymd(1999, 1, 1).and_hms(1, 1, 1)
);
set_invalid_test!(
    year_too_big,
    set_datetime,
    &NaiveDate::from_ymd(2100, 1, 1).and_hms(1, 1, 1)
);

#[test]
fn can_set_datetime() {
    let mut rtc = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SECONDS], vec![0b1101_1000]),
        I2cTrans::write(
            ADDR,
            vec![
                Register::SECONDS,
                0b1101_1000,
                0b0101_1001,
                0b0010_0011,
                0b0000_0010,
                0b0001_0011,
                0b0000_1000,
                0b0001_1000,
            ],
        ),
    ]);
    let dt = get_valid_datetime();
    rtc.set_datetime(&dt).unwrap();
    destroy(rtc);
}

#[test]
fn can_set_time() {
    let mut rtc = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SECONDS], vec![0b1101_1000]),
        I2cTrans::write(
            ADDR,
            vec![Register::SECONDS, 0b1101_1000, 0b0101_1001, 0b0010_0011],
        ),
    ]);
    rtc.set_time(&NaiveTime::from_hms(23, 59, 58)).unwrap();
    destroy(rtc);
}

#[test]
fn can_set_date() {
    let mut rtc = new(&trans_write!(
        DOW,
        [0b0000_0010, 0b0001_0011, 0b0000_1000, 0b0001_1000]
    ));
    rtc.set_date(&NaiveDate::from_ymd(2018, 8, 13)).unwrap();
    destroy(rtc);
}

macro_rules! individual_test {
    ($name:ident, $register:ident, $get_method:ident,
        $set_method:ident, $value:expr, $bin_value:expr, $too_small:expr, $too_big:expr
     ) => {
        mod $name {
            use super::*;

            get_test!(
                get,
                $get_method,
                $value,
                trans_read!($register, [$bin_value])
            );
            set_test!(
                set,
                $set_method,
                $value,
                trans_write!($register, [$bin_value])
            );
            set_invalid_test!(too_small, $set_method, $too_small);
            set_invalid_test!(too_big, $set_method, $too_big);
        }
    };
}

individual_test!(day_of_month, DOM, get_day, set_day, 31, 0b0011_0001, 0, 32);
individual_test!(day_of_week, DOW, get_weekday, set_weekday, 7, 7, 0, 8);
individual_test!(month, MONTH, get_month, set_month, 12, 0b0001_0010, 0, 13);
individual_test!(
    year,
    YEAR,
    get_year,
    set_year,
    2099,
    0b1001_1001,
    1999,
    2100
);

mod minutes {
    use super::*;
    get_test!(get, get_minutes, 59, trans_read!(MINUTES, [0b0101_1001]));
    set_invalid_test!(wrong, set_minutes, 60);
    set_test!(set, set_minutes, 59, trans_write!(MINUTES, [0b0101_1001]));
}
