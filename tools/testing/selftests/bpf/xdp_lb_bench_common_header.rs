/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

pub const F_IPV6: u32 = 1 << 0;
pub const F_LRU_BYPASS: u32 = 1 << 1;

pub const CH_RING_SIZE: u32 = 65537; /* per-VIP consistent hash ring slots */
pub const MAX_VIPS: u32 = 16;
pub const CH_RINGS_SIZE: u32 = MAX_VIPS * CH_RING_SIZE;
pub const MAX_REALS: u32 = 512;
pub const DEFAULT_LRU_SIZE: u32 = 100000; /* connection tracking cache size */
pub const ONE_SEC: u32 = 1000000000_u32; /* 1 sec in nanosec */
pub const MAX_CONN_RATE: u32 = 100000000; /* high enough to never trigger in bench */
pub const LRU_UDP_TIMEOUT: u64 = 30000000000_u64; /* 30 sec in nanosec */
pub const PCKT_FRAGMENTED: u32 = 0x3FFF;
pub const KNUTH_HASH_MULT: u32 = 2654435761_u32;
pub const IPIP_V4_PREFIX: u32 = 4268; /* 172.16/12 in network order */
pub const IPIP_V6_PREFIX1: u32 = 1; /* 0100::/64 (RFC 6666 discard) */
pub const IPIP_V6_PREFIX2: u32 = 0;
pub const IPIP_V6_PREFIX3: u32 = 0;

/* Stats indices (0..MAX_VIPS-1 are per-VIP packet/byte counters) */
pub const STATS_LRU: u32 = MAX_VIPS + 0; /* v1: total VIP packets, v2: LRU misses */
pub const STATS_XDP_TX: u32 = MAX_VIPS + 1;
pub const STATS_XDP_PASS: u32 = MAX_VIPS + 2;
pub const STATS_XDP_DROP: u32 = MAX_VIPS + 3;
pub const STATS_NEW_CONN: u32 = MAX_VIPS + 4; /* v1: conn count, v2: last reset ts */
pub const STATS_LRU_MISS: u32 = MAX_VIPS + 5; /* v1: TCP LRU misses */
pub const STATS_SIZE: u32 = MAX_VIPS + 6;

/*
 * In C, lb_htons maps to bpf_htons under __BPF__ and htons otherwise, and
 * LB_INLINE maps to static inline attributes. Preserve that conditional intent.
 */
#[cfg(__BPF__)]
unsafe extern "C" {
    fn bpf_htons(x: __u16) -> __u16;
}

#[cfg(not(__BPF__))]
unsafe extern "C" {
    fn htons(x: __u16) -> __u16;
}

#[inline]
pub unsafe fn lb_htons(x: __u16) -> __u16 {
    #[cfg(__BPF__)]
    {
        unsafe { bpf_htons(x) }
    }
    #[cfg(not(__BPF__))]
    {
        unsafe { htons(x) }
    }
}

#[inline]
pub unsafe fn create_encap_ipv4_src(port: __u16, src: __be32) -> __be32 {
    let mut ip_suffix: __u32 = unsafe { lb_htons(port) } as __u32;

    ip_suffix <<= 16;
    ip_suffix ^= src;
    ((0xFFFF0000_u32 & ip_suffix) | IPIP_V4_PREFIX) as __be32
}

#[inline]
pub unsafe fn create_encap_ipv6_src(port: __u16, src: __be32, saddr: *mut __be32) {
    unsafe {
        *saddr.add(0) = IPIP_V6_PREFIX1 as __be32;
        *saddr.add(1) = IPIP_V6_PREFIX2 as __be32;
        *saddr.add(2) = IPIP_V6_PREFIX3 as __be32;
        *saddr.add(3) = src ^ port as __be32;
    }
}

#[repr(C)]
pub union flow_key_src {
    pub src: __be32,
    pub srcv6: [__be32; 4],
}

#[repr(C)]
pub union flow_key_dst {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
}

#[repr(C)]
pub union flow_key_ports {
    pub ports: __u32,
    pub port16: [__u16; 2],
}

#[repr(C)]
pub struct flow_key {
    pub src: flow_key_src,
    pub dst: flow_key_dst,
    pub ports: flow_key_ports,
    pub proto: __u8,
    pub pad: [__u8; 3],
}

#[repr(C)]
pub union vip_definition_vip {
    pub vip: __be32,
    pub vipv6: [__be32; 4],
}

#[repr(C)]
pub struct vip_definition {
    pub vip: vip_definition_vip,
    pub port: __u16,
    pub proto: __u8,
    pub pad: __u8,
}

#[repr(C)]
pub struct vip_meta {
    pub flags: __u32,
    pub vip_num: __u32,
}

#[repr(C)]
pub struct real_pos_lru {
    pub pos: __u32,
    pub atime: __u64,
}

#[repr(C)]
pub struct real_definition {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
    pub flags: __u8,
}

#[repr(C)]
pub struct lb_stats {
    pub v1: __u64,
    pub v2: __u64,
}

#[repr(C)]
pub struct ctl_value {
    pub mac: [__u8; 6],
    pub pad: [__u8; 2],
}
