// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
//
// Rust translation of testing/selftests/bpf/progs/kfree_skb.c.
// C includes provide BPF helper declarations, endian helpers, tracing macros,
// map-definition macros, and kernel integer aliases.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
pub const BPF_F_CURRENT_CPU: __u64 = 0xffff_ffff;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// Original C:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
//     __type(key, int);
//     __type(value, int);
// } perf_buf_map SEC(".maps");
#[repr(C)]
pub struct perf_buf_map_def {}

#[no_mangle]
#[link_section = ".maps"]
pub static mut perf_buf_map: perf_buf_map_def = perf_buf_map_def {};

// #define _(P) (__builtin_preserve_access_index(P))
// Field accesses below are written directly at each translated use site.

/* define few struct-s that bpf program needs to access */
#[repr(C)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn(head: *mut callback_head)>,
}

#[repr(C)]
pub struct dev_ifalias {
    pub rcuhead: callback_head,
}

#[repr(C)]
pub struct net_device {
    /* same as kernel's struct net_device */
    pub ifindex: c_int,
    pub ifalias: *mut dev_ifalias,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct refcount_struct {
    pub refs: atomic_t,
}

pub type refcount_t = refcount_struct;

#[repr(C)]
pub struct sk_buff {
    /* field names and sizes should match to those in the kernel */
    pub len: c_uint,
    pub data_len: c_uint,
    pub mac_len: __u16,
    pub hdr_len: __u16,
    pub queue_mapping: __u16,
    pub dev: *mut net_device,
    /* order of the fields doesn't matter */
    pub users: refcount_t,
    pub data: *mut c_uchar,
    pub __pkt_type_offset: [c_char; 0],
    pub cb: [c_char; 48],
}

pub type c_uint = u32;
pub type c_uchar = u8;

#[repr(C)]
pub struct meta {
    pub ifindex: c_int,
    pub cb32_0: __u32,
    pub cb8_0: __u8,
}

unsafe extern "C" {
    pub fn bpf_probe_read_kernel(dst: *mut c_void, size: __u64, unsafe_ptr: *const c_void) -> c_int;
    pub fn bpf_printk(fmt: *const c_char, ...) -> c_int;
    pub fn bpf_skb_output(
        ctx: *mut c_void,
        map: *mut c_void,
        flags: __u64,
        data: *const c_void,
        size: __u64,
    ) -> c_int;
}

#[inline]
pub const fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

/* TRACE_EVENT(kfree_skb,
 *         TP_PROTO(struct sk_buff *skb, void *location),
 */
#[no_mangle]
#[link_section = "tp_btf/kfree_skb"]
pub unsafe extern "C" fn trace_kfree_skb(skb: *mut sk_buff, location: *mut c_void) -> c_int {
    let mut dev: *mut net_device;
    let mut ptr: *mut callback_head;
    let mut func: *mut c_void;
    let mut users: c_int;
    let mut data: *mut c_uchar;
    let mut pkt_data: c_ushort = 0;
    let mut meta: meta = meta {
        ifindex: 0,
        cb32_0: 0,
        cb8_0: 0,
    };
    let mut pkt_type: c_char = 0;
    let mut cb32: *mut __u32;
    let mut cb8: *mut __u8;

    // __builtin_preserve_access_index(({ ... }))
    users = (*skb).users.refs.counter;
    data = (*skb).data;
    dev = (*skb).dev;
    ptr = (*(*dev).ifalias).rcuhead.next;
    func = (*ptr).func.map_or(core::ptr::null_mut(), |f| f as *mut c_void);
    cb8 = (*skb).cb.as_mut_ptr() as *mut __u8;
    cb32 = (*skb).cb.as_mut_ptr() as *mut __u32;

    meta.ifindex = (*dev).ifindex;
    meta.cb8_0 = *cb8.add(8);
    meta.cb32_0 = *cb32.add(2);

    bpf_probe_read_kernel(
        &mut pkt_type as *mut c_char as *mut c_void,
        core::mem::size_of_val(&pkt_type) as __u64,
        (*skb).__pkt_type_offset.as_ptr() as *const c_void,
    );
    pkt_type = ((pkt_type as __u8) & 7) as c_char;

    /* read eth proto */
    bpf_probe_read_kernel(
        &mut pkt_data as *mut c_ushort as *mut c_void,
        core::mem::size_of_val(&pkt_data) as __u64,
        data.add(12) as *const c_void,
    );

    bpf_printk(
        b"rcuhead.next %llx func %llx\n\0".as_ptr() as *const c_char,
        ptr as __u64,
        func as __u64,
    );
    bpf_printk(
        b"skb->len %d users %d pkt_type %x\n\0".as_ptr() as *const c_char,
        (*skb).len as c_int,
        users,
        pkt_type as c_int,
    );
    bpf_printk(
        b"skb->queue_mapping %d\n\0".as_ptr() as *const c_char,
        (*skb).queue_mapping as c_int,
    );
    bpf_printk(
        b"dev->ifindex %d data %llx pkt_data %x\n\0".as_ptr() as *const c_char,
        meta.ifindex,
        data as __u64,
        pkt_data as c_int,
    );
    bpf_printk(
        b"cb8_0:%x cb32_0:%x\n\0".as_ptr() as *const c_char,
        meta.cb8_0 as c_int,
        meta.cb32_0 as c_int,
    );

    if users != 1 || pkt_data != bpf_htons(0x86dd) || meta.ifindex != 1 {
        /* raw tp ignores return value */
        return 0;
    }

    /* send first 72 byte of the packet to user space */
    bpf_skb_output(
        skb as *mut c_void,
        &raw mut perf_buf_map as *mut perf_buf_map_def as *mut c_void,
        (72u64 << 32) | BPF_F_CURRENT_CPU,
        &meta as *const meta as *const c_void,
        core::mem::size_of_val(&meta) as __u64,
    );
    0
}

pub type c_ushort = u16;

#[repr(C)]
pub struct result_t {
    pub fentry_test_ok: bool,
    pub fexit_test_ok: bool,
}

#[no_mangle]
pub static mut result: result_t = result_t {
    fentry_test_ok: false,
    fexit_test_ok: false,
};

#[no_mangle]
#[link_section = "fentry/eth_type_trans"]
pub unsafe extern "C" fn fentry_eth_type_trans(
    skb: *mut sk_buff,
    dev: *mut net_device,
    protocol: c_ushort,
) -> c_int {
    let mut len: c_int;
    let mut ifindex: c_int;

    // __builtin_preserve_access_index(({ ... }))
    len = (*skb).len as c_int;
    ifindex = (*dev).ifindex;

    /* fentry sees full packet including L2 header */
    if len != 74 || ifindex != 1 {
        return 0;
    }
    result.fentry_test_ok = true;
    0
}

#[no_mangle]
#[link_section = "fexit/eth_type_trans"]
pub unsafe extern "C" fn fexit_eth_type_trans(
    skb: *mut sk_buff,
    dev: *mut net_device,
    protocol: c_ushort,
) -> c_int {
    let mut len: c_int;
    let mut ifindex: c_int;

    // __builtin_preserve_access_index(({ ... }))
    len = (*skb).len as c_int;
    ifindex = (*dev).ifindex;

    /* fexit sees packet without L2 header that eth_type_trans should have
     * consumed.
     */
    if len != 60 || protocol != bpf_htons(0x86dd) || ifindex != 1 {
        return 0;
    }
    result.fexit_test_ok = true;
    0
}
