/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of lib/api/fs/tracing_path.h.
// C dependencies: <linux/types.h>, <dirent.h>, and free(3).

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn tracing_events__opendir() -> *mut DIR;
    pub fn tracing_events__scandir_alphasort(namelist: *mut *mut *mut dirent) -> c_int;

    pub fn tracing_path_set(mountpoint: *const c_char);
    pub fn tracing_path_mount() -> *const c_char;

    pub fn get_tracing_file(name: *const c_char) -> *mut c_char;
    pub fn put_tracing_file(file: *mut c_char);

    pub fn get_events_file(name: *const c_char) -> *mut c_char;
    pub fn put_events_file(file: *mut c_char);

    pub fn free(ptr: *mut c_void);

    pub fn tracing_path__strerror_open_tp(
        err: c_int,
        buf: *mut c_char,
        size: size_t,
        sys: *const c_char,
        name: *const c_char,
    ) -> c_int;
}

// #define zput_events_file(ptr) ({ free(*ptr); *ptr = NULL; })
pub unsafe fn zput_events_file(ptr: *mut *mut c_char) {
    unsafe {
        free((*ptr).cast::<c_void>());
        *ptr = core::ptr::null_mut();
    }
}
