// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright 2019, 2020 Cloudflare

// C dependencies from the original header:
// <stdbool.h>, <stddef.h>, <stdint.h>, <string.h>
// <linux/if_ether.h>, <linux/in.h>, <linux/ip.h>, <linux/ipv6.h>,
// and <netinet/udp.h>.

// The original C header temporarily restores the compiler builtin offsetof()
// when libbpf has redefined it, for use in static asserts on older clang.

#[repr(C, packed)]
pub struct gre_base_hdr {
    pub flags: u16,
    pub protocol: u16,
}

#[repr(C)]
pub struct guehdr {
    // C bitfield layout:
    // little endian: hlen:5, control:1, variant:2
    // big endian:    variant:2, control:1, hlen:5
    pub hlen_control_variant: u8,
    pub proto_ctype: u8,
    pub flags: u16,
}

#[repr(C, packed)]
pub struct unigue {
    // C bitfield layout:
    // little endian: _r:2, last_hop_gre:1, forward_syn:1, version:4
    // big endian:    version:4, forward_syn:1, last_hop_gre:1, _r:2
    pub r_last_hop_gre_forward_syn_version: u8,
    pub reserved: u8,
    pub next_hop: u8,
    pub hop_count: u8,
    // Next hops go here
}

#[repr(C, packed)]
pub struct encap_gre_t {
    pub eth: ethhdr,
    pub ip: iphdr,
    pub gre: gre_base_hdr,
}

#[repr(C, packed)]
pub struct encap_headers_t {
    pub eth: ethhdr,
    pub ip: iphdr,
    pub udp: udphdr,
    pub gue: guehdr,
    pub unigue: unigue,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
