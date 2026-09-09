/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * super.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

use core::ffi::{c_char, c_int};

// `struct super_block` and `sigset_t` are supplied by the surrounding
// translation unit/dependencies.
extern "C" {
    pub fn __ocfs2_error(
        sb: *mut super_block,
        function: *const c_char,
        fmt: *const c_char,
        ...,
    ) -> c_int;

    pub fn __ocfs2_abort(
        sb: *mut super_block,
        function: *const c_char,
        fmt: *const c_char,
        ...,
    );

    pub fn ocfs2_block_signals(oldset: *mut sigset_t);
    pub fn ocfs2_unblock_signals(oldset: *mut sigset_t);
}

#[macro_export]
macro_rules! ocfs2_error {
    ($sb:expr, $fmt:expr $(, $arg:expr)*) => {
        unsafe {
            $crate::__ocfs2_error(
                $sb,
                concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                $fmt,
                $($arg),*
            )
        }
    };
}

#[macro_export]
macro_rules! ocfs2_abort {
    ($sb:expr, $fmt:expr $(, $arg:expr)*) => {
        unsafe {
            $crate::__ocfs2_abort(
                $sb,
                concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                $fmt,
                $($arg),*
            )
        }
    };
}

/*
 * Void signal blockers, because in-kernel sigprocmask() only fails
 * when SIG_* is wrong.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
