use ds1307::Ds1307;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};

pub const ADDR: u8 = 0b110_1000;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const SECONDS: u8 = 0x00;
    pub const MINUTES: u8 = 0x01;
    pub const HOURS: u8 = 0x02;
    pub const DOW: u8 = 0x03;
    pub const DOM: u8 = 0x04;
    pub const MONTH: u8 = 0x05;
    pub const YEAR: u8 = 0x06;
    pub const SQWOUT: u8 = 0x07;
    pub const RAM_BEGIN: u8 = 0x08;
    pub const RAM_END: u8 = 0x3F;
}

pub fn new(transactions: &[I2cTrans]) -> Ds1307<I2cMock> {
    Ds1307::new(I2cMock::new(transactions))
}

pub fn destroy(dev: Ds1307<I2cMock>) {
    dev.destroy().done();
}

#[macro_export]
macro_rules! assert_invalid_input_data {
    ($result:expr) => {
        match $result {
            Err(Error::InvalidInputData) => (),
            _ => panic!("InvalidInputData error not returned."),
        }
    };
}

#[macro_export]
macro_rules! set_invalid_test {
    ($name:ident, $method:ident, $( $value:expr ),+) => {
        #[test]
        fn $name() {
            let mut rtc = new(&[]);
            assert_invalid_input_data!(rtc.$method($($value),*));
            destroy(rtc);
        }
    };
}

#[macro_export]
macro_rules! trans_read {
    ($register:ident, [ $( $read_bin:expr ),+ ]) => {
        [ I2cTrans::write_read(ADDR, vec![Register::$register], vec![$( $read_bin ),*]) ]
    }
}

#[macro_export]
macro_rules! trans_write {
    ($register:ident, [ $( $read_bin:expr ),+ ]) => {
        [ I2cTrans::write(ADDR, vec![Register::$register, $( $read_bin ),*]) ]
    }
}

#[macro_export]
macro_rules! get_test {
    ($name:ident, $method:ident, $expected:expr, $transactions:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&$transactions);
            assert_eq!($expected, dev.$method().unwrap());
            destroy(dev);
        }
    };
}

#[macro_export]
macro_rules! set_test {
    ($name:ident, $method:ident, $value:expr, $transactions:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&$transactions);
            dev.$method($value).unwrap();
            destroy(dev);
        }
    };
}
