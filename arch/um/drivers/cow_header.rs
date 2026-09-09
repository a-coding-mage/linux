/* SPDX-License-Identifier: GPL-2.0 */

// <asm/types.h> equivalents used by this header:
// __u32 -> u32, __u64 -> u64, int -> i32,
// unsigned long -> core::ffi::c_ulong, unsigned long long -> u64.

unsafe extern "C" {
    pub fn init_cow_file(
        fd: i32,
        cow_file: *mut core::ffi::c_char,
        backing_file: *mut core::ffi::c_char,
        sectorsize: i32,
        alignment: i32,
        bitmap_offset_out: *mut i32,
        bitmap_len_out: *mut core::ffi::c_ulong,
        data_offset_out: *mut i32,
    ) -> i32;

    pub fn file_reader(
        offset: u64,
        buf: *mut core::ffi::c_char,
        len: i32,
        arg: *mut core::ffi::c_void,
    ) -> i32;

    pub fn read_cow_header(
        reader: unsafe extern "C" fn(
            u64,
            *mut core::ffi::c_char,
            i32,
            *mut core::ffi::c_void,
        ) -> i32,
        arg: *mut core::ffi::c_void,
        version_out: *mut u32,
        backing_file_out: *mut *mut core::ffi::c_char,
        mtime_out: *mut i64,
        size_out: *mut u64,
        sectorsize_out: *mut i32,
        align_out: *mut u32,
        bitmap_offset_out: *mut i32,
    ) -> i32;

    pub fn write_cow_header(
        cow_file: *mut core::ffi::c_char,
        fd: i32,
        backing_file: *mut core::ffi::c_char,
        sectorsize: i32,
        alignment: i32,
        size: *mut u64,
    ) -> i32;

    pub fn cow_sizes(
        version: i32,
        size: u64,
        sectorsize: i32,
        align: i32,
        bitmap_offset: i32,
        bitmap_len_out: *mut core::ffi::c_ulong,
        data_offset_out: *mut i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
