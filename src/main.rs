//! 用于测试ldc161x板子能否正常工作

fn main() {
    use ldc1614::Ldc;
    use rppal::i2c::I2c;
    let mut i2c = I2c::new().unwrap();
    let _ldc = Ldc::<0x2a>::new(&mut i2c);
}
