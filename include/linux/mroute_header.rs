/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(feature = "CONFIG_IP_MROUTE")]
#[inline]
pub fn ip_mroute_opt(opt: ::core::ffi::c_int) -> bool {
    opt >= MRT_BASE && opt <= MRT_MAX
}

#[cfg(feature = "CONFIG_IP_MROUTE")]
extern "C" {
    pub fn ip_mroute_setsockopt(
        _: *mut sock,
        _: ::core::ffi::c_int,
        _: sockptr_t,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn ip_mroute_getsockopt(
        _: *mut sock,
        _: ::core::ffi::c_int,
        _: sockptr_t,
        _: sockptr_t,
    ) -> ::core::ffi::c_int;
    pub fn ipmr_ioctl(
        _: *mut sock,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn ipmr_compat_ioctl(
        _: *mut sock,
        _: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn ip_mr_init() -> ::core::ffi::c_int;
    pub fn ipmr_rule_default(_: *const fib_rule) -> bool;
    pub fn ipmr_sk_ioctl(
        _: *mut sock,
        _: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_IP_MROUTE"))]
#[inline]
pub fn ip_mroute_setsockopt(
    _: *mut sock,
    _: ::core::ffi::c_int,
    _: sockptr_t,
    _: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    -ENOPROTOOPT
}

#[cfg(not(feature = "CONFIG_IP_MROUTE"))]
#[inline]
pub fn ip_mroute_getsockopt(
    _: *mut sock,
    _: ::core::ffi::c_int,
    _: sockptr_t,
    _: sockptr_t,
) -> ::core::ffi::c_int {
    -ENOPROTOOPT
}

#[cfg(not(feature = "CONFIG_IP_MROUTE"))]
#[inline]
pub fn ipmr_ioctl(
    _: *mut sock,
    _: ::core::ffi::c_int,
    _: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    -ENOIOCTLCMD
}

#[cfg(not(feature = "CONFIG_IP_MROUTE"))]
#[inline]
pub fn ip_mr_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_IP_MROUTE"))]
#[inline]
pub fn ip_mroute_opt(_: ::core::ffi::c_int) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_IP_MROUTE"))]
#[inline]
pub fn ipmr_rule_default(_: *const fib_rule) -> bool {
    true
}

#[cfg(not(feature = "CONFIG_IP_MROUTE"))]
#[inline]
pub fn ipmr_sk_ioctl(
    _: *mut sock,
    _: ::core::ffi::c_uint,
    _: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    1
}

pub const VIFF_STATIC: u16 = 0x8000;

#[repr(C)]
pub struct mfc_cache_cmp_arg {
    pub mfc_mcastgrp: __be32,
    pub mfc_origin: __be32,
}

/**
 * struct mfc_cache - multicast routing entries
 * @_c: Common multicast routing information; has to be first [for casting]
 * @mfc_mcastgrp: destination multicast group address
 * @mfc_origin: source address
 * @cmparg: used for rhashtable comparisons
 */
#[repr(C)]
pub struct mfc_cache {
    pub _c: mr_mfc,
    pub fields: mfc_cache_fields,
}

#[repr(C)]
pub union mfc_cache_fields {
    pub addresses: mfc_cache_addresses,
    pub cmparg: mfc_cache_cmp_arg,
}

#[repr(C)]
pub struct mfc_cache_addresses {
    pub mfc_mcastgrp: __be32,
    pub mfc_origin: __be32,
}

extern "C" {
    pub fn ipmr_get_route(
        _: *mut net,
        _: *mut sk_buff,
        _: __be32,
        _: __be32,
        _: *mut rtmsg,
        _: u32,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
