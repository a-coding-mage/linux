// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC Tx data buffering.
 *
 * Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// The declarations below are supplied by the surrounding kernel translation.

use core::ffi::c_void;
use core::sync::atomic::{AtomicI32, Ordering};

type SizeT = usize;
type GfpT = u32;
type RxrpcSeqT = u32;

#[repr(C)]
pub struct rxrpc_txbuf {
    pub ref_: Refcount,
    pub call_debug_id: u32,
    pub debug_id: u32,
    pub alloc_size: SizeT,
    pub space: SizeT,
    pub offset: SizeT,
    pub flags: u32,
    pub seq: RxrpcSeqT,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct rxrpc_call {
    pub conn: *mut rxrpc_connection,
    pub debug_id: u32,
    pub send_top: RxrpcSeqT,
}

#[repr(C)]
pub struct rxrpc_connection {
    pub tx_data_alloc: page_frag_cache,
    pub tx_data_alloc_lock: Mutex,
    pub out_clientflag: u32,
}

#[repr(C)]
pub struct Refcount {
    pub value: AtomicI32,
}

#[repr(C)]
pub struct Mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page_frag_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rxrpc_jumbo_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    pub value: AtomicI32,
}

#[repr(C)]
pub enum rxrpc_txbuf_trace {
    _Opaque,
}

const L1_CACHE_BYTES: SizeT = 0; // Supplied by the target kernel configuration.

const RXRPC_TXBUF_ALLOC_DATA: rxrpc_txbuf_trace = rxrpc_txbuf_trace::_Opaque;
const RXRPC_TXBUF_FREE: rxrpc_txbuf_trace = rxrpc_txbuf_trace::_Opaque;

extern "C" {
    static rxrpc_txbuf_debug_ids: atomic_t;
    static mut rxrpc_nr_txbuf: atomic_t;

    fn kzalloc_obj<T>(gfp: GfpT) -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn round_up(value: SizeT, align: SizeT) -> SizeT;
    fn umax(a: SizeT, b: SizeT) -> SizeT;
    fn mutex_lock(lock: *mut Mutex);
    fn mutex_unlock(lock: *mut Mutex);
    fn page_frag_alloc_align(
        cache: *mut page_frag_cache,
        size: SizeT,
        gfp: GfpT,
        align: SizeT,
    ) -> *mut c_void;
    fn page_frag_free(ptr: *mut c_void);
    fn refcount_set(r: *mut Refcount, value: i32);
    fn refcount_read(r: *const Refcount) -> i32;
    fn __refcount_dec_and_test(r: *mut Refcount, result: *mut i32) -> bool;
    fn atomic_inc_return(v: *const atomic_t) -> i32;
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec(v: *mut atomic_t);
    fn trace_rxrpc_txbuf(
        debug_id: u32,
        call_debug_id: u32,
        seq: RxrpcSeqT,
        refs: i32,
        what: rxrpc_txbuf_trace,
    );
}

static mut RXRPC_TXBUF_DEBUG_IDS: atomic_t = atomic_t { value: AtomicI32::new(0) };

/*
 * Allocate and partially initialise a data transmission buffer.
 */
pub unsafe fn rxrpc_alloc_data_txbuf(
    call: *mut rxrpc_call,
    data_size: SizeT,
    mut data_align: SizeT,
    gfp: GfpT,
) -> *mut rxrpc_txbuf {
    let txb = kzalloc_obj::<rxrpc_txbuf>(gfp);
    if txb.is_null() {
        return core::ptr::null_mut();
    }

    /* We put a jumbo header in the buffer, but not a full wire header to
     * avoid delayed-corruption problems with zerocopy.
     */
    let jsize = core::mem::size_of::<rxrpc_jumbo_header>();
    let doff = round_up(jsize, data_align);
    let total = doff + data_size;

    data_align = umax(data_align, L1_CACHE_BYTES);
    mutex_lock(&mut (*(*call).conn).tx_data_alloc_lock);
    let buf = page_frag_alloc_align(
        &mut (*(*call).conn).tx_data_alloc,
        total,
        gfp,
        data_align,
    );
    mutex_unlock(&mut (*(*call).conn).tx_data_alloc_lock);
    if buf.is_null() {
        kfree(txb.cast());
        return core::ptr::null_mut();
    }

    refcount_set(&mut (*txb).ref_, 1);
    (*txb).call_debug_id = (*call).debug_id;
    (*txb).debug_id = atomic_inc_return(&rxrpc_txbuf_debug_ids) as u32;
    (*txb).alloc_size = data_size;
    (*txb).space = data_size;
    (*txb).offset = 0;
    (*txb).flags = (*(*call).conn).out_clientflag;
    (*txb).seq = (*call).send_top + 1;
    (*txb).data = buf.add(doff);

    trace_rxrpc_txbuf((*txb).debug_id, (*txb).call_debug_id, (*txb).seq, 1, RXRPC_TXBUF_ALLOC_DATA);
    atomic_inc(&mut rxrpc_nr_txbuf);
    txb
}

pub unsafe fn rxrpc_see_txbuf(txb: *mut rxrpc_txbuf, what: rxrpc_txbuf_trace) {
    let r = refcount_read(&(*txb).ref_);
    trace_rxrpc_txbuf((*txb).debug_id, (*txb).call_debug_id, (*txb).seq, r, what);
}

unsafe fn rxrpc_free_txbuf(txb: *mut rxrpc_txbuf) {
    trace_rxrpc_txbuf((*txb).debug_id, (*txb).call_debug_id, (*txb).seq, 0, RXRPC_TXBUF_FREE);
    if !(*txb).data.is_null() {
        page_frag_free((*txb).data);
    }
    kfree(txb.cast());
    atomic_dec(&mut rxrpc_nr_txbuf);
}

pub unsafe fn rxrpc_put_txbuf(txb: *mut rxrpc_txbuf, what: rxrpc_txbuf_trace) {
    if !txb.is_null() {
        let debug_id = (*txb).debug_id;
        let call_debug_id = (*txb).call_debug_id;
        let seq = (*txb).seq;
        let mut r = 0;
        let dead = __refcount_dec_and_test(&mut (*txb).ref_, &mut r);
        trace_rxrpc_txbuf(debug_id, call_debug_id, seq, r - 1, what);
        if dead {
            rxrpc_free_txbuf(txb);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
