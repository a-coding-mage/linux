/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header. The __KERNEL__-only option mask is
// retained below as a conditional Rust definition.

pub const XT_RPFILTER_LOOSE: u32 = 1 << 0;
pub const XT_RPFILTER_VALID_MARK: u32 = 1 << 1;
pub const XT_RPFILTER_ACCEPT_LOCAL: u32 = 1 << 2;
pub const XT_RPFILTER_INVERT: u32 = 1 << 3;

#[cfg(feature = "__KERNEL__")]
pub const XT_RPFILTER_OPTION_MASK: u32 = XT_RPFILTER_LOOSE
    | XT_RPFILTER_VALID_MARK
    | XT_RPFILTER_ACCEPT_LOCAL
    | XT_RPFILTER_INVERT;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_rpfilter_info {
    pub flags: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
