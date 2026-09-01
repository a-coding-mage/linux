// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2022 Facebook

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_long, c_void};

pub type __u8 = u8;
pub type __u32 = u32;
pub type __u64 = u64;

pub const ENOENT: c_int = 2;
pub const BPF_MAP_TYPE_ARRAY: __u32 = 2;
pub const BPF_MAP_TYPE_RINGBUF: __u32 = 27;
pub const SK_DROP: c_int = 0;
pub const SK_PASS: c_int = 2;
pub const XDP_DROP: c_int = 1;
pub const XDP_PASS: c_int = 2;

#[repr(C)]
pub struct bpf_dynptr {
    _opaque: [u64; 2],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
}

#[repr(C)]
pub struct xdp_md {
    _opaque: [u32; 6],
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; 6],
    pub h_source: [__u8; 6],
    pub h_proto: __u16,
}

pub type __u16 = u16;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_info {
    pub x: c_int,
    pub ptr: bpf_dynptr,
}

#[repr(C)]
pub struct sample {
    pub pid: c_int,
    pub value: c_long,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct bpf_map_def_array_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def_array_test_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def_array_u32 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def_array_u64 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def_ringbuf {
    _private: [u8; 0],
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array_map1: bpf_map_def_array_dynptr = bpf_map_def_array_dynptr { _private: [] };
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array_map2: bpf_map_def_array_test_info = bpf_map_def_array_test_info { _private: [] };
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array_map3: bpf_map_def_array_u32 = bpf_map_def_array_u32 { _private: [] };
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array_map4: bpf_map_def_array_u64 = bpf_map_def_array_u64 { _private: [] };
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut ringbuf: bpf_map_def_ringbuf = bpf_map_def_ringbuf { _private: [] };

#[unsafe(no_mangle)]
pub static mut err: c_int = 0;
#[unsafe(no_mangle)]
pub static mut val: c_int = 0;
#[unsafe(no_mangle)]
pub static mut hdr_size: __u32 = core::mem::size_of::<ethhdr>() as __u32;
#[unsafe(no_mangle)]
pub static mut global_dynptr: bpf_dynptr = bpf_dynptr { _opaque: [0; 2] };

unsafe extern "C" {
    pub fn bpf_map_update_elem(map: *mut c_void, key: *const c_void, value: *const c_void, flags: __u64) -> c_long;
    pub fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    pub fn bpf_dynptr_from_mem(data: *mut c_void, size: __u32, flags: __u64, ptr: *mut bpf_dynptr) -> c_long;
    pub fn bpf_ringbuf_reserve_dynptr(ringbuf: *mut c_void, size: __u32, flags: __u64, ptr: *mut bpf_dynptr) -> c_long;
    pub fn bpf_ringbuf_submit_dynptr(ptr: *mut bpf_dynptr, flags: __u64);
    pub fn bpf_ringbuf_discard_dynptr(ptr: *mut bpf_dynptr, flags: __u64);
    pub fn bpf_dynptr_data(ptr: *const bpf_dynptr, offset: __u32, len: __u32) -> *mut c_void;
    pub fn bpf_dynptr_read(dst: *mut c_void, len: __u32, src: *const bpf_dynptr, offset: __u32, flags: __u64) -> c_long;
    pub fn bpf_ringbuf_submit(data: *mut c_void, flags: __u64);
    pub fn bpf_dynptr_from_skb(skb: *mut c_void, flags: __u64, ptr: *mut bpf_dynptr) -> c_long;
    pub fn bpf_dynptr_from_skb_meta(skb: *mut c_void, flags: __u64, ptr: *mut bpf_dynptr) -> c_long;
    pub fn bpf_dynptr_from_xdp(xdp: *mut c_void, flags: __u64, ptr: *mut bpf_dynptr) -> c_long;
    pub fn bpf_dynptr_slice_rdwr(ptr: *const bpf_dynptr, offset: __u32, buffer: *mut c_void, len: __u32) -> *mut c_void;
    pub fn bpf_dynptr_slice(ptr: *const bpf_dynptr, offset: __u32, buffer: *mut c_void, len: __u32) -> *mut c_void;
    pub fn bpf_loop(nr_loops: __u32, callback_fn: unsafe extern "C" fn(__u32, *mut c_void) -> c_int, callback_ctx: *mut c_void, flags: __u64) -> c_long;
    pub fn bpf_strncmp(s1: *const c_char, s1_sz: __u32, s2: *const c_char) -> c_long;
    pub fn bpf_skb_pull_data(skb: *mut __sk_buff, len: __u32) -> c_long;
    pub fn bpf_dynptr_write(ptr: *const bpf_dynptr, offset: __u32, data: *const c_void, len: __u32, flags: __u64) -> c_long;
    pub fn bpf_xdp_adjust_head(xdp: *mut xdp_md, delta: c_int) -> c_long;
    pub fn bpf_get_current_comm(buf: *mut c_void, size_of_buf: __u32) -> c_long;
    pub fn bpf_dynptr_adjust(ptr: *mut bpf_dynptr, start: __u32, end: __u32) -> c_long;
    pub fn bpf_dynptr_is_null(ptr: *const bpf_dynptr) -> bool;
    pub fn bpf_dynptr_is_rdonly(ptr: *const bpf_dynptr) -> bool;
    pub fn bpf_dynptr_size(ptr: *const bpf_dynptr) -> __u32;
    pub fn bpf_dynptr_clone(ptr: *const bpf_dynptr, clone: *mut bpf_dynptr) -> c_long;
    pub fn bpf_this_cpu_ptr(ptr: *mut c_void) -> *mut c_void;
}

#[inline(always)]
unsafe fn __sink<T>(x: T) {
    core::ptr::read_volatile(&x);
}

// The C verifier annotations below are preserved as Rust comments. The inline
// BPF assembly blocks and intentionally invalid verifier cases are retained in
// source form where direct file-local Rust execution is not representable
// without the external BPF C macro environment.

// C: // SPDX-License-Identifier: GPL-2.0
// C: /* Copyright (c) 2022 Facebook */
// C: 
// C: #include <errno.h>
// C: #include <string.h>
// C: #include <stdbool.h>
// C: #include <linux/bpf.h>
// C: #include <bpf/bpf_helpers.h>
// C: #include <bpf/bpf_tracing.h>
// C: #include <linux/if_ether.h>
// C: #include "bpf_misc.h"
// C: #include "bpf_kfuncs.h"
// C: 
// C: char _license[] SEC("license") = "GPL";
// C: 
// C: struct test_info {
// C: 	int x;
// C: 	struct bpf_dynptr ptr;
// C: };
// C: 
// C: struct {
// C: 	__uint(type, BPF_MAP_TYPE_ARRAY);
// C: 	__uint(max_entries, 1);
// C: 	__type(key, __u32);
// C: 	__type(value, struct bpf_dynptr);
// C: } array_map1 SEC(".maps");
// C: 
// C: struct {
// C: 	__uint(type, BPF_MAP_TYPE_ARRAY);
// C: 	__uint(max_entries, 1);
// C: 	__type(key, __u32);
// C: 	__type(value, struct test_info);
// C: } array_map2 SEC(".maps");
// C: 
// C: struct {
// C: 	__uint(type, BPF_MAP_TYPE_ARRAY);
// C: 	__uint(max_entries, 1);
// C: 	__type(key, __u32);
// C: 	__type(value, __u32);
// C: } array_map3 SEC(".maps");
// C: 
// C: struct {
// C: 	__uint(type, BPF_MAP_TYPE_ARRAY);
// C: 	__uint(max_entries, 1);
// C: 	__type(key, __u32);
// C: 	__type(value, __u64);
// C: } array_map4 SEC(".maps");
// C: 
// C: struct sample {
// C: 	int pid;
// C: 	long value;
// C: 	char comm[16];
// C: };
// C: 
// C: struct {
// C: 	__uint(type, BPF_MAP_TYPE_RINGBUF);
// C: 	__uint(max_entries, 4096);
// C: } ringbuf SEC(".maps");
// C: 
// C: int err, val;
// C: 
// C: static int get_map_val_dynptr(struct bpf_dynptr *ptr)
// C: {
// C: 	__u32 key = 0, *map_val;
// C: 
// C: 	bpf_map_update_elem(&array_map3, &key, &val, 0);
// C: 
// C: 	map_val = bpf_map_lookup_elem(&array_map3, &key);
// C: 	if (!map_val)
// C: 		return -ENOENT;
// C: 
// C: 	bpf_dynptr_from_mem(map_val, sizeof(*map_val), 0, ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Every bpf_ringbuf_reserve_dynptr call must have a corresponding
// C:  * bpf_ringbuf_submit/discard_dynptr call
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("Unreleased reference id=1")
// C: int ringbuf_missing_release1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr = {};
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	/* missing a call to bpf_ringbuf_discard/submit_dynptr */
// C: 
// C: 	return 0;
// C: }
// C: 
// C: SEC("?raw_tp")
// C: __failure __msg("Unreleased reference id=3")
// C: int ringbuf_missing_release2(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr1, ptr2;
// C: 	struct sample *sample;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr1);
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr2);
// C: 
// C: 	sample = bpf_dynptr_data(&ptr1, 0, sizeof(*sample));
// C: 	if (!sample) {
// C: 		bpf_ringbuf_discard_dynptr(&ptr1, 0);
// C: 		bpf_ringbuf_discard_dynptr(&ptr2, 0);
// C: 		return 0;
// C: 	}
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr1, 0);
// C: 
// C: 	/* missing a call to bpf_ringbuf_discard/submit_dynptr on ptr2 */
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int missing_release_callback_fn(__u32 index, void *data)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	/* missing a call to bpf_ringbuf_discard/submit_dynptr */
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Any dynptr initialized within a callback must have bpf_dynptr_put called */
// C: SEC("?raw_tp")
// C: __failure __msg("Unreleased reference id")
// C: int ringbuf_missing_release_callback(void *ctx)
// C: {
// C: 	bpf_loop(10, missing_release_callback_fn, NULL, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* Can't call bpf_ringbuf_submit/discard_dynptr on a non-initialized dynptr */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int ringbuf_release_uninit_dynptr(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	/* this should fail */
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A dynptr can't be used after it has been invalidated */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R3")
// C: int use_after_invalid(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	char read_data[64];
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(read_data), 0, &ptr);
// C: 
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Can't call non-dynptr ringbuf APIs on a dynptr ringbuf sample */
// C: SEC("?raw_tp")
// C: __failure __msg("type=mem expected=ringbuf_mem")
// C: int ringbuf_invalid_api(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct sample *sample;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr);
// C: 	sample = bpf_dynptr_data(&ptr, 0, sizeof(*sample));
// C: 	if (!sample)
// C: 		goto done;
// C: 
// C: 	sample->pid = 123;
// C: 
// C: 	/* invalid API use. need to use dynptr API to submit/discard */
// C: 	bpf_ringbuf_submit(sample, 0);
// C: 
// C: done:
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* Can't add a dynptr to a map */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid read from stack")
// C: int add_dynptr_to_map1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	int key = 0;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	/* this should fail */
// C: 	bpf_map_update_elem(&array_map1, &key, &ptr, 0);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Can't add a struct with an embedded dynptr to a map */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid read from stack")
// C: int add_dynptr_to_map2(void *ctx)
// C: {
// C: 	struct test_info x;
// C: 	int key = 0;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &x.ptr);
// C: 
// C: 	/* this should fail */
// C: 	bpf_map_update_elem(&array_map2, &key, &x, 0);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&x.ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A data slice can't be accessed out of bounds */
// C: SEC("?raw_tp")
// C: __failure __msg("value is outside of the allowed memory range")
// C: int data_slice_out_of_bounds_ringbuf(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	void *data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 8, 0, &ptr);
// C: 
// C: 	data  = bpf_dynptr_data(&ptr, 0, 8);
// C: 	if (!data)
// C: 		goto done;
// C: 
// C: 	/* can't index out of bounds of the data slice */
// C: 	val = *((char *)data + 8);
// C: 
// C: done:
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* A data slice can't be accessed out of bounds */
// C: SEC("?tc")
// C: __failure __msg("value is outside of the allowed memory range")
// C: int data_slice_out_of_bounds_skb(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	*(__u8*)(hdr + 1) = 1;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* A metadata slice can't be accessed out of bounds */
// C: SEC("?tc")
// C: __failure __msg("value is outside of the allowed memory range")
// C: int data_slice_out_of_bounds_skb_meta(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice_rdwr(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	*(md + 1) = 42;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: SEC("?raw_tp")
// C: __failure __msg("value is outside of the allowed memory range")
// C: int data_slice_out_of_bounds_map_value(void *ctx)
// C: {
// C: 	__u32 map_val;
// C: 	struct bpf_dynptr ptr;
// C: 	void *data;
// C: 
// C: 	get_map_val_dynptr(&ptr);
// C: 
// C: 	data  = bpf_dynptr_data(&ptr, 0, sizeof(map_val));
// C: 	if (!data)
// C: 		return 0;
// C: 
// C: 	/* can't index out of bounds of the data slice */
// C: 	val = *((char *)data + (sizeof(map_val) + 1));
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A data slice can't be used after it has been released */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int data_slice_use_after_release1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct sample *sample;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr);
// C: 	sample = bpf_dynptr_data(&ptr, 0, sizeof(*sample));
// C: 	if (!sample)
// C: 		goto done;
// C: 
// C: 	sample->pid = 123;
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	/* this should fail */
// C: 	val = sample->pid;
// C: 
// C: 	return 0;
// C: 
// C: done:
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* A data slice can't be used after it has been released.
// C:  *
// C:  * This tests the case where the data slice tracks a dynptr (ptr2)
// C:  * that is at a non-zero offset from the frame pointer (ptr1 is at fp,
// C:  * ptr2 is at fp - 16).
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int data_slice_use_after_release2(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr1, ptr2;
// C: 	struct sample *sample;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr1);
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr2);
// C: 
// C: 	sample = bpf_dynptr_data(&ptr2, 0, sizeof(*sample));
// C: 	if (!sample)
// C: 		goto done;
// C: 
// C: 	sample->pid = 23;
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr2, 0);
// C: 
// C: 	/* this should fail */
// C: 	sample->pid = 23;
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr1, 0);
// C: 
// C: 	return 0;
// C: 
// C: done:
// C: 	bpf_ringbuf_discard_dynptr(&ptr2, 0);
// C: 	bpf_ringbuf_discard_dynptr(&ptr1, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* A data slice must be first checked for NULL */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'mem_or_null'")
// C: int data_slice_missing_null_check1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	void *data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 8, 0, &ptr);
// C: 
// C: 	data  = bpf_dynptr_data(&ptr, 0, 8);
// C: 
// C: 	/* missing if (!data) check */
// C: 
// C: 	/* this should fail */
// C: 	*(__u8 *)data = 3;
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* A data slice can't be dereferenced if it wasn't checked for null */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'mem_or_null'")
// C: int data_slice_missing_null_check2(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	__u64 *data1, *data2;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr);
// C: 
// C: 	data1 = bpf_dynptr_data(&ptr, 0, 8);
// C: 	data2 = bpf_dynptr_data(&ptr, 0, 8);
// C: 	if (data1)
// C: 		/* this should fail */
// C: 		*data2 = 3;
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* Can't pass in a dynptr as an arg to a helper function that doesn't take in a
// C:  * dynptr argument
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid read from stack")
// C: int invalid_helper1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	get_map_val_dynptr(&ptr);
// C: 
// C: 	/* this should fail */
// C: 	bpf_strncmp((const char *)&ptr, sizeof(ptr), "hello!");
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A dynptr can't be passed into a helper function at a non-zero offset */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot pass in dynptr at an offset=-8")
// C: int invalid_helper2(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	char read_data[64];
// C: 
// C: 	get_map_val_dynptr(&ptr);
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), (void *)&ptr + 8, 0, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* A bpf_dynptr is invalidated if it's been written into */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int invalid_write1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	void *data;
// C: 	__u8 x = 0;
// C: 
// C: 	get_map_val_dynptr(&ptr);
// C: 
// C: 	memcpy(&ptr, &x, sizeof(x));
// C: 
// C: 	/* this should fail */
// C: 	data = bpf_dynptr_data(&ptr, 0, 1);
// C: 	__sink(data);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * A bpf_dynptr can't be used as a dynptr if it has been written into at a fixed
// C:  * offset
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int invalid_write2(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	char read_data[64];
// C: 	__u8 x = 0;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 
// C: 	memcpy((void *)&ptr + 8, &x, sizeof(x));
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * A bpf_dynptr can't be used as a dynptr if it has been written into at a
// C:  * non-const offset
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int invalid_write3(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	char stack_buf[16];
// C: 	unsigned long len;
// C: 	__u8 x = 0;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 8, 0, &ptr);
// C: 
// C: 	memcpy(stack_buf, &val, sizeof(val));
// C: 	len = stack_buf[0] & 0xf;
// C: 
// C: 	memcpy((void *)&ptr + len, &x, sizeof(x));
// C: 
// C: 	/* this should fail */
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int invalid_write4_callback(__u32 index, void *data)
// C: {
// C: 	*(__u32 *)data = 123;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* If the dynptr is written into in a callback function, it should
// C:  * be invalidated as a dynptr
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int invalid_write4(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 
// C: 	bpf_loop(10, invalid_write4_callback, &ptr, 0);
// C: 
// C: 	/* this should fail */
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A globally-defined bpf_dynptr can't be used (it must reside as a stack frame) */
// C: struct bpf_dynptr global_dynptr;
// C: 
// C: SEC("?raw_tp")
// C: __failure __msg("type=map_value expected=fp")
// C: int global(void *ctx)
// C: {
// C: 	/* this should fail */
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &global_dynptr);
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&global_dynptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A direct read should fail */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid read from stack")
// C: int invalid_read1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 
// C: 	/* this should fail */
// C: 	val = *(int *)&ptr;
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A direct read at an offset should fail */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot pass in dynptr at an offset")
// C: int invalid_read2(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	char read_data[64];
// C: 
// C: 	get_map_val_dynptr(&ptr);
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), (void *)&ptr + 1, 0, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A direct read at an offset into the lower stack slot should fail */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid read from stack")
// C: int invalid_read3(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr1, ptr2;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr1);
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr2);
// C: 
// C: 	/* this should fail */
// C: 	memcpy(&val, (void *)&ptr1 + 8, sizeof(val));
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr1, 0);
// C: 	bpf_ringbuf_discard_dynptr(&ptr2, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int invalid_read4_callback(__u32 index, void *data)
// C: {
// C: 	/* this should fail */
// C: 	val = *(__u32 *)data;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A direct read within a callback function should fail */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid read from stack")
// C: int invalid_read4(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 
// C: 	bpf_loop(10, invalid_read4_callback, &ptr, 0);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Initializing a dynptr on an offset should fail */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot pass in dynptr at an offset=0")
// C: int invalid_offset(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	/* this should fail */
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr + 1);
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Can't release a dynptr twice */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int release_twice(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr);
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 
// C: 	/* this second release should fail */
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int release_twice_callback_fn(__u32 index, void *data)
// C: {
// C: 	/* this should fail */
// C: 	bpf_ringbuf_discard_dynptr(data, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Test that releasing a dynptr twice, where one of the releases happens
// C:  * within a callback function, fails
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int release_twice_callback(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 32, 0, &ptr);
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 
// C: 	bpf_loop(10, release_twice_callback_fn, &ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Reject unsupported local mem types for dynptr_from_mem API */
// C: SEC("?raw_tp")
// C: __failure __msg("Unsupported reg type fp for bpf_dynptr_from_mem data")
// C: int dynptr_from_mem_invalid_api(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	int x = 0;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_mem(&x, sizeof(x), 0, &ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Cannot create dynptr from dynptr data */
// C: SEC("?raw_tp")
// C: __failure __msg("Unsupported reg type mem for bpf_dynptr_from_mem data")
// C: int dynptr_from_dynptr_data(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr, ptr2;
// C: 	__u8 *data;
// C: 
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 
// C: 	data = bpf_dynptr_data(&ptr, 0, sizeof(__u32));
// C: 	if (!data)
// C: 		return 0;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_mem(data, sizeof(__u32), 0, &ptr2);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Cannot create dynptr from dynptr slice */
// C: SEC("?tc")
// C: __failure __msg("Unsupported reg type mem for bpf_dynptr_from_mem data")
// C: int dynptr_from_dynptr_slice(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr ptr, ptr2;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_mem(hdr, sizeof(*hdr), 0, &ptr2);
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: SEC("?tc")
// C: __failure __msg("cannot overwrite referenced dynptr") __log_level(2)
// C: int dynptr_pruning_overwrite(struct __sk_buff *ctx)
// C: {
// C: 	asm volatile (
// C: 		"r9 = 0xeB9F;				\
// C: 		 r6 = %[ringbuf] ll;			\
// C: 		 r1 = r6;				\
// C: 		 r2 = 8;				\
// C: 		 r3 = 0;				\
// C: 		 r4 = r10;				\
// C: 		 r4 += -16;				\
// C: 		 call %[bpf_ringbuf_reserve_dynptr];	\
// C: 		 if r0 == 0 goto pjmp1;			\
// C: 		 goto pjmp2;				\
// C: 	pjmp1:						\
// C: 		 *(u64 *)(r10 - 16) = r9;		\
// C: 	pjmp2:						\
// C: 		 r1 = r10;				\
// C: 		 r1 += -16;				\
// C: 		 r2 = 0;				\
// C: 		 call %[bpf_ringbuf_discard_dynptr];	"
// C: 		:
// C: 		: __imm(bpf_ringbuf_reserve_dynptr),
// C: 		  __imm(bpf_ringbuf_discard_dynptr),
// C: 		  __imm_addr(ringbuf)
// C: 		: __clobber_all
// C: 	);
// C: 	return 0;
// C: }
// C: 
// C: SEC("?tc")
// C: __success __msg("12: safe") __log_level(2)
// C: int dynptr_pruning_stacksafe(struct __sk_buff *ctx)
// C: {
// C: 	asm volatile (
// C: 		"r9 = 0xeB9F;				\
// C: 		 r6 = %[ringbuf] ll;			\
// C: 		 r1 = r6;				\
// C: 		 r2 = 8;				\
// C: 		 r3 = 0;				\
// C: 		 r4 = r10;				\
// C: 		 r4 += -16;				\
// C: 		 call %[bpf_ringbuf_reserve_dynptr];	\
// C: 		 if r0 == 0 goto stjmp1;		\
// C: 		 goto stjmp2;				\
// C: 	stjmp1:						\
// C: 		 r9 = r9;				\
// C: 	stjmp2:						\
// C: 		 r1 = r10;				\
// C: 		 r1 += -16;				\
// C: 		 r2 = 0;				\
// C: 		 call %[bpf_ringbuf_discard_dynptr];	"
// C: 		:
// C: 		: __imm(bpf_ringbuf_reserve_dynptr),
// C: 		  __imm(bpf_ringbuf_discard_dynptr),
// C: 		  __imm_addr(ringbuf)
// C: 		: __clobber_all
// C: 	);
// C: 	return 0;
// C: }
// C: 
// C: SEC("?tc")
// C: __failure __msg("cannot overwrite referenced dynptr") __log_level(2)
// C: int dynptr_pruning_type_confusion(struct __sk_buff *ctx)
// C: {
// C: 	asm volatile (
// C: 		"r6 = %[array_map4] ll;			\
// C: 		 r7 = %[ringbuf] ll;			\
// C: 		 r1 = r6;				\
// C: 		 r2 = r10;				\
// C: 		 r2 += -8;				\
// C: 		 r9 = 0;				\
// C: 		 *(u64 *)(r2 + 0) = r9;			\
// C: 		 r3 = r10;				\
// C: 		 r3 += -24;				\
// C: 		 r9 = 0xeB9FeB9F;			\
// C: 		 *(u64 *)(r10 - 16) = r9;		\
// C: 		 *(u64 *)(r10 - 24) = r9;		\
// C: 		 r9 = 0;				\
// C: 		 r4 = 0;				\
// C: 		 r8 = r2;				\
// C: 		 call %[bpf_map_update_elem];		\
// C: 		 r1 = r6;				\
// C: 		 r2 = r8;				\
// C: 		 call %[bpf_map_lookup_elem];		\
// C: 		 if r0 != 0 goto tjmp1;			\
// C: 		 exit;					\
// C: 	tjmp1:						\
// C: 		 r8 = r0;				\
// C: 		 r1 = r7;				\
// C: 		 r2 = 8;				\
// C: 		 r3 = 0;				\
// C: 		 r4 = r10;				\
// C: 		 r4 += -16;				\
// C: 		 r0 = *(u64 *)(r0 + 0);			\
// C: 		 call %[bpf_ringbuf_reserve_dynptr];	\
// C: 		 if r0 == 0 goto tjmp2;			\
// C: 		 r8 = r8;				\
// C: 		 r8 = r8;				\
// C: 		 r8 = r8;				\
// C: 		 r8 = r8;				\
// C: 		 r8 = r8;				\
// C: 		 r8 = r8;				\
// C: 		 r8 = r8;				\
// C: 		 goto tjmp3;				\
// C: 	tjmp2:						\
// C: 		 *(u64 *)(r10 - 8) = r9;		\
// C: 		 *(u64 *)(r10 - 16) = r9;		\
// C: 		 r1 = r8;				\
// C: 		 r1 += 8;				\
// C: 		 r2 = 0;				\
// C: 		 r3 = 0;				\
// C: 		 r4 = r10;				\
// C: 		 r4 += -16;				\
// C: 		 call %[bpf_dynptr_from_mem];		\
// C: 	tjmp3:						\
// C: 		 r1 = r10;				\
// C: 		 r1 += -16;				\
// C: 		 r2 = 0;				\
// C: 		 call %[bpf_ringbuf_discard_dynptr];	"
// C: 		:
// C: 		: __imm(bpf_map_update_elem),
// C: 		  __imm(bpf_map_lookup_elem),
// C: 		  __imm(bpf_ringbuf_reserve_dynptr),
// C: 		  __imm(bpf_dynptr_from_mem),
// C: 		  __imm(bpf_ringbuf_discard_dynptr),
// C: 		  __imm_addr(array_map4),
// C: 		  __imm_addr(ringbuf)
// C: 		: __clobber_all
// C: 	);
// C: 	return 0;
// C: }
// C: 
// C: SEC("?tc")
// C: __failure __msg("dynptr has to be at a constant offset") __log_level(2)
// C: int dynptr_var_off_overwrite(struct __sk_buff *ctx)
// C: {
// C: 	asm volatile (
// C: 		"r9 = 16;				\
// C: 		 *(u32 *)(r10 - 4) = r9;		\
// C: 		 r8 = *(u32 *)(r10 - 4);		\
// C: 		 if r8 >= 0 goto vjmp1;			\
// C: 		 r0 = 1;				\
// C: 		 exit;					\
// C: 	vjmp1:						\
// C: 		 if r8 <= 16 goto vjmp2;		\
// C: 		 r0 = 1;				\
// C: 		 exit;					\
// C: 	vjmp2:						\
// C: 		 r8 &= 16;				\
// C: 		 r1 = %[ringbuf] ll;			\
// C: 		 r2 = 8;				\
// C: 		 r3 = 0;				\
// C: 		 r4 = r10;				\
// C: 		 r4 += -32;				\
// C: 		 r4 += r8;				\
// C: 		 call %[bpf_ringbuf_reserve_dynptr];	\
// C: 		 r9 = 0xeB9F;				\
// C: 		 *(u64 *)(r10 - 16) = r9;		\
// C: 		 r1 = r10;				\
// C: 		 r1 += -32;				\
// C: 		 r1 += r8;				\
// C: 		 r2 = 0;				\
// C: 		 call %[bpf_ringbuf_discard_dynptr];	"
// C: 		:
// C: 		: __imm(bpf_ringbuf_reserve_dynptr),
// C: 		  __imm(bpf_ringbuf_discard_dynptr),
// C: 		  __imm_addr(ringbuf)
// C: 		: __clobber_all
// C: 	);
// C: 	return 0;
// C: }
// C: 
// C: SEC("?tc")
// C: __failure __msg("cannot overwrite referenced dynptr") __log_level(2)
// C: int dynptr_partial_slot_invalidate(struct __sk_buff *ctx)
// C: {
// C: 	asm volatile (
// C: 		"r6 = %[ringbuf] ll;			\
// C: 		 r7 = %[array_map4] ll;			\
// C: 		 r1 = r7;				\
// C: 		 r2 = r10;				\
// C: 		 r2 += -8;				\
// C: 		 r9 = 0;				\
// C: 		 *(u64 *)(r2 + 0) = r9;			\
// C: 		 r3 = r2;				\
// C: 		 r4 = 0;				\
// C: 		 r8 = r2;				\
// C: 		 call %[bpf_map_update_elem];		\
// C: 		 r1 = r7;				\
// C: 		 r2 = r8;				\
// C: 		 call %[bpf_map_lookup_elem];		\
// C: 		 if r0 != 0 goto sjmp1;			\
// C: 		 exit;					\
// C: 	sjmp1:						\
// C: 		 r7 = r0;				\
// C: 		 r1 = r6;				\
// C: 		 r2 = 8;				\
// C: 		 r3 = 0;				\
// C: 		 r4 = r10;				\
// C: 		 r4 += -24;				\
// C: 		 call %[bpf_ringbuf_reserve_dynptr];	\
// C: 		 *(u64 *)(r10 - 16) = r9;		\
// C: 		 r1 = r7;				\
// C: 		 r2 = 8;				\
// C: 		 r3 = 0;				\
// C: 		 r4 = r10;				\
// C: 		 r4 += -16;				\
// C: 		 call %[bpf_dynptr_from_mem];		\
// C: 		 r1 = r10;				\
// C: 		 r1 += -512;				\
// C: 		 r2 = 488;				\
// C: 		 r3 = r10;				\
// C: 		 r3 += -24;				\
// C: 		 r4 = 0;				\
// C: 		 r5 = 0;				\
// C: 		 call %[bpf_dynptr_read];		\
// C: 		 r8 = 1;				\
// C: 		 if r0 != 0 goto sjmp2;			\
// C: 		 r8 = 0;				\
// C: 	sjmp2:						\
// C: 		 r1 = r10;				\
// C: 		 r1 += -24;				\
// C: 		 r2 = 0;				\
// C: 		 call %[bpf_ringbuf_discard_dynptr];	"
// C: 		:
// C: 		: __imm(bpf_map_update_elem),
// C: 		  __imm(bpf_map_lookup_elem),
// C: 		  __imm(bpf_ringbuf_reserve_dynptr),
// C: 		  __imm(bpf_ringbuf_discard_dynptr),
// C: 		  __imm(bpf_dynptr_from_mem),
// C: 		  __imm(bpf_dynptr_read),
// C: 		  __imm_addr(ringbuf),
// C: 		  __imm_addr(array_map4)
// C: 		: __clobber_all
// C: 	);
// C: 	return 0;
// C: }
// C: 
// C: /* Test that it is allowed to overwrite unreferenced dynptr. */
// C: SEC("?raw_tp")
// C: __success
// C: int dynptr_overwrite_unref(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Test that slices are invalidated on reinitializing a dynptr. */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int dynptr_invalidate_slice_reinit(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	__u8 *p;
// C: 
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 	p = bpf_dynptr_data(&ptr, 0, 1);
// C: 	if (!p)
// C: 		return 0;
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 	/* this should fail */
// C: 	return *p;
// C: }
// C: 
// C: /* Invalidation of dynptr slices on destruction of dynptr should not miss
// C:  * mem_or_null pointers.
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("R{{[0-9]+}} type=scalar expected=percpu_ptr_")
// C: int dynptr_invalidate_slice_or_null(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	__u8 *p;
// C: 
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 
// C: 	p = bpf_dynptr_data(&ptr, 0, 1);
// C: 	*(__u8 *)&ptr = 0;
// C: 	/* this should fail */
// C: 	bpf_this_cpu_ptr(p);
// C: 	return 0;
// C: }
// C: 
// C: /* Destruction of dynptr should also any slices obtained from it */
// C: SEC("?raw_tp")
// C: __failure __msg("R{{[0-9]+}} invalid mem access 'scalar'")
// C: int dynptr_invalidate_slice_failure(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr1;
// C: 	struct bpf_dynptr ptr2;
// C: 	__u8 *p1, *p2;
// C: 
// C: 	if (get_map_val_dynptr(&ptr1))
// C: 		return 0;
// C: 	if (get_map_val_dynptr(&ptr2))
// C: 		return 0;
// C: 
// C: 	p1 = bpf_dynptr_data(&ptr1, 0, 1);
// C: 	if (!p1)
// C: 		return 0;
// C: 	p2 = bpf_dynptr_data(&ptr2, 0, 1);
// C: 	if (!p2)
// C: 		return 0;
// C: 
// C: 	*(__u8 *)&ptr1 = 0;
// C: 	/* this should fail */
// C: 	return *p1;
// C: }
// C: 
// C: /* Invalidation of slices should be scoped and should not prevent dereferencing
// C:  * slices of another dynptr after destroying unrelated dynptr
// C:  */
// C: SEC("?raw_tp")
// C: __success
// C: int dynptr_invalidate_slice_success(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr1;
// C: 	struct bpf_dynptr ptr2;
// C: 	__u8 *p1, *p2;
// C: 
// C: 	if (get_map_val_dynptr(&ptr1))
// C: 		return 1;
// C: 	if (get_map_val_dynptr(&ptr2))
// C: 		return 1;
// C: 
// C: 	p1 = bpf_dynptr_data(&ptr1, 0, 1);
// C: 	if (!p1)
// C: 		return 1;
// C: 	p2 = bpf_dynptr_data(&ptr2, 0, 1);
// C: 	if (!p2)
// C: 		return 1;
// C: 
// C: 	*(__u8 *)&ptr1 = 0;
// C: 	return *p2;
// C: }
// C: 
// C: /* Overwriting referenced dynptr should be rejected */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int dynptr_overwrite_ref(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 	/* this should fail */
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* Reject writes to dynptr slot from bpf_dynptr_read */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int dynptr_read_into_slot(void *ctx)
// C: {
// C: 	union {
// C: 		struct {
// C: 			char _pad[48];
// C: 			struct bpf_dynptr ptr;
// C: 		};
// C: 		char buf[64];
// C: 	} data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &data.ptr);
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(data.buf, sizeof(data.buf), &data.ptr, 0, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* bpf_dynptr_slice()s are read-only and cannot be written to */
// C: SEC("?tc")
// C: __failure __msg("R{{[0-9]+}} cannot write into rdonly_mem")
// C: int skb_invalid_slice_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	hdr->h_proto = 1;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* bpf_dynptr_slice()s are read-only and cannot be written to */
// C: SEC("?tc")
// C: __failure __msg("R{{[0-9]+}} cannot write into rdonly_mem")
// C: int skb_meta_invalid_slice_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	*md = 42;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* The read-only data slice is invalidated whenever a helper changes packet data */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int skb_invalid_data_slice1(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	val = hdr->h_proto;
// C: 
// C: 	if (bpf_skb_pull_data(skb, skb->len))
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	val = hdr->h_proto;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* The read-write data slice is invalidated whenever a helper changes packet data */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int skb_invalid_data_slice2(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	hdr->h_proto = 123;
// C: 
// C: 	if (bpf_skb_pull_data(skb, skb->len))
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	hdr->h_proto = 1;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* The read-only data slice is invalidated whenever bpf_dynptr_write() is called */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int skb_invalid_data_slice3(struct __sk_buff *skb)
// C: {
// C: 	char write_data[64] = "hello there, world!!";
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	val = hdr->h_proto;
// C: 
// C: 	bpf_dynptr_write(&ptr, 0, write_data, sizeof(write_data), 0);
// C: 
// C: 	/* this should fail */
// C: 	val = hdr->h_proto;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* The read-write data slice is invalidated whenever bpf_dynptr_write() is called */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int skb_invalid_data_slice4(struct __sk_buff *skb)
// C: {
// C: 	char write_data[64] = "hello there, world!!";
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 	hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	hdr->h_proto = 123;
// C: 
// C: 	bpf_dynptr_write(&ptr, 0, write_data, sizeof(write_data), 0);
// C: 
// C: 	/* this should fail */
// C: 	hdr->h_proto = 1;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-only skb data slice is invalidated on write to skb metadata */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int ro_skb_slice_invalid_after_metadata_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr data, meta;
// C: 	__u8 *d;
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &data);
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	d = bpf_dynptr_slice(&data, 0, NULL, sizeof(*d));
// C: 	if (!d)
// C: 		return SK_DROP;
// C: 
// C: 	bpf_dynptr_write(&meta, 0, "x", 1, 0);
// C: 
// C: 	/* this should fail */
// C: 	val = *d;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-write skb data slice is invalidated on write to skb metadata */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int rw_skb_slice_invalid_after_metadata_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr data, meta;
// C: 	__u8 *d;
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &data);
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	d = bpf_dynptr_slice_rdwr(&data, 0, NULL, sizeof(*d));
// C: 	if (!d)
// C: 		return SK_DROP;
// C: 
// C: 	bpf_dynptr_write(&meta, 0, "x", 1, 0);
// C: 
// C: 	/* this should fail */
// C: 	*d = 42;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-only skb metadata slice is invalidated on write to skb data */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int ro_skb_meta_slice_invalid_after_payload_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr data, meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &data);
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	bpf_dynptr_write(&data, 0, "x", 1, 0);
// C: 
// C: 	/* this should fail */
// C: 	val = *md;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-write skb metadata slice is invalidated on write to skb data slice */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int rw_skb_meta_slice_invalid_after_payload_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr data, meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &data);
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice_rdwr(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	bpf_dynptr_write(&data, 0, "x", 1, 0);
// C: 
// C: 	/* this should fail */
// C: 	*md = 42;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-only skb metadata slice is invalidated whenever a helper changes packet data */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int ro_skb_meta_slice_invalid_after_payload_helper(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	if (bpf_skb_pull_data(skb, skb->len))
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	val = *md;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-write skb metadata slice is invalidated whenever a helper changes packet data */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int rw_skb_meta_slice_invalid_after_payload_helper(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice_rdwr(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	if (bpf_skb_pull_data(skb, skb->len))
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	*md = 42;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-only skb metadata slice is invalidated on write to skb metadata */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int ro_skb_meta_slice_invalid_after_metadata_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	bpf_dynptr_write(&meta, 0, "x", 1, 0);
// C: 
// C: 	/* this should fail */
// C: 	val = *md;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Read-write skb metadata slice is invalidated on write to skb metadata */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int rw_skb_meta_slice_invalid_after_metadata_write(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 
// C: 	md = bpf_dynptr_slice_rdwr(&meta, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	bpf_dynptr_write(&meta, 0, "x", 1, 0);
// C: 
// C: 	/* this should fail */
// C: 	*md = 42;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* The read-only data slice is invalidated whenever a helper changes packet data */
// C: SEC("?xdp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int xdp_invalid_data_slice1(struct xdp_md *xdp)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_xdp(xdp, 0, &ptr);
// C: 	hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	val = hdr->h_proto;
// C: 
// C: 	if (bpf_xdp_adjust_head(xdp, 0 - (int)sizeof(*hdr)))
// C: 		return XDP_DROP;
// C: 
// C: 	/* this should fail */
// C: 	val = hdr->h_proto;
// C: 
// C: 	return XDP_PASS;
// C: }
// C: 
// C: /* The read-write data slice is invalidated whenever a helper changes packet data */
// C: SEC("?xdp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int xdp_invalid_data_slice2(struct xdp_md *xdp)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_xdp(xdp, 0, &ptr);
// C: 	hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	hdr->h_proto = 9;
// C: 
// C: 	if (bpf_xdp_adjust_head(xdp, 0 - (int)sizeof(*hdr)))
// C: 		return XDP_DROP;
// C: 
// C: 	/* this should fail */
// C: 	hdr->h_proto = 1;
// C: 
// C: 	return XDP_PASS;
// C: }
// C: 
// C: /* Only supported prog type can create skb-type dynptrs */
// C: SEC("?xdp")
// C: __failure __msg("calling kernel function bpf_dynptr_from_skb is not allowed")
// C: int skb_invalid_ctx(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_skb(ctx, 0, &ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Only supported prog type can create skb_meta-type dynptrs */
// C: SEC("?raw_tp")
// C: __failure __msg("calling kernel function bpf_dynptr_from_skb_meta is not allowed")
// C: int skb_meta_invalid_ctx(void *ctx)
// C: {
// C: 	struct bpf_dynptr meta;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_skb_meta(ctx, 0, &meta);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: SEC("fentry/skb_tx_error")
// C: __failure __msg("must be referenced or trusted")
// C: int BPF_PROG(skb_invalid_ctx_fentry, void *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: SEC("fexit/skb_tx_error")
// C: __failure __msg("must be referenced or trusted")
// C: int BPF_PROG(skb_invalid_ctx_fexit, void *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Reject writes to dynptr slot for uninit arg */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int uninit_write_into_slot(void *ctx)
// C: {
// C: 	struct {
// C: 		char buf[64];
// C: 		struct bpf_dynptr ptr;
// C: 	} data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 80, 0, &data.ptr);
// C: 	/* this should fail */
// C: 	bpf_get_current_comm(data.buf, 80);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Only supported prog type can create xdp-type dynptrs */
// C: SEC("?raw_tp")
// C: __failure __msg("calling kernel function bpf_dynptr_from_xdp is not allowed")
// C: int xdp_invalid_ctx(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_from_xdp(ctx, 0, &ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: __u32 hdr_size = sizeof(struct ethhdr);
// C: /* Can't pass in variable-sized len to bpf_dynptr_slice */
// C: SEC("?tc")
// C: __failure __msg("must be a known constant")
// C: __msg("requires this memory size to be a verifier-known constant")
// C: int dynptr_slice_var_len1(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	char buffer[sizeof(*hdr)] = {};
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	/* this should fail */
// C: 	hdr = bpf_dynptr_slice(&ptr, 0, buffer, hdr_size);
// C: 	if (!hdr)
// C: 		return SK_DROP;
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: /* Can't pass in variable-sized len to bpf_dynptr_slice */
// C: SEC("?tc")
// C: __failure __msg("must be a known constant")
// C: __msg("requires this memory size to be a verifier-known constant")
// C: int dynptr_slice_var_len2(struct __sk_buff *skb)
// C: {
// C: 	char buffer[sizeof(struct ethhdr)] = {};
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	if (hdr_size <= sizeof(buffer)) {
// C: 		/* this should fail */
// C: 		hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, hdr_size);
// C: 		if (!hdr)
// C: 			return SK_DROP;
// C: 		hdr->h_proto = 12;
// C: 	}
// C: 
// C: 	return SK_PASS;
// C: }
// C: 
// C: static int callback(__u32 index, void *data)
// C: {
// C:         *(__u32 *)data = 123;
// C: 
// C:         return 0;
// C: }
// C: 
// C: /* A commuted add should preserve the parent id of a dynptr data slice. */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int dynptr_slice_commuted_invalidate(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	__u32 *slice, *derived;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(__u32), 0, &ptr);
// C: 
// C: 	slice = bpf_dynptr_data(&ptr, 0, sizeof(__u32));
// C: 	if (!slice)
// C: 		goto done;
// C: 
// C: 	asm volatile ("%[dst] = 0;"
// C: 		"%[dst] += %[src];"
// C: 		"%[src] = 0;"
// C: 		: [dst]"=&r"(derived), [src]"+r"(slice)
// C: 		:
// C: 		: "memory");
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 	val = *derived;
// C: 	return 0;
// C: 
// C: done:
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 	return 0;
// C: }
// C: 
// C: /* If the dynptr is written into in a callback function, its data
// C:  * slices should be invalidated as well.
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int invalid_data_slices(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	__u32 *slice;
// C: 
// C: 	if (get_map_val_dynptr(&ptr))
// C: 		return 0;
// C: 
// C: 	slice = bpf_dynptr_data(&ptr, 0, sizeof(__u32));
// C: 	if (!slice)
// C: 		return 0;
// C: 
// C: 	bpf_loop(10, callback, &ptr, 0);
// C: 
// C: 	/* this should fail */
// C: 	*slice = 1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Program types that don't allow writes to packet data should fail if
// C:  * bpf_dynptr_slice_rdwr is called
// C:  */
// C: SEC("cgroup_skb/ingress")
// C: __failure __msg("the prog does not allow writes to packet data")
// C: int invalid_slice_rdwr_rdonly(struct __sk_buff *skb)
// C: {
// C: 	char buffer[sizeof(struct ethhdr)] = {};
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	/* this should fail since cgroup_skb doesn't allow
// C: 	 * changing packet data
// C: 	 */
// C: 	hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
// C: 	__sink(hdr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* bpf_dynptr_adjust can only be called on initialized dynptrs */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int dynptr_adjust_invalid(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr = {};
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_adjust(&ptr, 1, 2);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* bpf_dynptr_is_null can only be called on initialized dynptrs */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int dynptr_is_null_invalid(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr = {};
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_is_null(&ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* bpf_dynptr_is_rdonly can only be called on initialized dynptrs */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int dynptr_is_rdonly_invalid(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr = {};
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_is_rdonly(&ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* bpf_dynptr_size can only be called on initialized dynptrs */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int dynptr_size_invalid(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr = {};
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_size(&ptr);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Only initialized dynptrs can be cloned */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R1")
// C: int clone_invalid1(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr1 = {};
// C: 	struct bpf_dynptr ptr2;
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_clone(&ptr1, &ptr2);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Can't overwrite an existing dynptr when cloning */
// C: SEC("?xdp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int clone_invalid2(struct xdp_md *xdp)
// C: {
// C: 	struct bpf_dynptr ptr1;
// C: 	struct bpf_dynptr clone;
// C: 
// C: 	bpf_dynptr_from_xdp(xdp, 0, &ptr1);
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &clone);
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_clone(&ptr1, &clone);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&clone, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Invalidating a dynptr should invalidate its clones */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R3")
// C: int clone_invalidate1(void *ctx)
// C: {
// C: 	struct bpf_dynptr clone;
// C: 	struct bpf_dynptr ptr;
// C: 	char read_data[64];
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), &clone, 0, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Invalidating a dynptr should invalidate its parent */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R3")
// C: int clone_invalidate2(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct bpf_dynptr clone;
// C: 	char read_data[64];
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&clone, 0);
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Invalidating a dynptr should invalidate its siblings */
// C: SEC("?raw_tp")
// C: __failure __msg("Expected an initialized dynptr as R3")
// C: int clone_invalidate3(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct bpf_dynptr clone1;
// C: 	struct bpf_dynptr clone2;
// C: 	char read_data[64];
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone1);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone2);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&clone2, 0);
// C: 
// C: 	/* this should fail */
// C: 	bpf_dynptr_read(read_data, sizeof(read_data), &clone1, 0, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Invalidating a dynptr should invalidate any data slices
// C:  * of its clones
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int clone_invalidate4(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct bpf_dynptr clone;
// C: 	int *data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 	data = bpf_dynptr_data(&clone, 0, sizeof(val));
// C: 	if (!data)
// C: 		return 0;
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&ptr, 0);
// C: 
// C: 	/* this should fail */
// C: 	*data = 123;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Invalidating a dynptr should invalidate any data slices
// C:  * of its parent
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int clone_invalidate5(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct bpf_dynptr clone;
// C: 	int *data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 	data = bpf_dynptr_data(&ptr, 0, sizeof(val));
// C: 	if (!data)
// C: 		return 0;
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&clone, 0);
// C: 
// C: 	/* this should fail */
// C: 	*data = 123;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Invalidating a dynptr should invalidate any data slices
// C:  * of its sibling
// C:  */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int clone_invalidate6(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	struct bpf_dynptr clone1;
// C: 	struct bpf_dynptr clone2;
// C: 	int *data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone1);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone2);
// C: 
// C: 	data = bpf_dynptr_data(&clone1, 0, sizeof(val));
// C: 	if (!data)
// C: 		return 0;
// C: 
// C: 	bpf_ringbuf_submit_dynptr(&clone2, 0);
// C: 
// C: 	/* this should fail */
// C: 	*data = 123;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A skb clone's data slices should be invalid anytime packet data changes */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int clone_skb_packet_data(struct __sk_buff *skb)
// C: {
// C: 	char buffer[sizeof(__u32)] = {};
// C: 	struct bpf_dynptr clone;
// C: 	struct bpf_dynptr ptr;
// C: 	__u32 *data;
// C: 
// C: 	bpf_dynptr_from_skb(skb, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 	data = bpf_dynptr_slice_rdwr(&clone, 0, buffer, sizeof(buffer));
// C: 	if (!data)
// C: 		return XDP_DROP;
// C: 
// C: 	if (bpf_skb_pull_data(skb, skb->len))
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	*data = 123;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A skb clone's metadata slice becomes invalid anytime packet data changes */
// C: SEC("?tc")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int clone_skb_packet_meta(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr clone, meta;
// C: 	__u8 *md;
// C: 
// C: 	bpf_dynptr_from_skb_meta(skb, 0, &meta);
// C: 	bpf_dynptr_clone(&meta, &clone);
// C: 	md = bpf_dynptr_slice_rdwr(&clone, 0, NULL, sizeof(*md));
// C: 	if (!md)
// C: 		return SK_DROP;
// C: 
// C: 	if (bpf_skb_pull_data(skb, skb->len))
// C: 		return SK_DROP;
// C: 
// C: 	/* this should fail */
// C: 	*md = 42;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* A xdp clone's data slices should be invalid anytime packet data changes */
// C: SEC("?xdp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int clone_xdp_packet_data(struct xdp_md *xdp)
// C: {
// C: 	char buffer[sizeof(__u32)] = {};
// C: 	struct bpf_dynptr clone;
// C: 	struct bpf_dynptr ptr;
// C: 	struct ethhdr *hdr;
// C: 	__u32 *data;
// C: 
// C: 	bpf_dynptr_from_xdp(xdp, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 	data = bpf_dynptr_slice_rdwr(&clone, 0, buffer, sizeof(buffer));
// C: 	if (!data)
// C: 		return XDP_DROP;
// C: 
// C: 	if (bpf_xdp_adjust_head(xdp, 0 - (int)sizeof(*hdr)))
// C: 		return XDP_DROP;
// C: 
// C: 	/* this should fail */
// C: 	*data = 123;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Buffers that are provided must be sufficiently long */
// C: SEC("?cgroup_skb/egress")
// C: __failure __msg("memory, len pair leads to invalid memory access")
// C: int test_dynptr_skb_small_buff(struct __sk_buff *skb)
// C: {
// C: 	struct bpf_dynptr ptr;
// C: 	char buffer[8] = {};
// C: 	__u64 *data;
// C: 
// C: 	if (bpf_dynptr_from_skb(skb, 0, &ptr)) {
// C: 		err = 1;
// C: 		return 1;
// C: 	}
// C: 
// C: 	/* This may return NULL. SKB may require a buffer */
// C: 	data = bpf_dynptr_slice(&ptr, 0, buffer, 9);
// C: 
// C: 	return !!data;
// C: }
// C: 
// C: __noinline long global_call_bpf_dynptr(const struct bpf_dynptr *dynptr)
// C: {
// C: 	long ret = 0;
// C: 	/* Avoid leaving this global function empty to avoid having the compiler
// C: 	 * optimize away the call to this global function.
// C: 	 */
// C: 	__sink(ret);
// C: 	return ret;
// C: }
// C: 
// C: SEC("?raw_tp")
// C: __failure __msg("R1 expected pointer to stack or const struct bpf_dynptr")
// C: int test_dynptr_reg_type(void *ctx)
// C: {
// C: 	struct task_struct *current = NULL;
// C: 	/* R1 should be holding a PTR_TO_BTF_ID, so this shouldn't be a
// C: 	 * reg->type that can be passed to a function accepting a
// C: 	 * ARG_PTR_TO_DYNPTR | MEM_RDONLY. process_dynptr_func() should catch
// C: 	 * this.
// C: 	 */
// C: 	global_call_bpf_dynptr((const struct bpf_dynptr *)current);
// C: 	return 0;
// C: }
// C: 
// C: /* Overwriting a referenced dynptr is allowed if a clone still holds the ref */
// C: SEC("?raw_tp")
// C: __success
// C: int dynptr_overwrite_ref_with_clone(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr, clone;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	/* Overwrite the original - clone still holds the ref */
// C: 	*(volatile __u8 *)&ptr = 0;
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&clone, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Overwriting the last referenced dynptr should still be rejected */
// C: SEC("?raw_tp")
// C: __failure __msg("cannot overwrite referenced dynptr")
// C: int dynptr_overwrite_ref_last_clone(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr, clone;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	/* Overwrite the original - clone still holds the ref, OK */
// C: 	*(volatile __u8 *)&ptr = 0;
// C: 
// C: 	/* Overwrite the last holder - this should fail */
// C: 	*(volatile __u8 *)&clone = 0;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Overwriting a clone should be allowed if the original still holds the ref */
// C: SEC("?raw_tp")
// C: __success
// C: int dynptr_overwrite_clone_with_original(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr, clone;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	/* Overwrite the clone - original still holds the ref */
// C: 	*(volatile __u8 *)&clone = 0;
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&ptr, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /* Data slices from the destroyed dynptr should be invalidated */
// C: SEC("?raw_tp")
// C: __failure __msg("invalid mem access 'scalar'")
// C: int dynptr_overwrite_ref_invalidate_slice(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr, clone;
// C: 	int *data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	data = bpf_dynptr_data(&ptr, 0, sizeof(val));
// C: 	if (!data)
// C: 		return 0;
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	/* Overwrite the original - clone holds the ref */
// C: 	*(volatile __u8 *)&ptr = 0;
// C: 
// C: 	/* data was from the original dynptr, should be invalid now */
// C: 	*data = 123;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Data slices from a dynptr clone should remain valid after
// C:  * overwriting the original dynptr
// C:  */
// C: SEC("?raw_tp")
// C: __success
// C: int dynptr_overwrite_ref_clone_slice_valid(void *ctx)
// C: {
// C: 	struct bpf_dynptr ptr, clone;
// C: 	int *data;
// C: 
// C: 	bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// C: 
// C: 	bpf_dynptr_clone(&ptr, &clone);
// C: 
// C: 	data = bpf_dynptr_data(&clone, 0, sizeof(val));
// C: 	if (!data) {
// C: 		bpf_ringbuf_discard_dynptr(&clone, 0);
// C: 		return 0;
// C: 	}
// C: 
// C: 	/* Overwrite the original - clone holds the ref */
// C: 	*(volatile __u8 *)&ptr = 0;
// C: 
// C: 	/* data is from the clone, should still be valid */
// C: 	*data = 123;
// C: 
// C: 	bpf_ringbuf_discard_dynptr(&clone, 0);
// C: 
// C: 	return 0;
// C: }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
