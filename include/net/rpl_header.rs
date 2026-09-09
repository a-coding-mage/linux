/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  RPL implementation
 *
 *  Author:
 *  (C) 2020 Alexander Aring <alex.aring@gmail.com>
 */

// Dependency supplied by the Linux RPL header: `ipv6_rpl_sr_hdr` and
// `in6_addr`.

// Equivalent of IS_ENABLED(CONFIG_IPV6_RPL_LWTUNNEL).  When the RPL
// lightweight-tunnel configuration is enabled, these are external symbols.
#[cfg(feature = "CONFIG_IPV6_RPL_LWTUNNEL")]
unsafe extern "C" {
    pub fn rpl_init() -> ::core::ffi::c_int;
    pub fn rpl_exit();
}

// Fallback used when CONFIG_IPV6_RPL_LWTUNNEL is disabled.
#[cfg(not(feature = "CONFIG_IPV6_RPL_LWTUNNEL"))]
#[inline]
pub fn rpl_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_IPV6_RPL_LWTUNNEL"))]
#[inline]
pub fn rpl_exit() {}

unsafe extern "C" {
    pub fn ipv6_rpl_srh_decompress(
        outhdr: *mut ipv6_rpl_sr_hdr,
        inhdr: *const ipv6_rpl_sr_hdr,
        daddr: *const in6_addr,
        n: u8,
    );

    pub fn ipv6_rpl_srh_compress(
        outhdr: *mut ipv6_rpl_sr_hdr,
        inhdr: *const ipv6_rpl_sr_hdr,
        daddr: *const in6_addr,
        n: u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
