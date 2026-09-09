/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2020 Intel Corporation. */

// External Linux/kernel types and constants are supplied by dependent modules.

pub const XSK_PRIV_MAX: usize = 24;

#[repr(C)]
pub struct xdp_buff_xsk {
    pub xdp: xdp_buff,
    pub cb: [u8; XSK_PRIV_MAX],
    pub dma: dma_addr_t,
    pub frame_dma: dma_addr_t,
    pub pool: *mut xsk_buff_pool,
    pub list_node: list_head,
}

// XSK_CHECK_PRIV_TYPE(t): BUILD_BUG_ON(size_of::<t>() > offsetofend(xdp_buff_xsk, cb))
// XSK_TX_COMPL_FITS(t): BUILD_BUG_ON(size_of::<xsk_tx_metadata_compl>() > size_of::<t>())

#[repr(C)]
pub struct xsk_dma_map {
    pub dma_pages: *mut dma_addr_t,
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub users: refcount_t,
    pub list: list_head, // Protected by the RTNL_LOCK
    pub dma_pages_cnt: u32,
}

#[repr(C)]
pub struct xsk_buff_pool {
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub xsk_tx_list: list_head,
    pub xsk_tx_list_lock: spinlock_t,
    pub users: refcount_t,
    pub umem: *mut xdp_umem,
    pub work: work_struct,
    pub rx_lock: spinlock_t,
    pub free_list: list_head,
    pub xskb_list: list_head,
    pub heads_cnt: u32,
    pub queue_id: u16,
    pub fq: *mut xsk_queue,
    pub cq: *mut xsk_queue,
    pub dma_pages: *mut dma_addr_t,
    pub heads: *mut xdp_buff_xsk,
    pub tx_descs: *mut xdp_desc,
    pub chunk_mask: u64,
    pub addrs_cnt: u64,
    pub free_list_cnt: u32,
    pub dma_pages_cnt: u32,
    pub free_heads_cnt: u32,
    pub headroom: u32,
    pub chunk_size: u32,
    pub chunk_shift: u32,
    pub frame_len: u32,
    pub tx_descs_nentries: u32,
    pub reclaim_descs: u32,
    pub tx_zc_pending_descs: u32,
    pub xdp_zc_max_segs: u32,
    pub tx_metadata_len: u8,
    pub cached_need_wakeup: u8,
    pub uses_need_wakeup: bool,
    pub unaligned: bool,
    pub tx_sw_csum: bool,
    pub addrs: *mut core::ffi::c_void,
    pub cq_prod_lock: spinlock_t,
    pub free_heads: [*mut xdp_buff_xsk; 0],
}

pub const XSK_NEXT_PG_CONTIG_SHIFT: u32 = 0;
pub const XSK_NEXT_PG_CONTIG_MASK: u64 = 1u64 << XSK_NEXT_PG_CONTIG_SHIFT;

unsafe extern "C" {
    pub fn xp_create_and_assign_umem(xs: *mut xdp_sock, umem: *mut xdp_umem, max_segs: u32) -> *mut xsk_buff_pool;
    pub fn xp_assign_dev(pool: *mut xsk_buff_pool, dev: *mut net_device, queue_id: u16, flags: u16) -> i32;
    pub fn xp_assign_dev_shared(pool: *mut xsk_buff_pool, umem_xs: *mut xdp_sock, dev: *mut net_device, queue_id: u16) -> i32;
    pub fn xp_alloc_tx_descs(pool: *mut xsk_buff_pool, xs: *mut xdp_sock, max_segs: u32) -> i32;
    pub fn xp_destroy(pool: *mut xsk_buff_pool);
    pub fn xp_get_pool(pool: *mut xsk_buff_pool);
    pub fn xp_put_pool(pool: *mut xsk_buff_pool) -> bool;
    pub fn xp_clear_dev(pool: *mut xsk_buff_pool);
    pub fn xp_add_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock);
    pub fn xp_del_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock);
    pub fn xp_free(xskb: *mut xdp_buff_xsk);
    pub fn xp_set_rxq_info(pool: *mut xsk_buff_pool, rxq: *mut xdp_rxq_info);
    pub fn xp_fill_cb(pool: *mut xsk_buff_pool, desc: *mut xsk_cb_desc);
    pub fn xp_dma_map(pool: *mut xsk_buff_pool, dev: *mut device, attrs: usize, pages: *mut *mut page, nr_pages: u32) -> i32;
    pub fn xp_dma_unmap(pool: *mut xsk_buff_pool, attrs: usize);
    pub fn xp_alloc(pool: *mut xsk_buff_pool) -> *mut xdp_buff;
    pub fn xp_alloc_batch(pool: *mut xsk_buff_pool, xdp: *mut *mut xdp_buff, max: u32) -> u32;
    pub fn xp_can_alloc(pool: *mut xsk_buff_pool, count: u32) -> bool;
    pub fn xp_raw_get_data(pool: *mut xsk_buff_pool, addr: u64) -> *mut core::ffi::c_void;
    pub fn xp_raw_get_dma(pool: *mut xsk_buff_pool, addr: u64) -> dma_addr_t;
}

#[repr(C)]
pub struct xdp_desc_ctx {
    pub dma: dma_addr_t,
    pub meta: *mut xsk_tx_metadata,
}

unsafe extern "C" {
    pub fn xp_raw_get_ctx(pool: *const xsk_buff_pool, addr: u64, options: u32) -> xdp_desc_ctx;
}

#[inline]
pub unsafe fn xp_init_xskb_addr(xskb: *mut xdp_buff_xsk, pool: *mut xsk_buff_pool, addr: u64) {
    (*xskb).xdp.data_hard_start = (*pool).addrs.add(addr as usize).add((*pool).headroom as usize);
}

#[inline]
pub unsafe fn xp_init_xskb_dma(xskb: *mut xdp_buff_xsk, pool: *mut xsk_buff_pool, dma_pages: *mut dma_addr_t, addr: u64) {
    (*xskb).frame_dma = (*dma_pages.add((addr >> PAGE_SHIFT) as usize) & !XSK_NEXT_PG_CONTIG_MASK) + (addr & !PAGE_MASK);
    (*xskb).dma = (*xskb).frame_dma + (*pool).headroom as dma_addr_t + XDP_PACKET_HEADROOM as dma_addr_t;
}

#[inline] pub unsafe fn xp_get_dma(xskb: *mut xdp_buff_xsk) -> dma_addr_t { (*xskb).dma }
#[inline] pub unsafe fn xp_get_frame_dma(xskb: *mut xdp_buff_xsk) -> dma_addr_t { (*xskb).frame_dma }

#[inline]
pub unsafe fn xp_dma_sync_for_cpu(xskb: *mut xdp_buff_xsk) {
    dma_sync_single_for_cpu((*(*xskb).pool).dev, (*xskb).dma, (*(*xskb).pool).frame_len as usize, DMA_BIDIRECTIONAL);
}

#[inline]
pub unsafe fn xp_dma_sync_for_device(pool: *mut xsk_buff_pool, dma: dma_addr_t, size: usize) {
    dma_sync_single_for_device((*pool).dev, dma, size, DMA_BIDIRECTIONAL);
}

#[inline]
pub unsafe fn xp_desc_crosses_non_contig_pg(pool: *mut xsk_buff_pool, addr: u64, len: u32) -> bool {
    let cross_pg = (addr & (PAGE_SIZE - 1)) + len as u64 > PAGE_SIZE;
    if !cross_pg { return false; }
    !(*pool).dma_pages.is_null() && !((*(*pool).dma_pages.add((addr >> PAGE_SHIFT) as usize)) & XSK_NEXT_PG_CONTIG_MASK != 0)
}

#[inline] pub unsafe fn xp_mb_desc(desc: *const xdp_desc) -> bool { (*desc).options & XDP_PKT_CONTD != 0 }
#[inline] pub unsafe fn xp_aligned_extract_addr(pool: *mut xsk_buff_pool, addr: u64) -> u64 { addr & (*pool).chunk_mask }
#[inline] pub fn xp_unaligned_extract_addr(addr: u64) -> u64 { addr & XSK_UNALIGNED_BUF_ADDR_MASK }
#[inline] pub fn xp_unaligned_extract_offset(addr: u64) -> u64 { addr >> XSK_UNALIGNED_BUF_OFFSET_SHIFT }
#[inline] pub fn xp_unaligned_add_offset_to_addr(addr: u64) -> u64 { xp_unaligned_extract_addr(addr) + xp_unaligned_extract_offset(addr) }
#[inline] pub unsafe fn xp_aligned_extract_idx(pool: *mut xsk_buff_pool, addr: u64) -> u32 { (xp_aligned_extract_addr(pool, addr) >> (*pool).chunk_shift) as u32 }

#[inline]
pub unsafe fn xp_release(xskb: *mut xdp_buff_xsk) {
    if (*(*xskb).pool).unaligned {
        let pool = (*xskb).pool;
        (*pool).free_heads.add((*pool).free_heads_cnt as usize).write(xskb);
        (*pool).free_heads_cnt += 1;
    }
}

#[inline]
pub unsafe fn xp_get_handle(xskb: *mut xdp_buff_xsk, pool: *mut xsk_buff_pool) -> u64 {
    let mut orig_addr = (*xskb).xdp.data.offset_from((*pool).addrs as *const u8) as u64;
    if !(*pool).unaligned { return orig_addr; }
    let mut offset = (*xskb).xdp.data.offset_from((*xskb).xdp.data_hard_start) as u64;
    offset += (*pool).headroom as u64;
    orig_addr -= offset;
    orig_addr + (offset << XSK_UNALIGNED_BUF_OFFSET_SHIFT)
}

#[inline] pub unsafe fn xp_tx_metadata_enabled(pool: *const xsk_buff_pool) -> bool { (*pool).tx_metadata_len > 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
