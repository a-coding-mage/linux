// SPDX-License-Identifier: GPL-2.0
//
// The definitions below depend on kernel and UBIFS declarations supplied by
// the surrounding translation unit.

use core::ffi::{c_char, c_int};

extern "C" {
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct task_struct {
    pub pid: c_int,
}

#[repr(C)]
pub struct va_format {
    pub fmt: *const c_char,
    pub va: *mut va_list,
}

#[repr(C)]
pub struct va_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ubifs_info {
    pub vi: ubifs_volume_info,
    pub assert_action: usize,
}

#[repr(C)]
pub struct ubifs_volume_info {
    pub ubi_num: c_int,
    pub vol_id: c_int,
}

extern "C" {
    fn pr_notice(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
}

extern "C" {
    fn va_start(ap: *mut va_list, last: *const c_char);
    fn va_end(ap: *mut va_list);
}

pub const ASSACT_REPORT: usize = 0;
pub const ASSACT_RO: usize = 1;
pub const ASSACT_PANIC: usize = 2;

/* Normal UBIFS messages */
pub unsafe extern "C" fn ubifs_msg(c: *const ubifs_info, fmt: *const c_char, ...) {
    let mut args = va_list { _private: [] };
    let mut vaf: va_format;

    va_start(&mut args, fmt);

    vaf.fmt = fmt;
    vaf.va = &mut args;

    pr_notice(
        b"UBIFS (ubi%d:%d): %pV\n\0".as_ptr() as *const c_char,
        (*c).vi.ubi_num,
        (*c).vi.vol_id,
        &vaf as *const va_format,
    );

    va_end(&mut args);
}

/* UBIFS error messages */
pub unsafe extern "C" fn ubifs_err(c: *const ubifs_info, fmt: *const c_char, ...) {
    let mut args = va_list { _private: [] };
    let mut vaf: va_format;

    va_start(&mut args, fmt);

    vaf.fmt = fmt;
    vaf.va = &mut args;

    pr_err(
        b"UBIFS error (ubi%d:%d pid %d): %ps: %pV\n\0".as_ptr() as *const c_char,
        (*c).vi.ubi_num,
        (*c).vi.vol_id,
        (*current).pid,
        0usize,
        &vaf as *const va_format,
    );

    va_end(&mut args);
}

/* UBIFS warning messages */
pub unsafe extern "C" fn ubifs_warn(c: *const ubifs_info, fmt: *const c_char, ...) {
    let mut args = va_list { _private: [] };
    let mut vaf: va_format;

    va_start(&mut args, fmt);

    vaf.fmt = fmt;
    vaf.va = &mut args;

    pr_warn(
        b"UBIFS warning (ubi%d:%d pid %d): %ps: %pV\n\0".as_ptr() as *const c_char,
        (*c).vi.ubi_num,
        (*c).vi.vol_id,
        (*current).pid,
        0usize,
        &vaf as *const va_format,
    );

    va_end(&mut args);
}

static mut assert_names: [*const c_char; 3] = [
    b"report\0".as_ptr() as *const c_char,
    b"read-only\0".as_ptr() as *const c_char,
    b"panic\0".as_ptr() as *const c_char,
];

pub unsafe extern "C" fn ubifs_assert_action_name(c: *mut ubifs_info) -> *const c_char {
    assert_names[(*c).assert_action]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
