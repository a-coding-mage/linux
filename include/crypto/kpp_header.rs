/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Key-agreement Protocol Primitives (KPP) */
/*
 * C dependencies: linux/atomic.h, linux/container_of.h, linux/crypto.h,
 * linux/slab.h. Their declarations are supplied by other translated units.
 */

#[repr(C)]
pub struct kpp_request {
    pub base: crypto_async_request,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub src_len: c_uint,
    pub dst_len: c_uint,
    pub __ctx: [c_void; 0],
}

#[repr(C)]
pub struct crypto_kpp {
    pub reqsize: c_uint,
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct kpp_alg {
    pub set_secret: Option<unsafe extern "C" fn(*mut crypto_kpp, *const c_void, c_uint) -> c_int>,
    pub generate_public_key: Option<unsafe extern "C" fn(*mut kpp_request) -> c_int>,
    pub compute_shared_secret: Option<unsafe extern "C" fn(*mut kpp_request) -> c_int>,
    pub max_size: Option<unsafe extern "C" fn(*mut crypto_kpp) -> c_uint>,
    pub init: Option<unsafe extern "C" fn(*mut crypto_kpp) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut crypto_kpp)>,
    pub base: crypto_alg,
}

extern "C" {
    pub fn crypto_alloc_kpp(alg_name: *const c_char, type_: u32, mask: u32) -> *mut crypto_kpp;
    pub fn crypto_has_kpp(alg_name: *const c_char, type_: u32, mask: u32) -> c_int;
}

#[inline]
pub unsafe fn crypto_kpp_tfm(tfm: *mut crypto_kpp) -> *mut crypto_tfm {
    &mut (*tfm).base
}

#[inline]
pub unsafe fn __crypto_kpp_alg(alg: *mut crypto_alg) -> *mut kpp_alg {
    // Equivalent to container_of(alg, struct kpp_alg, base).
    (alg as *mut u8).sub(core::mem::offset_of!(kpp_alg, base)) as *mut kpp_alg
}

#[inline]
pub unsafe fn __crypto_kpp_tfm(tfm: *mut crypto_tfm) -> *mut crypto_kpp {
    (tfm as *mut u8).sub(core::mem::offset_of!(crypto_kpp, base)) as *mut crypto_kpp
}

#[inline]
pub unsafe fn crypto_kpp_alg(tfm: *mut crypto_kpp) -> *mut kpp_alg {
    __crypto_kpp_alg((*crypto_kpp_tfm(tfm)).__crt_alg)
}

#[inline]
pub unsafe fn crypto_kpp_reqsize(tfm: *mut crypto_kpp) -> c_uint { (*tfm).reqsize }

#[inline]
pub unsafe fn kpp_request_set_tfm(req: *mut kpp_request, tfm: *mut crypto_kpp) {
    (*req).base.tfm = crypto_kpp_tfm(tfm);
}

#[inline]
pub unsafe fn crypto_kpp_reqtfm(req: *mut kpp_request) -> *mut crypto_kpp {
    __crypto_kpp_tfm((*req).base.tfm)
}

#[inline]
pub unsafe fn crypto_kpp_get_flags(tfm: *mut crypto_kpp) -> u32 {
    crypto_tfm_get_flags(crypto_kpp_tfm(tfm))
}

#[inline]
pub unsafe fn crypto_kpp_set_flags(tfm: *mut crypto_kpp, flags: u32) {
    crypto_tfm_set_flags(crypto_kpp_tfm(tfm), flags)
}

#[inline]
pub unsafe fn crypto_free_kpp(tfm: *mut crypto_kpp) {
    crypto_destroy_tfm(tfm as *mut c_void, crypto_kpp_tfm(tfm));
}

#[inline]
pub unsafe fn kpp_request_alloc(tfm: *mut crypto_kpp, gfp: gfp_t) -> *mut kpp_request {
    let req = kmalloc(core::mem::size_of::<kpp_request>() + crypto_kpp_reqsize(tfm) as usize, gfp)
        as *mut kpp_request;
    if likely(!req.is_null()) { kpp_request_set_tfm(req, tfm); }
    req
}

#[inline]
pub unsafe fn kpp_request_free(req: *mut kpp_request) { kfree_sensitive(req as *mut c_void); }

#[inline]
pub unsafe fn kpp_request_set_callback(req: *mut kpp_request, flgs: u32, cmpl: crypto_completion_t, data: *mut c_void) {
    (*req).base.complete = cmpl;
    (*req).base.data = data;
    (*req).base.flags = flgs;
}

#[inline]
pub unsafe fn kpp_request_set_input(req: *mut kpp_request, input: *mut scatterlist, input_len: c_uint) {
    (*req).src = input;
    (*req).src_len = input_len;
}

#[inline]
pub unsafe fn kpp_request_set_output(req: *mut kpp_request, output: *mut scatterlist, output_len: c_uint) {
    (*req).dst = output;
    (*req).dst_len = output_len;
}

pub const CRYPTO_KPP_SECRET_TYPE_UNKNOWN: c_int = 0;
pub const CRYPTO_KPP_SECRET_TYPE_DH: c_int = 1;
pub const CRYPTO_KPP_SECRET_TYPE_ECDH: c_int = 2;

#[repr(C)]
pub struct kpp_secret {
    pub type_: u16,
    pub len: u16,
}

#[inline]
pub unsafe fn crypto_kpp_set_secret(tfm: *mut crypto_kpp, buffer: *const c_void, len: c_uint) -> c_int {
    ((*crypto_kpp_alg(tfm)).set_secret.unwrap())(tfm, buffer, len)
}

#[inline]
pub unsafe fn crypto_kpp_generate_public_key(req: *mut kpp_request) -> c_int {
    let tfm = crypto_kpp_reqtfm(req);
    ((*crypto_kpp_alg(tfm)).generate_public_key.unwrap())(req)
}

#[inline]
pub unsafe fn crypto_kpp_compute_shared_secret(req: *mut kpp_request) -> c_int {
    let tfm = crypto_kpp_reqtfm(req);
    ((*crypto_kpp_alg(tfm)).compute_shared_secret.unwrap())(req)
}

#[inline]
pub unsafe fn crypto_kpp_maxsize(tfm: *mut crypto_kpp) -> c_uint {
    ((*crypto_kpp_alg(tfm)).max_size.unwrap())(tfm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
