/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const PP_FLAG_DMA_MAP: u32 = 1 << 0;
pub const PP_FLAG_DMA_SYNC_DEV: u32 = 1 << 1;
pub const PP_FLAG_SYSTEM_POOL: u32 = 1 << 2;
pub const PP_FLAG_ALLOW_UNREADABLE_NETMEM: u32 = 1 << 3;
pub const PP_FLAG_ALL: u32 = PP_FLAG_DMA_MAP
    | PP_FLAG_DMA_SYNC_DEV
    | PP_FLAG_SYSTEM_POOL
    | PP_FLAG_ALLOW_UNREADABLE_NETMEM;

/* PP_DMA_INDEX_LIMIT is XA_LIMIT(1, BIT(PP_DMA_INDEX_BITS) - 1). */

#[cfg(any())]
pub const PP_ALLOC_CACHE_REFILL: usize = 4;
#[cfg(all(not(any()), any()))]
pub const PP_ALLOC_CACHE_REFILL: usize = 16;
#[cfg(not(any()))]
pub const PP_ALLOC_CACHE_REFILL: usize = 64;
pub const PP_ALLOC_CACHE_SIZE: usize = PP_ALLOC_CACHE_REFILL * 2;

#[repr(C)]
pub struct pp_alloc_cache {
    pub count: u32,
    pub cache: [netmem_ref; PP_ALLOC_CACHE_SIZE],
}

#[repr(C)]
pub struct page_pool_params_fast {
    pub order: core::ffi::c_uint,
    pub pool_size: core::ffi::c_uint,
    pub nid: core::ffi::c_int,
    pub dev: *mut device,
    pub napi: *mut napi_struct,
    pub dma_dir: dma_data_direction,
    pub max_len: core::ffi::c_uint,
    pub offset: core::ffi::c_uint,
}

#[repr(C)]
pub struct page_pool_params_slow {
    pub netdev: *mut net_device,
    pub queue_idx: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
    pub init_callback: Option<unsafe extern "C" fn(netmem_ref, *mut core::ffi::c_void)>,
    pub init_arg: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct page_pool_params {
    pub fast: page_pool_params_fast,
    pub slow: page_pool_params_slow,
}

#[cfg(CONFIG_PAGE_POOL_STATS)]
#[repr(C)]
pub struct page_pool_alloc_stats {
    pub fast: u64,
    pub slow: u64,
    pub slow_high_order: u64,
    pub empty: u64,
    pub refill: u64,
    pub waive: u64,
}

#[cfg(CONFIG_PAGE_POOL_STATS)]
#[repr(C)]
pub struct page_pool_recycle_stats {
    pub cached: u64,
    pub cache_full: u64,
    pub ring: u64,
    pub ring_full: u64,
    pub released_refcnt: u64,
}

#[cfg(CONFIG_PAGE_POOL_STATS)]
#[repr(C)]
pub struct page_pool_stats {
    pub alloc_stats: page_pool_alloc_stats,
    pub recycle_stats: page_pool_recycle_stats,
}

pub const PAGE_POOL_FRAG_GROUP_ALIGN: usize = 4 * core::mem::size_of::<core::ffi::c_long>();

#[repr(C)]
pub struct pp_memory_provider_params {
    pub mp_priv: *mut core::ffi::c_void,
    pub mp_ops: *const memory_provider_ops,
    pub rx_page_size: u32,
}

#[repr(C)]
pub struct page_pool {
    pub p: page_pool_params_fast,
    pub cpuid: core::ffi::c_int,
    pub pages_state_hold_cnt: u32,
    pub has_init_callback: bool,
    pub dma_map: bool,
    pub dma_sync: bool,
    pub dma_sync_for_cpu: bool,
    #[cfg(CONFIG_PAGE_POOL_STATS)]
    pub system: bool,
    pub frag_users: core::ffi::c_long,
    pub frag_page: netmem_ref,
    pub frag_offset: core::ffi::c_uint,
    pub release_dw: delayed_work,
    pub disconnect: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub defer_start: core::ffi::c_ulong,
    pub defer_warn: core::ffi::c_ulong,
    #[cfg(CONFIG_PAGE_POOL_STATS)]
    pub alloc_stats: page_pool_alloc_stats,
    pub xdp_mem_id: u32,
    pub alloc: pp_alloc_cache,
    pub ring: ptr_ring,
    pub mp_priv: *mut core::ffi::c_void,
    pub mp_ops: *const memory_provider_ops,
    pub dma_mapped: xarray,
    #[cfg(CONFIG_PAGE_POOL_STATS)]
    pub recycle_stats: *mut page_pool_recycle_stats,
    pub pages_state_release_cnt: atomic_t,
    pub user_cnt: refcount_t,
    pub destroy_cnt: u64,
    pub slow: page_pool_params_slow,
    pub user: page_pool_user,
}

#[repr(C)]
pub struct page_pool_user {
    pub list: hlist_node,
    pub detach_time: ktime_t,
    pub id: u32,
}

pub struct memory_provider_ops;

#[repr(C)]
pub struct xdp_mem_info;

extern "C" {
    pub fn page_pool_alloc_pages(pool: *mut page_pool, gfp: gfp_t) -> *mut page;
    pub fn page_pool_alloc_netmems(pool: *mut page_pool, gfp: gfp_t) -> netmem_ref;
    pub fn page_pool_alloc_frag(pool: *mut page_pool, offset: *mut core::ffi::c_uint, size: core::ffi::c_uint, gfp: gfp_t) -> *mut page;
    pub fn page_pool_alloc_frag_netmem(pool: *mut page_pool, offset: *mut core::ffi::c_uint, size: core::ffi::c_uint, gfp: gfp_t) -> netmem_ref;
    pub fn page_pool_create(params: *const page_pool_params) -> *mut page_pool;
    pub fn page_pool_create_percpu(params: *const page_pool_params, cpuid: core::ffi::c_int) -> *mut page_pool;

    #[cfg(CONFIG_PAGE_POOL)]
    pub fn page_pool_enable_direct_recycling(pool: *mut page_pool, napi: *mut napi_struct);
    #[cfg(CONFIG_PAGE_POOL)]
    pub fn page_pool_disable_direct_recycling(pool: *mut page_pool);
    #[cfg(CONFIG_PAGE_POOL)]
    pub fn page_pool_destroy(pool: *mut page_pool);
    #[cfg(CONFIG_PAGE_POOL)]
    pub fn page_pool_use_xdp_mem(pool: *mut page_pool, disconnect: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, mem: *const xdp_mem_info);
    #[cfg(CONFIG_PAGE_POOL)]
    pub fn page_pool_put_netmem_bulk(data: *mut netmem_ref, count: u32);

    pub fn page_pool_put_unrefed_netmem(pool: *mut page_pool, netmem: netmem_ref, dma_sync_size: core::ffi::c_uint, allow_direct: bool);
    pub fn page_pool_put_unrefed_page(pool: *mut page_pool, page: *mut page, dma_sync_size: core::ffi::c_uint, allow_direct: bool);
    pub fn page_pool_update_nid(pool: *mut page_pool, new_nid: core::ffi::c_int);
}

#[cfg(not(CONFIG_PAGE_POOL))]
pub unsafe fn page_pool_destroy(_pool: *mut page_pool) {}
#[cfg(not(CONFIG_PAGE_POOL))]
pub unsafe fn page_pool_use_xdp_mem(_pool: *mut page_pool, _disconnect: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, _mem: *const xdp_mem_info) {}
#[cfg(not(CONFIG_PAGE_POOL))]
pub unsafe fn page_pool_put_netmem_bulk(_data: *mut netmem_ref, _count: u32) {}

pub fn is_page_pool_compiled_in() -> bool {
    cfg!(CONFIG_PAGE_POOL)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
