/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

pub const LTQ_SYS_TYPE_LEN: usize = 0x100;
pub const LTQ_SYS_REV_LEN: usize = 0x10;

#[repr(C)]
pub struct ltq_soc_info {
    pub name: *mut u8,
    pub rev: ::core::ffi::c_uint,
    pub rev_type: [u8; LTQ_SYS_REV_LEN],
    pub srev: ::core::ffi::c_uint,
    pub partnum: ::core::ffi::c_uint,
    pub r#type: ::core::ffi::c_uint,
    pub sys_type: [u8; LTQ_SYS_TYPE_LEN],
    pub compatible: *mut u8,
}

unsafe extern "C" {
    pub fn ltq_soc_detect(i: *mut ltq_soc_info);
    pub fn ltq_soc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
