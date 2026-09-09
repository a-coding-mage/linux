// SPDX-License-Identifier: GPL-2.0-or-later OR copyleft-next-0.3.1
/*
 * proc sysctl test driver
 *
 * Copyright (C) 2017 Luis R. Rodriguez <mcgrof@kernel.org>
 */

/*
 * This module provides an interface to the proc sysctl interfaces.  This
 * driver requires CONFIG_SYSCTL. It will not normally be loaded by the
 * system unless explicitly requested by name. You can also build this driver
 * into your kernel.
 */

// Kernel headers supplied by external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: c_uint,
    pub proc_handler: Option<unsafe extern "C" fn()>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

extern "C" {
    static SYSCTL_ZERO: c_void;
    static SYSCTL_ONE: c_void;
    static SYSCTL_TWO: c_void;
    static SYSCTL_THREE: c_void;
    static SYSCTL_FOUR: c_void;
    static SYSCTL_ONE_HUNDRED: c_void;
    static SYSCTL_TWO_HUNDRED: c_void;
    static SYSCTL_ONE_THOUSAND: c_void;
    static SYSCTL_THREE_THOUSAND: c_void;
    static SYSCTL_INT_MAX: c_void;
    static SYSCTL_MAXOLDUID: c_void;
    static SYSCTL_NEG_ONE: c_void;

    fn proc_dointvec();
    fn proc_dointvec_minmax();
    fn proc_douintvec();
    fn proc_dostring();
    fn proc_do_large_bitmap();
    fn proc_dou8vec_minmax();
    fn register_sysctl(path: *const c_char, table: *const ctl_table) -> *mut ctl_table_header;
    fn register_sysctl_mount_point(path: *const c_char) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SYSCTL_TEST_BITMAP_SIZE: usize = 65536;

static mut i_zero: c_int = 0;
static mut i_one_hundred: c_int = 100;
static mut match_int_ok: c_int = 1;

const TEST_H_SETUP_NODE: usize = 0;
const TEST_H_MNT: usize = 1;
const TEST_H_MNTERROR: usize = 2;
const TEST_H_EMPTY_ADD: usize = 3;
const TEST_H_EMPTY: usize = 4;
const TEST_H_U8: usize = 5;
const TEST_H_SIZE: usize = 6; // Always at the end

static mut ctl_headers: [*mut ctl_table_header; TEST_H_SIZE] = [core::ptr::null_mut(); TEST_H_SIZE];

#[repr(C)]
struct test_sysctl_data {
    int_0001: c_int,
    int_0002: c_int,
    int_0003: [c_int; 4],
    boot_int: c_int,
    uint_0001: c_uint,
    string_0001: [c_char; 65],
    bitmap_0001: *mut c_ulong,
}

static mut test_data: test_sysctl_data = test_sysctl_data {
    int_0001: 60,
    int_0002: 1,
    int_0003: [0, 1, 2, 3],
    boot_int: 0,
    uint_0001: 314,
    string_0001: [0; 65],
    bitmap_0001: core::ptr::null_mut(),
};

// These are all under /proc/sys/debug/test_sysctl/
static mut test_table: [ctl_table; 8] = [
    ctl_table { procname: b"int_0001\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: b"int_0002\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: b"int_0003\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<[c_int; 4]>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: b"match_int\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<c_int>(), mode: 0o444, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: b"boot_int\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: b"uint_0001\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<c_uint>(), mode: 0o644, proc_handler: Some(proc_douintvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: b"string_0001\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: 65, mode: 0o644, proc_handler: Some(proc_dostring), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: b"bitmap_0001\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: SYSCTL_TEST_BITMAP_SIZE, mode: 0o644, proc_handler: Some(proc_do_large_bitmap), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
];

static mut test_table_unregister: [ctl_table; 1] = [ctl_table { procname: b"unregister_error\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() }];
static test_table_empty: [ctl_table; 0] = [];
static mut table_u8_over: [ctl_table; 1] = [ctl_table { procname: b"u8_over\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: 1, mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() }];
static mut table_u8_under: [ctl_table; 1] = [ctl_table { procname: b"u8_under\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: 1, mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() }];
static mut table_u8_valid: [ctl_table; 1] = [ctl_table { procname: b"u8_valid\0".as_ptr() as *const c_char, data: core::ptr::null_mut(), maxlen: 1, mode: 0o644, proc_handler: Some(proc_dou8vec_minmax), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() }];

unsafe fn test_sysctl_calc_match_int_ok() {
    let match_int = [
        (*( &SYSCTL_ZERO as *const _ as *const c_int), 0), (*( &SYSCTL_ONE as *const _ as *const c_int), 1),
        (*( &SYSCTL_TWO as *const _ as *const c_int), 2), (*( &SYSCTL_THREE as *const _ as *const c_int), 3),
        (*( &SYSCTL_FOUR as *const _ as *const c_int), 4), (*( &SYSCTL_ONE_HUNDRED as *const _ as *const c_int), 100),
        (*( &SYSCTL_TWO_HUNDRED as *const _ as *const c_int), 200), (*( &SYSCTL_ONE_THOUSAND as *const _ as *const c_int), 1000),
        (*( &SYSCTL_THREE_THOUSAND as *const _ as *const c_int), 3000), (*( &SYSCTL_INT_MAX as *const _ as *const c_int), c_int::MAX),
        (*( &SYSCTL_MAXOLDUID as *const _ as *const c_int), 65535), (*( &SYSCTL_NEG_ONE as *const _ as *const c_int), -1),
    ];
    for (defined, wanted) in match_int { if defined != wanted { match_int_ok = 0; } }
}

unsafe fn test_sysctl_setup_node_tests() -> c_int {
    test_sysctl_calc_match_int_ok();
    test_data.bitmap_0001 = kzalloc(SYSCTL_TEST_BITMAP_SIZE / 8, GFP_KERNEL) as *mut c_ulong;
    if test_data.bitmap_0001.is_null() { return -ENOMEM; }
    ctl_headers[TEST_H_SETUP_NODE] = register_sysctl(b"debug/test_sysctl\0".as_ptr() as *const c_char, test_table.as_ptr());
    if ctl_headers[TEST_H_SETUP_NODE].is_null() { kfree(test_data.bitmap_0001 as *mut c_void); return -ENOMEM; }
    0
}

unsafe fn test_sysctl_run_unregister_nested() -> c_int {
    let unregister = register_sysctl(b"debug/test_sysctl/unregister_error\0".as_ptr() as *const c_char, test_table_unregister.as_ptr());
    if unregister.is_null() { return -ENOMEM; }
    unregister_sysctl_table(unregister); 0
}

unsafe fn test_sysctl_run_register_mount_point() -> c_int {
    ctl_headers[TEST_H_MNT] = register_sysctl_mount_point(b"debug/test_sysctl/mnt\0".as_ptr() as *const c_char);
    if ctl_headers[TEST_H_MNT].is_null() { return -ENOMEM; }
    ctl_headers[TEST_H_MNTERROR] = register_sysctl(b"debug/test_sysctl/mnt/mnt_error\0".as_ptr() as *const c_char, test_table_unregister.as_ptr());
    // Don't check the result: expected failure is success; success exposes mnt_error to the test script.
    0
}

unsafe fn test_sysctl_run_register_empty() -> c_int {
    // Test that an empty dir can be created.
    ctl_headers[TEST_H_EMPTY_ADD] = register_sysctl(b"debug/test_sysctl/empty_add\0".as_ptr() as *const c_char, test_table_empty.as_ptr());
    if ctl_headers[TEST_H_EMPTY_ADD].is_null() { return -ENOMEM; }
    // Test that register on top of an empty dir works.
    ctl_headers[TEST_H_EMPTY] = register_sysctl(b"debug/test_sysctl/empty_add/empty\0".as_ptr() as *const c_char, test_table_empty.as_ptr());
    if ctl_headers[TEST_H_EMPTY].is_null() { return -ENOMEM; }
    0
}

unsafe fn test_sysctl_register_u8_extra() -> c_int {
    ctl_headers[TEST_H_U8] = register_sysctl(b"debug/test_sysctl\0".as_ptr() as *const c_char, table_u8_over.as_ptr());
    if !ctl_headers[TEST_H_U8].is_null() { return -ENOMEM; }
    ctl_headers[TEST_H_U8] = register_sysctl(b"debug/test_sysctl\0".as_ptr() as *const c_char, table_u8_under.as_ptr());
    if !ctl_headers[TEST_H_U8].is_null() { return -ENOMEM; }
    ctl_headers[TEST_H_U8] = register_sysctl(b"debug/test_sysctl\0".as_ptr() as *const c_char, table_u8_valid.as_ptr());
    if ctl_headers[TEST_H_U8].is_null() { return -ENOMEM; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn test_sysctl_init() -> c_int {
    let funcs: [unsafe fn() -> c_int; 5] = [test_sysctl_setup_node_tests, test_sysctl_run_unregister_nested, test_sysctl_run_register_mount_point, test_sysctl_run_register_empty, test_sysctl_register_u8_extra];
    let mut err = 0;
    let mut i = 0;
    while err == 0 && i < funcs.len() { err = funcs[i](); i += 1; }
    err
}

#[no_mangle]
pub unsafe extern "C" fn test_sysctl_exit() {
    kfree(test_data.bitmap_0001 as *mut c_void);
    for header in ctl_headers.iter() { if !(*header).is_null() { unregister_sysctl_table(*header); } }
}

// MODULE_AUTHOR("Luis R. Rodriguez <mcgrof@kernel.org>");
// MODULE_DESCRIPTION("proc sysctl test driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
