// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/array_access.c */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;
use core::mem::offset_of;
use core::ptr;

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;
const BPF_F_RDONLY_PROG: u32 = 1 << 7;
const BPF_F_WRONLY_PROG: u32 = 1 << 8;
const BPF_F_ANY_ALIGNMENT: u32 = 2;

const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub map_flags: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static map_array_ro: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<test_val>() as u32,
    map_flags: BPF_F_RDONLY_PROG,
};

#[link_section = ".maps"]
#[no_mangle]
pub static map_array_wo: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<test_val>() as u32,
    map_flags: BPF_F_WRONLY_PROG,
};

#[link_section = ".maps"]
#[no_mangle]
pub static map_array_pcpu: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<test_val>() as u32,
    map_flags: 0,
};

#[link_section = ".maps"]
#[no_mangle]
pub static map_array: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<test_val>() as u32,
    map_flags: 0,
};

#[link_section = ".maps"]
#[no_mangle]
pub static map_hash_48b: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i64>() as u32,
    value_size: core::mem::size_of::<test_val>() as u32,
    map_flags: 0,
};

extern "C" {
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut test_val;
    fn bpf_probe_read_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i64;
    fn bpf_get_prandom_u32() -> __u32;
}

const test_val_foo: usize = offset_of!(test_val, foo);

// SEC("socket")
// __description("valid map access into an array with a constant")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("valid map access into an array with a register")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_register_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = 4",
        "r1 <<= 2",
        "r0 += r1",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("valid map access into an array with a variable")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_variable_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = *(u32*)(r0 + 0)",
        "if r1 >= {max_entries} goto 1f",
        "r1 <<= 2",
        "r0 += r1",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        max_entries = const MAX_ENTRIES,
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("valid map access into an array with a signed variable")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn array_with_a_signed_variable() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 3f",
        "r1 = *(u32*)(r0 + 0)",
        "if w1 s> 0xffffffff goto 1f",
        "w1 = 0",
        "1:",
        "w2 = {max_entries}",
        "if r2 s> r1 goto 2f",
        "w1 = 0",
        "2:",
        "w1 <<= 2",
        "r0 += r1",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "3:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        max_entries = const MAX_ENTRIES,
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("invalid map access into an array with a constant")
// __failure __msg("invalid access to map value, value_size=48 off=48 size=8")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + {imm_0}) = r1",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        imm_0 = const ((MAX_ENTRIES + 1) << 2),
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("invalid map access into an array with a register")
// __failure __msg("R0 min value is outside of the allowed memory range")
// __failure_unpriv
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_register_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = {imm_0}",
        "r1 <<= 2",
        "r0 += r1",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        imm_0 = const (MAX_ENTRIES + 1),
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("invalid map access into an array with a variable")
// __failure
// __msg("R0 unbounded memory access, make sure to bounds check any such access")
// __failure_unpriv
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_variable_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = *(u32*)(r0 + 0)",
        "r1 <<= 2",
        "r0 += r1",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("invalid map access into an array with no floor check")
// __failure __msg("R0 unbounded memory access")
// __failure_unpriv __msg_unpriv("R0 leaks addr")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn array_with_no_floor_check() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 2f",
        "r1 = *(u64*)(r0 + 0)",
        "w2 = {max_entries}",
        "if r2 s> r1 goto 1f",
        "w1 = 0",
        "1:",
        "w1 <<= 2",
        "r0 += r1",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "2:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        max_entries = const MAX_ENTRIES,
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("invalid map access into an array with a invalid max check")
// __failure __msg("invalid access to map value, value_size=48 off=44 size=8")
// __failure_unpriv __msg_unpriv("R0 leaks addr")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn with_a_invalid_max_check_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 2f",
        "r1 = *(u32*)(r0 + 0)",
        "w2 = {imm_0}",
        "if r2 > r1 goto 1f",
        "w1 = 0",
        "1:",
        "w1 <<= 2",
        "r0 += r1",
        "r1 = {test_val_foo}",
        "*(u64*)(r0 + 0) = r1",
        "2:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        imm_0 = const (MAX_ENTRIES + 1),
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("invalid map access into an array with a invalid max check")
// __failure __msg("R0 pointer += pointer")
// __failure_unpriv
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn with_a_invalid_max_check_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r8 = r0",
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r0 += r8",
        "r0 = *(u32*)(r0 + {test_val_foo})",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        test_val_foo = const test_val_foo,
    );
}

// SEC("socket")
// __description("valid read map access into a read-only array 1")
// __success __success_unpriv __retval(28)
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_1_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_ro} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r0 = *(u32*)(r0 + 0)",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_array_ro = sym map_array_ro,
    );
}

extern "C" {
    fn bpf_csum_diff(from: *const c_void, from_size: u32, to: *const c_void, to_size: u32, seed: u32) -> i64;
    fn bpf_skb_load_bytes(skb: *mut c_void, offset: u32, to: *mut c_void, len: u32) -> i64;
}

// SEC("tc")
// __description("valid read map access into a read-only array 2")
// __success __retval(65507)
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_2_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_ro} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = r0",
        "r2 = 4",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "1:",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_array_ro = sym map_array_ro,
    );
}

// SEC("socket")
// __description("invalid write map access into a read-only array 1")
// __failure __msg("write into map forbidden")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_1_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_ro} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = 42",
        "*(u64*)(r0 + 0) = r1",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_array_ro = sym map_array_ro,
    );
}

// SEC("tc")
// __description("invalid write map access into a read-only array 2")
// __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_2_2() {
    asm!(
        "r6 = r1",
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_ro} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = r6",
        "r2 = 0",
        "r3 = r0",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        map_array_ro = sym map_array_ro,
    );
}

// SEC("socket")
// __description("valid write map access into a write-only array 1")
// __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_1_1() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_wo} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = 42",
        "*(u64*)(r0 + 0) = r1",
        "1:",
        "r0 = 1",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_array_wo = sym map_array_wo,
    );
}

// SEC("tc")
// __description("valid write map access into a write-only array 2")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_2_1() {
    asm!(
        "r6 = r1",
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_wo} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = r6",
        "r2 = 0",
        "r3 = r0",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        map_array_wo = sym map_array_wo,
    );
}

// SEC("socket")
// __description("invalid read map access into a write-only array 1")
// __failure __msg("read from map forbidden")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_1_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_wo} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r0 = *(u64*)(r0 + 0)",
        "1:",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_array_wo = sym map_array_wo,
    );
}

// SEC("tc")
// __description("invalid read map access into a write-only array 2")
// __failure __msg("read from map forbidden")
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_2_2() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_array_wo} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 == 0 goto 1f",
        "r1 = r0",
        "r2 = 4",
        "r3 = 0",
        "r4 = 0",
        "r5 = 0",
        "call {bpf_csum_diff}",
        "1:",
        "exit",
        bpf_csum_diff = sym bpf_csum_diff,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_array_wo = sym map_array_wo,
    );
}

// SEC("socket")
// __description("valid map access into an array using constant without nullness")
// __success __retval(4) __log_level(2)
// __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -8) = {{(1|r[0-9])}}")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_no_nullness() -> u32 {
    /* Need 8-byte alignment for spill tracking */
    #[repr(align(8))]
    struct AlignedU32(__u32);
    let mut key = AlignedU32(1);
    let val: *mut test_val;

    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key.0 as *mut _ as *const c_void);
    (*val).index = test_val_foo as u32;

    (*val).index
}

// SEC("socket")
// __description("valid multiple map access into an array using constant without nullness")
// __success __retval(8) __log_level(2)
// __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -16) = {{(0|r[0-9])}}")
// __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -8) = {{(1|r[0-9])}}")
#[no_mangle]
pub unsafe extern "C" fn multiple_array_with_a_constant_no_nullness() -> u32 {
    #[repr(align(8))]
    struct AlignedU32(__u32);
    let mut key = AlignedU32(1);
    let mut key2 = AlignedU32(0);
    let val: *mut test_val;
    let val2: *mut test_val;

    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key.0 as *mut _ as *const c_void);
    (*val).index = test_val_foo as u32;

    val2 = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key2.0 as *mut _ as *const c_void);
    (*val2).index = test_val_foo as u32;

    (*val).index.wrapping_add((*val2).index)
}

// SEC("socket")
// __description("valid map access into an array using natural aligned 32-bit constant 0 without nullness")
// __success __retval(4)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_32bit_constant_0_no_nullness() -> u32 {
    /* Unlike the above tests, 32-bit zeroing is precisely tracked even
     * if writes are not aligned to BPF_REG_SIZE. This tests that our
     * STACK_ZERO handling functions.
     */
    let val: *mut test_val;
    let mut key: __u32 = 0;

    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key as *mut _ as *const c_void);
    (*val).index = test_val_foo as u32;

    (*val).index
}

// SEC("socket")
// __description("valid map access into a pcpu array using constant without nullness")
// __success __retval(4) __log_level(2)
// __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -8) = {{(1|r[0-9])}}")
#[no_mangle]
pub unsafe extern "C" fn a_pcpu_array_with_a_constant_no_nullness() -> u32 {
    #[repr(align(8))]
    struct AlignedU32(__u32);
    let mut key = AlignedU32(1);
    let val: *mut test_val;

    val = bpf_map_lookup_elem(&map_array_pcpu as *const _ as *const c_void, &mut key.0 as *mut _ as *const c_void);
    (*val).index = test_val_foo as u32;

    (*val).index
}

// SEC("socket")
// __description("invalid map access into an array using constant without nullness")
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_no_nullness_out_of_bounds() -> u32 {
    /* Out of bounds */
    #[repr(align(8))]
    struct AlignedU32(__u32);
    let mut key = AlignedU32(3);
    let val: *mut test_val;

    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key.0 as *mut _ as *const c_void);
    (*val).index = test_val_foo as u32;

    (*val).index
}

// SEC("socket")
// __description("invalid map access into an array using constant smaller than key_size")
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_too_small() -> u32 {
    #[repr(align(8))]
    struct AlignedU32(__u32);
    let mut key = AlignedU32(0);
    let val: *mut test_val;

    /* Mark entire key as STACK_MISC */
    bpf_probe_read_user(&mut key.0 as *mut _ as *mut c_void, core::mem::size_of_val(&key.0) as u32, ptr::null());

    /* Spilling only the bottom byte results in a tnum const of 1.
     * We want to check that the verifier rejects it, as the spill is < 4B.
     */
    *(&mut key.0 as *mut __u32 as *mut __u8) = 1;
    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key.0 as *mut _ as *const c_void);

    /* Should fail, as verifier cannot prove in-bound lookup */
    (*val).index = test_val_foo as u32;

    (*val).index
}

// SEC("socket")
// __description("invalid map access into an array using constant larger than key_size")
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_too_big() -> u32 {
    let val: *mut test_val;
    let mut key: __u64 = 1;

    /* Even if the constant value is < max_entries, if the spill size is
     * larger than the key size, the set bits may not be where we expect them
     * to be on different endian architectures.
     */
    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key as *mut _ as *const c_void);
    (*val).index = test_val_foo as u32;

    (*val).index
}

// SEC("socket")
// __description("invalid elided lookup using const and non-const key")
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn mixed_const_and_non_const_key_lookup() -> u32 {
    #[repr(align(8))]
    struct AlignedU32(__u32);
    let mut key = AlignedU32(0);
    let val: *mut test_val;
    let rand: __u32;

    rand = bpf_get_prandom_u32();
    key.0 = if rand > 42 { 1 } else { rand };
    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &mut key.0 as *mut _ as *const c_void);

    (*val).index
}

// SEC("socket")
// __failure __msg("invalid read from stack R2 off=4096 size=4")
#[no_mangle]
pub unsafe extern "C" fn key_lookup_at_invalid_fp() {
    asm!(
        "r1 = {map_array} ll",
        "r2 = r10",
        "r2 += 4096",
        "call {bpf_map_lookup_elem}",
        "r0 = *(u64*)(r0 + 0)",
        "exit",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_array = sym map_array,
    );
}

#[repr(align(8))]
pub struct AlignedGlobalKey(pub __u32);

#[no_mangle]
pub static mut global_key: AlignedGlobalKey = AlignedGlobalKey(0);

// SEC("socket")
// __description("invalid elided lookup using non-stack key")
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn non_stack_key_lookup() -> u32 {
    let val: *mut test_val;

    ptr::write_volatile(&mut global_key.0, 1);
    val = bpf_map_lookup_elem(&map_array as *const _ as *const c_void, &raw const global_key as *const c_void);
    (*val).index = test_val_foo as u32;

    (*val).index
}

// SEC("socket")
// __description("doesn't reject UINT64_MAX as s64 for irrelevant maps")
// __success __retval(42)
#[no_mangle]
pub unsafe extern "C" fn doesnt_reject_irrelevant_maps() -> u32 {
    let mut key: __u64 = 0xFFFFFFFFFFFFFFFF;
    let val: *mut test_val;

    val = bpf_map_lookup_elem(&map_hash_48b as *const _ as *const c_void, &mut key as *mut _ as *const c_void);
    if !val.is_null() {
        return (*val).index;
    }

    42
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
