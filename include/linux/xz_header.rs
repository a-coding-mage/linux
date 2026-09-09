/* SPDX-License-Identifier: 0BSD */

/* XZ decompressor public header translated from C. */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xz_mode {
    XZ_SINGLE,
    XZ_PREALLOC,
    XZ_DYNALLOC,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xz_ret {
    XZ_OK,
    XZ_STREAM_END,
    XZ_UNSUPPORTED_CHECK,
    XZ_MEM_ERROR,
    XZ_MEMLIMIT_ERROR,
    XZ_FORMAT_ERROR,
    XZ_OPTIONS_ERROR,
    XZ_DATA_ERROR,
    XZ_BUF_ERROR,
}

#[repr(C)]
pub struct xz_buf {
    pub r#in: *const u8,
    pub in_pos: usize,
    pub in_size: usize,

    pub out: *mut u8,
    pub out_pos: usize,
    pub out_size: usize,
}

/* Opaque type to hold the XZ decoder state. */
#[repr(C)]
pub struct xz_dec {
    _private: [u8; 0],
}

/* Opaque type to hold the MicroLZMA decoder state. */
#[repr(C)]
pub struct xz_dec_microlzma {
    _private: [u8; 0],
}

extern "C" {
    pub fn xz_dec_init(mode: xz_mode, dict_max: u32) -> *mut xz_dec;

    pub fn xz_dec_run(s: *mut xz_dec, b: *mut xz_buf) -> xz_ret;

    pub fn xz_dec_reset(s: *mut xz_dec);

    pub fn xz_dec_end(s: *mut xz_dec);

    pub fn xz_dec_microlzma_alloc(
        mode: xz_mode,
        dict_size: u32,
    ) -> *mut xz_dec_microlzma;

    pub fn xz_dec_microlzma_reset(
        s: *mut xz_dec_microlzma,
        comp_size: u32,
        uncomp_size: u32,
        uncomp_size_is_exact: ::core::ffi::c_int,
    );

    pub fn xz_dec_microlzma_run(s: *mut xz_dec_microlzma, b: *mut xz_buf) -> xz_ret;

    pub fn xz_dec_microlzma_end(s: *mut xz_dec_microlzma);
}

/*
 * Standalone builds need a CRC32 implementation. For normal in-kernel use,
 * the kernel's own CRC32 module is used instead.
 *
 * The C header conditionally exposes these declarations according to
 * XZ_INTERNAL_CRC32. Define the feature when the standalone implementation
 * is enabled; otherwise these declarations are omitted.
 */
#[cfg(feature = "xz_internal_crc32")]
extern "C" {
    pub fn xz_crc32_init();

    pub fn xz_crc32(buf: *const u8, size: usize, crc: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
