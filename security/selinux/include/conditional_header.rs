/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface to booleans in the security server. This is exported
 * for the selinuxfs.
 *
 * Author: Karl MacMillan <kmacmillan@tresys.com>
 *
 * Copyright (C) 2003 - 2004 Tresys Technology, LLC
 */

/* C dependency: #include "security.h" */
use core::ffi::{c_char, c_int};

use crate::security::selinux_policy;

unsafe extern "C" {
    pub fn security_get_bools(
        policy: *mut selinux_policy,
        len: *mut u32,
        names: *mut *mut *mut c_char,
        values: *mut *mut c_int,
    ) -> c_int;

    pub fn security_set_bools(len: u32, values: *const c_int) -> c_int;

    pub fn security_get_bool_value(index: u32) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
