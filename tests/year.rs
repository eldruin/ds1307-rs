extern crate ds1307;
extern crate embedded_hal_mock as hal;
use self::ds1307::Error;
use self::hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, Register, ADDR};

get_test!(get, get_year, 2099, trans_read!(YEAR, [0b1001_1001]));

set_invalid_test!(too_small, set_year, 1999);
set_invalid_test!(too_big, set_year, 2100);

set_test!(set, set_year, 2099, trans_write!(YEAR, [0b1001_1001]));
