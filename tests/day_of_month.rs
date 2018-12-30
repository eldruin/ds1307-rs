extern crate ds1307;

mod common;
use common::{setup, check_sent_data, assert_invalid_input_data_error};

const DOM_REGISTER : u8 = 0x04;

#[test]
fn can_read_day_of_month() {
    let mut rtc = setup(&[0b0011_0001]);
    assert_eq!(31, rtc.get_day().unwrap());
    check_sent_data(rtc, &[DOM_REGISTER]);
}

#[test]
fn too_small_day_of_month_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_day(0));
}

#[test]
fn too_big_day_of_month_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_day(32));
}

#[test]
fn can_write_day_of_month() {
    let mut rtc = setup(&[0]);
    rtc.set_day(7).unwrap();
    check_sent_data(rtc, &[DOM_REGISTER, 0b0000_0111]);
}
