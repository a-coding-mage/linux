// SPDX-License-Identifier: GPL-2.0-or-later
/* Socket buffer accounting
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the Linux kernel and rxrpc headers.

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rxrpc_skb_trace {
    rxrpc_skb_put_purge,
}

extern "C" {
    static mut rxrpc_n_rx_skbs: atomic_t;

    fn atomic_inc_return(v: *mut atomic_t) -> ::std::os::raw::c_int;
    fn atomic_read(v: *const atomic_t) -> ::std::os::raw::c_int;
    fn atomic_dec_return(v: *mut atomic_t) -> ::std::os::raw::c_int;
    fn refcount_read(r: *const refcount_t) -> ::std::os::raw::c_uint;
    fn trace_rxrpc_skb(
        skb: *mut sk_buff,
        users: ::std::os::raw::c_uint,
        n: ::std::os::raw::c_int,
        why: rxrpc_skb_trace,
    );
    fn skb_get(skb: *mut sk_buff);
    fn consume_skb(skb: *mut sk_buff);
    fn skb_dequeue(list: *mut sk_buff_head) -> *mut sk_buff;
}

// select_skb_count(skb) expands to (&rxrpc_n_rx_skbs).
#[inline]
unsafe fn select_skb_count(_skb: *mut sk_buff) -> *mut atomic_t {
    &raw mut rxrpc_n_rx_skbs
}

/*
 * Note the allocation or reception of a socket buffer.
 */
pub unsafe fn rxrpc_new_skb(skb: *mut sk_buff, why: rxrpc_skb_trace) {
    let n: ::std::os::raw::c_int = atomic_inc_return(select_skb_count(skb));
    trace_rxrpc_skb(skb, refcount_read(skb_users(skb)), n, why);
}

/*
 * Note the re-emergence of a socket buffer from a queue or buffer.
 */
pub unsafe fn rxrpc_see_skb(skb: *mut sk_buff, why: rxrpc_skb_trace) {
    if !skb.is_null() {
        let n: ::std::os::raw::c_int = atomic_read(select_skb_count(skb));
        trace_rxrpc_skb(skb, refcount_read(skb_users(skb)), n, why);
    }
}

/*
 * Note the addition of a ref on a socket buffer.
 */
pub unsafe fn rxrpc_get_skb(skb: *mut sk_buff, why: rxrpc_skb_trace) {
    let n: ::std::os::raw::c_int = atomic_inc_return(select_skb_count(skb));
    trace_rxrpc_skb(skb, refcount_read(skb_users(skb)), n, why);
    skb_get(skb);
}

/*
 * Note the destruction of a socket buffer.
 */
pub unsafe fn rxrpc_free_skb(skb: *mut sk_buff, why: rxrpc_skb_trace) {
    if !skb.is_null() {
        let n: ::std::os::raw::c_int = atomic_dec_return(select_skb_count(skb));
        trace_rxrpc_skb(skb, refcount_read(skb_users(skb)), n, why);
        consume_skb(skb);
    }
}

/*
 * Clear a queue of socket buffers.
 */
pub unsafe fn rxrpc_purge_queue(list: *mut sk_buff_head) {
    let mut skb: *mut sk_buff;

    loop {
        skb = skb_dequeue(list);
        if skb.is_null() {
            break;
        }
        let n: ::std::os::raw::c_int = atomic_dec_return(select_skb_count(skb));
        trace_rxrpc_skb(skb, refcount_read(skb_users(skb)), n, rxrpc_skb_put_purge);
        consume_skb(skb);
    }
}

// Field access supplied by struct sk_buff from the kernel headers.
extern "C" {
    fn skb_users(skb: *mut sk_buff) -> *const refcount_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
