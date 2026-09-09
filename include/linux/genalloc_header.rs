/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Basic general purpose allocator for managing special purpose
 * memory, for example, memory that is not managed by the regular
 * kmalloc/kfree interface. Uses for this includes on-device special
 * memory, uncached memory etc.
 *
 * It is safe to use the allocator in NMI handlers and other special
 * unblockable contexts that could otherwise deadlock on locks. This
 * is implemented by using atomic operations and retries on any
 * conflicts. The disadvantage is that there may be livelocks in
 * extreme cases. For better scalability, one allocator can be used
 * for each CPU.
 *
 * The lockless operation only works if there is enough memory
 * available. If new memory is added to the pool a lock has to be
 * still taken. So any user relying on locklessness has to ensure
 * that sufficient memory is preallocated.
 *
 * The basic atomic operation of this allocator is cmpxchg on long.
 * On architectures that don't have NMI-safe cmpxchg implementation,
 * the allocator can NOT be used in NMI handler. So code uses the
 * allocator in NMI handler should depend on
 * CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct gen_pool {
    pub lock: spinlock_t,
    pub chunks: list_head,
    pub min_alloc_order: ::core::ffi::c_int,
    pub algo: genpool_algo_t,
    pub data: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct gen_pool_chunk {
    pub next_chunk: list_head,
    pub avail: atomic_long_t,
    pub phys_addr: phys_addr_t,
    pub owner: *mut ::core::ffi::c_void,
    pub start_addr: ::core::primitive::usize,
    pub end_addr: ::core::primitive::usize,
    pub bits: [::core::primitive::usize; 0],
}

#[repr(C)]
pub struct genpool_data_align {
    pub align: ::core::ffi::c_int,
}

#[repr(C)]
pub struct genpool_data_fixed {
    pub offset: ::core::primitive::usize,
}

pub type genpool_algo_t = unsafe extern "C" fn(
    map: *mut ::core::primitive::usize,
    size: ::core::primitive::usize,
    start: ::core::primitive::usize,
    nr: ::core::ffi::c_uint,
    data: *mut ::core::ffi::c_void,
    pool: *mut gen_pool,
    start_addr: ::core::primitive::usize,
) -> ::core::primitive::usize;

unsafe extern "C" {
    pub fn gen_pool_create(
        min_alloc_order: ::core::ffi::c_int,
        nid: ::core::ffi::c_int,
    ) -> *mut gen_pool;
    pub fn gen_pool_virt_to_phys(pool: *mut gen_pool, addr: ::core::primitive::usize) -> phys_addr_t;
    pub fn gen_pool_add_owner(
        pool: *mut gen_pool,
        addr: ::core::primitive::usize,
        phys: phys_addr_t,
        size: ::core::primitive::usize,
        nid: ::core::ffi::c_int,
        owner: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn gen_pool_destroy(pool: *mut gen_pool);
    pub fn gen_pool_alloc_algo_owner(
        pool: *mut gen_pool,
        size: ::core::primitive::usize,
        algo: genpool_algo_t,
        data: *mut ::core::ffi::c_void,
        owner: *mut *mut ::core::ffi::c_void,
    ) -> ::core::primitive::usize;
    pub fn gen_pool_dma_alloc(pool: *mut gen_pool, size: ::core::primitive::usize, dma: *mut dma_addr_t) -> *mut ::core::ffi::c_void;
    pub fn gen_pool_dma_alloc_algo(pool: *mut gen_pool, size: ::core::primitive::usize, dma: *mut dma_addr_t, algo: genpool_algo_t, data: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn gen_pool_dma_alloc_align(pool: *mut gen_pool, size: ::core::primitive::usize, dma: *mut dma_addr_t, align: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    pub fn gen_pool_dma_zalloc(pool: *mut gen_pool, size: ::core::primitive::usize, dma: *mut dma_addr_t) -> *mut ::core::ffi::c_void;
    pub fn gen_pool_dma_zalloc_algo(pool: *mut gen_pool, size: ::core::primitive::usize, dma: *mut dma_addr_t, algo: genpool_algo_t, data: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn gen_pool_dma_zalloc_align(pool: *mut gen_pool, size: ::core::primitive::usize, dma: *mut dma_addr_t, align: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    pub fn gen_pool_free_owner(pool: *mut gen_pool, addr: ::core::primitive::usize, size: ::core::primitive::usize, owner: *mut *mut ::core::ffi::c_void);
    pub fn gen_pool_for_each_chunk(pool: *mut gen_pool, callback: unsafe extern "C" fn(*mut gen_pool, *mut gen_pool_chunk, *mut ::core::ffi::c_void), data: *mut ::core::ffi::c_void);
    pub fn gen_pool_avail(pool: *mut gen_pool) -> ::core::primitive::usize;
    pub fn gen_pool_size(pool: *mut gen_pool) -> ::core::primitive::usize;
    pub fn gen_pool_set_algo(pool: *mut gen_pool, algo: genpool_algo_t, data: *mut ::core::ffi::c_void);
    pub fn gen_pool_first_fit(map: *mut ::core::primitive::usize, size: ::core::primitive::usize, start: ::core::primitive::usize, nr: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, pool: *mut gen_pool, start_addr: ::core::primitive::usize) -> ::core::primitive::usize;
    pub fn gen_pool_fixed_alloc(map: *mut ::core::primitive::usize, size: ::core::primitive::usize, start: ::core::primitive::usize, nr: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, pool: *mut gen_pool, start_addr: ::core::primitive::usize) -> ::core::primitive::usize;
    pub fn gen_pool_first_fit_align(map: *mut ::core::primitive::usize, size: ::core::primitive::usize, start: ::core::primitive::usize, nr: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, pool: *mut gen_pool, start_addr: ::core::primitive::usize) -> ::core::primitive::usize;
    pub fn gen_pool_first_fit_order_align(map: *mut ::core::primitive::usize, size: ::core::primitive::usize, start: ::core::primitive::usize, nr: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, pool: *mut gen_pool, start_addr: ::core::primitive::usize) -> ::core::primitive::usize;
    pub fn gen_pool_best_fit(map: *mut ::core::primitive::usize, size: ::core::primitive::usize, start: ::core::primitive::usize, nr: ::core::ffi::c_uint, data: *mut ::core::ffi::c_void, pool: *mut gen_pool, start_addr: ::core::primitive::usize) -> ::core::primitive::usize;
    pub fn devm_gen_pool_create(dev: *mut device, min_alloc_order: ::core::ffi::c_int, nid: ::core::ffi::c_int, name: *const ::core::ffi::c_char) -> *mut gen_pool;
    pub fn gen_pool_get(dev: *mut device, name: *const ::core::ffi::c_char) -> *mut gen_pool;
    pub fn gen_pool_has_addr(pool: *mut gen_pool, start: ::core::primitive::usize, size: ::core::primitive::usize) -> bool;
}

pub unsafe fn gen_pool_add_virt(pool: *mut gen_pool, addr: usize, phys: phys_addr_t, size: usize, nid: i32) -> i32 {
    gen_pool_add_owner(pool, addr, phys, size, nid, core::ptr::null_mut())
}

pub unsafe fn gen_pool_add(pool: *mut gen_pool, addr: usize, size: usize, nid: i32) -> i32 {
    gen_pool_add_virt(pool, addr, (-1i32) as phys_addr_t, size, nid)
}

pub unsafe fn gen_pool_alloc_owner(pool: *mut gen_pool, size: usize, owner: *mut *mut core::ffi::c_void) -> usize {
    gen_pool_alloc_algo_owner(pool, size, (*pool).algo, (*pool).data, owner)
}

pub unsafe fn gen_pool_alloc_algo(pool: *mut gen_pool, size: usize, algo: genpool_algo_t, data: *mut core::ffi::c_void) -> usize {
    gen_pool_alloc_algo_owner(pool, size, algo, data, core::ptr::null_mut())
}

pub unsafe fn gen_pool_alloc(pool: *mut gen_pool, size: usize) -> usize {
    gen_pool_alloc_algo(pool, size, (*pool).algo, (*pool).data)
}

pub unsafe fn gen_pool_free(pool: *mut gen_pool, addr: usize, size: usize) {
    gen_pool_free_owner(pool, addr, size, core::ptr::null_mut())
}

// CONFIG_OF controls whether the device-tree helper is supplied by the build.
#[cfg(CONFIG_OF)]
unsafe extern "C" {
    pub fn of_gen_pool_get(np: *mut device_node, propname: *const ::core::ffi::c_char, index: ::core::ffi::c_int) -> *mut gen_pool;
}

#[cfg(not(CONFIG_OF))]
pub unsafe fn of_gen_pool_get(_np: *mut device_node, _propname: *const ::core::ffi::c_char, _index: ::core::ffi::c_int) -> *mut gen_pool {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
