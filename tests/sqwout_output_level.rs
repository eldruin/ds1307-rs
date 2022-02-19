use ds1307::SqwOutLevel;
use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(
    get_high,
    square_wave_output_level,
    SqwOutLevel::High,
    trans_read!(SQWOUT, [0b1000_0000])
);
get_test!(
    get_low,
    square_wave_output_level,
    SqwOutLevel::Low,
    trans_read!(SQWOUT, [0])
);

#[test]
fn set_low() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SQWOUT], vec![0b1001_0011]),
        I2cTrans::write(ADDR, vec![Register::SQWOUT, 0b0001_0011]),
    ]);
    dev.set_square_wave_output_level(SqwOutLevel::Low).unwrap();
    destroy(dev);
}

#[test]
fn set_high() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SQWOUT], vec![0b0001_0011]),
        I2cTrans::write(ADDR, vec![Register::SQWOUT, 0b1001_0011]),
    ]);
    dev.set_square_wave_output_level(SqwOutLevel::High).unwrap();
    destroy(dev);
}

#[test]
fn set_does_nothing_if_matches() {
    let mut dev = new(&trans_read!(SQWOUT, [0b1001_0011]));
    dev.set_square_wave_output_level(SqwOutLevel::High).unwrap();
    destroy(dev);
}
