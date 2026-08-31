// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/unpriv.c */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

type __u32 = u32;

// Dependencies supplied by the BPF selftest harness in the original C source:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, ../../../include/linux/filter.h, bpf_misc.h.
unsafe extern "C" {
    #[link_name = "bpf_prog_active"]
    static bpf_prog_active: i32;

    fn bpf_loop(nr_loops: __u32, callback_fn: unsafe extern "C" fn(__u32, *mut c_void) -> i32, callback_ctx: *mut c_void, flags: __u32) -> i32;
}

extern "C" {
    fn bpf_tail_call();
    fn bpf_trace_printk();
    fn bpf_map_update_elem();
    fn bpf_map_lookup_elem();
    fn bpf_get_hash_recalc();
    fn bpf_sk_lookup_tcp();
    fn bpf_sk_release();
    fn bpf_skb_load_bytes_relative();
}

const POINTER_VALUE: i32 = 0;

const __sk_buff_mark: i32 = 0; /* offsetof(struct __sk_buff, mark) */
const bpf_sock_mark: i32 = 0; /* offsetof(struct bpf_sock, mark) */
const sizeof_bpf_sock_tuple: i32 = 0; /* sizeof(struct bpf_sock_tuple) */
const st_mem: u64 = 0; /* BPF_ST_MEM(BPF_W, BPF_REG_1, offsetof(struct __sk_buff, mark), 42) */

#[repr(C)]
pub struct map_hash_8b_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut map_hash_8b: map_hash_8b_def = map_hash_8b_def { _private: [] };

#[repr(C)]
pub struct map_prog1_socket_def {
    pub values: [*mut c_void; 3],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut map_prog1_socket: map_prog1_socket_def = map_prog1_socket_def {
    values: [
        dummy_prog_42_socket as *mut c_void,
        dummy_prog_loop1_socket as *mut c_void,
        dummy_prog_24_socket as *mut c_void,
    ],
};

macro_rules! BPF_SK_LOOKUP {
    ($func:literal) => {
        concat!(
            "r2 = 0;",
            "*(u32*)(r10 - 8) = r2;",
            "*(u64*)(r10 - 16) = r2;",
            "*(u64*)(r10 - 24) = r2;",
            "*(u64*)(r10 - 32) = r2;",
            "*(u64*)(r10 - 40) = r2;",
            "*(u64*)(r10 - 48) = r2;",
            "r2 = r10;",
            "r2 += -48;",
            "r3 = {sizeof_bpf_sock_tuple};",
            "r4 = 0;",
            "r5 = 0;",
            "call {", $func, "};",
        )
    };
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_42_socket() {
    asm!("r0 = 42; exit;", options(noreturn));
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_24_socket() {
    asm!("r0 = 24; exit;", options(noreturn));
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_loop1_socket() {
    asm!(
        "r3 = 1;",
        "r2 = {map_prog1_socket} ll;",
        "call {bpf_tail_call};",
        "r0 = 41;",
        "exit;",
        bpf_tail_call = sym bpf_tail_call,
        map_prog1_socket = sym map_prog1_socket,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn pseudo_btf_id_log_masks_address() {
    // __description("unpriv: pseudo btf id log masks address")
    // __success_unpriv
    // __msg_unpriv("0: (18) r1 = 0x0")
    // __not_msg_unpriv("0: (18) r1 = 0x{{[1-9a-f][0-9a-f]*}}")
    // __retval_unpriv(0)
    // __log_level(2)
    asm!(
        "r1 = {bpf_prog_active} ll;",
        "r0 = 0;",
        "exit;",
        bpf_prog_active = sym bpf_prog_active,
        options(noreturn)
    );
}

unsafe extern "C" fn pseudo_func_callback(_index: __u32, _ctx: *mut c_void) -> i32 {
    return 0;
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn unpriv_pseudo_func_policy(ctx: *mut c_void) -> i32 {
    // __description("unpriv: pseudo function policy diagnostic")
    // __success __failure_unpriv
    // __msg_unpriv("loading/calling other bpf or kernel functions")
    // __not_msg_unpriv("BPF-to-BPF function call")
    // __msg_unpriv("policy check failed for BPF function reference")
    // __msg_unpriv("avoid BPF function references in unprivileged")
    bpf_loop(1, pseudo_func_callback, core::ptr::null_mut(), 0);
    return 0;
}

macro_rules! naked_asm_fn {
    ($(#[$meta:meta])* $name:ident, $asm_text:expr $(, $($operand:tt)*)?) => {
        $(#[$meta])*
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            asm!($asm_text $(, $($operand)*)?, options(noreturn));
        }
    };
}

// __description("unpriv: return pointer")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(POINTER_VALUE)
naked_asm_fn!(#[link_section = "socket"] unpriv_return_pointer, "r0 = r10; exit;");

// __description("unpriv: add const to pointer")
// __success __success_unpriv __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_add_const_to_pointer, "r1 += 8; r0 = 0; exit;");

// __description("unpriv: add pointer to pointer")
// __failure __msg("R1 pointer += pointer")
// __failure_unpriv
naked_asm_fn!(#[link_section = "socket"] unpriv_add_pointer_to_pointer, "r1 += r10; r0 = 0; exit;");

// __description("unpriv: neg pointer")
// __success __failure_unpriv __msg_unpriv("R1 pointer arithmetic")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_neg_pointer, "r1 = -r1; r0 = 0; exit;");

// __description("unpriv: cmp pointer with const")
// __success __failure_unpriv __msg_unpriv("R1 pointer comparison")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_cmp_pointer_with_const, "if r1 == 0 goto l0_%=; l0_%=: r0 = 0; exit;");

// __description("unpriv: cmp pointer with pointer")
// __success __failure_unpriv __msg_unpriv("R10 pointer comparison")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_cmp_pointer_with_pointer, "if r1 == r10 goto l0_%=; l0_%=: r0 = 0; exit;");

// __description("unpriv: check that printk is disallowed")
// __success
naked_asm_fn!(#[link_section = "tracepoint"] check_that_printk_is_disallowed,
    "r1 = 0; *(u64*)(r10 - 8) = r1; r1 = r10; r1 += -8; r2 = 8; r3 = r1; call {bpf_trace_printk}; r0 = 0; exit;",
    bpf_trace_printk = sym bpf_trace_printk
);

// __description("unpriv: pass pointer to helper function")
// __success __failure_unpriv __msg_unpriv("R4 leaks addr")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] pass_pointer_to_helper_function,
    "r1 = 0; *(u64*)(r10 - 8) = r1; r2 = r10; r2 += -8; r1 = {map_hash_8b} ll; r3 = r2; r4 = r2; call {bpf_map_update_elem}; r0 = 0; exit;",
    bpf_map_update_elem = sym bpf_map_update_elem,
    map_hash_8b = sym map_hash_8b
);

// __description("unpriv: indirectly pass pointer on stack to helper function")
// __success __failure_unpriv
// __msg_unpriv("invalid read from stack R2 off -8+0 size 8")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] on_stack_to_helper_function,
    "*(u64*)(r10 - 8) = r10; r2 = r10; r2 += -8; r1 = {map_hash_8b} ll; call {bpf_map_lookup_elem}; r0 = 0; exit;",
    bpf_map_lookup_elem = sym bpf_map_lookup_elem,
    map_hash_8b = sym map_hash_8b
);

// __description("unpriv: mangle pointer on stack 1")
// __success __failure_unpriv __msg_unpriv("attempt to corrupt spilled")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] mangle_pointer_on_stack_1, "*(u64*)(r10 - 8) = r10; r0 = 0; *(u32*)(r10 - 8) = r0; r0 = 0; exit;");

// __description("unpriv: mangle pointer on stack 2")
// __success __failure_unpriv __msg_unpriv("attempt to corrupt spilled")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] mangle_pointer_on_stack_2, "*(u64*)(r10 - 8) = r10; r0 = 0; *(u8*)(r10 - 1) = r0; r0 = 0; exit;");

// __description("unpriv: read pointer from stack in small chunks")
// __failure __msg("invalid size")
// __failure_unpriv
naked_asm_fn!(#[link_section = "socket"] from_stack_in_small_chunks, "*(u64*)(r10 - 8) = r10; r0 = *(u32*)(r10 - 8); r0 = 0; exit;");

// __description("unpriv: write pointer into ctx")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv __msg_unpriv("R1 leaks addr")
naked_asm_fn!(#[link_section = "socket"] unpriv_write_pointer_into_ctx, "*(u64*)(r1 + 0) = r1; r0 = 0; exit;");

// __description("unpriv: spill/fill of ctx")
// __success __success_unpriv __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_spill_fill_of_ctx, "r6 = r10; r6 += -8; *(u64*)(r6 + 0) = r1; r1 = *(u64*)(r6 + 0); r0 = 0; exit;");

// __description("unpriv: spill/fill of ctx 2")
// __success __retval(0)
naked_asm_fn!(#[link_section = "tc"] spill_fill_of_ctx_2,
    "r6 = r10; r6 += -8; *(u64*)(r6 + 0) = r1; r1 = *(u64*)(r6 + 0); call {bpf_get_hash_recalc}; r0 = 0; exit;",
    bpf_get_hash_recalc = sym bpf_get_hash_recalc
);

// __description("unpriv: spill/fill of ctx 3")
// __failure __msg("R1 type=fp expected=ctx")
naked_asm_fn!(#[link_section = "tc"] spill_fill_of_ctx_3,
    "r6 = r10; r6 += -8; *(u64*)(r6 + 0) = r1; *(u64*)(r6 + 0) = r10; r1 = *(u64*)(r6 + 0); call {bpf_get_hash_recalc}; exit;",
    bpf_get_hash_recalc = sym bpf_get_hash_recalc
);

// __description("unpriv: spill/fill of ctx 4")
// __failure __msg("R1 type=scalar expected=ctx")
naked_asm_fn!(#[link_section = "tc"] spill_fill_of_ctx_4,
    "r6 = r10; r6 += -8; *(u64*)(r6 + 0) = r1; r0 = 1; lock *(u64 *)(r10 - 8) += r0; r1 = *(u64*)(r6 + 0); call {bpf_get_hash_recalc}; exit;",
    bpf_get_hash_recalc = sym bpf_get_hash_recalc
);

// __description("unpriv: spill/fill of different pointers stx")
// __failure __msg("same insn cannot be used with different pointers")
naked_asm_fn!(#[link_section = "tc"] fill_of_different_pointers_stx,
    "r3 = 42; r6 = r10; r6 += -8; if r1 == 0 goto l0_%=; r2 = r10; r2 += -16; *(u64*)(r6 + 0) = r2; l0_%=: if r1 != 0 goto l1_%=; *(u64*)(r6 + 0) = r1; l1_%=: r1 = *(u64*)(r6 + 0); *(u32*)(r1 + {__sk_buff_mark}) = r3; r0 = 0; exit;",
    __sk_buff_mark = const __sk_buff_mark
);

/* Same as above, but use BPF_ST_MEM to save 42
 * instead of BPF_STX_MEM.
 */
// __description("unpriv: spill/fill of different pointers st")
// __failure __msg("same insn cannot be used with different pointers")
naked_asm_fn!(#[link_section = "tc"] fill_of_different_pointers_st,
    "r6 = r10; r6 += -8; if r1 == 0 goto l0_%=; r2 = r10; r2 += -16; *(u64*)(r6 + 0) = r2; l0_%=: if r1 != 0 goto l1_%=; *(u64*)(r6 + 0) = r1; l1_%=: r1 = *(u64*)(r6 + 0); .8byte {st_mem}; r0 = 0; exit;",
    st_mem = const st_mem
);

// __description("unpriv: spill/fill of different pointers stx - ctx and sock")
// __failure __msg("type=ctx expected=sock")
naked_asm_fn!(#[link_section = "tc"] pointers_stx_ctx_and_sock,
    concat!(
        "r8 = r1;",
        BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
        "r2 = r0; r6 = r10; r6 += -8; r1 = r8; if r1 == 0 goto l0_%=; *(u64*)(r6 + 0) = r2; l0_%=: if r1 != 0 goto l1_%=; *(u64*)(r6 + 0) = r1; l1_%=: r1 = *(u64*)(r6 + 0); r3 = 42; *(u32*)(r1 + {__sk_buff_mark}) = r3; if r1 == 0 goto l2_%=; call {bpf_sk_release}; l2_%=: r0 = 0; exit;"
    ),
    bpf_sk_lookup_tcp = sym bpf_sk_lookup_tcp,
    bpf_sk_release = sym bpf_sk_release,
    __sk_buff_mark = const __sk_buff_mark,
    sizeof_bpf_sock_tuple = const sizeof_bpf_sock_tuple
);

// __description("unpriv: spill/fill of different pointers stx - leak sock")
// __failure
// .errstr = "same insn cannot be used with different pointers",
// __msg("Unreleased reference")
naked_asm_fn!(#[link_section = "tc"] different_pointers_stx_leak_sock,
    concat!(
        "r8 = r1;",
        BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
        "r2 = r0; r6 = r10; r6 += -8; r1 = r8; if r1 == 0 goto l0_%=; *(u64*)(r6 + 0) = r2; l0_%=: if r1 != 0 goto l1_%=; *(u64*)(r6 + 0) = r1; l1_%=: r1 = *(u64*)(r6 + 0); r3 = 42; *(u32*)(r1 + {__sk_buff_mark}) = r3; exit;"
    ),
    bpf_sk_lookup_tcp = sym bpf_sk_lookup_tcp,
    __sk_buff_mark = const __sk_buff_mark,
    sizeof_bpf_sock_tuple = const sizeof_bpf_sock_tuple
);

// __description("unpriv: spill/fill of different pointers stx - sock and ctx (read)")
// __failure __msg("same insn cannot be used with different pointers")
naked_asm_fn!(#[link_section = "tc"] stx_sock_and_ctx_read,
    concat!(
        "r8 = r1;",
        BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
        "r2 = r0; r6 = r10; r6 += -8; r1 = r8; if r1 == 0 goto l0_%=; *(u64*)(r6 + 0) = r1; l0_%=: if r1 != 0 goto l1_%=; *(u64*)(r6 + 0) = r2; l1_%=: r1 = *(u64*)(r6 + 0); if r1 == 0 goto l2_%=; r3 = *(u32*)(r1 + {bpf_sock_mark}); call {bpf_sk_release}; l2_%=: r0 = 0; exit;"
    ),
    bpf_sk_lookup_tcp = sym bpf_sk_lookup_tcp,
    bpf_sk_release = sym bpf_sk_release,
    bpf_sock_mark = const bpf_sock_mark,
    sizeof_bpf_sock_tuple = const sizeof_bpf_sock_tuple
);

// __description("unpriv: spill/fill of different pointers stx - sock and ctx (write)")
// __failure
// .errstr = "same insn cannot be used with different pointers",
// __msg("cannot write into sock")
naked_asm_fn!(#[link_section = "tc"] stx_sock_and_ctx_write,
    concat!(
        "r8 = r1;",
        BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
        "r2 = r0; r6 = r10; r6 += -8; r1 = r8; if r1 == 0 goto l0_%=; *(u64*)(r6 + 0) = r1; l0_%=: if r1 != 0 goto l1_%=; *(u64*)(r6 + 0) = r2; l1_%=: r1 = *(u64*)(r6 + 0); if r1 == 0 goto l2_%=; r3 = 42; *(u32*)(r1 + {bpf_sock_mark}) = r3; call {bpf_sk_release}; l2_%=: r0 = 0; exit;"
    ),
    bpf_sk_lookup_tcp = sym bpf_sk_lookup_tcp,
    bpf_sk_release = sym bpf_sk_release,
    bpf_sock_mark = const bpf_sock_mark,
    sizeof_bpf_sock_tuple = const sizeof_bpf_sock_tuple
);

// __description("unpriv: write pointer into map elem value")
// __success __failure_unpriv __msg_unpriv("R0 leaks addr")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] pointer_into_map_elem_value,
    "r1 = 0; *(u64*)(r10 - 8) = r1; r2 = r10; r2 += -8; r1 = {map_hash_8b} ll; call {bpf_map_lookup_elem}; if r0 == 0 goto l0_%=; *(u64*)(r0 + 0) = r0; l0_%=: exit;",
    bpf_map_lookup_elem = sym bpf_map_lookup_elem,
    map_hash_8b = sym map_hash_8b
);

// __description("alu32: mov u32 const")
// __success __success_unpriv
// __retval(0)
// #ifdef SPEC_V1
// __xlated_unpriv("if r0 == 0x0 goto pc+2")
// __xlated_unpriv("nospec") /* inserted to prevent `R7 invalid mem access 'scalar'` */
// __xlated_unpriv("goto pc-1") /* sanitized dead code */
// __xlated_unpriv("exit")
// #endif
naked_asm_fn!(#[link_section = "socket"] alu32_mov_u32_const, "w7 = 0; w7 ^= w7; w0 = w7; if r0 == 0 goto l0_%=; r0 = *(u64*)(r7 + 0); l0_%=: exit;");

// __description("unpriv: partial copy of pointer")
// __success __failure_unpriv __msg_unpriv("R10 partial copy")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_partial_copy_of_pointer, "w1 = w10; r0 = 0; exit;");

// __description("unpriv: pass pointer to tail_call")
// __success __failure_unpriv __msg_unpriv("R3 leaks addr into helper")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] pass_pointer_to_tail_call,
    "r3 = r1; r2 = {map_prog1_socket} ll; call {bpf_tail_call}; r0 = 0; exit;",
    bpf_tail_call = sym bpf_tail_call,
    map_prog1_socket = sym map_prog1_socket
);

// __description("unpriv: cmp map pointer with zero")
// __success __success_unpriv
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] cmp_map_pointer_with_zero,
    "r1 = {map_hash_8b} ll; if r1 == 0 goto l0_%=; l0_%=: r0 = 0; exit;",
    map_hash_8b = sym map_hash_8b
);

// __description("unpriv: cmp map pointer with const")
// __success __failure_unpriv __msg_unpriv("R1 pointer comparison prohibited")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] cmp_map_pointer_with_const,
    "r1 = {map_hash_8b} ll; if r1 == 0x0000beef goto l0_%=; l0_%=: r0 = 0; exit;",
    map_hash_8b = sym map_hash_8b
);

// __description("unpriv: write into frame pointer")
// __failure __msg("frame pointer is read only")
// __failure_unpriv
naked_asm_fn!(#[link_section = "socket"] unpriv_write_into_frame_pointer, "r10 = r1; r0 = 0; exit;");

// __description("unpriv: spill/fill frame pointer")
// __failure __msg("frame pointer is read only")
// __failure_unpriv
naked_asm_fn!(#[link_section = "socket"] unpriv_spill_fill_frame_pointer, "r6 = r10; r6 += -8; *(u64*)(r6 + 0) = r10; r10 = *(u64*)(r6 + 0); r0 = 0; exit;");

// __description("unpriv: cmp of frame pointer")
// __success __failure_unpriv __msg_unpriv("R10 pointer comparison")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_cmp_of_frame_pointer, "if r10 == 0 goto l0_%=; l0_%=: r0 = 0; exit;");

// __description("unpriv: adding of fp, reg")
// __success __failure_unpriv
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_adding_of_fp_reg, "r0 = 0; r1 = 0; r1 += r10; *(u64*)(r1 - 8) = r0; exit;");

// __description("unpriv: adding of fp, imm")
// __success __failure_unpriv
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_adding_of_fp_imm, "r0 = 0; r1 = r10; r1 += 0; *(u64*)(r1 - 8) = r0; exit;");

// __description("unpriv: cmp of stack pointer")
// __success __failure_unpriv __msg_unpriv("R2 pointer comparison")
// __retval(0)
naked_asm_fn!(#[link_section = "socket"] unpriv_cmp_of_stack_pointer, "r2 = r10; r2 += -8; if r2 == 0 goto l0_%=; l0_%=: r0 = 0; exit;");

// __description("unpriv: Spectre v1 path-based type confusion of scalar as stack-ptr")
// __success __success_unpriv __retval(0)
// #ifdef SPEC_V1
// __xlated_unpriv("if r0 != 0x1 goto pc+2")
// __xlated_unpriv("nospec")
// __xlated_unpriv("r9 = *(u8 *)(r6 +0)")
// #endif
naked_asm_fn!(#[link_section = "socket"] unpriv_spec_v1_type_confusion,
    "r1 = 0; *(u64*)(r10 - 8) = r1; r2 = r10; r2 += -8; r1 = {map_hash_8b} ll; call {bpf_map_lookup_elem}; if r0 == 0 goto l2_%=; r2 = r10; r2 += -8; r1 = {map_hash_8b} ll; r6 = r10; r6 += -8; r9 = 0xffffc900; r9 <<= 32; r0 = *(u64 *)(r0 + 0); if r0 != 0x0 goto l0_%=; r6 = r9; l0_%=: if r0 != 0x1 goto l1_%=; r9 = *(u8 *)(r6 + 0); l1_%=: r9 &= 1; r9 <<= 9; *(u64*)(r10 - 8) = r9; call {bpf_map_lookup_elem}; if r0 == 0 goto l2_%=; r0 = *(u64 *)(r0 + 0); l2_%=: r0 = 0; exit;",
    bpf_map_lookup_elem = sym bpf_map_lookup_elem,
    map_hash_8b = sym map_hash_8b
);

// __description("unpriv: ldimm64 before Spectre v4 barrier")
// __success __success_unpriv
// __retval(0)
// #ifdef SPEC_V4
// __xlated_unpriv("r1 = 0x2020200005642020")
// __xlated_unpriv("*(u64 *)(r10 -8) = r1")
// __xlated_unpriv("nospec")
// #endif
naked_asm_fn!(#[link_section = "socket"] unpriv_ldimm64_spectre_v4, "r1 = 0x2020200005642020 ll; *(u64 *)(r10 -8) = r1; r0 = 0; exit;");

// __description("unpriv: Spectre v1 and v4 barrier")
// __success __success_unpriv
// __retval(0)
// #ifdef SPEC_V1 && SPEC_V4: translated __xlated_unpriv expectations preserved as comments in source.
naked_asm_fn!(#[link_section = "socket"] unpriv_spectre_v1_and_v4,
    "r1 = 0; *(u64*)(r10 - 8) = r1; r2 = r10; r2 += -8; r1 = {map_hash_8b} ll; call {bpf_map_lookup_elem}; r8 = r0; r2 = r10; r2 += -8; r1 = {map_hash_8b} ll; call {bpf_map_lookup_elem}; r9 = r0; r0 = r10; r1 = 0; r2 = r10; if r8 != 0 goto l0_%=; if r9 != 0 goto l0_%=; r0 = 0; l0_%=: if r8 != 0 goto l1_%=; goto l2_%=; l1_%=: if r9 == 0 goto l3_%=; r2 = r0; l2_%=: *(u64 *)(r2 -64) = r1; l3_%=: r0 = 0; exit;",
    bpf_map_lookup_elem = sym bpf_map_lookup_elem,
    map_hash_8b = sym map_hash_8b
);

// __description("unpriv: Spectre v1 and v4 barrier (simple)")
// __success __success_unpriv
// __retval(0)
// #ifdef SPEC_V1 && SPEC_V4: translated __xlated_unpriv expectations preserved as comments in source.
naked_asm_fn!(#[link_section = "socket"] unpriv_spectre_v1_and_v4_simple,
    "r8 = 0; r8 ^= r8; r9 = 0; r9 ^= r9; r0 = r10; r1 = 0; r2 = r10; if r8 != 0 goto l0_%=; if r9 != 0 goto l0_%=; r0 = 0; l0_%=: if r8 != 0 goto l1_%=; goto l2_%=; l1_%=: if r9 == 0 goto l3_%=; r2 = r0; l2_%=: *(u64 *)(r2 -64) = r1; l3_%=: r0 = 0; exit;"
);

// __description("unpriv: ldimm64 before Spectre v1 and v4 barrier (simple)")
// __success __success_unpriv
// __retval(0)
// #ifdef SPEC_V1 && SPEC_V4: translated __xlated_unpriv expectations preserved as comments in source.
naked_asm_fn!(#[link_section = "socket"] unpriv_ldimm64_spectre_v1_and_v4_simple,
    "r8 = 0; r8 ^= r8; r9 = 0; r9 ^= r9; r0 = r10; r1 = 0; r2 = r10; if r8 != 0 goto l0_%=; if r9 != 0 goto l0_%=; r0 = 0; l0_%=: if r8 != 0 goto l1_%=; goto l2_%=; l1_%=: if r9 == 0 goto l3_%=; r2 = r0; r1 = 0x2020200005642020 ll; l2_%=: *(u64 *)(r2 -64) = r1; l3_%=: r0 = 0; exit;"
);

// __description("unpriv: nospec after dead stack write in helper")
// __success __success_unpriv
// __retval(0)
/* Dead code sanitizer rewrites the call to `goto -1`. */
naked_asm_fn!(#[link_section = "socket"] unpriv_dead_helper_stack_write_nospec_result,
    "r0 = 0; if r0 != 1 goto l0_%=; r2 = 0; r3 = r10; r3 += -16; r4 = 4; r5 = 0; call {bpf_skb_load_bytes_relative}; l0_%=: exit;",
    bpf_skb_load_bytes_relative = sym bpf_skb_load_bytes_relative
);

// __description("unpriv: Spectre v4 stack write slot index")
// __success __success_unpriv
// __retval(0)
// #ifdef SPEC_V4
// __xlated_unpriv("r0 = 0")
// __xlated_unpriv("*(u32 *)(r10 -4) = r0")
// __xlated_unpriv("nospec")
// __xlated_unpriv("*(u32 *)(r10 -8) = r0")
// __xlated_unpriv("nospec")
// __xlated_unpriv("exit")
// #endif
naked_asm_fn!(#[link_section = "socket"] stack_write_nospec_slot_index, "r0 = 0; *(u32 *)(r10 - 4) = r0; *(u32 *)(r10 - 8) = r0; exit;");

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
