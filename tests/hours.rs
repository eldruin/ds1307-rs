extern crate ds1307;
use ds1307::Hours;

mod common;
use common::{assert_invalid_input_data_error, check_sent_data, setup};

const HOURS_REGISTER: u8 = 0x02;

#[test]
fn can_read_24h_hours() {
    let mut rtc = setup(&[0b0010_0011]);
    check_hours!(rtc.get_hours().unwrap(), H24, 23);
    check_sent_data(rtc, &[HOURS_REGISTER]);
}

#[test]
fn wrong_24h_hours_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_hours(Hours::H24(24)));
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
    check_hours!(rtc.get_hours().unwrap(), AM, 12);
    check_sent_data(rtc, &[HOURS_REGISTER]);
}

#[test]
fn too_small_12h_am_hours_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_hours(Hours::AM(0)));
}

#[test]
fn too_big_12h_am_hours_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_hours(Hours::AM(13)));
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
    check_hours!(rtc.get_hours().unwrap(), PM, 12);
    check_sent_data(rtc, &[HOURS_REGISTER]);
}

#[test]
fn too_small_12h_pm_hours_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_hours(Hours::PM(0)));
}

#[test]
fn too_big_12h_pm_hours_returns_error() {
    let mut rtc = setup(&[0]);
    assert_invalid_input_data_error(rtc.set_hours(Hours::PM(13)));
}

#[test]
fn can_write_12h_pm_hours() {
    let mut rtc = setup(&[0]);
    rtc.set_hours(Hours::PM(12)).unwrap();
    check_sent_data(rtc, &[HOURS_REGISTER, 0b0111_0010]);
}
