// SPDX-License-Identifier: GPL-2.0-or-later

// C dependency includes are intentionally omitted; the referenced kernel and
// LZ4 types, constants, functions, and zcomp interfaces are supplied elsewhere.

#[repr(C)]
struct lz4hc_ctx {
    mem: *mut core::ffi::c_void,
    dstrm: *mut LZ4_streamDecode_t,
    cstrm: *mut LZ4_streamHC_t,
}

unsafe fn lz4hc_release_params(_params: *mut zcomp_params) {
}

unsafe fn lz4hc_setup_params(params: *mut zcomp_params) -> i32 {
    if (*params).level == ZCOMP_PARAM_NOT_SET {
        (*params).level = LZ4HC_DEFAULT_CLEVEL;
    } else if (*params).level < 1 || (*params).level > LZ4HC_MAX_CLEVEL {
        /*
         * Use < 1 rather than < LZ4HC_MIN_CLEVEL here because
         * LZ4HC_compress_generic() only clamps levels below 1
         * (levels 1 and 2 are valid). LZ4HC_MIN_CLEVEL (3) is
         * advisory and not enforced by the library.
         */
        pr_err!("invalid compression level {}\n", (*params).level);
        return -EINVAL;
    }

    0
}

unsafe fn lz4hc_destroy(ctx: *mut zcomp_ctx) {
    let zctx = (*ctx).context as *mut lz4hc_ctx;

    if zctx.is_null() {
        return;
    }

    kfree((*zctx).dstrm as *mut core::ffi::c_void);
    kfree((*zctx).cstrm as *mut core::ffi::c_void);
    vfree((*zctx).mem);
    kfree(zctx as *mut core::ffi::c_void);
}

unsafe fn lz4hc_create(params: *mut zcomp_params, ctx: *mut zcomp_ctx) -> i32 {
    let zctx = kzalloc::<lz4hc_ctx>();

    if zctx.is_null() {
        return -ENOMEM;
    }

    (*ctx).context = zctx as *mut core::ffi::c_void;
    if (*params).dict_sz == 0 {
        (*zctx).mem = vmalloc(LZ4HC_MEM_COMPRESS);
        if (*zctx).mem.is_null() {
            lz4hc_destroy(ctx);
            return -EINVAL;
        }
    } else {
        (*zctx).dstrm = kzalloc::<LZ4_streamDecode_t>();
        if (*zctx).dstrm.is_null() {
            lz4hc_destroy(ctx);
            return -EINVAL;
        }

        (*zctx).cstrm = kzalloc::<LZ4_streamHC_t>();
        if (*zctx).cstrm.is_null() {
            lz4hc_destroy(ctx);
            return -EINVAL;
        }
    }

    0
}

unsafe fn lz4hc_compress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let zctx = (*ctx).context as *mut lz4hc_ctx;
    let mut ret: i32;

    if (*zctx).cstrm.is_null() {
        ret = LZ4_compress_HC(
            (*req).src,
            (*req).dst,
            (*req).src_len,
            (*req).dst_len,
            (*params).level,
            (*zctx).mem,
        );
    } else {
        /* Cstrm needs to be reset */
        LZ4_resetStreamHC((*zctx).cstrm, (*params).level);
        ret = LZ4_loadDictHC((*zctx).cstrm, (*params).dict, (*params).dict_sz);
        if ret != (*params).dict_sz {
            return -EINVAL;
        }
        ret = LZ4_compress_HC_continue(
            (*zctx).cstrm,
            (*req).src,
            (*req).dst,
            (*req).src_len,
            (*req).dst_len,
        );
    }
    if ret == 0 {
        return -EINVAL;
    }
    (*req).dst_len = ret;
    0
}

unsafe fn lz4hc_decompress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let zctx = (*ctx).context as *mut lz4hc_ctx;
    let mut ret: i32;

    if (*zctx).dstrm.is_null() {
        ret = LZ4_decompress_safe(
            (*req).src,
            (*req).dst,
            (*req).src_len,
            (*req).dst_len,
        );
    } else {
        /* Dstrm needs to be reset */
        ret = LZ4_setStreamDecode((*zctx).dstrm, (*params).dict, (*params).dict_sz);
        if ret == 0 {
            return -EINVAL;
        }
        ret = LZ4_decompress_safe_continue(
            (*zctx).dstrm,
            (*req).src,
            (*req).dst,
            (*req).src_len,
            (*req).dst_len,
        );
    }
    if ret < 0 {
        return -EINVAL;
    }
    0
}

pub static backend_lz4hc: zcomp_ops = zcomp_ops {
    compress: Some(lz4hc_compress),
    decompress: Some(lz4hc_decompress),
    create_ctx: Some(lz4hc_create),
    destroy_ctx: Some(lz4hc_destroy),
    setup_params: Some(lz4hc_setup_params),
    release_params: Some(lz4hc_release_params),
    name: b"lz4hc\0".as_ptr() as *const core::ffi::c_char,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
