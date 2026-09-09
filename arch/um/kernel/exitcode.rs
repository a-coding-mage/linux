// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the Linux/UML kernel environment.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_ops {
    pub proc_open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub proc_read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut i64) -> isize>,
    pub proc_lseek: Option<unsafe extern "C" fn(*mut file, i64, c_int) -> i64>,
    pub proc_release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub proc_write:
        Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut i64) -> isize>,
}

extern "C" {
    fn seq_printf(m: *mut seq_file, format: *const c_char, ...) -> c_int;
    fn single_open(
        file: *mut file,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn simple_strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> i64;
    fn proc_create(
        name: *const c_char,
        mode: u32,
        parent: *mut proc_dir_entry,
        proc_ops: *const proc_ops,
    ) -> *mut proc_dir_entry;
    fn printk(format: *const c_char, ...) -> c_int;
    fn seq_read(file: *mut file, buffer: *mut c_char, count: usize, pos: *mut i64) -> isize;
    fn seq_lseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn single_release(inode: *mut inode, file: *mut file) -> c_int;
}

/*
 * If read and write race, the read will still atomically read a valid
 * value.
 */
#[no_mangle]
pub static mut uml_exitcode: c_int = 0;

unsafe extern "C" fn exitcode_proc_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let val: c_int;

    /*
     * Save uml_exitcode in a local so that we don't need to guarantee
     * that sprintf accesses it atomically.
     */
    val = uml_exitcode;
    seq_printf(m, b"%d\n\0".as_ptr() as *const c_char, val);
    0
}

unsafe extern "C" fn exitcode_proc_open(_inode: *mut inode, file: *mut file) -> c_int {
    single_open(file, exitcode_proc_show, core::ptr::null_mut())
}

unsafe extern "C" fn exitcode_proc_write(
    _file: *mut file,
    buffer: *const c_char,
    count: usize,
    _pos: *mut i64,
) -> isize {
    let mut buf = [0i8; core::mem::size_of::<[u8; 6]>()];
    let size = core::cmp::min(count, buf.len());

    if copy_from_user(buf.as_mut_ptr() as *mut c_void, buffer as *const c_void, size) != 0 {
        return -14;
    }

    let mut end: *mut c_char = core::ptr::null_mut();
    let tmp = simple_strtol(buf.as_ptr(), &mut end, 0) as c_int;
    if !end.is_null() && *end != 0 && !((*end as u8).is_ascii_whitespace()) {
        return -22;
    }

    uml_exitcode = tmp;
    count as isize
}

static EXITCODE_PROC_OPS: proc_ops = proc_ops {
    proc_open: Some(exitcode_proc_open),
    proc_read: Some(seq_read),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(single_release),
    proc_write: Some(exitcode_proc_write),
};

unsafe extern "C" fn make_proc_exitcode() -> c_int {
    let ent = proc_create(
        b"exitcode\0".as_ptr() as *const c_char,
        0o600,
        core::ptr::null_mut(),
        &EXITCODE_PROC_OPS,
    );
    if ent.is_null() {
        printk(
            b"make_proc_exitcode : Failed to register /proc/exitcode\n\0".as_ptr()
                as *const c_char,
        );
        return 0;
    }
    0
}

// __initcall(make_proc_exitcode);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
