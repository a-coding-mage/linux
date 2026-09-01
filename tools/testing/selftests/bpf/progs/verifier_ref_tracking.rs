// SPDX-License-Identifier: GPL-2.0
// Converted from tools/testing/selftests/bpf/verifier/ref_tracking.c
//
// Rust translation of testing/selftests/bpf/progs/verifier_ref_tracking.c.
// C include dependencies intentionally remain external:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, ../../../include/linux/filter.h, "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

pub type __u64 = u64;
pub type __s32 = i32;

// C verifier/libbpf metadata macros such as SEC(), __description(),
// __success, __failure, __retval(), __msg(), __flag(), __auxiliary,
// __naked, __noinline, __ksym, __uint(), __type(), and __array() are supplied
// by the original BPF C environment. Their intent is preserved in comments
// attached to the corresponding Rust items below.

// #define BPF_SK_LOOKUP(func) ...
macro_rules! BPF_SK_LOOKUP {
    ($func:literal) => {
        concat!(
            "r2 = 0;\n",
            "*(u32*)(r10 - 8) = r2;\n",
            "*(u64*)(r10 - 16) = r2;\n",
            "*(u64*)(r10 - 24) = r2;\n",
            "*(u64*)(r10 - 32) = r2;\n",
            "*(u64*)(r10 - 40) = r2;\n",
            "*(u64*)(r10 - 48) = r2;\n",
            "r2 = r10;\n",
            "r2 += -48;\n",
            "r3 = %[sizeof_bpf_sock_tuple];\n",
            "r4 = 0;\n",
            "r5 = 0;\n",
            "call %[", $func, "];\n",
        )
    };
}

#[repr(C)]
pub struct bpf_key {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn bpf_key_put(key: *mut bpf_key);
    pub fn bpf_lookup_system_key(id: __u64) -> *mut bpf_key;
    pub fn bpf_lookup_user_key(serial: __s32, flags: __u64) -> *mut bpf_key;
}

/* BTF FUNC records are not generated for kfuncs referenced
 * from inline assembly. These records are necessary for
 * libbpf to link the program. The function below is a hack
 * to ensure that BTF FUNC records are generated.
 */
pub unsafe extern "C" fn __kfunc_btf_root() {
    unsafe {
        bpf_key_put(core::ptr::null_mut());
        let _ = bpf_lookup_system_key(0);
        let _ = bpf_lookup_user_key(0, 0);
    }
}

pub const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

// SEC(".maps")
// struct { __uint(type, BPF_MAP_TYPE_ARRAY); __uint(max_entries, 1);
//          __type(key, int); __type(value, struct test_val); } map_array_48b;
unsafe extern "C" {
    pub static mut map_array_48b: core::ffi::c_void;
}

// SEC(".maps")
// struct { __uint(type, BPF_MAP_TYPE_RINGBUF); __uint(max_entries, 4096); } map_ringbuf;
unsafe extern "C" {
    pub static mut map_ringbuf: core::ffi::c_void;
}

unsafe extern "C" {
    pub fn dummy_prog_42_tc();
    pub fn dummy_prog_24_tc();
    pub fn dummy_prog_loop1_tc();
}

// SEC(".maps") BPF_MAP_TYPE_PROG_ARRAY, max_entries 4, key_size sizeof(int),
// values[0] = dummy_prog_42_tc, values[1] = dummy_prog_loop1_tc,
// values[2] = dummy_prog_24_tc.
unsafe extern "C" {
    pub static mut map_prog1_tc: core::ffi::c_void;
}

macro_rules! naked_bpf_prog {
    ($(#[$meta:meta])* $vis:vis fn $name:ident, $asm:expr) => {
        $(#[$meta])*
        $vis unsafe extern "C" fn $name() {
            core::arch::asm!($asm, options(noreturn));
        }
    };
}

// SEC("tc") __auxiliary __naked
naked_bpf_prog!(pub fn dummy_prog_42_tc_body, "r0 = 42; exit;");

// SEC("tc") __auxiliary __naked
naked_bpf_prog!(pub fn dummy_prog_24_tc_body, "r0 = 24; exit;");

// SEC("tc") __auxiliary __naked
naked_bpf_prog!(pub fn dummy_prog_loop1_tc_body, concat!(
    "r3 = 1;\n",
    "r2 = %[map_prog1_tc] ll;\n",
    "call %[bpf_tail_call];\n",
    "r0 = 41;\n",
    "exit;\n",
));

// SEC("tc") __description("reference tracking: leak potential reference")
// __failure __msg("Unreleased reference") __naked
naked_bpf_prog!(pub fn reference_tracking_leak_potential_reference, concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r6 = r0; /* leak reference */\n",
    "exit;\n",
));

// SEC("tc") __description("reference tracking: leak potential reference to sock_common")
// __failure __msg("Unreleased reference") __naked
naked_bpf_prog!(pub fn potential_reference_to_sock_common_1, concat!(
    BPF_SK_LOOKUP!("bpf_skc_lookup_tcp"),
    "r6 = r0; /* leak reference */\n",
    "exit;\n",
));

// SEC("tc") __description("reference tracking: leak potential reference on stack")
// __failure __msg("Unreleased reference") __naked
naked_bpf_prog!(pub fn leak_potential_reference_on_stack, concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r4 = r10;\n",
    "r4 += -8;\n",
    "*(u64*)(r4 + 0) = r0;\n",
    "r0 = 0;\n",
    "exit;\n",
));

// SEC("tc") __description("reference tracking: leak potential reference on stack 2")
// __failure __msg("Unreleased reference") __naked
naked_bpf_prog!(pub fn potential_reference_on_stack_2, concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r4 = r10;\n",
    "r4 += -8;\n",
    "*(u64*)(r4 + 0) = r0;\n",
    "r0 = 0;\n",
    "r1 = 0;\n",
    "*(u64*)(r4 + 0) = r1;\n",
    "exit;\n",
));

// SEC("tc") __description("reference tracking: zero potential reference")
// __failure __msg("Unreleased reference") __naked
naked_bpf_prog!(pub fn reference_tracking_zero_potential_reference, concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r0 = 0; /* leak reference */\n",
    "exit;\n",
));

// SEC("tc") __description("reference tracking: zero potential reference to sock_common")
// __failure __msg("Unreleased reference") __naked
naked_bpf_prog!(pub fn potential_reference_to_sock_common_2, concat!(
    BPF_SK_LOOKUP!("bpf_skc_lookup_tcp"),
    "r0 = 0; /* leak reference */\n",
    "exit;\n",
));

// SEC("tc") __description("reference tracking: copy and zero potential references")
// __failure __msg("Unreleased reference") __naked
naked_bpf_prog!(pub fn copy_and_zero_potential_references, concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r7 = r0;\n",
    "r0 = 0;\n",
    "r7 = 0; /* leak reference */\n",
    "exit;\n",
));

macro_rules! bpf_asm_prog {
    ($name:ident, $section:literal, $description:literal, $expect:literal, $asm:expr) => {
        #[doc = concat!("SEC(\"", $section, "\")")]
        #[doc = concat!("__description(\"", $description, "\")")]
        #[doc = $expect]
        pub unsafe extern "C" fn $name() {
            core::arch::asm!($asm, options(noreturn));
        }
    };
}

bpf_asm_prog!(acquire_release_user_key_reference, "lsm.s/bpf", "reference tracking: acquire/release user key reference", "__success __naked", concat!(
    "r1 = -3;\n",
    "r2 = 0;\n",
    "call %[bpf_lookup_user_key];\n",
    "if r0 == 0 goto l0_%=;\n",
    "r1 = r0;\n",
    "call %[bpf_key_put];\n",
    "l0_%=: r0 = 0;\n",
    "exit;\n",
));

bpf_asm_prog!(acquire_release_system_key_reference, "lsm.s/bpf", "reference tracking: acquire/release system key reference", "__success __naked", concat!(
    "r1 = 1;\n",
    "call %[bpf_lookup_system_key];\n",
    "if r0 == 0 goto l0_%=;\n",
    "r1 = r0;\n",
    "call %[bpf_key_put];\n",
    "l0_%=: r0 = 0;\n",
    "exit;\n",
));

bpf_asm_prog!(user_key_reference_without_check, "lsm.s/bpf", "reference tracking: release user key reference without check", "__failure __msg(\"Possibly NULL pointer passed to trusted R1\") __naked", concat!(
    "r1 = -3;\n",
    "r2 = 0;\n",
    "call %[bpf_lookup_user_key];\n",
    "r1 = r0;\n",
    "call %[bpf_key_put];\n",
    "r0 = 0;\n",
    "exit;\n",
));

bpf_asm_prog!(system_key_reference_without_check, "lsm.s/bpf", "reference tracking: release system key reference without check", "__failure __msg(\"Possibly NULL pointer passed to trusted R1\") __naked", concat!(
    "r1 = 1;\n",
    "call %[bpf_lookup_system_key];\n",
    "r1 = r0;\n",
    "call %[bpf_key_put];\n",
    "r0 = 0;\n",
    "exit;\n",
));

bpf_asm_prog!(release_with_null_key_pointer, "lsm.s/bpf", "reference tracking: release with NULL key pointer", "__failure __msg(\"Possibly NULL pointer passed to trusted R1\") __naked", concat!(
    "r1 = 0;\n",
    "call %[bpf_key_put];\n",
    "r0 = 0;\n",
    "exit;\n",
));

bpf_asm_prog!(potential_reference_to_user_key, "lsm.s/bpf", "reference tracking: leak potential reference to user key", "__failure __msg(\"Unreleased reference\") __naked", concat!(
    "r1 = -3;\n",
    "r2 = 0;\n",
    "call %[bpf_lookup_user_key];\n",
    "exit;\n",
));

bpf_asm_prog!(potential_reference_to_system_key, "lsm.s/bpf", "reference tracking: leak potential reference to system key", "__failure __msg(\"Unreleased reference\") __naked", concat!(
    "r1 = 1;\n",
    "call %[bpf_lookup_system_key];\n",
    "exit;\n",
));

macro_rules! tc_prog {
    ($name:ident, $description:literal, $expect:literal, $asm:expr) => {
        bpf_asm_prog!($name, "tc", $description, $expect, $asm);
    };
}

tc_prog!(tracking_release_reference_without_check, "reference tracking: release reference without check", "__failure __msg(\"type=sock_or_null expected=sock\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "/* reference in r0 may be NULL */\n",
    "r1 = r0;\n",
    "r2 = 0;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(to_sock_common_without_check, "reference tracking: release reference to sock_common without check", "__failure __msg(\"type=sock_common_or_null expected=sock\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_skc_lookup_tcp"),
    "/* reference in r0 may be NULL */\n",
    "r1 = r0;\n",
    "r2 = 0;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(reference_tracking_release_reference, "reference tracking: release reference", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(release_reference_to_sock_common, "reference tracking: release reference to sock_common", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_skc_lookup_tcp"),
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(reference_tracking_release_reference_2, "reference tracking: release reference 2", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(reference_tracking_release_reference_twice, "reference tracking: release reference twice", "__failure __msg(\"type=scalar expected=sock\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "r6 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(release_reference_twice_inside_branch, "reference tracking: release reference twice inside branch", "__failure __msg(\"type=scalar expected=sock\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "r6 = r0;\n",
    "if r0 == 0 goto l0_%=; /* goto end */\n",
    "call %[bpf_sk_release];\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(check_free_in_one_subbranch, "reference tracking: alloc, check, free in one subbranch", "__failure __msg(\"Unreleased reference\") __flag(BPF_F_ANY_ALIGNMENT) __naked", concat!(
    "r2 = *(u32*)(r1 + %[__sk_buff_data]);\n",
    "r3 = *(u32*)(r1 + %[__sk_buff_data_end]);\n",
    "r0 = r2;\n",
    "r0 += 16;\n",
    "/* if (offsetof(skb, mark) > data_len) exit; */\n",
    "if r0 <= r3 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = *(u32*)(r2 + %[__sk_buff_mark]);\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r6 == 0 goto l1_%=; /* mark == 0? */\n",
    "/* Leak reference in R0 */\n",
    "exit;\n",
    "l1_%=: if r0 == 0 goto l2_%=; /* sk NULL? */\n",
    "r1 = r0;\n",
    "call %[bpf_sk_release];\n",
    "l2_%=: exit;\n",
));

tc_prog!(check_free_in_both_subbranches, "reference tracking: alloc, check, free in both subbranches", "__success __retval(0) __flag(BPF_F_ANY_ALIGNMENT) __naked", concat!(
    "r2 = *(u32*)(r1 + %[__sk_buff_data]);\n",
    "r3 = *(u32*)(r1 + %[__sk_buff_data_end]);\n",
    "r0 = r2;\n",
    "r0 += 16;\n",
    "/* if (offsetof(skb, mark) > data_len) exit; */\n",
    "if r0 <= r3 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = *(u32*)(r2 + %[__sk_buff_mark]);\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r6 == 0 goto l1_%=; /* mark == 0? */\n",
    "if r0 == 0 goto l2_%=; /* sk NULL? */\n",
    "r1 = r0;\n",
    "call %[bpf_sk_release];\n",
    "l2_%=: exit;\n",
    "l1_%=: if r0 == 0 goto l3_%=; /* sk NULL? */\n",
    "r1 = r0;\n",
    "call %[bpf_sk_release];\n",
    "l3_%=: exit;\n",
));

tc_prog!(call_free_reference_in_subprog, "reference tracking in call: free reference in subprog", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0; /* unchecked reference */\n",
    "call call_free_reference_in_subprog__1;\n",
    "r0 = 0;\n",
    "exit;\n",
));

naked_bpf_prog!(fn call_free_reference_in_subprog__1, concat!(
    "/* subprog 1 */\n",
    "r2 = r1;\n",
    "if r2 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(reference_in_subprog_and_outside, "reference tracking in call: free reference in subprog and outside", "__failure __msg(\"type=scalar expected=sock\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0; /* unchecked reference */\n",
    "r6 = r0;\n",
    "call reference_in_subprog_and_outside__1;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
));

naked_bpf_prog!(fn reference_in_subprog_and_outside__1, concat!(
    "/* subprog 1 */\n",
    "r2 = r1;\n",
    "if r2 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(alloc_leak_reference_in_subprog, "reference tracking in call: alloc & leak reference in subprog", "__failure __msg(\"Unreleased reference\") __naked", concat!(
    "r4 = r10;\n",
    "r4 += -8;\n",
    "call alloc_leak_reference_in_subprog__1;\n",
    "r1 = r0;\n",
    "r0 = 0;\n",
    "exit;\n",
));

naked_bpf_prog!(fn alloc_leak_reference_in_subprog__1, concat!(
    "/* subprog 1 */\n",
    "r6 = r4;\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "/* spill unchecked sk_ptr into stack of caller */\n",
    "*(u64*)(r6 + 0) = r0;\n",
    "r1 = r0;\n",
    "exit;\n",
));

tc_prog!(alloc_in_subprog_release_outside, "reference tracking in call: alloc in subprog, release outside", "__success __retval(POINTER_VALUE) __naked", concat!(
    "r4 = r10;\n",
    "call alloc_in_subprog_release_outside__1;\n",
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

naked_bpf_prog!(fn alloc_in_subprog_release_outside__1, concat!(
    "/* subprog 1 */\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "exit; /* return sk */\n",
));

tc_prog!(ptr_leak_into_caller_stack, "reference tracking in call: sk_ptr leak into caller stack", "__failure __msg(\"Unreleased reference\") __naked", concat!(
    "r4 = r10;\n",
    "r4 += -8;\n",
    "call ptr_leak_into_caller_stack__1;\n",
    "r0 = 0;\n",
    "exit;\n",
));

naked_bpf_prog!(fn ptr_leak_into_caller_stack__1, concat!(
    "/* subprog 1 */\n",
    "r5 = r10;\n",
    "r5 += -8;\n",
    "*(u64*)(r5 + 0) = r4;\n",
    "call ptr_leak_into_caller_stack__2;\n",
    "/* spill unchecked sk_ptr into stack of caller */\n",
    "r5 = r10;\n",
    "r5 += -8;\n",
    "r4 = *(u64*)(r5 + 0);\n",
    "*(u64*)(r4 + 0) = r0;\n",
    "exit;\n",
));

naked_bpf_prog!(fn ptr_leak_into_caller_stack__2, concat!(
    "/* subprog 2 */\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "exit;\n",
));

tc_prog!(ptr_spill_into_caller_stack, "reference tracking in call: sk_ptr spill into caller stack", "__success __retval(0) __naked", concat!(
    "r4 = r10;\n",
    "r4 += -8;\n",
    "call ptr_spill_into_caller_stack__1;\n",
    "r0 = 0;\n",
    "exit;\n",
));

naked_bpf_prog!(fn ptr_spill_into_caller_stack__1, concat!(
    "/* subprog 1 */\n",
    "r5 = r10;\n",
    "r5 += -8;\n",
    "*(u64*)(r5 + 0) = r4;\n",
    "call ptr_spill_into_caller_stack__2;\n",
    "/* spill unchecked sk_ptr into stack of caller */\n",
    "r5 = r10;\n",
    "r5 += -8;\n",
    "r4 = *(u64*)(r5 + 0);\n",
    "*(u64*)(r4 + 0) = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "/* now the sk_ptr is verified, free the reference */\n",
    "r1 = *(u64*)(r4 + 0);\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

naked_bpf_prog!(fn ptr_spill_into_caller_stack__2, concat!(
    "/* subprog 2 */\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "exit;\n",
));

tc_prog!(reference_tracking_allow_ld_abs, "reference tracking: allow LD_ABS", "__success __retval(0) __naked", concat!(
    "r6 = r1;\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: r0 = *(u8*)skb[0];\n",
    "r0 = *(u16*)skb[0];\n",
    "r0 = *(u32*)skb[0];\n",
    "exit;\n",
));

tc_prog!(ld_abs_while_holding_reference, "reference tracking: forbid LD_ABS while holding reference", "__failure __msg(\"BPF_LD_[ABS|IND] would lead to reference leak\") __naked", concat!(
    "r6 = r1;\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r0 = *(u8*)skb[0];\n",
    "r0 = *(u16*)skb[0];\n",
    "r0 = *(u32*)skb[0];\n",
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(reference_tracking_allow_ld_ind, "reference tracking: allow LD_IND", "__success __retval(1) __naked", concat!(
    "r6 = r1;\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: r7 = 1;\n",
    ".8byte %[ld_ind];\n",
    "r0 = r7;\n",
    "exit;\n",
));

tc_prog!(ld_ind_while_holding_reference, "reference tracking: forbid LD_IND while holding reference", "__failure __msg(\"BPF_LD_[ABS|IND] would lead to reference leak\") __naked", concat!(
    "r6 = r1;\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r4 = r0;\n",
    "r7 = 1;\n",
    ".8byte %[ld_ind];\n",
    "r0 = r7;\n",
    "r1 = r4;\n",
    "if r1 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(check_reference_or_tail_call, "reference tracking: check reference or tail call", "__success __retval(0) __naked", concat!(
    "r7 = r1;\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "/* if (sk) bpf_sk_release() */\n",
    "r1 = r0;\n",
    "if r1 != 0 goto l0_%=;\n",
    "/* bpf_tail_call() */\n",
    "r3 = 3;\n",
    "r2 = %[map_prog1_tc] ll;\n",
    "r1 = r7;\n",
    "call %[bpf_tail_call];\n",
    "r0 = 0;\n",
    "exit;\n",
    "l0_%=: call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(release_reference_then_tail_call, "reference tracking: release reference then tail call", "__success __retval(0) __naked", concat!(
    "r7 = r1;\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "/* if (sk) bpf_sk_release() */\n",
    "r1 = r0;\n",
    "if r1 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: /* bpf_tail_call() */\n",
    "r3 = 3;\n",
    "r2 = %[map_prog1_tc] ll;\n",
    "r1 = r7;\n",
    "call %[bpf_tail_call];\n",
    "r0 = 0;\n",
    "exit;\n",
));

tc_prog!(possible_reference_over_tail_call, "reference tracking: leak possible reference over tail call", "__failure __msg(\"tail_call would lead to reference leak\") __naked", concat!(
    "r7 = r1;\n",
    "/* Look up socket and store in REG_6 */\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "/* bpf_tail_call() */\n",
    "r6 = r0;\n",
    "r3 = 3;\n",
    "r2 = %[map_prog1_tc] ll;\n",
    "r1 = r7;\n",
    "call %[bpf_tail_call];\n",
    "r0 = 0;\n",
    "/* if (sk) bpf_sk_release() */\n",
    "r1 = r6;\n",
    "if r1 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(checked_reference_over_tail_call, "reference tracking: leak checked reference over tail call", "__failure __msg(\"tail_call would lead to reference leak\") __naked", concat!(
    "r7 = r1;\n",
    "/* Look up socket and store in REG_6 */\n",
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r6 = r0;\n",
    "/* if (!sk) goto end */\n",
    "if r0 == 0 goto l0_%=;\n",
    "/* bpf_tail_call() */\n",
    "r3 = 0;\n",
    "r2 = %[map_prog1_tc] ll;\n",
    "r1 = r7;\n",
    "call %[bpf_tail_call];\n",
    "r0 = 0;\n",
    "r1 = r6;\n",
    "l0_%=: call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(and_release_sock_or_null, "reference tracking: mangle and release sock_or_null", "__failure __msg(\"R1 pointer arithmetic on sock_or_null prohibited\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "r1 += 5;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(tracking_mangle_and_release_sock, "reference tracking: mangle and release sock", "__failure __msg(\"R1 pointer arithmetic on sock prohibited\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "r1 += 5;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(reference_tracking_access_member, "reference tracking: access member", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r6 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "r2 = *(u32*)(r0 + 4);\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(reference_tracking_write_to_member, "reference tracking: write to member", "__failure __msg(\"cannot write into sock\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r6 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "r1 = r6;\n",
    "r2 = 42 ll;\n",
    "*(u32*)(r1 + %[bpf_sock_mark]) = r2;\n",
    "r1 = r6;\n",
    "l0_%=: call %[bpf_sk_release];\n",
    "r0 = 0 ll;\n",
    "exit;\n",
));

tc_prog!(_64_bit_access_of_member, "reference tracking: invalid 64-bit access of member", "__failure __msg(\"invalid sock access off=0 size=8\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r6 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "r2 = *(u64*)(r0 + 0);\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(reference_tracking_access_after_release, "reference tracking: access after release", "__failure __msg(\"!read_ok\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r1 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "call %[bpf_sk_release];\n",
    "r2 = *(u32*)(r1 + 0);\n",
    "l0_%=: exit;\n",
));

tc_prog!(tracking_direct_access_for_lookup, "reference tracking: direct access for lookup", "__success __retval(0) __naked", concat!(
    "/* Check that the packet is at least 64B long */\n",
    "r2 = *(u32*)(r1 + %[__sk_buff_data]);\n",
    "r3 = *(u32*)(r1 + %[__sk_buff_data_end]);\n",
    "r0 = r2;\n",
    "r0 += 64;\n",
    "if r0 > r3 goto l0_%=;\n",
    "/* sk = sk_lookup_tcp(ctx, skb->data, ...) */\n",
    "r3 = %[sizeof_bpf_sock_tuple];\n",
    "r4 = 0;\n",
    "r5 = 0;\n",
    "call %[bpf_sk_lookup_tcp];\n",
    "r6 = r0;\n",
    "if r0 == 0 goto l0_%=;\n",
    "r2 = *(u32*)(r0 + 4);\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(bpf_tcp_sock_after_release, "reference tracking: use ptr from bpf_tcp_sock() after release", "__failure __msg(\"invalid mem access\") __flag(BPF_F_ANY_ALIGNMENT) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_tcp_sock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r7 = r0;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "r0 = *(u32*)(r7 + %[bpf_tcp_sock_snd_cwnd]);\n",
    "exit;\n",
));

tc_prog!(bpf_sk_fullsock_after_release, "reference tracking: use ptr from bpf_sk_fullsock() after release", "__failure __msg(\"invalid mem access\") __flag(BPF_F_ANY_ALIGNMENT) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_sk_fullsock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r7 = r0;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "r0 = *(u32*)(r7 + %[bpf_sock_type]);\n",
    "exit;\n",
));

tc_prog!(sk_fullsock_tp_after_release, "reference tracking: use ptr from bpf_sk_fullsock(tp) after release", "__failure __msg(\"invalid mem access\") __flag(BPF_F_ANY_ALIGNMENT) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_tcp_sock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r1 = r0;\n",
    "call %[bpf_sk_fullsock];\n",
    "r1 = r6;\n",
    "r6 = r0;\n",
    "call %[bpf_sk_release];\n",
    "if r6 != 0 goto l2_%=;\n",
    "exit;\n",
    "l2_%=: r0 = *(u32*)(r6 + %[bpf_sock_type]);\n",
    "exit;\n",
));

tc_prog!(after_bpf_sk_release_tp, "reference tracking: use sk after bpf_sk_release(tp)", "__failure __msg(\"invalid mem access\") __flag(BPF_F_ANY_ALIGNMENT) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_tcp_sock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r1 = r0;\n",
    "call %[bpf_sk_release];\n",
    "r0 = *(u32*)(r6 + %[bpf_sock_type]);\n",
    "exit;\n",
));

tc_prog!(after_bpf_sk_release_sk, "reference tracking: use ptr from bpf_get_listener_sock() after bpf_sk_release(sk)", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_get_listener_sock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r1 = r6;\n",
    "r6 = r0;\n",
    "call %[bpf_sk_release];\n",
    "r0 = *(u32*)(r6 + %[bpf_sock_src_port]);\n",
    "exit;\n",
));

tc_prog!(bpf_sk_release_listen_sk, "reference tracking: bpf_sk_release(listen_sk)", "__failure __msg(\"release helper bpf_sk_release expects referenced PTR_TO_BTF_ID passed to R1\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_get_listener_sock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r1 = r0;\n",
    "call %[bpf_sk_release];\n",
    "r0 = *(u32*)(r6 + %[bpf_sock_type]);\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
));

/* !bpf_sk_fullsock(sk) is checked but !bpf_tcp_sock(sk) is not checked */
tc_prog!(and_bpf_tcp_sock_sk, "reference tracking: tp->snd_cwnd after bpf_sk_fullsock(sk) and bpf_tcp_sock(sk)", "__failure __msg(\"invalid mem access\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_sk_fullsock];\n",
    "r7 = r0;\n",
    "r1 = r6;\n",
    "call %[bpf_tcp_sock];\n",
    "r8 = r0;\n",
    "if r7 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r0 = *(u32*)(r8 + %[bpf_tcp_sock_snd_cwnd]);\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(tracking_valid_pointer_null_comparison, "reference tracking: branch tracking valid pointer null comparison", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r6 = r0;\n",
    "r3 = 1;\n",
    "if r6 != 0 goto l0_%=;\n",
    "r3 = 0;\n",
    "l0_%=: if r6 == 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "l1_%=: exit;\n",
));

tc_prog!(tracking_valid_pointer_value_comparison, "reference tracking: branch tracking valid pointer value comparison", "__failure __msg(\"Unreleased reference\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "r6 = r0;\n",
    "r3 = 1;\n",
    "if r6 == 0 goto l0_%=;\n",
    "r3 = 0;\n",
    "if r6 == 1234 goto l0_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "l0_%=: exit;\n",
));

tc_prog!(sk_release_btf_tcp_sock, "reference tracking: bpf_sk_release(btf_tcp_sock)", "__success __retval(0) __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_skc_to_tcp_sock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r1 = r0;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
));

tc_prog!(to_tcp_sock_after_release, "reference tracking: use ptr from bpf_skc_to_tcp_sock() after release", "__failure __msg(\"invalid mem access\") __naked", concat!(
    BPF_SK_LOOKUP!("bpf_sk_lookup_tcp"),
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r6 = r0;\n",
    "r1 = r0;\n",
    "call %[bpf_skc_to_tcp_sock];\n",
    "if r0 != 0 goto l1_%=;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "exit;\n",
    "l1_%=: r7 = r0;\n",
    "r1 = r6;\n",
    "call %[bpf_sk_release];\n",
    "r0 = *(u8*)(r7 + 0);\n",
    "exit;\n",
));

bpf_asm_prog!(to_leak_released_ptr_reg, "socket", "reference tracking: try to leak released ptr reg", "__success __failure_unpriv __msg_unpriv(\"R8 !read_ok\") __retval(0) __naked", concat!(
    "r0 = 0;\n",
    "*(u32*)(r10 - 4) = r0;\n",
    "r2 = r10;\n",
    "r2 += -4;\n",
    "r1 = %[map_array_48b] ll;\n",
    "call %[bpf_map_lookup_elem];\n",
    "if r0 != 0 goto l0_%=;\n",
    "exit;\n",
    "l0_%=: r9 = r0;\n",
    "r0 = 0;\n",
    "r1 = %[map_ringbuf] ll;\n",
    "r2 = 8;\n",
    "r3 = 0;\n",
    "call %[bpf_ringbuf_reserve];\n",
    "if r0 != 0 goto l1_%=;\n",
    "exit;\n",
    "l1_%=: r8 = r0;\n",
    "r1 = r8;\n",
    "r2 = 0;\n",
    "call %[bpf_ringbuf_discard];\n",
    "r0 = 0;\n",
    "*(u64*)(r9 + 0) = r8;\n",
    "exit;\n",
));

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
