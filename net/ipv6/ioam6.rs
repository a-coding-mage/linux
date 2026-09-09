// SPDX-License-Identifier: GPL-2.0+
// IPv6 IOAM implementation.  Kernel types and helpers are supplied by other
// translation units; their declarations are intentionally left external.

#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

use core::ffi::c_void;

extern "C" {
    fn kfree_rcu(p: *mut c_void, field: usize);
    fn kfree(p: *mut c_void);
}

#[repr(C)] pub struct ioam6_namespace { pub id: u16, pub data: u32, pub data_wide: u64, pub schema: *mut ioam6_schema, pub head: [u8; 0], pub rcu: [u8; 0] }
#[repr(C)] pub struct ioam6_schema { pub id: u32, pub len: u32, pub hdr: u32, pub data: *mut u8, pub ns: *mut ioam6_namespace, pub head: [u8; 0], pub rcu: [u8; 0] }
#[repr(C)] pub struct ioam6_pernet_data { pub lock: c_void, pub namespaces: c_void, pub schemas: c_void }
#[repr(C)] pub struct sk_buff { pub len: u32, pub dev: *mut c_void }
#[repr(C)] pub struct netlink_callback { pub skb: *mut sk_buff, pub args: [u64; 8], pub nlh: *mut c_void }
#[repr(C)] pub struct genl_info { pub attrs: *mut *mut c_void }
#[repr(C)] pub struct ioam6_trace_hdr { pub namespace_id: u16, pub nodelen: u8, pub remlen: u8, pub overflow: u8, pub type_: u32, pub data: *mut u8 }
#[repr(C)] pub struct net { pub ipv6: c_void }

const EINVAL: i32 = 22; const EEXIST: i32 = 17; const ENOMEM: i32 = 12; const ENOENT: i32 = 2;
const EMSGSIZE: i32 = 90; const EAGAIN: i32 = 11;
const IOAM6_U32_UNAVAILABLE: u32 = 0xffff_ffff; const IOAM6_U64_UNAVAILABLE: u64 = 0xffff_ffff_ffff_ffff;
const IOAM6_U16_UNAVAILABLE: u16 = 0xffff;
const IOAM6_MASK_SHORT_FIELDS: u32 = 0xff1f_fc00; const IOAM6_MASK_WIDE_FIELDS: u32 = 0x00e0_0000;

extern "C" {
    fn ioam6_pernet(net: *mut net) -> *mut ioam6_pernet_data;
    fn rhashtable_lookup_fast(t: *mut c_void, key: *const c_void, p: *const c_void) -> *mut c_void;
    fn rhashtable_lookup_insert_fast(t: *mut c_void, node: *mut c_void, p: *const c_void) -> i32;
    fn rhashtable_remove_fast(t: *mut c_void, node: *mut c_void, p: *const c_void) -> i32;
    fn mutex_lock(p: *mut c_void); fn mutex_unlock(p: *mut c_void);
    fn rcu_assign_pointer(p: *mut *mut c_void, v: *mut c_void);
    fn rcu_dereference(p: *mut *mut c_void) -> *mut c_void;
    fn hweight32(v: u32) -> u32;
}

unsafe fn ioam6_ns_release(ns: *mut ioam6_namespace) { kfree_rcu(ns.cast(), 0); }
unsafe fn ioam6_sc_release(sc: *mut ioam6_schema) { kfree_rcu(sc.cast(), 0); }
unsafe fn ioam6_free_ns(ptr: *mut c_void, _arg: *mut c_void) { if !ptr.is_null() { ioam6_ns_release(ptr.cast()); } }
unsafe fn ioam6_free_sc(ptr: *mut c_void, _arg: *mut c_void) { if !ptr.is_null() { ioam6_sc_release(ptr.cast()); } }

pub unsafe fn ioam6_namespace(net: *mut net, id: u16) -> *mut ioam6_namespace {
    let d = ioam6_pernet(net); rhashtable_lookup_fast((&mut (*d).namespaces) as *mut _ as *mut c_void, (&id as *const _).cast(), core::ptr::null()).cast()
}

pub unsafe fn ioam6_trace_compute_nodelen(trace_type: u32) -> u8 {
    (hweight32(trace_type & IOAM6_MASK_SHORT_FIELDS) + 2 * hweight32(trace_type & IOAM6_MASK_WIDE_FIELDS)) as u8
}

unsafe fn fill_empty(data: &mut *mut u8, count: usize) {
    for _ in 0..count { *(*data as *mut u32) = IOAM6_U32_UNAVAILABLE.to_be(); *data = (*data).add(4); }
}

unsafe fn __ioam6_fill_trace_data(_skb: *mut sk_buff, ns: *mut ioam6_namespace, trace: *mut ioam6_trace_hdr, sc: *mut ioam6_schema, sclen: u32, _is_input: bool) {
    let mut data = (*trace).data.add((*trace).remlen as usize * 4 - (*trace).nodelen as usize * 4 - sclen as usize * 4);
    let t = (*trace).type_;
    if t & (1 << 0) != 0 { *data.cast::<u32>() = (((*ns).id as u32) << 16 | (*ns).data).to_be(); data = data.add(4); }
    if t & (1 << 1) != 0 { *data.cast::<u32>() = (IOAM6_U16_UNAVAILABLE as u32 * 0x0001_0001).to_be(); data = data.add(4); }
    if t & (1 << 2) != 0 { *data.cast::<u32>() = IOAM6_U32_UNAVAILABLE.to_be(); data = data.add(4); }
    if t & (1 << 3) != 0 { *data.cast::<u32>() = IOAM6_U32_UNAVAILABLE.to_be(); data = data.add(4); }
    if t & (1 << 4) != 0 { *data.cast::<u32>() = IOAM6_U32_UNAVAILABLE.to_be(); data = data.add(4); }
    if t & (1 << 5) != 0 { *data.cast::<u32>() = (*ns).data; data = data.add(4); }
    if t & (1 << 6) != 0 { *data.cast::<u32>() = IOAM6_U32_UNAVAILABLE.to_be(); data = data.add(4); }
    if t & (1 << 7) != 0 { *data.cast::<u32>() = IOAM6_U32_UNAVAILABLE.to_be(); data = data.add(4); }
    if t & (1 << 8) != 0 { *data.cast::<u64>() = (*ns).data_wide; data = data.add(8); }
    if t & (1 << 9) != 0 { fill_empty(&mut data, 2); }
    if t & (1 << 10) != 0 { *data.cast::<u64>() = (*ns).data_wide; data = data.add(8); }
    if t & (1 << 11) != 0 { fill_empty(&mut data, 1); }
    if t & (0x003f_f000) != 0 { fill_empty(&mut data, (hweight32(t & 0x003f_f000)) as usize); }
    if t & (1 << 22) != 0 { if sc.is_null() { *data.cast::<u32>() = (IOAM6_U32_UNAVAILABLE >> 8).to_be(); } else { *data.cast::<u32>() = (*sc).hdr; data = data.add(4); core::ptr::copy_nonoverlapping((*sc).data, data, (*sc).len as usize); } }
}

pub unsafe fn ioam6_fill_trace_data(skb: *mut sk_buff, ns: *mut ioam6_namespace, trace: *mut ioam6_trace_hdr, is_input: bool) {
    if (*trace).overflow != 0 { return; }
    let sc = rcu_dereference((&mut (*ns).schema) as *mut _ as *mut *mut c_void).cast::<ioam6_schema>();
    let mut sclen = if (*trace).type_ & (1 << 22) != 0 { 1 } else { 0 };
    if !sc.is_null() && sclen != 0 { sclen += (*sc).len / 4; }
    if (*trace).remlen == 0 || (*trace).remlen as u32 < (*trace).nodelen as u32 + sclen { (*trace).overflow = 1; return; }
    __ioam6_fill_trace_data(skb, ns, trace, sc, sclen, is_input); (*trace).remlen -= (*trace).nodelen + sclen as u8;
}

// Generic-netlink operations, namespace/schema mutation, event emission, and
// per-network initialization retain their C linkage and are supplied through
// the kernel translation layer.
extern "C" { pub fn ioam6_event(type_: u32, net: *mut net, gfp: u32, opt: *mut c_void, opt_len: u32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
