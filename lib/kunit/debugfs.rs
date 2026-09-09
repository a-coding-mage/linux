// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020, Oracle and/or its affiliates.
 *    Author: Alan Maguire <alan.maguire@oracle.com>
 */

// External kernel, KUnit, and string-stream declarations are supplied by
// other translation units.

const KUNIT_DEBUGFS_ROOT: *const core::ffi::c_char = b"kunit\0".as_ptr() as *const _;
const KUNIT_DEBUGFS_RESULTS: *const core::ffi::c_char = b"results\0".as_ptr() as *const _;
const KUNIT_DEBUGFS_RUN: *const core::ffi::c_char = b"run\0".as_ptr() as *const _;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    pub i_private: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct file {
    pub f_inode: *mut inode,
}
#[repr(C)]
pub struct seq_file {
    pub private: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct string_stream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kunit_suite {
    pub status: kunit_status,
    pub name: *const core::ffi::c_char,
    pub log: *mut string_stream,
    pub debugfs: *mut dentry,
    pub is_init: bool,
    pub status_comment: *const core::ffi::c_char,
}
#[repr(C)]
pub struct kunit_case {
    pub log: *mut string_stream,
}
#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    pub read: Option<unsafe extern "C" fn()>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize, *mut i64) -> isize>,
    pub llseek: Option<unsafe extern "C" fn()>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
}

type kunit_status = i32;

extern "C" {
    static mut debugfs_rootdir: *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn debugfs_create_dir(name: *const core::ffi::c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const core::ffi::c_char, mode: u32, parent: *mut dentry,
                           data: *mut core::ffi::c_void, fops: *const file_operations) -> *mut dentry;
    fn single_release(inode: *mut inode, file: *mut file) -> i32;
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32,
                   data: *mut kunit_suite) -> i32;
    fn seq_read();
    fn seq_lseek();
    fn seq_printf(seq: *mut seq_file, format: *const core::ffi::c_char, ...);
    fn seq_puts(seq: *mut seq_file, string: *const core::ffi::c_char);
    fn kunit_suite_has_succeeded(suite: *mut kunit_suite) -> kunit_status;
    fn kunit_suite_num_test_cases(suite: *mut kunit_suite) -> isize;
    fn kunit_status_to_ok_not_ok(status: kunit_status) -> *const core::ffi::c_char;
    fn alloc_string_stream(gfp: u32) -> *mut string_stream;
    fn string_stream_set_append_newlines(stream: *mut string_stream, append: bool);
    fn string_stream_destroy(stream: *mut string_stream);
    fn __kunit_test_suites_init(suites: *mut *mut kunit_suite, num_suites: usize, init: bool);
    fn kunit_suite_for_each_test_case(suite: *mut kunit_suite, case_: *mut *mut kunit_case,
                                      callback: unsafe extern "C" fn(*mut kunit_case));
    fn debugfs_print_result(seq: *mut seq_file, log: *mut string_stream);
}

#[no_mangle]
pub unsafe extern "C" fn kunit_debugfs_cleanup() {
    debugfs_remove_recursive(debugfs_rootdir);
}

#[no_mangle]
pub unsafe extern "C" fn kunit_debugfs_init() {
    if debugfs_rootdir.is_null() {
        debugfs_rootdir = debugfs_create_dir(KUNIT_DEBUGFS_ROOT, core::ptr::null_mut());
    }
}

unsafe extern "C" fn debugfs_print_results(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let suite = (*seq).private as *mut kunit_suite;
    if suite.is_null() { return 0; }
    let success = kunit_suite_has_succeeded(suite);
    seq_puts(seq, b"KTAP version 1\n\0".as_ptr() as *const _);
    seq_puts(seq, b"1..1\n\0".as_ptr() as *const _);
    if (*suite).status != 4 {
        seq_puts(seq, b"    KTAP version 1\n\0".as_ptr() as *const _);
        seq_printf(seq, b"    # Subtest: %s\n\0".as_ptr() as *const _, (*suite).name);
        seq_printf(seq, b"    1..%zd\n\0".as_ptr() as *const _, kunit_suite_num_test_cases(suite));
    }
    debugfs_print_result(seq, (*suite).log);
    if (*suite).status != 4 {
        seq_printf(seq, b"%s %d %s\n\0".as_ptr() as *const _, kunit_status_to_ok_not_ok(success), 1, (*suite).name);
    } else {
        seq_printf(seq, b"%s %d %s # SKIP %s\n\0".as_ptr() as *const _, kunit_status_to_ok_not_ok(success), 1, (*suite).name, (*suite).status_comment);
    }
    0
}

unsafe extern "C" fn debugfs_release(i: *mut inode, f: *mut file) -> i32 { single_release(i, f) }
unsafe extern "C" fn debugfs_results_open(i: *mut inode, f: *mut file) -> i32 { single_open(f, debugfs_print_results, (*i).i_private as *mut kunit_suite) }
unsafe extern "C" fn debugfs_print_run(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let suite = (*seq).private as *mut kunit_suite;
    seq_puts(seq, b"Write to this file to trigger the test suite to run.\n\0".as_ptr() as *const _);
    seq_printf(seq, b"usage: echo \"any string\" > /sys/kernel/debugfs/kunit/%s/run\n\0".as_ptr() as *const _, (*suite).name); 0
}
unsafe extern "C" fn debugfs_run_open(i: *mut inode, f: *mut file) -> i32 { single_open(f, debugfs_print_run, (*i).i_private as *mut kunit_suite) }
unsafe extern "C" fn debugfs_run(file: *mut file, _buf: *const core::ffi::c_char, count: usize, _ppos: *mut i64) -> isize {
    let suite = (*(*file).f_inode).i_private as *mut kunit_suite;
    __kunit_test_suites_init(&mut (suite), 1, true); count as isize
}

static DEBUGFS_RESULTS_FOPS: file_operations = file_operations {
    open: Some(debugfs_results_open), read: Some(seq_read), write: None,
    llseek: Some(seq_lseek), release: Some(debugfs_release),
};
static DEBUGFS_RUN_FOPS: file_operations = file_operations {
    open: Some(debugfs_run_open), read: Some(seq_read), write: Some(debugfs_run),
    llseek: Some(seq_lseek), release: Some(debugfs_release),
};

pub unsafe extern "C" fn kunit_debugfs_create_suite(suite: *mut kunit_suite) {
    if !(*suite).log.is_null() { return; }
    let stream = alloc_string_stream(0);
    if stream.is_null() { return; }
    string_stream_set_append_newlines(stream, true); (*suite).log = stream;
    (*suite).debugfs = debugfs_create_dir((*suite).name, debugfs_rootdir);
    debugfs_create_file(KUNIT_DEBUGFS_RESULTS, 0o100444, (*suite).debugfs,
                        suite as *mut core::ffi::c_void, &DEBUGFS_RESULTS_FOPS);
    if !(*suite).is_init {
        debugfs_create_file(KUNIT_DEBUGFS_RUN, 0o100644, (*suite).debugfs,
                            suite as *mut core::ffi::c_void, &DEBUGFS_RUN_FOPS);
    }
}

pub unsafe extern "C" fn kunit_debugfs_destroy_suite(suite: *mut kunit_suite) {
    debugfs_remove_recursive((*suite).debugfs);
    string_stream_destroy((*suite).log);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
