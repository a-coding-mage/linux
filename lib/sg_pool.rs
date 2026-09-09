// SPDX-License-Identifier: GPL-2.0-only

// Declarations supplied by the surrounding kernel build.
use core::ffi::{c_char, c_int, c_uint, c_void};

type SizeT = usize;
type GfpT = c_uint;

#[repr(C)]
pub struct Scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SgTable {
    pub sgl: *mut Scatterlist,
    pub nents: c_uint,
    pub orig_nents: c_uint,
}

#[repr(C)]
pub struct KmemCache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mempool {
    _private: [u8; 0],
}

type MempoolT = Mempool;

extern "C" {
    static mut SG_CHUNK_SIZE: c_uint;

    fn get_count_order(n: c_uint) -> c_uint;
    fn mempool_free(element: *mut c_void, pool: *mut MempoolT);
    fn mempool_alloc(pool: *mut MempoolT, gfp_mask: GfpT) -> *mut c_void;
    fn sg_init_table(sgl: *mut Scatterlist, nents: c_uint);
    fn __sg_free_table(
        table: *mut SgTable,
        max_ents: c_uint,
        first_chunk: c_uint,
        free_fn: unsafe extern "C" fn(*mut Scatterlist, c_uint),
        orig_nents: c_uint,
    );
    fn __sg_alloc_table(
        table: *mut SgTable,
        nents: c_int,
        max_ents: c_uint,
        first_chunk: *mut Scatterlist,
        nents_first_chunk: c_uint,
        gfp_mask: GfpT,
        alloc_fn: unsafe extern "C" fn(c_uint, GfpT) -> *mut Scatterlist,
    ) -> c_int;
    fn kmem_cache_create(
        name: *const c_char,
        size: SizeT,
        align: SizeT,
        flags: c_uint,
        ctor: *mut c_void,
    ) -> *mut KmemCache;
    fn kmem_cache_destroy(cache: *mut KmemCache);
    fn mempool_create_slab_pool(min_nr: c_int, cache: *mut KmemCache) -> *mut MempoolT;
    fn mempool_destroy(pool: *mut MempoolT);
    fn printk(fmt: *const c_char, ...);
}

const SG_MEMPOOL_SIZE: c_int = 2;
const GFP_ATOMIC: GfpT = 0;
const SLAB_HWCACHE_ALIGN: c_uint = 0;
const KERN_ERR: &[u8] = b"<3>\0";

#[repr(C)]
struct SgPool {
    size: SizeT,
    name: *mut c_char,
    slab: *mut KmemCache,
    pool: *mut MempoolT,
}

static mut SG_POOLS: [SgPool; 3] = [
    SgPool { size: 8, name: b"sgpool-8\0" as *const u8 as *mut c_char, slab: core::ptr::null_mut(), pool: core::ptr::null_mut() },
    SgPool { size: 16, name: b"sgpool-16\0" as *const u8 as *mut c_char, slab: core::ptr::null_mut(), pool: core::ptr::null_mut() },
    SgPool { size: 0, name: core::ptr::null_mut(), slab: core::ptr::null_mut(), pool: core::ptr::null_mut() },
];

unsafe fn sg_pool_index(nents: u16) -> c_uint {
    debug_assert!((nents as c_uint) <= SG_CHUNK_SIZE);
    if nents <= 8 {
        0
    } else {
        get_count_order(nents as c_uint).wrapping_sub(3)
    }
}

unsafe extern "C" fn sg_pool_free(sgl: *mut Scatterlist, nents: c_uint) {
    let sgp = SG_POOLS.as_mut_ptr().add(sg_pool_index(nents as u16) as usize);
    mempool_free(sgl as *mut c_void, (*sgp).pool);
}

unsafe extern "C" fn sg_pool_alloc(nents: c_uint, gfp_mask: GfpT) -> *mut Scatterlist {
    let sgp = SG_POOLS.as_mut_ptr().add(sg_pool_index(nents as u16) as usize);
    mempool_alloc((*sgp).pool, gfp_mask) as *mut Scatterlist
}

pub unsafe extern "C" fn sg_free_table_chained(table: *mut SgTable, mut nents_first_chunk: c_uint) {
    if (*table).orig_nents <= nents_first_chunk {
        return;
    }
    if nents_first_chunk == 1 {
        nents_first_chunk = 0;
    }
    __sg_free_table(table, SG_CHUNK_SIZE, nents_first_chunk, sg_pool_free, (*table).orig_nents);
}

pub unsafe extern "C" fn sg_alloc_table_chained(
    table: *mut SgTable,
    nents: c_int,
    mut first_chunk: *mut Scatterlist,
    mut nents_first_chunk: c_uint,
) -> c_int {
    debug_assert!(nents != 0);
    if !first_chunk.is_null() && nents_first_chunk != 0 && nents <= nents_first_chunk as c_int {
        (*table).nents = nents as c_uint;
        (*table).orig_nents = nents as c_uint;
        sg_init_table((*table).sgl, nents as c_uint);
        return 0;
    }
    if nents_first_chunk <= 1 {
        first_chunk = core::ptr::null_mut();
        nents_first_chunk = 0;
    }
    let ret = __sg_alloc_table(table, nents, SG_CHUNK_SIZE, first_chunk, nents_first_chunk, GFP_ATOMIC, sg_pool_alloc);
    if ret != 0 {
        sg_free_table_chained(table, nents_first_chunk);
    }
    ret
}

unsafe fn sg_pool_init() -> c_int {
    let mut i = 0usize;
    while i < SG_POOLS.len() {
        let sgp = SG_POOLS.as_mut_ptr().add(i);
        let size = (*sgp).size * core::mem::size_of::<Scatterlist>();
        (*sgp).slab = kmem_cache_create((*sgp).name, size, 0, SLAB_HWCACHE_ALIGN, core::ptr::null_mut());
        if (*sgp).slab.is_null() {
            printk(b"<3>SG_POOL: can't init sg slab %s\n\0".as_ptr() as *const c_char, (*sgp).name);
            break;
        }
        (*sgp).pool = mempool_create_slab_pool(SG_MEMPOOL_SIZE, (*sgp).slab);
        if (*sgp).pool.is_null() {
            printk(b"<3>SG_POOL: can't init sg mempool %s\n\0".as_ptr() as *const c_char, (*sgp).name);
            break;
        }
        i += 1;
    }
    if i == SG_POOLS.len() {
        return 0;
    }
    i = 0;
    while i < SG_POOLS.len() {
        let sgp = SG_POOLS.as_mut_ptr().add(i);
        mempool_destroy((*sgp).pool);
        kmem_cache_destroy((*sgp).slab);
        i += 1;
    }
    -12
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
