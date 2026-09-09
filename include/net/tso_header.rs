/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

pub const TSO_HEADER_SIZE: c_int = 256;

#[repr(C)]
pub struct tso_t {
    pub next_frag_idx: c_int,
    pub size: c_int,
    pub data: *mut c_void,
    pub ip_id: u16,
    pub tlen: u8,
    pub ipv6: bool,
    pub tcp_seq: u32,
}

/* Calculate the worst case buffer count. `skb_shinfo` is supplied by the kernel headers. */
#[inline]
pub unsafe fn tso_count_descs(skb: *const sk_buff) -> c_int {
    (*skb_shinfo(skb)).gso_segs as c_int * 2 + (*skb_shinfo(skb)).nr_frags as c_int
}

extern "C" {
    pub fn tso_build_hdr(skb: *const sk_buff, hdr: *mut c_char, tso: *mut tso_t,
                         size: c_int, is_last: bool);
    pub fn tso_build_data(skb: *const sk_buff, tso: *mut tso_t, size: c_int);
    pub fn tso_start(skb: *mut sk_buff, tso: *mut tso_t) -> c_int;
}

#[repr(C)]
pub struct tso_dma_map {
    pub dev: *mut device,
    pub skb: *const sk_buff,
    pub hdr_len: usize,
    /* IOVA path */
    pub iova_state: dma_iova_state,
    pub iova_offset: usize,
    pub total_len: usize,
    /* Fallback path if IOVA path fails */
    pub frag_idx: c_int,
    pub offset: u32,
    pub linear_dma: dma_addr_t,
    pub linear_len: u32,
    pub nr_frags: u32,
    pub frags: [tso_dma_map_frag; MAX_SKB_FRAGS],
}

#[repr(C)]
pub struct tso_dma_map_frag {
    pub dma: dma_addr_t,
    pub len: u32,
}

#[repr(C)]
pub struct tso_dma_map_completion_state {
    pub iova_state: dma_iova_state,
    pub total_len: usize,
}

extern "C" {
    pub fn tso_dma_map_init(map: *mut tso_dma_map, dev: *mut device,
                            skb: *const sk_buff, hdr_len: u32) -> c_int;
    pub fn tso_dma_map_cleanup(map: *mut tso_dma_map);
    pub fn tso_dma_map_count(map: *mut tso_dma_map, len: u32) -> u32;
    pub fn tso_dma_map_next(map: *mut tso_dma_map, addr: *mut dma_addr_t,
                            chunk_len: *mut u32, mapping_len: *mut u32,
                            seg_remaining: u32) -> bool;
}

#[inline]
pub unsafe fn tso_dma_map_completion_save(
    map: *const tso_dma_map,
    cstate: *mut tso_dma_map_completion_state,
) {
    (*cstate).iova_state = (*map).iova_state;
    (*cstate).total_len = (*map).total_len;
}

#[inline]
pub unsafe fn tso_dma_map_complete(
    dev: *mut device,
    cstate: *mut tso_dma_map_completion_state,
) -> bool {
    if dma_use_iova(&(*cstate).iova_state) {
        dma_iova_destroy(dev, &mut (*cstate).iova_state, (*cstate).total_len,
                         DMA_TO_DEVICE, 0);
        return true;
    }

    false
}

/* Types, constants, and kernel-header macros referenced above are supplied by dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
