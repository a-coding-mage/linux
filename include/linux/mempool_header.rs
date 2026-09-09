/* SPDX-License-Identifier: GPL-2.0 */
/*
 * memory buffer pool support
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/sched.h, linux/alloc_tag.h, linux/wait.h, linux/compiler.h

use core::ffi::c_void;

pub struct kmem_cache;

pub type mempool_alloc_t = unsafe extern "C" fn(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void;
pub type mempool_free_t = unsafe extern "C" fn(element: *mut c_void, pool_data: *mut c_void);

#[repr(C)]
pub struct mempool {
    pub lock: spinlock_t,
    pub min_nr: core::ffi::c_int, // nr of elements at *elements
    pub curr_nr: core::ffi::c_int, // Current nr of elements at *elements
    pub elements: *mut *mut c_void,
    pub pool_data: *mut c_void,
    pub alloc: Option<mempool_alloc_t>,
    pub free: Option<mempool_free_t>,
    pub wait: wait_queue_head_t,
}

pub type mempool_t = mempool;

#[inline]
pub unsafe fn mempool_initialized(pool: *mut mempool) -> bool {
    (*pool).elements != core::ptr::null_mut()
}

#[inline]
pub unsafe fn mempool_is_saturated(pool: *mut mempool) -> bool {
    READ_ONCE((*pool).curr_nr) >= (*pool).min_nr
}

extern "C" {
    pub fn mempool_exit(pool: *mut mempool);
    pub fn mempool_init_node(
        pool: *mut mempool,
        min_nr: core::ffi::c_int,
        alloc_fn: Option<mempool_alloc_t>,
        free_fn: Option<mempool_free_t>,
        pool_data: *mut c_void,
        gfp_mask: gfp_t,
        node_id: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn mempool_init_noprof(
        pool: *mut mempool,
        min_nr: core::ffi::c_int,
        alloc_fn: Option<mempool_alloc_t>,
        free_fn: Option<mempool_free_t>,
        pool_data: *mut c_void,
    ) -> core::ffi::c_int;
    pub fn mempool_create(
        min_nr: core::ffi::c_int,
        alloc_fn: Option<mempool_alloc_t>,
        free_fn: Option<mempool_free_t>,
        pool_data: *mut c_void,
    ) -> *mut mempool;
    pub fn mempool_create_node_noprof(
        min_nr: core::ffi::c_int,
        alloc_fn: Option<mempool_alloc_t>,
        free_fn: Option<mempool_free_t>,
        pool_data: *mut c_void,
        gfp_mask: gfp_t,
        nid: core::ffi::c_int,
    ) -> *mut mempool;
    pub fn mempool_resize(pool: *mut mempool, new_min_nr: core::ffi::c_int) -> core::ffi::c_int;
    pub fn mempool_destroy(pool: *mut mempool);
    pub fn mempool_alloc_noprof(pool: *mut mempool, gfp_mask: gfp_t) -> *mut c_void;
    pub fn mempool_alloc_bulk_noprof(
        pool: *mut mempool,
        elem: *mut *mut c_void,
        count: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn mempool_alloc_preallocated(pool: *mut mempool) -> *mut c_void;
    pub fn mempool_free(element: *mut c_void, pool: *mut mempool);
    pub fn mempool_free_bulk(
        pool: *mut mempool,
        elem: *mut *mut c_void,
        count: core::ffi::c_uint,
    ) -> core::ffi::c_uint;

    pub fn mempool_alloc_slab(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void;
    pub fn mempool_free_slab(element: *mut c_void, pool_data: *mut c_void);
    pub fn mempool_kmalloc(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void;
    pub fn mempool_kfree(element: *mut c_void, pool_data: *mut c_void);
    pub fn mempool_alloc_pages(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void;
    pub fn mempool_free_pages(element: *mut c_void, pool_data: *mut c_void);
}

macro_rules! mempool_init {
    ($($args:expr),* $(,)?) => { alloc_hooks!(mempool_init_noprof($($args),*)) };
}
macro_rules! mempool_create_node {
    ($($args:expr),* $(,)?) => { alloc_hooks!(mempool_create_node_noprof($($args),*)) };
}
macro_rules! mempool_alloc {
    ($($args:expr),* $(,)?) => { alloc_hooks!(mempool_alloc_noprof($($args),*)) };
}
macro_rules! mempool_alloc_bulk {
    ($($args:expr),* $(,)?) => { alloc_hooks!(mempool_alloc_bulk_noprof($($args),*)) };
}

macro_rules! mempool_create {
    ($min_nr:expr, $alloc_fn:expr, $free_fn:expr, $pool_data:expr) => {
        mempool_create_node!($min_nr, $alloc_fn, $free_fn, $pool_data, GFP_KERNEL, NUMA_NO_NODE)
    };
}
macro_rules! mempool_init_slab_pool {
    ($pool:expr, $min_nr:expr, $kc:expr) => {
        mempool_init!($pool, $min_nr, mempool_alloc_slab, mempool_free_slab, $kc as *mut c_void)
    };
}
macro_rules! mempool_create_slab_pool {
    ($min_nr:expr, $kc:expr) => {
        mempool_create!($min_nr, mempool_alloc_slab, mempool_free_slab, $kc as *mut c_void)
    };
}
macro_rules! mempool_init_kmalloc_pool {
    ($pool:expr, $min_nr:expr, $size:expr) => {
        mempool_init!($pool, $min_nr, mempool_kmalloc, mempool_kfree, ($size as usize) as *mut c_void)
    };
}
macro_rules! mempool_create_kmalloc_pool {
    ($min_nr:expr, $size:expr) => {
        mempool_create!($min_nr, mempool_kmalloc, mempool_kfree, ($size as usize) as *mut c_void)
    };
}
macro_rules! mempool_init_page_pool {
    ($pool:expr, $min_nr:expr, $order:expr) => {
        mempool_init!($pool, $min_nr, mempool_alloc_pages, mempool_free_pages, ($order as isize) as *mut c_void)
    };
}
macro_rules! mempool_create_page_pool {
    ($min_nr:expr, $order:expr) => {
        mempool_create!($min_nr, mempool_alloc_pages, mempool_free_pages, ($order as isize) as *mut c_void)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
