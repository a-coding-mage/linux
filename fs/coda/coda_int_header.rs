/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations corresponding to the C header's incomplete structs.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_system_type {
    _private: [u8; 0],
}

pub type loff_t = i64;

unsafe extern "C" {
    pub static mut coda_fs_type: file_system_type;
    pub static mut coda_timeout: ::core::ffi::c_ulong;
    pub static mut coda_hard: ::core::ffi::c_int;
    pub static mut coda_fake_statfs: ::core::ffi::c_int;

    pub fn coda_destroy_inodecache();
    // The C declaration carries the kernel's __init attribute.
    pub fn coda_init_inodecache() -> ::core::ffi::c_int;
    pub fn coda_fsync(
        coda_file: *mut file,
        start: loff_t,
        end: loff_t,
        datasync: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// When CONFIG_SYSCTL is enabled, these are supplied externally by the C
// implementation. The alternate declarations below preserve the header's
// empty inline implementations when it is disabled.
#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" {
    pub fn coda_sysctl_init();
    pub fn coda_sysctl_clean();
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
#[inline]
pub unsafe fn coda_sysctl_init() {}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
#[inline]
pub unsafe fn coda_sysctl_clean() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
