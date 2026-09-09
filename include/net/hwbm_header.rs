/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to: #include <linux/mutex.h>

#[repr(C)]
pub struct hwbm_pool {
    /* Capacity of the pool */
    pub size: ::core::ffi::c_int,
    /* Size of the buffers managed */
    pub frag_size: ::core::ffi::c_int,
    /* Number of buffers currently used by this pool */
    pub buf_num: ::core::ffi::c_int,
    /* constructor called during allocation */
    pub construct: ::core::option::Option<
        unsafe extern "C" fn(bm_pool: *mut hwbm_pool, buf: *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    /* protect access to the buffer counter*/
    pub buf_lock: crate::mutex,
    /* private data */
    pub priv_: *mut ::core::ffi::c_void,
}

#[cfg(feature = "CONFIG_HWBM")]
extern "C" {
    pub fn hwbm_buf_free(bm_pool: *mut hwbm_pool, buf: *mut ::core::ffi::c_void);
    pub fn hwbm_pool_refill(bm_pool: *mut hwbm_pool, gfp: crate::gfp_t) -> ::core::ffi::c_int;
    pub fn hwbm_pool_add(bm_pool: *mut hwbm_pool, buf_num: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_HWBM"))]
#[inline]
pub unsafe fn hwbm_buf_free(_bm_pool: *mut hwbm_pool, _buf: *mut ::core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_HWBM"))]
#[inline]
pub unsafe fn hwbm_pool_refill(_bm_pool: *mut hwbm_pool, _gfp: crate::gfp_t) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_HWBM"))]
#[inline]
pub unsafe fn hwbm_pool_add(_bm_pool: *mut hwbm_pool, _buf_num: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
