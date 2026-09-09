/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    tvaudio.h - definition for tvaudio inputs

    Copyright (C) 2006 Hans Verkuil (hverkuil@kernel.org)

*/

/*
 * i2c bus addresses for the chips supported by tvaudio.c
 */

pub const I2C_ADDR_TDA8425: u16 = 0x82;
pub const I2C_ADDR_TDA9840: u16 = 0x84;
pub const I2C_ADDR_TDA9874: u16 = 0xb0; /* also used by 9875 */
pub const I2C_ADDR_TDA9875: u16 = 0xb0;
/* Duplicate definitions in the source header have the same values. */
pub const I2C_ADDR_TDA985x_L: u16 = 0xb4; /* also used by 9873 */
pub const I2C_ADDR_TDA985x_H: u16 = 0xb6;
pub const I2C_ADDR_TEA6300: u16 = 0x80; /* also used by 6320 */
pub const I2C_ADDR_TEA6420: u16 = 0x98;
pub const I2C_ADDR_PIC16C54: u16 = 0x96; /* PV951 */

/* The tvaudio module accepts the following inputs: */
pub const TVAUDIO_INPUT_TUNER: i32 = 0;
pub const TVAUDIO_INPUT_RADIO: i32 = 1;
pub const TVAUDIO_INPUT_EXTERN: i32 = 2;
pub const TVAUDIO_INPUT_INTERN: i32 = 3;

/* Supplied by the dependent I2C declarations. */
extern "C" {
    static I2C_CLIENT_END: u16;
}

static TVAUDIO_ADDRS: [u16; 9] = [
    I2C_ADDR_TDA8425 >> 1,
    I2C_ADDR_TEA6300 >> 1,
    I2C_ADDR_TEA6420 >> 1,
    I2C_ADDR_TDA9840 >> 1,
    I2C_ADDR_TDA985x_L >> 1,
    I2C_ADDR_TDA985x_H >> 1,
    I2C_ADDR_TDA9874 >> 1,
    I2C_ADDR_PIC16C54 >> 1,
    /* I2C_CLIENT_END is an external C dependency. */
    I2C_CLIENT_END,
];

#[inline]
pub fn tvaudio_addrs() -> *const u16 {
    TVAUDIO_ADDRS.as_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
