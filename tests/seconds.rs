extern crate ds1307;

mod common;
use common::{setup, check_sent_data, assert_invalid_input_data_error};

const SECONDS_REGISTER : u8 = 0x00;

#[test]
fn can_read_seconds() {
    let mut rtc = setup(&[0b0101_1001]);
    assert_eq!(59, rtc.get_seconds().unwrap());
    check_sent_data(rtc, &[0]);
}

#[test]
fn ch_bit_is_ignored() {
    let mut rtc = setup(&[0b1101_1001]);
    assert_eq!(59, rtc.get_seconds().unwrap());
}

#[test]
fn wrong_seconds_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_seconds(60));
}

#[test]
fn can_write_seconds() {
    let mut rtc = setup(&[0]);
    rtc.set_seconds(59).unwrap();
    check_sent_data(rtc, &[SECONDS_REGISTER, 0b0101_1001]);
}

#[test]
fn ch_bit_is_kept_when_writing_seconds() {
    let mut rtc = setup(&[0b1000_0000]);
    rtc.set_seconds(59).unwrap();
    check_sent_data(rtc, &[SECONDS_REGISTER, 0b1101_1001]);
}