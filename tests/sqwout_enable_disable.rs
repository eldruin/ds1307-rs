extern crate ds1307;

#[allow(dead_code)]
mod common;
use common::{check_sent_data, setup};

const SQWOUT_REGISTER: u8 = 0x07;

#[test]
fn can_read_is_not_enabled() {
    let mut rtc = setup(&[0b0000_0000]);
    assert!(!rtc.is_square_wave_output_enabled().unwrap());
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_read_is_enabled() {
    let mut rtc = setup(&[0b0001_0000]);
    assert!(rtc.is_square_wave_output_enabled().unwrap());
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_enable() {
    let mut rtc = setup(&[0]);
    rtc.enable_square_wave_output().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b0001_0000]);
}

#[test]
fn enable_keeps_status_of_other_flags() {
    let mut rtc = setup(&[0b1000_0011]);
    rtc.enable_square_wave_output().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b1001_0011]);
}

#[test]
fn when_already_enabled_then_enable_does_nothing() {
    let mut rtc = setup(&[0b0001_0000]);
    rtc.enable_square_wave_output().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_disable() {
    let mut rtc = setup(&[0b0001_0000]);
    rtc.disable_square_wave_output().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0]);
}

#[test]
fn disable_keeps_status_of_other_flags() {
    let mut rtc = setup(&[0b1001_0011]);
    rtc.disable_square_wave_output().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b1000_0011]);
}

#[test]
fn when_already_disabled_then_disable_does_nothing() {
    let mut rtc = setup(&[0]);
    rtc.disable_square_wave_output().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}
