/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// The original header guard and C includes are intentionally omitted.
// CONFIG_PROC_FS is a build-time condition from the C environment; the
// corresponding Rust declarations are retained under the same feature name.

#[repr(C)]
pub struct ksmbd_share_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nls_table {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kstat {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ksmbd_file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct path {
    _private: [u8; 0],
}
#[repr(C)]
pub struct unicode_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ksmbd_dir_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

pub const KSMBD_DIR_INFO_ALIGNMENT: usize = 8;
pub const NTFS_TIME_OFFSET: u64 = (369u64 * 365 + 89) * 24 * 3600 * 10_000_000;

extern "C" {
    pub fn match_pattern(str_: *const core::ffi::c_char, len: usize,
                         pattern: *const core::ffi::c_char) -> i32;
    pub fn ksmbd_validate_filename(filename: *mut core::ffi::c_char) -> i32;
    pub fn parse_stream_name(filename: *mut core::ffi::c_char,
                             stream_name: *mut *mut core::ffi::c_char,
                             s_type: *mut i32) -> i32;
    pub fn convert_to_nt_pathname(share: *mut ksmbd_share_config,
                                  path: *const path) -> *mut core::ffi::c_char;
    pub fn get_nlink(st: *mut kstat) -> i32;
    pub fn ksmbd_conv_path_to_unix(path: *mut core::ffi::c_char);
    pub fn ksmbd_strip_last_slash(path: *mut core::ffi::c_char);
    pub fn ksmbd_conv_path_to_windows(path: *mut core::ffi::c_char);
    pub fn ksmbd_casefold_sharename(um: *mut unicode_map,
                                    name: *const core::ffi::c_char)
                                    -> *mut core::ffi::c_char;
    pub fn ksmbd_extract_sharename(um: *mut unicode_map,
                                   treename: *const core::ffi::c_char)
                                   -> *mut core::ffi::c_char;
    pub fn ksmbd_convert_dir_info_name(d_info: *mut ksmbd_dir_info,
                                       local_nls: *const nls_table,
                                       conv_len: *mut i32)
                                       -> *mut core::ffi::c_char;
    pub fn ksmbd_NTtimeToUnix(ntutc: u64) -> timespec64;
    pub fn ksmbd_UnixTimeToNT(t: timespec64) -> u64;
    pub fn ksmbd_systime() -> i64;
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[repr(C)]
pub struct ksmbd_const_name {
    pub const_value: u32,
    pub name: *const core::ffi::c_char,
}

#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub fn ksmbd_proc_init() -> i32;
    pub fn ksmbd_proc_cleanup();
    pub fn ksmbd_proc_reset();
    pub fn ksmbd_proc_create(name: *const core::ffi::c_char,
                             show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
                             v: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn ksmbd_proc_show_flag_names(m: *mut seq_file,
                                      table: *const ksmbd_const_name,
                                      count: i32,
                                      flags: u32);
    pub fn ksmbd_proc_const_name(table: *const ksmbd_const_name,
                                 count: i32,
                                 const_value: u32) -> *const core::ffi::c_char;
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn ksmbd_proc_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn ksmbd_proc_cleanup() {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub unsafe fn ksmbd_proc_reset() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
