use ds1307::Error;
use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(
    can_read_minutes,
    get_minutes,
    59,
    trans_read!(MINUTES, [0b0101_1001])
);

set_invalid_test!(wrong_returns_error, set_minutes, 60);

set_test!(
    can_write_minutes,
    set_minutes,
    59,
    trans_write!(MINUTES, [0b0101_1001])
);
