/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of net/page_pool/helpers.h. */

#[cfg(feature = "CONFIG_PAGE_POOL_STATS")]
extern "C" {
    pub fn page_pool_ethtool_stats_get_count() -> i32;
    pub fn page_pool_ethtool_stats_get_strings(data: *mut u8) -> *mut u8;
    pub fn page_pool_ethtool_stats_get(data: *mut u64, stats: *const core::ffi::c_void) -> *mut u64;
    pub fn page_pool_get_stats(pool: *const page_pool, stats: *mut page_pool_stats);
}

#[cfg(not(feature = "CONFIG_PAGE_POOL_STATS"))]
#[inline]
pub fn page_pool_ethtool_stats_get_count() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PAGE_POOL_STATS"))]
#[inline]
pub fn page_pool_ethtool_stats_get_strings(data: *mut u8) -> *mut u8 { data }
#[cfg(not(feature = "CONFIG_PAGE_POOL_STATS"))]
#[inline]
pub fn page_pool_ethtool_stats_get(data: *mut u64, _stats: *const core::ffi::c_void) -> *mut u64 { data }

#[inline]
pub unsafe fn page_pool_dev_alloc_pages(pool: *mut page_pool) -> *mut page {
    page_pool_alloc_pages(pool, GFP_ATOMIC | __GFP_NOWARN)
}

#[inline]
pub unsafe fn page_pool_dev_alloc_frag(pool: *mut page_pool, offset: *mut u32, size: u32) -> *mut page {
    page_pool_alloc_frag(pool, offset, size, GFP_ATOMIC | __GFP_NOWARN)
}

#[inline]
pub unsafe fn page_pool_alloc_netmem(pool: *mut page_pool, offset: *mut u32, size: *mut u32, gfp: gfp_t) -> netmem_ref {
    let max_size = PAGE_SIZE << (*pool).p.order;
    if (*size << 1) > max_size {
        *size = max_size;
        *offset = 0;
        return page_pool_alloc_netmems(pool, gfp);
    }
    let netmem = page_pool_alloc_frag_netmem(pool, offset, *size, gfp);
    if netmem == 0 { return 0; }
    if (*pool).frag_offset + *size > max_size {
        *size = max_size - *offset;
        (*pool).frag_offset = max_size;
    }
    netmem
}

#[inline]
pub unsafe fn page_pool_dev_alloc_netmem(pool: *mut page_pool, offset: *mut u32, size: *mut u32) -> netmem_ref {
    page_pool_alloc_netmem(pool, offset, size, GFP_ATOMIC | __GFP_NOWARN)
}
#[inline]
pub unsafe fn page_pool_dev_alloc_netmems(pool: *mut page_pool) -> netmem_ref {
    page_pool_alloc_netmems(pool, GFP_ATOMIC | __GFP_NOWARN)
}
#[inline]
pub unsafe fn page_pool_alloc(pool: *mut page_pool, offset: *mut u32, size: *mut u32, gfp: gfp_t) -> *mut page {
    netmem_to_page(page_pool_alloc_netmem(pool, offset, size, gfp))
}
#[inline]
pub unsafe fn page_pool_dev_alloc(pool: *mut page_pool, offset: *mut u32, size: *mut u32) -> *mut page {
    page_pool_alloc(pool, offset, size, GFP_ATOMIC | __GFP_NOWARN)
}
#[inline]
pub unsafe fn page_pool_alloc_va(pool: *mut page_pool, size: *mut u32, gfp: gfp_t) -> *mut core::ffi::c_void {
    let mut offset = 0u32;
    let page = page_pool_alloc(pool, &mut offset, size, gfp & !__GFP_HIGHMEM);
    if page.is_null() { return core::ptr::null_mut(); }
    (page_address(page) as *mut u8).add(offset as usize) as *mut core::ffi::c_void
}
#[inline]
pub unsafe fn page_pool_dev_alloc_va(pool: *mut page_pool, size: *mut u32) -> *mut core::ffi::c_void {
    page_pool_alloc_va(pool, size, GFP_ATOMIC | __GFP_NOWARN)
}
#[inline]
pub unsafe fn page_pool_get_dma_dir(pool: *const page_pool) -> dma_data_direction { (*pool).p.dma_dir }
#[inline]
pub unsafe fn page_pool_fragment_netmem(netmem: netmem_ref, nr: i64) {
    atomic_long_set(netmem_get_pp_ref_count_ref(netmem), nr);
}
#[inline]
pub unsafe fn page_pool_fragment_page(page: *mut page, nr: i64) {
    page_pool_fragment_netmem(page_to_netmem(page), nr);
}
#[inline]
pub unsafe fn page_pool_unref_netmem(netmem: netmem_ref, nr: i64) -> i64 {
    let refs = netmem_get_pp_ref_count_ref(netmem);
    if atomic_long_read(refs) == nr {
        /* BUILD_BUG_ON(__builtin_constant_p(nr) && nr != 1); */
        /* The non-constant draining case resets the count to one. */
        atomic_long_set(refs, 1);
        return 0;
    }
    let ret = atomic_long_sub_return(nr, refs);
    WARN_ON(ret < 0);
    if ret == 0 { atomic_long_set(refs, 1); }
    ret
}
#[inline]
pub unsafe fn page_pool_unref_page(page: *mut page, nr: i64) -> i64 { page_pool_unref_netmem(page_to_netmem(page), nr) }
#[inline]
pub unsafe fn page_pool_ref_netmem(netmem: netmem_ref) { atomic_long_inc(netmem_get_pp_ref_count_ref(netmem)); }
#[inline]
pub unsafe fn page_pool_ref_page(page: *mut page) { page_pool_ref_netmem(page_to_netmem(page)); }
#[inline]
pub unsafe fn page_pool_unref_and_test(netmem: netmem_ref) -> bool { page_pool_unref_netmem(netmem, 1) == 0 }

#[inline]
pub unsafe fn page_pool_put_netmem(pool: *mut page_pool, netmem: netmem_ref, dma_sync_size: u32, allow_direct: bool) {
    #[cfg(feature = "CONFIG_PAGE_POOL")]
    {
        if !page_pool_unref_and_test(netmem) { return; }
        page_pool_put_unrefed_netmem(pool, netmem, dma_sync_size, allow_direct);
    }
}
#[inline]
pub unsafe fn page_pool_put_page(pool: *mut page_pool, page: *mut page, dma_sync_size: u32, allow_direct: bool) {
    page_pool_put_netmem(pool, page_to_netmem(page), dma_sync_size, allow_direct);
}
#[inline]
pub unsafe fn page_pool_put_full_netmem(pool: *mut page_pool, netmem: netmem_ref, allow_direct: bool) { page_pool_put_netmem(pool, netmem, u32::MAX, allow_direct); }
#[inline]
pub unsafe fn page_pool_put_full_page(pool: *mut page_pool, page: *mut page, allow_direct: bool) { page_pool_put_netmem(pool, page_to_netmem(page), u32::MAX, allow_direct); }
#[inline]
pub unsafe fn page_pool_recycle_direct(pool: *mut page_pool, page: *mut page) { page_pool_put_full_page(pool, page, true); }
#[inline]
pub unsafe fn page_pool_recycle_direct_netmem(pool: *mut page_pool, netmem: netmem_ref) { page_pool_put_full_netmem(pool, netmem, true); }

pub const PAGE_POOL_32BIT_ARCH_WITH_64BIT_DMA: bool = core::mem::size_of::<dma_addr_t>() > core::mem::size_of::<usize>();

#[inline]
pub unsafe fn page_pool_free_va(pool: *mut page_pool, va: *mut core::ffi::c_void, allow_direct: bool) {
    page_pool_put_page(pool, virt_to_head_page(va), u32::MAX, allow_direct);
}
#[inline]
pub unsafe fn page_pool_get_dma_addr_netmem(netmem: netmem_ref) -> dma_addr_t {
    let mut ret = netmem_get_dma_addr(netmem);
    if PAGE_POOL_32BIT_ARCH_WITH_64BIT_DMA { ret <<= PAGE_SHIFT; }
    ret
}
#[inline]
pub unsafe fn page_pool_get_dma_addr(page: *const page) -> dma_addr_t { page_pool_get_dma_addr_netmem(page_to_netmem(page)) }
#[inline]
pub unsafe fn __page_pool_dma_sync_for_cpu(pool: *const page_pool, dma_addr: dma_addr_t, offset: u32, dma_sync_size: u32) {
    dma_sync_single_range_for_cpu((*pool).p.dev, dma_addr, offset + (*pool).p.offset, dma_sync_size, page_pool_get_dma_dir(pool));
}
#[inline]
pub unsafe fn page_pool_dma_sync_for_cpu(pool: *const page_pool, page: *const page, offset: u32, dma_sync_size: u32) {
    __page_pool_dma_sync_for_cpu(pool, page_pool_get_dma_addr(page), offset, dma_sync_size);
}
#[inline]
pub unsafe fn page_pool_dma_sync_netmem_for_cpu(pool: *const page_pool, netmem: netmem_ref, offset: u32, dma_sync_size: u32) {
    if !(*pool).dma_sync_for_cpu { return; }
    __page_pool_dma_sync_for_cpu(pool, page_pool_get_dma_addr_netmem(netmem), offset, dma_sync_size);
}
#[inline]
pub unsafe fn page_pool_get(pool: *mut page_pool) { refcount_inc(&mut (*pool).user_cnt); }
#[inline]
pub unsafe fn page_pool_put(pool: *mut page_pool) -> bool { refcount_dec_and_test(&mut (*pool).user_cnt) }
#[inline]
pub unsafe fn page_pool_nid_changed(pool: *mut page_pool, new_nid: i32) { if (*pool).p.nid != new_nid { page_pool_update_nid(pool, new_nid); } }
#[inline]
pub unsafe fn page_pool_is_unreadable(pool: *mut page_pool) -> bool { !(*pool).mp_ops.is_null() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
