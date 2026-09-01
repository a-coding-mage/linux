// SPDX-License-Identifier: GPL-2.0-only
/*
 * C-Media CMI8788 driver - helper functions
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub type u8 = ::core::ffi::c_uchar;
pub type u16 = ::core::ffi::c_ushort;
pub type u32 = ::core::ffi::c_uint;

const EIO: ::core::ffi::c_int = 5;
const UINT_MAX: ::core::ffi::c_uint = ::core::ffi::c_uint::MAX;

#[repr(C)]
pub struct oxygen_saved_registers {
    pub _8: [u8; 0],
    pub _16: [u16; 0],
    pub _32: [u32; 0],
}

#[repr(C)]
pub struct oxygen {
    pub addr: ::core::ffi::c_ulong,
    pub saved_registers: oxygen_saved_registers,
    pub ac97_waitqueue: wait_queue_head_t,
    pub saved_ac97_registers: [[u16; 0]; 0],
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn inb(port: ::core::ffi::c_ulong) -> u8;
    fn inw(port: ::core::ffi::c_ulong) -> u16;
    fn inl(port: ::core::ffi::c_ulong) -> u32;
    fn outb(value: u8, port: ::core::ffi::c_ulong);
    fn outw(value: u16, port: ::core::ffi::c_ulong);
    fn outl(value: u32, port: ::core::ffi::c_ulong);
    fn cpu_to_le16(value: u16) -> u16;
    fn cpu_to_le32(value: u32) -> u32;
    fn msecs_to_jiffies(m: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    fn wait_event_timeout(
        wq_head: *mut wait_queue_head_t,
        condition: ::core::ffi::c_int,
        timeout: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
    fn udelay(usecs: ::core::ffi::c_ulong);
    fn msleep(msecs: ::core::ffi::c_uint);
    fn dev_err(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
}

unsafe extern "C" {
    static OXYGEN_AC97_INTERRUPT_STATUS: ::core::ffi::c_uint;
    static OXYGEN_AC97_REG_ADDR_SHIFT: ::core::ffi::c_uint;
    static OXYGEN_AC97_REG_DIR_WRITE: u32;
    static OXYGEN_AC97_REG_CODEC_SHIFT: ::core::ffi::c_uint;
    static OXYGEN_AC97_REGS: ::core::ffi::c_uint;
    static OXYGEN_AC97_INT_WRITE_DONE: ::core::ffi::c_uint;
    static OXYGEN_AC97_REG_DIR_READ: u32;
    static OXYGEN_AC97_INT_READ_DONE: ::core::ffi::c_uint;
    static OXYGEN_SPI_CONTROL: ::core::ffi::c_uint;
    static OXYGEN_SPI_BUSY: u8;
    static OXYGEN_SPI_DATA1: ::core::ffi::c_uint;
    static OXYGEN_SPI_DATA2: ::core::ffi::c_uint;
    static OXYGEN_SPI_DATA_LENGTH_3: u8;
    static OXYGEN_SPI_DATA3: ::core::ffi::c_uint;
    static OXYGEN_2WIRE_MAP: ::core::ffi::c_uint;
    static OXYGEN_2WIRE_DATA: ::core::ffi::c_uint;
    static OXYGEN_2WIRE_CONTROL: ::core::ffi::c_uint;
    static OXYGEN_2WIRE_DIR_WRITE: u8;
    static OXYGEN_MPU401: ::core::ffi::c_uint;
    static MPU401_TX_FULL: u8;
    static MPU401_RESET: u8;
    static MPU401_ENTER_UART: u8;
    static OXYGEN_EEPROM_CONTROL: ::core::ffi::c_uint;
    static OXYGEN_EEPROM_DIR_READ: u8;
    static OXYGEN_EEPROM_STATUS: ::core::ffi::c_uint;
    static OXYGEN_EEPROM_BUSY: u8;
    static OXYGEN_EEPROM_DATA: ::core::ffi::c_uint;
    static OXYGEN_EEPROM_DIR_WRITE: u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_read8(chip: *mut oxygen, reg: ::core::ffi::c_uint) -> u8 {
    unsafe { inb((*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_read16(chip: *mut oxygen, reg: ::core::ffi::c_uint) -> u16 {
    unsafe { inw((*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_read32(chip: *mut oxygen, reg: ::core::ffi::c_uint) -> u32 {
    unsafe { inl((*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write8(chip: *mut oxygen, reg: ::core::ffi::c_uint, value: u8) {
    unsafe {
        outb(value, (*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        *(*chip).saved_registers._8.as_mut_ptr().add(reg as usize) = value;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write16(chip: *mut oxygen, reg: ::core::ffi::c_uint, value: u16) {
    unsafe {
        outw(value, (*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        *(*chip)
            .saved_registers
            ._16
            .as_mut_ptr()
            .add((reg / 2) as usize) = cpu_to_le16(value);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write32(chip: *mut oxygen, reg: ::core::ffi::c_uint, value: u32) {
    unsafe {
        outl(value, (*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        *(*chip)
            .saved_registers
            ._32
            .as_mut_ptr()
            .add((reg / 4) as usize) = cpu_to_le32(value);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write8_masked(
    chip: *mut oxygen,
    reg: ::core::ffi::c_uint,
    value: u8,
    mask: u8,
) {
    unsafe {
        let mut tmp: u8 = inb((*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        tmp &= !mask;
        tmp |= value & mask;
        outb(tmp, (*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        *(*chip).saved_registers._8.as_mut_ptr().add(reg as usize) = tmp;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write16_masked(
    chip: *mut oxygen,
    reg: ::core::ffi::c_uint,
    value: u16,
    mask: u16,
) {
    unsafe {
        let mut tmp: u16 = inw((*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        tmp &= !mask;
        tmp |= value & mask;
        outw(tmp, (*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        *(*chip)
            .saved_registers
            ._16
            .as_mut_ptr()
            .add((reg / 2) as usize) = cpu_to_le16(tmp);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write32_masked(
    chip: *mut oxygen,
    reg: ::core::ffi::c_uint,
    value: u32,
    mask: u32,
) {
    unsafe {
        let mut tmp: u32 = inl((*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        tmp &= !mask;
        tmp |= value & mask;
        outl(tmp, (*chip).addr.wrapping_add(reg as ::core::ffi::c_ulong));
        *(*chip)
            .saved_registers
            ._32
            .as_mut_ptr()
            .add((reg / 4) as usize) = cpu_to_le32(tmp);
    }
}

unsafe fn oxygen_ac97_wait(chip: *mut oxygen, mask: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    unsafe {
        let mut status: u8 = 0;

        /*
         * Reading the status register also clears the bits, so we have to save
         * the read bits in status.
         */
        wait_event_timeout(
            &mut (*chip).ac97_waitqueue,
            {
                status |= oxygen_read8(chip, OXYGEN_AC97_INTERRUPT_STATUS);
                (status as ::core::ffi::c_uint & mask) as ::core::ffi::c_int
            },
            msecs_to_jiffies(1).wrapping_add(1),
        );
        /*
         * Check even after a timeout because this function should not require
         * the AC'97 interrupt to be enabled.
         */
        status |= oxygen_read8(chip, OXYGEN_AC97_INTERRUPT_STATUS);
        if status as ::core::ffi::c_uint & mask != 0 {
            0
        } else {
            -EIO
        }
    }
}

/*
 * About 10% of AC'97 register reads or writes fail to complete, but even those
 * where the controller indicates completion aren't guaranteed to have actually
 * happened.
 *
 * It's hard to assign blame to either the controller or the codec because both
 * were made by C-Media ...
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write_ac97(
    chip: *mut oxygen,
    codec: ::core::ffi::c_uint,
    index: ::core::ffi::c_uint,
    data: u16,
) {
    unsafe {
        let mut count: ::core::ffi::c_uint;
        let mut succeeded: ::core::ffi::c_uint;
        let mut reg: u32;

        reg = data as u32;
        reg |= index << OXYGEN_AC97_REG_ADDR_SHIFT;
        reg |= OXYGEN_AC97_REG_DIR_WRITE;
        reg |= codec << OXYGEN_AC97_REG_CODEC_SHIFT;
        succeeded = 0;
        count = 5;
        while count > 0 {
            udelay(5);
            oxygen_write32(chip, OXYGEN_AC97_REGS, reg);
            /* require two "completed" writes, just to be sure */
            if oxygen_ac97_wait(chip, OXYGEN_AC97_INT_WRITE_DONE) >= 0 {
                succeeded = succeeded.wrapping_add(1);
                if succeeded >= 2 {
                    *(*chip)
                        .saved_ac97_registers
                        .as_mut_ptr()
                        .add(codec as usize)
                        .cast::<u16>()
                        .add((index / 2) as usize) = data;
                    return;
                }
            }
            count = count.wrapping_sub(1);
        }
        dev_err((*(*chip).card).dev, c"AC'97 write timeout\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_read_ac97(
    chip: *mut oxygen,
    codec: ::core::ffi::c_uint,
    index: ::core::ffi::c_uint,
) -> u16 {
    unsafe {
        let mut count: ::core::ffi::c_uint;
        let mut last_read: ::core::ffi::c_uint = UINT_MAX;
        let mut reg: u32;

        reg = index << OXYGEN_AC97_REG_ADDR_SHIFT;
        reg |= OXYGEN_AC97_REG_DIR_READ;
        reg |= codec << OXYGEN_AC97_REG_CODEC_SHIFT;
        count = 5;
        while count > 0 {
            udelay(5);
            oxygen_write32(chip, OXYGEN_AC97_REGS, reg);
            udelay(10);
            if oxygen_ac97_wait(chip, OXYGEN_AC97_INT_READ_DONE) >= 0 {
                let value: u16 = oxygen_read16(chip, OXYGEN_AC97_REGS);
                /* we require two consecutive reads of the same value */
                if value as ::core::ffi::c_uint == last_read {
                    return value;
                }
                last_read = value as ::core::ffi::c_uint;
                /*
                 * Invert the register value bits to make sure that two
                 * consecutive unsuccessful reads do not return the same
                 * value.
                 */
                reg ^= 0xffff;
            }
            count = count.wrapping_sub(1);
        }
        dev_err(
            (*(*chip).card).dev,
            c"AC'97 read timeout on codec %u\n".as_ptr(),
            codec,
        );
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write_ac97_masked(
    chip: *mut oxygen,
    codec: ::core::ffi::c_uint,
    index: ::core::ffi::c_uint,
    data: u16,
    mask: u16,
) {
    unsafe {
        let mut value: u16 = oxygen_read_ac97(chip, codec, index);
        value &= !mask;
        value |= data & mask;
        oxygen_write_ac97(chip, codec, index, value);
    }
}

unsafe fn oxygen_wait_spi(chip: *mut oxygen) -> ::core::ffi::c_int {
    unsafe {
        let mut count: ::core::ffi::c_uint;

        /*
         * Higher timeout to be sure: 200 us;
         * actual transaction should not need more than 40 us.
         */
        count = 50;
        while count > 0 {
            udelay(4);
            if oxygen_read8(chip, OXYGEN_SPI_CONTROL) & OXYGEN_SPI_BUSY == 0 {
                return 0;
            }
            count = count.wrapping_sub(1);
        }
        dev_err((*(*chip).card).dev, c"oxygen: SPI wait timeout\n".as_ptr());
        -EIO
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write_spi(
    chip: *mut oxygen,
    control: u8,
    data: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        /*
         * We need to wait AFTER initiating the SPI transaction,
         * otherwise read operations will not work.
         */
        oxygen_write8(chip, OXYGEN_SPI_DATA1, data as u8);
        oxygen_write8(chip, OXYGEN_SPI_DATA2, (data >> 8) as u8);
        if control & OXYGEN_SPI_DATA_LENGTH_3 != 0 {
            oxygen_write8(chip, OXYGEN_SPI_DATA3, (data >> 16) as u8);
        }
        oxygen_write8(chip, OXYGEN_SPI_CONTROL, control);
        oxygen_wait_spi(chip)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write_i2c(chip: *mut oxygen, device: u8, map: u8, data: u8) {
    unsafe {
        /* should not need more than about 300 us */
        msleep(1);

        oxygen_write8(chip, OXYGEN_2WIRE_MAP, map);
        oxygen_write8(chip, OXYGEN_2WIRE_DATA, data);
        oxygen_write8(chip, OXYGEN_2WIRE_CONTROL, device | OXYGEN_2WIRE_DIR_WRITE);
    }
}

unsafe fn _write_uart(chip: *mut oxygen, port: ::core::ffi::c_uint, data: u8) {
    unsafe {
        if oxygen_read8(chip, OXYGEN_MPU401 + 1) & MPU401_TX_FULL != 0 {
            msleep(1);
        }
        oxygen_write8(chip, OXYGEN_MPU401 + port, data);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_reset_uart(chip: *mut oxygen) {
    unsafe {
        _write_uart(chip, 1, MPU401_RESET);
        msleep(1); /* wait for ACK */
        _write_uart(chip, 1, MPU401_ENTER_UART);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write_uart(chip: *mut oxygen, data: u8) {
    unsafe {
        _write_uart(chip, 0, data);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_read_eeprom(chip: *mut oxygen, index: ::core::ffi::c_uint) -> u16 {
    unsafe {
        let mut timeout: ::core::ffi::c_uint;

        oxygen_write8(chip, OXYGEN_EEPROM_CONTROL, (index as u8) | OXYGEN_EEPROM_DIR_READ);
        timeout = 0;
        while timeout < 100 {
            udelay(1);
            if !(oxygen_read8(chip, OXYGEN_EEPROM_STATUS) & OXYGEN_EEPROM_BUSY) != 0 {
                break;
            }
            timeout = timeout.wrapping_add(1);
        }
        oxygen_read16(chip, OXYGEN_EEPROM_DATA)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxygen_write_eeprom(
    chip: *mut oxygen,
    index: ::core::ffi::c_uint,
    value: u16,
) {
    unsafe {
        let mut timeout: ::core::ffi::c_uint;

        oxygen_write16(chip, OXYGEN_EEPROM_DATA, value);
        oxygen_write8(
            chip,
            OXYGEN_EEPROM_CONTROL,
            (index as u8) | OXYGEN_EEPROM_DIR_WRITE,
        );
        timeout = 0;
        while timeout < 10 {
            msleep(1);
            if !(oxygen_read8(chip, OXYGEN_EEPROM_STATUS) & OXYGEN_EEPROM_BUSY) != 0 {
                return;
            }
            timeout = timeout.wrapping_add(1);
        }
        dev_err((*(*chip).card).dev, c"EEPROM write timeout\n".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
