/* SPDX-License-Identifier: GPL-2.0 */

// C header guard omitted from executable Rust.

pub const SW842_MEM_COMPRESS: u32 = 0xf000;

unsafe extern "C" {
    pub fn sw842_compress(
        src: *const u8,
        srclen: u32,
        dst: *mut u8,
        destlen: *mut u32,
        wmem: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn sw842_decompress(
        src: *const u8,
        srclen: u32,
        dst: *mut u8,
        destlen: *mut u32,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
