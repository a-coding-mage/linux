// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cryptographic API.
 *
 * Copyright (c) 2013 Chanho Min <chanho.min@lge.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.

use core::ffi::c_void;

type u8 = core::ffi::c_uchar;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;

extern "C" {
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(addr: *mut c_void);
    fn LZ4_compress_default(
        src: *const u8,
        dst: *mut u8,
        src_size: c_int,
        dst_capacity: c_int,
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
}

const LZ4_MEM_COMPRESS: usize = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct crypto_scomp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scomp_alg {
    pub streams: scomp_streams,
    pub compress: unsafe extern "C" fn(
        *mut crypto_scomp,
        *const u8,
        c_uint,
        *mut u8,
        *mut c_uint,
        *mut c_void,
    ) -> c_int,
    pub decompress: unsafe extern "C" fn(
        *mut crypto_scomp,
        *const u8,
        c_uint,
        *mut u8,
        *mut c_uint,
        *mut c_void,
    ) -> c_int,
    pub base: crypto_alg,
}

#[repr(C)]
pub struct scomp_streams {
    pub alloc_ctx: unsafe extern "C" fn() -> *mut c_void,
    pub free_ctx: unsafe extern "C" fn(*mut c_void),
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_module: *mut c_void,
}

unsafe fn lz4_alloc_ctx() -> *mut c_void {
    let ctx: *mut c_void;

    ctx = vmalloc(LZ4_MEM_COMPRESS);
    if ctx.is_null() {
        return (-ENOMEM) as isize as *mut c_void;
    }

    ctx
}

unsafe fn lz4_free_ctx(ctx: *mut c_void) {
    vfree(ctx);
}

unsafe fn __lz4_compress_crypto(
    src: *const u8,
    slen: c_uint,
    dst: *mut u8,
    dlen: *mut c_uint,
    ctx: *mut c_void,
) -> c_int {
    let out_len = LZ4_compress_default(src, dst, slen as c_int, *dlen as c_int, ctx);

    if out_len == 0 {
        return -EINVAL;
    }

    *dlen = out_len as c_uint;
    0
}

unsafe fn lz4_scompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: c_uint,
    dst: *mut u8,
    dlen: *mut c_uint,
    ctx: *mut c_void,
) -> c_int {
    __lz4_compress_crypto(src, slen, dst, dlen, ctx)
}

unsafe fn __lz4_decompress_crypto(
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

unsafe fn lz4_sdecompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: c_uint,
    dst: *mut u8,
    dlen: *mut c_uint,
    _ctx: *mut c_void,
) -> c_int {
    __lz4_decompress_crypto(src, slen, dst, dlen, core::ptr::null_mut())
}

static mut SCOMP: scomp_alg = scomp_alg {
    streams: scomp_streams {
        alloc_ctx: lz4_alloc_ctx,
        free_ctx: lz4_free_ctx,
    },
    compress: lz4_scompress,
    decompress: lz4_sdecompress,
    base: crypto_alg {
        cra_name: b"lz4\0".as_ptr(),
        cra_driver_name: b"lz4-scomp\0".as_ptr(),
        cra_module: core::ptr::null_mut(),
    },
};

unsafe fn lz4_mod_init() -> c_int {
    crypto_register_scomp(&mut SCOMP)
}

unsafe fn lz4_mod_fini() {
    crypto_unregister_scomp(&mut SCOMP);
}

// module_init(lz4_mod_init);
// module_exit(lz4_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("LZ4 Compression Algorithm");
// MODULE_ALIAS_CRYPTO("lz4");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
