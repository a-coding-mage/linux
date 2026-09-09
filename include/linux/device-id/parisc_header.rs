/* SPDX-License-Identifier: GPL-2.0 */

// The original header includes <linux/types.h> when __KERNEL__ is defined.

pub const PA_HWTYPE_ANY_ID: u8 = 0xff;
pub const PA_HVERSION_REV_ANY_ID: u8 = 0xff;
pub const PA_HVERSION_ANY_ID: u16 = 0xffff;
pub const PA_SVERSION_ANY_ID: u32 = 0xffff_ffff;

#[repr(C)]
pub struct parisc_device_id {
    pub hw_type: u8,       /* 5 bits used */
    pub hversion_rev: u8,  /* 4 bits */
    pub hversion: u16,     /* 12 bits */
    pub sversion: u32,     /* 20 bits */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
