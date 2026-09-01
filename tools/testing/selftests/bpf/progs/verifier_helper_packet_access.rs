// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/helper_packet_access.c */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![feature(asm_experimental_arch)]
#![feature(naked_functions)]

use core::arch::asm;
use core::ffi::{c_longlong, c_void};
use core::ptr;

// Dependencies from <linux/bpf.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
// The original C source uses SEC(), __description(), __success, __failure,
// __retval(), __msg(), __naked, __imm(), __imm_addr(), __imm_const(), and
// __clobber_all from those headers.
const BPF_MAP_TYPE_HASH: u32 = 1;

extern "C" {
    fn bpf_map_update_elem(map: *mut c_void, key: *const c_void, value: *const c_void, flags: u64) -> i64;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_skb_store_bytes(skb: *mut c_void, offset: u32, from: *const c_void, len: u32, flags: u64) -> i64;
    fn bpf_skb_load_bytes(skb: *const c_void, offset: u32, to: *mut c_void, len: u32) -> i64;
    fn bpf_csum_diff(from: *const c_void, from_size: u32, to: *const c_void, to_size: u32, seed: u32) -> i64;
}

#[repr(C)]
pub struct map_hash_8b_def {
    pub r#type: *mut [u32; BPF_MAP_TYPE_HASH as usize],
    pub max_entries: *mut [u32; 1],
    pub key: *mut c_longlong,
    pub value: *mut c_longlong,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash_8b: map_hash_8b_def = map_hash_8b_def {
    r#type: ptr::null_mut(),
    max_entries: ptr::null_mut(),
    key: ptr::null_mut(),
    value: ptr::null_mut(),
};

// offsetof(struct xdp_md, data)
const xdp_md_data: usize = 0;
// offsetof(struct xdp_md, data_end)
const xdp_md_data_end: usize = 4;
// offsetof(struct __sk_buff, data)
const __sk_buff_data: usize = 76;
// offsetof(struct __sk_buff, data_end)
const __sk_buff_data_end: usize = 80;
const __imm_0: i32 = !0;

#[link_section = "xdp"]
#[no_mangle]
// __description("helper access to packet: test1, valid packet_ptr range")
// __success __retval(0)
pub unsafe extern "C" fn test1_valid_packet_ptr_range() {
    asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data})",
        "r3 = *(u32*)(r1 + {xdp_md_data_end})",
        "r1 = r2",
        "r1 += 8",
        "if r1 > r3 goto 0f",
        "r1 = {map_hash_8b} ll",
        "r3 = r2",
        "r4 = 0",
        "call {bpf_map_update_elem}",
        "0:",
        "r0 = 0",
        "exit",
        xdp_md_data = const xdp_md_data,
        xdp_md_data_end = const xdp_md_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_update_elem = sym bpf_map_update_elem,
        options(noreturn)
    );
}

#[link_section = "xdp"]
#[no_mangle]
// __description("helper access to packet: test2, unchecked packet_ptr")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn packet_test2_unchecked_packet_ptr() {
    asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data})",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "r0 = 0",
        "exit",
        xdp_md_data = const xdp_md_data,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "xdp"]
#[no_mangle]
// __description("helper access to packet: test3, variable add")
// __success __retval(0)
pub unsafe extern "C" fn to_packet_test3_variable_add() {
    asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data})",
        "r3 = *(u32*)(r1 + {xdp_md_data_end})",
        "r4 = r2",
        "r4 += 8",
        "if r4 > r3 goto 0f",
        "r5 = *(u8*)(r2 + 0)",
        "r4 = r2",
        "r4 += r5",
        "r5 = r4",
        "r5 += 8",
        "if r5 > r3 goto 0f",
        "r1 = {map_hash_8b} ll",
        "r2 = r4",
        "call {bpf_map_lookup_elem}",
        "0:",
        "r0 = 0",
        "exit",
        xdp_md_data = const xdp_md_data,
        xdp_md_data_end = const xdp_md_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "xdp"]
#[no_mangle]
// __description("helper access to packet: test4, packet_ptr with bad range")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn packet_ptr_with_bad_range_1() {
    asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data})",
        "r3 = *(u32*)(r1 + {xdp_md_data_end})",
        "r4 = r2",
        "r4 += 4",
        "if r4 > r3 goto 0f",
        "r0 = 0",
        "exit",
        "0:",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "r0 = 0",
        "exit",
        xdp_md_data = const xdp_md_data,
        xdp_md_data_end = const xdp_md_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "xdp"]
#[no_mangle]
// __description("helper access to packet: test5, packet_ptr with too short range")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn ptr_with_too_short_range_1() {
    asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data})",
        "r3 = *(u32*)(r1 + {xdp_md_data_end})",
        "r2 += 1",
        "r4 = r2",
        "r4 += 7",
        "if r4 > r3 goto 0f",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "0:",
        "r0 = 0",
        "exit",
        xdp_md_data = const xdp_md_data,
        xdp_md_data_end = const xdp_md_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test6, cls valid packet_ptr range")
// __success __retval(0)
pub unsafe extern "C" fn cls_valid_packet_ptr_range() {
    asm!(
        "r2 = *(u32*)(r1 + {__sk_buff_data})",
        "r3 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r1 = r2",
        "r1 += 8",
        "if r1 > r3 goto 0f",
        "r1 = {map_hash_8b} ll",
        "r3 = r2",
        "r4 = 0",
        "call {bpf_map_update_elem}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_update_elem = sym bpf_map_update_elem,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test7, cls unchecked packet_ptr")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn test7_cls_unchecked_packet_ptr() {
    asm!(
        "r2 = *(u32*)(r1 + {__sk_buff_data})",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test8, cls variable add")
// __success __retval(0)
pub unsafe extern "C" fn packet_test8_cls_variable_add() {
    asm!(
        "r2 = *(u32*)(r1 + {__sk_buff_data})",
        "r3 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r4 = r2",
        "r4 += 8",
        "if r4 > r3 goto 0f",
        "r5 = *(u8*)(r2 + 0)",
        "r4 = r2",
        "r4 += r5",
        "r5 = r4",
        "r5 += 8",
        "if r5 > r3 goto 0f",
        "r1 = {map_hash_8b} ll",
        "r2 = r4",
        "call {bpf_map_lookup_elem}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test9, cls packet_ptr with bad range")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn packet_ptr_with_bad_range_2() {
    asm!(
        "r2 = *(u32*)(r1 + {__sk_buff_data})",
        "r3 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r4 = r2",
        "r4 += 4",
        "if r4 > r3 goto 0f",
        "r0 = 0",
        "exit",
        "0:",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test10, cls packet_ptr with too short range")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn ptr_with_too_short_range_2() {
    asm!(
        "r2 = *(u32*)(r1 + {__sk_buff_data})",
        "r3 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r2 += 1",
        "r4 = r2",
        "r4 += 7",
        "if r4 > r3 goto 0f",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test11, cls unsuitable helper 1")
// __failure __msg("helper access to the packet")
pub unsafe extern "C" fn test11_cls_unsuitable_helper_1() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r3 = r6",
        "r3 += 7",
        "if r3 > r7 goto 0f",
        "r2 = 0",
        "r4 = 42",
        "r5 = 0",
        "call {bpf_skb_store_bytes}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_skb_store_bytes = sym bpf_skb_store_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test12, cls unsuitable helper 2")
// __failure __msg("helper access to the packet")
pub unsafe extern "C" fn test12_cls_unsuitable_helper_2() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r3 = r6",
        "r6 += 8",
        "if r6 > r7 goto 0f",
        "r2 = 0",
        "r4 = 4",
        "call {bpf_skb_load_bytes}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test13, cls helper ok")
// __success __retval(0)
pub unsafe extern "C" fn packet_test13_cls_helper_ok() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 = r6",
        "r2 = 4",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test14, cls helper ok sub")
// __success __retval(0)
pub unsafe extern "C" fn test14_cls_helper_ok_sub() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 -= 4",
        "r2 = 4",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test15, cls helper fail sub")
// __failure __msg("R1 min value is negative")
pub unsafe extern "C" fn test15_cls_helper_fail_sub() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 -= 12",
        "r2 = 4",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test16, cls helper fail range 1")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn cls_helper_fail_range_1() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 = r6",
        "r2 = 8",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test17, cls helper fail range 2")
// __failure __msg("R2 min value is negative")
pub unsafe extern "C" fn cls_helper_fail_range_2() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 = r6",
        "r2 = -9",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test18, cls helper fail range 3")
// __failure __msg("R2 min value is negative")
pub unsafe extern "C" fn cls_helper_fail_range_3() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 = r6",
        "r2 = {__imm_0}",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __imm_0 = const __imm_0,
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test19, cls helper range zero")
// __success __retval(0)
pub unsafe extern "C" fn test19_cls_helper_range_zero() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 = r6",
        "r2 = 0",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test20, pkt end as input")
// __failure __msg("R1 type=pkt_end expected=fp")
pub unsafe extern "C" fn test20_pkt_end_as_input() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r1 = r7",
        "r2 = 4",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("helper access to packet: test21, wrong reg")
// __failure __msg("invalid access to packet")
pub unsafe extern "C" fn to_packet_test21_wrong_reg() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_data})",
        "r7 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r6 += 1",
        "r1 = r6",
        "r1 += 7",
        "if r1 > r7 goto 0f",
        "r2 = 4",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "r0 = 0",
        "0:",
        "exit",
        __sk_buff_data = const __sk_buff_data,
        __sk_buff_data_end = const __sk_buff_data_end,
        bpf_csum_diff = sym bpf_csum_diff,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
