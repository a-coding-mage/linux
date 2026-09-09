/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2019 HUAWEI, Inc.
 *             https://www.huawei.com/
 */

// Dependency declarations from internal.h are supplied by the surrounding crate.

#[repr(C)]
pub struct z_erofs_decompress_req {
    pub sb: *mut super_block,
    pub in_: *mut *mut page,
    pub out: *mut *mut page,
    pub inpages: ::core::ffi::c_uint,
    pub outpages: ::core::ffi::c_uint,
    pub pageofs_in: ::core::ffi::c_ushort,
    pub pageofs_out: ::core::ffi::c_ushort,
    pub inputsize: ::core::ffi::c_uint,
    pub outputsize: ::core::ffi::c_uint,
    pub alg: ::core::ffi::c_uint, // the algorithm for decompression
    pub inplace_io: bool,
    pub partial_decoding: bool,
    pub fillgaps: bool,
    pub gfp: gfp_t, // allocation flags for extra temporary buffers
}

#[repr(C)]
pub struct z_erofs_decompressor {
    pub config: Option<unsafe extern "C" fn(
        *mut super_block,
        *mut erofs_super_block,
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub decompress: Option<unsafe extern "C" fn(
        *mut z_erofs_decompress_req,
        *mut *mut page,
    ) -> *const ::core::ffi::c_char>,
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub name: *mut ::core::ffi::c_char,
}

pub const Z_EROFS_SHORTLIVED_PAGE: usize = (!0usize) << 2;
pub const Z_EROFS_PREALLOCATED_FOLIO: *mut ::core::ffi::c_void = ((-2isize as usize) << 2) as *mut ::core::ffi::c_void;

/*
 * Currently, short-lived pages are pages directly from buddy system
 * with specific page->private (Z_EROFS_SHORTLIVED_PAGE).
 * In the future world of Memdescs, it should be type 0 (Misc) memory
 * which type can be checked with a new helper.
 */
#[inline]
pub unsafe fn z_erofs_is_shortlived_page(page: *mut page) -> bool {
    (*page).private == Z_EROFS_SHORTLIVED_PAGE
}

#[inline]
pub unsafe fn z_erofs_put_shortlivedpage(pagepool: *mut *mut page, page: *mut page) -> bool {
    if !z_erofs_is_shortlived_page(page) {
        return false;
    }
    erofs_pagepool_add(pagepool, page);
    true
}

extern "C" {
    pub static z_erofs_lzma_decomp: z_erofs_decompressor;
    pub static z_erofs_deflate_decomp: z_erofs_decompressor;
    pub static z_erofs_zstd_decomp: z_erofs_decompressor;
    pub static mut z_erofs_decomp: *const *const z_erofs_decompressor;
    pub fn erofs_pagepool_add(pagepool: *mut *mut page, page: *mut page);

    pub fn z_erofs_stream_switch_bufs(
        dctx: *mut z_erofs_stream_dctx,
        dst: *mut *mut ::core::ffi::c_void,
        src: *mut *mut ::core::ffi::c_void,
        pgpl: *mut *mut page,
    ) -> *const ::core::ffi::c_char;
    pub fn z_erofs_fixup_insize(
        rq: *mut z_erofs_decompress_req,
        padbuf: *const ::core::ffi::c_char,
        padbufsize: ::core::ffi::c_uint,
    ) -> *const ::core::ffi::c_char;
    pub fn z_erofs_init_decompressor() -> ::core::ffi::c_int;
    pub fn z_erofs_exit_decompressor();
    pub fn z_erofs_crypto_decompress(
        rq: *mut z_erofs_decompress_req,
        pgpl: *mut *mut page,
    ) -> ::core::ffi::c_int;
    pub fn z_erofs_crypto_enable_engine(
        name: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn z_erofs_crypto_disable_all_engines();
    pub fn z_erofs_crypto_show_engines(
        buf: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        sep: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct z_erofs_stream_dctx {
    pub rq: *mut z_erofs_decompress_req,
    pub no: ::core::ffi::c_int,
    pub ni: ::core::ffi::c_int, // the current {en,de}coded page #
    pub avail_out: ::core::ffi::c_uint, // remaining bytes in the decoded buffer
    pub inbuf_pos: ::core::ffi::c_uint,
    pub inbuf_sz: ::core::ffi::c_uint, // current status of the encoded buffer
    pub kin: *mut u8,
    pub kout: *mut u8, // buffer mapped pointers
    pub bounce: *mut ::core::ffi::c_void, // bounce buffer for inplace I/Os
    pub bounced: bool, // is the bounce buffer used now?
}

// CONFIG_EROFS_FS_ZIP_ACCEL is a build-time condition; these declarations
// correspond to the enabled branch. The disabled branch supplies no-op
// inline equivalents in configurations without the feature.

#[cfg(not(CONFIG_EROFS_FS_ZIP_ACCEL))]
#[inline]
pub unsafe fn z_erofs_crypto_disable_all_engines() {}

#[cfg(not(CONFIG_EROFS_FS_ZIP_ACCEL))]
#[inline]
pub unsafe fn z_erofs_crypto_show_engines(
    _buf: *mut ::core::ffi::c_char,
    _size: ::core::ffi::c_int,
    _sep: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
