// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cryptographic API.
 *
 * Copyright (c) 2013 Chanho Min <chanho.min@lge.com>
 */

use core::ffi::c_void;

// Translated dependencies supplied by the surrounding kernel sources.
type u8 = core::ffi::c_uchar;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;

const LZ4HC_MEM_COMPRESS: usize = 0; // supplied by linux/lz4.h
const LZ4HC_DEFAULT_CLEVEL: c_int = 0; // supplied by linux/lz4.h
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct crypto_scomp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scomp_alg_streams {
    pub alloc_ctx: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub free_ctx: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_module: *mut c_void,
}

#[repr(C)]
pub struct scomp_alg {
    pub streams: scomp_alg_streams,
    pub compress: Option<unsafe extern "C" fn(
        *mut crypto_scomp, *const u8, c_uint, *mut u8, *mut c_uint, *mut c_void,
    ) -> c_int>,
    pub decompress: Option<unsafe extern "C" fn(
        *mut crypto_scomp, *const u8, c_uint, *mut u8, *mut c_uint, *mut c_void,
    ) -> c_int>,
    pub base: crypto_alg,
}

extern "C" {
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn ERR_PTR(error: isize) -> *mut c_void;
    fn LZ4_compress_HC(
        src: *const u8,
        dst: *mut u8,
        input_size: c_int,
        max_output_size: c_int,
        compression_level: c_int,
        working_memory: *mut c_void,
    ) -> c_int;
    fn LZ4_decompress_safe(
        src: *const u8,
        dst: *mut u8,
        compressed_size: c_int,
        dst_capacity: c_int,
    ) -> c_int;
    fn crypto_register_scomp(alg: *mut scomp_alg) -> c_int;
    fn crypto_unregister_scomp(alg: *mut scomp_alg);
    static mut THIS_MODULE: *mut c_void;
}

unsafe extern "C" fn lz4hc_alloc_ctx() -> *mut c_void {
    let ctx: *mut c_void = vmalloc(LZ4HC_MEM_COMPRESS);
    if ctx.is_null() {
        return ERR_PTR(-(ENOMEM as isize));
    }
    ctx
}

unsafe extern "C" fn lz4hc_free_ctx(ctx: *mut c_void) {
    vfree(ctx);
}

unsafe fn __lz4hc_compress_crypto(
    src: *const u8,
    slen: c_uint,
    dst: *mut u8,
    dlen: *mut c_uint,
    ctx: *mut c_void,
) -> c_int {
    let out_len = LZ4_compress_HC(
        src,
        dst,
        slen as c_int,
        *dlen as c_int,
        LZ4HC_DEFAULT_CLEVEL,
        ctx,
    );

    if out_len == 0 {
        return -EINVAL;
    }

    *dlen = out_len as c_uint;
    0
}

unsafe extern "C" fn lz4hc_scompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: c_uint,
    dst: *mut u8,
    dlen: *mut c_uint,
    ctx: *mut c_void,
) -> c_int {
    __lz4hc_compress_crypto(src, slen, dst, dlen, ctx)
}

unsafe fn __lz4hc_decompress_crypto(
    src: *const u8,
    slen: c_uint,
    dst: *mut u8,
    dlen: *mut c_uint,
    _ctx: *mut c_void,
) -> c_int {
    let out_len = LZ4_decompress_safe(src, dst, slen as c_int, *dlen as c_int);

    if out_len < 0 {
        return -EINVAL;
    }

    *dlen = out_len as c_uint;
    0
}

unsafe extern "C" fn lz4hc_sdecompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: c_uint,
    dst: *mut u8,
    dlen: *mut c_uint,
    _ctx: *mut c_void,
) -> c_int {
    __lz4hc_decompress_crypto(src, slen, dst, dlen, core::ptr::null_mut())
}

static mut scomp: scomp_alg = scomp_alg {
    streams: scomp_alg_streams {
        alloc_ctx: Some(lz4hc_alloc_ctx),
        free_ctx: Some(lz4hc_free_ctx),
    },
    compress: Some(lz4hc_scompress),
    decompress: Some(lz4hc_sdecompress),
    base: crypto_alg {
        cra_name: b"lz4hc\0".as_ptr(),
        cra_driver_name: b"lz4hc-scomp\0".as_ptr(),
        cra_module: core::ptr::null_mut(), // THIS_MODULE
    },
};

unsafe extern "C" fn lz4hc_mod_init() -> c_int {
    crypto_register_scomp(&mut scomp)
}

unsafe extern "C" fn lz4hc_mod_fini() {
    crypto_unregister_scomp(&mut scomp);
}

// module_init(lz4hc_mod_init);
// module_exit(lz4hc_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("LZ4HC Compression Algorithm");
// MODULE_ALIAS_CRYPTO("lz4hc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
