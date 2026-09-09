// SPDX-License-Identifier: GPL-2.0-or-later

// C preprocessor format prefix: "lzo: "
// The declarations and constants supplied by linux/kernel.h, linux/slab.h,
// linux/lzo.h, and backend_lzo.h are external dependencies of this file.

unsafe fn lzo_release_params(params: *mut zcomp_params) {
    let _ = params;
}

unsafe fn lzo_setup_params(params: *mut zcomp_params) -> i32 {
    if (*params).dict_sz != 0 {
        pr_err!("dictionary is not supported\n");
        return -EOPNOTSUPP;
    }
    if (*params).level != ZCOMP_PARAM_NOT_SET {
        pr_err!("compression level is not supported\n");
        return -EOPNOTSUPP;
    }
    0
}

unsafe fn lzo_create(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
) -> i32 {
    let _ = params;
    (*ctx).context = kzalloc(LZO1X_MEM_COMPRESS, GFP_KERNEL);
    if (*ctx).context.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn lzo_destroy(ctx: *mut zcomp_ctx) {
    kfree((*ctx).context);
}

unsafe fn lzo_compress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let _ = params;
    let ret = lzo1x_1_compress(
        (*req).src,
        (*req).src_len,
        (*req).dst,
        &mut (*req).dst_len,
        (*ctx).context,
    );
    if ret == LZO_E_OK { 0 } else { ret }
}

unsafe fn lzo_decompress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let _ = params;
    let _ = ctx;
    let ret = lzo1x_decompress_safe(
        (*req).src,
        (*req).src_len,
        (*req).dst,
        &mut (*req).dst_len,
    );
    if ret == LZO_E_OK { 0 } else { ret }
}

const backend_lzo: zcomp_ops = zcomp_ops {
    compress: Some(lzo_compress),
    decompress: Some(lzo_decompress),
    create_ctx: Some(lzo_create),
    destroy_ctx: Some(lzo_destroy),
    setup_params: Some(lzo_setup_params),
    release_params: Some(lzo_release_params),
    name: b"lzo\0".as_ptr() as *const i8,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
