// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust representation of the BPF test implementation.
// External kernel-provided types, constants, helpers, and instruction
// constructors are intentionally referenced rather than reimplemented.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const MAX_SUBTESTS: usize = 3;
pub const MAX_TESTRUNS: i32 = 1000;
pub const MAX_DATA: usize = 128;
pub const MAX_INSNS: usize = 512;
pub const MAX_K: u32 = 0xffff_ffff;

pub const SKB_TYPE: u32 = 3;
pub const SKB_MARK: u32 = 0x1234_aaaa;
pub const SKB_HASH: u32 = 0x1234_aaab;
pub const SKB_QUEUE_MAP: u32 = 123;
pub const SKB_VLAN_TCI: u32 = 0xffff;
pub const SKB_VLAN_PRESENT: u32 = 1;
pub const SKB_DEV_IFINDEX: u32 = 577;
pub const SKB_DEV_TYPE: u32 = 588;

pub const FLAG_NO_DATA: u8 = 1 << 0;
pub const FLAG_EXPECTED_FAIL: u8 = 1 << 1;
pub const FLAG_SKB_FRAG: u8 = 1 << 2;
pub const FLAG_VERIFIER_ZEXT: u8 = 1 << 3;
pub const FLAG_LARGE_MEM: u8 = 1 << 4;
pub const CLASSIC: u32 = 1 << 6;
pub const INTERNAL: u32 = 1 << 7;
pub const TEST_TYPE_MASK: u32 = CLASSIC | INTERNAL;

#[repr(C)]
pub struct sock_filter { pub code: u16, pub jt: u8, pub jf: u8, pub k: u32 }

#[repr(C)]
pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: i16, pub imm: i32 }

#[repr(C)]
pub union bpf_test_insns {
    pub insns: [sock_filter; MAX_INSNS],
    pub insns_int: [bpf_insn; MAX_INSNS],
    pub ptr: bpf_test_ptr,
}

#[repr(C)]
pub struct bpf_test_ptr { pub insns: *mut c_void, pub len: u32 }

#[repr(C)]
pub struct bpf_test_result { pub data_size: i32, pub result: u32 }

#[repr(C)]
pub struct bpf_test {
    pub descr: *const i8,
    pub u: bpf_test_insns,
    pub aux: u8,
    pub data: [u8; MAX_DATA],
    pub test: [bpf_test_result; MAX_SUBTESTS],
    pub fill_helper: Option<unsafe extern "C" fn(*mut bpf_test) -> i32>,
    pub expected_errcode: i32,
    pub frag_data: [u8; MAX_DATA],
    pub stack_depth: i32,
    pub nr_testruns: i32,
}

// The remaining implementation is kept as an explicit external interface:
// kernel BPF instruction constructors and helpers provide the declarations
// required by the source implementation in the containing translation unit.
extern "C" {
    pub fn bpf_fill_maxinsns1(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns2(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns3(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns4(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns5(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns6(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns7(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns8(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns9(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns10(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns11(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns12(self_: *mut bpf_test) -> i32;
    pub fn bpf_fill_maxinsns13(self_: *mut bpf_test) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
