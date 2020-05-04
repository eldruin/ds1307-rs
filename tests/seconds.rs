use ds1307::{Error, Rtcc};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(
    can_read_seconds,
    get_seconds,
    59,
    trans_read!(SECONDS, [0b0101_1001])
);

get_test!(
    ch_bit_is_ignored,
    get_seconds,
    59,
    trans_read!(SECONDS, [0b1101_1001])
);

set_invalid_test!(wrong_seconds_returns_error, set_seconds, 60);

set_test!(
    can_write_seconds,
    set_seconds,
    59,
    [
        I2cTrans::write_read(ADDR, vec![Register::SECONDS], vec![0]),
        I2cTrans::write(ADDR, vec![Register::SECONDS, 0b0101_1001])
    ]
);

set_test!(
    ch_bit_is_kept_when_writing_seconds,
    set_seconds,
    59,
    [
        I2cTrans::write_read(ADDR, vec![Register::SECONDS], vec![0b1000_0000]),
        I2cTrans::write(ADDR, vec![Register::SECONDS, 0b1101_1001])
    ]
);
