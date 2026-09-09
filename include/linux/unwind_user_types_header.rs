/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Unwind types, listed in priority order: lower numbers are attempted first if
 * available.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum unwind_user_type_bits {
    UNWIND_USER_TYPE_FP_BIT = 0,

    NR_UNWIND_USER_TYPE_BITS,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum unwind_user_type {
    /* Type "none" for the start of stack walk iteration. */
    UNWIND_USER_TYPE_NONE = 0,
    UNWIND_USER_TYPE_FP = 1 << (unwind_user_type_bits::UNWIND_USER_TYPE_FP_BIT as u32),
}

#[repr(C)]
pub struct unwind_stacktrace {
    pub nr: u32,
    pub entries: *mut usize,
}

#[repr(C)]
pub struct unwind_user_frame {
    pub cfa_off: i32,
    pub ra_off: i32,
    pub fp_off: i32,
    pub use_fp: bool,
}

#[repr(C)]
pub struct unwind_user_state {
    pub ip: usize,
    pub sp: usize,
    pub fp: usize,
    pub ws: u32,
    pub current_type: unwind_user_type,
    pub available_types: u32,
    pub topmost: bool,
    pub done: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
