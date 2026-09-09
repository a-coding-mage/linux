/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency intent: declarations from <uapi/asm/types.h> are supplied externally.

#[cfg(not(asm))]
#[repr(C)]
pub union register_pair {
    pub pair: u128,
    pub fields: register_pair_fields,
}

#[cfg(not(asm))]
#[repr(C)]
pub struct register_pair_fields {
    pub even: core::ffi::c_ulong,
    pub odd: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
