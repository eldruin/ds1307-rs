extern crate ds1307;
use ds1307::Hours;

mod common;
use common::{setup, check_sent_data};

#[test]
fn can_read_datetime() {
    let mut rtc = setup(&[0b1101_1000, 0b0101_1001, 0b0010_0011, 0b0000_0010,
                          0b0001_0011, 0b0000_1000, 0b0001_1000]);
    let datetime = rtc.get_datetime().unwrap();
    assert_eq!(2018, datetime.year);
    assert_eq!(08,   datetime.month);
    assert_eq!(13,   datetime.day);
    assert_eq!(2,    datetime.weekday);
    if let Hours::H24(h) = datetime.hour {
        assert_eq!(23, h);
    }
    else {
        panic!();
    }
    assert_eq!(59,   datetime.minute);
    assert_eq!(58,   datetime.second);
    check_sent_data(rtc, &[0x00]);
}

