extern crate ds1307;
use ds1307::{DateTime, Error, Hours};
extern crate embedded_hal_mock as hal;
use self::hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, Register, ADDR};

const DT: DateTime = DateTime {
    year: 2018,
    month: 8,
    day: 13,
    weekday: 2,
    hour: Hours::H24(23),
    minute: 59,
    second: 58,
};

get_test!(
    read_datetime,
    get_datetime,
    DT,
    trans_read!(
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
    )
);

fn get_valid_datetime() -> DateTime {
    DateTime {
        year: 2099,
        month: 12,
        day: 31,
        weekday: 7,
        hour: Hours::H24(23),
        minute: 59,
        second: 58,
    }
}

fn check_set_datetime<F>(mut f: F)
where
    F: FnMut(&mut DateTime),
{
    let mut rtc = new(&[]);
    let mut dt = get_valid_datetime();
    f(&mut dt);
    assert_invalid_input_data!(rtc.set_datetime(&dt));
    destroy(rtc);
}

#[test]
fn wrong_year_returns_error() {
    check_set_datetime(|ref mut dt| dt.year = 1999);
    check_set_datetime(|ref mut dt| dt.year = 2100);
}

#[test]
fn wrong_month_returns_error() {
    check_set_datetime(|ref mut dt| dt.month = 0);
    check_set_datetime(|ref mut dt| dt.month = 13);
}

#[test]
fn wrong_day_returns_error() {
    check_set_datetime(|ref mut dt| dt.day = 0);
    check_set_datetime(|ref mut dt| dt.day = 32);
}

#[test]
fn wrong_weekday_returns_error() {
    check_set_datetime(|ref mut dt| dt.weekday = 0);
    check_set_datetime(|ref mut dt| dt.weekday = 8);
}

#[test]
fn wrong_hour_returns_error() {
    check_set_datetime(|ref mut dt| dt.hour = Hours::H24(24));
    check_set_datetime(|ref mut dt| dt.hour = Hours::AM(0));
    check_set_datetime(|ref mut dt| dt.hour = Hours::AM(13));
    check_set_datetime(|ref mut dt| dt.hour = Hours::PM(0));
    check_set_datetime(|ref mut dt| dt.hour = Hours::PM(13));
}

#[test]
fn wrong_minute_returns_error() {
    check_set_datetime(|ref mut dt| dt.minute = 60);
}

#[test]
fn wrong_second_returns_error() {
    check_set_datetime(|ref mut dt| dt.second = 60);
}

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
                0b0000_0111,
                0b0011_0001,
                0b0001_0010,
                0b1001_1001,
            ],
        ),
    ]);
    let dt = get_valid_datetime();
    rtc.set_datetime(&dt).unwrap();
    destroy(rtc);
}
