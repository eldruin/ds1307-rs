extern crate ds1307;

#[allow(dead_code)]
mod common;
use common::{check_sent_data, setup};

const SQWOUT_REGISTER: u8 = 0x07;

#[test]
fn can_read_output_level_low() {
    let mut rtc = setup(&[0]);
    assert!(!rtc.get_square_wave_output_level().unwrap());
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_read_output_level_high() {
    let mut rtc = setup(&[0b1000_0000]);
    assert!(rtc.get_square_wave_output_level().unwrap());
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_write_low_output_level() {
    let mut rtc = setup(&[0b1000_0000]);
    rtc.set_square_wave_output_level_low().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0]);
}

#[test]
fn when_already_low_then_set_level_low_does_nothing() {
    let mut rtc = setup(&[0]);
    rtc.set_square_wave_output_level_low().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_write_high_output_level() {
    let mut rtc = setup(&[0]);
    rtc.set_square_wave_output_level_high().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b1000_0000]);
}

#[test]
fn when_already_high_then_set_level_low_does_nothing() {
    let mut rtc = setup(&[0b1000_0000]);
    rtc.set_square_wave_output_level_high().unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_write_output_level_parameter_low() {
    let mut rtc = setup(&[0b1000_0000]);
    rtc.set_square_wave_output_level(false).unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0]);
}

#[test]
fn can_write_output_level_parameter_high() {
    let mut rtc = setup(&[0]);
    rtc.set_square_wave_output_level(true).unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b1000_0000]);
}

#[test]
fn write_output_level_keeps_status_of_other_flags() {
    let mut rtc = setup(&[0b1001_0011]);
    rtc.set_square_wave_output_level(false).unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b0001_0011]);
}
