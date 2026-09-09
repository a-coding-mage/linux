// SPDX-License-Identifier: GPL-2.0
/*
 * decompress.c
 *
 * Detect the decompression method based on magic number
 */

// Declarations supplied by the Linux decompression headers are external
// dependencies of this translation.

pub type decompress_fn = Option<unsafe extern "C" fn()>;

extern "C" {
    pub fn gunzip(
    );
    pub fn bunzip2(
    );
    pub fn unlzma(
    );
    pub fn unxz(
    );
    pub fn unlzo(
    );
    pub fn unlz4(
    );
    pub fn unzstd(
    );
    pub fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
}

#[repr(C)]
pub struct compress_format {
    pub magic: [u8; 2],
    pub name: *const core::ffi::c_char,
    pub decompressor: decompress_fn,
}

static COMPRESSED_FORMATS: [compress_format; 9] = [
    compress_format { magic: [0x1f, 0x8b], name: b"gzip\0".as_ptr() as *const _, decompressor: Some(gunzip) },
    compress_format { magic: [0x1f, 0x9e], name: b"gzip\0".as_ptr() as *const _, decompressor: Some(gunzip) },
    compress_format { magic: [0x42, 0x5a], name: b"bzip2\0".as_ptr() as *const _, decompressor: Some(bunzip2) },
    compress_format { magic: [0x5d, 0x00], name: b"lzma\0".as_ptr() as *const _, decompressor: Some(unlzma) },
    compress_format { magic: [0xfd, 0x37], name: b"xz\0".as_ptr() as *const _, decompressor: Some(unxz) },
    compress_format { magic: [0x89, 0x4c], name: b"lzo\0".as_ptr() as *const _, decompressor: Some(unlzo) },
    compress_format { magic: [0x02, 0x21], name: b"lz4\0".as_ptr() as *const _, decompressor: Some(unlz4) },
    compress_format { magic: [0x28, 0xb5], name: b"zstd\0".as_ptr() as *const _, decompressor: Some(unzstd) },
    compress_format { magic: [0, 0], name: core::ptr::null(), decompressor: None },
];

pub unsafe fn decompress_method(
    inbuf: *const u8,
    len: i64,
    name: *mut *const core::ffi::c_char,
) -> decompress_fn {
    if len < 2 {
        if !name.is_null() {
            *name = core::ptr::null();
        }
        return None; // Need at least this much...
    }

    // pr_debug("Compressed data magic: %#.2x %#.2x\n", inbuf[0], inbuf[1]);

    let mut cf = &COMPRESSED_FORMATS[0];
    while !cf.name.is_null() {
        if memcmp(
            inbuf as *const core::ffi::c_void,
            cf.magic.as_ptr() as *const core::ffi::c_void,
            2,
        ) == 0 {
            break;
        }
        cf = &COMPRESSED_FORMATS[(cf as *const compress_format).offset_from(COMPRESSED_FORMATS.as_ptr()) as usize + 1];
    }

    if !name.is_null() {
        *name = cf.name;
    }
    cf.decompressor
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
