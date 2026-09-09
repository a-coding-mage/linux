/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const SWIOTLB_VERBOSE: u32 = 1 << 0; /* verbose initialization */
pub const SWIOTLB_ANY: u32 = 1 << 1; /* allow any memory for the buffer */

/* Maximum allowable number of contiguous slabs to map; must be a power of 2. */
pub const IO_TLB_SEGSIZE: usize = 128;
/* log of the size of each IO TLB slab. */
pub const IO_TLB_SHIFT: usize = 11;
pub const IO_TLB_SIZE: usize = 1 << IO_TLB_SHIFT;

/* compile-time default; CONFIG_SWIOTLB_DEFAULT_SIZE_MB may override this. */
#[cfg(CONFIG_SWIOTLB)]
pub const IO_TLB_DEFAULT_SIZE: usize = (CONFIG_SWIOTLB_DEFAULT_SIZE_MB as usize) << 20;
#[cfg(not(CONFIG_SWIOTLB))]
pub const IO_TLB_DEFAULT_SIZE: usize = 64usize << 20;

extern "C" {
    pub fn swiotlb_size_or_default() -> usize;
    pub fn swiotlb_init_remap(
        addressing_limit: bool,
        flags: u32,
        remap: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> i32>,
    );
    pub fn swiotlb_init_late(size: usize, gfp_mask: gfp_t,
        remap: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn swiotlb_update_mem_attributes();
}

#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn swiotlb_init(_: bool, _: u32) {}
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn swiotlb_exit() {}
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn swiotlb_dev_init(_: *mut device) {}
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn swiotlb_max_mapping_size(_: *mut device) -> usize { usize::MAX }
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn is_swiotlb_allocated() -> bool { false }
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn is_swiotlb_active(_: *mut device) -> bool { false }
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn swiotlb_adjust_size(_: usize) {}
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn default_swiotlb_base() -> phys_addr_t { 0 }
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn default_swiotlb_limit() -> phys_addr_t { 0 }

#[cfg(CONFIG_SWIOTLB)]
#[repr(C)]
pub struct io_tlb_pool {
    pub start: phys_addr_t,
    pub end: phys_addr_t,
    pub vaddr: *mut core::ffi::c_void,
    pub nslabs: usize,
    pub late_alloc: bool,
    pub nareas: u32,
    pub area_nslabs: u32,
    pub areas: *mut io_tlb_area,
    pub slots: *mut io_tlb_slot,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub node: list_head,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub dyn_free: rcu_work,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub transient: bool,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub cc_shared: bool,
}

#[cfg(CONFIG_SWIOTLB)]
#[repr(C)]
pub struct io_tlb_mem {
    pub defpool: io_tlb_pool,
    pub nslabs: usize,
    pub debugfs: *mut dentry,
    pub force_bounce: bool,
    pub for_alloc: bool,
    pub cc_shared: bool,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub can_grow: bool,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub phys_limit: u64,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub lock: spinlock_t,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub pools: list_head,
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    pub dyn_alloc: work_struct,
    #[cfg(CONFIG_DEBUG_FS)]
    pub total_used: atomic_long_t,
    #[cfg(CONFIG_DEBUG_FS)]
    pub used_hiwater: atomic_long_t,
    #[cfg(CONFIG_DEBUG_FS)]
    pub transient_nslabs: atomic_long_t,
}

extern "C" {
    pub fn __swiotlb_find_pool(dev: *mut device, paddr: phys_addr_t) -> *mut io_tlb_pool;
}

#[cfg(CONFIG_SWIOTLB)]
pub unsafe fn swiotlb_find_pool(dev: *mut device, paddr: phys_addr_t) -> *mut io_tlb_pool {
    let mem = (*dev).dma_io_tlb_mem;
    if mem.is_null() { return core::ptr::null_mut(); }
    #[cfg(CONFIG_SWIOTLB_DYNAMIC)]
    {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        if core::ptr::read_volatile(&(*dev).dma_uses_io_tlb) { return __swiotlb_find_pool(dev, paddr); }
    }
    #[cfg(not(CONFIG_SWIOTLB_DYNAMIC))]
    if paddr >= (*mem).defpool.start && paddr < (*mem).defpool.end { return &mut (*mem).defpool; }
    core::ptr::null_mut()
}

#[cfg(CONFIG_SWIOTLB)]
pub unsafe fn is_swiotlb_force_bounce(dev: *mut device) -> bool {
    let mem = (*dev).dma_io_tlb_mem; !mem.is_null() && (*mem).force_bounce
}

#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn swiotlb_find_pool(_: *mut device, _: phys_addr_t) -> *mut io_tlb_pool { core::ptr::null_mut() }
#[cfg(not(CONFIG_SWIOTLB))]
pub unsafe fn is_swiotlb_force_bounce(_: *mut device) -> bool { false }

extern "C" {
    pub fn swiotlb_init(addressing_limited: bool, flags: u32);
    pub fn swiotlb_exit();
    pub fn swiotlb_dev_init(dev: *mut device);
    pub fn swiotlb_max_mapping_size(dev: *mut device) -> usize;
    pub fn is_swiotlb_allocated() -> bool;
    pub fn is_swiotlb_active(dev: *mut device) -> bool;
    pub fn swiotlb_adjust_size(size: usize);
    pub fn default_swiotlb_base() -> phys_addr_t;
    pub fn default_swiotlb_limit() -> phys_addr_t;
    pub fn swiotlb_tbl_map_single(hwdev: *mut device, phys: phys_addr_t, mapping_size: usize,
        alloc_aligned_mask: u32, dir: dma_data_direction, attrs: *mut usize) -> phys_addr_t;
    pub fn swiotlb_map(dev: *mut device, phys: phys_addr_t, size: usize,
        dir: dma_data_direction, attrs: usize) -> dma_addr_t;
    pub fn __swiotlb_tbl_unmap_single(hwdev: *mut device, tlb_addr: phys_addr_t,
        mapping_size: usize, dir: dma_data_direction, attrs: usize, pool: *mut io_tlb_pool);
    pub fn __swiotlb_sync_single_for_device(dev: *mut device, tlb_addr: phys_addr_t,
        size: usize, dir: dma_data_direction, pool: *mut io_tlb_pool);
    pub fn __swiotlb_sync_single_for_cpu(dev: *mut device, tlb_addr: phys_addr_t,
        size: usize, dir: dma_data_direction, pool: *mut io_tlb_pool);
    pub fn swiotlb_print_info();
}

pub unsafe fn swiotlb_tbl_unmap_single(dev: *mut device, addr: phys_addr_t, size: usize,
    dir: dma_data_direction, attrs: usize) {
    let pool = swiotlb_find_pool(dev, addr);
    if !pool.is_null() { __swiotlb_tbl_unmap_single(dev, addr, size, dir, attrs, pool); }
}
pub unsafe fn swiotlb_sync_single_for_device(dev: *mut device, addr: phys_addr_t, size: usize,
    dir: dma_data_direction) { let pool = swiotlb_find_pool(dev, addr); if !pool.is_null() { __swiotlb_sync_single_for_device(dev, addr, size, dir, pool); } }
pub unsafe fn swiotlb_sync_single_for_cpu(dev: *mut device, addr: phys_addr_t, size: usize,
    dir: dma_data_direction) { let pool = swiotlb_find_pool(dev, addr); if !pool.is_null() { __swiotlb_sync_single_for_cpu(dev, addr, size, dir, pool); } }

#[cfg(CONFIG_DMA_RESTRICTED_POOL)]
extern "C" {
    pub fn swiotlb_alloc(dev: *mut device, size: usize, attrs: usize) -> *mut page;
    pub fn swiotlb_free(dev: *mut device, page: *mut page, size: usize) -> bool;
    pub fn swiotlb_free_from_pool(dev: *mut device, tlb_addr: phys_addr_t, pool: *mut io_tlb_pool);
}
#[cfg(CONFIG_DMA_RESTRICTED_POOL)]
pub unsafe fn is_swiotlb_for_alloc(dev: *mut device) -> bool { (*(*dev).dma_io_tlb_mem).for_alloc }
#[cfg(not(CONFIG_DMA_RESTRICTED_POOL))]
pub unsafe fn swiotlb_alloc(_: *mut device, _: usize, _: usize) -> *mut page { core::ptr::null_mut() }
#[cfg(not(CONFIG_DMA_RESTRICTED_POOL))]
pub unsafe fn swiotlb_free(_: *mut device, _: *mut page, _: usize) -> bool { false }
#[cfg(not(CONFIG_DMA_RESTRICTED_POOL))]
pub unsafe fn swiotlb_free_from_pool(_: *mut device, _: phys_addr_t, _: *mut io_tlb_pool) {}
#[cfg(not(CONFIG_DMA_RESTRICTED_POOL))]
pub unsafe fn is_swiotlb_for_alloc(_: *mut device) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
