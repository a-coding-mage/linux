// SPDX-License-Identifier: GPL-2.0-or-later

// #define pr_fmt(fmt) "lz4: " fmt
// C dependencies supplied by the surrounding kernel translation are referenced below.

#[repr(C)]
struct lz4_ctx {
    mem: *mut core::ffi::c_void,
    dstrm: *mut LZ4_streamDecode_t,
    cstrm: *mut LZ4_stream_t,
}

unsafe fn lz4_release_params(params: *mut zcomp_params) {
    let dict_stream = (*params).drv_data as *mut LZ4_stream_t;

    (*params).drv_data = core::ptr::null_mut();
    if dict_stream.is_null() {
        return;
    }

    kfree(dict_stream as *mut core::ffi::c_void);
}

unsafe fn lz4_setup_params(params: *mut zcomp_params) -> i32 {
    let dict_stream: *mut LZ4_stream_t;
    let ret: i32;

    if (*params).level == ZCOMP_PARAM_NOT_SET {
        (*params).level = LZ4_ACCELERATION_DEFAULT;
    } else if (*params).level < LZ4_ACCELERATION_DEFAULT {
        pr_err!("invalid compression level {}\n", (*params).level);
        return -EINVAL;
    }

    if (*params).dict.is_null() || (*params).dict_sz == 0 {
        return 0;
    }

    dict_stream = kzalloc::<LZ4_stream_t>(GFP_KERNEL);
    if dict_stream.is_null() {
        return -ENOMEM;
    }

    ret = LZ4_loadDict(dict_stream, (*params).dict, (*params).dict_sz);
    if ret != (*params).dict_sz {
        kfree(dict_stream as *mut core::ffi::c_void);
        return -EINVAL;
    }
    (*params).drv_data = dict_stream as *mut core::ffi::c_void;

    0
}

unsafe fn lz4_destroy(ctx: *mut zcomp_ctx) {
    let zctx = (*ctx).context as *mut lz4_ctx;

    if zctx.is_null() {
        return;
    }

    vfree((*zctx).mem);
    kfree((*zctx).dstrm as *mut core::ffi::c_void);
    kfree((*zctx).cstrm as *mut core::ffi::c_void);
    kfree(zctx as *mut core::ffi::c_void);
}

unsafe fn lz4_create(params: *mut zcomp_params, ctx: *mut zcomp_ctx) -> i32 {
    let zctx = kzalloc::<lz4_ctx>(GFP_KERNEL);

    if zctx.is_null() {
        return -ENOMEM;
    }

    (*ctx).context = zctx as *mut core::ffi::c_void;
    if (*params).dict_sz == 0 {
        (*zctx).mem = vmalloc(LZ4_MEM_COMPRESS);
        if (*zctx).mem.is_null() {
            lz4_destroy(ctx);
            return -ENOMEM;
        }
    } else {
        (*zctx).dstrm = kzalloc::<LZ4_streamDecode_t>(GFP_KERNEL);
        if (*zctx).dstrm.is_null() {
            lz4_destroy(ctx);
            return -ENOMEM;
        }

        (*zctx).cstrm = kzalloc::<LZ4_stream_t>(GFP_KERNEL);
        if (*zctx).cstrm.is_null() {
            lz4_destroy(ctx);
            return -ENOMEM;
        }
    }

    0
}

unsafe fn lz4_compress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let zctx = (*ctx).context as *mut lz4_ctx;
    let ret: i32;

    if (*zctx).cstrm.is_null() {
        ret = LZ4_compress_fast(
            (*req).src,
            (*req).dst,
            (*req).src_len,
            (*req).dst_len,
            (*params).level,
            (*zctx).mem,
        );
    } else {
        // Cstrm needs to be reset
        core::ptr::copy_nonoverlapping(
            (*params).drv_data as *const LZ4_stream_t,
            (*zctx).cstrm,
            1,
        );
        ret = LZ4_compress_fast_continue(
            (*zctx).cstrm,
            (*req).src,
            (*req).dst,
            (*req).src_len,
            (*req).dst_len,
            (*params).level,
        );
    }
    if ret == 0 {
        return -EINVAL;
    }
    (*req).dst_len = ret;
    0
}

unsafe fn lz4_decompress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let zctx = (*ctx).context as *mut lz4_ctx;
    let ret: i32;

    if (*zctx).dstrm.is_null() {
        ret = LZ4_decompress_safe(
            (*req).src,
            (*req).dst,
            (*req).src_len,
            (*req).dst_len,
        );
    } else {
        // Dstrm needs to be reset
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

#[no_mangle]
pub static mut backend_lz4: zcomp_ops = zcomp_ops {
    compress: Some(lz4_compress),
    decompress: Some(lz4_decompress),
    create_ctx: Some(lz4_create),
    destroy_ctx: Some(lz4_destroy),
    setup_params: Some(lz4_setup_params),
    release_params: Some(lz4_release_params),
    name: b"lz4\0".as_ptr() as *const i8,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
