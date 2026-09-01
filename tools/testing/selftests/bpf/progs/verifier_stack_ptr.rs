// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/stack_ptr.c */

// C includes removed in Rust translation:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <limits.h>, "bpf_misc.h"

pub const MAX_ENTRIES: usize = 11;
pub const SHRT_MAX: i32 = i16::MAX as i32;
pub const SHRT_MIN: i32 = i16::MIN as i32;
pub const INT_MIN: i32 = i32::MIN;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

// Original C map declaration used BPF helper macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct test_val);
// } map_array_48b SEC(".maps");
#[repr(C)]
pub struct map_array_48b_def {
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static map_array_48b: map_array_48b_def = map_array_48b_def { _private: [] };

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

// SEC("socket")
// __description("PTR_TO_STACK store/load")
// __success __success_unpriv __retval(0xfaceb00c)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_stack_store_load() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -10",
        "r0 = 0xfaceb00c",
        "*(u64*)(r1 + 2) = r0",
        "r0 = *(u64*)(r1 + 2)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK store/load - bad alignment on off")
// __failure __msg("misaligned stack access off -8+2 size 8")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn load_bad_alignment_on_off() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -8",
        "r0 = 0xfaceb00c",
        "*(u64*)(r1 + 2) = r0",
        "r0 = *(u64*)(r1 + 2)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK store/load - bad alignment on reg")
// __failure __msg("misaligned stack access off -10+8 size 8")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn load_bad_alignment_on_reg() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -10",
        "r0 = 0xfaceb00c",
        "*(u64*)(r1 + 8) = r0",
        "r0 = *(u64*)(r1 + 8)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK store/load - out of bounds low")
// __failure __msg("invalid write to stack R1 off=-79992 size=8")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn load_out_of_bounds_low() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -80000",
        "r0 = 0xfaceb00c",
        "*(u64*)(r1 + 8) = r0",
        "r0 = *(u64*)(r1 + 8)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK store/load - out of bounds high")
// __failure __msg("invalid write to stack R1 off=0 size=8")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn load_out_of_bounds_high() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -8",
        "r0 = 0xfaceb00c",
        "*(u64*)(r1 + 8) = r0",
        "r0 = *(u64*)(r1 + 8)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check high 1")
// __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_1() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -1",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check high 2")
// __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_2() {
    core::arch::asm!(
        "r1 = r10",
        "r0 = 42",
        "*(u8*)(r1 - 1) = r0",
        "r0 = *(u8*)(r1 - 1)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check high 3")
// __success __failure_unpriv
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
// __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_3() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += 0",
        "r0 = 42",
        "*(u8*)(r1 - 1) = r0",
        "r0 = *(u8*)(r1 - 1)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check high 4")
// __failure __msg("invalid write to stack R1 off=0 size=1")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_4() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += 0",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check high 5")
// __failure __msg("invalid write to stack R1")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_5() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += {imm_0}",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        imm_0 = const (1 << 29) - 1,
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check high 6")
// __failure __msg("invalid write to stack")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_6() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += {imm_0}",
        "r0 = 42",
        "*(u8*)(r1 + {shrt_max}) = r0",
        "r0 = *(u8*)(r1 + {shrt_max})",
        "exit",
        imm_0 = const (1 << 29) - 1,
        shrt_max = const SHRT_MAX,
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check high 7")
// __failure __msg("fp pointer offset")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_high_7() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += {imm_0}",
        "r1 += {imm_0}",
        "r0 = 42",
        "*(u8*)(r1 + {shrt_max}) = r0",
        "r0 = *(u8*)(r1 + {shrt_max})",
        "exit",
        imm_0 = const (1 << 29) - 1,
        shrt_max = const SHRT_MAX,
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check low 1")
// __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_1() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -512",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check low 2")
// __success __failure_unpriv
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
// __retval(42)
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_2() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -513",
        "r0 = 42",
        "*(u8*)(r1 + 1) = r0",
        "r0 = *(u8*)(r1 + 1)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check low 3")
// __failure __msg("invalid write to stack R1 off=-513 size=1")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_3() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -513",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check low 4")
// __failure __msg("math between fp pointer")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_4() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += {int_min}",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        int_min = const INT_MIN,
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check low 5")
// __failure __msg("invalid write to stack")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_5() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += {imm_0}",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        imm_0 = const -((1 << 29) - 1),
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check low 6")
// __failure __msg("invalid write to stack")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_6() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += {imm_0}",
        "r0 = 42",
        "*(u8*)(r1  {shrt_min}) = r0",
        "r0 = *(u8*)(r1  {shrt_min})",
        "exit",
        imm_0 = const -((1 << 29) - 1),
        shrt_min = const SHRT_MIN,
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK check low 7")
// __failure __msg("fp pointer offset")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn to_stack_check_low_7() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += {imm_0}",
        "r1 += {imm_0}",
        "r0 = 42",
        "*(u8*)(r1  {shrt_min}) = r0",
        "r0 = *(u8*)(r1  {shrt_min})",
        "exit",
        imm_0 = const -((1 << 29) - 1),
        shrt_min = const SHRT_MIN,
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK mixed reg/k, 1")
// __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn stack_mixed_reg_k_1() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -3",
        "r2 = -3",
        "r1 += r2",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK mixed reg/k, 2")
// __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn stack_mixed_reg_k_2() {
    core::arch::asm!(
        "r0 = 0",
        "*(u64*)(r10 - 8) = r0",
        "r0 = 0",
        "*(u64*)(r10 - 16) = r0",
        "r1 = r10",
        "r1 += -3",
        "r2 = -3",
        "r1 += r2",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r5 = r10",
        "r0 = *(u8*)(r5 - 6)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK mixed reg/k, 3")
// __success __success_unpriv __retval(-3)
#[no_mangle]
pub unsafe extern "C" fn stack_mixed_reg_k_3() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -3",
        "r2 = -3",
        "r1 += r2",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = r2",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK reg")
// __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn ptr_to_stack_reg() {
    core::arch::asm!(
        "r1 = r10",
        "r2 = -3",
        "r1 += r2",
        "r0 = 42",
        "*(u8*)(r1 + 0) = r0",
        "r0 = *(u8*)(r1 + 0)",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("stack pointer arithmetic")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stack_pointer_arithmetic() {
    core::arch::asm!(
        "r1 = 4",
        "goto l0_0",
        "l0_0: r7 = r10",
        "r7 += -10",
        "r7 += -10",
        "r2 = r7",
        "r2 += r1",
        "r0 = 0",
        "*(u32*)(r2 + 4) = r0",
        "r2 = r7",
        "r2 += 8",
        "r0 = 0",
        "*(u32*)(r2 + 4) = r0",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// SEC("tc")
// __description("store PTR_TO_STACK in R10 to array map using BPF_B")
// __success __retval(42)
#[no_mangle]
pub unsafe extern "C" fn array_map_using_bpf_b() {
    core::arch::asm!(
        "/* Load pointer to map. */",
        "r2 = r10",
        "r2 += -8",
        "r1 = 0",
        "*(u64*)(r2 + 0) = r1",
        "r1 = {map_array_48b} ll",
        "call {bpf_map_lookup_elem}",
        "if r0 != 0 goto l0_1",
        "r0 = 2",
        "exit",
        "l0_1: r1 = r0",
        "/* Copy R10 to R9. */",
        "r9 = r10",
        "/* Pollute other registers with unaligned values. */",
        "r2 = -1",
        "r3 = -1",
        "r4 = -1",
        "r5 = -1",
        "r6 = -1",
        "r7 = -1",
        "r8 = -1",
        "/* Store both R9 and R10 with BPF_B and read back. */",
        "*(u8*)(r1 + 0) = r10",
        "r2 = *(u8*)(r1 + 0)",
        "*(u8*)(r1 + 0) = r9",
        "r3 = *(u8*)(r1 + 0)",
        "/* Should read back as same value. */",
        "if r2 == r3 goto l1_1",
        "r0 = 1",
        "exit",
        "l1_1: r0 = 42",
        "exit",
        map_array_48b = sym map_array_48b,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK stack size > 512")
// __failure __msg("invalid write to stack R1 off=-520 size=8")
#[no_mangle]
pub unsafe extern "C" fn stack_check_size_gt_512() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -520",
        "r0 = 42",
        "*(u64*)(r1 + 0) = r0",
        "exit",
        options(noreturn)
    );
}

// Original C was guarded by: #ifdef __BPF_FEATURE_MAY_GOTO

// SEC("socket")
// __description("PTR_TO_STACK stack size 512 with may_goto with jit")
// __load_if_JITed()
// __success __retval(42)
#[cfg(__BPF_FEATURE_MAY_GOTO)]
#[no_mangle]
pub unsafe extern "C" fn stack_check_size_512_with_may_goto_jit() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -512",
        "r0 = 42",
        "*(u32*)(r1 + 0) = r0",
        "may_goto l0_2",
        "r2 = 100",
        "l0_2:",
        "exit",
        options(noreturn)
    );
}

// SEC("socket")
// __description("PTR_TO_STACK stack size 512 with may_goto without jit")
// __load_if_no_JITed()
// __failure __msg("stack size 520(extra 8) is too large")
#[cfg(__BPF_FEATURE_MAY_GOTO)]
#[no_mangle]
pub unsafe extern "C" fn stack_check_size_512_with_may_goto() {
    core::arch::asm!(
        "r1 = r10",
        "r1 += -512",
        "r0 = 42",
        "*(u32*)(r1 + 0) = r0",
        "may_goto l0_3",
        "r2 = 100",
        "l0_3:",
        "exit",
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
