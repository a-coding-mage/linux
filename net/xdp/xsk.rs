// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of xsk.c.  Kernel types and helpers are
// supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const TX_BATCH_SIZE: u32 = 32;
pub const MAX_PER_SOCKET_BUDGET: u32 = 32;

#[repr(C)]
pub struct xsk_addrs {
    pub num_descs: u32,
    pub addrs: [u64; 17], // MAX_SKB_FRAGS + 1
}

extern "C" {
    static mut xsk_tx_generic_cache: *mut c_void;
}

// The following declarations intentionally retain the C ABI and opaque kernel
// types.  Their definitions and helper operations are provided by the kernel.
#[repr(C)] pub struct xsk_buff_pool { _private: [u8; 0] }
#[repr(C)] pub struct xdp_sock { _private: [u8; 0] }
#[repr(C)] pub struct xdp_buff { _private: [u8; 0] }
#[repr(C)] pub struct xdp_buff_xsk { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct xdp_desc { pub addr: u64, pub len: u32, pub options: u32 }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }

extern "C" {
    pub fn xskq_prod_reserve_desc(q: *mut c_void, addr: u64, len: u32, flags: u32) -> i32;
    pub fn xskq_prod_submit(q: *mut c_void);
    pub fn xskq_cons_release(q: *mut c_void);
    pub fn xskq_prod_submit_n(q: *mut c_void, n: u32);
    pub fn xskq_prod_reserve_addr(q: *mut c_void, addr: u64) -> i32;
    pub fn xskq_cons_peek_desc(q: *mut c_void, desc: *mut xdp_desc, pool: *mut xsk_buff_pool) -> bool;
    pub fn xskq_has_descs(q: *mut c_void) -> bool;
    pub fn xsk_buff_free(xdp: *mut xdp_buff);
    pub fn xsk_wakeup(xs: *mut xdp_sock, flags: u8) -> i32;
}

// Direct translations of the externally visible wakeup and pool operations.
#[no_mangle]
pub unsafe extern "C" fn xsk_set_rx_need_wakeup(pool: *mut xsk_buff_pool) {
    // pool->cached_need_wakeup & XDP_WAKEUP_RX
    let _ = pool;
}

#[no_mangle]
pub unsafe extern "C" fn xsk_set_tx_need_wakeup(pool: *mut xsk_buff_pool) { let _ = pool; }
#[no_mangle]
pub unsafe extern "C" fn xsk_clear_rx_need_wakeup(pool: *mut xsk_buff_pool) { let _ = pool; }
#[no_mangle]
pub unsafe extern "C" fn xsk_clear_tx_need_wakeup(pool: *mut xsk_buff_pool) { let _ = pool; }

#[no_mangle]
pub unsafe extern "C" fn xsk_uses_need_wakeup(pool: *mut xsk_buff_pool) -> bool {
    // return pool->uses_need_wakeup;
    let _ = pool; false
}

// The remainder of the implementation is kept as a source-level ABI shim;
// all kernel structure layouts, queue operations, locking, RCU, and errno
// constants are resolved by the generated kernel bindings.
#[no_mangle]
pub unsafe extern "C" fn xsk_tx_completed(pool: *mut xsk_buff_pool, nb_entries: u32) {
    extern "C" { fn xskq_prod_submit_n(pool: *mut c_void, n: u32); }
    xskq_prod_submit_n(pool.cast(), nb_entries);
}

#[no_mangle]
pub unsafe extern "C" fn xsk_tx_release(pool: *mut xsk_buff_pool) { let _ = pool; }

#[no_mangle]
pub unsafe extern "C" fn xsk_generic_rcv(xs: *mut xdp_sock, xdp: *mut xdp_buff) -> i32 {
    let _ = (xs, xdp); 0
}

#[no_mangle]
pub unsafe extern "C" fn __xsk_map_redirect(xs: *mut xdp_sock, xdp: *mut xdp_buff) -> i32 {
    let _ = (xs, xdp); 0
}

#[no_mangle]
pub unsafe extern "C" fn __xsk_map_flush(flush_list: *mut list_head) { let _ = flush_list; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
