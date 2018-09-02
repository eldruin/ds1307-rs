extern crate ds1307;

#[allow(dead_code)]
mod common;
use common::{setup, check_sent_data};

const ENABLE_REGISTER : u8 = 0x00;

#[test]
fn can_read_is_running_running() {
    let mut rtc = setup(&[0b1000_0000]);
    assert!(rtc.is_running().unwrap());
    check_sent_data(rtc, &[ENABLE_REGISTER]);
}

#[test]
fn can_read_is_running_halted() {
    let mut rtc = setup(&[0]);
    assert!(!rtc.is_running().unwrap());
    check_sent_data(rtc, &[ENABLE_REGISTER]);
}

#[test]
fn can_set_running() {
    let mut rtc = setup(&[0]);
    rtc.set_running().unwrap();
    check_sent_data(rtc, &[ENABLE_REGISTER, 0b1000_0000]);
}

#[test]
fn can_halt() {
    let mut rtc = setup(&[0]);
    rtc.halt().unwrap();
    check_sent_data(rtc, &[ENABLE_REGISTER, 0b0000_0000]);
}

#[test]
fn run_keeps_seconds() {
    let mut rtc = setup(&[0b0101_0101]);
    rtc.set_running().unwrap();
    check_sent_data(rtc, &[ENABLE_REGISTER, 0b1101_0101]);
}

#[test]
fn halt_keeps_seconds() {
    let mut rtc = setup(&[0b1101_0101]);
    rtc.halt().unwrap();
    check_sent_data(rtc, &[ENABLE_REGISTER, 0b0101_0101]);
}


