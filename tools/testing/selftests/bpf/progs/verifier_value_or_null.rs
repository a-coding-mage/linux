// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/value_or_null.c */

// C dependencies translated as external intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

pub const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: core::ffi::c_uint,
    pub foo: [core::ffi::c_int; MAX_ENTRIES],
}

// Original BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, long long);
//     __type(value, struct test_val);
// } map_hash_48b SEC(".maps");
#[used]
#[unsafe(link_section = ".maps")]
pub static mut map_hash_48b: bpf_map_def_map_hash_48b = bpf_map_def_map_hash_48b {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i64>() as u32,
    value_size: core::mem::size_of::<test_val>() as u32,
};

// Original BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, long long);
//     __type(value, long long);
// } map_hash_8b SEC(".maps");
#[used]
#[unsafe(link_section = ".maps")]
pub static mut map_hash_8b: bpf_map_def_map_hash_8b = bpf_map_def_map_hash_8b {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i64>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

// External constants and helpers are supplied by the BPF selftest environment.
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_F_ANY_ALIGNMENT: u32 = 2;
pub const BPF_F_TEST_STATE_FREQ: u32 = 4;

#[repr(C)]
pub struct bpf_map_def_map_hash_48b {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[repr(C)]
pub struct bpf_map_def_map_hash_8b {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

unsafe extern "C" {
    pub fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn bpf_ktime_get_ns() -> u64;
}

// SEC("tc")
// __description("multiple registers share map_lookup_elem result")
// __success __retval(0)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn share_map_lookup_elem_result() {
    core::arch::asm!(
        r#"
        r1 = 10
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        call {bpf_map_lookup_elem}
        r4 = r0
        if r0 == 0 goto 0f
        r1 = 0
        *(u64*)(r4 + 0) = r1
0:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("alu ops on ptr_to_map_value_or_null, 1")
// __failure __msg("R4 pointer arithmetic on map_value_or_null")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_value_or_null_1() {
    core::arch::asm!(
        r#"
        r1 = 10
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        call {bpf_map_lookup_elem}
        r4 = r0
        r4 += -2
        r4 += 2
        if r0 == 0 goto 0f
        r1 = 0
        *(u64*)(r4 + 0) = r1
0:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("alu ops on ptr_to_map_value_or_null, 2")
// __failure __msg("R4 pointer arithmetic on map_value_or_null")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_value_or_null_2() {
    core::arch::asm!(
        r#"
        r1 = 10
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        call {bpf_map_lookup_elem}
        r4 = r0
        r4 &= -1
        if r0 == 0 goto 0f
        r1 = 0
        *(u64*)(r4 + 0) = r1
0:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("alu ops on ptr_to_map_value_or_null, 3")
// __failure __msg("R4 pointer arithmetic on map_value_or_null")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_value_or_null_3() {
    core::arch::asm!(
        r#"
        r1 = 10
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        call {bpf_map_lookup_elem}
        r4 = r0
        r4 <<= 1
        if r0 == 0 goto 0f
        r1 = 0
        *(u64*)(r4 + 0) = r1
0:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("invalid memory access with multiple map_lookup_elem calls")
// __failure __msg("R4 !read_ok")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multiple_map_lookup_elem_calls() {
    core::arch::asm!(
        r#"
        r1 = 10
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        r8 = r1
        r7 = r2
        call {bpf_map_lookup_elem}
        r4 = r0
        r1 = r8
        r2 = r7
        call {bpf_map_lookup_elem}
        if r0 == 0 goto 0f
        r1 = 0
        *(u64*)(r4 + 0) = r1
0:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("tc")
// __description("valid indirect map_lookup_elem access with 2nd lookup in branch")
// __success __retval(0)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn with_2nd_lookup_in_branch() {
    core::arch::asm!(
        r#"
        r1 = 10
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        r8 = r1
        r7 = r2
        call {bpf_map_lookup_elem}
        r2 = 10
        if r2 != 0 goto 0f
        r1 = r8
        r2 = r7
        call {bpf_map_lookup_elem}
0:      r4 = r0
        if r0 == 0 goto 1f
        r1 = 0
        *(u64*)(r4 + 0) = r1
1:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("socket")
// __description("invalid map access from else condition")
// __failure __msg("R0 unbounded memory access")
// __failure_unpriv __msg_unpriv("R0 leaks addr")
// __flag(BPF_F_ANY_ALIGNMENT)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_access_from_else_condition() {
    core::arch::asm!(
        r#"
        r1 = 0
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_48b} ll
        call {bpf_map_lookup_elem}
        if r0 == 0 goto 0f
        r1 = *(u32*)(r0 + 0)
        if r1 >= {max_entries_minus_one} goto 1f
        r1 += 1
1:      r1 <<= 2
        r0 += r1
        r1 = {test_val_foo}
        *(u64*)(r0 + 0) = r1
0:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_48b = sym map_hash_48b,
        max_entries_minus_one = const (MAX_ENTRIES - 1),
        test_val_foo = const core::mem::offset_of!(test_val, foo),
        options(noreturn)
    );
}

// SEC("tc")
// __description("map lookup and null branch prediction")
// __success __retval(0)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_and_null_branch_prediction() {
    core::arch::asm!(
        r#"
        r1 = 10
        *(u64*)(r10 - 8) = r1
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        call {bpf_map_lookup_elem}
        r6 = r0
        if r6 == 0 goto 0f
        if r6 != 0 goto 0f
        r10 += 10
0:      exit
        "#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("MAP_VALUE_OR_NULL check_ids() in regsafe()")
// __failure __msg("R8 invalid mem access 'map_value_or_null'")
// __failure_unpriv __msg_unpriv("")
// __flag(BPF_F_TEST_STATE_FREQ)
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn null_check_ids_in_regsafe() {
    core::arch::asm!(
        r#"
        r1 = 0
        *(u64*)(r10 - 8) = r1
        /* r9 = map_lookup_elem(...) */
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        call {bpf_map_lookup_elem}
        r9 = r0
        /* r8 = map_lookup_elem(...) */
        r2 = r10
        r2 += -8
        r1 = {map_hash_8b} ll
        call {bpf_map_lookup_elem}
        r8 = r0
        /* r7 = ktime_get_ns() */
        call {bpf_ktime_get_ns}
        r7 = r0
        /* r6 = ktime_get_ns() */
        call {bpf_ktime_get_ns}
        r6 = r0
        /*
         * if r6 > r7 goto +1    ; no new information about the state is derived from
         *                       ; this check, thus produced verifier states differ
         *                       ; only in 'insn_idx'
         * r9 = r8               ; optionally share ID between r9 and r8
         */
        if r6 > r7 goto 0f
        r9 = r8
0:      /* if r9 == 0 goto <exit> */
        if r9 == 0 goto 1f
        /*
         * read map value via r8, this is not always
         * safe because r8 might be not equal to r9.
         */
        r0 = *(u64*)(r8 + 0)
1:      /* exit 0 */
        r0 = 0
        exit
        "#,
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[used]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
