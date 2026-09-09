// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cryptographic API.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    fn kvmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kvfree(ptr: *mut c_void);
    fn lzorle1x_1_compress_safe(
        src: *const u8,
        slen: u32,
        dst: *mut u8,
        dlen: *mut usize,
        wrkmem: *mut c_void,
    ) -> i32;
    fn lzo1x_decompress_safe(
        src: *const u8,
        slen: u32,
        dst: *mut u8,
        dlen: *mut usize,
    ) -> i32;
    fn crypto_register_scomp(alg: *mut scomp_alg) -> i32;
    fn crypto_unregister_scomp(alg: *mut scomp_alg);
}

extern "C" {
    static THIS_MODULE: *mut c_void;
}

const LZO1X_MEM_COMPRESS: usize = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: isize = 12;
const EINVAL: i32 = 22;
const LZO_E_OK: i32 = 0;

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
pub struct crypto_alg_base {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_module: *mut c_void,
}

#[repr(C)]
pub struct scomp_alg {
    pub streams: scomp_alg_streams,
    pub compress: Option<unsafe extern "C" fn(
        *mut crypto_scomp,
        *const u8,
        u32,
        *mut u8,
        *mut u32,
        *mut c_void,
    ) -> i32>,
    pub decompress: Option<unsafe extern "C" fn(
        *mut crypto_scomp,
        *const u8,
        u32,
        *mut u8,
        *mut u32,
        *mut c_void,
    ) -> i32>,
    pub base: crypto_alg_base,
}

unsafe extern "C" fn lzorle_alloc_ctx() -> *mut c_void {
    let ctx = kvmalloc(LZO1X_MEM_COMPRESS, GFP_KERNEL);
    if ctx.is_null() {
        return (-ENOMEM) as *mut c_void;
    }
    ctx
}

unsafe extern "C" fn lzorle_free_ctx(ctx: *mut c_void) {
    kvfree(ctx);
}

unsafe extern "C" fn __lzorle_compress(
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    ctx: *mut c_void,
) -> i32 {
    // size_t(ulong) <-> uint on 64 bit
    let mut tmp_len = *dlen as usize;
    let err = lzorle1x_1_compress_safe(src, slen, dst, &mut tmp_len, ctx);

    if err != LZO_E_OK {
        return -EINVAL;
    }

    *dlen = tmp_len as u32;
    0
}

unsafe extern "C" fn lzorle_scompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    ctx: *mut c_void,
) -> i32 {
    __lzorle_compress(src, slen, dst, dlen, ctx)
}

unsafe extern "C" fn __lzorle_decompress(
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
) -> i32 {
    // size_t(ulong) <-> uint on 64 bit
    let mut tmp_len = *dlen as usize;
    let err = lzo1x_decompress_safe(src, slen, dst, &mut tmp_len);

    if err != LZO_E_OK {
        return -EINVAL;
    }

    *dlen = tmp_len as u32;
    0
}

unsafe extern "C" fn lzorle_sdecompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    _ctx: *mut c_void,
) -> i32 {
    __lzorle_decompress(src, slen, dst, dlen)
}

static mut SCOMP: scomp_alg = scomp_alg {
    streams: scomp_alg_streams {
        alloc_ctx: Some(lzorle_alloc_ctx),
        free_ctx: Some(lzorle_free_ctx),
    },
    compress: Some(lzorle_scompress),
    decompress: Some(lzorle_sdecompress),
    base: crypto_alg_base {
        cra_name: b"lzo-rle\0".as_ptr(),
        cra_driver_name: b"lzo-rle-scomp\0".as_ptr(),
        cra_module: core::ptr::null_mut(), // THIS_MODULE
    },
};

unsafe extern "C" fn lzorle_mod_init() -> i32 {
    crypto_register_scomp(&raw mut SCOMP)
}

unsafe extern "C" fn lzorle_mod_fini() {
    crypto_unregister_scomp(&raw mut SCOMP);
}

// module_init(lzorle_mod_init);
// module_exit(lzorle_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("LZO-RLE Compression Algorithm");
// MODULE_ALIAS_CRYPTO("lzo-rle");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
