/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* Dependency types and functions are supplied by the surrounding kernel translation. */
pub type gfp_t = u32;

#[repr(C)]
pub struct io_alloc_cache {
    pub entries: *mut *mut c_void,
    pub nr_cached: usize,
    pub max_cached: usize,
    pub elem_size: usize,
    pub init_clear: usize,
}

pub const IO_ALLOC_CACHE_MAX: usize = 128;

unsafe extern "C" {
    pub fn io_alloc_cache_free(cache: *mut io_alloc_cache, free: unsafe extern "C" fn(*const c_void));
    pub fn io_alloc_cache_init(
        cache: *mut io_alloc_cache,
        max_nr: u32,
        size: u32,
        init_bytes: u32,
    ) -> bool;
    pub fn io_cache_alloc_new(cache: *mut io_alloc_cache, gfp: gfp_t) -> *mut c_void;

    pub fn kasan_mempool_poison_object(entry: *const c_void) -> bool;
    pub fn kasan_mempool_unpoison_object(entry: *const c_void, size: usize);
    pub fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    pub fn kvfree(addr: *const c_void);
}

#[inline]
pub unsafe fn io_alloc_cache_put(cache: *mut io_alloc_cache, entry: *mut c_void) -> bool {
    let cache_ref = &mut *cache;
    if cache_ref.nr_cached < cache_ref.max_cached {
        if !kasan_mempool_poison_object(entry) {
            return false;
        }
        *cache_ref.entries.add(cache_ref.nr_cached) = entry;
        cache_ref.nr_cached += 1;
        return true;
    }
    false
}

#[inline]
pub unsafe fn io_alloc_cache_get(cache: *mut io_alloc_cache) -> *mut c_void {
    let cache_ref = &mut *cache;
    if cache_ref.nr_cached != 0 {
        cache_ref.nr_cached -= 1;
        let entry = *cache_ref.entries.add(cache_ref.nr_cached);

        /* CONFIG_KASAN: clear the initial bytes after unpoisoning, when enabled. */
        #[cfg(feature = "CONFIG_KASAN")]
        {
            kasan_mempool_unpoison_object(entry, cache_ref.elem_size);
            if cache_ref.init_clear != 0 {
                memset(entry, 0, cache_ref.init_clear);
            }
        }
        return entry;
    }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn io_cache_alloc(cache: *mut io_alloc_cache, gfp: gfp_t) -> *mut c_void {
    let obj = io_alloc_cache_get(cache);
    if !obj.is_null() {
        return obj;
    }
    io_cache_alloc_new(cache, gfp)
}

#[inline]
pub unsafe fn io_cache_free(cache: *mut io_alloc_cache, obj: *mut c_void) {
    if !io_alloc_cache_put(cache, obj) {
        kvfree(obj);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
