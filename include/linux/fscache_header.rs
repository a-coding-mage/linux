/* SPDX-License-Identifier: GPL-2.0-or-later */
/* General filesystem caching interface */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: fs.h, netfs.h and writeback.h.

#[cfg(any(feature = "CONFIG_FSCACHE", feature = "CONFIG_FSCACHE_MODULE"))]
pub const __fscache_available: i32 = 1;
#[cfg(not(any(feature = "CONFIG_FSCACHE", feature = "CONFIG_FSCACHE_MODULE")))]
pub const __fscache_available: i32 = 0;

#[repr(C)]
pub struct fscache_volume {
    pub ref_: refcount_t,
    pub n_cookies: atomic_t,
    pub n_accesses: atomic_t,
    pub debug_id: core::ffi::c_uint,
    pub key_hash: core::ffi::c_uint,
    pub key: *mut u8,
    pub proc_link: list_head,
    pub hash_link: hlist_bl_node,
    pub work: work_struct,
    pub cache: *mut fscache_cache,
    pub cache_priv: *mut core::ffi::c_void,
    pub lock: spinlock_t,
    pub flags: core::ffi::c_ulong,
    pub coherency_len: u8,
    pub coherency: [u8; 0],
}

#[repr(C)]
pub struct fscache_cookie {
    pub ref_: refcount_t,
    pub n_active: atomic_t,
    pub n_accesses: atomic_t,
    pub debug_id: core::ffi::c_uint,
    pub inval_counter: core::ffi::c_uint,
    pub lock: spinlock_t,
    pub volume: *mut fscache_volume,
    pub cache_priv: *mut core::ffi::c_void,
    pub hash_link: hlist_bl_node,
    pub proc_link: list_head,
    pub commit_link: list_head,
    pub work: work_struct,
    pub object_size: loff_t,
    pub unused_at: core::ffi::c_ulong,
    pub flags: core::ffi::c_ulong,
    pub state: fscache_cookie_state,
    pub advice: u8,
    pub key_len: u8,
    pub aux_len: u8,
    pub key_hash: u32,
    pub key_or_inline_key: fscache_cookie_key,
    pub aux_or_inline_aux: fscache_cookie_aux,
}

#[repr(C)]
pub union fscache_cookie_key {
    pub key: *mut core::ffi::c_void,
    pub inline_key: [u8; 16],
}
#[repr(C)]
pub union fscache_cookie_aux {
    pub aux: *mut core::ffi::c_void,
    pub inline_aux: [u8; 8],
}

#[repr(i32)]
pub enum fscache_want_state { FSCACHE_WANT_PARAMS, FSCACHE_WANT_WRITE, FSCACHE_WANT_READ }
#[repr(i8)]
pub enum fscache_cookie_state {
    FSCACHE_COOKIE_STATE_QUIESCENT,
    FSCACHE_COOKIE_STATE_LOOKING_UP,
    FSCACHE_COOKIE_STATE_CREATING,
    FSCACHE_COOKIE_STATE_ACTIVE,
    FSCACHE_COOKIE_STATE_INVALIDATING,
    FSCACHE_COOKIE_STATE_FAILED,
    FSCACHE_COOKIE_STATE_LRU_DISCARDING,
    FSCACHE_COOKIE_STATE_WITHDRAWING,
    FSCACHE_COOKIE_STATE_RELINQUISHING,
    FSCACHE_COOKIE_STATE_DROPPED,
}
pub const FSCACHE_COOKIE_STATE__NR: u8 = 10;

pub const FSCACHE_ADV_SINGLE_CHUNK: u8 = 0x01;
pub const FSCACHE_ADV_WRITE_CACHE: u8 = 0x00;
pub const FSCACHE_ADV_WRITE_NOCACHE: u8 = 0x02;
pub const FSCACHE_ADV_WANT_CACHE_SIZE: u8 = 0x04;
pub const FSCACHE_INVAL_DIO_WRITE: u8 = 0x01;

pub const FSCACHE_VOLUME_RELINQUISHED: u32 = 0;
pub const FSCACHE_VOLUME_INVALIDATE: u32 = 1;
pub const FSCACHE_VOLUME_COLLIDED_WITH: u32 = 2;
pub const FSCACHE_VOLUME_ACQUIRE_PENDING: u32 = 3;
pub const FSCACHE_VOLUME_CREATING: u32 = 4;
pub const FSCACHE_COOKIE_RELINQUISHED: u32 = 0;
pub const FSCACHE_COOKIE_RETIRED: u32 = 1;
pub const FSCACHE_COOKIE_IS_CACHING: u32 = 2;
pub const FSCACHE_COOKIE_NO_DATA_TO_READ: u32 = 3;
pub const FSCACHE_COOKIE_NEEDS_UPDATE: u32 = 4;
pub const FSCACHE_COOKIE_HAS_BEEN_CACHED: u32 = 5;
pub const FSCACHE_COOKIE_DISABLED: u32 = 6;
pub const FSCACHE_COOKIE_LOCAL_WRITE: u32 = 7;
pub const FSCACHE_COOKIE_NO_ACCESS_WAKE: u32 = 8;
pub const FSCACHE_COOKIE_DO_RELINQUISH: u32 = 9;
pub const FSCACHE_COOKIE_DO_WITHDRAW: u32 = 10;
pub const FSCACHE_COOKIE_DO_LRU_DISCARD: u32 = 11;
pub const FSCACHE_COOKIE_DO_PREP_TO_WRITE: u32 = 12;
pub const FSCACHE_COOKIE_HAVE_DATA: u32 = 13;
pub const FSCACHE_COOKIE_IS_HASHED: u32 = 14;
pub const FSCACHE_COOKIE_DO_INVALIDATE: u32 = 15;

// External declarations and kernel helpers used by the inline interface.
extern "C" {
    pub fn __fscache_acquire_volume(a: *const i8, b: *const i8, c: *const core::ffi::c_void, d: usize) -> *mut fscache_volume;
    pub fn __fscache_relinquish_volume(a: *mut fscache_volume, b: *const core::ffi::c_void, c: bool);
    pub fn __fscache_acquire_cookie(a: *mut fscache_volume, b: u8, c: *const core::ffi::c_void, d: usize, e: *const core::ffi::c_void, f: usize, g: loff_t) -> *mut fscache_cookie;
    pub fn __fscache_use_cookie(a: *mut fscache_cookie, b: bool);
    pub fn __fscache_unuse_cookie(a: *mut fscache_cookie, b: *const core::ffi::c_void, c: *const loff_t);
    pub fn __fscache_relinquish_cookie(a: *mut fscache_cookie, b: bool);
    pub fn __fscache_resize_cookie(a: *mut fscache_cookie, b: loff_t);
    pub fn __fscache_invalidate(a: *mut fscache_cookie, b: *const core::ffi::c_void, c: loff_t, d: core::ffi::c_uint);
    pub fn __fscache_begin_read_operation(a: *mut netfs_cache_resources, b: *mut fscache_cookie) -> i32;
    pub fn __fscache_begin_write_operation(a: *mut netfs_cache_resources, b: *mut fscache_cookie) -> i32;
    pub fn __fscache_clear_page_bits(a: *mut address_space, b: loff_t, c: usize);
    pub fn __fscache_write_to_cache(a:*mut fscache_cookie,b:*mut address_space,c:loff_t,d:usize,e:loff_t,f:netfs_io_terminated_t,g:*mut core::ffi::c_void,h:bool,i:bool);
}

#[inline]
pub unsafe fn fscache_get_aux(cookie: *mut fscache_cookie) -> *mut core::ffi::c_void {
    if (*cookie).aux_len as usize <= 8 { (*cookie).aux_or_inline_aux.inline_aux.as_mut_ptr() as *mut _ } else { (*cookie).aux_or_inline_aux.aux }
}

#[inline]
pub unsafe fn fscache_update_aux(cookie: *mut fscache_cookie, aux_data: *const core::ffi::c_void, object_size: *const loff_t) {
    let p = fscache_get_aux(cookie);
    if !aux_data.is_null() && !p.is_null() { core::ptr::copy_nonoverlapping(aux_data as *const u8, p as *mut u8, (*cookie).aux_len as usize); }
    if !object_size.is_null() { (*cookie).object_size = *object_size; }
}

// The remaining inline wrappers retain the original kernel operations and are
// declared against the external types/functions supplied by the translation.
pub unsafe fn fscache_acquire_volume(a: *const i8,b: *const i8,c: *const core::ffi::c_void,d: usize)->*mut fscache_volume { if __fscache_available != 0 { __fscache_acquire_volume(a,b,c,d) } else { core::ptr::null_mut() } }
pub unsafe fn fscache_relinquish_volume(a:*mut fscache_volume,b:*const core::ffi::c_void,c:bool){ if !a.is_null(){__fscache_relinquish_volume(a,b,c)} }
pub unsafe fn fscache_acquire_cookie(a:*mut fscache_volume,b:u8,c:*const core::ffi::c_void,d:usize,e:*const core::ffi::c_void,f:usize,g:loff_t)->*mut fscache_cookie { if !a.is_null(){__fscache_acquire_cookie(a,b,c,d,e,f,g)}else{core::ptr::null_mut()} }
pub unsafe fn fscache_use_cookie(a:*mut fscache_cookie,b:bool){if !a.is_null(){__fscache_use_cookie(a,b)}}
pub unsafe fn fscache_unuse_cookie(a:*mut fscache_cookie,b:*const core::ffi::c_void,c:*const loff_t){if !a.is_null(){__fscache_unuse_cookie(a,b,c)}}
pub unsafe fn fscache_relinquish_cookie(a:*mut fscache_cookie,b:bool){if !a.is_null(){__fscache_relinquish_cookie(a,b)}}
pub unsafe fn fscache_resize_cookie(a:*mut fscache_cookie,b:loff_t){if !a.is_null(){__fscache_resize_cookie(a,b)}}
pub unsafe fn fscache_invalidate(a:*mut fscache_cookie,b:*const core::ffi::c_void,c:loff_t,d:core::ffi::c_uint){if !a.is_null(){__fscache_invalidate(a,b,c,d)}}

#[inline]
pub unsafe fn __fscache_update_cookie(cookie:*mut fscache_cookie, aux_data:*const core::ffi::c_void, object_size:*const loff_t) {
    fscache_update_aux(cookie, aux_data, object_size);
    // smp_wmb(); set_bit(FSCACHE_COOKIE_NEEDS_UPDATE, &cookie->flags);
}
#[inline]
pub unsafe fn fscache_update_cookie(a:*mut fscache_cookie,b:*const core::ffi::c_void,c:*const loff_t){if !a.is_null(){__fscache_update_cookie(a,b,c)}}

#[inline]
pub unsafe fn fscache_operation_valid(cres:*const netfs_cache_resources)->*const netfs_cache_ops {
    if cres.is_null(){core::ptr::null()}else{(*cres).cache_priv as *const netfs_cache_ops}
}
#[inline]
pub unsafe fn fscache_begin_read_operation(a:*mut netfs_cache_resources,b:*mut fscache_cookie)->i32 { if !b.is_null(){__fscache_begin_read_operation(a,b)}else{-105} }
#[inline]
pub unsafe fn fscache_begin_write_operation(a:*mut netfs_cache_resources,b:*mut fscache_cookie)->i32 { if !b.is_null(){__fscache_begin_write_operation(a,b)}else{-105} }
#[inline]
pub unsafe fn fscache_end_operation(cres:*mut netfs_cache_resources){let ops=fscache_operation_valid(cres);if !ops.is_null(){((*ops).end_operation)(cres)}}
#[inline]
pub unsafe fn fscache_clear_page_bits(a:*mut address_space,b:loff_t,c:usize,d:bool){if d{__fscache_clear_page_bits(a,b,c)}}
#[inline]
pub unsafe fn fscache_write_to_cache(a:*mut fscache_cookie,b:*mut address_space,c:loff_t,d:usize,e:loff_t,f:netfs_io_terminated_t,g:*mut core::ffi::c_void,h:bool,i:bool){if i{__fscache_write_to_cache(a,b,c,d,e,f,g,h,i)}else if !f.is_none(){f.unwrap()(g,-105)}}

#[inline]
pub unsafe fn fscache_note_page_release(cookie:*mut fscache_cookie){
    // If data has been written to the cache, no longer skip cache reads.
    if !cookie.is_null() {
        // test_bit(FSCACHE_COOKIE_HAVE_DATA, &cookie->flags) &&
        // test_bit(FSCACHE_COOKIE_NO_DATA_TO_READ, &cookie->flags)
        // clear_bit(FSCACHE_COOKIE_NO_DATA_TO_READ, &cookie->flags);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
