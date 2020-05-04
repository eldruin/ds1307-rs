use ds1307::{Ds1307, NaiveDate, Rtcc};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds1307::new(dev);
    let datetime = NaiveDate::from_ymd(2020, 5, 2).and_hms(19, 59, 58);
    rtc.set_datetime(&datetime).unwrap();
    // ...
    let datetime = rtc.get_datetime().unwrap();
    println!("{}", datetime);
    // This will print something like: 2020-05-02 19:59:58
}
