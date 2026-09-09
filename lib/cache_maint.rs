// SPDX-License-Identifier: GPL-2.0
/*
 * Generic support for Memory System Cache Maintenance operations.
 *
 * Coherency maintenance drivers register with this simple framework that will
 * iterate over each registered instance to first kick off invalidation and
 * then to wait until it is complete.
 *
 * If no implementations are registered yet cpu_cache_has_invalidate_memregion()
 * will return false. If this runs concurrently with unregistration then a
 * race exists but this is no worse than the case where the operations instance
 * responsible for a given memory region has not yet registered.
 */

// Linux kernel types, list primitives, locking, allocation, and exports are
// supplied by the surrounding translation unit.
use core::ffi::c_void;

extern "C" {
    fn kfree(p: *mut c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kref_init(kref: *mut Kref);
    fn kref_put(kref: *mut Kref, release: unsafe extern "C" fn(*mut Kref));
    fn rwsem_read_lock(sem: *mut Rwsem);
    fn rwsem_read_unlock(sem: *mut Rwsem);
    fn rwsem_write_lock(sem: *mut Rwsem);
    fn rwsem_write_unlock(sem: *mut Rwsem);
}

const GFP_KERNEL: u32 = 0;
const EINVAL: i32 = 22;

#[repr(C)]
pub struct Kref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ListHead {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Rwsem {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CcInvalParams {
    pub addr: usize,
    pub size: usize,
}

#[repr(C)]
pub struct CacheCoherencyOps {
    pub wbinv: Option<unsafe extern "C" fn(*mut CacheCoherencyOpsInst, *mut c_void) -> i32>,
    pub done: Option<unsafe extern "C" fn(*mut CacheCoherencyOpsInst) -> i32>,
}

#[repr(C)]
pub struct CacheCoherencyOpsInst {
    pub ops: *const CacheCoherencyOps,
    pub node: ListHead,
    pub kref: Kref,
}

// Equivalent to LIST_HEAD(cache_ops_instance_list) and DECLARE_RWSEM(...).
static mut CACHE_OPS_INSTANCE_LIST: ListHead = ListHead { _private: [] };
static mut CACHE_OPS_INSTANCE_LIST_LOCK: Rwsem = Rwsem { _private: [] };

unsafe extern "C" fn __cache_coherency_ops_instance_free(kref: *mut Kref) {
    // Equivalent to container_of(kref, struct cache_coherency_ops_inst, kref).
    let cci = (kref as *mut u8).sub(core::mem::offset_of!(CacheCoherencyOpsInst, kref))
        as *mut CacheCoherencyOpsInst;
    kfree(cci.cast());
}

#[no_mangle]
pub unsafe extern "C" fn cache_coherency_ops_instance_put(
    cci: *mut CacheCoherencyOpsInst,
) {
    kref_put(&mut (*cci).kref, __cache_coherency_ops_instance_free);
}

unsafe fn cache_inval_one(cci: *mut CacheCoherencyOpsInst, data: *mut c_void) -> i32 {
    if (*cci).ops.is_null() {
        return -EINVAL;
    }
    match (*(*cci).ops).wbinv {
        Some(wbinv) => wbinv(cci, data),
        None => -EINVAL,
    }
}

unsafe fn cache_inval_done_one(cci: *mut CacheCoherencyOpsInst) -> i32 {
    if (*cci).ops.is_null() {
        return -EINVAL;
    }
    match (*(*cci).ops).done {
        Some(done) => done(cci),
        None => 0,
    }
}

unsafe fn cache_invalidate_memregion(addr: usize, size: usize) -> i32 {
    let params = CcInvalParams { addr, size };
    rwsem_read_lock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);

    // list_for_each_entry(cci, &cache_ops_instance_list, node)
    let mut cci: *mut CacheCoherencyOpsInst = list_first_entry(&raw mut CACHE_OPS_INSTANCE_LIST);
    while !cci.is_null() {
        let ret = cache_inval_one(cci, (&params as *const CcInvalParams).cast_mut().cast());
        if ret != 0 {
            rwsem_read_unlock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
            return ret;
        }
        cci = list_next_entry(cci);
    }
    cci = list_first_entry(&raw mut CACHE_OPS_INSTANCE_LIST);
    while !cci.is_null() {
        let ret = cache_inval_done_one(cci);
        if ret != 0 {
            rwsem_read_unlock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
            return ret;
        }
        cci = list_next_entry(cci);
    }
    rwsem_read_unlock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
    0
}

unsafe extern "C" {
    fn list_first_entry(list: *mut ListHead) -> *mut CacheCoherencyOpsInst;
    fn list_next_entry(cci: *mut CacheCoherencyOpsInst) -> *mut CacheCoherencyOpsInst;
    fn INIT_LIST_HEAD(node: *mut ListHead);
    fn list_add(node: *mut ListHead, list: *mut ListHead);
    fn list_del(node: *mut ListHead);
}

#[no_mangle]
pub unsafe extern "C" fn _cache_coherency_ops_instance_alloc(
    ops: *const CacheCoherencyOps,
    size: usize,
) -> *mut CacheCoherencyOpsInst {
    if ops.is_null() || (*ops).wbinv.is_none() {
        return core::ptr::null_mut();
    }
    let cci = kzalloc(size, GFP_KERNEL) as *mut CacheCoherencyOpsInst;
    if cci.is_null() {
        return core::ptr::null_mut();
    }
    (*cci).ops = ops;
    INIT_LIST_HEAD(&mut (*cci).node);
    kref_init(&mut (*cci).kref);
    cci
}

#[no_mangle]
pub unsafe extern "C" fn cache_coherency_ops_instance_register(
    cci: *mut CacheCoherencyOpsInst,
) -> i32 {
    rwsem_write_lock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
    list_add(&mut (*cci).node, &raw mut CACHE_OPS_INSTANCE_LIST);
    rwsem_write_unlock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
    0
}

#[no_mangle]
pub unsafe extern "C" fn cache_coherency_ops_instance_unregister(
    cci: *mut CacheCoherencyOpsInst,
) {
    rwsem_write_lock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
    list_del(&mut (*cci).node);
    rwsem_write_unlock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn cpu_cache_invalidate_memregion(start: usize, len: usize) -> i32 {
    cache_invalidate_memregion(start, len)
}

#[no_mangle]
pub unsafe extern "C" fn cpu_cache_has_invalidate_memregion() -> bool {
    rwsem_read_lock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
    let result = !list_empty(&raw mut CACHE_OPS_INSTANCE_LIST);
    rwsem_read_unlock(&raw mut CACHE_OPS_INSTANCE_LIST_LOCK);
    result
}

unsafe extern "C" {
    fn list_empty(list: *mut ListHead) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
