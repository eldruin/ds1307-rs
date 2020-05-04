use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(
    sqwout_enabled,
    square_wave_output_enabled,
    true,
    trans_read!(SQWOUT, [0b0001_0000])
);
get_test!(
    sqwout_disabled,
    square_wave_output_enabled,
    false,
    trans_read!(SQWOUT, [0])
);

#[test]
fn enable() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SQWOUT], vec![0b1000_0011]),
        I2cTrans::write(ADDR, vec![Register::SQWOUT, 0b1001_0011]),
    ]);
    dev.enable_square_wave_output().unwrap();
    destroy(dev);
}

#[test]
fn when_already_enabled_then_enable_does_nothing() {
    let mut dev = new(&trans_read!(SQWOUT, [0b0001_0000]));
    dev.enable_square_wave_output().unwrap();
    destroy(dev);
}

#[test]
fn disable() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SQWOUT], vec![0b1001_0011]),
        I2cTrans::write(ADDR, vec![Register::SQWOUT, 0b1000_0011]),
    ]);
    dev.disable_square_wave_output().unwrap();
    destroy(dev);
}

#[test]
fn when_already_disabled_then_disable_does_nothing() {
    let mut dev = new(&trans_read!(SQWOUT, [0]));
    dev.disable_square_wave_output().unwrap();
    destroy(dev);
}
