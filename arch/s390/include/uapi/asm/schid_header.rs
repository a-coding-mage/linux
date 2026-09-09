/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h>; __u32 is represented by u32.
// The C declaration uses packed, aligned(4) bit-fields in one 32-bit word.
#[repr(C, packed(4))]
#[derive(Copy, Clone, Default)]
pub struct subchannel_id {
    bits: u32,
}

impl subchannel_id {
    pub const CSSID_MASK: u32 = 0x0000_00ff;
    pub const M_MASK: u32 = 0x0000_1000;
    pub const SSID_MASK: u32 = 0x0000_6000;
    pub const ONE_MASK: u32 = 0x0000_8000;
    pub const SCH_NO_MASK: u32 = 0xffff_0000;

    #[inline]
    pub const fn new(bits: u32) -> Self {
        Self { bits }
    }

    #[inline]
    pub const fn bits(self) -> u32 {
        self.bits
    }

    #[inline]
    pub const fn cssid(self) -> u32 {
        self.bits & Self::CSSID_MASK
    }

    #[inline]
    pub fn set_cssid(&mut self, value: u32) {
        self.bits = (self.bits & !Self::CSSID_MASK) | (value & Self::CSSID_MASK);
    }

    #[inline]
    pub const fn m(self) -> u32 {
        (self.bits & Self::M_MASK) >> 12
    }

    #[inline]
    pub fn set_m(&mut self, value: u32) {
        self.bits = (self.bits & !Self::M_MASK) | ((value & 1) << 12);
    }

    #[inline]
    pub const fn ssid(self) -> u32 {
        (self.bits & Self::SSID_MASK) >> 13
    }

    #[inline]
    pub fn set_ssid(&mut self, value: u32) {
        self.bits = (self.bits & !Self::SSID_MASK) | ((value & 3) << 13);
    }

    #[inline]
    pub const fn one(self) -> u32 {
        (self.bits & Self::ONE_MASK) >> 15
    }

    #[inline]
    pub fn set_one(&mut self, value: u32) {
        self.bits = (self.bits & !Self::ONE_MASK) | ((value & 1) << 15);
    }

    #[inline]
    pub const fn sch_no(self) -> u32 {
        (self.bits & Self::SCH_NO_MASK) >> 16
    }

    #[inline]
    pub fn set_sch_no(&mut self, value: u32) {
        self.bits = (self.bits & !Self::SCH_NO_MASK) | ((value & 0xffff) << 16);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
