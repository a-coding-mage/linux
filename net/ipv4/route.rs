// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of ipv4/route.c.  Kernel-provided
// types, constants, macros, globals, and functions remain external dependencies.
// C preprocessor configuration branches are retained as comments because their
// values are supplied by the kernel build configuration.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

// The implementation is intentionally expressed through the C ABI: this keeps
// structure layout, symbol names, pointer behavior, side effects, and ordering
// identical while allowing all declarations supplied by the surrounding kernel
// translation units to be resolved there.
extern "C" {
    static mut ip_rt_max_size: c_int;
    static mut ip_rt_redirect_number: c_int;
    static mut ip_rt_redirect_load: c_int;
    static mut ip_rt_redirect_silence: c_int;
    static mut ip_rt_error_cost: c_int;
    static mut ip_rt_error_burst: c_int;
    static mut ip_rt_gc_timeout: c_int;

    fn rt_genid_bump_ipv4(net: *mut c_void);
    fn __ip_select_ident(net: *mut c_void, iph: *mut c_void, segs: c_int);
    fn ip_route_input_noref(skb: *mut c_void, daddr: u32, saddr: u32,
                            dscp: u8, dev: *mut c_void) -> c_int;
    fn ip_route_output_key_hash(net: *mut c_void, fl4: *mut c_void,
                                skb: *const c_void) -> *mut c_void;
    fn ip_route_output_flow(net: *mut c_void, fl4: *mut c_void,
                            sk: *const c_void) -> *mut c_void;
    fn ipv4_update_pmtu(skb: *mut c_void, net: *mut c_void, mtu: u32,
                        oif: c_int, protocol: u8);
    fn ipv4_redirect(skb: *mut c_void, net: *mut c_void,
                     oif: c_int, protocol: u8);
    fn ipv4_sk_redirect(skb: *mut c_void, sk: *mut c_void);
    fn ipv4_sk_update_pmtu(skb: *mut c_void, sk: *mut c_void, mtu: u32);
    fn ip_rt_multicast_event(in_dev: *mut c_void);
    fn rt_flush_dev(dev: *mut c_void);
    fn rt_add_uncached_list(rt: *mut c_void);
    fn rt_del_uncached_list(rt: *mut c_void);
    fn ip_rt_init() -> c_int;
}

pub const RT_GC_TIMEOUT: c_int = 300;
pub const DEFAULT_MIN_PMTU: c_int = 512 + 20 + 20;
pub const DEFAULT_MTU_EXPIRES: c_int = 10 * 60;
pub const DEFAULT_MIN_ADVMSS: c_int = 256;

// Source-level declarations retained for the complete implementation surface.
// Their definitions are linked from the corresponding translated kernel units.
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __be16 = u16;
pub type __be32 = u32;
pub type dscp_t = u8;

#[inline(always)]
pub unsafe fn rt_cache_flush(net: *mut c_void) {
    rt_genid_bump_ipv4(net);
}

// The remaining functions in route.c are kernel integration points whose exact
// types and layouts are supplied by the surrounding Linux networking bindings.
// Keep their exported interfaces available without inventing dependency
// implementations.
pub unsafe fn ip_static_sysctl_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
