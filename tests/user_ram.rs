extern crate ds1307;

mod common;
use common::{assert_invalid_input_data_error, check_sent_data, setup};

const RAM_REGISTER_BEGIN: u8 = 0x08;
const RAM_BYTE_COUNT: usize = 56;

#[test]
fn too_long_array_read_returns_error() {
    let mut rtc = setup(&[0]);
    let mut data = [0; RAM_BYTE_COUNT + 1];
    assert_invalid_input_data_error(rtc.read_ram(0, &mut data));
}

#[test]
fn too_big_address_offset_read_returns_error() {
    let mut rtc = setup(&[0]);
    let mut data = [0];
    assert_invalid_input_data_error(rtc.read_ram(RAM_BYTE_COUNT as u8, &mut data));
}

#[test]
fn too_much_data_read_returns_error() {
    let mut rtc = setup(&[0]);
    let mut data = [0; RAM_BYTE_COUNT];
    assert_invalid_input_data_error(rtc.read_ram(1, &mut data));
}

#[test]
fn empty_data_read_does_nothing() {
    let mut rtc = setup(&[0]);
    let mut data = [];
    rtc.read_ram(0, &mut data).unwrap();
    let dev = rtc.destroy();
    assert_eq!(dev.get_last_address(), None);
    assert_eq!(dev.get_write_data(), &[]);
}

#[test]
fn can_read_whole_ram() {
    let mut rtc = setup(&[0xAB; RAM_BYTE_COUNT]);
    let mut data = [0; RAM_BYTE_COUNT];
    rtc.read_ram(0, &mut data).unwrap();
    check_sent_data(rtc, &[RAM_REGISTER_BEGIN]);
    for d in data.iter() {
        assert_eq!(0xAB, *d);
    }
}

#[test]
fn can_read_last_ram_address() {
    let mut rtc = setup(&[0xAB]);
    let mut data = [0];
    rtc.read_ram(RAM_BYTE_COUNT as u8 - 1, &mut data).unwrap();
    check_sent_data(rtc, &[RAM_REGISTER_BEGIN + RAM_BYTE_COUNT as u8 - 1]);
    assert_eq!(0xAB, data[0]);
}

#[test]
fn too_long_array_write_returns_error() {
    let mut rtc = setup(&[0]);
    let mut data = [0; RAM_BYTE_COUNT + 1];
    assert_invalid_input_data_error(rtc.write_ram(0, &mut data));
}

#[test]
fn too_big_address_offset_write_returns_error() {
    let mut rtc = setup(&[0]);
    let mut data = [0];
    assert_invalid_input_data_error(rtc.write_ram(RAM_BYTE_COUNT as u8, &mut data));
}

#[test]
fn too_much_data_write_returns_error() {
    let mut rtc = setup(&[0]);
    let mut data = [0; RAM_BYTE_COUNT];
    assert_invalid_input_data_error(rtc.write_ram(1, &mut data));
}

#[test]
fn empty_data_write_does_nothing() {
    let mut rtc = setup(&[0]);
    rtc.write_ram(0, &[]).unwrap();
    let dev = rtc.destroy();
    assert_eq!(dev.get_last_address(), None);
    assert_eq!(dev.get_write_data(), &[]);
}

#[test]
fn can_write_whole_ram() {
    let mut rtc = setup(&[0]);
    let mut data = [0xAB; RAM_BYTE_COUNT];
    rtc.write_ram(0, &mut data).unwrap();
    let mut output = [0xAB; RAM_BYTE_COUNT + 1];
    output[0] = RAM_REGISTER_BEGIN;
    check_sent_data(rtc, &output);
}

#[test]
fn can_write_last_ram_address() {
    let mut rtc = setup(&[0xAB]);
    let mut data = [0xAB];
    rtc.write_ram(RAM_BYTE_COUNT as u8 - 1, &mut data).unwrap();
    check_sent_data(rtc, &[RAM_REGISTER_BEGIN + RAM_BYTE_COUNT as u8 - 1, 0xAB]);
}
