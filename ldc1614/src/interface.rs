use core::marker;
use embedded_hal::i2c;
use tock_registers::{fields, UIntLike};

struct I2cRegister<SlaveAddr, Register, const REGISTER_ADDR: u16, const REGISTER_BYTE_LEN: usize>
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName,
{
    slave_address: SlaveAddr,
    register: marker::PhantomData<Register>,
}

impl<SlaveAddr, Register, const REGISTER_ADDR: u16, const REGISTER_BYTE_LEN: usize>
    I2cRegister<SlaveAddr, Register, REGISTER_ADDR, REGISTER_BYTE_LEN>
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName,
{
    pub fn read<Bits, I2C>(&self, i2c: &mut I2C, field: fields::Field<Bits, Register>) -> Bits
    where
        I2C: i2c::I2c<SlaveAddr>,
        Bits: UIntLike + From<usize>,
    {
        assert!(
            REGISTER_BYTE_LEN == 1
                || REGISTER_BYTE_LEN == 2
                || REGISTER_BYTE_LEN == 4
                || REGISTER_BYTE_LEN == 8,
            "RegisterByteLen must be 1, 2, 4, or 8"
        );
        let mut read = [0u8; REGISTER_BYTE_LEN];
        if REGISTER_ADDR < u8::MAX as u16 {
            i2c.write_read(self.slave_address, &[REGISTER_ADDR as u8], &mut read)
                .unwrap();
        } else {
            i2c.write_read(
                self.slave_address,
                &[(REGISTER_ADDR >> 8) as u8, REGISTER_ADDR as u8],
                &mut read,
            )
            .unwrap();
        }
        let mut read_fixed = [0u8; 8];
        read_fixed[..REGISTER_BYTE_LEN].copy_from_slice(&read);
        let read = usize::from_be_bytes(read_fixed);
        println!("read: {}", read);
        field.read(Bits::from(read))
    }

    pub fn write<Bits, I2C>(&self, i2c: &mut I2C, field: fields::FieldValue<Bits, Register>)
    where
        I2C: i2c::I2c<SlaveAddr>,
        Bits: UIntLike + Into<usize>,
    {
        assert!(
            REGISTER_BYTE_LEN == 1
                || REGISTER_BYTE_LEN == 2
                || REGISTER_BYTE_LEN == 4
                || REGISTER_BYTE_LEN == 8,
            "RegisterByteLen must be 1, 2, 4, or 8"
        );

        let mut read = [0; 10];
        let mut register_len = 1;
        if REGISTER_ADDR < u8::MAX as u16 {
            read[0] = REGISTER_ADDR as u8;
            read[1..(1 + REGISTER_BYTE_LEN)]
                .copy_from_slice(&field.value.into().to_be_bytes()[(8 - REGISTER_BYTE_LEN)..]);
        } else {
            register_len = 2;
            read[0] = (REGISTER_ADDR >> 8) as u8;
            read[1] = REGISTER_ADDR as u8;
            read[2..(2 + REGISTER_BYTE_LEN)]
                .copy_from_slice(&field.value.into().to_be_bytes()[(8 - REGISTER_BYTE_LEN)..]);
        }
        // std::thread::sleep(std::time::Duration::from_secs(1));

        i2c.write(
            self.slave_address,
            &read[..(register_len + REGISTER_BYTE_LEN)],
        )
        .unwrap();
    }
}

pub struct ReadOnlyI2cRegister<
    SlaveAddr,
    Register,
    const REGISTER_ADDR: u16,
    const REGISTER_BYTE_LEN: usize,
>(I2cRegister<SlaveAddr, Register, REGISTER_ADDR, REGISTER_BYTE_LEN>)
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName;

pub struct WriteOnlyI2cRegister<
    SlaveAddr,
    Register,
    const REGISTER_ADDR: u16,
    const REGISTER_BYTE_LEN: usize,
>(I2cRegister<SlaveAddr, Register, REGISTER_ADDR, REGISTER_BYTE_LEN>)
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName;

impl<SlaveAddr, Register, const REGISTER_ADDR: u16, const REGISTER_BYTE_LEN: usize>
    ReadOnlyI2cRegister<SlaveAddr, Register, REGISTER_ADDR, REGISTER_BYTE_LEN>
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName,
{
    pub fn read<Bits: UIntLike + From<usize>, I2C: i2c::I2c<SlaveAddr>>(
        &self,
        i2c: &mut I2C,
        field: fields::Field<Bits, Register>,
    ) -> Bits {
        self.0.read(i2c, field)
    }
    pub fn new(slave_address: SlaveAddr) -> Self {
        Self(I2cRegister {
            slave_address,
            register: marker::PhantomData,
        })
    }
}

impl<SlaveAddr, Register, const REGISTER_ADDR: u16, const REGISTER_BYTE_LEN: usize>
    WriteOnlyI2cRegister<SlaveAddr, Register, REGISTER_ADDR, REGISTER_BYTE_LEN>
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName,
{
    pub fn write<Bits: UIntLike + Into<usize>, I2C: i2c::I2c<SlaveAddr>>(
        &self,
        i2c: &mut I2C,
        field: fields::FieldValue<Bits, Register>,
    ) {
        self.0.write(i2c, field)
    }
    pub fn new(slave_address: SlaveAddr) -> Self {
        Self(I2cRegister {
            slave_address,
            register: marker::PhantomData,
        })
    }
}

pub struct ReadWriteI2cRegister<
    SlaveAddr,
    Register,
    const REGISTER_ADDR: u16,
    const REGISTER_BYTE_LEN: usize,
>(I2cRegister<SlaveAddr, Register, REGISTER_ADDR, REGISTER_BYTE_LEN>)
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName;

impl<SlaveAddr, Register, const REGISTER_ADDR: u16, const REGISTER_BYTE_LEN: usize>
    ReadWriteI2cRegister<SlaveAddr, Register, REGISTER_ADDR, REGISTER_BYTE_LEN>
where
    SlaveAddr: i2c::AddressMode + Copy,
    Register: tock_registers::RegisterLongName,
{
    pub fn read<Bits: UIntLike + From<usize>, I2C: i2c::I2c<SlaveAddr>>(
        &self,
        i2c: &mut I2C,
        field: fields::Field<Bits, Register>,
    ) -> Bits {
        self.0.read(i2c, field)
    }
    pub fn write<Bits: UIntLike + Into<usize>, I2C: i2c::I2c<SlaveAddr>>(
        &self,
        i2c: &mut I2C,
        field: fields::FieldValue<Bits, Register>,
    ) {
        self.0.write(i2c, field)
    }
    pub fn new(slave_address: SlaveAddr) -> Self {
        Self(I2cRegister {
            slave_address,
            register: marker::PhantomData,
        })
    }
}

// 仔细一想，还是不打算用trait了。泛用性不是很高，和i2c强绑定。除非放弃不可变性否则没法把i2c相关的东西以一个较低的代价迁移到结构体内。
// trait Readable {
//     type SlaveAddr: i2c::AddressMode + Copy;
//     type Register: tock_registers::RegisterLongName;
//     fn read<Bits: UIntLike + From<usize>, I2C: i2c::I2c<Self::SlaveAddr>>(
//         &self,
//         i2c: &mut I2C,
//         field: fields::FieldValue<Bits, Self::Register>,
//     ) -> Bits;
// }

// trait Writeable {
//     type SlaveAddr: i2c::AddressMode + Copy;
//     type Register: tock_registers::RegisterLongName;
//     fn write<Bits: UIntLike + From<usize>, I2C: i2c::I2c<Self::SlaveAddr>>(
//         &self,
//         i2c: &mut I2C,
//         field: fields::FieldValue<Bits, Self::Register>,
//     );
// }

// 发现很多写法很有问题，之后有机会改吧
#[cfg(feature = "tock")]
mod tock {
    use core::marker;
    use embedded_hal::i2c;
    use std::sync;
    use tock_registers::interfaces;

    pub struct ReadOnlyI2c<
        I2C,
        SlaveAddr,
        const LEN: usize,
        const REG_ADDR: u8,
        R: tock_registers::RegisterLongName = (),
    >
    where
        I2C: i2c::I2c<SlaveAddr>,
        SlaveAddr: i2c::AddressMode + Copy,
    {
        inner: sync::RwLock<I2C>,
        slave_address: SlaveAddr,
        register_address: u8,
        associated_register: marker::PhantomData<R>,
    }

    impl<I2C, SlaveAddr, const LEN: usize, const REG_ADDR: u8, R> interfaces::Readable
        for ReadOnlyI2c<I2C, SlaveAddr, LEN, REG_ADDR, R>
    where
        I2C: i2c::I2c<SlaveAddr>,
        R: tock_registers::RegisterLongName,
        SlaveAddr: i2c::AddressMode + Copy,
    {
        type T = usize;
        type R = R;
        fn get(&self) -> Self::T {
            // let reg_address = self._register_long_name.
            let mut result: [u8; LEN] = [0; LEN];
            self.inner
                .write()
                .unwrap()
                .write_read(self.slave_address, &[self.register_address], &mut result)
                .unwrap();
            assert!(
                LEN == 1 || LEN == 2 || LEN == 4 || LEN == 8,
                "LEN must be 1, 2, 4, or 8"
            );
            match LEN {
                //写出这样而不是循环是因为我希望编译器能给我优化了，因为LEN是个常量，且match从ACT上看是收敛的
                1 => result[0] as usize,
                2 => (result[0] as usize) << 8 | result[1] as usize,
                4 => {
                    (result[0] as usize) << 24
                        | (result[1] as usize) << 16
                        | (result[2] as usize) << 8
                        | result[3] as usize
                }
                8 => {
                    (result[0] as usize) << 56
                        | (result[1] as usize) << 48
                        | (result[2] as usize) << 40
                        | (result[3] as usize) << 32
                        | (result[4] as usize) << 24
                        | (result[5] as usize) << 16
                        | (result[6] as usize) << 8
                        | result[7] as usize
                }
                _ => unreachable!(),
            }
        }
    }

    pub struct WriteOnlyI2c<
        I2C,
        SlaveAddr,
        const LEN: usize,
        const REG_ADDR: u8,
        R: tock_registers::RegisterLongName = (),
    >
    where
        I2C: i2c::I2c<SlaveAddr>,
        SlaveAddr: i2c::AddressMode + Copy,
    {
        inner: sync::RwLock<I2C>,
        slave_address: SlaveAddr,
        register_address: u8,
        associated_register: marker::PhantomData<R>,
    }

    impl<I2C, SlaveAddr, const LEN: usize, const REG_ADDR: u8, R> interfaces::Writeable
        for WriteOnlyI2c<I2C, SlaveAddr, LEN, REG_ADDR, R>
    where
        I2C: i2c::I2c<SlaveAddr>,
        R: tock_registers::RegisterLongName,
        SlaveAddr: i2c::AddressMode + Copy,
    {
        type T = usize;
        type R = R;
        fn set(&self, value: Self::T) {
            assert!(
                LEN == 1 || LEN == 2 || LEN == 4 || LEN == 8,
                "LEN must be 1, 2, 4, or 8"
            );
            let mut result: [u8; 9] = [0; 9];
            result[0] = self.register_address;
            match LEN {
                1 => result[1] = value as u8,
                2 => {
                    result[1] = (value >> 8) as u8;
                    result[2] = value as u8;
                }
                4 => {
                    result[1] = (value >> 24) as u8;
                    result[2] = (value >> 16) as u8;
                    result[3] = (value >> 8) as u8;
                    result[4] = value as u8;
                }
                8 => {
                    result[1] = (value >> 56) as u8;
                    result[2] = (value >> 48) as u8;
                    result[3] = (value >> 40) as u8;
                    result[4] = (value >> 32) as u8;
                    result[5] = (value >> 24) as u8;
                    result[6] = (value >> 16) as u8;
                    result[7] = (value >> 8) as u8;
                    result[8] = value as u8;
                }
                _ => unreachable!(),
            }
            self.inner
                .write()
                .unwrap()
                .write(self.slave_address, &result)
                .unwrap();
        }
    }

    pub struct ReadWriteI2c<
        I2C,
        SlaveAddr,
        const LEN: usize,
        const REG_ADDR: u8,
        R: tock_registers::RegisterLongName = (),
    >
    where
        I2C: i2c::I2c<SlaveAddr>,
        SlaveAddr: i2c::AddressMode + Copy,
    {
        inner: sync::RwLock<I2C>,
        slave_address: SlaveAddr,
        register_address: u8,
        associated_register: marker::PhantomData<R>,
    }

    impl<I2C, SlaveAddr, const LEN: usize, const REG_ADDR: u8, R> interfaces::Writeable
        for ReadWriteI2c<I2C, SlaveAddr, LEN, REG_ADDR, R>
    where
        I2C: i2c::I2c<SlaveAddr>,
        R: tock_registers::RegisterLongName,
        SlaveAddr: i2c::AddressMode + Copy,
    {
        type T = usize;
        type R = R;
        fn set(&self, value: Self::T) {
            assert!(
                LEN == 1 || LEN == 2 || LEN == 4 || LEN == 8,
                "LEN must be 1, 2, 4, or 8"
            );
            let mut result: [u8; 9] = [0; 9];
            result[0] = self.register_address;
            match LEN {
                1 => result[1] = value as u8,
                2 => {
                    result[1] = (value >> 8) as u8;
                    result[2] = value as u8;
                }
                4 => {
                    result[1] = (value >> 24) as u8;
                    result[2] = (value >> 16) as u8;
                    result[3] = (value >> 8) as u8;
                    result[4] = value as u8;
                }
                8 => {
                    result[1] = (value >> 56) as u8;
                    result[2] = (value >> 48) as u8;
                    result[3] = (value >> 40) as u8;
                    result[4] = (value >> 32) as u8;
                    result[5] = (value >> 24) as u8;
                    result[6] = (value >> 16) as u8;
                    result[7] = (value >> 8) as u8;
                    result[8] = value as u8;
                }
                _ => unreachable!(),
            }
            self.inner
                .write()
                .unwrap()
                .write(self.slave_address, &result)
                .unwrap();
        }
    }

    impl<I2C, SlaveAddr, const LEN: usize, const REG_ADDR: u8, R> interfaces::Readable
        for ReadWriteI2c<I2C, SlaveAddr, LEN, REG_ADDR, R>
    where
        I2C: i2c::I2c<SlaveAddr>,
        R: tock_registers::RegisterLongName,
        SlaveAddr: i2c::AddressMode + Copy,
    {
        type T = usize;
        type R = R;
        fn get(&self) -> Self::T {
            // let reg_address = self._register_long_name.
            let mut result: [u8; LEN] = [0; LEN];
            self.inner
                .write()
                .unwrap()
                .write_read(self.slave_address, &[self.register_address], &mut result)
                .unwrap();
            assert!(
                LEN == 1 || LEN == 2 || LEN == 4 || LEN == 8,
                "LEN must be 1, 2, 4, or 8"
            );
            match LEN {
                //写出这样而不是循环是因为我希望编译器能给我优化了，因为LEN是个常量，且match从ACT上看是收敛的
                1 => result[0] as usize,
                2 => (result[0] as usize) << 8 | result[1] as usize,
                4 => {
                    (result[0] as usize) << 24
                        | (result[1] as usize) << 16
                        | (result[2] as usize) << 8
                        | result[3] as usize
                }
                8 => {
                    (result[0] as usize) << 56
                        | (result[1] as usize) << 48
                        | (result[2] as usize) << 40
                        | (result[3] as usize) << 32
                        | (result[4] as usize) << 24
                        | (result[5] as usize) << 16
                        | (result[6] as usize) << 8
                        | result[7] as usize
                }
                _ => unreachable!(),
            }
        }
    }
}
