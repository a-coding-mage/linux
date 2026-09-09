// SPDX-License-Identifier: GPL-2.0-or-later

// #define pr_fmt(fmt) "lzo-rle: " fmt
// C dependencies: linux/kernel.h, linux/slab.h, linux/lzo.h, and
// backend_lzorle.h provide the referenced types, constants, macros, and APIs.

unsafe fn lzorle_release_params(_params: *mut zcomp_params) {
}

unsafe fn lzorle_setup_params(params: *mut zcomp_params) -> i32 {
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

unsafe fn lzorle_create(
    _params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
) -> i32 {
    (*ctx).context = kzalloc(LZO1X_MEM_COMPRESS, GFP_KERNEL);
    if (*ctx).context.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn lzorle_destroy(ctx: *mut zcomp_ctx) {
    kfree((*ctx).context);
}

unsafe fn lzorle_compress(
    _params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let ret = lzorle1x_1_compress(
        (*req).src,
        (*req).src_len,
        (*req).dst,
        &mut (*req).dst_len,
        (*ctx).context,
    );
    if ret == LZO_E_OK { 0 } else { ret }
}

unsafe fn lzorle_decompress(
    _params: *mut zcomp_params,
    _ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let ret = lzo1x_decompress_safe(
        (*req).src,
        (*req).src_len,
        (*req).dst,
        &mut (*req).dst_len,
    );
    if ret == LZO_E_OK { 0 } else { ret }
}

const backend_lzorle: zcomp_ops = zcomp_ops {
    compress: Some(lzorle_compress),
    decompress: Some(lzorle_decompress),
    create_ctx: Some(lzorle_create),
    destroy_ctx: Some(lzorle_destroy),
    setup_params: Some(lzorle_setup_params),
    release_params: Some(lzorle_release_params),
    name: "lzo-rle",
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
