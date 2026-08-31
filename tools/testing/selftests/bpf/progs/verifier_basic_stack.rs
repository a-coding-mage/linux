// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/basic_stack.c */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;

/*
 * C dependencies:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

pub const BPF_MAP_TYPE_HASH: u32 = 1;

#[repr(C)]
pub struct map_hash_8b_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static map_hash_8b: map_hash_8b_def = map_hash_8b_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i64>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

// SEC("socket")
// __description("stack out of bounds")
// __failure __msg("invalid write to stack")
// __failure_unpriv
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn stack_out_of_bounds() {
    asm!(
        "r1 = 0",
        "*(u64*)(r10 + 8) = r1",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("uninitialized stack1")
// __success __log_level(4)
// __msg("subprog 0 (uninitialized_stack1) main {{.*}} stack 8")
// __failure_unpriv __msg_unpriv("invalid read from stack")
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn uninitialized_stack1() {
    asm!(
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_hash_8b} ll",
        "call {bpf_map_lookup_elem}",
        "exit",
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("uninitialized stack2")
// __success __log_level(4)
// __msg("subprog 0 (uninitialized_stack2) main insns_self {{[0-9]+}} insns_total {{[0-9]+}} stack 8")
// __failure_unpriv __msg_unpriv("invalid read from stack")
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn uninitialized_stack2() {
    asm!(
        "r2 = r10",
        "r0 = *(u64*)(r2 - 8)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("invalid fp arithmetic")
// __failure __msg("R1 subtraction from stack pointer")
// __failure_unpriv
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn invalid_fp_arithmetic() {
    /* If this gets ever changed, make sure JITs can deal with it. */
    asm!(
        "r0 = 0",
        "r1 = r10",
        "r1 -= 8",
        "*(u64*)(r1 + 0) = r0",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("non-invalid fp arithmetic")
// __success __success_unpriv __retval(0)
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn non_invalid_fp_arithmetic() {
    asm!(
        "r0 = 0",
        "*(u64*)(r10 - 8) = r0",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("misaligned read from stack")
// __failure __msg("misaligned stack access")
// __failure_unpriv
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn misaligned_read_from_stack() {
    asm!(
        "r2 = r10",
        "r0 = *(u64*)(r2 - 4)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("stack pointer arithmetic preserves frame number")
// __failure __msg("R7 invalid mem access 'scalar'")
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn stack_ptr_arith_preserves_frameno() {
    asm!(
        "r3 = 0",
        "*(u64 *)(r10 - 8) = r3",
        "r1 = {map_hash_8b} ll",
        "r2 = r10",
        "r2 += -8",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto +2",
        "r0 = 0",
        "exit",
        "r1 = r0",
        "r2 = 0",
        "r3 = 0",
        "call {stack_ptr_arith_preserves_frameno_subprog}",
        "r0 = 0",
        "exit",
        map_hash_8b = sym map_hash_8b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        stack_ptr_arith_preserves_frameno_subprog = sym stack_ptr_arith_preserves_frameno_subprog,
        options(noreturn)
    );
}

#[used]
#[no_mangle]
pub unsafe extern "C" fn stack_ptr_arith_preserves_frameno_subprog() {
    asm!(
        "*(u64 *)(r10 - 8) = r1",
        "r6 = -8",
        "r6 += r10",
        "*(u64 *)(r6 + 0) = r2",
        "r7 = *(u64 *)(r10 - 8)",
        "*(u64 *)(r7 + 0) = r3",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
