/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header __NET_FOU_H.
// C dependencies:
//   <linux/skbuff.h>
//   <net/flow.h>
//   <net/gue.h>
//   <net/ip_tunnels.h>
//   <net/udp.h>

extern "C" {
    pub fn fou_encap_hlen(e: *mut crate::ip_tunnel_encap) -> usize;
    pub fn gue_encap_hlen(e: *mut crate::ip_tunnel_encap) -> usize;

    pub fn __fou_build_header(
        skb: *mut crate::sk_buff,
        e: *mut crate::ip_tunnel_encap,
        protocol: *mut u8,
        sport: *mut crate::__be16,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn __gue_build_header(
        skb: *mut crate::sk_buff,
        e: *mut crate::ip_tunnel_encap,
        protocol: *mut u8,
        sport: *mut crate::__be16,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn register_fou_bpf() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
