use ds1307::{DateTimeAccess, Ds1307, NaiveDate};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds1307::new(dev);
    let datetime = NaiveDate::from_ymd_opt(2022, 1, 2)
        .unwrap()
        .and_hms_opt(19, 59, 58)
        .unwrap();
    rtc.set_datetime(&datetime).unwrap();
    // ...
    let datetime = rtc.datetime().unwrap();
    println!("{datetime}");
    // This will print something like: 2022-01-02 19:59:58
}
