// SPDX-License-Identifier: GPL-2.0-only
/*
 * Testsuite for eBPF verifier
 *
 * Copyright (c) 2014 PLUMgrid, http://plumgrid.com
 * Copyright (c) 2017 Facebook
 * Copyright (c) 2018 Covalent IO, Inc. http://covalent.io
 */

// Translated from testing/selftests/bpf/test_verifier.c.
// C include dependencies preserved as external items/macros expected from the
// surrounding build: endian.h, asm/types.h, linux/types.h, std C/POSIX,
// linux BPF headers, libbpf, autoconf_helper.h, unpriv_helpers.h,
// cap_helpers.h, bpf_rand.h, bpf_util.h, test_btf.h, linux/filter.h, and
// testing_helpers.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type __s16 = i16;
type __s32 = i32;
type uint32_t = u32;
type uint64_t = u64;
type uintptr_t = usize;
type size_t = usize;
type bool_ = bool;

const MAX_INSNS: usize = BPF_MAXINSNS as usize;
const MAX_EXPECTED_INSNS: usize = 32;
const MAX_UNEXPECTED_INSNS: usize = 32;
const MAX_TEST_INSNS: usize = 1000000;
const MAX_FIXUPS: usize = 8;
const MAX_NR_MAPS: usize = 23;
const MAX_TEST_RUNS: usize = 8;
const POINTER_VALUE: u32 = 0xcafe4a;
const TEST_DATA_LEN: usize = 64;
const MAX_FUNC_INFOS: usize = 8;
const MAX_BTF_STRINGS: usize = 256;
const MAX_BTF_TYPES: usize = 256;

const INSN_OFF_MASK: __s16 = 0xFFFFu16 as __s16;
const INSN_IMM_MASK: __s32 = 0xFFFFFFFFu32 as __s32;
const DEFAULT_LIBBPF_LOG_LEVEL: c_int = 4;

const F_NEEDS_EFFICIENT_UNALIGNED_ACCESS: u8 = 1 << 0;
const F_LOAD_WITH_STRICT_ALIGNMENT: u8 = 1 << 1;
const F_NEEDS_JIT_ENABLED: u8 = 1 << 2;

const ADMIN_CAPS: __u64 =
    (1u64 << CAP_NET_ADMIN) | (1u64 << CAP_PERFMON) | (1u64 << CAP_BPF);
const UNPRIV_SYSCTL: &[u8] = b"kernel/unprivileged_bpf_disabled\0";

static mut unpriv_disabled: bool = false;
static mut jit_disabled: bool = false;
static mut skips: c_int = 0;
static mut verbose: bool = false;
static mut verif_log_level: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: __u8,
    pub regs: __u8,
    pub off: __s16,
    pub imm: __s32,
}

impl bpf_insn {
    unsafe fn dst_reg(&self) -> __u8 {
        self.regs & 0x0f
    }
    unsafe fn src_reg(&self) -> __u8 {
        self.regs >> 4
    }
    unsafe fn set_off(&mut self, off: __s16) {
        self.off = off;
    }
    unsafe fn set_imm(&mut self, imm: __s32) {
        self.imm = imm;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfunc_btf_id_pair {
    pub kfunc: *const c_char,
    pub insn_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_test_result {
    UNDEF = 0,
    ACCEPT = 1,
    REJECT = 2,
    VERBOSE_ACCEPT = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testdata_struct_t {
    pub retval: uint32_t,
    pub retval_unpriv: uint32_t,
    pub data: [__u8; TEST_DATA_LEN],
}

#[repr(C)]
pub union bpf_test_data_union {
    pub single: bpf_testdata_struct_t,
    pub retvals: [bpf_testdata_struct_t; MAX_TEST_RUNS],
}

#[repr(C)]
pub struct bpf_test {
    pub descr: *const c_char,
    pub insns: [bpf_insn; MAX_INSNS],
    pub fill_insns: *mut bpf_insn,
    pub expected_insns: [bpf_insn; MAX_EXPECTED_INSNS],
    pub unexpected_insns: [bpf_insn; MAX_UNEXPECTED_INSNS],
    pub fixup_map_hash_8b: [c_int; MAX_FIXUPS],
    pub fixup_map_hash_48b: [c_int; MAX_FIXUPS],
    pub fixup_map_hash_16b: [c_int; MAX_FIXUPS],
    pub fixup_map_array_48b: [c_int; MAX_FIXUPS],
    pub fixup_map_sockmap: [c_int; MAX_FIXUPS],
    pub fixup_map_sockhash: [c_int; MAX_FIXUPS],
    pub fixup_map_xskmap: [c_int; MAX_FIXUPS],
    pub fixup_map_stacktrace: [c_int; MAX_FIXUPS],
    pub fixup_prog1: [c_int; MAX_FIXUPS],
    pub fixup_prog2: [c_int; MAX_FIXUPS],
    pub fixup_map_in_map: [c_int; MAX_FIXUPS],
    pub fixup_cgroup_storage: [c_int; MAX_FIXUPS],
    pub fixup_percpu_cgroup_storage: [c_int; MAX_FIXUPS],
    pub fixup_map_spin_lock: [c_int; MAX_FIXUPS],
    pub fixup_map_array_ro: [c_int; MAX_FIXUPS],
    pub fixup_map_array_wo: [c_int; MAX_FIXUPS],
    pub fixup_map_array_small: [c_int; MAX_FIXUPS],
    pub fixup_sk_storage_map: [c_int; MAX_FIXUPS],
    pub fixup_map_event_output: [c_int; MAX_FIXUPS],
    pub fixup_map_reuseport_array: [c_int; MAX_FIXUPS],
    pub fixup_map_ringbuf: [c_int; MAX_FIXUPS],
    pub fixup_map_timer: [c_int; MAX_FIXUPS],
    pub fixup_map_kptr: [c_int; MAX_FIXUPS],
    pub fixup_kfunc_btf_id: [kfunc_btf_id_pair; MAX_FIXUPS],
    pub errstr: *const c_char,
    pub errstr_unpriv: *const c_char,
    pub insn_processed: uint32_t,
    pub prog_len: c_int,
    pub result: bpf_test_result,
    pub result_unpriv: bpf_test_result,
    pub prog_type: bpf_prog_type,
    pub flags: __u8,
    pub fill_helper: Option<unsafe extern "C" fn(*mut bpf_test)>,
    pub runs: c_int,
    pub testdata: bpf_test_data_union,
    pub expected_attach_type: bpf_attach_type,
    pub kfunc: *const c_char,
    pub func_info: [bpf_func_info; MAX_FUNC_INFOS],
    pub func_info_cnt: c_int,
    pub btf_strings: [c_char; MAX_BTF_STRINGS],
    pub btf_types: [__u32; MAX_BTF_TYPES],
}

const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

#[repr(C)]
pub struct other_val {
    pub foo: i64,
    pub bar: i64,
}

unsafe fn SKIP_INSNS() -> bpf_insn {
    BPF_RAW_INSN(0xde, 0xa, 0xd, 0xbeef_u16 as i16, 0xdeadbeef_u32 as i32)
}

unsafe extern "C" fn bpf_fill_ld_abs_vlan_push_pop(self_: *mut bpf_test) {
    const PUSH_CNT: c_int = 51;
    let len: c_uint = (1u32 << 15) - PUSH_CNT as u32 * 2 * 5 * 6;
    let insn = (*self_).fill_insns;
    let mut i: c_int = 0;
    let mut k: c_int = 0;

    *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_6, BPF_REG_1); i += 1;
    loop {
        for _j in 0..PUSH_CNT {
            *insn.add(i as usize) = BPF_LD_ABS(BPF_B, 0); i += 1;
            *insn.add(i as usize) = BPF_JMP32_IMM(BPF_JNE, BPF_REG_0, 0x34, len as c_int - i - 3); i += 1;
            *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_1, BPF_REG_6); i += 1;
            *insn.add(i as usize) = BPF_MOV64_IMM(BPF_REG_2, 1); i += 1;
            *insn.add(i as usize) = BPF_MOV64_IMM(BPF_REG_3, 2); i += 1;
            *insn.add(i as usize) = BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_skb_vlan_push); i += 1;
            *insn.add(i as usize) = BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, len as c_int - i - 3); i += 1;
        }
        for _j in 0..PUSH_CNT {
            *insn.add(i as usize) = BPF_LD_ABS(BPF_B, 0); i += 1;
            *insn.add(i as usize) = BPF_JMP32_IMM(BPF_JNE, BPF_REG_0, 0x34, len as c_int - i - 3); i += 1;
            *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_1, BPF_REG_6); i += 1;
            *insn.add(i as usize) = BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_skb_vlan_pop); i += 1;
            *insn.add(i as usize) = BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, len as c_int - i - 3); i += 1;
        }
        k += 1;
        if k >= 5 { break; }
    }
    while i < len as c_int - 3 {
        *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 0xbef);
        i += 1;
    }
    *insn.add((len - 3) as usize) = BPF_JMP_A(1);
    *insn.add((len - 2) as usize) = BPF_MOV32_IMM(BPF_REG_0, 0);
    *insn.add((len - 1) as usize) = BPF_EXIT_INSN();
    (*self_).prog_len = len as c_int;
}

unsafe extern "C" fn bpf_fill_jump_around_ld_abs(self_: *mut bpf_test) {
    let insn = (*self_).fill_insns;
    let len: c_uint = (1u32 << 15) / 7;
    let mut i: c_int = 0;

    *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_6, BPF_REG_1); i += 1;
    *insn.add(i as usize) = BPF_LD_ABS(BPF_B, 0); i += 1;
    *insn.add(i as usize) = BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 10, len as c_int - i - 2); i += 1;
    while i < len as c_int - 1 {
        *insn.add(i as usize) = BPF_LD_ABS(BPF_B, 1);
        i += 1;
    }
    *insn.add(i as usize) = BPF_EXIT_INSN();
    (*self_).prog_len = i + 1;
}

unsafe extern "C" fn bpf_fill_rand_ld_dw(self_: *mut bpf_test) {
    let insn = (*self_).fill_insns;
    let mut res: uint64_t = 0;
    let mut i: c_int = 0;
    *insn.add(i as usize) = BPF_MOV32_IMM(BPF_REG_0, 0); i += 1;
    while i < (*self_).testdata.single.retval as c_int {
        let val = bpf_semi_rand_get() as uint64_t;
        let tmp = BPF_LD_IMM64(BPF_REG_1, val);
        res ^= val;
        *insn.add(i as usize) = tmp[0]; i += 1;
        *insn.add(i as usize) = tmp[1]; i += 1;
        *insn.add(i as usize) = BPF_ALU64_REG(BPF_XOR, BPF_REG_0, BPF_REG_1); i += 1;
    }
    *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_1, BPF_REG_0); i += 1;
    *insn.add(i as usize) = BPF_ALU64_IMM(BPF_RSH, BPF_REG_1, 32); i += 1;
    *insn.add(i as usize) = BPF_ALU64_REG(BPF_XOR, BPF_REG_0, BPF_REG_1); i += 1;
    *insn.add(i as usize) = BPF_EXIT_INSN();
    (*self_).prog_len = i + 1;
    res ^= res >> 32;
    (*self_).testdata.single.retval = res as uint32_t;
}

const MAX_JMP_SEQ: c_int = 8192;

unsafe extern "C" fn bpf_fill_scale1(self_: *mut bpf_test) {
    let insn = (*self_).fill_insns;
    let mut i: c_int = 0;
    let mut k: c_int = 0;
    *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_6, BPF_REG_1); i += 1;
    while { k += 1; k <= MAX_JMP_SEQ } {
        *insn.add(i as usize) = BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_prandom_u32); i += 1;
        *insn.add(i as usize) = BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, bpf_semi_rand_get() as c_int, 2); i += 1;
        *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_1, BPF_REG_10); i += 1;
        *insn.add(i as usize) = BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_6, -8 * (k % 64 + 1)); i += 1;
    }
    while i < MAX_TEST_INSNS as c_int - MAX_JMP_SEQ * 4 {
        *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 42);
        i += 1;
    }
    *insn.add(i as usize) = BPF_EXIT_INSN();
    (*self_).prog_len = i + 1;
    (*self_).testdata.single.retval = 42;
}

unsafe extern "C" fn bpf_fill_scale2(self_: *mut bpf_test) {
    const FUNC_NEST: c_int = 7;
    let insn = (*self_).fill_insns;
    let mut i: c_int = 0;
    for _k in 0..FUNC_NEST {
        *insn.add(i as usize) = BPF_CALL_REL(1); i += 1;
        *insn.add(i as usize) = BPF_EXIT_INSN(); i += 1;
    }
    *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_6, BPF_REG_1); i += 1;
    let mut k: c_int = 0;
    while { k += 1; k <= MAX_JMP_SEQ } {
        *insn.add(i as usize) = BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_prandom_u32); i += 1;
        *insn.add(i as usize) = BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, bpf_semi_rand_get() as c_int, 2); i += 1;
        *insn.add(i as usize) = BPF_MOV64_REG(BPF_REG_1, BPF_REG_10); i += 1;
        *insn.add(i as usize) = BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_6, -8 * (k % (64 - 4 * FUNC_NEST) + 1)); i += 1;
    }
    while i < MAX_TEST_INSNS as c_int - MAX_JMP_SEQ * 4 {
        *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 42);
        i += 1;
    }
    *insn.add(i as usize) = BPF_EXIT_INSN();
    (*self_).prog_len = i + 1;
    (*self_).testdata.single.retval = 42;
}

unsafe extern "C" fn bpf_fill_scale(self_: *mut bpf_test) {
    match (*self_).testdata.single.retval {
        1 => bpf_fill_scale1(self_),
        2 => bpf_fill_scale2(self_),
        _ => (*self_).prog_len = 0,
    }
}

unsafe fn bpf_fill_torturous_jumps_insn_1(insn: *mut bpf_insn) -> c_int {
    let len: c_uint = 259;
    let hlen: c_uint = 128;
    *insn.add(0) = BPF_EMIT_CALL(BPF_FUNC_get_prandom_u32);
    for i in 1..=hlen {
        *insn.add(i as usize) = BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, i as c_int, hlen as c_int);
        *insn.add((i + hlen) as usize) = BPF_JMP_A((hlen - i) as c_int);
    }
    *insn.add((len - 2) as usize) = BPF_MOV64_IMM(BPF_REG_0, 1);
    *insn.add((len - 1) as usize) = BPF_EXIT_INSN();
    len as c_int
}

unsafe fn bpf_fill_torturous_jumps_insn_2(insn: *mut bpf_insn) -> c_int {
    let len: c_uint = 4100;
    let jmp_off: c_uint = 2048;
    let mut i: c_uint;
    *insn.add(0) = BPF_EMIT_CALL(BPF_FUNC_get_prandom_u32);
    i = 1;
    while i <= jmp_off {
        *insn.add(i as usize) = BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, i as c_int, jmp_off as c_int);
        i += 1;
    }
    *insn.add(i as usize) = BPF_JMP_A(jmp_off as c_int); i += 1;
    while i <= jmp_off * 2 + 1 {
        for j in 0..16 {
            *insn.add((i + j) as usize) = BPF_JMP_A((16 - j - 1) as c_int);
        }
        i += 16;
    }
    *insn.add((len - 2) as usize) = BPF_MOV64_IMM(BPF_REG_0, 2);
    *insn.add((len - 1) as usize) = BPF_EXIT_INSN();
    len as c_int
}

unsafe extern "C" fn bpf_fill_torturous_jumps(self_: *mut bpf_test) {
    let insn = (*self_).fill_insns;
    let mut i: c_int = 0;
    match (*self_).testdata.single.retval {
        1 => { (*self_).prog_len = bpf_fill_torturous_jumps_insn_1(insn); }
        2 => { (*self_).prog_len = bpf_fill_torturous_jumps_insn_2(insn); }
        3 => {
            *insn.add(i as usize) = BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 4); i += 1;
            *insn.add(i as usize) = BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 262); i += 1;
            *insn.add(i as usize) = BPF_ST_MEM(BPF_B, BPF_REG_10, -32, 0); i += 1;
            *insn.add(i as usize) = BPF_MOV64_IMM(BPF_REG_0, 3); i += 1;
            *insn.add(i as usize) = BPF_EXIT_INSN(); i += 1;
            i += bpf_fill_torturous_jumps_insn_1(insn.add(i as usize));
            i += bpf_fill_torturous_jumps_insn_2(insn.add(i as usize));
            (*self_).prog_len = i;
        }
        _ => (*self_).prog_len = 0,
    }
}

unsafe extern "C" fn bpf_fill_big_prog_with_loop_1(self_: *mut bpf_test) {
    let insn = (*self_).fill_insns;
    let len: c_int = getpagesize() - 25;
    let callback_load_idx: c_int;
    let callback_idx: c_int;
    let mut i: c_int = 0;
    *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_1, 1); i += 1;
    callback_load_idx = i;
    *insn.add(i as usize) = BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, BPF_REG_2, BPF_PSEUDO_FUNC, 0, 777); i += 1;
    *insn.add(i as usize) = BPF_RAW_INSN(0, 0, 0, 0, 0); i += 1;
    *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_3, 0); i += 1;
    *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_4, 0); i += 1;
    *insn.add(i as usize) = BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_loop); i += 1;
    while i < len - 3 {
        *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 0);
        i += 1;
    }
    *insn.add(i as usize) = BPF_EXIT_INSN(); i += 1;
    callback_idx = i;
    *insn.add(i as usize) = BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 0); i += 1;
    *insn.add(i as usize) = BPF_EXIT_INSN(); i += 1;
    (*insn.add(callback_load_idx as usize)).imm = callback_idx - callback_load_idx - 1;
    (*self_).func_info[1].insn_off = callback_idx as __u32;
    (*self_).prog_len = i;
    assert!(i == len);
}

// Macro translations preserving original expansion intent.
// BPF_SK_LOOKUP(func): 13 instruction sequence that initializes a tuple and calls BPF_FUNC_func.
// BPF_DIRECT_PKT_R2: 7 instruction sequence preparing direct packet access through r2.
// BPF_RAND_UEXT_R7: 4 instruction sequence initializing R7 to a random positive u32.
// BPF_RAND_SEXT_R7: 5 instruction sequence initializing R7 to a random negative u32.

// Original C uses:
// static struct bpf_test tests[] = {
// #define FILL_ARRAY
// #include <verifier/tests.h>
// #undef FILL_ARRAY
// };
// The generated verifier test data is an external build dependency.
extern "C" {
    static mut tests: [bpf_test; 0];
}

unsafe fn probe_filter_length(fp: *const bpf_insn) -> c_int {
    let mut len: c_int = MAX_INSNS as c_int - 1;
    while len > 0 {
        if (*fp.add(len as usize)).code != 0 || (*fp.add(len as usize)).imm != 0 {
            break;
        }
        len -= 1;
    }
    len + 1
}

unsafe fn skip_unsupported_map(map_type: bpf_map_type) -> bool {
    if !libbpf_probe_bpf_map_type(map_type, ptr::null()) {
        printf(b"SKIP (unsupported map type %d)\n\0".as_ptr() as *const c_char, map_type as c_int);
        skips += 1;
        return true;
    }
    false
}

unsafe fn __create_map(type_: uint32_t, size_key: uint32_t, size_value: uint32_t, max_elem: uint32_t, extra_flags: uint32_t) -> c_int {
    let mut opts: bpf_map_create_opts = zeroed();
    opts.map_flags = if type_ == BPF_MAP_TYPE_HASH as u32 { BPF_F_NO_PREALLOC } else { 0 } | extra_flags;
    let fd = bpf_map_create(type_ as bpf_map_type, ptr::null(), size_key, size_value, max_elem, &mut opts);
    if fd < 0 {
        if skip_unsupported_map(type_ as bpf_map_type) { return -1; }
        printf(b"Failed to create hash map '%s'!\n\0".as_ptr() as *const c_char, strerror(errno()));
    }
    fd
}

unsafe fn create_map(type_: uint32_t, size_key: uint32_t, size_value: uint32_t, max_elem: uint32_t) -> c_int {
    __create_map(type_, size_key, size_value, max_elem, 0)
}

unsafe fn update_map(fd: c_int, mut index: c_int) {
    let mut value: test_val = zeroed();
    value.index = ((6 + 1) * size_of::<c_int>()) as c_uint;
    value.foo[6] = 0xabcdef12u32 as c_int;
    assert!(bpf_map_update_elem(fd, &mut index as *mut _ as *const c_void, &mut value as *mut _ as *const c_void, 0) == 0);
}

unsafe fn create_prog_dummy_simple(prog_type: bpf_prog_type, ret: c_int) -> c_int {
    let prog = [BPF_MOV64_IMM(BPF_REG_0, ret), BPF_EXIT_INSN()];
    bpf_prog_load(prog_type, ptr::null(), b"GPL\0".as_ptr() as *const c_char, prog.as_ptr(), prog.len() as c_int, ptr::null_mut())
}

unsafe fn create_prog_dummy_loop(prog_type: bpf_prog_type, mfd: c_int, idx: c_int, ret: c_int) -> c_int {
    let prog = [
        BPF_MOV64_IMM(BPF_REG_3, idx),
        BPF_LD_MAP_FD(BPF_REG_2, mfd),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_tail_call),
        BPF_MOV64_IMM(BPF_REG_0, ret),
        BPF_EXIT_INSN(),
    ];
    bpf_prog_load(prog_type, ptr::null(), b"GPL\0".as_ptr() as *const c_char, prog.as_ptr(), prog.len() as c_int, ptr::null_mut())
}

unsafe fn create_prog_array(prog_type: bpf_prog_type, max_elem: uint32_t, mut p1key: c_int, mut p2key: c_int, mut p3key: c_int) -> c_int {
    let mut mfd = bpf_map_create(BPF_MAP_TYPE_PROG_ARRAY, ptr::null(), size_of::<c_int>() as u32, size_of::<c_int>() as u32, max_elem, ptr::null_mut());
    if mfd < 0 {
        if skip_unsupported_map(BPF_MAP_TYPE_PROG_ARRAY) { return -1; }
        printf(b"Failed to create prog array '%s'!\n\0".as_ptr() as *const c_char, strerror(errno()));
        return -1;
    }
    let mut p1fd = create_prog_dummy_simple(prog_type, 42);
    let mut p2fd = create_prog_dummy_loop(prog_type, mfd, p2key, 41);
    let mut p3fd = create_prog_dummy_simple(prog_type, 24);
    if p1fd < 0 || p2fd < 0 || p3fd < 0 ||
       bpf_map_update_elem(mfd, &mut p1key as *mut _ as *const c_void, &mut p1fd as *mut _ as *const c_void, BPF_ANY) < 0 ||
       bpf_map_update_elem(mfd, &mut p2key as *mut _ as *const c_void, &mut p2fd as *mut _ as *const c_void, BPF_ANY) < 0 ||
       bpf_map_update_elem(mfd, &mut p3key as *mut _ as *const c_void, &mut p3fd as *mut _ as *const c_void, BPF_ANY) < 0 {
        close(mfd);
        mfd = -1;
    }
    close(p3fd); close(p2fd); close(p1fd);
    mfd
}

unsafe fn create_map_in_map() -> c_int {
    let mut opts: bpf_map_create_opts = zeroed();
    let inner_map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, ptr::null(), size_of::<c_int>() as u32, size_of::<c_int>() as u32, 1, ptr::null_mut());
    if inner_map_fd < 0 {
        if skip_unsupported_map(BPF_MAP_TYPE_ARRAY) { return -1; }
        printf(b"Failed to create array '%s'!\n\0".as_ptr() as *const c_char, strerror(errno()));
        return inner_map_fd;
    }
    opts.inner_map_fd = inner_map_fd;
    let outer_map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY_OF_MAPS, ptr::null(), size_of::<c_int>() as u32, size_of::<c_int>() as u32, 1, &mut opts);
    if outer_map_fd < 0 {
        if skip_unsupported_map(BPF_MAP_TYPE_ARRAY_OF_MAPS) { return -1; }
        printf(b"Failed to create array of maps '%s'!\n\0".as_ptr() as *const c_char, strerror(errno()));
    }
    close(inner_map_fd);
    outer_map_fd
}

unsafe fn create_cgroup_storage(percpu: bool) -> c_int {
    let type_ = if percpu { BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE } else { BPF_MAP_TYPE_CGROUP_STORAGE };
    let fd = bpf_map_create(type_, ptr::null(), size_of::<bpf_cgroup_storage_key>() as u32, TEST_DATA_LEN as u32, 0, ptr::null_mut());
    if fd < 0 {
        if skip_unsupported_map(type_) { return -1; }
        printf(b"Failed to create cgroup storage '%s'!\n\0".as_ptr() as *const c_char, strerror(errno()));
    }
    fd
}

static btf_str_sec: &[u8] = b"\0bpf_spin_lock\0val\0cnt\0l\0bpf_timer\0timer\0t\0btf_ptr\0prog_test_ref_kfunc\0ptr\0kptr\0kptr_untrusted\0prog_test_member\0";

static mut btf_raw_types: [__u32; 21] = [
    BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 32, 4),
    BTF_TYPE_ENC(1, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 1), 4),
    BTF_MEMBER_ENC(15, 1, 0),
    BTF_TYPE_ENC(15, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 2), 8),
    BTF_MEMBER_ENC(19, 1, 0),
    BTF_MEMBER_ENC(23, 2, 32),
    BTF_TYPE_ENC(25, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 0), 16),
    BTF_TYPE_ENC(35, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 1), 16),
    BTF_MEMBER_ENC(41, 4, 0),
    BTF_STRUCT_ENC(51, 0, 0),
    BTF_STRUCT_ENC(95, 0, 0),
    BTF_TYPE_TAG_ENC(80, 6),
    BTF_TYPE_TAG_ENC(75, 6),
    BTF_TYPE_TAG_ENC(75, 7),
    BTF_PTR_ENC(8),
    BTF_PTR_ENC(9),
    BTF_PTR_ENC(10),
    BTF_STRUCT_ENC(43, 3, 24),
    BTF_MEMBER_ENC(71, 11, 0),
    BTF_MEMBER_ENC(71, 12, 64),
    BTF_MEMBER_ENC(71, 13, 128),
];

static mut bpf_vlog: [c_char; (u32::MAX >> 5) as usize] = [0; (u32::MAX >> 5) as usize];

unsafe fn load_btf_spec(types: *mut __u32, types_len: c_int, strings: *const c_char, strings_len: c_int) -> c_int {
    let hdr = btf_header {
        magic: BTF_MAGIC,
        version: BTF_VERSION,
        flags: 0,
        hdr_len: size_of::<btf_header>() as u32,
        type_off: 0,
        type_len: types_len as u32,
        str_off: types_len as u32,
        str_len: strings_len as u32,
    };
    let mut opts: bpf_btf_load_opts = zeroed();
    opts.log_buf = bpf_vlog.as_mut_ptr();
    opts.log_size = size_of_val(&bpf_vlog) as u32;
    opts.log_level = if verbose { verif_log_level } else { DEFAULT_LIBBPF_LOG_LEVEL } as u32;
    let raw_btf = malloc(size_of::<btf_header>() + types_len as usize + strings_len as usize);
    let mut ptr_ = raw_btf as *mut u8;
    memcpy(ptr_ as *mut c_void, &hdr as *const _ as *const c_void, size_of::<btf_header>());
    ptr_ = ptr_.add(size_of::<btf_header>());
    memcpy(ptr_ as *mut c_void, types as *const c_void, types_len as usize);
    ptr_ = ptr_.add(types_len as usize);
    memcpy(ptr_ as *mut c_void, strings as *const c_void, strings_len as usize);
    ptr_ = ptr_.add(strings_len as usize);
    let btf_fd = bpf_btf_load(raw_btf, ptr_.offset_from(raw_btf as *mut u8) as usize, &mut opts);
    if btf_fd < 0 {
        printf(b"Failed to load BTF spec: '%s'\n\0".as_ptr() as *const c_char, strerror(errno()));
    }
    free(raw_btf);
    if btf_fd < 0 { -1 } else { btf_fd }
}

unsafe fn load_btf() -> c_int {
    load_btf_spec(btf_raw_types.as_mut_ptr(), size_of_val(&btf_raw_types) as c_int, btf_str_sec.as_ptr() as *const c_char, btf_str_sec.len() as c_int)
}

unsafe fn load_btf_for_test(test: *mut bpf_test) -> c_int {
    let mut types_num = 0usize;
    while types_num < MAX_BTF_TYPES && (*test).btf_types[types_num] != BTF_END_RAW {
        types_num += 1;
    }
    let types_len = types_num * size_of::<__u32>();
    load_btf_spec((*test).btf_types.as_mut_ptr(), types_len as c_int, (*test).btf_strings.as_ptr(), size_of_val(&(*test).btf_strings) as c_int)
}

unsafe fn create_map_spin_lock() -> c_int {
    let mut opts: bpf_map_create_opts = zeroed();
    opts.btf_key_type_id = 1; opts.btf_value_type_id = 3;
    let btf_fd = load_btf();
    if btf_fd < 0 { return -1; }
    opts.btf_fd = btf_fd;
    let fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, b"test_map\0".as_ptr() as *const c_char, 4, 8, 1, &mut opts);
    if fd < 0 { printf(b"Failed to create map with spin_lock\n\0".as_ptr() as *const c_char); }
    fd
}

unsafe fn create_sk_storage_map() -> c_int {
    let mut opts: bpf_map_create_opts = zeroed();
    opts.map_flags = BPF_F_NO_PREALLOC; opts.btf_key_type_id = 1; opts.btf_value_type_id = 3;
    let btf_fd = load_btf();
    if btf_fd < 0 { return -1; }
    opts.btf_fd = btf_fd;
    let fd = bpf_map_create(BPF_MAP_TYPE_SK_STORAGE, b"test_map\0".as_ptr() as *const c_char, 4, 8, 0, &mut opts);
    close(opts.btf_fd);
    if fd < 0 { printf(b"Failed to create sk_storage_map\n\0".as_ptr() as *const c_char); }
    fd
}

unsafe fn create_map_timer() -> c_int {
    let mut opts: bpf_map_create_opts = zeroed();
    opts.btf_key_type_id = 1; opts.btf_value_type_id = 5;
    let btf_fd = load_btf();
    if btf_fd < 0 { return -1; }
    opts.btf_fd = btf_fd;
    let fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, b"test_map\0".as_ptr() as *const c_char, 4, 16, 1, &mut opts);
    if fd < 0 { printf(b"Failed to create map with timer\n\0".as_ptr() as *const c_char); }
    fd
}

unsafe fn create_map_kptr() -> c_int {
    let mut opts: bpf_map_create_opts = zeroed();
    opts.btf_key_type_id = 1; opts.btf_value_type_id = 14;
    let btf_fd = load_btf();
    if btf_fd < 0 { return -1; }
    opts.btf_fd = btf_fd;
    let fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, b"test_map\0".as_ptr() as *const c_char, 4, 24, 1, &mut opts);
    if fd < 0 { printf(b"Failed to create map with btf_id pointer\n\0".as_ptr() as *const c_char); }
    fd
}

unsafe fn set_root(set: bool) {
    let mut caps: __u64 = 0;
    if set {
        if cap_enable_effective(1u64 << CAP_SYS_ADMIN, &mut caps) != 0 {
            perror(b"cap_disable_effective(CAP_SYS_ADMIN)\0".as_ptr() as *const c_char);
        }
    } else if cap_disable_effective(1u64 << CAP_SYS_ADMIN, &mut caps) != 0 {
        perror(b"cap_disable_effective(CAP_SYS_ADMIN)\0".as_ptr() as *const c_char);
    }
}

unsafe fn ptr_to_u64(ptr_: *const c_void) -> __u64 {
    ptr_ as uintptr_t as __u64
}

unsafe fn btf__load_testmod_btf(vmlinux: *mut btf) -> *mut btf {
    let mut info: bpf_btf_info = zeroed();
    let mut len: __u32 = size_of::<bpf_btf_info>() as __u32;
    let mut btf_: *mut btf = ptr::null_mut();
    let mut name = [0 as c_char; 64];
    let mut id: __u32 = 0;
    set_root(true);
    loop {
        let mut err = bpf_btf_get_next_id(id, &mut id);
        if err != 0 {
            if errno() == ENOENT { break; }
            perror(b"bpf_btf_get_next_id failed\0".as_ptr() as *const c_char);
            break;
        }
        let fd = bpf_btf_get_fd_by_id(id);
        if fd < 0 {
            if errno() == ENOENT { continue; }
            perror(b"bpf_btf_get_fd_by_id failed\0".as_ptr() as *const c_char);
            break;
        }
        ptr::write_bytes(&mut info as *mut _, 0, 1);
        info.name_len = size_of_val(&name) as u32;
        info.name = ptr_to_u64(name.as_mut_ptr() as *const c_void);
        len = size_of::<bpf_btf_info>() as __u32;
        err = bpf_obj_get_info_by_fd(fd, &mut info, &mut len);
        if err != 0 {
            close(fd);
            perror(b"bpf_obj_get_info_by_fd failed\0".as_ptr() as *const c_char);
            break;
        }
        if strcmp(b"bpf_testmod\0".as_ptr() as *const c_char, name.as_ptr()) != 0 {
            close(fd);
            continue;
        }
        btf_ = btf__load_from_kernel_by_id_split(id, vmlinux);
        if btf_.is_null() {
            close(fd);
            break;
        }
        btf__set_fd(btf_, fd);
        break;
    }
    set_root(false);
    btf_
}

static mut testmod_btf: *mut btf = ptr::null_mut();
static mut vmlinux_btf: *mut btf = ptr::null_mut();

unsafe fn kfuncs_cleanup() {
    btf__free(testmod_btf);
    btf__free(vmlinux_btf);
}

unsafe fn fixup_prog_kfuncs(prog: *mut bpf_insn, fd_array: *mut c_int, mut fixup_kfunc_btf_id: *mut kfunc_btf_id_pair) {
    while !(*fixup_kfunc_btf_id).kfunc.is_null() {
        let mut btf_id: c_int = 0;
        if vmlinux_btf.is_null() { vmlinux_btf = btf__load_vmlinux_btf(); }
        if !vmlinux_btf.is_null() {
            btf_id = btf__find_by_name_kind(vmlinux_btf, (*fixup_kfunc_btf_id).kfunc, BTF_KIND_FUNC);
            if btf_id < 0 { btf_id = 0; }
        }
        if btf_id == 0 {
            if testmod_btf.is_null() { testmod_btf = btf__load_testmod_btf(vmlinux_btf); }
            if !testmod_btf.is_null() {
                btf_id = btf__find_by_name_kind(testmod_btf, (*fixup_kfunc_btf_id).kfunc, BTF_KIND_FUNC);
                if btf_id < 0 { btf_id = 0; }
                if btf_id != 0 {
                    *fd_array = btf__fd(testmod_btf);
                    (*prog.add((*fixup_kfunc_btf_id).insn_idx as usize)).off = 1;
                }
            }
        }
        (*prog.add((*fixup_kfunc_btf_id).insn_idx as usize)).imm = btf_id;
        fixup_kfunc_btf_id = fixup_kfunc_btf_id.add(1);
    }
}

unsafe fn patch_fixups(prog: *mut bpf_insn, mut fixups: *mut c_int, fd: c_int) {
    while *fixups != 0 {
        (*prog.add(*fixups as usize)).imm = fd;
        fixups = fixups.add(1);
    }
}

unsafe fn do_test_fixup(test: *mut bpf_test, prog_type: bpf_prog_type, prog: *mut bpf_insn, map_fds: *mut c_int, fd_array: *mut c_int) {
    if (*test).fill_helper.is_some() {
        (*test).fill_insns = calloc(MAX_TEST_INSNS, size_of::<bpf_insn>()) as *mut bpf_insn;
        ((*test).fill_helper.unwrap())(test);
    }
    if (*test).fixup_map_hash_8b[0] != 0 { *map_fds.add(0) = create_map(BPF_MAP_TYPE_HASH as u32, size_of::<i64>() as u32, size_of::<i64>() as u32, 1); patch_fixups(prog, (*test).fixup_map_hash_8b.as_mut_ptr(), *map_fds.add(0)); }
    if (*test).fixup_map_hash_48b[0] != 0 { *map_fds.add(1) = create_map(BPF_MAP_TYPE_HASH as u32, size_of::<i64>() as u32, size_of::<test_val>() as u32, 1); patch_fixups(prog, (*test).fixup_map_hash_48b.as_mut_ptr(), *map_fds.add(1)); }
    if (*test).fixup_map_hash_16b[0] != 0 { *map_fds.add(2) = create_map(BPF_MAP_TYPE_HASH as u32, size_of::<i64>() as u32, size_of::<other_val>() as u32, 1); patch_fixups(prog, (*test).fixup_map_hash_16b.as_mut_ptr(), *map_fds.add(2)); }
    if (*test).fixup_map_array_48b[0] != 0 { *map_fds.add(3) = create_map(BPF_MAP_TYPE_ARRAY as u32, size_of::<c_int>() as u32, size_of::<test_val>() as u32, 1); update_map(*map_fds.add(3), 0); patch_fixups(prog, (*test).fixup_map_array_48b.as_mut_ptr(), *map_fds.add(3)); }
    if (*test).fixup_prog1[0] != 0 { *map_fds.add(4) = create_prog_array(prog_type, 4, 0, 1, 2); patch_fixups(prog, (*test).fixup_prog1.as_mut_ptr(), *map_fds.add(4)); }
    if (*test).fixup_prog2[0] != 0 { *map_fds.add(5) = create_prog_array(prog_type, 8, 7, 1, 2); patch_fixups(prog, (*test).fixup_prog2.as_mut_ptr(), *map_fds.add(5)); }
    if (*test).fixup_map_in_map[0] != 0 { *map_fds.add(6) = create_map_in_map(); patch_fixups(prog, (*test).fixup_map_in_map.as_mut_ptr(), *map_fds.add(6)); }
    if (*test).fixup_cgroup_storage[0] != 0 { *map_fds.add(7) = create_cgroup_storage(false); patch_fixups(prog, (*test).fixup_cgroup_storage.as_mut_ptr(), *map_fds.add(7)); }
    if (*test).fixup_percpu_cgroup_storage[0] != 0 { *map_fds.add(8) = create_cgroup_storage(true); patch_fixups(prog, (*test).fixup_percpu_cgroup_storage.as_mut_ptr(), *map_fds.add(8)); }
    if (*test).fixup_map_sockmap[0] != 0 { *map_fds.add(9) = create_map(BPF_MAP_TYPE_SOCKMAP as u32, size_of::<c_int>() as u32, size_of::<c_int>() as u32, 1); patch_fixups(prog, (*test).fixup_map_sockmap.as_mut_ptr(), *map_fds.add(9)); }
    if (*test).fixup_map_sockhash[0] != 0 { *map_fds.add(10) = create_map(BPF_MAP_TYPE_SOCKHASH as u32, size_of::<c_int>() as u32, size_of::<c_int>() as u32, 1); patch_fixups(prog, (*test).fixup_map_sockhash.as_mut_ptr(), *map_fds.add(10)); }
    if (*test).fixup_map_xskmap[0] != 0 { *map_fds.add(11) = create_map(BPF_MAP_TYPE_XSKMAP as u32, size_of::<c_int>() as u32, size_of::<c_int>() as u32, 1); patch_fixups(prog, (*test).fixup_map_xskmap.as_mut_ptr(), *map_fds.add(11)); }
    if (*test).fixup_map_stacktrace[0] != 0 { *map_fds.add(12) = create_map(BPF_MAP_TYPE_STACK_TRACE as u32, size_of::<u32>() as u32, size_of::<u64>() as u32, 1); patch_fixups(prog, (*test).fixup_map_stacktrace.as_mut_ptr(), *map_fds.add(12)); }
    if (*test).fixup_map_spin_lock[0] != 0 { *map_fds.add(13) = create_map_spin_lock(); patch_fixups(prog, (*test).fixup_map_spin_lock.as_mut_ptr(), *map_fds.add(13)); }
    if (*test).fixup_map_array_ro[0] != 0 { *map_fds.add(14) = __create_map(BPF_MAP_TYPE_ARRAY as u32, size_of::<c_int>() as u32, size_of::<test_val>() as u32, 1, BPF_F_RDONLY_PROG); update_map(*map_fds.add(14), 0); patch_fixups(prog, (*test).fixup_map_array_ro.as_mut_ptr(), *map_fds.add(14)); }
    if (*test).fixup_map_array_wo[0] != 0 { *map_fds.add(15) = __create_map(BPF_MAP_TYPE_ARRAY as u32, size_of::<c_int>() as u32, size_of::<test_val>() as u32, 1, BPF_F_WRONLY_PROG); update_map(*map_fds.add(15), 0); patch_fixups(prog, (*test).fixup_map_array_wo.as_mut_ptr(), *map_fds.add(15)); }
    if (*test).fixup_map_array_small[0] != 0 { *map_fds.add(16) = __create_map(BPF_MAP_TYPE_ARRAY as u32, size_of::<c_int>() as u32, 1, 1, 0); update_map(*map_fds.add(16), 0); patch_fixups(prog, (*test).fixup_map_array_small.as_mut_ptr(), *map_fds.add(16)); }
    if (*test).fixup_sk_storage_map[0] != 0 { *map_fds.add(17) = create_sk_storage_map(); patch_fixups(prog, (*test).fixup_sk_storage_map.as_mut_ptr(), *map_fds.add(17)); }
    if (*test).fixup_map_event_output[0] != 0 { *map_fds.add(18) = __create_map(BPF_MAP_TYPE_PERF_EVENT_ARRAY as u32, size_of::<c_int>() as u32, size_of::<c_int>() as u32, 1, 0); patch_fixups(prog, (*test).fixup_map_event_output.as_mut_ptr(), *map_fds.add(18)); }
    if (*test).fixup_map_reuseport_array[0] != 0 { *map_fds.add(19) = __create_map(BPF_MAP_TYPE_REUSEPORT_SOCKARRAY as u32, size_of::<u32>() as u32, size_of::<u64>() as u32, 1, 0); patch_fixups(prog, (*test).fixup_map_reuseport_array.as_mut_ptr(), *map_fds.add(19)); }
    if (*test).fixup_map_ringbuf[0] != 0 { *map_fds.add(20) = create_map(BPF_MAP_TYPE_RINGBUF as u32, 0, 0, getpagesize() as u32); patch_fixups(prog, (*test).fixup_map_ringbuf.as_mut_ptr(), *map_fds.add(20)); }
    if (*test).fixup_map_timer[0] != 0 { *map_fds.add(21) = create_map_timer(); patch_fixups(prog, (*test).fixup_map_timer.as_mut_ptr(), *map_fds.add(21)); }
    if (*test).fixup_map_kptr[0] != 0 { *map_fds.add(22) = create_map_kptr(); patch_fixups(prog, (*test).fixup_map_kptr.as_mut_ptr(), *map_fds.add(22)); }
    fixup_prog_kfuncs(prog, fd_array, (*test).fixup_kfunc_btf_id.as_mut_ptr());
}

unsafe fn set_admin(admin: bool) -> c_int {
    let err = if admin { cap_enable_effective(ADMIN_CAPS, ptr::null_mut()) } else { cap_disable_effective(ADMIN_CAPS, ptr::null_mut()) };
    if err != 0 {
        perror(if admin { b"cap_enable_effective(ADMIN_CAPS)\0".as_ptr() } else { b"cap_disable_effective(ADMIN_CAPS)\0".as_ptr() } as *const c_char);
    }
    err
}

unsafe fn do_prog_test_run(fd_prog: c_int, unpriv: bool, expected_val: uint32_t, data: *mut c_void, size_data: size_t) -> c_int {
    let mut tmp = [0u8; TEST_DATA_LEN << 2];
    let size_tmp: __u32 = size_of_val(&tmp) as __u32;
    let mut topts: bpf_test_run_opts = zeroed();
    topts.data_in = data;
    topts.data_size_in = size_data as u32;
    topts.data_out = tmp.as_mut_ptr() as *mut c_void;
    topts.data_size_out = size_tmp;
    topts.repeat = 1;
    if unpriv { set_admin(true); }
    let err = bpf_prog_test_run_opts(fd_prog, &mut topts);
    let saved_errno = errno();
    if unpriv { set_admin(false); }
    if err != 0 {
        match saved_errno {
            ENOTSUPP => { printf(b"Did not run the program (not supported) \0".as_ptr() as *const c_char); return 0; }
            EPERM if unpriv => { printf(b"Did not run the program (no permission) \0".as_ptr() as *const c_char); return 0; }
            _ => { printf(b"FAIL: Unexpected bpf_prog_test_run error (%s) \0".as_ptr() as *const c_char, strerror(saved_errno)); return err; }
        }
    }
    if topts.retval != expected_val && expected_val != POINTER_VALUE {
        printf(b"FAIL retval %d != %d \0".as_ptr() as *const c_char, topts.retval, expected_val);
        return 1;
    }
    0
}

unsafe fn cmp_str_seq(mut log: *const c_char, mut exp: *const c_char) -> bool {
    let mut needle = [0 as c_char; 200];
    loop {
        if strlen(exp) == 0 { break; }
        let mut p = strchr(exp, b'\t' as c_int);
        if p.is_null() { p = exp.add(strlen(exp)); }
        let len = p.offset_from(exp) as usize;
        if len >= needle.len() || len == 0 {
            printf(b"FAIL\nTestcase bug\n\0".as_ptr() as *const c_char);
            return false;
        }
        memcpy(needle.as_mut_ptr() as *mut c_void, exp as *const c_void, len);
        needle[len] = 0;
        let q = strstr(log, needle.as_ptr());
        if q.is_null() {
            printf(b"FAIL\nUnexpected verifier log!\nEXP: %s\nRES:\n\0".as_ptr() as *const c_char, needle.as_ptr());
            return false;
        }
        log = q.add(len);
        exp = p.add(1);
        if *p == 0 { break; }
    }
    true
}

unsafe fn is_null_insn(insn: *mut bpf_insn) -> bool {
    let null_insn: bpf_insn = zeroed();
    memcmp(insn as *const c_void, &null_insn as *const _ as *const c_void, size_of::<bpf_insn>()) == 0
}

unsafe fn is_skip_insn(insn: *mut bpf_insn) -> bool {
    let skip_insn = SKIP_INSNS();
    memcmp(insn as *const c_void, &skip_insn as *const _ as *const c_void, size_of::<bpf_insn>()) == 0
}

unsafe fn null_terminated_insn_len(seq: *mut bpf_insn, max_len: c_int) -> c_int {
    for i in 0..max_len {
        if is_null_insn(seq.add(i as usize)) { return i; }
    }
    max_len
}

unsafe fn compare_masked_insn(orig: *mut bpf_insn, masked: *mut bpf_insn) -> bool {
    let mut orig_masked = *orig;
    if (*masked).imm == INSN_IMM_MASK { orig_masked.imm = INSN_IMM_MASK; }
    if (*masked).off == INSN_OFF_MASK { orig_masked.off = INSN_OFF_MASK; }
    memcmp(&orig_masked as *const _ as *const c_void, masked as *const c_void, size_of::<bpf_insn>()) == 0
}

unsafe fn find_insn_subseq(seq: *mut bpf_insn, subseq: *mut bpf_insn, seq_len: c_int, subseq_len: c_int) -> c_int {
    if subseq_len > seq_len { return -1; }
    for i in 0..(seq_len - subseq_len + 1) {
        let mut found = true;
        for j in 0..subseq_len {
            if !compare_masked_insn(seq.add((i + j) as usize), subseq.add(j as usize)) {
                found = false;
                break;
            }
        }
        if found { return i; }
    }
    -1
}

unsafe fn find_skip_insn_marker(seq: *mut bpf_insn, len: c_int) -> c_int {
    for i in 0..len {
        if is_skip_insn(seq.add(i as usize)) { return i; }
    }
    -1
}

unsafe fn find_all_insn_subseqs(mut seq: *mut bpf_insn, mut subseqs: *mut bpf_insn, mut seq_len: c_int, max_subseqs_len: c_int) -> bool {
    let mut subseqs_len = null_terminated_insn_len(subseqs, max_subseqs_len);
    while subseqs_len > 0 {
        let skip_idx = find_skip_insn_marker(subseqs, subseqs_len);
        let cur_subseq_len = if skip_idx < 0 { subseqs_len } else { skip_idx };
        let subseq_idx = find_insn_subseq(seq, subseqs, seq_len, cur_subseq_len);
        if subseq_idx < 0 { return false; }
        seq = seq.add((subseq_idx + cur_subseq_len) as usize);
        seq_len -= subseq_idx + cur_subseq_len;
        subseqs = subseqs.add((cur_subseq_len + 1) as usize);
        subseqs_len -= cur_subseq_len + 1;
    }
    true
}

unsafe fn print_insn(buf: *mut bpf_insn, cnt: c_int) {
    printf(b"  addr  op d s off  imm\n\0".as_ptr() as *const c_char);
    for i in 0..cnt {
        let insn = buf.add(i as usize);
        if is_null_insn(insn) { break; }
        if is_skip_insn(insn) {
            printf(b"  ...\n\0".as_ptr() as *const c_char);
        } else {
            printf(b"  %04x: %02x %1x %x %04hx %08x\n\0".as_ptr() as *const c_char, i, (*insn).code as c_int, (*insn).dst_reg() as c_int, (*insn).src_reg() as c_int, (*insn).off as c_int, (*insn).imm);
        }
    }
}

unsafe fn check_xlated_program(test: *mut bpf_test, fd_prog: c_int) -> bool {
    let mut buf: *mut bpf_insn = ptr::null_mut();
    let mut cnt: c_uint = 0;
    let mut result = true;
    let check_expected = !is_null_insn((*test).expected_insns.as_mut_ptr());
    let check_unexpected = !is_null_insn((*test).unexpected_insns.as_mut_ptr());
    if !check_expected && !check_unexpected { return result; }
    if get_xlated_program(fd_prog, &mut buf, &mut cnt) != 0 {
        printf(b"FAIL: can't get xlated program\n\0".as_ptr() as *const c_char);
        return false;
    }
    if check_expected && !find_all_insn_subseqs(buf, (*test).expected_insns.as_mut_ptr(), cnt as c_int, MAX_EXPECTED_INSNS as c_int) {
        printf(b"FAIL: can't find expected subsequence of instructions\n\0".as_ptr() as *const c_char);
        result = false;
        if verbose {
            printf(b"Program:\n\0".as_ptr() as *const c_char); print_insn(buf, cnt as c_int);
            printf(b"Expected subsequence:\n\0".as_ptr() as *const c_char); print_insn((*test).expected_insns.as_mut_ptr(), MAX_EXPECTED_INSNS as c_int);
        }
    }
    if check_unexpected && find_all_insn_subseqs(buf, (*test).unexpected_insns.as_mut_ptr(), cnt as c_int, MAX_UNEXPECTED_INSNS as c_int) {
        printf(b"FAIL: found unexpected subsequence of instructions\n\0".as_ptr() as *const c_char);
        result = false;
        if verbose {
            printf(b"Program:\n\0".as_ptr() as *const c_char); print_insn(buf, cnt as c_int);
            printf(b"Un-expected subsequence:\n\0".as_ptr() as *const c_char); print_insn((*test).unexpected_insns.as_mut_ptr(), MAX_UNEXPECTED_INSNS as c_int);
        }
    }
    free(buf as *mut c_void);
    result
}

unsafe fn do_test_single(test: *mut bpf_test, unpriv: bool, passes: *mut c_int, errors: *mut c_int) {
    let mut fd_prog: c_int = -1;
    let mut btf_fd: c_int = -1;
    let mut prog_type = (*test).prog_type;
    let mut prog = (*test).insns.as_mut_ptr();
    let mut opts: bpf_prog_load_opts = zeroed();
    let mut map_fds = [-1; MAX_NR_MAPS];
    let mut fd_array = [-1, -1];
    if ((*test).flags & F_NEEDS_JIT_ENABLED) != 0 && jit_disabled {
        printf(b"SKIP (requires BPF JIT)\n\0".as_ptr() as *const c_char);
        skips += 1; sched_yield(); return;
    }
    if prog_type as c_int == 0 { prog_type = BPF_PROG_TYPE_SOCKET_FILTER; }
    let fixup_skips = skips;
    do_test_fixup(test, prog_type, prog, map_fds.as_mut_ptr(), fd_array.as_mut_ptr().add(1));
    let prog_len = if !(*test).fill_insns.is_null() { prog = (*test).fill_insns; (*test).prog_len } else { probe_filter_length(prog) };
    if fixup_skips != skips { return; }
    let mut pflags = testing_prog_flags();
    if ((*test).flags & F_LOAD_WITH_STRICT_ALIGNMENT) != 0 { pflags |= BPF_F_STRICT_ALIGNMENT; }
    if ((*test).flags & F_NEEDS_EFFICIENT_UNALIGNED_ACCESS) != 0 { pflags |= BPF_F_ANY_ALIGNMENT; }
    if ((*test).flags & !3) != 0 { pflags |= (*test).flags as u32; }
    let expected_ret = if unpriv && (*test).result_unpriv != bpf_test_result::UNDEF { (*test).result_unpriv } else { (*test).result };
    let expected_err = if unpriv && !(*test).errstr_unpriv.is_null() { (*test).errstr_unpriv } else { (*test).errstr };
    opts.expected_attach_type = (*test).expected_attach_type;
    if expected_ret == bpf_test_result::VERBOSE_ACCEPT { opts.log_level = 2 | 4; }
    else if verbose { opts.log_level = (verif_log_level | 4) as u32; }
    else { opts.log_level = DEFAULT_LIBBPF_LOG_LEVEL as u32; }
    opts.prog_flags = pflags;
    if fd_array[1] != -1 { opts.fd_array = fd_array.as_mut_ptr(); }
    if (prog_type == BPF_PROG_TYPE_TRACING || prog_type == BPF_PROG_TYPE_LSM) && !(*test).kfunc.is_null() {
        let attach_btf_id = libbpf_find_vmlinux_btf_id((*test).kfunc, opts.expected_attach_type);
        if attach_btf_id < 0 {
            printf(b"FAIL\nFailed to find BTF ID for '%s'!\n\0".as_ptr() as *const c_char, (*test).kfunc);
            *errors += 1; return;
        }
        opts.attach_btf_id = attach_btf_id as u32;
    }
    if (*test).btf_types[0] != 0 {
        btf_fd = load_btf_for_test(test);
        if btf_fd < 0 { goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return; }
        opts.prog_btf_fd = btf_fd;
    }
    if (*test).func_info_cnt != 0 {
        opts.func_info = (*test).func_info.as_mut_ptr() as *mut c_void;
        opts.func_info_cnt = (*test).func_info_cnt as u32;
        opts.func_info_rec_size = size_of::<bpf_func_info>() as u32;
    }
    opts.log_buf = bpf_vlog.as_mut_ptr();
    opts.log_size = size_of_val(&bpf_vlog) as u32;
    fd_prog = bpf_prog_load(prog_type, ptr::null(), b"GPL\0".as_ptr() as *const c_char, prog, prog_len, &mut opts);
    let saved_errno = errno();
    if fd_prog < 0 && prog_type != BPF_PROG_TYPE_TRACING && !libbpf_probe_bpf_prog_type(prog_type, ptr::null()) {
        printf(b"SKIP (unsupported program type %d)\n\0".as_ptr() as *const c_char, prog_type as c_int);
        skips += 1; close_test_fds(test, fd_prog, btf_fd, &mut map_fds); return;
    }
    if fd_prog < 0 && saved_errno == ENOTSUPP {
        printf(b"SKIP (program uses an unsupported feature)\n\0".as_ptr() as *const c_char);
        skips += 1; close_test_fds(test, fd_prog, btf_fd, &mut map_fds); return;
    }
    let mut alignment_prevented_execution = 0;
    if expected_ret == bpf_test_result::ACCEPT || expected_ret == bpf_test_result::VERBOSE_ACCEPT {
        if fd_prog < 0 {
            printf(b"FAIL\nFailed to load prog '%s'!\n\0".as_ptr() as *const c_char, strerror(saved_errno));
            goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return;
        }
        // #ifndef CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS
        // If the surrounding build lacks efficient unaligned access, set alignment_prevented_execution.
        if expected_ret == bpf_test_result::VERBOSE_ACCEPT && !cmp_str_seq(bpf_vlog.as_ptr(), expected_err) {
            goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return;
        }
    } else {
        if fd_prog >= 0 {
            printf(b"FAIL\nUnexpected success to load!\n\0".as_ptr() as *const c_char);
            goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return;
        }
        if expected_err.is_null() || !cmp_str_seq(bpf_vlog.as_ptr(), expected_err) {
            printf(b"FAIL\nUnexpected error message!\n\tEXP: %s\n\tRES: %s\n\0".as_ptr() as *const c_char, expected_err, bpf_vlog.as_ptr());
            goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return;
        }
    }
    if !unpriv && (*test).insn_processed != 0 {
        let proc = strstr(bpf_vlog.as_ptr(), b"processed \0".as_ptr() as *const c_char);
        let insn_processed = atoi(proc.add(10)) as uint32_t;
        if (*test).insn_processed != insn_processed {
            printf(b"FAIL\nUnexpected insn_processed %u vs %u\n\0".as_ptr() as *const c_char, insn_processed, (*test).insn_processed);
            goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return;
        }
    }
    if verbose { printf(b", verifier log:\n%s\0".as_ptr() as *const c_char, bpf_vlog.as_ptr()); }
    if !check_xlated_program(test, fd_prog) { goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return; }
    let mut run_errs = 0;
    let mut run_successes = 0;
    if alignment_prevented_execution == 0 && fd_prog >= 0 && (*test).runs >= 0 {
        if (*test).runs == 0 { (*test).runs = 1; }
        for i in 0..(*test).runs {
            let rv = &mut (*test).testdata.retvals[i as usize];
            let expected_val = if unpriv && rv.retval_unpriv != 0 { rv.retval_unpriv } else { rv.retval };
            let err = do_prog_test_run(fd_prog, unpriv, expected_val, rv.data.as_mut_ptr() as *mut c_void, size_of_val(&rv.data));
            if err != 0 {
                printf(b"(run %d/%d) \0".as_ptr() as *const c_char, i + 1, (*test).runs);
                run_errs += 1;
            } else { run_successes += 1; }
        }
    }
    if run_errs == 0 {
        *passes += 1;
        if run_successes > 1 { printf(b"%d cases \0".as_ptr() as *const c_char, run_successes); }
        printf(b"OK\0".as_ptr() as *const c_char);
        if alignment_prevented_execution != 0 { printf(b" (NOTE: not executed due to unknown alignment)\0".as_ptr() as *const c_char); }
        printf(b"\n\0".as_ptr() as *const c_char);
    } else {
        printf(b"\n\0".as_ptr() as *const c_char);
        goto_fail_log(test, errors, fd_prog, btf_fd, &mut map_fds); return;
    }
    close_test_fds(test, fd_prog, btf_fd, &mut map_fds);
}

unsafe fn close_test_fds(test: *mut bpf_test, fd_prog: c_int, btf_fd: c_int, map_fds: &mut [c_int; MAX_NR_MAPS]) {
    if !(*test).fill_insns.is_null() { free((*test).fill_insns as *mut c_void); }
    close(fd_prog);
    close(btf_fd);
    for fd in map_fds.iter() { close(*fd); }
    sched_yield();
}

unsafe fn goto_fail_log(test: *mut bpf_test, errors: *mut c_int, fd_prog: c_int, btf_fd: c_int, map_fds: &mut [c_int; MAX_NR_MAPS]) {
    *errors += 1;
    printf(b"%s\0".as_ptr() as *const c_char, bpf_vlog.as_ptr());
    close_test_fds(test, fd_prog, btf_fd, map_fds);
}

unsafe fn is_admin() -> bool {
    let mut caps: __u64 = 0;
    if cap_disable_effective(1u64 << CAP_SYS_ADMIN, &mut caps) != 0 {
        perror(b"cap_disable_effective(CAP_SYS_ADMIN)\0".as_ptr() as *const c_char);
        return false;
    }
    (caps & ADMIN_CAPS) == ADMIN_CAPS
}

unsafe fn test_as_unpriv(test: *mut bpf_test) -> bool {
    // #ifndef CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS
    // On strict-alignment architectures, tests needing efficient unaligned access are not run unprivileged.
    (*test).prog_type as c_int == 0 ||
        (*test).prog_type == BPF_PROG_TYPE_SOCKET_FILTER ||
        (*test).prog_type == BPF_PROG_TYPE_CGROUP_SKB
}

unsafe fn do_test(unpriv: bool, from: c_uint, to: c_uint) -> c_int {
    let mut passes = 0;
    let mut errors = 0;
    unload_bpf_testmod(verbose);
    if load_bpf_testmod(verbose) != 0 { return EXIT_FAILURE; }
    for i in from..to {
        let test = tests.as_mut_ptr().add(i as usize);
        if test_as_unpriv(test) && unpriv_disabled {
            printf(b"#%d/u %s SKIP\n\0".as_ptr() as *const c_char, i, (*test).descr);
            skips += 1;
        } else if test_as_unpriv(test) {
            if !unpriv { set_admin(false); }
            printf(b"#%d/u %s \0".as_ptr() as *const c_char, i, (*test).descr);
            do_test_single(test, true, &mut passes, &mut errors);
            if !unpriv { set_admin(true); }
        }
        if unpriv {
            printf(b"#%d/p %s SKIP\n\0".as_ptr() as *const c_char, i, (*test).descr);
            skips += 1;
        } else {
            printf(b"#%d/p %s \0".as_ptr() as *const c_char, i, (*test).descr);
            do_test_single(test, false, &mut passes, &mut errors);
        }
    }
    unload_bpf_testmod(verbose);
    kfuncs_cleanup();
    printf(b"Summary: %d PASSED, %d SKIPPED, %d FAILED\n\0".as_ptr() as *const c_char, passes, skips, errors);
    if errors != 0 { EXIT_FAILURE } else { EXIT_SUCCESS }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut from: c_uint = 0;
    let mut to: c_uint = tests.len() as c_uint;
    let unpriv = !is_admin();
    let mut arg = 1;
    let mut argc_mut = argc;
    if argc_mut > 1 && strcmp(*argv.add(1), b"-v\0".as_ptr() as *const c_char) == 0 {
        arg += 1; verbose = true; verif_log_level = 1; argc_mut -= 1;
    }
    if argc_mut > 1 && strcmp(*argv.add(1), b"-vv\0".as_ptr() as *const c_char) == 0 {
        arg += 1; verbose = true; verif_log_level = 2; argc_mut -= 1;
    }
    if argc_mut == 3 {
        let l = atoi(*argv.add(arg as usize)) as c_uint;
        let u = atoi(*argv.add((arg + 1) as usize)) as c_uint;
        if l < to && u < to { from = l; to = u + 1; }
    } else if argc_mut == 2 {
        let t = atoi(*argv.add(arg as usize)) as c_uint;
        if t < to { from = t; to = t + 1; }
    }
    unpriv_disabled = get_unpriv_disabled();
    if unpriv && unpriv_disabled {
        printf(b"Cannot run as unprivileged user with sysctl %s.\n\0".as_ptr() as *const c_char, UNPRIV_SYSCTL.as_ptr());
        return EXIT_FAILURE;
    }
    jit_disabled = !is_jit_enabled();
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    bpf_semi_rand_init();
    do_test(unpriv, from, to)
}

// External dependency declarations and constants originally supplied by C headers.
#[repr(C)] pub struct btf { _private: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub struct bpf_func_info { pub insn_off: __u32, pub type_id: __u32 }
#[repr(C)] pub struct bpf_cgroup_storage_key { _private: [u8; 0] }
#[repr(C)] pub struct btf_header { pub magic: __u16, pub version: __u8, pub flags: __u8, pub hdr_len: __u32, pub type_off: __u32, pub type_len: __u32, pub str_off: __u32, pub str_len: __u32 }
type __u16 = u16;
pub type bpf_prog_type = c_int;
pub type bpf_map_type = c_int;
pub type bpf_attach_type = c_int;
#[repr(C)] pub struct bpf_map_create_opts { pub map_flags: __u32, pub inner_map_fd: c_int, pub btf_fd: c_int, pub btf_key_type_id: __u32, pub btf_value_type_id: __u32 }
#[repr(C)] pub struct bpf_btf_load_opts { pub log_buf: *mut c_char, pub log_size: __u32, pub log_level: __u32 }
#[repr(C)] pub struct bpf_test_run_opts { pub data_in: *mut c_void, pub data_size_in: __u32, pub data_out: *mut c_void, pub data_size_out: __u32, pub repeat: __u32, pub retval: __u32 }
#[repr(C)] pub struct bpf_prog_load_opts { pub expected_attach_type: bpf_attach_type, pub log_level: __u32, pub prog_flags: __u32, pub fd_array: *mut c_int, pub attach_btf_id: __u32, pub prog_btf_fd: c_int, pub func_info: *mut c_void, pub func_info_cnt: __u32, pub func_info_rec_size: __u32, pub log_buf: *mut c_char, pub log_size: __u32 }
#[repr(C)] pub struct bpf_btf_info { pub name_len: __u32, pub name: __u64 }

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn sched_yield() -> c_int;

    fn cap_enable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
    fn cap_disable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
    fn get_unpriv_disabled() -> bool;
    fn is_jit_enabled() -> bool;
    fn bpf_semi_rand_init();
    fn bpf_semi_rand_get() -> __u32;
    fn testing_prog_flags() -> __u32;
    fn unload_bpf_testmod(verbose: bool);
    fn load_bpf_testmod(verbose: bool) -> c_int;
    fn get_xlated_program(fd_prog: c_int, buf: *mut *mut bpf_insn, cnt: *mut c_uint) -> c_int;

    fn libbpf_probe_bpf_map_type(map_type: bpf_map_type, opts: *const c_void) -> bool;
    fn libbpf_probe_bpf_prog_type(prog_type: bpf_prog_type, opts: *const c_void) -> bool;
    fn libbpf_find_vmlinux_btf_id(name: *const c_char, attach_type: bpf_attach_type) -> c_int;
    fn libbpf_set_strict_mode(mode: c_int) -> c_int;
    fn bpf_map_create(map_type: bpf_map_type, name: *const c_char, key_size: __u32, value_size: __u32, max_entries: __u32, opts: *mut bpf_map_create_opts) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_prog_load(prog_type: bpf_prog_type, name: *const c_char, license: *const c_char, insns: *const bpf_insn, insn_cnt: c_int, opts: *mut bpf_prog_load_opts) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_btf_load(raw_btf: *const c_void, raw_btf_size: size_t, opts: *mut bpf_btf_load_opts) -> c_int;
    fn bpf_btf_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_btf_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut bpf_btf_info, info_len: *mut __u32) -> c_int;
    fn btf__load_from_kernel_by_id_split(id: __u32, base_btf: *mut btf) -> *mut btf;
    fn btf__set_fd(btf: *mut btf, fd: c_int);
    fn btf__free(btf: *mut btf);
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: __u32) -> c_int;
    fn btf__fd(btf: *mut btf) -> c_int;
}

unsafe fn errno() -> c_int { *__errno_location() }
extern "C" { fn __errno_location() -> *mut c_int; }

// Numeric constants and BPF instruction constructors are supplied by the
// translated build environment. Values below are declarations-by-use stand-ins
// for source-level translation and mirror the C dependency boundary.
const BPF_MAXINSNS: c_int = 4096;
const CAP_NET_ADMIN: u64 = 12;
const CAP_PERFMON: u64 = 38;
const CAP_BPF: u64 = 39;
const CAP_SYS_ADMIN: u64 = 21;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const ENOTSUPP: c_int = 524;
const BPF_ANY: __u64 = 0;
const BPF_F_NO_PREALLOC: __u32 = 1;
const BPF_F_RDONLY_PROG: __u32 = 1 << 7;
const BPF_F_WRONLY_PROG: __u32 = 1 << 8;
const BPF_F_STRICT_ALIGNMENT: __u32 = 1;
const BPF_F_ANY_ALIGNMENT: __u32 = 2;
const LIBBPF_STRICT_ALL: c_int = 0xffffffffu32 as c_int;
const BTF_MAGIC: __u16 = 0xeb9f;
const BTF_VERSION: __u8 = 1;
const BTF_INT_SIGNED: __u32 = 1;
const BTF_KIND_STRUCT: __u32 = 4;
const BTF_KIND_FUNC: __u32 = 12;
const BTF_END_RAW: __u32 = 0;

const BPF_REG_0: c_int = 0; const BPF_REG_1: c_int = 1; const BPF_REG_2: c_int = 2; const BPF_REG_3: c_int = 3; const BPF_REG_4: c_int = 4; const BPF_REG_5: c_int = 5; const BPF_REG_6: c_int = 6; const BPF_REG_7: c_int = 7; const BPF_REG_10: c_int = 10;
const BPF_B: c_int = 0x10; const BPF_W: c_int = 0x00; const BPF_DW: c_int = 0x18;
const BPF_JMP: c_int = 0x05; const BPF_CALL: c_int = 0x80; const BPF_JEQ: c_int = 0x10; const BPF_JNE: c_int = 0x50; const BPF_JLE: c_int = 0xb0;
const BPF_MOV: c_int = 0xb0; const BPF_ADD: c_int = 0x00; const BPF_XOR: c_int = 0xa0; const BPF_RSH: c_int = 0x70; const BPF_LSH: c_int = 0x60; const BPF_OR: c_int = 0x40; const BPF_ARSH: c_int = 0xc0;
const BPF_LD: c_int = 0x00; const BPF_IMM: c_int = 0x00; const BPF_PSEUDO_FUNC: c_int = 4;
const BPF_PROG_TYPE_SOCKET_FILTER: bpf_prog_type = 1;
const BPF_PROG_TYPE_CGROUP_SKB: bpf_prog_type = 8;
const BPF_PROG_TYPE_TRACING: bpf_prog_type = 26;
const BPF_PROG_TYPE_LSM: bpf_prog_type = 29;
const BPF_MAP_TYPE_HASH: bpf_map_type = 1;
const BPF_MAP_TYPE_ARRAY: bpf_map_type = 2;
const BPF_MAP_TYPE_PROG_ARRAY: bpf_map_type = 3;
const BPF_MAP_TYPE_PERF_EVENT_ARRAY: bpf_map_type = 4;
const BPF_MAP_TYPE_STACK_TRACE: bpf_map_type = 7;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: bpf_map_type = 12;
const BPF_MAP_TYPE_SOCKMAP: bpf_map_type = 15;
const BPF_MAP_TYPE_CGROUP_STORAGE: bpf_map_type = 19;
const BPF_MAP_TYPE_REUSEPORT_SOCKARRAY: bpf_map_type = 20;
const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: bpf_map_type = 21;
const BPF_MAP_TYPE_SOCKHASH: bpf_map_type = 18;
const BPF_MAP_TYPE_XSKMAP: bpf_map_type = 17;
const BPF_MAP_TYPE_SK_STORAGE: bpf_map_type = 24;
const BPF_MAP_TYPE_RINGBUF: bpf_map_type = 27;
const BPF_FUNC_skb_vlan_push: c_int = 18;
const BPF_FUNC_skb_vlan_pop: c_int = 19;
const BPF_FUNC_get_prandom_u32: c_int = 7;
const BPF_FUNC_loop: c_int = 181;
const BPF_FUNC_tail_call: c_int = 12;

const fn BTF_INFO_ENC(kind: __u32, kind_flag: __u32, vlen: __u32) -> __u32 { (kind << 24) | (kind_flag << 31) | vlen }
const fn BTF_TYPE_INT_ENC(name: __u32, encoding: __u32, offset: __u32, bits: __u32, size: __u32) -> __u32 { name ^ encoding ^ offset ^ bits ^ size }
const fn BTF_TYPE_ENC(name: __u32, info: __u32, size: __u32) -> __u32 { name ^ info ^ size }
const fn BTF_MEMBER_ENC(name: __u32, type_: __u32, bits_offset: __u32) -> __u32 { name ^ type_ ^ bits_offset }
const fn BTF_STRUCT_ENC(name: __u32, vlen: __u32, size: __u32) -> __u32 { name ^ vlen ^ size }
const fn BTF_TYPE_TAG_ENC(name: __u32, type_: __u32) -> __u32 { name ^ type_ }
const fn BTF_PTR_ENC(type_: __u32) -> __u32 { type_ }

unsafe fn BPF_RAW_INSN(code: c_int, dst: c_int, src: c_int, off: c_int, imm: c_int) -> bpf_insn { bpf_insn { code: code as u8, regs: ((src as u8) << 4) | (dst as u8 & 0xf), off: off as i16, imm } }
unsafe fn BPF_MOV64_REG(dst: c_int, src: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_MOV, dst, src, 0, 0) }
unsafe fn BPF_MOV64_IMM(dst: c_int, imm: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_MOV, dst, 0, 0, imm) }
unsafe fn BPF_MOV32_IMM(dst: c_int, imm: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_MOV, dst, 0, 0, imm) }
unsafe fn BPF_LD_ABS(size: c_int, imm: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_LD | size, 0, 0, 0, imm) }
unsafe fn BPF_JMP32_IMM(op: c_int, dst: c_int, imm: c_int, off: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_JMP | op, dst, 0, off, imm) }
unsafe fn BPF_JMP_IMM(op: c_int, dst: c_int, imm: c_int, off: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_JMP | op, dst, 0, off, imm) }
unsafe fn BPF_JMP_A(off: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_JMP, 0, 0, off, 0) }
unsafe fn BPF_EXIT_INSN() -> bpf_insn { BPF_RAW_INSN(BPF_JMP | 0x90, 0, 0, 0, 0) }
unsafe fn BPF_ALU64_IMM(op: c_int, dst: c_int, imm: c_int) -> bpf_insn { BPF_RAW_INSN(op, dst, 0, 0, imm) }
unsafe fn BPF_ALU64_REG(op: c_int, dst: c_int, src: c_int) -> bpf_insn { BPF_RAW_INSN(op, dst, src, 0, 0) }
unsafe fn BPF_STX_MEM(size: c_int, dst: c_int, src: c_int, off: c_int) -> bpf_insn { BPF_RAW_INSN(size, dst, src, off, 0) }
unsafe fn BPF_ST_MEM(size: c_int, dst: c_int, off: c_int, imm: c_int) -> bpf_insn { BPF_RAW_INSN(size, dst, 0, off, imm) }
unsafe fn BPF_CALL_REL(imm: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, imm) }
unsafe fn BPF_EMIT_CALL(func: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, func) }
unsafe fn BPF_LD_MAP_FD(dst: c_int, fd: c_int) -> bpf_insn { BPF_RAW_INSN(BPF_LD | BPF_DW | BPF_IMM, dst, 1, 0, fd) }
unsafe fn BPF_LD_IMM64(dst: c_int, imm: u64) -> [bpf_insn; 2] { [BPF_RAW_INSN(BPF_LD | BPF_DW | BPF_IMM, dst, 0, 0, imm as i32), BPF_RAW_INSN(0, 0, 0, 0, (imm >> 32) as i32)] }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
