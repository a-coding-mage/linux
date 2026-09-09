/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2024-2025 Intel Corporation */

// C dependencies: <linux/if_vlan.h>, <net/page_pool/helpers.h>, <net/xdp.h>

pub const LIBETH_SKB_HEADROOM: usize = NET_SKB_PAD + NET_IP_ALIGN;
pub const LIBETH_XDP_HEADROOM: usize = ALIGN(XDP_PACKET_HEADROOM, NET_SKB_PAD) + NET_IP_ALIGN;
pub const LIBETH_MAX_HEADROOM: usize = LIBETH_XDP_HEADROOM;
pub const LIBETH_RX_LL_LEN: usize = ETH_HLEN + 2 * VLAN_HLEN + ETH_FCS_LEN;
pub const LIBETH_MAX_HEAD: usize = roundup_pow_of_two(max(MAX_HEADER, 256));
pub const LIBETH_RX_PAGE_ORDER: usize = 0;
pub const LIBETH_RX_BUF_STRIDE: usize = SKB_DATA_ALIGN(128);

#[inline]
pub const fn LIBETH_RX_PAGE_LEN(hr: usize) -> usize {
    ALIGN_DOWN(SKB_MAX_ORDER(hr, LIBETH_RX_PAGE_ORDER), LIBETH_RX_BUF_STRIDE)
}

#[repr(C)]
pub struct libeth_fqe {
    pub netmem: netmem_ref,
    pub offset: u32,
    pub truesize: u32,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libeth_fqe_type {
    LIBETH_FQE_MTU = 0,
    LIBETH_FQE_SHORT,
    LIBETH_FQE_HDR,
}

#[repr(C)]
pub struct libeth_fq {
    pub pp: *mut page_pool,
    pub fqes: *mut libeth_fqe,
    pub truesize: u32,
    pub count: u32,
    // C bit-fields: type:2, hsplit:1, xdp:1, no_napi:1.
    pub type_: libeth_fqe_type,
    pub hsplit: bool,
    pub xdp: bool,
    pub no_napi: bool,
    pub buf_len: u32,
    pub nid: i32,
}

extern "C" {
    pub fn libeth_rx_fq_create(fq: *mut libeth_fq, napi_dev: *mut core::ffi::c_void) -> i32;
    pub fn libeth_rx_fq_destroy(fq: *mut libeth_fq);
    pub fn libeth_rx_recycle_slow(netmem: netmem_ref);
    pub fn libeth_rx_pt_gen_hash_type(pt: *mut libeth_rx_pt);
}

#[inline]
pub unsafe fn libeth_rx_alloc(fq: *const libeth_fq, i: u32) -> dma_addr_t {
    let buf = &mut *(*fq).fqes.add(i as usize);
    buf.truesize = (*fq).truesize;
    buf.netmem = page_pool_dev_alloc_netmem((*fq).pp, &mut buf.offset, &mut buf.truesize);
    if buf.netmem.is_null() {
        return DMA_MAPPING_ERROR;
    }
    page_pool_get_dma_addr_netmem(buf.netmem) + buf.offset as dma_addr_t
        + (*(*fq).pp).p.offset as dma_addr_t
}

#[inline]
pub unsafe fn libeth_rx_sync_for_cpu(fqe: *const libeth_fqe, len: u32) -> bool {
    let netmem = (*fqe).netmem;
    if len == 0 {
        libeth_rx_recycle_slow(netmem);
        return false;
    }
    page_pool_dma_sync_netmem_for_cpu(netmem_get_pp(netmem), netmem, (*fqe).offset, len);
    true
}

pub const LIBETH_RX_PT_OUTER_L2: u32 = 0;
pub const LIBETH_RX_PT_OUTER_IPV4: u32 = 1;
pub const LIBETH_RX_PT_OUTER_IPV6: u32 = 2;
pub const LIBETH_RX_PT_NOT_FRAG: u32 = 0;
pub const LIBETH_RX_PT_FRAG: u32 = 1;
pub const LIBETH_RX_PT_TUNNEL_IP_NONE: u32 = 0;
pub const LIBETH_RX_PT_TUNNEL_IP_IP: u32 = 1;
pub const LIBETH_RX_PT_TUNNEL_IP_GRENAT: u32 = 2;
pub const LIBETH_RX_PT_TUNNEL_IP_GRENAT_MAC: u32 = 3;
pub const LIBETH_RX_PT_TUNNEL_IP_GRENAT_MAC_VLAN: u32 = 4;
pub const LIBETH_RX_PT_TUNNEL_END_NONE: u32 = 0;
pub const LIBETH_RX_PT_TUNNEL_END_IPV4: u32 = 1;
pub const LIBETH_RX_PT_TUNNEL_END_IPV6: u32 = 2;
pub const LIBETH_RX_PT_INNER_NONE: u32 = 0;
pub const LIBETH_RX_PT_INNER_UDP: u32 = 1;
pub const LIBETH_RX_PT_INNER_TCP: u32 = 2;
pub const LIBETH_RX_PT_INNER_SCTP: u32 = 3;
pub const LIBETH_RX_PT_INNER_ICMP: u32 = 4;
pub const LIBETH_RX_PT_INNER_TIMESYNC: u32 = 5;

pub const LIBETH_RX_PT_PAYLOAD_NONE: u32 = PKT_HASH_TYPE_NONE;
pub const LIBETH_RX_PT_PAYLOAD_L2: u32 = PKT_HASH_TYPE_L2;
pub const LIBETH_RX_PT_PAYLOAD_L3: u32 = PKT_HASH_TYPE_L3;
pub const LIBETH_RX_PT_PAYLOAD_L4: u32 = PKT_HASH_TYPE_L4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_rx_pt {
    pub outer_ip: u32,
    pub outer_frag: u32,
    pub tunnel_type: u32,
    pub tunnel_end_prot: u32,
    pub tunnel_end_frag: u32,
    pub inner_prot: u32,
    pub payload_layer: pkt_hash_types,
    pub pad: u32,
    pub hash_type: xdp_rss_hash_type,
}

#[repr(C)]
pub struct libeth_rx_csum {
    pub l3l4p: u32, pub ipe: u32, pub eipe: u32, pub eudpe: u32,
    pub ipv6exadd: u32, pub l4e: u32, pub pprs: u32, pub nat: u32,
    pub raw_csum_valid: u32, pub pad: u32, pub raw_csum: u32,
}

#[repr(C)]
pub struct libeth_rqe_info {
    pub len: u32,
    pub ptype: u32,
    pub eop: u32,
    pub rxe: u32,
    pub vlan: u32,
}

#[inline]
pub fn libeth_rx_pt_get_ip_ver(pt: libeth_rx_pt) -> u32 {
    pt.outer_ip
}

#[inline]
pub unsafe fn libeth_rx_pt_has_checksum(dev: *const net_device, pt: libeth_rx_pt) -> bool {
    pt.inner_prot > LIBETH_RX_PT_INNER_NONE && ((*dev).features & NETIF_F_RXCSUM) != 0
}

#[inline]
pub unsafe fn libeth_rx_pt_has_hash(dev: *const net_device, pt: libeth_rx_pt) -> bool {
    pt.payload_layer as u32 > LIBETH_RX_PT_PAYLOAD_NONE && ((*dev).features & NETIF_F_RXHASH) != 0
}

#[inline]
pub unsafe fn libeth_rx_pt_set_hash(skb: *mut sk_buff, hash: u32, pt: libeth_rx_pt) {
    skb_set_hash(skb, hash, pt.payload_layer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
