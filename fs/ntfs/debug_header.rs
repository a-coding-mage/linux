/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NTFS kernel debug support.
 *
 * Copyright (c) 2001-2004 Anton Altaparmakov
 */

use core::ffi::{c_char, c_int, c_void};

/* Supplied by the corresponding translated kernel and runlist code. */
#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct runlist_element {
    _private: [u8; 0],
}

#[cfg(feature = "DEBUG")]
extern "C" {
    pub static mut debug_msgs: c_int;

    pub fn __ntfs_debug(
        file: *const c_char,
        line: c_int,
        function: *const c_char,
        format: *const c_char,
        ...,
    );

    pub fn ntfs_debug_dump_runlist(rl: *const runlist_element);
}

/* ntfs_debug writes a debug-level message only when DEBUG is enabled. */
#[cfg(feature = "DEBUG")]
#[macro_export]
macro_rules! ntfs_debug {
    ($f:expr $(, $a:expr)*) => {
        unsafe {
            $crate::__ntfs_debug(
                concat!(file!(), "\0").as_ptr() as *const core::ffi::c_char,
                line!() as core::ffi::c_int,
                concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                $f,
                $($a),*
            )
        }
    };
}

/* Build-time !DEBUG condition: these calls are compiled as no-ops. */
#[cfg(not(feature = "DEBUG"))]
#[macro_export]
macro_rules! ntfs_debug {
    ($fmt:expr $(, $args:expr)*) => {{
        if false {
            let _ = ($fmt, ($($args),*));
        }
    }};
}

#[cfg(not(feature = "DEBUG"))]
#[macro_export]
macro_rules! ntfs_debug_dump_runlist {
    ($rl:expr) => {{
        if false {
            let _ = $rl;
        }
    }};
}

#[cfg(feature = "DEBUG")]
#[macro_export]
macro_rules! ntfs_debug_dump_runlist {
    ($rl:expr) => {
        unsafe { $crate::ntfs_debug_dump_runlist($rl) }
    };
}

extern "C" {
    pub fn __ntfs_warning(
        function: *const c_char,
        sb: *const super_block,
        fmt: *const c_char,
        ...,
    );

    pub fn __ntfs_error(
        function: *const c_char,
        sb: *mut super_block,
        fmt: *const c_char,
        ...,
    );

    pub fn ntfs_handle_error(sb: *mut super_block);
}

#[macro_export]
macro_rules! ntfs_warning {
    ($sb:expr, $f:expr $(, $a:expr)*) => {
        unsafe {
            $crate::__ntfs_warning(
                concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                $sb,
                $f,
                $($a),*
            )
        }
    };
}

#[macro_export]
macro_rules! ntfs_error {
    ($sb:expr, $f:expr $(, $a:expr)*) => {
        unsafe {
            $crate::__ntfs_error(
                concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                $sb,
                $f,
                $($a),*
            )
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
