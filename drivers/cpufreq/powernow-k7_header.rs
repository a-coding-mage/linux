/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  (C) 2003 Dave Jones.
 *
 *  AMD-specific information
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub union msr_fidvidctl {
    pub bits: msr_fidvidctl_bits,
    pub val: u64,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct msr_fidvidctl_bits {
    pub val: u64,
}

impl msr_fidvidctl_bits {
    pub const fn new(val: u64) -> Self { Self { val } }

    pub const fn FID(self) -> u64 { self.val & 0x1f }
    pub const fn reserved1(self) -> u64 { (self.val >> 5) & 0x7 }
    pub const fn VID(self) -> u64 { (self.val >> 8) & 0x1f }
    pub const fn reserved2(self) -> u64 { (self.val >> 13) & 0x7 }
    pub const fn FIDC(self) -> u64 { (self.val >> 16) & 0x1 }
    pub const fn VIDC(self) -> u64 { (self.val >> 17) & 0x1 }
    pub const fn reserved3(self) -> u64 { (self.val >> 18) & 0x3 }
    pub const fn FIDCHGRATIO(self) -> u64 { (self.val >> 20) & 0x1 }
    pub const fn reserved4(self) -> u64 { (self.val >> 21) & 0x7ff }
    pub const fn SGTC(self) -> u64 { (self.val >> 32) & 0xfffff }
    pub const fn reserved5(self) -> u64 { (self.val >> 52) & 0xfff }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union msr_fidvidstatus {
    pub bits: msr_fidvidstatus_bits,
    pub val: u64,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct msr_fidvidstatus_bits {
    pub val: u64,
}

impl msr_fidvidstatus_bits {
    pub const fn new(val: u64) -> Self { Self { val } }

    pub const fn CFID(self) -> u64 { self.val & 0x1f }
    pub const fn reserved1(self) -> u64 { (self.val >> 5) & 0x7 }
    pub const fn SFID(self) -> u64 { (self.val >> 8) & 0x1f }
    pub const fn reserved2(self) -> u64 { (self.val >> 13) & 0x7 }
    pub const fn MFID(self) -> u64 { (self.val >> 16) & 0x1f }
    pub const fn reserved3(self) -> u64 { (self.val >> 21) & 0x7ff }
    pub const fn CVID(self) -> u64 { (self.val >> 32) & 0x1f }
    pub const fn reserved4(self) -> u64 { (self.val >> 37) & 0x7 }
    pub const fn SVID(self) -> u64 { (self.val >> 40) & 0x1f }
    pub const fn reserved5(self) -> u64 { (self.val >> 45) & 0x7 }
    pub const fn MVID(self) -> u64 { (self.val >> 48) & 0x1f }
    pub const fn reserved6(self) -> u64 { (self.val >> 53) & 0x7ff }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
