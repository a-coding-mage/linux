/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Dependency supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

/* Dependency supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

pub const ZCOMP_PARAM_NOT_SET: i32 = i32::MIN;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct deflate_params {
    pub winbits: i32,
}

/*
 * Immutable driver (backend) parameters. The driver may attach private
 * data to it (e.g. driver representation of the dictionary, etc.).
 *
 * This data is kept per-comp and is shared among execution contexts.
 */
#[repr(C)]
pub union zcomp_params__bindgen_ty_1 {
    pub deflate: deflate_params,
}

#[repr(C)]
pub struct zcomp_params {
    pub dict: *mut core::ffi::c_void,
    pub dict_sz: usize,
    pub level: i32,
    pub __bindgen_anon_1: zcomp_params__bindgen_ty_1,
    pub drv_data: *mut core::ffi::c_void,
}

/*
 * Run-time driver context - scratch buffers, etc. It is modified during
 * request execution (compression/decompression), cannot be shared, so
 * it's in per-CPU area.
 */
#[repr(C)]
pub struct zcomp_ctx {
    pub context: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct zcomp_strm {
    pub lock: mutex,
    /* compression buffer */
    pub buffer: *mut core::ffi::c_void,
    /* local copy of handle memory */
    pub local_copy: *mut core::ffi::c_void,
    pub ctx: zcomp_ctx,
}

#[repr(C)]
pub struct zcomp_req {
    pub src: *const u8,
    pub src_len: usize,
    pub dst: *mut u8,
    pub dst_len: usize,
}

#[repr(C)]
pub struct zcomp_ops {
    pub compress: Option<unsafe extern "C" fn(
        params: *mut zcomp_params,
        ctx: *mut zcomp_ctx,
        req: *mut zcomp_req,
    ) -> i32>,
    pub decompress: Option<unsafe extern "C" fn(
        params: *mut zcomp_params,
        ctx: *mut zcomp_ctx,
        req: *mut zcomp_req,
    ) -> i32>,
    pub create_ctx: Option<unsafe extern "C" fn(
        params: *mut zcomp_params,
        ctx: *mut zcomp_ctx,
    ) -> i32>,
    pub destroy_ctx: Option<unsafe extern "C" fn(ctx: *mut zcomp_ctx)>,
    pub setup_params: Option<unsafe extern "C" fn(params: *mut zcomp_params) -> i32>,
    pub release_params: Option<unsafe extern "C" fn(params: *mut zcomp_params)>,
    pub name: *const core::ffi::c_char,
}

/* dynamic per-device compression frontend */
#[repr(C)]
pub struct zcomp {
    pub stream: *mut zcomp_strm,
    pub ops: *const zcomp_ops,
    pub params: *mut zcomp_params,
    pub node: hlist_node,
}

extern "C" {
    pub fn zcomp_cpu_up_prepare(cpu: u32, node: *mut hlist_node) -> i32;
    pub fn zcomp_cpu_dead(cpu: u32, node: *mut hlist_node) -> i32;
    pub fn zcomp_available_show(comp: *const core::ffi::c_char,
                                buf: *mut core::ffi::c_char,
                                at: isize) -> isize;
    pub fn zcomp_lookup_backend_name(comp: *const core::ffi::c_char)
        -> *const core::ffi::c_char;

    pub fn zcomp_create(alg: *const core::ffi::c_char,
                        params: *mut zcomp_params) -> *mut zcomp;
    pub fn zcomp_destroy(comp: *mut zcomp);

    pub fn zcomp_stream_get(comp: *mut zcomp) -> *mut zcomp_strm;
    pub fn zcomp_stream_put(zstrm: *mut zcomp_strm);

    pub fn zcomp_compress(comp: *mut zcomp,
                          zstrm: *mut zcomp_strm,
                          src: *const core::ffi::c_void,
                          dst_len: *mut u32) -> i32;
    pub fn zcomp_decompress(comp: *mut zcomp,
                            zstrm: *mut zcomp_strm,
                            src: *const core::ffi::c_void,
                            src_len: u32,
                            dst: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
