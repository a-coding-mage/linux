/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * i2c-dev.h - I2C bus char device interface
 *
 * Copyright (C) 1995-97 Simon G. Vogl
 * Copyright (C) 1998-99 Frodo Looijaard <frodol@dds.nl>
 */

// Dependency intent from <linux/types.h> and <linux/compiler.h> is preserved
// through the referenced kernel types below. The C __user annotation has no
// direct Rust syntax and is represented by raw pointers.

/* /dev/i2c-X ioctl commands. The ioctl parameter is always an unsigned long,
 * except for I2C_FUNCS (pointer to an unsigned long), I2C_RDWR (pointer to
 * struct i2c_rdwr_ioctl_data), and I2C_SMBUS (pointer to
 * struct i2c_smbus_ioctl_data).
 */
pub const I2C_RETRIES: u32 = 0x0701; /* number of times a device address should be polled when not acknowledging */
pub const I2C_TIMEOUT: u32 = 0x0702; /* set timeout in units of 10 ms */

/* NOTE: Slave address is 7 or 10 bits, but 10-bit addresses
 * are NOT supported! (due to code brokenness)
 */
pub const I2C_SLAVE: u32 = 0x0703; /* Use this slave address */
pub const I2C_SLAVE_FORCE: u32 = 0x0706; /* Use this slave address, even if it is already in use by a driver! */
pub const I2C_TENBIT: u32 = 0x0704; /* 0 for 7 bit addrs, != 0 for 10 bit */

pub const I2C_FUNCS: u32 = 0x0705; /* Get the adapter functionality mask */

pub const I2C_RDWR: u32 = 0x0707; /* Combined R/W transfer (one STOP only) */

pub const I2C_PEC: u32 = 0x0708; /* != 0 to use PEC with SMBus */
pub const I2C_SMBUS: u32 = 0x0720; /* SMBus transfer */

/* This is the structure as used in the I2C_SMBUS ioctl call */
#[repr(C)]
pub struct i2c_smbus_ioctl_data {
    pub read_write: __u8,
    pub command: __u8,
    pub size: __u32,
    pub data: *mut i2c_smbus_data,
}

/* This is the structure as used in the I2C_RDWR ioctl call */
#[repr(C)]
pub struct i2c_rdwr_ioctl_data {
    pub msgs: *mut i2c_msg, /* pointers to i2c_msgs */
    pub nmsgs: __u32, /* number of i2c_msgs */
}

pub const I2C_RDWR_IOCTL_MAX_MSGS: u32 = 42;
/* Originally defined with a typo, keep it for compatibility */
pub const I2C_RDRW_IOCTL_MAX_MSGS: u32 = I2C_RDWR_IOCTL_MAX_MSGS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
