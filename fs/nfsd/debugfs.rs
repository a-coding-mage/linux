// SPDX-License-Identifier: GPL-2.0

// Translated from debugfs.c. C header-provided dependencies remain external.

use core::ffi::c_void;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

extern "C" {
    static mut nfsd_disable_splice_read: bool;
    static mut nfsd_io_cache_read: u64;
    static mut nfsd_io_cache_write: u64;

    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn debugfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const u8,
        mode: u32,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    #[cfg(feature = "CONFIG_NFSD_V4")]
    fn debugfs_create_bool(
        name: *const u8,
        mode: u32,
        parent: *mut dentry,
        value: *mut bool,
    ) -> *mut dentry;
    #[cfg(feature = "CONFIG_NFSD_V4")]
    static mut nfsd_delegts_enabled: bool;
}

const NFSD_IO_BUFFERED: u64 = 0;
const NFSD_IO_DONTCACHE: u64 = 1;
const NFSD_IO_DIRECT: u64 = 2;
const EINVAL: i32 = 22;
const S_IWUSR: u32 = 0o200;
const S_IRUGO: u32 = 0o444;

static mut nfsd_dsr_fops: file_operations = file_operations { _private: [] };
static mut nfsd_io_cache_read_fops: file_operations = file_operations { _private: [] };
static mut nfsd_io_cache_write_fops: file_operations = file_operations { _private: [] };

static mut nfsd_top_dir: *mut dentry = core::ptr::null_mut();

unsafe fn nfsd_dsr_get(_data: *mut c_void, val: *mut u64) -> i32 {
    *val = if nfsd_disable_splice_read { 1 } else { 0 };
    0
}

unsafe fn nfsd_dsr_set(_data: *mut c_void, val: u64) -> i32 {
    nfsd_disable_splice_read = val > 0;
    if !nfsd_disable_splice_read {
        // Must use buffered I/O if splice_read is enabled.
        nfsd_io_cache_read = NFSD_IO_BUFFERED;
    }
    0
}

unsafe fn nfsd_io_cache_read_get(_data: *mut c_void, val: *mut u64) -> i32 {
    *val = nfsd_io_cache_read;
    0
}

unsafe fn nfsd_io_cache_read_set(_data: *mut c_void, val: u64) -> i32 {
    let mut ret = 0;
    match val {
        NFSD_IO_BUFFERED => {
            nfsd_io_cache_read = NFSD_IO_BUFFERED;
        }
        NFSD_IO_DONTCACHE | NFSD_IO_DIRECT => {
            // Must disable splice_read when enabling NFSD_IO_DONTCACHE.
            nfsd_disable_splice_read = true;
            nfsd_io_cache_read = val;
        }
        _ => ret = -EINVAL,
    }
    ret
}

unsafe fn nfsd_io_cache_write_get(_data: *mut c_void, val: *mut u64) -> i32 {
    *val = nfsd_io_cache_write;
    0
}

unsafe fn nfsd_io_cache_write_set(_data: *mut c_void, val: u64) -> i32 {
    let mut ret = 0;
    match val {
        NFSD_IO_BUFFERED | NFSD_IO_DONTCACHE | NFSD_IO_DIRECT => {
            nfsd_io_cache_write = val;
        }
        _ => ret = -EINVAL,
    }
    ret
}

pub unsafe fn nfsd_debugfs_exit() {
    debugfs_remove_recursive(nfsd_top_dir);
    nfsd_top_dir = core::ptr::null_mut();
}

pub unsafe fn nfsd_debugfs_init() {
    nfsd_top_dir = debugfs_create_dir(b"nfsd\0".as_ptr(), core::ptr::null_mut());

    debugfs_create_file(
        b"disable-splice-read\0".as_ptr(),
        S_IWUSR | S_IRUGO,
        nfsd_top_dir,
        core::ptr::null_mut(),
        &nfsd_dsr_fops,
    );

    debugfs_create_file(
        b"io_cache_read\0".as_ptr(),
        0o644,
        nfsd_top_dir,
        core::ptr::null_mut(),
        &nfsd_io_cache_read_fops,
    );

    debugfs_create_file(
        b"io_cache_write\0".as_ptr(),
        0o644,
        nfsd_top_dir,
        core::ptr::null_mut(),
        &nfsd_io_cache_write_fops,
    );

    #[cfg(feature = "CONFIG_NFSD_V4")]
    debugfs_create_bool(
        b"delegated_timestamps\0".as_ptr(),
        0o644,
        nfsd_top_dir,
        &mut nfsd_delegts_enabled,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
