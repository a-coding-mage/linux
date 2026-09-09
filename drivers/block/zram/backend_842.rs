// SPDX-License-Identifier: GPL-2.0-or-later

// #define pr_fmt(fmt) "842: " fmt
// Kernel, SW842, vmalloc, and backend_842 dependencies are supplied externally.

use core::ffi::c_void;

#[repr(C)]
pub struct zcomp_params {
    pub dict_sz: usize,
    pub level: i32,
}

#[repr(C)]
pub struct zcomp_ctx {
    pub context: *mut c_void,
}

#[repr(C)]
pub struct zcomp_req {
    pub src: *const u8,
    pub src_len: u32,
    pub dst: *mut u8,
    pub dst_len: u32,
}

#[repr(C)]
pub struct zcomp_ops {
    pub compress: unsafe extern "C" fn(*mut zcomp_params, *mut zcomp_ctx, *mut zcomp_req) -> i32,
    pub decompress: unsafe extern "C" fn(*mut zcomp_params, *mut zcomp_ctx, *mut zcomp_req) -> i32,
    pub create_ctx: unsafe extern "C" fn(*mut zcomp_params, *mut zcomp_ctx) -> i32,
    pub destroy_ctx: unsafe extern "C" fn(*mut zcomp_ctx),
    pub setup_params: unsafe extern "C" fn(*mut zcomp_params) -> i32,
    pub release_params: unsafe extern "C" fn(*mut zcomp_params),
    pub name: *const u8,
}

extern "C" {
    fn pr_err(fmt: *const u8, ...);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn sw842_compress(
        src: *const u8,
        src_len: u32,
        dst: *mut u8,
        dst_len: *mut u32,
        context: *mut c_void,
    ) -> i32;
    fn sw842_decompress(src: *const u8, src_len: u32, dst: *mut u8, dst_len: *mut u32) -> i32;
}

const SW842_MEM_COMPRESS: usize = 0; // supplied by <linux/sw842.h>
const GFP_KERNEL: u32 = 0; // supplied by <linux/slab.h>
const EOPNOTSUPP: i32 = 95;
const ENOMEM: i32 = 12;
const ZCOMP_PARAM_NOT_SET: i32 = -1;

unsafe extern "C" fn release_params_842(_params: *mut zcomp_params) {}

unsafe extern "C" fn setup_params_842(params: *mut zcomp_params) -> i32 {
    if (*params).dict_sz != 0 {
        pr_err(b"842: dictionary is not supported\n\0".as_ptr());
        return -EOPNOTSUPP;
    }
    if (*params).level != ZCOMP_PARAM_NOT_SET {
        pr_err(b"842: compression level is not supported\n\0".as_ptr());
        return -EOPNOTSUPP;
    }
    0
}

unsafe extern "C" fn destroy_842(ctx: *mut zcomp_ctx) {
    kfree((*ctx).context);
}

unsafe extern "C" fn create_842(_params: *mut zcomp_params, ctx: *mut zcomp_ctx) -> i32 {
    (*ctx).context = kmalloc(SW842_MEM_COMPRESS, GFP_KERNEL);
    if (*ctx).context.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe extern "C" fn compress_842(
    _params: *mut zcomp_params,
    ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let mut dlen = (*req).dst_len;
    let ret = sw842_compress(
        (*req).src,
        (*req).src_len,
        (*req).dst,
        &mut dlen,
        (*ctx).context,
    );
    if ret == 0 {
        (*req).dst_len = dlen;
    }
    ret
}

unsafe extern "C" fn decompress_842(
    _params: *mut zcomp_params,
    _ctx: *mut zcomp_ctx,
    req: *mut zcomp_req,
) -> i32 {
    let mut dlen = (*req).dst_len;
    sw842_decompress((*req).src, (*req).src_len, (*req).dst, &mut dlen)
}

#[no_mangle]
pub static backend_842: zcomp_ops = zcomp_ops {
    compress: compress_842,
    decompress: decompress_842,
    create_ctx: create_842,
    destroy_ctx: destroy_842,
    setup_params: setup_params_842,
    release_params: release_params_842,
    name: b"842\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
