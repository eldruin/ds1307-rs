extern crate ds1307;
extern crate embedded_hal_mock as hal;
use self::ds1307::Error;
use self::hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, ADDR};

const RAM_BEGIN: u8 = 0x08;
const RAM_BYTE_COUNT: usize = 56;

set_invalid_test!(
    read_too_much_data,
    read_ram,
    0,
    &mut [0; RAM_BYTE_COUNT + 1]
);
set_invalid_test!(
    read_too_big_offset,
    read_ram,
    RAM_BYTE_COUNT as u8,
    &mut [0]
);
set_invalid_test!(read_overflow, read_ram, 1, &mut [0; RAM_BYTE_COUNT]);

#[test]
fn empty_data_read_does_nothing() {
    let mut rtc = new(&[]);
    let mut data = [];
    rtc.read_ram(0, &mut data).unwrap();
    destroy(rtc);
}

#[test]
fn can_read_whole_ram() {
    let mut rtc = new(&[I2cTrans::write_read(
        ADDR,
        vec![RAM_BEGIN],
        vec![0xAB; RAM_BYTE_COUNT],
    )]);
    let mut data = [0; RAM_BYTE_COUNT];
    rtc.read_ram(0, &mut data).unwrap();
    for d in data.iter() {
        assert_eq!(0xAB, *d);
    }
    destroy(rtc);
}

#[test]
fn can_read_last_ram_address() {
    let mut rtc = new(&[I2cTrans::write_read(
        ADDR,
        vec![RAM_BEGIN + RAM_BYTE_COUNT as u8 - 1],
        vec![0xAB],
    )]);
    let mut data = [0];
    rtc.read_ram(RAM_BYTE_COUNT as u8 - 1, &mut data).unwrap();
    assert_eq!(0xAB, data[0]);
    destroy(rtc);
}

set_invalid_test!(
    write_too_much_data,
    write_ram,
    0,
    &mut [0; RAM_BYTE_COUNT + 1]
);
set_invalid_test!(
    write_too_big_offset,
    write_ram,
    RAM_BYTE_COUNT as u8,
    &mut [0]
);
set_invalid_test!(write_overflow, write_ram, 1, &mut [0; RAM_BYTE_COUNT]);

#[test]
fn empty_data_write_does_nothing() {
    let mut rtc = new(&[]);
    let mut data = [];
    rtc.write_ram(0, &mut data).unwrap();
    destroy(rtc);
}

#[test]
fn can_write_whole_ram() {
    let mut expected = [0xAB; RAM_BYTE_COUNT + 1];
    expected[0] = RAM_BEGIN;
    let mut rtc = new(&[I2cTrans::write(ADDR, expected.to_vec())]);
    let data = [0xAB; RAM_BYTE_COUNT];
    rtc.write_ram(0, &data).unwrap();
    destroy(rtc);
}

#[test]
fn can_write_last_ram_address() {
    let mut rtc = new(&[I2cTrans::write(
        ADDR,
        vec![RAM_BEGIN + RAM_BYTE_COUNT as u8 - 1, 0xAB],
    )]);
    let data = [0xAB];
    rtc.write_ram(RAM_BYTE_COUNT as u8 - 1, &data).unwrap();
    destroy(rtc);
}
