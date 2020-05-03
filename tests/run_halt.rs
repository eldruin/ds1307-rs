use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(
    is_running,
    is_running,
    true,
    trans_read!(SECONDS, [0b1000_0000])
);
get_test!(not_running, is_running, false, trans_read!(SECONDS, [0]));

#[test]
fn can_set_running() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SECONDS], vec![0b0101_0101]),
        I2cTrans::write(ADDR, vec![Register::SECONDS, 0b1101_0101]),
    ]);
    dev.set_running().unwrap();
    destroy(dev);
}

#[test]
fn set_running_when_already_running_does_nothing() {
    let mut dev = new(&trans_read!(SECONDS, [0b1000_0000]));
    dev.set_running().unwrap();
    destroy(dev);
}

#[test]
fn can_halt() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SECONDS], vec![0b1101_0101]),
        I2cTrans::write(ADDR, vec![Register::SECONDS, 0b0101_0101]),
    ]);
    dev.halt().unwrap();
    destroy(dev);
}

#[test]
fn halt_when_already_halted_does_nothing() {
    let mut dev = new(&trans_read!(SECONDS, [0]));
    dev.halt().unwrap();
    destroy(dev);
}
