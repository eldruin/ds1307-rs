use ds1307::Error;
use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(can_read_dom, get_day, 31, trans_read!(DOM, [0b0011_0001]));

set_invalid_test!(too_small, set_day, 0);
set_invalid_test!(too_big, set_day, 32);

set_test!(can_set_dom, set_day, 7, trans_write!(DOM, [0b0000_0111]));
