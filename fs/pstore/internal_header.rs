/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux/Rust translation units.

use core::ffi::{c_char, c_int, c_void};

pub type ssize_t = isize;
pub type size_t = usize;

// Opaque types declared by the included headers.
#[repr(C)]
pub struct pstore_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pstore_record {
    _private: [u8; 0],
}

pub static mut kmsg_bytes: u32;

#[cfg(CONFIG_PSTORE_FTRACE)]
extern "C" {
    pub fn decode_ip(ip: usize) -> usize;
    pub fn pstore_register_ftrace();
    pub fn pstore_unregister_ftrace();
    pub fn pstore_ftrace_combine_log(
        dest_log: *mut *mut c_char,
        dest_log_size: *mut size_t,
        src_log: *const c_char,
        src_log_size: size_t,
    ) -> ssize_t;
}

#[cfg(not(CONFIG_PSTORE_FTRACE))]
#[inline]
pub unsafe fn pstore_register_ftrace() {}

#[cfg(not(CONFIG_PSTORE_FTRACE))]
#[inline]
pub unsafe fn pstore_unregister_ftrace() {}

#[cfg(not(CONFIG_PSTORE_FTRACE))]
#[inline]
pub unsafe fn decode_ip(ip: usize) -> usize {
    ip
}

#[cfg(not(CONFIG_PSTORE_FTRACE))]
#[inline]
pub unsafe fn pstore_ftrace_combine_log(
    _dest_log: *mut *mut c_char,
    dest_log_size: *mut size_t,
    _src_log: *const c_char,
    _src_log_size: size_t,
) -> ssize_t {
    *dest_log_size = 0;
    0
}

#[cfg(CONFIG_PSTORE_PMSG)]
extern "C" {
    pub fn pstore_register_pmsg();
    pub fn pstore_unregister_pmsg();
}

#[cfg(not(CONFIG_PSTORE_PMSG))]
#[inline]
pub unsafe fn pstore_register_pmsg() {}

#[cfg(not(CONFIG_PSTORE_PMSG))]
#[inline]
pub unsafe fn pstore_unregister_pmsg() {}

extern "C" {
    pub static mut psinfo: *mut pstore_info;

    pub fn pstore_set_kmsg_bytes(bytes: u32);
    pub fn pstore_get_records(arg: c_int);
    pub fn pstore_get_backend_records(
        psi: *mut pstore_info,
        root: *mut dentry,
        quiet: c_int,
    );
    pub fn pstore_put_backend_records(psi: *mut pstore_info) -> c_int;
    pub fn pstore_mkfile(root: *mut dentry, record: *mut pstore_record) -> c_int;
    pub fn pstore_record_init(record: *mut pstore_record, psi: *mut pstore_info);

    /* Called during pstore init/exit. */
    pub fn pstore_init_fs() -> c_int;
    pub fn pstore_exit_fs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
