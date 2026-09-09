/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2006-2010 Red Hat, Inc.  All rights reserved.
 */

// C declarations for the DLM user interface.

#[repr(C)]
pub struct dlm_lkb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_ls {
    _private: [u8; 0],
}

extern "C" {
    pub fn dlm_purge_lkb_callbacks(lkb: *mut dlm_lkb);
    pub fn dlm_user_add_ast(
        lkb: *mut dlm_lkb,
        flags: u32,
        mode: i32,
        status: i32,
        sbflags: u32,
    );
    pub fn dlm_user_init() -> i32;
    pub fn dlm_user_exit();
    pub fn dlm_device_deregister(ls: *mut dlm_ls) -> i32;
    pub fn dlm_user_daemon_available() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
