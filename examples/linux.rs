extern crate ds1307;
extern crate linux_embedded_hal as hal;

use ds1307::{DateTime, Hours, DS1307};
use hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = DS1307::new(dev);
    let datetime = DateTime {
        year: 2018,
        month: 08,
        day: 15,
        weekday: 4,
        hour: Hours::H24(19),
        minute: 59,
        second: 58,
    };
    rtc.set_datetime(&datetime).unwrap();

    let datetime = rtc.get_datetime().unwrap();

    // The hours depend on the RTC running mode.
    match datetime.hour {
        Hours::H24(h) => println!(
            "{}-{}-{}, {} {}:{}:{}",
            datetime.year,
            datetime.month,
            datetime.day,
            datetime.weekday,
            h,
            datetime.minute,
            datetime.second
        ),
        Hours::AM(h) => println!(
            "{}-{}-{}, {} {}:{}:{} AM",
            datetime.year,
            datetime.month,
            datetime.day,
            datetime.weekday,
            h,
            datetime.minute,
            datetime.second
        ),
        Hours::PM(h) => println!(
            "{}-{}-{}, {} {}:{}:{} PM",
            datetime.year,
            datetime.month,
            datetime.day,
            datetime.weekday,
            h,
            datetime.minute,
            datetime.second
        ),
    }
    // This will print something like: 2018-08-15, 4 19:59:58
}
