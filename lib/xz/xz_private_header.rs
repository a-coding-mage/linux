/* SPDX-License-Identifier: 0BSD */

/*
 * Private includes and definitions
 *
 * Author: Lasse Collin <lasse.collin@tukaani.org>
 */

/* C header guards and includes are intentionally omitted. The required
 * kernel/userspace dependencies are supplied by the surrounding translation.
 * Kernel configuration macros and XZ_PREBOOT retain their build-time intent. */

/* If no specific decoding mode is requested, enable support for all modes.
 * In Rust, the equivalent feature selection is represented by these defaults
 * when no corresponding build configuration is supplied. */

/* The DEC_IS_foo macros are compile-time mode predicates in the C source. */
#[inline]
pub const fn DEC_IS_SINGLE(mode: enum_xz_mode) -> bool {
    mode == XZ_SINGLE
}

#[inline]
pub const fn DEC_IS_PREALLOC(mode: enum_xz_mode) -> bool {
    mode == XZ_PREALLOC
}

#[inline]
pub const fn DEC_IS_DYNALLOC(mode: enum_xz_mode) -> bool {
    mode == XZ_DYNALLOC
}

#[inline]
pub const fn DEC_IS_MULTI(mode: enum_xz_mode) -> bool {
    mode != XZ_SINGLE
}

/*
 * If any of the BCJ filter decoders are wanted, define XZ_DEC_BCJ.
 * XZ_DEC_BCJ is used to enable generic support for BCJ decoders.
 * The original selection is a build-time configuration concern.
 */

/* Opaque types supplied by the corresponding XZ headers. */
#[repr(C)]
pub struct xz_dec_lzma2 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xz_dec_bcj {
    _private: [u8; 0],
}

/* External enum and buffer definitions are supplied by the XZ dependency. */
pub type enum_xz_mode = u32;
pub type enum_xz_ret = u32;
#[repr(C)]
pub struct xz_buf {
    _private: [u8; 0],
}

extern "C" {
    /*
     * Allocate memory for LZMA2 decoder. xz_dec_lzma2_reset() must be used
     * before calling xz_dec_lzma2_run().
     */
    pub fn xz_dec_lzma2_create(mode: enum_xz_mode, dict_max: u32) -> *mut xz_dec_lzma2;

    /*
     * Decode the LZMA2 properties (one byte) and reset the decoder. Return
     * XZ_OK on success, XZ_MEMLIMIT_ERROR if the preallocated dictionary is
     * not big enough, and XZ_OPTIONS_ERROR if props indicates something that
     * this decoder doesn't support.
     */
    pub fn xz_dec_lzma2_reset(s: *mut xz_dec_lzma2, props: u8) -> enum_xz_ret;

    /* Decode raw LZMA2 stream from b->in to b->out. */
    pub fn xz_dec_lzma2_run(s: *mut xz_dec_lzma2, b: *mut xz_buf) -> enum_xz_ret;

    /* Free the memory allocated for the LZMA2 decoder. */
    pub fn xz_dec_lzma2_end(s: *mut xz_dec_lzma2);

    /* Allocate memory for BCJ decoders. */
    pub fn xz_dec_bcj_create(single_call: bool) -> *mut xz_dec_bcj;

    /* Decode the Filter ID of a BCJ filter. */
    pub fn xz_dec_bcj_reset(s: *mut xz_dec_bcj, id: u8) -> enum_xz_ret;

    /* Decode raw BCJ + LZMA2 stream. */
    pub fn xz_dec_bcj_run(
        s: *mut xz_dec_bcj,
        lzma2: *mut xz_dec_lzma2,
        b: *mut xz_buf,
    ) -> enum_xz_ret;
}

/* C macro: #define xz_dec_bcj_end(s) kfree(s) */
#[inline]
pub unsafe fn xz_dec_bcj_end(s: *mut xz_dec_bcj) {
    kfree(s as *mut core::ffi::c_void);
}

/* Required external allocator symbol from the kernel/userspace environment. */
extern "C" {
    fn kfree(ptr: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
