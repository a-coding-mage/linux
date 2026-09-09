// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Declarations supplied by the surrounding kernel environment.
pub type gfp_t = u32;

#[repr(C)]
pub struct io_alloc_cache {
    pub entries: *mut *mut c_void,
    pub nr_cached: u32,
    pub max_cached: u32,
    pub elem_size: u32,
    pub init_clear: u32,
}

extern "C" {
    fn io_alloc_cache_get(cache: *mut io_alloc_cache) -> *mut c_void;
    fn kvfree(addr: *mut c_void);
    fn kvmalloc_array(n: usize, size: usize, flags: gfp_t) -> *mut c_void;
    fn kmalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
}

pub const GFP_KERNEL: gfp_t = 0;

pub unsafe fn io_alloc_cache_free(
    cache: *mut io_alloc_cache,
    free: Option<unsafe extern "C" fn(*const c_void)>,
) {
    let mut entry: *mut c_void;

    if (*cache).entries.is_null() {
        return;
    }

    loop {
        entry = io_alloc_cache_get(cache);
        if entry.is_null() {
            break;
        }
        if let Some(free_fn) = free {
            free_fn(entry as *const c_void);
        }
    }

    kvfree((*cache).entries as *mut c_void);
    (*cache).entries = core::ptr::null_mut();
}

/* returns false if the cache was initialized properly */
pub unsafe fn io_alloc_cache_init(
    cache: *mut io_alloc_cache,
    max_nr: u32,
    size: u32,
    init_bytes: u32,
) -> bool {
    (*cache).entries = kvmalloc_array(
        max_nr as usize,
        core::mem::size_of::<*mut c_void>(),
        GFP_KERNEL,
    ) as *mut *mut c_void;
    if (*cache).entries.is_null() {
        return true;
    }

    (*cache).nr_cached = 0;
    (*cache).max_cached = max_nr;
    (*cache).elem_size = size;
    (*cache).init_clear = init_bytes;
    false
}

pub unsafe fn io_cache_alloc_new(cache: *mut io_alloc_cache, gfp: gfp_t) -> *mut c_void {
    let obj: *mut c_void;

    obj = kmalloc((*cache).elem_size as usize, gfp);
    if !obj.is_null() && (*cache).init_clear != 0 {
        memset(obj, 0, (*cache).init_clear as usize);
    }
    obj
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
