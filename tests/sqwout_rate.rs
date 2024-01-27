use ds1307::SqwOutRate;
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{destroy, new, Register, ADDR};

get_test!(
    get_01,
    square_wave_output_rate,
    SqwOutRate::Khz8_192,
    trans_read!(SQWOUT, [0b0000_0010])
);

macro_rules! set {
    ($name:ident, $rate:ident, $bin:expr) => {
        #[test]
        fn $name() {
            let mut dev = new(&[
                I2cTrans::write_read(ADDR, vec![Register::SQWOUT], vec![0b1001_0000]),
                I2cTrans::write(ADDR, vec![Register::SQWOUT, 0b1001_0000 | $bin]),
            ]);
            dev.set_square_wave_output_rate(SqwOutRate::$rate).unwrap();
            destroy(dev);
        }
    };
}
set!(set_1hz, Hz1, 0);
set!(set_4khz, Khz4_096, 1);
set!(set_8khz, Khz8_192, 2);
set!(set_32khz, Khz32_768, 3);
