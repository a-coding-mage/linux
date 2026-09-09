/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h> and <linux/if.h> dependencies.

pub const XT_PHYSDEV_OP_IN: u32 = 0x01;
pub const XT_PHYSDEV_OP_OUT: u32 = 0x02;
pub const XT_PHYSDEV_OP_BRIDGED: u32 = 0x04;
pub const XT_PHYSDEV_OP_ISIN: u32 = 0x08;
pub const XT_PHYSDEV_OP_ISOUT: u32 = 0x10;
pub const XT_PHYSDEV_OP_MASK: u32 = 0x20 - 1;

#[repr(C)]
pub struct xt_physdev_info {
    pub physindev: [::core::ffi::c_char; crate::IFNAMSIZ],
    pub in_mask: [::core::ffi::c_char; crate::IFNAMSIZ],
    pub physoutdev: [::core::ffi::c_char; crate::IFNAMSIZ],
    pub out_mask: [::core::ffi::c_char; crate::IFNAMSIZ],
    pub invert: u8,
    pub bitmask: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
