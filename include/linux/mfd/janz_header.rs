/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Common Definitions for Janz MODULbus devices
 *
 * Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
 */

#[repr(C)]
pub struct janz_platform_data {
    /* MODULbus Module Number */
    pub modno: u32,
}

/* PLX bridge chip onboard registers */
#[repr(C)]
pub struct janz_cmodio_onboard_regs {
    pub unused1: u8,

    /*
     * Read access: interrupt status
     * Write access: interrupt disable
     */
    pub int_disable: u8,
    pub unused2: u8,

    /*
     * Read access: MODULbus number (hex switch)
     * Write access: interrupt enable
     */
    pub int_enable: u8,
    pub unused3: u8,

    /* write-only */
    pub reset_assert: u8,
    pub unused4: u8,

    /* write-only */
    pub reset_deassert: u8,
    pub unused5: u8,

    /* read-write access to serial EEPROM */
    pub eep: u8,
    pub unused6: u8,

    /* write-only access to EEPROM chip select */
    pub enid: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
