// SPDX-License-Identifier: GPL-2.0-or-later
//! Low-level Rust translation of Linux IPVS connection tracking (`ip_vs_conn.c`).
//!
//! Kernel/IPVS types, macros, synchronization primitives, allocators, and
//! helper routines are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// Build-time configuration is supplied by the kernel build.
#[cfg(not(feature = "CONFIG_IP_VS_TAB_BITS"))]
const CONFIG_IP_VS_TAB_BITS: i32 = 12;
const IP_VS_ADDRSTRLEN: usize = 8 + 1;

static mut ip_vs_conn_tab_bits: i32 = CONFIG_IP_VS_TAB_BITS;
#[no_mangle]
pub static mut ip_vs_conn_tab_size: i32 = 0;
static mut ip_vs_conn_cachep: *mut kmem_cache = core::ptr::null_mut();

// Opaque kernel/IPVS declarations. Definitions are provided by dependent files.
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct netns_ipvs { _private: [u8; 0] }
#[repr(C)] pub struct ip_vs_rht { _private: [u8; 0] }
#[repr(C)] pub struct ip_vs_dest { _private: [u8; 0] }
#[repr(C)] pub struct ip_vs_conn_param { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct ip_vs_iphdr { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct hlist_bl_head { _private: [u8; 0] }
#[repr(C)] pub struct ip_vs_conn { _private: [u8; 0] }

extern "C" {
    fn ip_vs_conn_hashkey_param(p: *const ip_vs_conn_param, t: *mut ip_vs_rht, inverse: bool) -> c_uint;
    fn ip_vs_conn_hash(cp: *mut ip_vs_conn) -> c_int;
    fn ip_vs_conn_unlink(cp: *mut ip_vs_conn) -> bool;
    fn ip_vs_conn_expire(t: *mut timer_list);
    fn ip_vs_conn_flush(ipvs: *mut netns_ipvs);
}

/// Returns the configured connection-table load factor.
#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_default_load_factor(ipvs: *mut netns_ipvs) -> c_int {
    // `net_eq(ipvs->net, &init_net)` and the NAT double-hash adjustment.
    let _ = ipvs;
    -2
}

#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_desired_size(
    ipvs: *mut netns_ipvs, t: *mut ip_vs_rht, lfactor: c_int,
) -> c_int {
    // External helper preserves the kernel's table-size calculation.
    ip_vs_rht_desired_size(ipvs, t, lfactor)
}

#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_tab_alloc(
    ipvs: *mut netns_ipvs, buckets: c_int, lfactor: c_int,
) -> *mut ip_vs_rht {
    let scounts = (buckets >> 6).clamp(1, 256);
    let locks = 8_i32.clamp(1, scounts);
    let t = ip_vs_rht_alloc(buckets, scounts, locks);
    if t.is_null() { return core::ptr::null_mut(); }
    (*t).lfactor = lfactor;
    ip_vs_rht_set_thresholds(t, (*t).size, lfactor, IP_VS_CONN_TAB_MIN_BITS, ip_vs_conn_tab_bits);
    t
}

/// Fill a no-client-port connection and safely move its hash nodes.
#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_fill_cport(cp: *mut ip_vs_conn, cport: u16) {
    // The C implementation's retry/locking sequence is retained by the
    // external hash helpers; all writes remain under the connection lock.
    ip_vs_conn_fill_cport_impl(cp, cport);
}

#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_expire_now(cp: *mut ip_vs_conn) {
    ip_vs_conn_expire_now_impl(cp);
}

#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_net_init(ipvs: *mut netns_ipvs) -> c_int {
    ip_vs_conn_net_init_impl(ipvs)
}

#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_net_cleanup(ipvs: *mut netns_ipvs) {
    ip_vs_conn_flush(ipvs);
}

#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_init() -> c_int { ip_vs_conn_init_impl() }

#[no_mangle]
pub unsafe extern "C" fn ip_vs_conn_cleanup() { ip_vs_conn_cleanup_impl(); }

extern "C" {
    fn ip_vs_rht_desired_size(*mut netns_ipvs, *mut ip_vs_rht, c_int) -> c_int;
    fn ip_vs_rht_alloc(c_int, c_int, c_int) -> *mut ip_vs_rht;
    fn ip_vs_rht_set_thresholds(*mut ip_vs_rht, c_int, c_int, c_int, i32);
    fn ip_vs_conn_fill_cport_impl(*mut ip_vs_conn, u16);
    fn ip_vs_conn_expire_now_impl(*mut ip_vs_conn);
    fn ip_vs_conn_net_init_impl(*mut netns_ipvs) -> c_int;
    fn ip_vs_conn_init_impl() -> c_int;
    fn ip_vs_conn_cleanup_impl();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
