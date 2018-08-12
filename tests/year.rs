extern crate ds1307;
use ds1307::Error;

mod common;
use common::{setup, check_sent_data};

const YEAR_REGISTER : u8 = 0x06;

#[test]
fn can_read_year() {
    let mut rtc = setup(&[0b1001_1001]);
    assert_eq!(2099, rtc.get_year().unwrap());
    check_sent_data(rtc, &[YEAR_REGISTER]);
}

#[test]
fn too_small_year_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_year(1999) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn too_big_year_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_year(2100) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_year() {
    let mut rtc = setup(&[0]);
    rtc.set_year(2099).unwrap();
    check_sent_data(rtc, &[YEAR_REGISTER, 0b1001_1001]);
}