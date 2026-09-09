// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Dependencies supplied by the kernel and other translation units.
use core::ffi::{c_char, c_int, c_void};

type SizeT = usize;
type LoffT = i64;
type SsizeT = isize;

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dentry {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bc_prefetch_is_enabled() -> bool;
    fn bc_prefetch_enable();
    fn bc_prefetch_disable();
    fn simple_read_from_buffer(
        to: *mut c_void,
        count: SizeT,
        ppos: *mut LoffT,
        from: *const c_void,
        available: SizeT,
    ) -> SsizeT;
    fn kstrtobool_from_user(
        s: *const c_char,
        count: SizeT,
        result: *mut bool,
    ) -> c_int;
    fn simple_open(file: *mut File, inode: *mut c_void) -> c_int;
    fn default_llseek(file: *mut File, offset: LoffT, whence: c_int) -> LoffT;
    fn debugfs_create_dir(name: *const c_char, parent: *mut Dentry) -> *mut Dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: u32,
        parent: *mut Dentry,
        data: *mut c_void,
        fops: *const FileOperations,
    ) -> *mut Dentry;
    static mut mips_debugfs_dir: *mut Dentry;
}

#[repr(C)]
pub struct FileOperations {
    pub open: Option<unsafe extern "C" fn(*mut File, *mut c_void) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn(*mut File, LoffT, c_int) -> LoffT>,
    pub read: Option<unsafe extern "C" fn(*mut File, *mut c_char, SizeT, *mut LoffT) -> SsizeT>,
    pub write:
        Option<unsafe extern "C" fn(*mut File, *const c_char, SizeT, *mut LoffT) -> SsizeT>,
}

unsafe extern "C" fn sc_prefetch_read(
    _file: *mut File,
    user_buf: *mut c_char,
    count: SizeT,
    ppos: *mut LoffT,
) -> SsizeT {
    let enabled = unsafe { bc_prefetch_is_enabled() };
    let mut buf = [0 as c_char; 3];

    buf[0] = if enabled { b'Y' as c_char } else { b'N' as c_char };
    buf[1] = b'\n' as c_char;
    buf[2] = 0;

    unsafe {
        simple_read_from_buffer(user_buf.cast(), count, ppos, buf.as_ptr().cast(), 2)
    }
}

unsafe extern "C" fn sc_prefetch_write(
    _file: *mut File,
    user_buf: *const c_char,
    count: SizeT,
    _ppos: *mut LoffT,
) -> SsizeT {
    let mut enabled = false;
    let err = unsafe { kstrtobool_from_user(user_buf, count, &mut enabled) };
    if err != 0 {
        return err as SsizeT;
    }

    if enabled {
        unsafe { bc_prefetch_enable() };
    } else {
        unsafe { bc_prefetch_disable() };
    }

    count as SsizeT
}

static SC_PREFETCH_FOPS: FileOperations = FileOperations {
    open: Some(simple_open),
    llseek: Some(default_llseek),
    read: Some(sc_prefetch_read),
    write: Some(sc_prefetch_write),
};

unsafe extern "C" fn sc_debugfs_init() -> c_int {
    let dir = unsafe {
        debugfs_create_dir(
            b"l2cache\0".as_ptr().cast(),
            mips_debugfs_dir,
        )
    };
    unsafe {
        debugfs_create_file(
            b"prefetch\0".as_ptr().cast(),
            0o444 | 0o200,
            dir,
            core::ptr::null_mut(),
            &SC_PREFETCH_FOPS,
        );
    }
    0
}

// late_initcall(sc_debugfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
