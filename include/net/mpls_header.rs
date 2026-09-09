/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 Nicira, Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/if_ether.h, linux/netdevice.h, and linux/mpls.h.

pub const MPLS_HLEN: usize = 4;

#[repr(C)]
pub struct mpls_shim_hdr {
    pub label_stack_entry: u32,
}

#[inline]
pub unsafe fn eth_p_mpls(eth_type: u16) -> bool {
    eth_type == htons(ETH_P_MPLS_UC) || eth_type == htons(ETH_P_MPLS_MC)
}

#[inline]
pub unsafe fn mpls_hdr(skb: *const sk_buff) -> *mut mpls_shim_hdr {
    skb_network_header(skb) as *mut mpls_shim_hdr
}

#[inline]
pub fn mpls_entry_encode(
    label: u32,
    ttl: u32,
    tc: u32,
    bos: bool,
) -> mpls_shim_hdr {
    let result = mpls_shim_hdr {
        label_stack_entry: cpu_to_be32(
            (label << MPLS_LS_LABEL_SHIFT)
                | (tc << MPLS_LS_TC_SHIFT)
                | (if bos { 1 << MPLS_LS_S_SHIFT } else { 0 })
                | (ttl << MPLS_LS_TTL_SHIFT),
        ),
    };
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
