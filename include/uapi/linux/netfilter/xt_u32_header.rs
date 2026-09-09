/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the translated Linux types header.

#[repr(C)]
pub enum xt_u32_ops {
    XT_U32_AND,
    XT_U32_LEFTSH,
    XT_U32_RIGHTSH,
    XT_U32_AT,
}

#[repr(C)]
pub struct xt_u32_location_element {
    pub number: __u32,
    pub nextop: __u8,
}

#[repr(C)]
pub struct xt_u32_value_element {
    pub min: __u32,
    pub max: __u32,
}

/*
 * Any way to allow for an arbitrary number of elements?
 * For now, I settle with a limit of 10 each.
 */
pub const XT_U32_MAXSIZE: usize = 10;

#[repr(C)]
pub struct xt_u32_test {
    pub location: [xt_u32_location_element; XT_U32_MAXSIZE + 1],
    pub value: [xt_u32_value_element; XT_U32_MAXSIZE + 1],
    pub nnums: __u8,
    pub nvalues: __u8,
}

#[repr(C)]
pub struct xt_u32 {
    pub tests: [xt_u32_test; XT_U32_MAXSIZE + 1],
    pub ntests: __u8,
    pub invert: __u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
