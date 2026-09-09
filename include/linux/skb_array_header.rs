/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/skb_array.h. */

use core::ffi::c_void;

#[repr(C)]
pub struct ptr_ring {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    pub len: i32,
}

pub type gfp_t = u32;

#[repr(C)]
pub struct skb_array {
    pub ring: ptr_ring,
}

extern "C" {
    fn __ptr_ring_full(ring: *mut ptr_ring) -> bool;
    fn ptr_ring_full(ring: *mut ptr_ring) -> bool;
    fn ptr_ring_produce(ring: *mut ptr_ring, ptr: *mut sk_buff) -> i32;
    fn ptr_ring_produce_irq(ring: *mut ptr_ring, ptr: *mut sk_buff) -> i32;
    fn ptr_ring_produce_bh(ring: *mut ptr_ring, ptr: *mut sk_buff) -> i32;
    fn ptr_ring_produce_any(ring: *mut ptr_ring, ptr: *mut sk_buff) -> i32;
    fn __ptr_ring_empty(ring: *mut ptr_ring) -> bool;
    fn __ptr_ring_peek(ring: *mut ptr_ring) -> *mut sk_buff;
    fn ptr_ring_empty(ring: *mut ptr_ring) -> bool;
    fn ptr_ring_empty_bh(ring: *mut ptr_ring) -> bool;
    fn ptr_ring_empty_irq(ring: *mut ptr_ring) -> bool;
    fn ptr_ring_empty_any(ring: *mut ptr_ring) -> bool;
    fn __ptr_ring_consume(ring: *mut ptr_ring) -> *mut sk_buff;
    fn ptr_ring_consume(ring: *mut ptr_ring) -> *mut sk_buff;
    fn ptr_ring_consume_batched(ring: *mut ptr_ring, array: *mut *mut c_void, n: i32) -> i32;
    fn ptr_ring_consume_irq(ring: *mut ptr_ring) -> *mut sk_buff;
    fn ptr_ring_consume_batched_irq(ring: *mut ptr_ring, array: *mut *mut c_void, n: i32) -> i32;
    fn ptr_ring_consume_any(ring: *mut ptr_ring) -> *mut sk_buff;
    fn ptr_ring_consume_batched_any(ring: *mut ptr_ring, array: *mut *mut c_void, n: i32) -> i32;
    fn ptr_ring_consume_bh(ring: *mut ptr_ring) -> *mut sk_buff;
    fn ptr_ring_consume_batched_bh(ring: *mut ptr_ring, array: *mut *mut c_void, n: i32) -> i32;
    fn skb_vlan_tag_present(skb: *mut sk_buff) -> bool;
    fn ptr_ring_peek_call(ring: *mut ptr_ring, f: unsafe extern "C" fn(*mut sk_buff) -> i32) -> i32;
    fn ptr_ring_peek_call_irq(ring: *mut ptr_ring, f: unsafe extern "C" fn(*mut sk_buff) -> i32) -> i32;
    fn ptr_ring_peek_call_bh(ring: *mut ptr_ring, f: unsafe extern "C" fn(*mut sk_buff) -> i32) -> i32;
    fn ptr_ring_peek_call_any(ring: *mut ptr_ring, f: unsafe extern "C" fn(*mut sk_buff) -> i32) -> i32;
    fn ptr_ring_init_noprof(ring: *mut ptr_ring, size: i32, gfp: gfp_t) -> i32;
    fn alloc_hooks_skb_array_init_noprof(a: *mut skb_array, size: i32, gfp: gfp_t) -> i32;
    fn kfree_skb(ptr: *mut c_void);
    fn ptr_ring_unconsume(ring: *mut ptr_ring, skbs: *mut *mut c_void, n: i32, destroy: unsafe extern "C" fn(*mut c_void));
    fn ptr_ring_resize(ring: *mut ptr_ring, size: i32, gfp: gfp_t, destroy: unsafe extern "C" fn(*mut c_void)) -> i32;
    fn ptr_ring_resize_multiple_bh_noprof(rings: *mut *mut ptr_ring, nrings: i32, size: u32, gfp: gfp_t, destroy: unsafe extern "C" fn(*mut c_void)) -> i32;
    fn ptr_ring_cleanup(ring: *mut ptr_ring, destroy: unsafe extern "C" fn(*mut c_void));
}

pub unsafe fn __skb_array_full(a: *mut skb_array) -> bool { __ptr_ring_full(&mut (*a).ring) }
pub unsafe fn skb_array_full(a: *mut skb_array) -> bool { ptr_ring_full(&mut (*a).ring) }
pub unsafe fn skb_array_produce(a: *mut skb_array, skb: *mut sk_buff) -> i32 { ptr_ring_produce(&mut (*a).ring, skb) }
pub unsafe fn skb_array_produce_irq(a: *mut skb_array, skb: *mut sk_buff) -> i32 { ptr_ring_produce_irq(&mut (*a).ring, skb) }
pub unsafe fn skb_array_produce_bh(a: *mut skb_array, skb: *mut sk_buff) -> i32 { ptr_ring_produce_bh(&mut (*a).ring, skb) }
pub unsafe fn skb_array_produce_any(a: *mut skb_array, skb: *mut sk_buff) -> i32 { ptr_ring_produce_any(&mut (*a).ring, skb) }
pub unsafe fn __skb_array_empty(a: *mut skb_array) -> bool { __ptr_ring_empty(&mut (*a).ring) }
pub unsafe fn __skb_array_peek(a: *mut skb_array) -> *mut sk_buff { __ptr_ring_peek(&mut (*a).ring) }
pub unsafe fn skb_array_empty(a: *mut skb_array) -> bool { ptr_ring_empty(&mut (*a).ring) }
pub unsafe fn skb_array_empty_bh(a: *mut skb_array) -> bool { ptr_ring_empty_bh(&mut (*a).ring) }
pub unsafe fn skb_array_empty_irq(a: *mut skb_array) -> bool { ptr_ring_empty_irq(&mut (*a).ring) }
pub unsafe fn skb_array_empty_any(a: *mut skb_array) -> bool { ptr_ring_empty_any(&mut (*a).ring) }
pub unsafe fn __skb_array_consume(a: *mut skb_array) -> *mut sk_buff { __ptr_ring_consume(&mut (*a).ring) }
pub unsafe fn skb_array_consume(a: *mut skb_array) -> *mut sk_buff { ptr_ring_consume(&mut (*a).ring) }
pub unsafe fn skb_array_consume_batched(a: *mut skb_array, array: *mut *mut sk_buff, n: i32) -> i32 { ptr_ring_consume_batched(&mut (*a).ring, array as *mut *mut c_void, n) }
pub unsafe fn skb_array_consume_irq(a: *mut skb_array) -> *mut sk_buff { ptr_ring_consume_irq(&mut (*a).ring) }
pub unsafe fn skb_array_consume_batched_irq(a: *mut skb_array, array: *mut *mut sk_buff, n: i32) -> i32 { ptr_ring_consume_batched_irq(&mut (*a).ring, array as *mut *mut c_void, n) }
pub unsafe fn skb_array_consume_any(a: *mut skb_array) -> *mut sk_buff { ptr_ring_consume_any(&mut (*a).ring) }
pub unsafe fn skb_array_consume_batched_any(a: *mut skb_array, array: *mut *mut sk_buff, n: i32) -> i32 { ptr_ring_consume_batched_any(&mut (*a).ring, array as *mut *mut c_void, n) }
pub unsafe fn skb_array_consume_bh(a: *mut skb_array) -> *mut sk_buff { ptr_ring_consume_bh(&mut (*a).ring) }
pub unsafe fn skb_array_consume_batched_bh(a: *mut skb_array, array: *mut *mut sk_buff, n: i32) -> i32 { ptr_ring_consume_batched_bh(&mut (*a).ring, array as *mut *mut c_void, n) }

pub unsafe extern "C" fn __skb_array_len_with_tag(skb: *mut sk_buff) -> i32 {
    if !skb.is_null() { (*skb).len + if skb_vlan_tag_present(skb) { 4 } else { 0 } } else { 0 }
}
pub unsafe fn skb_array_peek_len(a: *mut skb_array) -> i32 { ptr_ring_peek_call(&mut (*a).ring, __skb_array_len_with_tag) }
pub unsafe fn skb_array_peek_len_irq(a: *mut skb_array) -> i32 { ptr_ring_peek_call_irq(&mut (*a).ring, __skb_array_len_with_tag) }
pub unsafe fn skb_array_peek_len_bh(a: *mut skb_array) -> i32 { ptr_ring_peek_call_bh(&mut (*a).ring, __skb_array_len_with_tag) }
pub unsafe fn skb_array_peek_len_any(a: *mut skb_array) -> i32 { ptr_ring_peek_call_any(&mut (*a).ring, __skb_array_len_with_tag) }
pub unsafe fn skb_array_init_noprof(a: *mut skb_array, size: i32, gfp: gfp_t) -> i32 { ptr_ring_init_noprof(&mut (*a).ring, size, gfp) }
#[macro_export] macro_rules! skb_array_init { ($($arg:expr),* $(,)?) => { alloc_hooks_skb_array_init_noprof($($arg),*) }; }
pub unsafe extern "C" fn __skb_array_destroy_skb(ptr: *mut c_void) { kfree_skb(ptr); }
pub unsafe fn skb_array_unconsume(a: *mut skb_array, skbs: *mut *mut sk_buff, n: i32) { ptr_ring_unconsume(&mut (*a).ring, skbs as *mut *mut c_void, n, __skb_array_destroy_skb); }
pub unsafe fn skb_array_resize(a: *mut skb_array, size: i32, gfp: gfp_t) -> i32 { ptr_ring_resize(&mut (*a).ring, size, gfp, __skb_array_destroy_skb) }
pub unsafe fn skb_array_resize_multiple_bh_noprof(rings: *mut *mut skb_array, nrings: i32, size: u32, gfp: gfp_t) -> i32 { ptr_ring_resize_multiple_bh_noprof(rings as *mut *mut ptr_ring, nrings, size, gfp, __skb_array_destroy_skb) }
#[macro_export] macro_rules! skb_array_resize_multiple_bh { ($($arg:expr),* $(,)?) => { alloc_hooks_skb_array_resize_multiple_bh_noprof($($arg),*) }; }
pub unsafe fn skb_array_cleanup(a: *mut skb_array) { ptr_ring_cleanup(&mut (*a).ring, __skb_array_destroy_skb); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
