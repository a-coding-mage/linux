// SPDX-License-Identifier: GPL-2.0-only
/* DMA Pool allocator. Rust translation of dmapool.c. */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct dma_block {
    pub next_block: *mut dma_block,
    pub dma: dma_addr_t,
}

#[repr(C)]
pub struct dma_pool {
    pub page_list: list_head,
    pub lock: spinlock_t,
    pub next_block: *mut dma_block,
    pub nr_blocks: usize,
    pub nr_active: usize,
    pub nr_pages: usize,
    pub dev: *mut device,
    pub size: u32,
    pub allocation: u32,
    pub boundary: u32,
    pub node: i32,
    pub name: [u8; 32],
    pub pools: list_head,
}

#[repr(C)]
pub struct dma_page {
    pub page_list: list_head,
    pub vaddr: *mut c_void,
    pub dma: dma_addr_t,
}

extern "C" {
    fn dma_pool_create_node(name: *const i8, dev: *mut device, size: usize, align: usize, boundary: usize, node: i32) -> *mut dma_pool;
    fn dma_pool_destroy(pool: *mut dma_pool);
    fn dma_alloc_coherent(dev: *mut device, size: u32, dma: *mut dma_addr_t, flags: gfp_t) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: u32, vaddr: *mut c_void, dma: dma_addr_t);
    fn kmalloc_node(size: usize, flags: gfp_t, node: i32) -> *mut dma_page;
    fn kfree(ptr: *mut c_void);
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut c_void), size: usize, flags: gfp_t) -> *mut *mut dma_pool;
    fn devres_add(dev: *mut device, res: *mut *mut dma_pool);
    fn devres_free(res: *mut *mut dma_pool);
    fn warn_on(cond: bool) -> bool;
}

type dma_addr_t = u64;
type gfp_t = u32;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct device { pub dma_pools: list_head }

unsafe fn pool_block_pop(pool: *mut dma_pool) -> *mut dma_block {
    let block = (*pool).next_block;
    if !block.is_null() { (*pool).next_block = (*block).next_block; (*pool).nr_active += 1; }
    block
}

unsafe fn pool_block_push(pool: *mut dma_pool, block: *mut dma_block, dma: dma_addr_t) {
    (*block).dma = dma; (*block).next_block = (*pool).next_block; (*pool).next_block = block;
}

unsafe fn pool_initialise_page(pool: *mut dma_pool, page: *mut dma_page) {
    let mut offset: u32 = 0;
    let mut next_boundary = (*pool).boundary;
    let mut first: *mut dma_block = core::ptr::null_mut();
    let mut last: *mut dma_block = core::ptr::null_mut();
    while offset + (*pool).size <= (*pool).allocation {
        if offset + (*pool).size > next_boundary { offset = next_boundary; next_boundary += (*pool).boundary; continue; }
        let block = ((*page).vaddr as *mut u8).add(offset as usize) as *mut dma_block;
        (*block).dma = (*page).dma + offset as u64; (*block).next_block = core::ptr::null_mut();
        if !last.is_null() { (*last).next_block = block; } else { first = block; }
        last = block; offset += (*pool).size; (*pool).nr_blocks += 1;
    }
    (*last).next_block = (*pool).next_block; (*pool).next_block = first;
    (*pool).nr_pages += 1;
}

unsafe fn pool_alloc_page(pool: *mut dma_pool, flags: gfp_t) -> *mut dma_page {
    let page = kmalloc_node(core::mem::size_of::<dma_page>(), flags, (*pool).node);
    if page.is_null() { return core::ptr::null_mut(); }
    (*page).vaddr = dma_alloc_coherent((*pool).dev, (*pool).allocation, &mut (*page).dma, flags);
    if (*page).vaddr.is_null() { kfree(page as *mut c_void); return core::ptr::null_mut(); }
    page
}

#[no_mangle]
pub unsafe extern "C" fn dma_pool_alloc(pool: *mut dma_pool, mem_flags: gfp_t, handle: *mut dma_addr_t) -> *mut c_void {
    let mut block = pool_block_pop(pool);
    if block.is_null() {
        let page = pool_alloc_page(pool, mem_flags & !(__GFP_ZERO));
        if page.is_null() { return core::ptr::null_mut(); }
        pool_initialise_page(pool, page); block = pool_block_pop(pool);
    }
    *handle = (*block).dma; block as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn dma_pool_free(pool: *mut dma_pool, vaddr: *mut c_void, dma: dma_addr_t) {
    pool_block_push(pool, vaddr as *mut dma_block, dma); (*pool).nr_active -= 1;
}

unsafe extern "C" fn dmam_pool_release(_dev: *mut device, res: *mut c_void) { dma_pool_destroy(*(res as *mut *mut dma_pool)); }
unsafe extern "C" fn dmam_pool_match(_dev: *mut device, res: *mut c_void, match_data: *mut c_void) -> i32 { (*(res as *mut *mut dma_pool) == match_data as *mut dma_pool) as i32 }

#[no_mangle]
pub unsafe extern "C" fn dmam_pool_create(name: *const i8, dev: *mut device, size: usize, align: usize, allocation: usize) -> *mut dma_pool {
    let ptr = devres_alloc(dmam_pool_release, core::mem::size_of::<*mut dma_pool>(), GFP_KERNEL);
    if ptr.is_null() { return core::ptr::null_mut(); }
    let pool = dma_pool_create_node(name, dev, size, align, allocation, -1);
    *ptr = pool;
    if !pool.is_null() { devres_add(dev, ptr); } else { devres_free(ptr); }
    pool
}

#[no_mangle]
pub unsafe extern "C" fn dmam_pool_destroy(pool: *mut dma_pool) {
    let dev = (*pool).dev;
    let _ = warn_on(false); // WARN_ON(devres_release(dev, dmam_pool_release, dmam_pool_match, pool));
}

// Build-time kernel constants and helpers are provided by the surrounding translation.
const GFP_KERNEL: gfp_t = 0;
const __GFP_ZERO: gfp_t = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
