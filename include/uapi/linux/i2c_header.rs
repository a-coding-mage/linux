/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * i2c.h - definitions for the I2C bus interface
 *
 * Copyright (C) 1995-2000 Simon G. Vogl
 * With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and
 * Frodo Looijaard <frodol@dds.nl>
 */

#[repr(C)]
pub struct i2c_msg {
    pub addr: u16,
    pub flags: u16,
    pub len: u16,
    pub buf: *mut u8,
}

pub const I2C_M_RD: u16 = 0x0001; // guaranteed to be 0x0001!
pub const I2C_M_TEN: u16 = 0x0010; // use only if I2C_FUNC_10BIT_ADDR
pub const I2C_M_DMA_SAFE: u16 = 0x0200; // use only in kernel space
pub const I2C_M_RECV_LEN: u16 = 0x0400; // use only if I2C_FUNC_SMBUS_READ_BLOCK_DATA
pub const I2C_M_NO_RD_ACK: u16 = 0x0800; // use only if I2C_FUNC_PROTOCOL_MANGLING
pub const I2C_M_IGNORE_NAK: u16 = 0x1000; // use only if I2C_FUNC_PROTOCOL_MANGLING
pub const I2C_M_REV_DIR_ADDR: u16 = 0x2000; // use only if I2C_FUNC_PROTOCOL_MANGLING
pub const I2C_M_NOSTART: u16 = 0x4000; // use only if I2C_FUNC_NOSTART
pub const I2C_M_STOP: u16 = 0x8000; // use only if I2C_FUNC_PROTOCOL_MANGLING

// To determine what functionality is present
pub const I2C_FUNC_I2C: u32 = 0x00000001;
pub const I2C_FUNC_10BIT_ADDR: u32 = 0x00000002; // required for I2C_M_TEN
pub const I2C_FUNC_PROTOCOL_MANGLING: u32 = 0x00000004; // required for I2C_M_IGNORE_NAK etc.
pub const I2C_FUNC_SMBUS_PEC: u32 = 0x00000008;
pub const I2C_FUNC_NOSTART: u32 = 0x00000010; // required for I2C_M_NOSTART
pub const I2C_FUNC_SLAVE: u32 = 0x00000020;
pub const I2C_FUNC_SMBUS_BLOCK_PROC_CALL: u32 = 0x00008000; // SMBus 2.0 or later
pub const I2C_FUNC_SMBUS_QUICK: u32 = 0x00010000;
pub const I2C_FUNC_SMBUS_READ_BYTE: u32 = 0x00020000;
pub const I2C_FUNC_SMBUS_WRITE_BYTE: u32 = 0x00040000;
pub const I2C_FUNC_SMBUS_READ_BYTE_DATA: u32 = 0x00080000;
pub const I2C_FUNC_SMBUS_WRITE_BYTE_DATA: u32 = 0x00100000;
pub const I2C_FUNC_SMBUS_READ_WORD_DATA: u32 = 0x00200000;
pub const I2C_FUNC_SMBUS_WRITE_WORD_DATA: u32 = 0x00400000;
pub const I2C_FUNC_SMBUS_PROC_CALL: u32 = 0x00800000;
pub const I2C_FUNC_SMBUS_READ_BLOCK_DATA: u32 = 0x01000000; // required for I2C_M_RECV_LEN
pub const I2C_FUNC_SMBUS_WRITE_BLOCK_DATA: u32 = 0x02000000;
pub const I2C_FUNC_SMBUS_READ_I2C_BLOCK: u32 = 0x04000000; // I2C-like block xfer
pub const I2C_FUNC_SMBUS_WRITE_I2C_BLOCK: u32 = 0x08000000; // w/ 1-byte reg. addr.
pub const I2C_FUNC_SMBUS_HOST_NOTIFY: u32 = 0x10000000; // SMBus 2.0 or later

pub const I2C_FUNC_SMBUS_BYTE: u32 = I2C_FUNC_SMBUS_READ_BYTE | I2C_FUNC_SMBUS_WRITE_BYTE;
pub const I2C_FUNC_SMBUS_BYTE_DATA: u32 = I2C_FUNC_SMBUS_READ_BYTE_DATA | I2C_FUNC_SMBUS_WRITE_BYTE_DATA;
pub const I2C_FUNC_SMBUS_WORD_DATA: u32 = I2C_FUNC_SMBUS_READ_WORD_DATA | I2C_FUNC_SMBUS_WRITE_WORD_DATA;
pub const I2C_FUNC_SMBUS_BLOCK_DATA: u32 = I2C_FUNC_SMBUS_READ_BLOCK_DATA | I2C_FUNC_SMBUS_WRITE_BLOCK_DATA;
pub const I2C_FUNC_SMBUS_I2C_BLOCK: u32 = I2C_FUNC_SMBUS_READ_I2C_BLOCK | I2C_FUNC_SMBUS_WRITE_I2C_BLOCK;
pub const I2C_FUNC_SMBUS_EMUL: u32 = I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA | I2C_FUNC_SMBUS_PROC_CALL | I2C_FUNC_SMBUS_WRITE_BLOCK_DATA | I2C_FUNC_SMBUS_I2C_BLOCK | I2C_FUNC_SMBUS_PEC;
pub const I2C_FUNC_SMBUS_EMUL_ALL: u32 = I2C_FUNC_SMBUS_EMUL | I2C_FUNC_SMBUS_READ_BLOCK_DATA | I2C_FUNC_SMBUS_BLOCK_PROC_CALL;

// Data for SMBus Messages
pub const I2C_SMBUS_BLOCK_MAX: usize = 32; // As specified in SMBus standard
#[repr(C)]
pub union i2c_smbus_data {
    pub byte: u8,
    pub word: u16,
    pub block: [u8; I2C_SMBUS_BLOCK_MAX + 2],
}

// i2c_smbus_xfer read or write markers
pub const I2C_SMBUS_READ: u32 = 1;
pub const I2C_SMBUS_WRITE: u32 = 0;

// SMBus transaction types (size parameter in the above functions)
// Note: these no longer correspond to the (arbitrary) PIIX4 internal codes!
pub const I2C_SMBUS_QUICK: u32 = 0;
pub const I2C_SMBUS_BYTE: u32 = 1;
pub const I2C_SMBUS_BYTE_DATA: u32 = 2;
pub const I2C_SMBUS_WORD_DATA: u32 = 3;
pub const I2C_SMBUS_PROC_CALL: u32 = 4;
pub const I2C_SMBUS_BLOCK_DATA: u32 = 5;
pub const I2C_SMBUS_I2C_BLOCK_BROKEN: u32 = 6;
pub const I2C_SMBUS_BLOCK_PROC_CALL: u32 = 7; // SMBus 2.0
pub const I2C_SMBUS_I2C_BLOCK_DATA: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
