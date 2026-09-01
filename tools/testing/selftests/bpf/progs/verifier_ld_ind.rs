// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/ld_ind.c */

// Dependencies from the original C source:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, ../../../include/linux/filter.h, "bpf_misc.h"

#[SEC("socket")]
#[__description("ld_ind: check calling conv, r1")]
#[__failure]
#[__msg("R1 !read_ok")]
#[__failure_unpriv]
#[naked]
pub unsafe extern "C" fn ind_check_calling_conv_r1() {
    core::arch::asm!(
        "r6 = r1;",
        "r1 = 1;",
        ".8byte {ld_ind}",
        "r0 = r1;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_1, -0x200000),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: check calling conv, r2")]
#[__failure]
#[__msg("R2 !read_ok")]
#[__failure_unpriv]
#[naked]
pub unsafe extern "C" fn ind_check_calling_conv_r2() {
    core::arch::asm!(
        "r6 = r1;",
        "r2 = 1;",
        ".8byte {ld_ind}",
        "r0 = r2;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_2, -0x200000),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: check calling conv, r3")]
#[__failure]
#[__msg("R3 !read_ok")]
#[__failure_unpriv]
#[naked]
pub unsafe extern "C" fn ind_check_calling_conv_r3() {
    core::arch::asm!(
        "r6 = r1;",
        "r3 = 1;",
        ".8byte {ld_ind}",
        "r0 = r3;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_3, -0x200000),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: check calling conv, r4")]
#[__failure]
#[__msg("R4 !read_ok")]
#[__failure_unpriv]
#[naked]
pub unsafe extern "C" fn ind_check_calling_conv_r4() {
    core::arch::asm!(
        "r6 = r1;",
        "r4 = 1;",
        ".8byte {ld_ind}",
        "r0 = r4;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_4, -0x200000),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: check calling conv, r5")]
#[__failure]
#[__msg("R5 !read_ok")]
#[__failure_unpriv]
#[naked]
pub unsafe extern "C" fn ind_check_calling_conv_r5() {
    core::arch::asm!(
        "r6 = r1;",
        "r5 = 1;",
        ".8byte {ld_ind}",
        "r0 = r5;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_5, -0x200000),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: check calling conv, r7")]
#[__success]
#[__success_unpriv]
#[__retval(1)]
#[naked]
pub unsafe extern "C" fn ind_check_calling_conv_r7() {
    core::arch::asm!(
        "r6 = r1;",
        "r7 = 1;",
        ".8byte {ld_ind}",
        "r0 = r7;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_7, -0x200000),
        options(noreturn)
    );
}

/*
 * ld_{abs,ind} subprog that always sets r0=1 on the success path.
 * bpf_gen_ld_abs() emits a hidden exit with r0=0 when the load helper
 * fails. The verifier must model this failure return so that callers
 * account for r0=0 as a possible return value.
 */
#[naked]
#[inline(never)]
#[used]
unsafe extern "C" fn ldabs_subprog() -> i32 {
    core::arch::asm!(
        "r6 = r1;",
        ".8byte {ld_abs}",
        "r0 = 1;",
        "exit;",
        ld_abs = const BPF_LD_ABS(BPF_W, 0),
        options(noreturn)
    );
}

#[naked]
#[inline(never)]
#[used]
unsafe extern "C" fn ldind_subprog() -> i32 {
    core::arch::asm!(
        "r6 = r1;",
        "r7 = 0;",
        ".8byte {ld_ind}",
        "r0 = 1;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_7, 0),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_abs: subprog early exit on ld_abs failure")]
#[__failure]
#[__msg("R9 !read_ok")]
#[naked]
pub unsafe extern "C" fn ld_abs_subprog_early_exit() {
    core::arch::asm!(
        "call ldabs_subprog;",
        "if r0 != 0 goto l_exit_0;",
        "r0 = r9;",
        "l_exit_0:",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: subprog early exit on ld_ind failure")]
#[__failure]
#[__msg("R9 !read_ok")]
#[naked]
pub unsafe extern "C" fn ld_ind_subprog_early_exit() {
    core::arch::asm!(
        "call ldind_subprog;",
        "if r0 != 0 goto l_exit_0;",
        "r0 = r9;",
        "l_exit_0:",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_abs: subprog with both paths safe")]
#[__success]
#[naked]
pub unsafe extern "C" fn ld_abs_subprog_both_paths_safe() {
    core::arch::asm!(
        "call ldabs_subprog;",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: subprog with both paths safe")]
#[__success]
#[naked]
pub unsafe extern "C" fn ld_ind_subprog_both_paths_safe() {
    core::arch::asm!(
        "call ldind_subprog;",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

/*
 * ld_{abs,ind} in subprogs require scalar (int) return type in BTF.
 * A test with void return must be rejected.
 */
#[naked]
#[inline(never)]
#[used]
unsafe extern "C" fn ldabs_void_subprog() {
    core::arch::asm!(
        "r6 = r1;",
        ".8byte {ld_abs}",
        "r0 = 1;",
        "exit;",
        ld_abs = const BPF_LD_ABS(BPF_W, 0),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_abs: reject void return subprog")]
#[__failure]
#[__msg("LD_ABS is only allowed in functions that return 'int'")]
#[naked]
pub unsafe extern "C" fn ld_abs_void_subprog_reject() {
    core::arch::asm!(
        "call ldabs_void_subprog;",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

#[naked]
#[inline(never)]
#[used]
unsafe extern "C" fn ldind_void_subprog() {
    core::arch::asm!(
        "r6 = r1;",
        "r7 = 0;",
        ".8byte {ld_ind}",
        "r0 = 1;",
        "exit;",
        ld_ind = const BPF_LD_IND(BPF_W, BPF_REG_7, 0),
        options(noreturn)
    );
}

#[SEC("socket")]
#[__description("ld_ind: reject void return subprog")]
#[__failure]
#[__msg("LD_ABS is only allowed in functions that return 'int'")]
#[naked]
pub unsafe extern "C" fn ld_ind_void_subprog_reject() {
    core::arch::asm!(
        "call ldind_void_subprog;",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

#[SEC("license")]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
