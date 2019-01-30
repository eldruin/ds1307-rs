extern crate ds1307;
use ds1307::{DateTime, Hours};

mod common;
use common::{assert_invalid_input_data_error, check_sent_data, setup};

#[test]
fn can_read_datetime() {
    let mut rtc = setup(&[
        0b1101_1000,
        0b0101_1001,
        0b0010_0011,
        0b0000_0010,
        0b0001_0011,
        0b0000_1000,
        0b0001_1000,
    ]);
    let datetime = rtc.get_datetime().unwrap();
    assert_eq!(2018, datetime.year);
    assert_eq!(08, datetime.month);
    assert_eq!(13, datetime.day);
    assert_eq!(2, datetime.weekday);
    check_hours!(datetime.hour, H24, 23);
    assert_eq!(59, datetime.minute);
    assert_eq!(58, datetime.second);
    check_sent_data(rtc, &[0x00]);
}

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
    let mut rtc = setup(&[0]);
    let mut dt = get_valid_datetime();
    f(&mut dt);
    assert_invalid_input_data_error(rtc.set_datetime(&dt));
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
fn when_set_datetime_ch_bit_is_unchanged() {
    let mut rtc = setup(&[0b1101_1000]);
    let dt = get_valid_datetime();
    rtc.set_datetime(&dt).unwrap();
    let dev = rtc.destroy();
    assert_eq!(dev.get_write_data()[0], 0b1101_1000);
}

#[test]
fn can_set_datetime() {
    let mut rtc = setup(&[0b1101_1000]);
    let dt = get_valid_datetime();
    rtc.set_datetime(&dt).unwrap();
    check_sent_data(
        rtc,
        &[
            0b1101_1000,
            0b0101_1001,
            0b0010_0011,
            0b0000_0111,
            0b0011_0001,
            0b0001_0010,
            0b1001_1001,
        ],
    );
}
