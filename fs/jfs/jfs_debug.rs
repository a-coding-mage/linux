// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

// Kernel dependencies supplied by the surrounding repository.
// Build-time conditions are preserved from the original source.

#[cfg(all(proc_fs_jfs, config_jfs_debug))]
unsafe extern "C" {
    static mut jfsloglevel: ::kernel::ffi::c_int;

    fn seq_printf(
        m: *mut seq_file,
        fmt: *const ::kernel::ffi::c_char,
        ...,
    ) -> ::kernel::ffi::c_int;
    fn single_open(
        file: *mut file,
        show: unsafe extern "C" fn(*mut seq_file, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int,
        data: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    fn get_user(c: *mut ::kernel::ffi::c_char, buffer: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    fn seq_read() -> ::kernel::ffi::c_int;
    fn seq_lseek() -> ::kernel::ffi::c_int;
    fn single_release() -> ::kernel::ffi::c_int;
}

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

#[cfg(all(proc_fs_jfs, config_jfs_debug))]
unsafe extern "C" {
    fn proc_mkdir(name: *const ::kernel::ffi::c_char, parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    fn proc_create_single(
        name: *const ::kernel::ffi::c_char,
        mode: ::kernel::ffi::c_uint,
        parent: *mut proc_dir_entry,
        show: unsafe extern "C" fn(*mut seq_file, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int,
    ) -> *mut proc_dir_entry;
    fn proc_create(
        name: *const ::kernel::ffi::c_char,
        mode: ::kernel::ffi::c_uint,
        parent: *mut proc_dir_entry,
        ops: *const proc_ops,
    ) -> *mut proc_dir_entry;
    fn remove_proc_subtree(name: *const ::kernel::ffi::c_char, parent: *mut proc_dir_entry);
}

#[repr(C)]
pub struct proc_ops {
    pub proc_open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> ::kernel::ffi::c_int>,
    pub proc_read: Option<unsafe extern "C" fn() -> ::kernel::ffi::c_int>,
    pub proc_lseek: Option<unsafe extern "C" fn() -> ::kernel::ffi::c_int>,
    pub proc_release: Option<unsafe extern "C" fn() -> ::kernel::ffi::c_int>,
    pub proc_write: Option<unsafe extern "C" fn() -> ::kernel::ffi::c_int>,
}

#[cfg(all(proc_fs_jfs, config_jfs_debug))]
unsafe extern "C" fn jfs_loglevel_proc_show(m: *mut seq_file, _v: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int {
    let fmt = b"%d\n\0";
    seq_printf(m, fmt.as_ptr() as *const ::kernel::ffi::c_char, jfsloglevel);
    0
}

#[cfg(all(proc_fs_jfs, config_jfs_debug))]
unsafe extern "C" fn jfs_loglevel_proc_open(_inode: *mut inode, file: *mut file) -> ::kernel::ffi::c_int {
    single_open(file, jfs_loglevel_proc_show, ::core::ptr::null_mut())
}

#[cfg(all(proc_fs_jfs, config_jfs_debug))]
unsafe extern "C" fn jfs_loglevel_proc_write(
    _file: *mut file,
    buffer: *const ::kernel::ffi::c_char,
    count: usize,
    _ppos: *mut i64,
) -> isize {
    let mut c: ::kernel::ffi::c_char = 0;
    if get_user(&mut c, buffer) != 0 {
        return -14;
    }
    if c < b'0' as ::kernel::ffi::c_char || c > b'9' as ::kernel::ffi::c_char {
        return -22;
    }
    jfsloglevel = (c - b'0' as ::kernel::ffi::c_char) as ::kernel::ffi::c_int;
    count as isize
}

#[cfg(all(proc_fs_jfs, config_jfs_debug))]
static JFS_LOGLEVEL_PROC_OPS: proc_ops = proc_ops {
    proc_open: Some(jfs_loglevel_proc_open),
    proc_read: Some(seq_read),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(single_release),
    proc_write: Some(jfs_loglevel_proc_write),
};

#[cfg(proc_fs_jfs)]
pub unsafe extern "C" fn jfs_proc_init() {
    let base = proc_mkdir(b"fs/jfs\0".as_ptr() as *const ::kernel::ffi::c_char, ::core::ptr::null_mut());
    if base.is_null() {
        return;
    }

    #[cfg(config_jfs_statistics)]
    {
        proc_create_single(b"lmstats\0".as_ptr() as *const _, 0, base, jfs_lmstats_proc_show);
        proc_create_single(b"txstats\0".as_ptr() as *const _, 0, base, jfs_txstats_proc_show);
        proc_create_single(b"xtstat\0".as_ptr() as *const _, 0, base, jfs_xtstat_proc_show);
        proc_create_single(b"mpstat\0".as_ptr() as *const _, 0, base, jfs_mpstat_proc_show);
    }
    #[cfg(config_jfs_debug)]
    {
        proc_create_single(b"TxAnchor\0".as_ptr() as *const _, 0, base, jfs_txanchor_proc_show);
        proc_create(b"loglevel\0".as_ptr() as *const _, 0, base, &JFS_LOGLEVEL_PROC_OPS);
    }
}

#[cfg(proc_fs_jfs)]
pub unsafe extern "C" fn jfs_proc_clean() {
    remove_proc_subtree(b"fs/jfs\0".as_ptr() as *const ::kernel::ffi::c_char, ::core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
