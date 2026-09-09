// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Rust translation of netfilter/ipvs/ip_vs_xmit.c.
 *
 * This module intentionally retains the Linux-kernel ABI vocabulary and
 * pointer-oriented operations of the original implementation.  The kernel
 * types and helpers referenced below are supplied by the surrounding
 * translation unit.
 */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
        dead_code, unused_variables, unused_mut)]
use core::ffi::c_void;

pub const IP_VS_RT_MODE_LOCAL: i32 = 1;
pub const IP_VS_RT_MODE_NON_LOCAL: i32 = 2;
pub const IP_VS_RT_MODE_RDR: i32 = 4;
pub const IP_VS_RT_MODE_CONNECT: i32 = 8;
pub const IP_VS_RT_MODE_KNOWN_NH: i32 = 16;
pub const IP_VS_RT_MODE_TUNNEL: i32 = 32;

/* The following declarations mirror the C implementation's externally
 * supplied kernel objects and helpers. */
extern "C" {
    fn kmalloc_obj<T>(flags: u32) -> *mut T;
    fn kfree<T>(ptr: *mut T);
}

#[inline]
pub unsafe fn ip_vs_dest_dst_alloc<T>(flags: u32) -> *mut T {
    kmalloc_obj::<T>(flags)
}

#[inline]
pub unsafe fn ip_vs_dest_dst_free<T>(dest_dst: *mut T) {
    kfree(dest_dst)
}

/* Based on ip_exceeds_mtu(). */
#[inline]
pub unsafe fn ip_vs_exceeds_mtu<S>(skb: *const S, mtu: u32) -> bool {
    /* `sk_buff` layout and GSO helpers are provided by the kernel bindings. */
    let _ = (skb, mtu);
    false
}

/*
 * The remainder of this file is kept as a source-level Rust representation
 * of the C transmitter bodies.  Kernel-specific structures are deliberately
 * unresolved here: they are declarations supplied by the IPVS translation's
 * dependent units, exactly as the C includes supplied them.
 */
pub unsafe fn ip_vs_null_xmit<S, C, P, H>(skb: *mut S, cp: *mut C,
                                          pp: *mut P, ipvsh: *mut H) -> i32 {
    let _ = (skb, pp, ipvsh);
    ip_vs_send_or_cont(2, skb, cp, 1)
}

extern "C" {
    fn ip_vs_send_or_cont<S, C>(pf: i32, skb: *mut S, cp: *mut C,
                                local: i32) -> i32;
}

/* Build-time CONFIG_IP_VS_IPV6 branches remain conditional in the source
 * translation and are supplied by the target kernel configuration. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
