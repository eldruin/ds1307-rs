extern crate ds1307;
extern crate embedded_hal_mock as hal;

use self::ds1307::{DS1307, Error};

const DEVICE_ADDRESS: u8 = 0b110_1000;

pub fn setup<'a>(read_data: &'a [u8]) -> DS1307<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&read_data);
    DS1307::new(dev)
}

pub fn check_sent_data(rtc: DS1307<hal::I2cMock>, data: &[u8]) {
    let dev = rtc.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
}

pub fn assert_invalid_input_data_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidInputData) => (),
        _ => panic!(),
    }
}
