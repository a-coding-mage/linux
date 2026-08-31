// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// <vmlinux.h>, <bpf/bpf_endian.h>, <bpf/bpf_helpers.h>, <errno.h>,
// "bpf_kfuncs.h", "bpf_tracing_net.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_void};

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;

const META_SIZE: usize = 32;

extern "C" {
    static mut test_pass: bool;

    static BPF_STDERR: *mut c_void;

    fn bpf_stream_printk(stream: *mut c_void, fmt: *const c_char, ...) -> i32;
    fn bpf_dynptr_from_skb_meta(ctx: *mut __sk_buff, flags: __u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_from_skb(ctx: *mut __sk_buff, flags: __u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_read(dst: *mut c_void, len: __u32, src: *const bpf_dynptr, offset: __u32, flags: __u64) -> i32;
    fn bpf_dynptr_write(dst: *mut bpf_dynptr, offset: __u32, src: *const c_void, len: __u32, flags: __u64) -> i32;
    fn bpf_dynptr_slice(ptr: *const bpf_dynptr, offset: __u32, buffer__opt: *mut c_void, len: __u32) -> *mut __u8;
    fn bpf_dynptr_slice_rdwr(ptr: *mut bpf_dynptr, offset: __u32, buffer__opt: *mut c_void, len: __u32) -> *mut __u8;
    fn bpf_dynptr_adjust(ptr: *mut bpf_dynptr, start: __u32, end: __u32) -> i32;
    fn bpf_dynptr_size(ptr: *const bpf_dynptr) -> __u32;
    fn bpf_dynptr_is_rdonly(ptr: *const bpf_dynptr) -> bool;
    fn bpf_skb_load_bytes(ctx: *const __sk_buff, offset: __u32, to: *mut c_void, len: __u32) -> i32;
    fn bpf_xdp_get_buff_len(ctx: *mut xdp_md) -> __u32;
    fn bpf_xdp_load_bytes(ctx: *mut xdp_md, offset: __u32, buf: *mut c_void, len: __u32) -> i32;
    fn bpf_xdp_adjust_meta(ctx: *mut xdp_md, delta: i32) -> i32;
    fn bpf_skb_change_head(ctx: *mut __sk_buff, len: __u32, flags: __u64) -> i32;
    fn bpf_skb_vlan_push(ctx: *mut __sk_buff, vlan_proto: __u16, vlan_tci: __u16) -> i32;
    fn bpf_skb_vlan_pop(ctx: *mut __sk_buff) -> i32;
    fn bpf_skb_adjust_room(ctx: *mut __sk_buff, len_diff: i32, mode: __u32, flags: __u64) -> i32;
    fn bpf_skb_change_tail(ctx: *mut __sk_buff, len: __u32, flags: __u64) -> i32;
    fn bpf_skb_change_proto(ctx: *mut __sk_buff, proto: __u16, flags: __u64) -> i32;
    fn bpf_htons(x: __u16) -> __u16;
}

type __u64 = u64;

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
}

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; 6],
    pub h_source: [__u8; 6],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct ipv6hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

const TC_ACT_UNSPEC: i32 = -1;
const TC_ACT_OK: i32 = 0;
const TC_ACT_SHOT: i32 = 2;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const BPF_OK: i32 = 0;
const BPF_DROP: i32 = 2;
const BPF_ADJ_ROOM_MAC: __u32 = 1;
const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86DD;
const E2BIG: i32 = 7;
const ERANGE: i32 = 34;

#[inline]
unsafe fn ctx_ptr<T>(v: __u32) -> *mut T {
    v as usize as *mut T
}

static meta_want: [__u8; META_SIZE] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
];

unsafe fn __builtin_memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32 {
    let a = a as *const u8;
    let b = b as *const u8;
    let mut i = 0usize;
    while i < n {
        let av = *a.add(i);
        let bv = *b.add(i);
        if av != bv {
            return av as i32 - bv as i32;
        }
        i += 1;
    }
    0
}

unsafe fn __builtin_memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n);
    dst
}

unsafe fn __builtin_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    core::ptr::write_bytes(dst as *mut u8, c as u8, n);
    dst
}

static FAIL_FMT: &[u8] = b"FAIL:%s:%d: metadata mismatch\n  have:\n    %pI6\n    %pI6\n  want:\n    %pI6\n    %pI6\n\0";
static FILE: &[u8] = b"test_xdp_meta.c\0";

unsafe fn check_metadata_at(file: *const c_char, line: i32, meta_have: *mut __u8) -> bool {
    if __builtin_memcmp(meta_have as *const c_void, meta_want.as_ptr() as *const c_void, META_SIZE) == 0 {
        return true;
    }

    bpf_stream_printk(
        BPF_STDERR,
        FAIL_FMT.as_ptr() as *const c_char,
        file,
        line,
        meta_have.add(0x00),
        meta_have.add(0x10),
        meta_want.as_ptr().add(0x00),
        meta_want.as_ptr().add(0x10),
    );
    false
}

unsafe fn check_metadata(meta_have: *mut __u8) -> bool {
    check_metadata_at(FILE.as_ptr() as *const c_char, line!() as i32, meta_have)
}

unsafe fn check_skb_metadata_at(file: *const c_char, line: i32, skb: *mut __sk_buff) -> bool {
    let data_meta: *mut __u8 = ctx_ptr((*skb).data_meta);
    let data: *mut __u8 = ctx_ptr((*skb).data);

    data_meta.add(META_SIZE) <= data && check_metadata_at(file, line, data_meta)
}

unsafe fn check_skb_metadata(skb: *mut __sk_buff) -> bool {
    check_skb_metadata_at(FILE.as_ptr() as *const c_char, line!() as i32, skb)
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls(ctx: *mut __sk_buff) -> i32 {
    let meta_have: *mut __u8 = ctx_ptr((*ctx).data_meta);
    let data: *mut __u8 = ctx_ptr((*ctx).data);

    if meta_have.add(META_SIZE) > data {
        return TC_ACT_SHOT;
    }

    if !check_metadata(meta_have) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

/* Read from metadata using bpf_dynptr_read helper */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls_dynptr_read(ctx: *mut __sk_buff) -> i32 {
    let mut meta_have = [0u8; META_SIZE];
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    bpf_dynptr_read(meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32, meta.as_ptr(), 0, 0);

    if !check_metadata(meta_have.as_mut_ptr()) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

/* Write to metadata using bpf_dynptr_write helper */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls_dynptr_write(ctx: *mut __sk_buff) -> i32 {
    let mut data = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    bpf_dynptr_from_skb(ctx, 0, data.as_mut_ptr());
    let src = bpf_dynptr_slice(data.as_ptr(), core::mem::size_of::<ethhdr>() as __u32, core::ptr::null_mut(), META_SIZE as __u32);
    if src.is_null() {
        return TC_ACT_SHOT;
    }

    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    bpf_dynptr_write(meta.as_mut_ptr(), 0, src as *const c_void, META_SIZE as __u32, 0);

    TC_ACT_UNSPEC /* pass */
}

/* Read from metadata using read-only dynptr slice */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls_dynptr_slice(ctx: *mut __sk_buff) -> i32 {
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    let meta_have = bpf_dynptr_slice(meta.as_ptr(), 0, core::ptr::null_mut(), META_SIZE as __u32);
    if meta_have.is_null() {
        return TC_ACT_SHOT;
    }

    if !check_metadata(meta_have) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

/* Write to metadata using writeable dynptr slice */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls_dynptr_slice_rdwr(ctx: *mut __sk_buff) -> i32 {
    let mut data = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    bpf_dynptr_from_skb(ctx, 0, data.as_mut_ptr());
    let src = bpf_dynptr_slice(data.as_ptr(), core::mem::size_of::<ethhdr>() as __u32, core::ptr::null_mut(), META_SIZE as __u32);
    if src.is_null() {
        return TC_ACT_SHOT;
    }

    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    let dst = bpf_dynptr_slice_rdwr(meta.as_mut_ptr(), 0, core::ptr::null_mut(), META_SIZE as __u32);
    if dst.is_null() {
        return TC_ACT_SHOT;
    }

    __builtin_memcpy(dst as *mut c_void, src as *const c_void, META_SIZE);
    TC_ACT_UNSPEC /* pass */
}

/* Read skb metadata in chunks from various offsets in different ways. */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls_dynptr_offset_rd(ctx: *mut __sk_buff) -> i32 {
    let chunk_len: __u32 = (META_SIZE / 4) as __u32;
    let mut meta_have = [0u8; META_SIZE];
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut dst = meta_have.as_mut_ptr();

    /* 1. Regular read */
    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    bpf_dynptr_read(dst as *mut c_void, chunk_len, meta.as_ptr(), 0, 0);
    dst = dst.add(chunk_len as usize);

    /* 2. Read from an offset-adjusted dynptr */
    bpf_dynptr_adjust(meta.as_mut_ptr(), chunk_len, bpf_dynptr_size(meta.as_ptr()));
    bpf_dynptr_read(dst as *mut c_void, chunk_len, meta.as_ptr(), 0, 0);
    dst = dst.add(chunk_len as usize);

    /* 3. Read at an offset */
    bpf_dynptr_read(dst as *mut c_void, chunk_len, meta.as_ptr(), chunk_len, 0);
    dst = dst.add(chunk_len as usize);

    /* 4. Read from a slice starting at an offset */
    let src = bpf_dynptr_slice(meta.as_ptr(), 2 * chunk_len, core::ptr::null_mut(), chunk_len);
    if src.is_null() {
        return TC_ACT_SHOT;
    }
    __builtin_memcpy(dst as *mut c_void, src as *const c_void, chunk_len as usize);

    if !check_metadata(meta_have.as_mut_ptr()) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

/* Write skb metadata in chunks at various offsets in different ways. */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls_dynptr_offset_wr(ctx: *mut __sk_buff) -> i32 {
    let chunk_len: __u32 = (META_SIZE / 4) as __u32;
    let mut payload = [0u8; META_SIZE];
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    bpf_skb_load_bytes(ctx, core::mem::size_of::<ethhdr>() as __u32, payload.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&payload) as __u32);
    let mut src = payload.as_mut_ptr();

    /* 1. Regular write */
    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    bpf_dynptr_write(meta.as_mut_ptr(), 0, src as *const c_void, chunk_len, 0);
    src = src.add(chunk_len as usize);

    /* 2. Write to an offset-adjusted dynptr */
    bpf_dynptr_adjust(meta.as_mut_ptr(), chunk_len, bpf_dynptr_size(meta.as_ptr()));
    bpf_dynptr_write(meta.as_mut_ptr(), 0, src as *const c_void, chunk_len, 0);
    src = src.add(chunk_len as usize);

    /* 3. Write at an offset */
    bpf_dynptr_write(meta.as_mut_ptr(), chunk_len, src as *const c_void, chunk_len, 0);
    src = src.add(chunk_len as usize);

    /* 4. Write to a slice starting at an offset */
    let dst = bpf_dynptr_slice_rdwr(meta.as_mut_ptr(), 2 * chunk_len, core::ptr::null_mut(), chunk_len);
    if dst.is_null() {
        return TC_ACT_SHOT;
    }
    __builtin_memcpy(dst as *mut c_void, src as *const c_void, chunk_len as usize);

    TC_ACT_UNSPEC /* pass */
}

/* Pass an OOB offset to dynptr read, write, adjust, slice. */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ing_cls_dynptr_offset_oob(ctx: *mut __sk_buff) -> i32 {
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut md: __u8 = 0;

    let mut err = bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    if err != 0 {
        return TC_ACT_SHOT;
    }

    /* read offset OOB */
    err = bpf_dynptr_read(&mut md as *mut _ as *mut c_void, core::mem::size_of_val(&md) as __u32, meta.as_ptr(), META_SIZE as __u32, 0);
    if err != -E2BIG {
        return TC_ACT_SHOT;
    }

    /* write offset OOB */
    err = bpf_dynptr_write(meta.as_mut_ptr(), META_SIZE as __u32, &md as *const _ as *const c_void, core::mem::size_of_val(&md) as __u32, 0);
    if err != -E2BIG {
        return TC_ACT_SHOT;
    }

    /* adjust end offset OOB */
    err = bpf_dynptr_adjust(meta.as_mut_ptr(), 0, (META_SIZE + 1) as __u32);
    if err != -ERANGE {
        return TC_ACT_SHOT;
    }

    /* adjust start offset OOB */
    err = bpf_dynptr_adjust(meta.as_mut_ptr(), (META_SIZE + 1) as __u32, (META_SIZE + 1) as __u32);
    if err != -ERANGE {
        return TC_ACT_SHOT;
    }

    /* slice offset OOB */
    let mut p = bpf_dynptr_slice(meta.as_ptr(), META_SIZE as __u32, core::ptr::null_mut(), core::mem::size_of::<*mut __u8>() as __u32);
    if !p.is_null() {
        return TC_ACT_SHOT;
    }

    /* slice rdwr offset OOB */
    p = bpf_dynptr_slice_rdwr(meta.as_mut_ptr(), META_SIZE as __u32, core::ptr::null_mut(), core::mem::size_of::<*mut __u8>() as __u32);
    if !p.is_null() {
        return TC_ACT_SHOT;
    }

    TC_ACT_UNSPEC
}

/* Test packets carry test metadata pattern as payload. */
unsafe fn is_test_packet_xdp(ctx: *mut xdp_md) -> bool {
    let mut meta_have = [0u8; META_SIZE];

    let len = bpf_xdp_get_buff_len(ctx);
    if len < META_SIZE as __u32 {
        return false;
    }
    if bpf_xdp_load_bytes(ctx, len - META_SIZE as __u32, meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32) != 0 {
        return false;
    }
    if __builtin_memcmp(meta_have.as_ptr() as *const c_void, meta_want.as_ptr() as *const c_void, META_SIZE) != 0 {
        return false;
    }

    true
}

/* Test packets carry test metadata pattern as payload. */
unsafe fn is_test_packet_tc(ctx: *mut __sk_buff) -> bool {
    let mut meta_have = [0u8; META_SIZE];

    if (*ctx).len < META_SIZE as __u32 {
        return false;
    }
    if bpf_skb_load_bytes(ctx, (*ctx).len - META_SIZE as __u32, meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32) != 0 {
        return false;
    }
    if __builtin_memcmp(meta_have.as_ptr() as *const c_void, meta_want.as_ptr() as *const c_void, META_SIZE) != 0 {
        return false;
    }

    true
}

/* Reserve and clear space for metadata but don't populate it */
// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn ing_xdp_zalloc_meta(ctx: *mut xdp_md) -> i32 {
    /* Drop any non-test packets */
    if !is_test_packet_xdp(ctx) {
        return XDP_DROP;
    }

    let ret = bpf_xdp_adjust_meta(ctx, -(META_SIZE as i32));
    if ret < 0 {
        return XDP_DROP;
    }

    let meta: *mut __u8 = ctx_ptr((*ctx).data_meta);
    if meta.add(META_SIZE) > ctx_ptr((*ctx).data) {
        return XDP_DROP;
    }

    __builtin_memset(meta as *mut c_void, 0, META_SIZE);
    XDP_PASS
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn ing_xdp(ctx: *mut xdp_md) -> i32 {
    /* Drop any non-test packets */
    if !is_test_packet_xdp(ctx) {
        return XDP_DROP;
    }

    let ret = bpf_xdp_adjust_meta(ctx, -(META_SIZE as i32));
    if ret < 0 {
        return XDP_DROP;
    }

    let data_meta: *mut __u8 = ctx_ptr((*ctx).data_meta);
    let data: *mut __u8 = ctx_ptr((*ctx).data);

    if data_meta.add(META_SIZE) > data {
        return XDP_DROP;
    }

    __builtin_memcpy(data_meta as *mut c_void, meta_want.as_ptr() as *const c_void, META_SIZE);
    XDP_PASS
}

/*
 * Check that, when operating on a cloned packet, skb->data_meta..skb->data is
 * kept intact if prog writes to packet _payload_ using packet pointers.
 */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn clone_data_meta_survives_data_write(ctx: *mut __sk_buff) -> i32 {
    let meta_have: *mut __u8 = ctx_ptr((*ctx).data_meta);
    let eth: *mut ethhdr = ctx_ptr((*ctx).data);

    if eth.add(1) as *mut __u8 > ctx_ptr((*ctx).data_end) {
        return TC_ACT_SHOT;
    }
    /* Ignore non-test packets */
    if !is_test_packet_tc(ctx) {
        return TC_ACT_SHOT;
    }

    if meta_have.add(META_SIZE) > eth as *mut __u8 {
        return TC_ACT_SHOT;
    }

    if !check_metadata(meta_have) {
        return TC_ACT_SHOT;
    }

    /* Packet write to trigger unclone in prologue */
    (*eth).h_proto = 42;

    test_pass = true;
    TC_ACT_SHOT
}

/*
 * Check that, when operating on a cloned packet, skb->data_meta..skb->data is
 * kept intact if prog writes to packet _metadata_ using packet pointers.
 */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn clone_data_meta_survives_meta_write(ctx: *mut __sk_buff) -> i32 {
    let meta_have: *mut __u8 = ctx_ptr((*ctx).data_meta);
    let eth: *mut ethhdr = ctx_ptr((*ctx).data);

    if eth.add(1) as *mut __u8 > ctx_ptr((*ctx).data_end) {
        return TC_ACT_SHOT;
    }
    /* Ignore non-test packets */
    if !is_test_packet_tc(ctx) {
        return TC_ACT_SHOT;
    }

    if meta_have.add(META_SIZE) > eth as *mut __u8 {
        return TC_ACT_SHOT;
    }

    if !check_metadata(meta_have) {
        return TC_ACT_SHOT;
    }

    /* Metadata write to trigger unclone in prologue */
    *meta_have = 42;

    test_pass = true;
    TC_ACT_SHOT
}

/*
 * Check that, when operating on a cloned packet, metadata remains intact if
 * prog creates a r/w slice to packet _payload_.
 */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn clone_meta_dynptr_survives_data_slice_write(ctx: *mut __sk_buff) -> i32 {
    let mut data = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut meta_have = [0u8; META_SIZE];

    bpf_dynptr_from_skb(ctx, 0, data.as_mut_ptr());
    let eth = bpf_dynptr_slice_rdwr(data.as_mut_ptr(), 0, core::ptr::null_mut(), core::mem::size_of::<ethhdr>() as __u32) as *mut ethhdr;
    if eth.is_null() {
        return TC_ACT_SHOT;
    }
    /* Ignore non-test packets */
    if !is_test_packet_tc(ctx) {
        return TC_ACT_SHOT;
    }

    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    bpf_dynptr_read(meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32, meta.as_ptr(), 0, 0);
    if !check_metadata(meta_have.as_mut_ptr()) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

/*
 * Check that, when operating on a cloned packet, metadata remains intact if
 * prog creates an r/w slice to packet _metadata_.
 */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn clone_meta_dynptr_survives_meta_slice_write(ctx: *mut __sk_buff) -> i32 {
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    /* Ignore non-test packets */
    if !is_test_packet_tc(ctx) {
        return TC_ACT_SHOT;
    }

    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    let meta_have = bpf_dynptr_slice_rdwr(meta.as_mut_ptr(), 0, core::ptr::null_mut(), META_SIZE as __u32);
    if meta_have.is_null() {
        return TC_ACT_SHOT;
    }

    if !check_metadata(meta_have) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

/*
 * Check that, when operating on a cloned packet, skb_meta dynptr is read-write
 * before prog writes to packet _payload_ using dynptr_write helper and metadata
 * remains intact before and after the write.
 */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn clone_meta_dynptr_rw_before_data_dynptr_write(ctx: *mut __sk_buff) -> i32 {
    let mut data = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut meta_have = [0u8; META_SIZE];

    /* Ignore non-test packets */
    if !is_test_packet_tc(ctx) {
        return TC_ACT_SHOT;
    }

    /* Expect read-write metadata before unclone */
    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    if bpf_dynptr_is_rdonly(meta.as_ptr()) {
        return TC_ACT_SHOT;
    }

    let mut err = bpf_dynptr_read(meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32, meta.as_ptr(), 0, 0);
    if err != 0 || !check_metadata(meta_have.as_mut_ptr()) {
        return TC_ACT_SHOT;
    }

    /* Helper write to payload will unclone the packet */
    bpf_dynptr_from_skb(ctx, 0, data.as_mut_ptr());
    bpf_dynptr_write(data.as_mut_ptr(), core::mem::offset_of!(ethhdr, h_proto) as __u32, b"x\0".as_ptr() as *const c_void, 1, 0);

    err = bpf_dynptr_read(meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32, meta.as_ptr(), 0, 0);
    if err != 0 || !check_metadata(meta_have.as_mut_ptr()) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

/*
 * Check that, when operating on a cloned packet, skb_meta dynptr is read-write
 * before prog writes to packet _metadata_ using dynptr_write helper and
 * metadata remains intact before and after the write.
 */
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn clone_meta_dynptr_rw_before_meta_dynptr_write(ctx: *mut __sk_buff) -> i32 {
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut meta_have = [0u8; META_SIZE];

    /* Ignore non-test packets */
    if !is_test_packet_tc(ctx) {
        return TC_ACT_SHOT;
    }

    /* Expect read-write metadata before unclone */
    bpf_dynptr_from_skb_meta(ctx, 0, meta.as_mut_ptr());
    if bpf_dynptr_is_rdonly(meta.as_ptr()) {
        return TC_ACT_SHOT;
    }

    let mut err = bpf_dynptr_read(meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32, meta.as_ptr(), 0, 0);
    if err != 0 || !check_metadata(meta_have.as_mut_ptr()) {
        return TC_ACT_SHOT;
    }

    /* Helper write to metadata will unclone the packet */
    bpf_dynptr_write(meta.as_mut_ptr(), 0, &meta_have[0] as *const _ as *const c_void, 1, 0);

    err = bpf_dynptr_read(meta_have.as_mut_ptr() as *mut c_void, META_SIZE as __u32, meta.as_ptr(), 0, 0);
    if err != 0 || !check_metadata(meta_have.as_mut_ptr()) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

// SEC("lwt_xmit")
#[no_mangle]
pub unsafe extern "C" fn dummy_lwt_xmit(ctx: *mut __sk_buff) -> i32 {
    if bpf_skb_change_head(ctx, core::mem::size_of::<ipv6hdr>() as __u32, 0) != 0 {
        return BPF_DROP;
    }

    BPF_OK
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_is_meta_empty(ctx: *mut __sk_buff) -> i32 {
    if !is_test_packet_tc(ctx) {
        return TC_ACT_OK;
    }

    if (*ctx).data_meta != (*ctx).data {
        return TC_ACT_OK;
    }

    test_pass = true;
    TC_ACT_OK
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn helper_skb_vlan_push_pop(ctx: *mut __sk_buff) -> i32 {
    /* bpf_skb_vlan_push assumes HW offload for primary VLAN tag. Only
     * secondary tag push triggers an actual MAC header modification.
     */
    let mut err = bpf_skb_vlan_push(ctx, 0, 42);
    if err != 0 {
        return TC_ACT_SHOT;
    }
    err = bpf_skb_vlan_push(ctx, 0, 207);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    err = bpf_skb_vlan_pop(ctx);
    if err != 0 {
        return TC_ACT_SHOT;
    }
    err = bpf_skb_vlan_pop(ctx);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn helper_skb_adjust_room(ctx: *mut __sk_buff) -> i32 {
    /* Grow a 1 byte hole after the MAC header */
    let mut err = bpf_skb_adjust_room(ctx, 1, BPF_ADJ_ROOM_MAC, 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    /* Shrink a 1 byte hole after the MAC header */
    err = bpf_skb_adjust_room(ctx, -1, BPF_ADJ_ROOM_MAC, 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    /* Grow a 256 byte hole to trigger head reallocation */
    err = bpf_skb_adjust_room(ctx, 256, BPF_ADJ_ROOM_MAC, 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn helper_skb_change_head_tail(ctx: *mut __sk_buff) -> i32 {
    /* Reserve 1 extra in the front for packet data */
    let mut err = bpf_skb_change_head(ctx, 1, 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    /* Reserve 256 extra bytes in the front to trigger head reallocation */
    err = bpf_skb_change_head(ctx, 256, 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    /* Reserve 4k extra bytes in the back to trigger head reallocation */
    err = bpf_skb_change_tail(ctx, (*ctx).len + 4096, 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn helper_skb_change_proto(ctx: *mut __sk_buff) -> i32 {
    let mut err = bpf_skb_change_proto(ctx, bpf_htons(ETH_P_IPV6), 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    err = bpf_skb_change_proto(ctx, bpf_htons(ETH_P_IP), 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    if !check_skb_metadata(ctx) {
        return TC_ACT_SHOT;
    }

    test_pass = true;
    TC_ACT_SHOT
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
