/* SPDX-License-Identifier: GPL-2.0 */

use std::ffi::{c_char, c_int, c_void};

pub type ssize_t = isize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/* Conditional declarations translated from HAVE_ZSTD_SUPPORT dependency on <zstd.h>. */
#[cfg(HAVE_ZSTD_SUPPORT)]
#[repr(C)]
pub struct ZSTD_CStream {
    _private: [u8; 0],
}

#[cfg(HAVE_ZSTD_SUPPORT)]
#[repr(C)]
pub struct ZSTD_DStream {
    _private: [u8; 0],
}

#[cfg(HAVE_ZLIB_SUPPORT)]
extern "C" {
    pub fn gzip_decompress_to_file(input: *const c_char, output_fd: c_int) -> c_int;
    pub fn gzip_is_compressed(input: *const c_char) -> bool;
}

#[cfg(HAVE_LZMA_SUPPORT)]
extern "C" {
    pub fn lzma_decompress_stream_to_file(input: *mut FILE, output_fd: c_int) -> c_int;
    pub fn lzma_decompress_to_file(input: *const c_char, output_fd: c_int) -> c_int;
    pub fn lzma_is_compressed(input: *const c_char) -> bool;
}

#[cfg(not(HAVE_LZMA_SUPPORT))]
pub unsafe fn lzma_decompress_stream_to_file(
    _input: *mut FILE,
    _output_fd: c_int,
) -> c_int {
    -1
}

#[cfg(not(HAVE_LZMA_SUPPORT))]
pub unsafe fn lzma_decompress_to_file(
    _input: *const c_char,
    _output_fd: c_int,
) -> c_int {
    -1
}

#[cfg(not(HAVE_LZMA_SUPPORT))]
pub unsafe fn lzma_is_compressed(_input: *const c_char) -> bool {
    false
}

#[repr(C)]
pub struct zstd_data {
    #[cfg(HAVE_ZSTD_SUPPORT)]
    pub cstream: *mut ZSTD_CStream,
    #[cfg(HAVE_ZSTD_SUPPORT)]
    pub dstream: *mut ZSTD_DStream,
    #[cfg(HAVE_ZSTD_SUPPORT)]
    pub comp_level: c_int,
}

#[cfg(HAVE_ZSTD_SUPPORT)]
extern "C" {
    pub fn zstd_init(data: *mut zstd_data, level: c_int) -> c_int;
    pub fn zstd_fini(data: *mut zstd_data) -> c_int;

    pub fn zstd_compress_stream_to_records(
        data: *mut zstd_data,
        dst: *mut c_void,
        dst_size: usize,
        src: *mut c_void,
        src_size: usize,
        max_record_size: usize,
        process_header: Option<
            unsafe extern "C" fn(
                record: *mut c_void,
                dst_size: usize,
                data_size: usize,
            ) -> ssize_t,
        >,
    ) -> ssize_t;

    pub fn zstd_decompress_stream(
        data: *mut zstd_data,
        src: *mut c_void,
        src_size: usize,
        dst: *mut c_void,
        dst_size: usize,
    ) -> usize;
}

/* !HAVE_ZSTD_SUPPORT */
#[cfg(not(HAVE_ZSTD_SUPPORT))]
pub unsafe fn zstd_init(_data: *mut zstd_data, _level: c_int) -> c_int {
    0
}

#[cfg(not(HAVE_ZSTD_SUPPORT))]
pub unsafe fn zstd_fini(_data: *mut zstd_data) -> c_int {
    0
}

#[cfg(not(HAVE_ZSTD_SUPPORT))]
pub unsafe fn zstd_compress_stream_to_records(
    _data: *mut zstd_data,
    _dst: *mut c_void,
    _dst_size: usize,
    _src: *mut c_void,
    _src_size: usize,
    _max_record_size: usize,
    _process_header: Option<
        unsafe extern "C" fn(
            record: *mut c_void,
            dst_size: usize,
            data_size: usize,
        ) -> ssize_t,
    >,
) -> ssize_t {
    0
}

#[cfg(not(HAVE_ZSTD_SUPPORT))]
pub unsafe fn zstd_decompress_stream(
    _data: *mut zstd_data,
    _src: *mut c_void,
    _src_size: usize,
    _dst: *mut c_void,
    _dst_size: usize,
) -> usize {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
