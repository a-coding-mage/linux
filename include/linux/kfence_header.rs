/* SPDX-License-Identifier: GPL-2.0 */
/* Kernel Electric-Fence (KFENCE) public interface. */

/* C dependencies: linux/mm.h, linux/types.h, linux/atomic.h, linux/static_key.h. */

#[cfg(feature = "CONFIG_KFENCE")]
extern "C" {
    pub static mut kfence_sample_interval: ::core::ffi::c_ulong;
    pub static mut __kfence_pool: *mut ::core::ffi::c_char;
    pub static mut kfence_allocation_key: ::core::ffi::c_uchar;
    pub static mut kfence_allocation_gate: atomic_t;

    pub fn kfence_alloc_pool_and_metadata();
    pub fn kfence_init();
    pub fn kfence_shutdown_cache(s: *mut kmem_cache);
    pub fn __kfence_alloc(s: *mut kmem_cache, size: usize, flags: gfp_t) -> *mut ::core::ffi::c_void;
    pub fn kfence_ksize(addr: *const ::core::ffi::c_void) -> usize;
    pub fn kfence_object_start(addr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn __kfence_free(addr: *mut ::core::ffi::c_void);
    pub fn kfence_handle_page_fault(addr: ::core::ffi::c_ulong, is_write: bool, regs: *mut pt_regs) -> bool;
}

#[cfg(all(feature = "CONFIG_KFENCE", feature = "CONFIG_PRINTK"))]
extern "C" {
    // Declaration retained for the kernel's kmem_obj_info integration.
    pub fn __kfence_obj_info(kpp: *mut kmem_obj_info, object: *mut ::core::ffi::c_void, slab: *mut slab) -> bool;
}

#[cfg(feature = "CONFIG_KFENCE")]
pub const KFENCE_POOL_SIZE: usize = (CONFIG_KFENCE_NUM_OBJECTS + 1) * 2 * PAGE_SIZE;

#[cfg(feature = "CONFIG_KFENCE")]
#[inline(always)]
pub unsafe fn is_kfence_address(addr: *const ::core::ffi::c_void) -> bool {
    // Keep the pool-null check in the slow path after the range check.
    let difference = (addr as *const ::core::ffi::c_char).offset_from(__kfence_pool);
    (difference as usize) < KFENCE_POOL_SIZE && !__kfence_pool.is_null()
}

#[cfg(feature = "CONFIG_KFENCE")]
#[inline(always)]
pub unsafe fn kfence_alloc(s: *mut kmem_cache, size: usize, flags: gfp_t) -> *mut ::core::ffi::c_void {
    #[cfg(any(feature = "CONFIG_KFENCE_STATIC_KEYS", feature = "CONFIG_KFENCE_SAMPLE_INTERVAL_0"))]
    {
        if !static_branch_unlikely(&raw const kfence_allocation_key) { return core::ptr::null_mut(); }
    }
    #[cfg(not(any(feature = "CONFIG_KFENCE_STATIC_KEYS", feature = "CONFIG_KFENCE_SAMPLE_INTERVAL_0")))]
    {
        if !static_branch_likely(&raw const kfence_allocation_key) { return core::ptr::null_mut(); }
    }
    if atomic_read(&raw const kfence_allocation_gate) > 0 { return core::ptr::null_mut(); }
    __kfence_alloc(s, size, flags)
}

#[cfg(feature = "CONFIG_KFENCE")]
#[inline(always)]
pub unsafe fn kfence_free(addr: *mut ::core::ffi::c_void) -> bool {
    if !is_kfence_address(addr as *const _) { return false; }
    __kfence_free(addr);
    true
}

#[cfg(not(feature = "CONFIG_KFENCE"))]
pub const kfence_sample_interval: usize = 0;

#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn is_kfence_address(_addr: *const ::core::ffi::c_void) -> bool { false }
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_alloc_pool_and_metadata() {}
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_init() {}
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_shutdown_cache(_s: *mut kmem_cache) {}
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_alloc(_s: *mut kmem_cache, _size: usize, _flags: gfp_t) -> *mut ::core::ffi::c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_ksize(_addr: *const ::core::ffi::c_void) -> usize { 0 }
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_object_start(_addr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn __kfence_free(_addr: *mut ::core::ffi::c_void) {}
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_free(_addr: *mut ::core::ffi::c_void) -> bool { false }
#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)] pub unsafe fn kfence_handle_page_fault(_addr: ::core::ffi::c_ulong, _is_write: bool, _regs: *mut pt_regs) -> bool { false }
#[cfg(all(not(feature = "CONFIG_KFENCE"), feature = "CONFIG_PRINTK"))]
#[inline(always)] pub unsafe fn __kfence_obj_info(_kpp: *mut kmem_obj_info, _object: *mut ::core::ffi::c_void, _slab: *mut slab) -> bool { false }

/* External kernel types and helpers are supplied by the translated dependencies. */
extern "C" {
    fn static_branch_unlikely(key: *const ::core::ffi::c_uchar) -> bool;
    fn static_branch_likely(key: *const ::core::ffi::c_uchar) -> bool;
    fn atomic_read(v: *const atomic_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
