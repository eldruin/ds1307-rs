extern crate ds1307;
use ds1307::SQWOUTRateBits;

#[allow(dead_code)]
mod common;
use common::{setup, check_sent_data};

const SQWOUT_REGISTER : u8 = 0x07;

#[test]
fn can_read_output_rate() {
    let mut rtc = setup(&[0b0000_0010]);
    let rate = rtc.get_square_wave_output_rate().unwrap();
    assert_eq!(false, rate.rs0);
    assert_eq!(true, rate.rs1);
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn can_set_output_rate_to_11() {
    let mut rtc = setup(&[0b0000_0010]);
    rtc.set_square_wave_output_rate(SQWOUTRateBits {
        rs0: true,
        rs1: true
    }).unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b0000_0011]);
}

#[test]
fn can_set_output_rate_00() {
    let mut rtc = setup(&[0b0000_0001]);
    rtc.set_square_wave_output_rate(SQWOUTRateBits {
        rs0: false,
        rs1: false
    }).unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b0000_0000]);
}

#[test]
fn set_output_rate_to_same_rate_does_nothing() {
    let mut rtc = setup(&[0b0000_0001]);
    rtc.set_square_wave_output_rate(SQWOUTRateBits {
        rs0: true,
        rs1: false
    }).unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER]);
}

#[test]
fn set_output_rate_keeps_status_of_other_flags() {
    let mut rtc = setup(&[0b1001_0011]);
    rtc.set_square_wave_output_rate(SQWOUTRateBits {
        rs0: false,
        rs1: false
    }).unwrap();
    check_sent_data(rtc, &[SQWOUT_REGISTER, 0b1001_0000]);
}
