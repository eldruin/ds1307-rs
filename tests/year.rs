extern crate ds1307;

mod common;
use common::{assert_invalid_input_data_error, check_sent_data, setup};

const YEAR_REGISTER: u8 = 0x06;

#[test]
fn can_read_year() {
    let mut rtc = setup(&[0b1001_1001]);
    assert_eq!(2099, rtc.get_year().unwrap());
    check_sent_data(rtc, &[YEAR_REGISTER]);
}

#[test]
fn too_small_year_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_year(1999));
}

#[test]
fn too_big_year_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_year(2100));
}

#[test]
fn can_write_year() {
    let mut rtc = setup(&[0]);
    rtc.set_year(2099).unwrap();
    check_sent_data(rtc, &[YEAR_REGISTER, 0b1001_1001]);
}
