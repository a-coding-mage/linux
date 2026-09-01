// SPDX-License-Identifier: GPL-2.0
// Converted from tools/testing/selftests/bpf/verifier/bounds.c
// Rust translation of verifier_bounds.c.
//
// Original C includes removed; symbols from linux/bpf.h, linux/filter.h,
// bpf/bpf_helpers.h, and bpf_misc.h are expected to be supplied by the final build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

type __u32 = u32;
type __u64 = u64;

// struct { __uint(type, BPF_MAP_TYPE_HASH); __uint(max_entries, 1); __type(key, long long); __type(value, long long); } map_hash_8b SEC(".maps");
#[repr(C)]
pub struct bpf_map_def_placeholder {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key: i64,
    pub value: i64,
}

unsafe extern "C" {
    static map_hash_8b: bpf_map_def_placeholder;
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_get_prandom_u32() -> __u32;
    fn bpf_skb_store_bytes(skb: *mut c_void, offset: __u32, from: *const c_void, len: __u32, flags: __u64) -> i64;
    fn bpf_get_netns_cookie(ctx: *mut c_void) -> __u64;
}

// BPF constants/offsetof values are supplied by the final repository context.
const BPF_F_ANY_ALIGNMENT: __u32 = 0;
const BPF_F_TEST_REG_INVARIANTS: __u32 = 0;
const BPF_F_TEST_STATE_FREQ: __u32 = 0;
const __sk_buff_mark: i32 = 0;
const __sk_buff_data: i32 = 0;
const __sk_buff_data_end: i32 = 0;
const xdp_md_data: i32 = 0;
const xdp_md_data_end: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
