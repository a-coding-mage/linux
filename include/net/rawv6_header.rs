/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <net/protocol.h>
// #include <net/raw.h>

// The following types are provided by the corresponding translated network
// headers and are intentionally not redefined here.

extern "C" {
    pub static mut raw_v6_hashinfo: raw_hashinfo;

    pub fn raw_v6_match(
        net: *mut net,
        sk: *const sock,
        num: ::core::ffi::c_ushort,
        loc_addr: *const in6_addr,
        rmt_addr: *const in6_addr,
        dif: ::core::ffi::c_int,
        sdif: ::core::ffi::c_int,
    ) -> bool;

    pub fn raw_abort(sk: *mut sock, err: ::core::ffi::c_int) -> ::core::ffi::c_int;

    pub fn raw6_icmp_error(
        skb: *mut sk_buff,
        nexthdr: ::core::ffi::c_int,
        type_: u8,
        code: u8,
        inner_offset: ::core::ffi::c_int,
        _: __be32,
    );

    pub fn raw6_local_deliver(skb: *mut sk_buff, nexthdr: ::core::ffi::c_int) -> bool;

    pub fn rawv6_rcv(sk: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int;

    // Preserved from:
    // #if defined(CONFIG_IPV6_MIP6) || defined(CONFIG_IPV6_MIP6_MODULE)
    #[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
    pub fn rawv6_mh_filter_register(
        filter: Option<unsafe extern "C" fn(sock: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;

    #[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
    pub fn rawv6_mh_filter_unregister(
        filter: Option<unsafe extern "C" fn(sock: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;
    // #endif
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
