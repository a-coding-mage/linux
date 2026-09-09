// SPDX-License-Identifier: GPL-2.0
/*
 * M66592 driver platform data
 *
 * Copyright (C) 2009  Renesas Solutions Corp.
 */

pub const M66592_PLATDATA_XTAL_12MHZ: u32 = 0x01;
pub const M66592_PLATDATA_XTAL_24MHZ: u32 = 0x02;
pub const M66592_PLATDATA_XTAL_48MHZ: u32 = 0x03;

#[repr(C)]
pub struct m66592_platdata {
    /* C bit-fields occupy one unsigned-int allocation unit. */
    bits: u32,
}

impl m66592_platdata {
    #[inline]
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    /* one = on chip controller, zero = external controller */
    #[inline]
    pub const fn on_chip(&self) -> u32 {
        self.bits & 0x1
    }

    #[inline]
    pub fn set_on_chip(&mut self, value: u32) {
        self.bits = (self.bits & !0x1) | (value & 0x1);
    }

    /* one = big endian, zero = little endian */
    #[inline]
    pub const fn endian(&self) -> u32 {
        (self.bits >> 1) & 0x1
    }

    #[inline]
    pub fn set_endian(&mut self, value: u32) {
        self.bits = (self.bits & !(0x1 << 1)) | ((value & 0x1) << 1);
    }

    /* (external controller only) M66592_PLATDATA_XTAL_nnMHZ */
    #[inline]
    pub const fn xtal(&self) -> u32 {
        (self.bits >> 2) & 0x3
    }

    #[inline]
    pub fn set_xtal(&mut self, value: u32) {
        self.bits = (self.bits & !(0x3 << 2)) | ((value & 0x3) << 2);
    }

    /* (external controller only) one = 3.3V, zero = 1.5V */
    #[inline]
    pub const fn vif(&self) -> u32 {
        (self.bits >> 4) & 0x1
    }

    #[inline]
    pub fn set_vif(&mut self, value: u32) {
        self.bits = (self.bits & !(0x1 << 4)) | ((value & 0x1) << 4);
    }

    /* (external controller only) set one = WR0_N shorted to WR1_N */
    #[inline]
    pub const fn wr0_shorted_to_wr1(&self) -> u32 {
        (self.bits >> 5) & 0x1
    }

    #[inline]
    pub fn set_wr0_shorted_to_wr1(&mut self, value: u32) {
        self.bits = (self.bits & !(0x1 << 5)) | ((value & 0x1) << 5);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
