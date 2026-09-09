/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2007 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C header guard: __MEMORY_DOT_H__

use core::ffi::c_char;

// Opaque types declared in other headers.
#[repr(C)]
pub struct dlm_rsb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_lkb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_ls {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_mhandle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct writequeue_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_callback {
    _private: [u8; 0],
}

extern "C" {
    pub fn dlm_memory_init() -> i32;
    pub fn dlm_memory_exit();
    pub fn dlm_allocate_rsb() -> *mut dlm_rsb;
    pub fn dlm_free_rsb(r: *mut dlm_rsb);
    pub fn dlm_allocate_lkb() -> *mut dlm_lkb;
    pub fn dlm_free_lkb(l: *mut dlm_lkb);
    pub fn dlm_allocate_lvb(ls: *mut dlm_ls) -> *mut c_char;
    pub fn dlm_free_lvb(l: *mut c_char);
    pub fn dlm_allocate_mhandle() -> *mut dlm_mhandle;
    pub fn dlm_free_mhandle(mhandle: *mut dlm_mhandle);
    pub fn dlm_allocate_writequeue() -> *mut writequeue_entry;
    pub fn dlm_free_writequeue(writequeue: *mut writequeue_entry);
    pub fn dlm_allocate_msg() -> *mut dlm_msg;
    pub fn dlm_free_msg(msg: *mut dlm_msg);
    pub fn dlm_allocate_cb() -> *mut dlm_callback;
    pub fn dlm_free_cb(cb: *mut dlm_callback);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
