/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024 Oracle and/or its affiliates.
 *
 * This header defines XDR data type primitives specified in
 * Section 4 of RFC 4506, used by RPC programs implemented
 * in the Linux kernel.
 */

pub const TRUE: bool = true;
pub const FALSE: bool = false;

#[repr(C)]
pub struct string {
    pub len: u32,
    pub data: *mut u8,
}

#[repr(C)]
pub struct opaque {
    pub len: u32,
    pub data: *mut u8,
}

pub const XDR_void: i32 = 0;
pub const XDR_bool: i32 = 1;
pub const XDR_short: i32 = 1;
pub const XDR_unsigned_short: i32 = 1;
pub const XDR_int: i32 = 1;
pub const XDR_unsigned_int: i32 = 1;
pub const XDR_long: i32 = 1;
pub const XDR_unsigned_long: i32 = 1;
pub const XDR_hyper: i32 = 2;
pub const XDR_unsigned_hyper: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
