/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 Cumulus Networks, Inc.
 */

// Translated from <linux/types.h> and <net/lwtunnel.h>.

#[repr(C)]
pub struct mpls_iptunnel_encap {
    pub labels: u8,
    pub ttl_propagate: u8,
    pub default_ttl: u8,
    pub reserved1: u8,
    pub label: [u32; 0],
}

/// Equivalent to the C inline function `mpls_lwtunnel_encap`.
#[inline]
pub unsafe fn mpls_lwtunnel_encap(
    lwtstate: *mut lwtunnel_state,
) -> *mut mpls_iptunnel_encap {
    // SAFETY: The caller must provide a valid pointer to a `lwtunnel_state`
    // whose data points to an `mpls_iptunnel_encap`, as required by the C API.
    unsafe { (*lwtstate).data as *mut mpls_iptunnel_encap }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
