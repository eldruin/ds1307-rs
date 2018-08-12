extern crate ds1307;

mod common;
use common::{setup, check_sent_data};

const DOM_REGISTER : u8 = 0x04;

#[test]
fn can_read_day_of_month() {
    let mut rtc = setup(&[0b0011_0001]);
    assert_eq!(31, rtc.get_day_of_month().unwrap());
    check_sent_data(rtc, &[DOM_REGISTER]);
}