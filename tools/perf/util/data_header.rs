/* SPDX-License-Identifier: GPL-2.0 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = i64;
pub type pid_t = c_int;
pub type u64 = u64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn fileno(stream: *mut FILE) -> c_int;
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perf_data_mode {
    PERF_DATA_MODE_WRITE,
    PERF_DATA_MODE_READ,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perf_dir_version {
    PERF_DIR_SINGLE_FILE = 0,
    PERF_DIR_VERSION = 1,
}

/**
 * struct perf_data_file: A wrapper around a file used for perf.data reading or writing. Generally
 * part of struct perf_data.
 */
#[repr(C)]
pub struct perf_data_file {
    /**
     * @path: Path of file. Generally a copy of perf_data.path but for a
     * directory it is the file within the directory.
     */
    pub path: *mut c_char,
    pub handle: perf_data_file__bindgen_ty_1,
    /** @size: Size of file when opened. */
    pub size: c_ulong,
    /** @use_stdio: Use buffered stdio operations. */
    pub use_stdio: bool,
}

#[repr(C)]
pub union perf_data_file__bindgen_ty_1 {
    /** @fd: File descriptor for read/writes. Valid if use_stdio is false. */
    pub fd: c_int,
    /**
     * @fptr: Stdio FILE. Valid if use_stdio is true, currently just
     * pipes in perf inject.
     */
    pub fptr: *mut FILE,
}

pub type c_ulong = u64;

/**
 * struct perf_data: A wrapper around a file used for perf.data reading or writing.
 */
#[repr(C)]
pub struct perf_data {
    /** @path: Path to open and of the file. NULL implies 'perf.data' will be used. */
    pub path: *const c_char,
    /** @file: Underlying file to be used. */
    pub file: perf_data_file,
    /** @open: Has the file or directory been opened. */
    pub open: bool,
    /** @is_pipe: Underlying file is a pipe. */
    pub is_pipe: bool,
    /** @is_dir: Underlying file is a directory. */
    pub is_dir: bool,
    /** @force: Ignore opening a file creating created by a different user. */
    pub force: bool,
    /** @in_place_update: A file opened for reading but will be written to. */
    pub in_place_update: bool,
    /** @mode: Read or write mode. C declares this as enum perf_data_mode mode:8. */
    pub mode: perf_data_mode,

    pub dir: perf_data__bindgen_ty_1,
}

#[repr(C)]
pub struct perf_data__bindgen_ty_1 {
    /** @version: perf_dir_version. */
    pub version: u64,
    /** @files: perf data files for the directory. */
    pub files: *mut perf_data_file,
    /** @nr: Number of perf data files for the directory. */
    pub nr: c_int,
}

#[inline]
pub unsafe fn perf_data_file__fd(file: *mut perf_data_file) -> c_int {
    if unsafe { (*file).use_stdio } {
        unsafe { fileno((*file).handle.fptr) }
    } else {
        unsafe { (*file).handle.fd }
    }
}

unsafe extern "C" {
    pub fn perf_data_file__write(file: *mut perf_data_file, buf: *mut c_void, size: size_t) -> ssize_t;
    pub fn perf_data_file__seek(file: *mut perf_data_file, offset: off_t, whence: c_int) -> off_t;
}

#[inline]
pub unsafe fn perf_data__is_read(data: *mut perf_data) -> bool {
    unsafe { (*data).mode == perf_data_mode::PERF_DATA_MODE_READ }
}

#[inline]
pub unsafe fn perf_data__is_write(data: *mut perf_data) -> bool {
    unsafe { (*data).mode == perf_data_mode::PERF_DATA_MODE_WRITE }
}

#[inline]
pub unsafe fn perf_data__is_pipe(data: *mut perf_data) -> c_int {
    unsafe { (*data).is_pipe as c_int }
}

#[inline]
pub unsafe fn perf_data__is_dir(data: *mut perf_data) -> bool {
    unsafe { (*data).is_dir }
}

#[inline]
pub unsafe fn perf_data__is_single_file(data: *mut perf_data) -> bool {
    unsafe { (*data).dir.version == perf_dir_version::PERF_DIR_SINGLE_FILE as u64 }
}

#[inline]
pub unsafe fn perf_data__fd(data: *mut perf_data) -> c_int {
    unsafe { perf_data_file__fd(&mut (*data).file) }
}

unsafe extern "C" {
    pub fn perf_data__open(data: *mut perf_data) -> c_int;
    pub fn perf_data__close(data: *mut perf_data);
    pub fn perf_data__read(data: *mut perf_data, buf: *mut c_void, size: size_t) -> ssize_t;
    pub fn perf_data__write(data: *mut perf_data, buf: *mut c_void, size: size_t) -> ssize_t;
    pub fn perf_data__seek(data: *mut perf_data, offset: off_t, whence: c_int) -> off_t;
    /*
     * If at_exit is set, only rename current perf.data to
     * perf.data.<postfix>, continue write on original data.
     * Set at_exit when flushing the last output.
     *
     * Return value is fd of new output.
     */
    pub fn perf_data__switch(
        data: *mut perf_data,
        postfix: *const c_char,
        pos: size_t,
        at_exit: bool,
        new_filepath: *mut *mut c_char,
    ) -> c_int;

    pub fn perf_data__create_dir(data: *mut perf_data, nr: c_int) -> c_int;
    pub fn perf_data__open_dir(data: *mut perf_data) -> c_int;
    pub fn perf_data__close_dir(data: *mut perf_data);
    pub fn perf_data__size(data: *mut perf_data) -> c_ulong;
    pub fn perf_data__make_kcore_dir(data: *mut perf_data, buf: *mut c_char, buf_sz: size_t) -> c_int;
    pub fn perf_data__kallsyms_name(data: *mut perf_data) -> *mut c_char;
    pub fn perf_data__guest_kallsyms_name(data: *mut perf_data, machine_pid: pid_t) -> *mut c_char;

    pub fn has_kcore_dir(path: *const c_char) -> bool;
    pub fn is_perf_data(path: *const c_char) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
