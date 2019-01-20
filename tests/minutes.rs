extern crate ds1307;

mod common;
use common::{assert_invalid_input_data_error, check_sent_data, setup};

const MINUTES_REGISTER: u8 = 0x01;

#[test]
fn can_read_minutes() {
    let mut rtc = setup(&[0b0101_1001]);
    assert_eq!(59, rtc.get_minutes().unwrap());
    check_sent_data(rtc, &[MINUTES_REGISTER]);
}

#[test]
fn wrong_minutes_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_minutes(60));
}

#[test]
fn can_write_minutes() {
    let mut rtc = setup(&[0]);
    rtc.set_minutes(59).unwrap();
    check_sent_data(rtc, &[MINUTES_REGISTER, 0b0101_1001]);
}
