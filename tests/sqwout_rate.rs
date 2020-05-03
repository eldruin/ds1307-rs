extern crate ds1307;
use ds1307::SQWOUTRateBits;
extern crate embedded_hal_mock as hal;
use self::hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, Register, ADDR};

get_test!(
    get_01,
    get_square_wave_output_rate,
    SQWOUTRateBits {
        rs0: false,
        rs1: true
    },
    trans_read!(SQWOUT, [0b0000_0010])
);

#[test]
fn set_00() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SQWOUT], vec![0b1001_0010]),
        I2cTrans::write(ADDR, vec![Register::SQWOUT, 0b1001_0000]),
    ]);
    dev.set_square_wave_output_rate(SQWOUTRateBits {
        rs0: false,
        rs1: false,
    })
    .unwrap();
    destroy(dev);
}

#[test]
fn set_11() {
    let mut dev = new(&[
        I2cTrans::write_read(ADDR, vec![Register::SQWOUT], vec![0b1001_0010]),
        I2cTrans::write(ADDR, vec![Register::SQWOUT, 0b1001_0011]),
    ]);
    dev.set_square_wave_output_rate(SQWOUTRateBits {
        rs0: true,
        rs1: true,
    })
    .unwrap();
    destroy(dev);
}

#[test]
fn set_does_nothing_if_matches() {
    let mut dev = new(&trans_read!(SQWOUT, [0b0000_0011]));
    dev.set_square_wave_output_rate(SQWOUTRateBits {
        rs0: true,
        rs1: true,
    })
    .unwrap();
    destroy(dev);
}
