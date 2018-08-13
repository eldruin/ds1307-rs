extern crate ds1307;
use ds1307::Error;

mod common;
use common::{setup, check_sent_data};

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
    match rtc.set_day(0) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn too_big_day_of_month_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_day(8) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_day_of_month() {
    let mut rtc = setup(&[0]);
    rtc.set_day(7).unwrap();
    check_sent_data(rtc, &[DOM_REGISTER, 0b0000_0111]);
}
