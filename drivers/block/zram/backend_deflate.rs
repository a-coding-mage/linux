// SPDX-License-Identifier: GPL-2.0-or-later

// pr_fmt(fmt) = "deflate: " fmt

use core::ffi::c_void;

// Supplied by the surrounding zcomp/zlib interfaces.
extern "C" {
    fn zlib_deflate_workspacesize(window_bits: i32, mem_level: i32) -> usize;
    fn zlib_deflate_init2(
        stream: *mut z_stream_s,
        level: i32,
        method: i32,
        window_bits: i32,
        mem_level: i32,
        strategy: i32,
    ) -> i32;
    fn zlib_deflate_end(stream: *mut z_stream_s) -> i32;
    fn zlib_deflate_reset(stream: *mut z_stream_s) -> i32;
    fn zlib_deflate(stream: *mut z_stream_s, flush: i32) -> i32;
    fn zlib_inflate_workspacesize() -> usize;
    fn zlib_inflate_init2(stream: *mut z_stream_s, window_bits: i32) -> i32;
    fn zlib_inflate_end(stream: *mut z_stream_s) -> i32;
    fn zlib_inflate_reset(stream: *mut z_stream_s) -> i32;
    fn zlib_inflate(stream: *mut z_stream_s, flush: i32) -> i32;
    fn vzalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn kfree(ptr: *mut c_void);
}

const DEFLATE_DEF_WINBITS: i32 = -11;
const DEFLATE_DEF_MEMLEVEL: i32 = MAX_MEM_LEVEL;

const MAX_MEM_LEVEL: i32 = 9;
const ZCOMP_PARAM_NOT_SET: i32 = -1;
const Z_DEFAULT_COMPRESSION: i32 = -1;
const Z_BEST_COMPRESSION: i32 = 9;
const Z_DEFLATED: i32 = 8;
const Z_DEFAULT_STRATEGY: i32 = 0;
const Z_OK: i32 = 0;
const Z_STREAM_END: i32 = 1;
const Z_FINISH: i32 = 4;
const Z_SYNC_FLUSH: i32 = 2;
const EOPNOTSUPP: i32 = 95;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut u8,
    pub avail_in: usize,
    pub next_out: *mut u8,
    pub avail_out: usize,
    pub total_out: usize,
    pub workspace: *mut c_void,
}

#[repr(C)]
pub struct deflate_ctx {
    pub cctx: z_stream_s,
    pub dctx: z_stream_s,
}

#[repr(C)]
pub struct zcomp_params {
    pub dict_sz: usize,
    pub level: i32,
    pub deflate: zcomp_deflate_params,
}

#[repr(C)]
pub struct zcomp_deflate_params {
    pub winbits: i32,
}

#[repr(C)]
pub struct zcomp_ctx {
    pub context: *mut c_void,
}

#[repr(C)]
pub struct zcomp_req {
    pub src: *const c_void,
    pub src_len: usize,
    pub dst: *mut c_void,
    pub dst_len: usize,
}

#[repr(C)]
pub struct zcomp_ops {
    pub compress: Option<unsafe extern "C" fn(*mut zcomp_params, *mut zcomp_ctx, *mut zcomp_req) -> i32>,
    pub decompress: Option<unsafe extern "C" fn(*mut zcomp_params, *mut zcomp_ctx, *mut zcomp_req) -> i32>,
    pub create_ctx: Option<unsafe extern "C" fn(*mut zcomp_params, *mut zcomp_ctx) -> i32>,
    pub destroy_ctx: Option<unsafe extern "C" fn(*mut zcomp_ctx)>,
    pub setup_params: Option<unsafe extern "C" fn(*mut zcomp_params) -> i32>,
    pub release_params: Option<unsafe extern "C" fn(*mut zcomp_params)>,
    pub name: *const u8,
}

unsafe extern "C" fn deflate_release_params(_params: *mut zcomp_params) {}

unsafe extern "C" fn deflate_setup_params(params: *mut zcomp_params) -> i32 {
    if (*params).dict_sz != 0 {
        return -EOPNOTSUPP;
    }
    if (*params).level == ZCOMP_PARAM_NOT_SET {
        (*params).level = Z_DEFAULT_COMPRESSION;
    } else if (*params).level < Z_DEFAULT_COMPRESSION || (*params).level > Z_BEST_COMPRESSION {
        return -EINVAL;
    }
    if (*params).deflate.winbits == ZCOMP_PARAM_NOT_SET {
        (*params).deflate.winbits = DEFLATE_DEF_WINBITS;
    } else {
        let wb = (*params).deflate.winbits;
        if (wb < -15 || wb > -9) && (wb < 9 || wb > 15) {
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn deflate_destroy(ctx: *mut zcomp_ctx) {
    let zctx = (*ctx).context as *mut deflate_ctx;
    if zctx.is_null() { return; }
    if !(*zctx).cctx.workspace.is_null() {
        zlib_deflate_end(&mut (*zctx).cctx);
        vfree((*zctx).cctx.workspace);
    }
    if !(*zctx).dctx.workspace.is_null() {
        zlib_inflate_end(&mut (*zctx).dctx);
        vfree((*zctx).dctx.workspace);
    }
    kfree(zctx.cast());
}

unsafe extern "C" fn deflate_create(params: *mut zcomp_params, ctx: *mut zcomp_ctx) -> i32 {
    let zctx = vzalloc(core::mem::size_of::<deflate_ctx>()) as *mut deflate_ctx;
    if zctx.is_null() { return -ENOMEM; }
    (*ctx).context = zctx.cast();
    let sz = zlib_deflate_workspacesize((*params).deflate.winbits, MAX_MEM_LEVEL);
    (*zctx).cctx.workspace = vzalloc(sz);
    if (*zctx).cctx.workspace.is_null() { deflate_destroy(ctx); return -EINVAL; }
    let ret = zlib_deflate_init2(&mut (*zctx).cctx, (*params).level, Z_DEFLATED,
        (*params).deflate.winbits, DEFLATE_DEF_MEMLEVEL, Z_DEFAULT_STRATEGY);
    if ret != Z_OK { deflate_destroy(ctx); return -EINVAL; }
    let sz = zlib_inflate_workspacesize();
    (*zctx).dctx.workspace = vzalloc(sz);
    if (*zctx).dctx.workspace.is_null() { deflate_destroy(ctx); return -EINVAL; }
    let ret = zlib_inflate_init2(&mut (*zctx).dctx, (*params).deflate.winbits);
    if ret != Z_OK { deflate_destroy(ctx); return -EINVAL; }
    0
}

unsafe extern "C" fn deflate_compress(_params: *mut zcomp_params, ctx: *mut zcomp_ctx, req: *mut zcomp_req) -> i32 {
    let deflate = &mut (*( (*ctx).context as *mut deflate_ctx)).cctx;
    if zlib_deflate_reset(deflate) != Z_OK { return -EINVAL; }
    deflate.next_in = (*req).src as *mut u8; deflate.avail_in = (*req).src_len;
    deflate.next_out = (*req).dst as *mut u8; deflate.avail_out = (*req).dst_len;
    if zlib_deflate(deflate, Z_FINISH) != Z_STREAM_END { return -EINVAL; }
    (*req).dst_len = deflate.total_out; 0
}

unsafe extern "C" fn deflate_decompress(_params: *mut zcomp_params, ctx: *mut zcomp_ctx, req: *mut zcomp_req) -> i32 {
    let inflate = &mut (*( (*ctx).context as *mut deflate_ctx)).dctx;
    if zlib_inflate_reset(inflate) != Z_OK { return -EINVAL; }
    inflate.next_in = (*req).src as *mut u8; inflate.avail_in = (*req).src_len;
    inflate.next_out = (*req).dst as *mut u8; inflate.avail_out = (*req).dst_len;
    if zlib_inflate(inflate, Z_SYNC_FLUSH) != Z_STREAM_END { return -EINVAL; }
    0
}

#[no_mangle]
pub static backend_deflate: zcomp_ops = zcomp_ops {
    compress: Some(deflate_compress), decompress: Some(deflate_decompress),
    create_ctx: Some(deflate_create), destroy_ctx: Some(deflate_destroy),
    setup_params: Some(deflate_setup_params), release_params: Some(deflate_release_params),
    name: b"deflate\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
