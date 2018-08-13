extern crate ds1307;
use ds1307::Error;

mod common;
use common::{setup, check_sent_data};

const DOW_REGISTER : u8 = 0x03;

#[test]
fn can_read_weekday() {
    let mut rtc = setup(&[7]);
    assert_eq!(7, rtc.get_weekday().unwrap());
    check_sent_data(rtc, &[DOW_REGISTER]);
}

#[test]
fn too_small_weekday_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_weekday(0) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn too_big_weekday_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_weekday(8) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_weekday() {
    let mut rtc = setup(&[0]);
    rtc.set_weekday(7).unwrap();
    check_sent_data(rtc, &[DOW_REGISTER, 0b0000_0111]);
}

