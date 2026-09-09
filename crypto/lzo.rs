// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cryptographic API.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// crypto/internal/scompress.h, linux/init.h, linux/lzo.h, linux/module.h,
// and linux/slab.h.

unsafe fn lzo_alloc_ctx() -> *mut core::ffi::c_void {
    let ctx: *mut core::ffi::c_void = kvmalloc(LZO1X_MEM_COMPRESS, GFP_KERNEL);
    if ctx.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    ctx
}

unsafe fn lzo_free_ctx(ctx: *mut core::ffi::c_void) {
    kvfree(ctx);
}

unsafe fn __lzo_compress(
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    ctx: *mut core::ffi::c_void,
) -> i32 {
    // size_t(ulong) <-> uint on 64 bit
    let mut tmp_len: usize = *dlen as usize;
    let err: i32 = lzo1x_1_compress_safe(src, slen, dst, &mut tmp_len, ctx);

    if err != LZO_E_OK {
        return -EINVAL;
    }

    *dlen = tmp_len as u32;
    0
}

unsafe fn lzo_scompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    ctx: *mut core::ffi::c_void,
) -> i32 {
    __lzo_compress(src, slen, dst, dlen, ctx)
}

unsafe fn __lzo_decompress(
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
) -> i32 {
    let mut tmp_len: usize = *dlen as usize;
    let err: i32 = lzo1x_decompress_safe(src, slen, dst, &mut tmp_len);

    if err != LZO_E_OK {
        return -EINVAL;
    }

    *dlen = tmp_len as u32;
    0
}

unsafe fn lzo_sdecompress(
    _tfm: *mut crypto_scomp,
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    __lzo_decompress(src, slen, dst, dlen)
}

static mut scomp: scomp_alg = scomp_alg {
    streams: scomp_streams {
        alloc_ctx: Some(lzo_alloc_ctx),
        free_ctx: Some(lzo_free_ctx),
    },
    compress: Some(lzo_scompress),
    decompress: Some(lzo_sdecompress),
    base: crypto_alg {
        cra_name: b"lzo\0".as_ptr() as *const i8,
        cra_driver_name: b"lzo-scomp\0".as_ptr() as *const i8,
        cra_module: THIS_MODULE,
    },
};

unsafe fn lzo_mod_init() -> i32 {
    crypto_register_scomp(&raw mut scomp)
}

unsafe fn lzo_mod_fini() {
    crypto_unregister_scomp(&raw mut scomp);
}

// module_init(lzo_mod_init);
// module_exit(lzo_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("LZO Compression Algorithm");
// MODULE_ALIAS_CRYPTO("lzo");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
