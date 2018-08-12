extern crate ds1307;
use ds1307::{Hours, Error};

mod common;
use common::{setup, check_sent_data};

const HOURS_REGISTER : u8 = 0x02;

#[test]
fn can_read_24h_hours() {
    let mut rtc = setup(&[0b0010_0011]);
    match rtc.get_hours().unwrap() {
        Hours::H24(h) => assert_eq!(23, h),
        _ => panic!(),
    }
    check_sent_data(rtc, &[HOURS_REGISTER]);
}

#[test]
fn wrong_24h_hours_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_hours(Hours::H24(24)) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_24h_hours() {
    let mut rtc = setup(&[0]);
    rtc.set_hours(Hours::H24(23)).unwrap();
    check_sent_data(rtc, &[HOURS_REGISTER, 0b0010_0011]);
}

#[test]
fn can_read_12h_am_hours() {
    let mut rtc = setup(&[0b0101_0010]);
    match rtc.get_hours().unwrap() {
        Hours::AM(h) => assert_eq!(12, h),
        _ => panic!(),
    }
    check_sent_data(rtc, &[HOURS_REGISTER]);
}

#[test]
fn too_small_12h_am_hours_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_hours(Hours::AM(0)) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn too_big_12h_am_hours_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_hours(Hours::AM(13)) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_12h_am_hours() {
    let mut rtc = setup(&[0]);
    rtc.set_hours(Hours::AM(12)).unwrap();
    check_sent_data(rtc, &[HOURS_REGISTER, 0b0101_0010]);
}

#[test]
fn can_read_12h_pm_hours() {
    let mut rtc = setup(&[0b0111_0010]);
    match rtc.get_hours().unwrap() {
        Hours::PM(h) => assert_eq!(12, h),
        _ => panic!(),
    }
    check_sent_data(rtc, &[HOURS_REGISTER]);
}

#[test]
fn too_small_12h_pm_hours_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_hours(Hours::PM(0)) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn too_big_12h_pm_hours_returns_error() {
    let mut rtc = setup(&[0]);
    match rtc.set_hours(Hours::PM(13)) {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}

#[test]
fn can_write_12h_pm_hours() {
    let mut rtc = setup(&[0]);
    rtc.set_hours(Hours::PM(12)).unwrap();
    check_sent_data(rtc, &[HOURS_REGISTER, 0b0111_0010]);
}