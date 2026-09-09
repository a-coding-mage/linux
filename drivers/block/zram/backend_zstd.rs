// SPDX-License-Identifier: GPL-2.0-or-later

// pr_fmt(fmt) = "zstd: " fmt
// Linux kernel, slab, vmalloc, zstd, and backend_zstd.h dependencies are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct zstd_ctx {
    pub cctx: *mut zstd_cctx,
    pub dctx: *mut zstd_dctx,
    pub cctx_mem: *mut core::ffi::c_void,
    pub dctx_mem: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct zstd_params {
    pub custom_mem: zstd_custom_mem,
    pub cdict: *mut zstd_cdict,
    pub ddict: *mut zstd_ddict,
    pub cprm: zstd_parameters,
}

/*
 * For C/D dictionaries we need to provide zstd with zstd_custom_mem,
 * which zstd uses internally to allocate/free memory when needed.
 */
unsafe extern "C" fn zstd_custom_alloc(
    _opaque: *mut core::ffi::c_void,
    size: usize,
) -> *mut core::ffi::c_void {
    kvzalloc(size, GFP_NOIO | __GFP_NOWARN)
}

unsafe extern "C" fn zstd_custom_free(
    _opaque: *mut core::ffi::c_void,
    address: *mut core::ffi::c_void,
) {
    kvfree(address);
}

unsafe fn zstd_release_params(params: *mut zcomp_params) {
    let zp = (*params).drv_data as *mut zstd_params;

    (*params).drv_data = core::ptr::null_mut();
    if zp.is_null() {
        return;
    }

    zstd_free_cdict((*zp).cdict);
    zstd_free_ddict((*zp).ddict);
    kfree(zp as *mut core::ffi::c_void);
}

unsafe fn zstd_setup_params(params: *mut zcomp_params) -> i32 {
    let mut prm: zstd_compression_parameters;
    let zp = kzalloc_obj::<zstd_params>();

    if zp.is_null() {
        return -ENOMEM;
    }

    (*params).drv_data = zp as *mut core::ffi::c_void;
    if (*params).level == ZCOMP_PARAM_NOT_SET {
        (*params).level = zstd_default_clevel();
    } else if (*params).level < zstd_min_clevel()
        || (*params).level > zstd_max_clevel()
    {
        pr_err!("invalid compression level {}\n", (*params).level);
        return -EINVAL;
    }

    (*zp).cprm = zstd_get_params((*params).level, PAGE_SIZE);

    (*zp).custom_mem.customAlloc = Some(zstd_custom_alloc);
    (*zp).custom_mem.customFree = Some(zstd_custom_free);

    prm = zstd_get_cparams((*params).level, PAGE_SIZE, (*params).dict_sz);

    (*zp).cdict = zstd_create_cdict_byreference(
        (*params).dict,
        (*params).dict_sz,
        prm,
        (*zp).custom_mem,
    );
    if (*zp).cdict.is_null() {
        return -EINVAL;
    }

    (*zp).ddict = zstd_create_ddict_byreference(
        (*params).dict,
        (*params).dict_sz,
        (*zp).custom_mem,
    );
    if (*zp).ddict.is_null() {
        return -EINVAL;
    }

    0
}

unsafe fn zstd_destroy(ctx: *mut zcomp_ctx) {
    let zctx = (*ctx).context as *mut zstd_ctx;

    if zctx.is_null() {
        return;
    }

    /*
     * If ->cctx_mem and ->dctx_mem were allocated then we didn't use
     * C/D dictionary and ->cctx / ->dctx were "embedded" into these
     * buffers.
     *
     * If otherwise then we need to explicitly release ->cctx / ->dctx.
     */
    if !(*zctx).cctx_mem.is_null() {
        vfree((*zctx).cctx_mem);
    } else {
        zstd_free_cctx((*zctx).cctx);
    }

    if !(*zctx).dctx_mem.is_null() {
        vfree((*zctx).dctx_mem);
    } else {
        zstd_free_dctx((*zctx).dctx);
    }

    kfree(zctx as *mut core::ffi::c_void);
}

unsafe fn zstd_create(params: *mut zcomp_params, ctx: *mut zcomp_ctx) -> i32 {
    let zctx = kzalloc_obj::<zstd_ctx>();
    let mut prm: zstd_parameters;
    let mut sz: usize;

    if zctx.is_null() {
        return -ENOMEM;
    }

    (*ctx).context = zctx as *mut core::ffi::c_void;
    if (*params).dict_sz == 0 {
        prm = zstd_get_params((*params).level, PAGE_SIZE);
        sz = zstd_cctx_workspace_bound(&prm.cParams);
        (*zctx).cctx_mem = vzalloc(sz);
        if (*zctx).cctx_mem.is_null() {
            zstd_destroy(ctx);
            return -EINVAL;
        }

        (*zctx).cctx = zstd_init_cctx((*zctx).cctx_mem, sz);
        if (*zctx).cctx.is_null() {
            zstd_destroy(ctx);
            return -EINVAL;
        }

        sz = zstd_dctx_workspace_bound();
        (*zctx).dctx_mem = vzalloc(sz);
        if (*zctx).dctx_mem.is_null() {
            zstd_destroy(ctx);
            return -EINVAL;
        }

        (*zctx).dctx = zstd_init_dctx((*zctx).dctx_mem, sz);
        if (*zctx).dctx.is_null() {
            zstd_destroy(ctx);
            return -EINVAL;
        }
    } else {
        let zp = (*params).drv_data as *mut zstd_params;

        (*zctx).cctx = zstd_create_cctx_advanced((*zp).custom_mem);
        if (*zctx).cctx.is_null() {
            zstd_destroy(ctx);
            return -EINVAL;
        }

        (*zctx).dctx = zstd_create_dctx_advanced((*zp).custom_mem);
        if (*zctx).dctx.is_null() {
            zstd_destroy(ctx);
            return -EINVAL;
        }
    }

    0
}

unsafe fn zstd_compress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let zp = (*params).drv_data as *mut zstd_params;
    let zctx = (*ctx).context as *mut zstd_ctx;
    let ret: usize;

    if (*params).dict_sz == 0 {
        ret = zstd_compress_cctx(
            (*zctx).cctx, (*req).dst, (*req).dst_len, (*req).src,
            (*req).src_len, &(*zp).cprm,
        );
    } else {
        ret = zstd_compress_using_cdict(
            (*zctx).cctx, (*req).dst, (*req).dst_len, (*req).src,
            (*req).src_len, (*zp).cdict,
        );
    }
    if zstd_is_error(ret) {
        return -EINVAL;
    }
    (*req).dst_len = ret;
    0
}

unsafe fn zstd_decompress(
    params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let zp = (*params).drv_data as *mut zstd_params;
    let zctx = (*ctx).context as *mut zstd_ctx;
    let ret: usize;

    if (*params).dict_sz == 0 {
        ret = zstd_decompress_dctx(
            (*zctx).dctx, (*req).dst, (*req).dst_len, (*req).src,
            (*req).src_len,
        );
    } else {
        ret = zstd_decompress_using_ddict(
            (*zctx).dctx, (*req).dst, (*req).dst_len, (*req).src,
            (*req).src_len, (*zp).ddict,
        );
    }
    if zstd_is_error(ret) {
        return -EINVAL;
    }
    0
}

pub static backend_zstd: zcomp_ops = zcomp_ops {
    compress: Some(zstd_compress),
    decompress: Some(zstd_decompress),
    create_ctx: Some(zstd_create),
    destroy_ctx: Some(zstd_destroy),
    setup_params: Some(zstd_setup_params),
    release_params: Some(zstd_release_params),
    name: b"zstd\0".as_ptr() as *const i8,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
