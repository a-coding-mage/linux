/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2020 Jonathan Neuschäfer
 *
 * Register access and version information for the Netronix embedded
 * controller.
 */

// Forward declarations corresponding to the C header's external types.
#[allow(non_camel_case_types)]
pub enum device {}
#[allow(non_camel_case_types)]
pub enum regmap {}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ntxec {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

/*
 * Some registers, such as the battery status register (0x41), are in
 * big-endian, but others only have eight significant bits, which are in the
 * first byte transmitted over I2C (the MSB of the big-endian value).
 * This convenience function converts an 8-bit value to 16-bit for use in the
 * second kind of register.
 */
#[inline]
pub fn ntxec_reg8(value: u8) -> u16 {
    (value as u16) << 8
}

/* Known firmware versions */
pub const NTXEC_VERSION_KOBO_AURA: u16 = 0xd726; /* found in Kobo Aura */
pub const NTXEC_VERSION_TOLINO_SHINE2: u16 = 0xf110; /* found in Tolino Shine 2 HD */
pub const NTXEC_VERSION_TOLINO_VISION: u16 = 0xe135; /* found in Tolino Vision, contains RTC, ADC, PWM, home pad */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
