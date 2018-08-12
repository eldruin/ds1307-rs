extern crate ds1307;
use ds1307::Error;

mod common;
use common::{setup, check_sent_data};

const DOW_REGISTER : u8 = 0x03;

#[test]
fn can_read_day_of_week() {
    let mut rtc = setup(&[7]);
    assert_eq!(7, rtc.get_day_of_week().unwrap());
    check_sent_data(rtc, &[DOW_REGISTER]);
}

#[test]
fn too_small_day_of_week_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_day_of_week(0) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn too_big_day_of_week_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_day_of_week(8) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_day_of_week() {
    let mut rtc = setup(&[0]);
    rtc.set_day_of_week(7).unwrap();
    check_sent_data(rtc, &[DOW_REGISTER, 0b0000_0111]);
}

