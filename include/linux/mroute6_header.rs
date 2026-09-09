/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(CONFIG_IPV6_MROUTE)]
#[inline]
pub fn ip6_mroute_opt(opt: ::core::ffi::c_int) -> bool {
    (opt >= MRT6_BASE) && (opt <= MRT6_MAX)
}

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub fn ip6_mroute_opt(_opt: ::core::ffi::c_int) -> bool {
    false
}

#[cfg(CONFIG_IPV6_MROUTE)]
extern "C" {
    pub fn ip6_mroute_setsockopt(
        sock: *mut sock,
        optname: ::core::ffi::c_int,
        optval: sockptr_t,
        optlen: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn ip6_mroute_getsockopt(
        sock: *mut sock,
        optname: ::core::ffi::c_int,
        optval: sockptr_t,
        optlen: sockptr_t,
    ) -> ::core::ffi::c_int;
    pub fn ip6_mr_input(skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn ip6mr_compat_ioctl(
        sk: *mut sock,
        cmd: ::core::ffi::c_uint,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn ip6_mr_init() -> ::core::ffi::c_int;
    pub fn ip6_mr_output(
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
    ) -> ::core::ffi::c_int;
    pub fn ip6_mr_cleanup();
    pub fn ip6mr_ioctl(
        sk: *mut sock,
        cmd: ::core::ffi::c_int,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub unsafe fn ip6_mroute_setsockopt(
    _sock: *mut sock,
    _optname: ::core::ffi::c_int,
    _optval: sockptr_t,
    _optlen: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    -ENOPROTOOPT
}

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub unsafe fn ip6_mroute_getsockopt(
    _sock: *mut sock,
    _optname: ::core::ffi::c_int,
    _optval: sockptr_t,
    _optlen: sockptr_t,
) -> ::core::ffi::c_int {
    -ENOPROTOOPT
}

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub unsafe fn ip6mr_ioctl(
    _sk: *mut sock,
    _cmd: ::core::ffi::c_int,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    -ENOIOCTLCMD
}

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub fn ip6_mr_init() -> ::core::ffi::c_int { 0 }

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub unsafe fn ip6_mr_output(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> ::core::ffi::c_int {
    ip6_output(net, sk, skb)
}

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub fn ip6_mr_cleanup() {}

#[cfg(CONFIG_IPV6_MROUTE_MULTIPLE_TABLES)]
extern "C" {
    pub fn ip6mr_rule_default(rule: *const fib_rule) -> bool;
}

#[cfg(not(CONFIG_IPV6_MROUTE_MULTIPLE_TABLES))]
#[inline]
pub fn ip6mr_rule_default(_rule: *const fib_rule) -> bool { true }

pub const VIFF_STATIC: ::core::ffi::c_uint = 0x8000;

#[repr(C)]
pub struct mfc6_cache_cmp_arg {
    pub mf6c_mcastgrp: in6_addr,
    pub mf6c_origin: in6_addr,
}

#[repr(C)]
pub struct mfc6_cache_fields {
    pub mf6c_mcastgrp: in6_addr,
    pub mf6c_origin: in6_addr,
}

#[repr(C)]
pub union mfc6_cache_union {
    pub fields: mfc6_cache_fields,
    pub cmparg: mfc6_cache_cmp_arg,
}

#[repr(C)]
pub struct mfc6_cache {
    pub _c: mr_mfc,
    pub data: mfc6_cache_union,
}

pub const MFC_ASSERT_THRESH: ::core::ffi::c_uint = 3 * HZ;

extern "C" {
    pub fn ip6mr_get_route(
        net: *mut net,
        skb: *mut sk_buff,
        rtm: *mut rtmsg,
        portid: u32,
    ) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_IPV6_MROUTE)]
extern "C" {
    pub fn mroute6_is_socket(net: *mut net, skb: *mut sk_buff) -> bool;
    pub fn ip6mr_sk_done(sk: *mut sock) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_IPV6_MROUTE)]
#[inline]
pub unsafe fn ip6mr_sk_ioctl(
    sk: *mut sock,
    cmd: ::core::ffi::c_uint,
    arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    match cmd {
        SIOCGETMIFCNT_IN6 => {
            let mut buffer: sioc_mif_req6 = ::core::mem::zeroed();
            sock_ioctl_inout(sk, cmd, arg, &mut buffer, ::core::mem::size_of::<sioc_mif_req6>())
        }
        SIOCGETSGCNT_IN6 => {
            let mut buffer: sioc_sg_req6 = ::core::mem::zeroed();
            sock_ioctl_inout(sk, cmd, arg, &mut buffer, ::core::mem::size_of::<sioc_sg_req6>())
        }
        _ => 1,
    }
}

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub fn mroute6_is_socket(_net: *mut net, _skb: *mut sk_buff) -> bool { false }

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub fn ip6mr_sk_done(_sk: *mut sock) -> ::core::ffi::c_int { 0 }

#[cfg(not(CONFIG_IPV6_MROUTE))]
#[inline]
pub fn ip6mr_sk_ioctl(
    _sk: *mut sock,
    _cmd: ::core::ffi::c_uint,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
