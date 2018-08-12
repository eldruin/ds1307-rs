extern crate ds1307;
use ds1307::Error;

mod common;
use common::{setup, check_sent_data};

const MINUTES_REGISTER : u8 = 0x01;

#[test]
fn can_read_minutes() {
    let mut rtc = setup(&[0b0101_1001]);
    assert_eq!(59, rtc.get_minutes().unwrap());
    check_sent_data(rtc, &[MINUTES_REGISTER]);
}

#[test]
fn wrong_minutes_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_minutes(60) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_minutes() {
    let mut rtc = setup(&[0]);
    rtc.set_minutes(59).unwrap();
    check_sent_data(rtc, &[MINUTES_REGISTER, 0b0101_1001]);
}