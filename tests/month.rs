extern crate ds1307;
use ds1307::Error;

mod common;
use common::{setup, check_sent_data};

const MONTH_REGISTER : u8 = 0x05;

#[test]
fn can_read_month() {
    let mut rtc = setup(&[0b0001_0010]);
    assert_eq!(12, rtc.get_month().unwrap());
    check_sent_data(rtc, &[MONTH_REGISTER]);
}

#[test]
fn too_small_month_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_month(0) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn too_big_month_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_month(13) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_month() {
    let mut rtc = setup(&[0]);
    rtc.set_month(12).unwrap();
    check_sent_data(rtc, &[MONTH_REGISTER, 0b0001_0010]);
}