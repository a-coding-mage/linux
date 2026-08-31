// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// Rust translation of testing/selftests/bpf/progs/verifier_live_stack.c.
// C include dependencies removed from executable Rust: linux/bpf.h, bpf/bpf_helpers.h, filter.h, bpf_misc.h.

pub type __u32 = u32;
pub type __u64 = u64;

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> __u32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_loop(nr_loops: __u32, callback_fn: *mut core::ffi::c_void, callback_ctx: *mut core::ffi::c_void, flags: __u64) -> i64;
    fn bpf_tail_call(ctx: *mut core::ffi::c_void, prog_array_map: *mut core::ffi::c_void, index: __u32);
    fn bpf_snprintf(str_: *mut i8, str_size: __u32, fmt: *const i8, data: *const core::ffi::c_void, data_len: __u32) -> i64;
    fn bpf_iter_num_new(iter: *mut core::ffi::c_void, start: i32, end: i32) -> i32;
    fn bpf_iter_num_next(iter: *mut core::ffi::c_void) -> *mut i32;
    fn bpf_iter_num_destroy(iter: *mut core::ffi::c_void);
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct bpf_map_def_placeholder {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

// Original map declaration used __uint/__type BPF macros: HASH, key int, value long long.
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map: bpf_map_def_placeholder = bpf_map_def_placeholder { type_: 1, max_entries: 1, key_size: core::mem::size_of::<i32>() as u32, value_size: core::mem::size_of::<i64>() as u32 };

// Original map declaration used __uint/__type BPF macros: ARRAY, key __u32, value __u64.
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array_map_8b: bpf_map_def_placeholder = bpf_map_def_placeholder { type_: 2, max_entries: 1, key_size: core::mem::size_of::<__u32>() as u32, value_size: core::mem::size_of::<__u64>() as u32 };

#[unsafe(no_mangle)]
pub static snprintf_u64_fmt: [u8; 5] = *b"%llu\0";

// Original map declaration used __uint/__type BPF macros: PROG_ARRAY, key __u32, value __u32.
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_array: bpf_map_def_placeholder = bpf_map_def_placeholder { type_: 3, max_entries: 1, key_size: core::mem::size_of::<__u32>() as u32, value_size: core::mem::size_of::<__u32>() as u32 };

// SEC("socket")
// __log_level(2)
// __msg("0: (79) r1 = *(u64 *)(r10 -8)        ; use: fp0-8")
// __msg("1: (79) r2 = *(u64 *)(r10 -24)       ; use: fp0-24")
// __msg("2: (7b) *(u64 *)(r10 -8) = r1        ; def: fp0-8")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_read_simple_write() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __msg("2: (79) r0 = *(u64 *)(r10 -8)        ; use: fp0-8")
// __msg("6: (79) r0 = *(u64 *)(r10 -16)       ; use: fp0-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_write_join() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 > 42 goto 1f;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __msg("stack use/def subprog#0 must_write_not_same_slot (d0,cs0):")
// __msg("6: (7b) *(u64 *)(r2 +0) = r0{{$}}")
// __msg("Live regs before insn:")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn must_write_not_same_slot() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "r1 = -8;"
    // asm: "if r0 > 42 goto 1f;"
    // asm: "r1 = -16;"
    // asm: "1:"
    // asm: "r2 = r10;"
    // asm: "r2 += r1;"
    // asm: "*(u64 *)(r2 + 0) = r0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __msg("0: (7a) *(u64 *)(r10 -8) = 0         ; def: fp0-8")
// __msg("5: (85) call bpf_map_lookup_elem#1   ; use: fp0-8h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn must_write_not_same_type() {
    // asm: "*(u64*)(r10 - 8) = 0;"
    // asm: "r2 = r10;"
    // asm: "r2 += -8;"
    // asm: "r1 = %[map] ll;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 != 0 goto 1f;"
    // asm: "r0 = r10;"
    // asm: "r0 += -16;"
    // asm: "1:"
    // asm: "*(u64 *)(r0 + 0) = 42;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
/* Callee writes fp[0]-8: stack_use at call site has slots 0,1 live */
// __msg("stack use/def subprog#0 caller_stack_write (d0,cs0):")
// __msg("2: (85) call pc+1{{$}}")
// __msg("stack use/def subprog#1 write_first_param (d1,cs2):")
// __msg("4: (7a) *(u64 *)(r1 +0) = 7          ; def: fp0-8")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn caller_stack_write() {
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call write_first_param;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn write_first_param() {
    // asm: "*(u64 *)(r1 + 0) = 7;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __msg("stack use/def subprog#0 caller_stack_read (d0,cs0):")
// __msg("2: (85) call pc+{{.*}}                   ; use: fp0-8{{$}}")
// __msg("5: (85) call pc+{{.*}}                   ; use: fp0-16{{$}}")
// __msg("stack use/def subprog#1 read_first_param (d1,cs2):")
// __msg("7: (79) r0 = *(u64 *)(r1 +0)         ; use: fp0-8{{$}}")
// __msg("8: (95) exit")
// __msg("stack use/def subprog#1 read_first_param (d1,cs5):")
// __msg("7: (79) r0 = *(u64 *)(r1 +0)         ; use: fp0-16{{$}}")
// __msg("8: (95) exit")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn caller_stack_read() {
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call read_first_param;"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "call read_first_param;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn read_first_param() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn arg_track_join_convergence() {
    // asm: "r1 = 1;"
    // asm: "r2 = 2;"
    // asm: "call arg_track_join_convergence_subprog;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn arg_track_join_convergence_subprog() {
    // asm: "if r1 == 0 goto 1f;"
    // asm: "r0 = r1;"
    // asm: "goto 2f;"
    // asm: "1:"
    // asm: "r0 = r2;"
    // asm: "2:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __log_level(2)
/* fp0-8 consumed at insn 9, dead by insn 11. stack_def at insn 4 kills slots 0,1. */
// __msg("4: (7b) *(u64 *)(r10 -8) = r0        ; def: fp0-8")
/* stack_use at call site: callee reads fp0-8, slots 0,1 live */
// __msg("7: (85) call pc+{{.*}}               ; use: fp0-8")
/* read_first_param2: no caller stack live inside callee after first read */
// __msg("9: (79) r0 = *(u64 *)(r1 +0)         ; use: fp0-8")
// __msg("10: (b7) r0 = 0{{$}}")
// __msg("11: (05) goto pc+0{{$}}")
// __msg("12: (95) exit")
/*
 * Checkpoint at goto +0 fires because fp0-8 is dead → state pruning.
 */
// __msg("12: safe")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn caller_stack_pruning() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 42 goto 1f;"
    // asm: "r0 = %[map] ll;"
    // asm: "1:"
    // asm: "*(u64 *)(r10 - 8) = r0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call read_first_param2;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn read_first_param2() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure
// __msg("R1 type=scalar expected=map_ptr")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn caller_stack_pruning_callback() {
    // asm: "r0 = %[map] ll;"
    // asm: "*(u64 *)(r10 - 8) = r0;"
    // asm: "r1 = 2;"
    // asm: "r2 = loop_cb ll;"
    // asm: "r3 = r10;"
    // asm: "r3 += -8;"
    // asm: "r4 = 0;"
    // asm: "call %[bpf_loop];"
    // asm: "r0 = 42;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn loop_cb() {
    // asm: "r6 = r2;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Because of a bug in verifier.c:compute_postorder()
 * the program below overflowed traversal queue in that function.
 */
// SEC("socket")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syzbot_postorder_bug1() {
    // asm: "r0 = 0;"
    // asm: "if r0 != 0 goto -1;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __failure __msg("invalid read from stack R2 off=-1024 size=8")
// __flag(BPF_F_TEST_STATE_FREQ)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn caller_stack_write_tail_call() -> u64 {
    // asm: "r6 = r1;"
    // asm: "*(u64 *)(r10 - 8) = -8;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 42 goto 1f;"
    // asm: "goto 2f;"
    // asm: "1:"
    // asm: "*(u64 *)(r10 - 8) = -1024;"
    // asm: "2:"
    // asm: "r1 = r6;"
    // asm: "r2 = r10;"
    // asm: "r2 += -8;"
    // asm: "call write_tail_call;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
    loop {}
}


unsafe extern "C" fn write_tail_call() -> u64 {
    // asm: "r6 = r2;"
    // asm: "r2 = %[map_array] ll;"
    // asm: "r3 = 0;"
    // asm: "call %[bpf_tail_call];"
    // asm: "*(u64 *)(r6 + 0) = -16;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
    loop {}
}


/* Test precise subprog stack access analysis.
 * Caller passes fp-32 (SPI 3) to callee that only accesses arg+0 and arg+8
 * (SPIs 3 and 2). Slots 0 and 1 should NOT be live at the call site.
 *
 * Insn layout:
 *   0: *(u64*)(r10 - 8) = 0      write SPI 0
 *   1: *(u64*)(r10 - 16) = 0     write SPI 1
 *   2: *(u64*)(r10 - 24) = 0     write SPI 2
 *   3: *(u64*)(r10 - 32) = 0     write SPI 3
 *   4: r1 = r10
 *   5: r1 += -32
 *   6: call precise_read_two      passes fp-32 (SPI 3)
 *   7: r0 = 0
 *   8: exit
 *
 * At insn 6 only SPIs 2,3 should be live (slots 4-7, 0xf0).
 * SPIs 0,1 are written but never read → dead.
 */
// SEC("socket")
// __log_level(2)
// __msg("6: (85) call pc+{{.*}}                   ; use: fp0-24 fp0-32{{$}}")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_precise_stack_access() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u64 *)(r10 - 16) = 0;"
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "call precise_read_two;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Callee reads only at arg+0 (SPI 3) and arg+8 (SPI 2) */
unsafe extern "C" fn precise_read_two() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that multi-level subprog calls (callee passes arg-derived ptr
 * to another BPF subprog) are analyzed precisely.
 *
 * Caller passes fp-32 (SPI 3). The callee forwards it to inner_callee.
 * inner_callee only reads at offset 0 from the pointer.
 * The analysis recurses into forward_to_inner -> inner_callee and
 * determines only SPI 3 is accessed (slots 6-7, 0xc0), not all of SPIs 0-3.
 *
 * Insn layout:
 *   0: *(u64*)(r10 - 8) = 0      write SPI 0
 *   1: *(u64*)(r10 - 16) = 0     write SPI 1
 *   2: *(u64*)(r10 - 24) = 0     write SPI 2
 *   3: *(u64*)(r10 - 32) = 0     write SPI 3
 *   4: r1 = r10
 *   5: r1 += -32
 *   6: call forward_to_inner      passes fp-32 (SPI 3)
 *   7: r0 = 0
 *   8: exit
 */
// SEC("socket")
// __log_level(2)
// __msg("6: (85) call pc+{{.*}}                   ; use: fp0-32{{$}}")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_multilevel_conservative() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u64 *)(r10 - 16) = 0;"
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "call forward_to_inner;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Forwards arg to another subprog */
unsafe extern "C" fn forward_to_inner() {
    // asm: "call inner_callee;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn inner_callee() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test multi-frame precision loss: callee consumes caller stack early,
 * but static liveness keeps it live at pruning points inside callee.
 *
 * Caller stores map_ptr or scalar(42) at fp-8, then calls
 * consume_and_call_inner. The callee reads fp0-8 at entry (consuming
 * the slot), then calls do_nothing2. After do_nothing2 returns (a
 * pruning point), fp-8 should be dead -- the read already happened.
 * But because the call instruction's stack_use includes SPI 0, the
 * static live_stack_before at insn 7 is 0x1, keeping fp-8 live inside
 * the callee and preventing state pruning between the two paths.
 *
 * Insn layout:
 *   0: call bpf_get_prandom_u32
 *   1: if r0 == 42 goto pc+2    -> insn 4
 *   2: r0 = map ll (ldimm64 part1)
 *   3: (ldimm64 part2)
 *   4: *(u64)(r10 - 8) = r0     fp-8 = map_ptr OR scalar(42)
 *   5: r1 = r10
 *   6: r1 += -8
 *   7: call consume_and_call_inner
 *   8: r0 = 0
 *   9: exit
 *
 * At insn 7, live_stack_before = 0x3 (slots 0-1 live due to stack_use).
 * At insn 8, live_stack_before = 0x0 (SPI 0 dead, caller doesn't need it).
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __log_level(2)
// __success
// __msg(" 7: (85) call pc+{{.*}}                   ; use: fp0-8")
// __msg(" 8: {{.*}} (b7)")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callee_consumed_caller_stack() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 42 goto 1f;"
    // asm: "r0 = %[map] ll;"
    // asm: "1:"
    // asm: "*(u64 *)(r10 - 8) = r0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call consume_and_call_inner;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn consume_and_call_inner() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn do_nothing2() {
    // asm: "r0 = 0;"
    // asm: "r0 = 0;"
    // asm: "r0 = 0;"
    // asm: "r0 = 0;"
    // asm: "r0 = 0;"
    // asm: "r0 = 0;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Reproducer for unsound pruning when clean_verifier_state() promotes
 * live STACK_ZERO bytes to STACK_MISC.
 *
 * Program shape:
 * - Build key at fp-4:
 *   - path A keeps key byte as STACK_ZERO;
 *   - path B writes unknown byte making it STACK_MISC.
 * - Branches merge at a prune point before map_lookup.
 * - map_lookup on ARRAY map is value-sensitive to constant zero key:
 *   - path A: const key 0 => PTR_TO_MAP_VALUE (non-NULL);
 *   - path B: non-const key => PTR_TO_MAP_VALUE_OR_NULL.
 * - Dereference lookup result without null check.
 *
 * Note this behavior won't trigger at fp-8, since the verifier will
 * track 32-bit scalar spill differently as spilled_ptr.
 *
 * Correct verifier behavior: reject (path B unsafe).
 * With blanket STACK_ZERO->STACK_MISC promotion on live slots, cached path A
 * state can be generalized and incorrectly prune path B, making program load.
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_zero_to_misc_unsound_array_lookup() {
    // asm: "*(u32 *)(r10 - 4) = 0;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_nonconst%=;"
    // asm: "goto l_lookup%=;"
    // asm: "l_nonconst%=:"
    // asm: "*(u8 *)(r10 - 4) = r0;"
    // asm: "l_lookup%=:"
    // asm: "r2 = r10;"
    // asm: "r2 += -4;"
    // asm: "r1 = %[array_map_8b] ll;"
    // asm: "call %[bpf_map_lookup_elem];"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Subprog variant of stack_zero_to_misc_unsound_array_lookup.
 *
 * Check unsound pruning when a callee modifies the caller's
 * stack through a pointer argument.
 *
 * Program shape:
 *   main:
 *     *(u32)(fp - 4) = 0            key = 0 (all bytes STACK_ZERO)
 *     r1 = fp - 4
 *     call maybe_clobber_key        may overwrite key[0] with scalar
 *     <-- prune point: two states meet here -->
 *     r2 = fp - 4
 *     r1 = array_map_8b
 *     call bpf_map_lookup_elem      value-sensitive on const-zero key
 *     r0 = *(u64)(r0 + 0)           deref without null check
 *     exit
 *
 *   maybe_clobber_key(r1):
 *     r6 = r1                       save &key
 *     call bpf_get_prandom_u32
 *     if r0 == 0 goto skip          path A: key stays STACK_ZERO
 *     *(u8)(r6 + 0) = r0            path B: key[0] becomes STACK_MISC
 *   skip:
 *     r0 = 0
 *     exit
 *
 * Path A: const-zero key => array lookup => PTR_TO_MAP_VALUE => deref OK.
 * Path B: non-const key  => array lookup => PTR_TO_MAP_VALUE_OR_NULL => UNSAFE.
 *
 * If the cleaner collapses STACK_ZERO -> STACK_MISC for the live key
 * slot, path A's cached state matches path B, pruning the unsafe path.
 *
 * Correct verifier behaviour: reject.
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subprog_stack_zero_to_misc_unsound() {
    // asm: "*(u32 *)(r10 - 4) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -4;"
    // asm: "call maybe_clobber_key;"
    // asm: "r2 = r10;"
    // asm: "r2 += -4;"
    // asm: "r1 = %[array_map_8b] ll;"
    // asm: "call %[bpf_map_lookup_elem];"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn maybe_clobber_key() {
    // asm: "r6 = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "*(u8 *)(r6 + 0) = r0;"
    // asm: "1:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Demonstrate that subprog arg spill/reload breaks arg tracking,
 * inflating caller stack liveness and preventing state pruning.
 *
 * modifier2(fp-24) has two paths: one writes a scalar to *(r1+8)
 * = caller fp-16, the other leaves it as zero.  After modifier2
 * returns, fp-16 is never read again — it is dead.
 *
 * spill_reload_reader2(fp-24) only reads caller fp-8 via
 * *(r1+16), but it spills r1 across a helper call.  This
 * breaks compute_subprog_arg_access(): the reload from callee
 * stack cannot be connected back to arg1, so arg1 access goes
 * "all (conservative)".  At the call site (r1 = fp-24, slot 5)
 * apply_callee_stack_access() marks slots 0..5 as stack_use —
 * pulling fp-16 (slots 2-3) into live_stack_before even though
 * the reader never touches it.
 *
 * Result: at modifier2's return point two states with different
 * fp-16 values cannot be pruned.
 *
 * With correct (or old dynamic) liveness fp-16 is dead at that
 * point and the states prune → "6: safe" appears in the log.
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __log_level(2)
// __success
// __msg("6: safe")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spill_reload_inflates_stack_liveness() {
    // asm: "*(u64 *)(r10 - 24) = r1;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "call modifier2;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "call spill_reload_reader2;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Two paths: one writes a scalar to *(r1+8) = caller fp-16,
 * the other leaves it unchanged.  Both return 0 via separate
 * exits to prevent pruning inside the subprog at the merge.
 */
unsafe extern "C" fn modifier2() {
    // asm: "r6 = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "*(u64 *)(r6 + 8) = r0;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    // asm: "1:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Receives r1 = caller fp-24.  Only reads *(r1+16) = fp-8.
 * Spills r1 across a helper call → arg tracking goes conservative →
 * slots 0..5 all appear used instead of just slot 1 (fp-8).
 */
unsafe extern "C" fn spill_reload_reader2() {
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* BTF FUNC records are not generated for kfuncs referenced
 * from inline assembly. These records are necessary for
 * libbpf to link the program. The function below is a hack
 * to ensure that BTF FUNC records are generated.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __kfunc_btf_root() {
    unsafe {
    
    	bpf_iter_num_new(core::ptr::null_mut(), 0, 0);
    	bpf_iter_num_next(core::ptr::null_mut());
    	bpf_iter_num_destroy(core::ptr::null_mut());
    }
}


/* Test that open-coded iterator kfunc arguments get precise stack
 * liveness tracking. struct bpf_iter_num is 8 bytes (1 SPI).
 *
 * Insn layout:
 *   0: *(u64*)(r10 - 8) = 0      write SPI 0 (dead)
 *   1: *(u64*)(r10 - 16) = 0     write SPI 1 (dead)
 *   2: r1 = r10
 *   3: r1 += -24                 iter state at fp-24 (SPI 2)
 *   4: r2 = 0
 *   5: r3 = 10
 *   6: call bpf_iter_num_new     defines SPI 2 (KF_ITER_NEW) → 0x0
 *   7-8: r1 = fp-24
 *   9: call bpf_iter_num_next    uses SPI 2 → 0x30
 *  10: if r0 == 0 goto 2f
 *  11: goto 1b
 *  12-13: r1 = fp-24
 *  14: call bpf_iter_num_destroy uses SPI 2 → 0x30
 *  15: r0 = 0
 *  16: exit
 *
 * At insn 6, SPI 2 is defined (KF_ITER_NEW initializes, doesn't read),
 * so it kills liveness from successors. live_stack_before = 0x0.
 * At insns 9 and 14, SPI 2 is used (iter_next/destroy read the state),
 * so live_stack_before = 0x30.
 */
// SEC("socket")
// __success __log_level(2)
// __msg(" 6: (85) call bpf_iter_num_new{{.*}}          ; def: fp0-24{{$}}")
// __msg(" 9: (85) call bpf_iter_num_next{{.*}}         ; use: fp0-24{{$}}")
// __msg("14: (85) call bpf_iter_num_destroy{{.*}}      ; use: fp0-24{{$}}")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_iter_stack_liveness() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u64 *)(r10 - 16) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "r2 = 0;"
    // asm: "r3 = 10;"
    // asm: "call %[bpf_iter_num_new];"
    // asm: "1:"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "call %[bpf_iter_num_next];"
    // asm: "if r0 == 0 goto 2f;"
    // asm: "goto 1b;"
    // asm: "2:"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "call %[bpf_iter_num_destroy];"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Test for soundness bug in static stack liveness analysis.
 *
 * The static pre-pass tracks FP-derived register offsets to determine
 * which stack slots are accessed. When a PTR_TO_STACK is spilled to
 * the stack and later reloaded, the reload (BPF_LDX) kills FP-derived
 * tracking, making subsequent accesses through the reloaded pointer
 * invisible to the static analysis.
 *
 * This causes the analysis to incorrectly mark SPI 0 as dead at the
 * merge point. clean_verifier_state() zeros it in the cached state,
 * and stacksafe() accepts the new state against STACK_INVALID,
 * enabling incorrect pruning.
 *
 * Path A (verified first): stores PTR_TO_MAP_VALUE in SPI 0
 * Path B (verified second): stores scalar 42 in SPI 0
 * After merge: reads SPI 0 through spilled/reloaded PTR_TO_STACK
 * and dereferences the result as a pointer.
 *
 * Correct behavior: reject (path B dereferences a scalar)
 * Bug behavior: accept (path B is incorrectly pruned)
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R0 invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spill_ptr_liveness_type_confusion() {
    // asm: "r1 = %[map] ll;"
    // asm: "*(u32 *)(r10 - 32) = 0;"
    // asm: "r2 = r10;"
    // asm: "r2 += -32;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 == 0 goto l_exit%=;"
    // asm: "r6 = r0;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_scalar%=;"
    // asm: "*(u64 *)(r10 - 8) = r6;"
    // asm: "goto l_merge%=;"
    // asm: "l_scalar%=:"
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "l_merge%=:"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "goto +0;"
    // asm: "goto +0;"
    // asm: "goto +0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* === Tests for 4-byte stack slot liveness granularity === */

/* Test that a 4-byte aligned write is stack_def and kills liveness.
 *
 *   0: *(u64 *)(r10 - 8) = 0      def slots 0,1 (full SPI 0)
 *   1: *(u32 *)(r10 - 8) = 0      def slot 1 (4-byte write kills slot 1)
 *   2: r0 = *(u64 *)(r10 - 8)     use slots 0,1
 *   3: r0 = 0
 *   4: exit
 *
 * At insn 1, the 4-byte write defines slot 1. Slot 0 still flows
 * backward from insn 2's read: live_stack_before = 0x1.
 */
// SEC("socket")
// __log_level(2)
// __msg("1: (62) *(u32 *)(r10 -8) = 0         ; def: fp0-8h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn four_byte_write_kills_slot() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u32 *)(r10 - 8) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that a write to the upper half of an SPI is dead when only
 * the lower half is read. This was impossible at SPI granularity
 * where any read of the SPI kept the entire SPI live.
 *
 *   0: *(u32 *)(r10 - 8) = 0      def slot 1 (DEAD: never read)
 *   1: *(u32 *)(r10 - 4) = 0      def slot 0
 *   2: r0 = *(u32 *)(r10 - 4)     use slot 0 only
 *   3: r0 = 0
 *   4: exit
 *
 * At insn 0, nothing is live (0x0). Previously at SPI granularity,
 * the read at insn 2 would mark the full SPI 0 as live and the
 * 4-byte writes wouldn't count as def, so insn 0 would have had
 * SPI 0 live (0x1).
 */
// SEC("socket")
// __log_level(2)
// __msg("0: (62) *(u32 *)(r10 -8) = 0         ; def: fp0-8h")
// __msg("2: (61) r0 = *(u32 *)(r10 -4)        ; use: fp0-4h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dead_half_spi_write() {
    // asm: "*(u32 *)(r10 - 8) = 0;"
    // asm: "*(u32 *)(r10 - 4) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that a 4-byte read from the upper half of SPI 0 makes only
 * slot 1 live (0x2), not the full SPI (0x3).
 *
 *   0: *(u64 *)(r10 - 8) = 0      def slots 0,1
 *   1: r0 = *(u32 *)(r10 - 8)     use slot 1 only (upper half)
 *   2: r0 = 0
 *   3: exit
 *
 * At insn 1, live_stack_before = 0x2 (slot 1 only).
 */
// SEC("socket")
// __log_level(2)
// __msg("1: (61) r0 = *(u32 *)(r10 -8)        ; use: fp0-8h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn four_byte_read_upper_half() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that a 2-byte write does NOT count as stack_def.
 * Sub-4-byte writes don't fully cover a 4-byte slot,
 * so liveness passes through.
 *
 *   0: *(u64 *)(r10 - 8) = 0      def slots 0,1
 *   1: *(u16 *)(r10 - 4) = 0      NOT stack_def (2 < 4 bytes)
 *   2: r0 = *(u32 *)(r10 - 4)     use slot 0
 *   3: r0 = 0
 *   4: exit
 *
 * At insn 1, slot 0 still live (0x1) because 2-byte write
 * didn't kill it.
 */
// SEC("socket")
// __log_level(2)
// __msg("0: (7a) *(u64 *)(r10 -8) = 0         ; def: fp0-8")
// __msg("1: (6a) *(u16 *)(r10 -4) = 0{{$}}")
// __msg("2: (61) r0 = *(u32 *)(r10 -4)        ; use: fp0-4h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn two_byte_write_no_kill() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u16 *)(r10 - 4) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that a 1-byte write does NOT count as stack_def.
 *
 *   0: *(u64 *)(r10 - 8) = 0      def slots 0,1
 *   1: *(u8 *)(r10 - 4) = 0       NOT stack_def (1 < 4 bytes)
 *   2: r0 = *(u32 *)(r10 - 4)     use slot 0
 *   3: r0 = 0
 *   4: exit
 *
 * At insn 1, slot 0 still live (0x1).
 */
// SEC("socket")
// __log_level(2)
// __msg("0: (7a) *(u64 *)(r10 -8) = 0         ; def: fp0-8")
// __msg("1: (72) *(u8 *)(r10 -4) = 0")
// __msg("2: (61) r0 = *(u32 *)(r10 -4)        ; use: fp0-4h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn one_byte_write_no_kill() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u8 *)(r10 - 4) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test stack access beyond fp-256 exercising the second bitmask word.
 * fp-264 is SPI 32, slots 64-65, which are bits 0-1 of live_stack[1].
 *
 *   0: *(u64 *)(r10 - 264) = 0     def slots 64,65
 *   1: r0 = *(u64 *)(r10 - 264)    use slots 64,65
 *   2: r0 = 0
 *   3: exit
 *
 * At insn 1, live_stack high word has bits 0,1 set: 0x3:0x0.
 */
// SEC("socket")
// __log_level(2)
// __msg("1: (79) r0 = *(u64 *)(r10 -264)      ; use: fp0-264")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn high_stack_second_bitmask_word() {
    // asm: "*(u64 *)(r10 - 264) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that two separate 4-byte writes to each half of an SPI
 * together kill liveness for the full SPI.
 *
 *   0: *(u32 *)(r10 - 8) = 0      def slot 1 (upper half)
 *   1: *(u32 *)(r10 - 4) = 0      def slot 0 (lower half)
 *   2: r0 = *(u64 *)(r10 - 8)     use slots 0,1
 *   3: r0 = 0
 *   4: exit
 *
 * At insn 0: live_stack_before = 0x0 (both slots killed by insns 0,1).
 * At insn 1: live_stack_before = 0x2 (slot 1 still live, slot 0 killed here).
 */
// SEC("socket")
// __log_level(2)
// __msg("0: (62) *(u32 *)(r10 -8) = 0         ; def: fp0-8h")
// __msg("1: (62) *(u32 *)(r10 -4) = 0         ; def: fp0-4h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn two_four_byte_writes_kill_full_spi() {
    // asm: "*(u32 *)(r10 - 8) = 0;"
    // asm: "*(u32 *)(r10 - 4) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that 4-byte writes on both branches kill a slot at the
 * join point. Previously at SPI granularity, a 4-byte write was
 * not stack_def, so liveness would flow backward through the
 * branch that only had a 4-byte write.
 *
 *   0: call bpf_get_prandom_u32
 *   1: if r0 != 0 goto 1f
 *   2: *(u64 *)(r10 - 8) = 0       path A: def slots 0,1
 *   3: goto 2f
 * 1:4: *(u32 *)(r10 - 4) = 0       path B: def slot 0
 * 2:5: r0 = *(u32 *)(r10 - 4)      use slot 0
 *   6: r0 = 0
 *   7: exit
 *
 * Both paths define slot 0 before the read. At insn 1 (branch),
 * live_stack_before = 0x0 because slot 0 is killed on both paths.
 */
// SEC("socket")
// __log_level(2)
// __msg("1: (55) if r0 != 0x0 goto pc+2")
// __msg("2: (7a) *(u64 *)(r10 -8) = 0         ; def: fp0-8")
// __msg("3: (05) goto pc+1")
// __msg("4: (62) *(u32 *)(r10 -4) = 0         ; def: fp0-4h")
// __msg("5: (61) r0 = *(u32 *)(r10 -4)        ; use: fp0-4h")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn both_branches_kill_slot() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto 1f;"
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "goto 2f;"
    // asm: "1:"
    // asm: "*(u32 *)(r10 - 4) = 0;"
    // asm: "2:"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Soundness: cleaning the dead upper half of an SPI must not
 * affect the live lower half's type information for pruning.
 *
 * Both halves of SPI 0 are written separately. Only the lower
 * half (slot 0) is used as a 4-byte map key. The upper half
 * (slot 1) is dead and cleaned to STACK_INVALID.
 *
 * Path A: key stays 0 (STACK_ZERO) → non-null array lookup
 * Path B: key byte turns STACK_MISC → may-null array lookup
 * Deref without null check: safe for A, unsafe for B.
 *
 * If half-SPI cleaning incorrectly corrupted the live half's
 * type info, path A's cached state could generalize and unsoundly
 * prune path B.
 *
 * Expected: reject (path B unsafe).
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn half_spi_clean_preserves_stack_zero() {
    // asm: "*(u32 *)(r10 - 4) = 0;"
    // asm: "*(u32 *)(r10 - 8) = 0;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_nonconst%=;"
    // asm: "goto l_lookup%=;"
    // asm: "l_nonconst%=:"
    // asm: "*(u8 *)(r10 - 4) = r0;"
    // asm: "l_lookup%=:"
    // asm: "r2 = r10;"
    // asm: "r2 += -4;"
    // asm: "r1 = %[array_map_8b] ll;"
    // asm: "call %[bpf_map_lookup_elem];"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Model of scx_lavd's pick_idle_cpu_at_cpdom iat block:
 * conditional block with helper call and temporary stack spill,
 * spill dead after merge.
 *
 * Path A (fall-through): spill r6 to fp-8 across helper call
 * Path B (branch taken): skip the block entirely
 * At merge (insn 6): fp-8 is dead (never read after merge)
 *
 * Static liveness marks fp-8 dead at merge. clean_verifier_state()
 * converts path A's STACK_SPILL to STACK_INVALID. Path B has
 * STACK_INVALID. stacksafe() matches -> path B pruned -> "6: safe".
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __success
// __log_level(2)
// __msg("6: safe")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dead_spill_at_merge_enables_pruning() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "r6 = 7;"
    // asm: "if r0 != 0 goto l_skip%=;"
    // asm: "*(u64 *)(r10 - 8) = r6;"
    // asm: "call %[bpf_get_prandom_u32];"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * FP-offset tracking loses precision on second ADD, killing all liveness.
 *
 * fp_off_insn_xfer() handles "FP itself + negative imm" precisely
 * (e.g. r6 = r10; r6 += -24 -> slot 5).  But any subsequent ADD/SUB
 * on a register that already has non-zero spis falls through to
 * spis_set_all(), because the code only handles the FP-itself case.
 *
 * A write through this imprecise register enters the non-zero-spis
 * branch of set_indirect_stack_access(), which OR's the all-ones
 * mask into stack_def.  The backward liveness equation
 *
 *   stack_in = (stack_out & ~stack_def) | stack_use
 *
 * sees ~ALL = 0, killing ALL slot liveness at that instruction.
 *
 * At the merge pruning point, live_stack_before is empty.
 * clean_verifier_state() marks fp-8 as STACK_INVALID.
 * stacksafe() skips STACK_INVALID (line "continue"), so pruning
 * succeeds regardless of the current state's fp-8 value.
 * Path B is pruned, its null deref is never explored.
 *
 * Correct behavior: reject (path B dereferences NULL).
 * Bug behavior: accept (path B pruned away).
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R1 invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fp_add_loses_precision_kills_liveness() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_pathB%=;"
    // asm: "r1 = 0;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "goto l_merge%=;"
    // asm: "l_pathB%=:"
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "l_merge%=:"
    // asm: "r6 = r10;"
    // asm: "r6 += -24;"
    // asm: "r6 += 8;"
    // asm: "r7 = 0;"
    // asm: "*(u64 *)(r6 + 0) = r7;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R1 invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fp_spill_loses_precision_kills_liveness() {
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_pathB%=;"
    // asm: "r1 = 0;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "goto l_merge%=;"
    // asm: "l_pathB%=:"
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "l_merge%=:"
    // asm: "r6 = r10;"
    // asm: "r6 += -64;"
    // asm: "*(u64 *)(r10 - 160) = r6;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* === Tests for frame-based AT_FP tracking === */

/*
 * Test 1: conditional_stx_in_subprog
 * Subprog conditionally writes caller's slot.
 * Verify slot stays live (backward pass handles conditional def via CFG).
 *
 * Main writes fp-8=42, calls cond_writer(fp-8), reads fp-8.
 * cond_writer only writes on one path → parent_def only on that path.
 * The backward parent_live correctly keeps fp-8 live at entry
 * (conditional write doesn't kill liveness at the join).
 */
// SEC("socket")
// __log_level(2)
/* fp-8 live at call (callee conditionally writes → slot not killed) */
// __msg("1: (7b) *(u64 *)(r10 -8) = r1        ; def: fp0-8")
// __msg("4: (85) call pc+2{{$}}")
// __msg("5: (79) r0 = *(u64 *)(r10 -8)        ; use: fp0-8")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conditional_stx_in_subprog() {
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call cond_writer;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Conditionally writes to *(r1+0) */
unsafe extern "C" fn cond_writer() {
    // asm: "r6 = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "*(u64 *)(r6 + 0) = r0;"
    // asm: "1:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __msg("4: (85) call pc+{{.*}}                   ; use: fp0-16")
// __msg("7: (85) call pc+{{.*}}                   ; use: fp0-32")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multiple_callsites_different_offsets() {
    // asm: "*(u64 *)(r10 - 16) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "call read_first_param;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "call read_first_param;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Test 3: nested_fp_passthrough
 * main→A→B, main's FP forwarded to B. B accesses main's stack.
 * Verify liveness propagates through.
 *
 * Main passes fp-32 to outer_forwarder, which passes it to inner_reader.
 * inner_reader reads at arg+0 (= main's fp-32).
 * parent_live propagates transitively: inner→outer→main.
 */
// SEC("socket")
// __log_level(2)
/* At call to outer_forwarder: main's fp-32 (slots 6,7) should be live */
// __msg("6: (85) call pc+{{.*}}                   ; use: fp0-32")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nested_fp_passthrough() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u64 *)(r10 - 16) = 0;"
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "call outer_forwarder;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Forwards arg to inner_reader */
unsafe extern "C" fn outer_forwarder() {
    // asm: "call inner_reader;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn inner_reader() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Test 4: callee_must_write_before_read
 * Callee unconditionally writes parent slot before reading.
 * Verify slot is NOT live at call site (parent_def kills it).
 */
// SEC("socket")
// __log_level(2)
/* fp-8 NOT live at call: callee writes before reading (parent_def kills it) */
// __msg("2: .12345.... (85) call pc+")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callee_must_write_before_read() {
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call write_then_read;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Unconditionally writes *(r1+0), then reads it back */
unsafe extern "C" fn write_then_read() {
    // asm: "r6 = r1;"
    // asm: "r7 = 99;"
    // asm: "*(u64 *)(r6 + 0) = r7;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Test 5: return_site_liveness_bleeding
 * Main calls subprog twice. Slot used after one call but not the other.
 * Context-insensitive: slot conservatively live at both.
 *
 * After first call: read fp-8.
 * After second call: don't read fp-8.
 * Since parent_live is per-subprog (not per call-site),
 * fp-8 is live at both call sites.
 */
// SEC("socket")
// __log_level(2)
/* Both calls have fp-8 live due to context-insensitive parent_live */
// __msg("3: (85) call pc+{{.*}}                   ; use: fp0-8")
// __msg("7: (85) call pc+{{.*}}                   ; use: fp0-8")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn return_site_liveness_bleeding() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call read_first_param;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __msg("9: (85) call bpf_loop#181            ; use: fp0-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback_conditional_read_beyond_ctx() {
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = 2;"
    // asm: "r2 = cb_cond_read ll;"
    // asm: "r3 = r10;"
    // asm: "r3 += -8;"
    // asm: "r4 = 0;"
    // asm: "call %[bpf_loop];"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Callback conditionally reads *(ctx - 8) = caller fp-16 */
unsafe extern "C" fn cb_cond_read() {
    // asm: "r6 = r2;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __msg("14: (7b) *(u64 *)(r6 -8) = r7         ; def: fp0-16")
// __msg("15: (79) r0 = *(u64 *)(r6 -8)         ; use: fp0-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback_write_before_read_kills() {
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = 2;"
    // asm: "r2 = cb_write_read ll;"
    // asm: "r3 = r10;"
    // asm: "r3 += -8;"
    // asm: "r4 = 0;"
    // asm: "call %[bpf_loop];"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Callback unconditionally writes *(ctx-8), then reads it back.
 * The write (parent_def) kills liveness before entry.
 */
unsafe extern "C" fn cb_write_read() {
    // asm: "r6 = r2;"
    // asm: "r7 = 99;"
    // asm: "*(u64 *)(r6 - 8) = r7;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * bpf_loop callback conditionally writes fp-16 then unconditionally
 * reads it. The conditional write does NOT kill liveness
 */
// SEC("socket")
// __log_level(2)
// __msg("9: (85) call bpf_loop#181            ; use: fp0-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback_conditional_write_preserves() {
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = 2;"
    // asm: "r2 = cb_cond_write_read ll;"
    // asm: "r3 = r10;"
    // asm: "r3 += -8;"
    // asm: "r4 = 0;"
    // asm: "call %[bpf_loop];"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn cb_cond_write_read() {
    // asm: "r6 = r2;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "*(u64 *)(r6 - 8) = r0;"
    // asm: "1:"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Two bpf_loop calls with the same callback but different ctx pointers.
 *
 * First call: ctx=fp-8, second call: ctx=fp-24.
 */
// SEC("socket")
// __log_level(2)
// __msg(" 8: (85) call bpf_loop{{.*}}            ; use: fp0-8")
// __msg("15: (85) call bpf_loop{{.*}}            ; use: fp0-24")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback_two_calls_different_ctx() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "r1 = 1;"
    // asm: "r2 = cb_read_ctx ll;"
    // asm: "r3 = r10;"
    // asm: "r3 += -8;"
    // asm: "r4 = 0;"
    // asm: "call %[bpf_loop];"
    // asm: "r1 = 1;"
    // asm: "r2 = cb_read_ctx ll;"
    // asm: "r3 = r10;"
    // asm: "r3 += -24;"
    // asm: "r4 = 0;"
    // asm: "call %[bpf_loop];"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Callback reads at ctx+0 unconditionally */
unsafe extern "C" fn cb_read_ctx() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Reproducer for unsound pruning in refined_caller_live_stack().
 *
 * Three-level call chain: main → mid_fwd → grandchild_deref.
 * Main passes &fp-8 to mid_fwd, which forwards R1 to grandchild_deref.
 * grandchild_deref reads main's fp-8 through the forwarded pointer
 * and dereferences the result.
 *
 * refined_caller_live_stack() has a callee_offset++ when mid_fwd
 * (frame 1) is mid-call. This drops the transitive parent_live
 * contribution at mid_fwd's call instruction — the only place
 * where grandchild_deref's read of main's fp-8 is recorded.
 * As a result, main's fp-8 is cleaned to STACK_INVALID at the
 * pruning point inside grandchild_deref, and path B is
 * incorrectly pruned against path A.
 *
 * Path A: main stores PTR_TO_MAP_VALUE at fp-8
 * Path B: main stores scalar 42 at fp-8
 *
 * Correct behavior: reject (path B dereferences scalar)
 * Bug behavior: accept (path B pruned against cleaned path A)
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R0 invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn transitive_parent_stack_read_unsound() {
    // asm: "r1 = %[map] ll;"
    // asm: "*(u32 *)(r10 - 32) = 0;"
    // asm: "r2 = r10;"
    // asm: "r2 += -32;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 == 0 goto l_exit%=;"
    // asm: "r6 = r0;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_scalar%=;"
    // asm: "*(u64 *)(r10 - 8) = r6;"
    // asm: "goto l_merge%=;"
    // asm: "l_scalar%=:"
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "l_merge%=:"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call mid_fwd;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    // asm: "l_exit%=:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Forwards R1 (ptr to main's fp-8) to grandchild_deref */
unsafe extern "C" fn mid_fwd() {
    // asm: "call grandchild_deref;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Reads main's fp-8 through forwarded pointer, dereferences result */
unsafe extern "C" fn grandchild_deref() {
    // asm: "goto +0;"
    // asm: "goto +0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __success
// __msg("14: (79) r1 = *(u64 *)(r10 -8) // r6=fp0-8 r7=fp1-16 fp-8=fp1-16 fp-16=fp0-8")
// __msg("15: (79) r0 = *(u64 *)(r1 +0) // r1=fp1-16 r6=fp0-8 r7=fp1-16 fp-8=fp1-16 fp-16=fp0-8")
// __msg("stack use/def subprog#1 mid_two_fp_threshold (d1,cs2):")
// __msg("14: (79) r1 = *(u64 *)(r10 -8)        ; use: fp1-8")
// __msg("15: (79) r0 = *(u64 *)(r1 +0)         ; use: fp1-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn two_fp_clear_stack_threshold() {
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call mid_two_fp_threshold;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn mid_two_fp_threshold() {
    // asm: "r6 = r1;"
    // asm: "r7 = r10;"
    // asm: "r7 += -16;"
    // asm: "*(u64 *)(r10 - 8) = r7;"
    // asm: "*(u64 *)(r10 - 16) = r6;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "r2 = r6;"
    // asm: "call inner_nop_fptest;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn inner_nop_fptest() {
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __success
// __msg("13: (79) r1 = *(u64 *)(r10 -8) // r6=fp0-8 r7=fp1-16 fp-8=fp1-16 fp-16=fp0-8")
// __msg("14: (79) r0 = *(u64 *)(r1 +0) // r1=fp1-16 r6=fp0-8 r7=fp1-16 fp-8=fp1-16 fp-16=fp0-8")
// __msg("stack use/def subprog#1 mid_one_fp_threshold (d1,cs2):")
// __msg("13: (79) r1 = *(u64 *)(r10 -8)        ; use: fp1-8")
// __msg("14: (79) r0 = *(u64 *)(r1 +0)         ; use: fp1-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn one_fp_clear_stack_threshold() {
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call mid_one_fp_threshold;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn mid_one_fp_threshold() {
    // asm: "r6 = r1;"
    // asm: "r7 = r10;"
    // asm: "r7 += -16;"
    // asm: "*(u64 *)(r10 - 8) = r7;"
    // asm: "*(u64 *)(r10 - 16) = r6;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call inner_nop_fptest;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Reproducer for unsound pruning when a subprog forwards a parent
 * stack pointer (AT_PARENT) to a helper with a memory argument.
 *
 * set_call_stack_access_at() previously only tracked AT_CURRENT args,
 * skipping AT_PARENT entirely. This meant helper reads through parent
 * stack pointers did not set parent_use, letting the slot appear dead
 * at pruning checkpoints inside the subprog.
 *
 * Program shape:
 *   main:
 *     *(u32)(fp-4) = 0             key = STACK_ZERO (const 0)
 *     call bpf_get_prandom_u32
 *     if r0 != 0 goto clobber      path A (fall-through) first
 *     goto merge
 *   clobber:
 *     *(u8)(fp-4) = r0             path B: key[0] = STACK_MISC
 *   merge:
 *     r1 = fp - 4
 *     call fwd_parent_key_to_helper
 *     r0 = 0
 *     exit
 *
 *   fwd_parent_key_to_helper(r1 = &caller_fp-4):
 *     goto +0                      checkpoint
 *     r2 = r1                      R2 = AT_PARENT ptr to caller fp-4
 *     r1 = array_map_8b ll         R1 = array map
 *     call bpf_map_lookup_elem     reads key_size(4) from parent fp-4
 *     r0 = *(u64 *)(r0 + 0)        deref without null check
 *     r0 = 0
 *     exit
 *
 * Path A: STACK_ZERO key = const 0 -> array lookup -> PTR_TO_MAP_VALUE
 *         (non-NULL for in-bounds const key) -> deref OK.
 * Path B: STACK_MISC key = unknown -> array lookup ->
 *         PTR_TO_MAP_VALUE_OR_NULL -> deref UNSAFE.
 *
 * Bug: AT_PARENT R2 arg to bpf_map_lookup_elem skipped -> parent_use
 *      not set -> fp-4 cleaned at checkpoint -> STACK_ZERO collapses
 *      to STACK_INVALID -> path B pruned -> deref never checked.
 *
 * Correct verifier behavior: reject (path B deref of map_value_or_null).
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn helper_parent_stack_read_unsound() {
    // asm: "*(u32 *)(r10 - 4) = 0;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_clobber%=;"
    // asm: "goto l_merge%=;"
    // asm: "l_clobber%=:"
    // asm: "*(u8 *)(r10 - 4) = r0;"
    // asm: "l_merge%=:"
    // asm: "r1 = r10;"
    // asm: "r1 += -4;"
    // asm: "call fwd_parent_key_to_helper;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Subprog forwards parent stack pointer to bpf_map_lookup_elem as key
 * on an array map, then dereferences the result without a null check.
 * R1 = &parent_fp-4 (AT_PARENT in this frame).
 *
 * The helper reads key_size(4) bytes from parent stack.  The deref of
 * R0 reads the map value, NOT parent stack, so record_insn_mem_accesses
 * does not set parent_use for it.  The ONLY parent stack access is
 * through the helper's R2 arg.
 */
unsafe extern "C" fn fwd_parent_key_to_helper() {
    // asm: "goto +0;"
    // asm: "r2 = r1;"
    // asm: "r1 = %[array_map_8b] ll;"
    // asm: "call %[bpf_map_lookup_elem];"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Regression for keeping later helper args after a whole-stack fallback
 * on an earlier local arg.  The first bpf_snprintf() arg is a local
 * frame-derived pointer with offset-imprecise tracking (`fp1 ?`), which
 * conservatively marks the whole local stack live.  The fourth arg still
 * forwards &parent_fp-8 and must contribute nonlocal_use[0]=0:3.
 */
// SEC("socket")
// __log_level(2)
// __success
// __msg("call bpf_snprintf{{.*}}        ; use: fp1-8..-512 fp0-8")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn helper_arg_fallback_keeps_scanning() {
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call helper_snprintf_parent_after_local_fallback;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn helper_snprintf_parent_after_local_fallback() {
    // asm: "r6 = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "r0 &= 8;"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "r1 += r0;"
    // asm: "r2 = 8;"
    // asm: "r3 = %[snprintf_u64_fmt] ll;"
    // asm: "r4 = r6;"
    // asm: "r5 = 8;"
    // asm: "call %[bpf_snprintf];"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Test that propagate_callee_ancestor() correctly chains ancestor
 * liveness across sequential calls within a single frame.
 *
 * main → mid_seq_touch → {nop_callee, deref_ancestor}
 *
 * mid_seq_touch receives two pointers: R1 = &main_fp-8 (forwarded to
 * deref_ancestor) and R2 = &main_fp-16 (read directly by mid_seq_touch).
 * The direct read of fp-16 forces ensure_anc_arrays() to allocate
 * ancestor_live[0] for mid_seq_touch, so refined_caller_live_stack()
 * uses the refined path (not the conservative fallback).
 *
 * mid_seq_touch calls nop_callee first (no-op, creates a pruning point),
 * then calls deref_ancestor which reads main's fp-8 and dereferences it.
 *
 * propagate_callee_ancestor() propagates deref_ancestor's entry
 * ancestor_live[0] into mid_seq_touch's anc_use[0] at the call-to-deref
 * instruction.  mid_seq_touch's backward pass flows this backward so
 * ancestor_live[0] includes fp-8 at the pruning point between the calls.
 *
 * Without propagation, mid_seq_touch's ancestor_live[0] only has fp-16
 * (from the direct read) — fp-8 is missing.  refined_caller_live_stack()
 * Term 1 says fp-8 is dead, the verifier cleans it, and path B
 * (scalar 42) is incorrectly pruned against path A (MAP_VALUE).
 *
 * Path A: main stores PTR_TO_MAP_VALUE at fp-8  → deref succeeds
 * Path B: main stores scalar 42 at fp-8         → deref must fail
 *
 * Correct: reject (path B dereferences scalar)
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure __msg("R0 invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn propagate_callee_ancestor_chain() {
    // asm: "r1 = %[map] ll;"
    // asm: "*(u32 *)(r10 - 32) = 0;"
    // asm: "r2 = r10;"
    // asm: "r2 += -32;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 == 0 goto l_exit%=;"
    // asm: "r6 = r0;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 != 0 goto l_scalar%=;"
    // asm: "*(u64 *)(r10 - 8) = r6;"
    // asm: "goto l_merge%=;"
    // asm: "l_scalar%=:"
    // asm: "r1 = 42;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "l_merge%=:"
    // asm: "r1 = 99;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "r2 = r10;"
    // asm: "r2 += -16;"
    // asm: "call mid_seq_touch;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    // asm: "l_exit%=:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * R1 = &main_fp-8 (forwarded to deref_ancestor)
 * R2 = &main_fp-16 (read directly here → allocates ancestor_live[0])
 *
 * Reads main's fp-16 to force ancestor_live[0] allocation, then
 * calls nop_callee (pruning point), then deref_ancestor.
 */
unsafe extern "C" fn mid_seq_touch() {
    // asm: "r6 = r1;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn nop_callee() {
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Reads main's fp-8 through forwarded pointer, dereferences result */
unsafe extern "C" fn deref_ancestor() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Test: callee loads an fp-derived pointer from caller's stack, then
 * reads through it to access another caller stack slot.
 *
 * main stores PTR_TO_MAP_VALUE at fp-24, stores &fp-24 (an fp-derived
 * pointer) at fp-8, passes &fp-8 through mid_fwd_spilled_ptr to
 * load_ptr_deref_grandchild.  The leaf loads the pointer from main's
 * fp-8, then reads main's fp-24 through the loaded pointer.
 *
 * fill_from_stack() in arg_track_xfer() only handles local-frame
 * FP-derived loads (src_is_local_fp check requires frame == depth).
 * When a callee loads from a parent-frame pointer (frame < depth),
 * the loaded value gets ARG_NONE instead of being recognized as
 * fp-derived.  Subsequent reads through that loaded pointer are
 * invisible to liveness — nonlocal_use is never set for fp-24.
 *
 * clean_live_states() cleans the current state at every prune point.
 * Because liveness misses fp-24, refined_caller_live_stack() tells
 * __clean_func_state() that fp-24 is dead, which destroys the
 * PTR_TO_MAP_VALUE spill before the grandchild can read it.
 * The grandchild then reads STACK_INVALID → scalar, and the deref
 * is rejected with "R0 invalid mem access 'scalar'" — even though
 * fp-24 is genuinely live and holds a valid map pointer.
 *
 * This is a false positive: a valid program incorrectly rejected.
 */
// SEC("socket")
// __flag(BPF_F_TEST_STATE_FREQ)
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spilled_fp_cross_frame_deref() {
    // asm: "r1 = %[map] ll;"
    // asm: "*(u32 *)(r10 - 32) = 0;"
    // asm: "r2 = r10;"
    // asm: "r2 += -32;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 == 0 goto l_exit%=;"
    // asm: "*(u64 *)(r10 - 24) = r0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call mid_fwd_spilled_ptr;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    // asm: "l_exit%=:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Forwards R1 (ptr to main's fp-8, which holds &main_fp-24) to leaf */
unsafe extern "C" fn mid_fwd_spilled_ptr() {
    // asm: "call load_ptr_deref_grandchild;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * R1 = &main_fp-8 (where main stored ptr to fp-24)
 * Loads the ptr from main's fp-8, reads main's fp-24 through it,
 * then dereferences the result.
 */
unsafe extern "C" fn load_ptr_deref_grandchild() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Exercise merge_nonlocal_live().
 *
 * merge_shared_mid is analyzed twice (once from each wrapper), so the
 * callsite within merge_shared_mid that calls merge_leaf_read gets its
 * nonlocal_live info merged twice via merge_nonlocal_live().
 */
// SEC("socket")
// __log_level(2)
// __success
// __msg("14: (85) call pc+2	r1: fp0-16")
// __msg("17: (79) r0 = *(u64 *)(r1 +0) // r1=fp0-16")
// __msg("14: (85) call pc+2	r1: fp0-8")
// __msg("17: (79) r0 = *(u64 *)(r1 +0) // r1=fp0-8")
// __msg("5: (85) call pc+{{.*}}                   ; use: fp0-8 fp0-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_merge_nonlocal_live() {
    // asm: "r1 = 0;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call merge_wrapper_a;"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "call merge_wrapper_b;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn merge_wrapper_a() {
    // asm: "call merge_shared_mid;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn merge_wrapper_b() {
    // asm: "call merge_shared_mid;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn merge_shared_mid() {
    // asm: "call merge_leaf_read;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn merge_leaf_read() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Same bpf_loop instruction calls different callbacks depending on branch. */
// SEC("socket")
// __log_level(2)
// __success
// __msg("call bpf_loop#181            ; use: fp2-8..-512 fp1-8..-512 fp0-8..-512")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_loop_two_callbacks() {
    // asm: "r1 = 0;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call dyn_wrapper_a;"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "call dyn_wrapper_b;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn dyn_wrapper_a() {
    // asm: "call mid_dynamic_cb;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn dyn_wrapper_b() {
    // asm: "call mid_dynamic_cb;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn mid_dynamic_cb() {
    // asm: "r6 = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "r2 = dyn_cb_a ll;"
    // asm: "goto 2f;"
    // asm: "1:"
    // asm: "r2 = dyn_cb_b ll;"
    // asm: "2:"
    // asm: "r1 = 1;"
    // asm: "r3 = r6;"
    // asm: "r4 = 0;"
    // asm: "call %[bpf_loop];"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Callback A/B: read parent stack through ctx */
unsafe extern "C" fn dyn_cb_a() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn dyn_cb_b() {
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Path A: r0 = map_lookup result (non-FP, ARG_NONE for stack tracking)
 * Path B: r0 = fp-8 (FP-derived, frame=0, off=-8)
 * At the join: r0 is not guaranteed to be a frame pointer.
 */
// SEC("socket")
// __log_level(2)
// __msg("10: (79) r0 = *(u64 *)(r10 -8) // r0=fp0-8|fp0+0")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_or_non_stack_write() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "r2 = r10;"
    // asm: "r2 += -4;"
    // asm: "r1 = %[map] ll;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 != 0 goto 1f;"
    // asm: "r0 = r10;"
    // asm: "r0 += -8;"
    // asm: "1:"
    // asm: "*(u64 *)(r0 + 0) = 7;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// SEC("socket")
// __log_level(2)
// __flag(BPF_F_TEST_STATE_FREQ)
// __msg("subprog#2 write_first_read_second:")
// __msg("17: (7a) *(u64 *)(r1 +0) = 42{{$}}")
// __msg("18: (79) r0 = *(u64 *)(r2 +0) // r1=fp0-8 r2=fp0-16{{$}}")
// __msg("stack use/def subprog#2 write_first_read_second (d2,cs15):")
// __msg("17: (7a) *(u64 *)(r1 +0) = 42{{$}}")
// __msg("18: (79) r0 = *(u64 *)(r2 +0)         ; use: fp0-8 fp0-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shared_instance_must_write_overwrite() {
    // asm: "r1 = 1;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "r2 = r10;"
    // asm: "r2 += -16;"
    // asm: "call forwarding_rw;"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "r2 = r10;"
    // asm: "r2 += -8;"
    // asm: "call forwarding_rw;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn forwarding_rw() {
    // asm: "call write_first_read_second;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn write_first_read_second() {
    // asm: "*(u64 *)(r1 + 0) = 42;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Shared must_write when (callsite, depth) instance is reused.
 * Main calls fwd_to_stale_wr at two sites. fwd_to_stale_wr calls
 * stale_wr_leaf at a single internal callsite. Both calls share
 * stale_wr_leaf's (callsite, depth) instance.
 *
 * Call 1: stale_wr_leaf(map_value, fp-8) writes map, reads fp-8.
 * Call 2: stale_wr_leaf(fp-8, fp-8) writes fp-8, reads fp-8.
 *
 * The analysis can't presume that stale_wr_leaf() always writes fp-8,
 * it must conservatively join must_write masks computed for both calls.
 */
// SEC("socket")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stale_must_write_cross_callsite() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "*(u32 *)(r10 - 16) = 0;"
    // asm: "r1 = %[map] ll;"
    // asm: "r2 = r10;"
    // asm: "r2 += -16;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "r1 = r0;"
    // asm: "r2 = r10;"
    // asm: "r2 += -8;"
    // asm: "call fwd_to_stale_wr;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "r2 = r1;"
    // asm: "call fwd_to_stale_wr;"
    // asm: "1:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn fwd_to_stale_wr() {
    // asm: "call stale_wr_leaf;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn stale_wr_leaf() {
    // asm: "*(u64 *)(r1 + 0) = 42;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// Original conditional: #ifdef CAN_USE_LOAD_ACQ_STORE_REL

// SEC("socket")
// __log_level(2)
// __success
// __msg("*(u64 *)(r0 +0) = 42         ; def: fp0-16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_acquire_dont_clear_dst() {
    // asm: "r0 = r10;"
    // asm: "r0 += -16;"
    // asm: "*(u64 *)(r0 + 0) = r0;"
    // asm: ".8byte %[load_acquire_insn];"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


// Original conditional end: CAN_USE_LOAD_ACQ_STORE_REL

// SEC("socket")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn imprecise_fill_loses_cross_frame() {
    // asm: "*(u64 *)(r10 - 8) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "call imprecise_fill_cross_frame;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn imprecise_fill_cross_frame() {
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r2 = -8;"
    // asm: "r1 += r2;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that spill_to_stack with multi-offset dst (sz=8) joins instead
 * of overwriting. r1 has offsets [-8, -16]. Both slots hold FP-derived
 * pointers. Writing through r1 should join *val with existing values,
 * not destroy them.
 *
 *   fp-8  = &fp-24
 *   fp-16 = &fp-32
 *   r1 = fp-8 or fp-16 (two offsets from branch)
 *   *(u64 *)(r1 + 0) = &fp-24   -- writes to one slot, other untouched
 *   r0 = *(u64 *)(r10 - 16)     -- fill from fp-16
 *   r0 = *(u64 *)(r0 + 0)       -- deref: should produce use
 */
// SEC("socket")
// __log_level(2)
// __success
// __msg("20: (79) r0 = *(u64 *)(r10 -16)")
// __msg("21: (79) r0 = *(u64 *)(r0 +0)         ; use: fp0-24 fp0-32")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spill_join_with_multi_off() {
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "goto 2f;"
    // asm: "1:"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "2:"
    // asm: "r2 = r10;"
    // asm: "r2 += -24;"
    // asm: "*(u64 *)(r1 + 0) = r2;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/* Test that spill_to_stack with imprecise dst (off_cnt == 0, sz=8)
 * joins instead of overwriting. Use "r2 = -8; r1 += r2" to make
 * arg tracking lose offset precision while the main verifier keeps
 * r1 as PTR_TO_STACK with fixed offset. Both slots hold FP-derived
 * pointers. Writing through r1 should join *val with existing
 * values, not destroy them.
 *
 *   fp-8  = &fp-24
 *   fp-16 = &fp-32
 *   r1 = fp-8 (imprecise to arg tracking)
 *   *(u64 *)(r1 + 0) = &fp-24   -- since r1 is imprecise, this adds &fp-24
 *                                  to the set of possible values for all slots,
 *                                  hence the values at fp-16 become [fp-24, fp-32]
 *   r0 = *(u64 *)(r10 - 16)
 *   r0 = *(u64 *)(r0 + 0)       -- deref: should produce use of fp-24 or fp-32
 */
// SEC("socket")
// __log_level(2)
// __success
// __msg("15: (79) r0 = *(u64 *)(r0 +0)         ; use: fp0-24 fp0-32")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spill_join_with_imprecise_off() {
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = r10;"
    // asm: "r2 = -8;"
    // asm: "r1 += r2;"
    // asm: "r3 = r10;"
    // asm: "r3 += -24;"
    // asm: "*(u64 *)(r1 + 0) = r3;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Same as spill_join_with_multi_off but the write is BPF_ST (store
 * immediate) instead of BPF_STX. BPF_ST goes through
 * clear_stack_for_all_offs() rather than spill_to_stack(), and that
 * path also needs to join instead of overwriting.
 *
 *   fp-8  = &fp-24
 *   fp-16 = &fp-32
 *   r1 = fp-8 or fp-16 (two offsets from branch)
 *   *(u64 *)(r1 + 0) = 0        -- BPF_ST with immediate
 *   r0 = *(u64 *)(r10 - 16)     -- fill from fp-16
 *   r0 = *(u64 *)(r0 + 0)       -- deref: should produce use
 */
// SEC("socket")
// __log_level(2)
// __failure
// __msg("15: (7a) *(u64 *)(r1 +0) = 0	fp-8: fp0-24 -> fp0-24|fp0+0	fp-16: fp0-32 -> fp0-32|fp0+0")
// __msg("17: (79) r0 = *(u64 *)(r0 +0)         ; use: fp0-32")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn st_imm_join_with_multi_off() {
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "goto 2f;"
    // asm: "1:"
    // asm: "r1 = r10;"
    // asm: "r1 += -16;"
    // asm: "2:"
    // asm: "*(u64 *)(r1 + 0) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Check that BPF_ST with a known offset fully overwrites stack slot
 * from the arg tracking point of view.
 */
// SEC("socket")
// __log_level(2)
// __success
// __msg("5: (7a) *(u64 *)(r1 +0) = 0	fp-8: fp0-16 -> _{{$}}")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn st_imm_join_with_single_off() {
    // asm: "r2 = r10;"
    // asm: "r2 += -16;"
    // asm: "*(u64 *)(r10 - 8) = r2;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "*(u64 *)(r1 + 0) = 0;"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Same as spill_join_with_imprecise_off but the write is BPF_ST.
 * Use "r2 = -8; r1 += r2" to make arg tracking lose offset
 * precision while the main verifier keeps r1 as fixed-offset.
 *
 *   fp-8  = &fp-24
 *   fp-16 = &fp-32
 *   r1 = fp-8 (imprecise to arg tracking)
 *   *(u64 *)(r1 + 0) = 0        -- BPF_ST with immediate
 *   r0 = *(u64 *)(r10 - 16)     -- fill from fp-16
 *   r0 = *(u64 *)(r0 + 0)       -- deref: should produce use
 */
// SEC("socket")
// __log_level(2)
// __success
// __msg("13: (79) r0 = *(u64 *)(r0 +0)         ; use: fp0-32")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn st_imm_join_with_imprecise_off() {
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u64 *)(r10 - 32) = 0;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "*(u64 *)(r10 - 8) = r1;"
    // asm: "r1 = r10;"
    // asm: "r1 += -32;"
    // asm: "*(u64 *)(r10 - 16) = r1;"
    // asm: "r1 = r10;"
    // asm: "r2 = -8;"
    // asm: "r1 += r2;"
    // asm: "*(u64 *)(r1 + 0) = 0;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


/*
 * Test that spilling through an ARG_IMPRECISE pointer joins with
 * existing at_stack values. Subprog receives r1 = fp0-24 and
 * r2 = map_value, creates an ARG_IMPRECISE pointer by joining caller
 * and callee FP on two branches.
 *
 * Setup: callee spills &fp1-16 to fp1-8 (precise, tracked).
 * Then writes map_value through ARG_IMPRECISE r1 — on path A
 * this hits fp1-8, on path B it hits caller stack.
 * Since spill_to_stack is skipped for ARG_IMPRECISE dst,
 * fp1-8 tracking isn't joined with none.
 *
 * Expected after the imprecise write:
 * - arg tracking should show fp1-8 = fp1-16|fp1+0 (joined with none)
 * - read from fp1-8 and deref should produce use for fp1-16
 * - write through it should NOT produce def for fp1-16
 */
// SEC("socket")
// __log_level(2)
// __success
// __msg("26: (79) r0 = *(u64 *)(r10 -8) // r1=IMP3 r6=fp0-24 r7=fp1-16 fp-8=fp1-16|fp1+0")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn imprecise_dst_spill_join() {
    // asm: "*(u64 *)(r10 - 24) = 0;"
    // asm: "*(u32 *)(r10 - 32) = 0;"
    // asm: "r1 = %[map] ll;"
    // asm: "r2 = r10;"
    // asm: "r2 += -32;"
    // asm: "call %[bpf_map_lookup_elem];"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "r1 = r10;"
    // asm: "r1 += -24;"
    // asm: "r2 = r0;"
    // asm: "call imprecise_dst_spill_join_sub;"
    // asm: "1:"
    // asm: "r0 = 0;"
    // asm: "exit;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}


unsafe extern "C" fn imprecise_dst_spill_join_sub() {
    // asm: "r6 = r1;"
    // asm: "r8 = r2;"
    // asm: "*(u64 *)(r10 - 16) = 0;"
    // asm: "r7 = r10;"
    // asm: "r7 += -16;"
    // asm: "*(u64 *)(r10 - 8) = r7;"
    // asm: "call %[bpf_get_prandom_u32];"
    // asm: "r1 = r6;"
    // asm: "if r0 == 0 goto 1f;"
    // asm: "r1 = r10;"
    // asm: "r1 += -8;"
    // asm: "1:"
    // asm: "*(u64 *)(r1 + 0) = r8;"
    unsafe { core::arch::asm!("/* BPF inline assembly preserved in comments above. */"); }
}
