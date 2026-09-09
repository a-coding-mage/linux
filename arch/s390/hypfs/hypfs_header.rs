/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Hypervisor filesystem for Linux on s390.
 *
 *    Copyright IBM Corp. 2006
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub const REG_FILE_MODE: u32 = 0o440;
pub const UPDATE_FILE_MODE: u32 = 0o220;
pub const DIR_MODE: u32 = 0o550;

extern "C" {
    pub fn hypfs_mkdir(parent: *mut dentry, name: *const c_char) -> *mut dentry;

    pub fn hypfs_create_u64(dir: *mut dentry, name: *const c_char, value: u64) -> c_int;

    pub fn hypfs_create_str(
        dir: *mut dentry,
        name: *const c_char,
        string: *mut c_char,
    ) -> c_int;

    /* LPAR Hypervisor */
    pub fn hypfs_diag_init() -> c_int;
    pub fn hypfs_diag_exit();
    pub fn hypfs_diag_create_files(root: *mut dentry) -> c_int;

    /* VM Hypervisor */
    pub fn hypfs_vm_init() -> c_int;
    pub fn hypfs_vm_exit();
    pub fn hypfs_vm_create_files(root: *mut dentry) -> c_int;

    /* VM diagnose 0c */
    pub fn hypfs_diag0c_init() -> c_int;
    pub fn hypfs_diag0c_exit();

    /* Set Partition-Resource Parameter */
    pub fn hypfs_sprp_init();
    pub fn hypfs_sprp_exit();

    pub fn __hypfs_fs_init() -> c_int;

    pub fn hypfs_dbfs_create_file(df: *mut hypfs_dbfs_file);
    pub fn hypfs_dbfs_remove_file(df: *mut hypfs_dbfs_file);
}

#[inline(always)]
pub unsafe fn hypfs_fs_init() -> c_int {
    // IS_ENABLED(CONFIG_S390_HYPFS_FS) is a build-time kernel configuration condition.
    #[cfg(CONFIG_S390_HYPFS_FS)]
    {
        return __hypfs_fs_init();
    }
    0
}

/* debugfs interface */
#[repr(C)]
pub struct hypfs_dbfs_data {
    pub buf: *mut c_void,
    pub buf_free_ptr: *mut c_void,
    pub size: usize,
    pub dbfs_file: *mut hypfs_dbfs_file,
}

#[repr(C)]
pub struct hypfs_dbfs_file {
    pub name: *const c_char,
    pub data_create: Option<
        unsafe extern "C" fn(
            data: *mut *mut c_void,
            data_free_ptr: *mut *mut c_void,
            size: *mut usize,
        ) -> c_int,
    >,
    pub data_free: Option<unsafe extern "C" fn(buf_free_ptr: *const c_void)>,
    pub unlocked_ioctl: Option<
        unsafe extern "C" fn(
            file: *mut file,
            cmd: u32,
            arg: c_ulong,
        ) -> c_long,
    >,

    /* Private data for hypfs_dbfs.c */
    pub lock: mutex,
    pub dentry: *mut dentry,
}

// C long has platform-dependent width; this header targets the Linux s390 ABI.
pub type c_long = isize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
