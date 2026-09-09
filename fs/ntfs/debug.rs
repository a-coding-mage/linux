// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS kernel debug support.
 *
 * Copyright (c) 2001-2004 Anton Altaparmakov
 */

// Dependency intent: this file corresponds to debug.h and the kernel
// formatting, varargs, and NTFS definitions supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_longlong, c_void};

#[repr(C)]
pub struct super_block {
    pub s_id: *const c_char,
}

#[repr(C)]
pub struct va_format {
    pub fmt: *const c_char,
    pub va: *mut c_void,
}

#[repr(C)]
pub struct runlist_element {
    pub vcn: i64,
    pub lcn: i64,
    pub length: i64,
}

// External definitions supplied by the NTFS/kernel environment.
extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn ntfs_handle_error(sb: *mut super_block);
}

// The C preprocessor's DEBUG build condition is retained as a Rust cfg.
#[cfg(feature = "DEBUG")]
#[no_mangle]
pub unsafe extern "C" fn __ntfs_warning(
    function: *const c_char,
    sb: *const super_block,
    fmt: *const c_char,
    mut args: ...,
) {
    let mut vaf = va_format { fmt, va: &mut args as *mut _ as *mut c_void };
    let flen = if !function.is_null() { strlen(function) } else { 0 };
    if !sb.is_null() {
        pr_warn!("(device %s): %s(): %pV\n", (*sb).s_id, if flen != 0 { function } else { b"\0".as_ptr() as *const c_char }, &mut vaf);
    } else {
        pr_warn!("%s(): %pV\n", if flen != 0 { function } else { b"\0".as_ptr() as *const c_char }, &mut vaf);
    }
}

#[cfg(not(feature = "DEBUG"))]
#[no_mangle]
pub unsafe extern "C" fn __ntfs_warning(
    function: *const c_char,
    sb: *const super_block,
    fmt: *const c_char,
    mut args: ...,
) {
    let mut vaf = va_format { fmt, va: &mut args as *mut _ as *mut c_void };
    let flen = if !function.is_null() { strlen(function) } else { 0 };
    if !sb.is_null() {
        pr_warn_ratelimited!("(device %s): %s(): %pV\n", (*sb).s_id, if flen != 0 { function } else { b"\0".as_ptr() as *const c_char }, &mut vaf);
    } else {
        pr_warn_ratelimited!("%s(): %pV\n", if flen != 0 { function } else { b"\0".as_ptr() as *const c_char }, &mut vaf);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __ntfs_error(
    function: *const c_char,
    sb: *mut super_block,
    fmt: *const c_char,
    mut args: ...,
) {
    let mut vaf = va_format { fmt, va: &mut args as *mut _ as *mut c_void };
    let flen = if !function.is_null() { strlen(function) } else { 0 };
    if !sb.is_null() {
        pr_err_ratelimited!("(device %s): %s(): %pV\n", (*sb).s_id, if flen != 0 { function } else { b"\0".as_ptr() as *const c_char }, &mut vaf);
    } else {
        pr_err_ratelimited!("%s(): %pV\n", if flen != 0 { function } else { b"\0".as_ptr() as *const c_char }, &mut vaf);
    }
    if !sb.is_null() {
        ntfs_handle_error(sb);
    }
}

#[cfg(feature = "DEBUG")]
#[no_mangle]
pub static mut debug_msgs: c_int = 0;

#[cfg(feature = "DEBUG")]
pub unsafe extern "C" fn __ntfs_debug(
    file: *const c_char,
    line: c_int,
    function: *const c_char,
    fmt: *const c_char,
    mut args: ...,
) {
    if debug_msgs == 0 { return; }
    let flen = if !function.is_null() { strlen(function) } else { 0 };
    let mut vaf = va_format { fmt, va: &mut args as *mut _ as *mut c_void };
    pr_debug!("(%s, %d): %s(): %pV", file, line, if flen != 0 { function } else { b"\0".as_ptr() as *const c_char }, &mut vaf);
}

#[cfg(feature = "DEBUG")]
pub unsafe extern "C" fn ntfs_debug_dump_runlist(rl: *const runlist_element) {
    if debug_msgs == 0 { return; }
    pr_debug!("Dumping runlist (values in hex):\n");
    if rl.is_null() { pr_debug!("Run list not present.\n"); return; }
    pr_debug!("VCN              LCN               Run length\n");
    let lcn_str: [*const c_char; 5] = [b"LCN_DELALLOC     \0".as_ptr() as _, b"LCN_HOLE         \0".as_ptr() as _, b"LCN_RL_NOT_MAPPED\0".as_ptr() as _, b"LCN_ENOENT       \0".as_ptr() as _, b"LCN_unknown      \0".as_ptr() as _];
    let mut i = 0usize;
    loop {
        let e = &*rl.add(i);
        if e.lcn < 0 {
            let mut index = -e.lcn - 1;
            if index > 3 { index = 3; }
            pr_debug!("%-16Lx %s %-16Lx%s\n", e.vcn as c_longlong, lcn_str[index as usize], e.length as c_longlong, if e.length != 0 { b"\0".as_ptr() } else { b" (runlist end)\0".as_ptr() });
        } else {
            pr_debug!("%-16Lx %-16Lx  %-16Lx%s\n", e.vcn as c_longlong, e.lcn as c_longlong, e.length as c_longlong, if e.length != 0 { b"\0".as_ptr() } else { b" (runlist end)\0".as_ptr() });
        }
        if e.length == 0 { break; }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
