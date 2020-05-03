extern crate ds1307;
extern crate embedded_hal_mock as hal;
use self::ds1307::{Error, Hours};
use self::hal::i2c::Transaction as I2cTrans;
mod common;
use common::{destroy, new, Register, ADDR};

get_test!(
    can_read_24h_hours,
    get_hours,
    Hours::H24(23),
    trans_read!(HOURS, [0b0010_0011])
);
set_invalid_test!(wrong_h24, set_hours, Hours::H24(24));
set_test!(
    set_24h,
    set_hours,
    Hours::H24(23),
    trans_write!(HOURS, [0b0010_0011])
);

get_test!(
    can_read_h12_am_hours,
    get_hours,
    Hours::AM(12),
    trans_read!(HOURS, [0b0101_0010])
);
set_invalid_test!(h12_am_too_small, set_hours, Hours::AM(0));
set_invalid_test!(h12_am_too_big, set_hours, Hours::AM(13));
set_test!(
    set_h12_am,
    set_hours,
    Hours::AM(12),
    trans_write!(HOURS, [0b0101_0010])
);

get_test!(
    can_read_h12_pm_hours,
    get_hours,
    Hours::PM(12),
    trans_read!(HOURS, [0b0111_0010])
);
set_invalid_test!(h12_pm_too_small, set_hours, Hours::PM(0));
set_invalid_test!(h12_pm_too_big, set_hours, Hours::PM(13));
set_test!(
    set_h12_pm,
    set_hours,
    Hours::PM(12),
    trans_write!(HOURS, [0b0111_0010])
);
