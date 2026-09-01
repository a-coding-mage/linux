/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * C header guard removed in Rust translation.
 *
 * Original __ASSEMBLER__ branch:
 *
 *     .irp    num,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30
 *     .equ    .L__gpr_num_x\num, \num
 *     .equ    .L__gpr_num_w\num, \num
 *     .endr
 *     .equ    .L__gpr_num_xzr, 31
 *     .equ    .L__gpr_num_wzr, 31
 */

pub const __DEFINE_ASM_GPR_NUMS: &str =
    "\t.irp\tnum,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30\n\
\t.equ\t.L__gpr_num_x\\num, \\num\n\
\t.equ\t.L__gpr_num_w\\num, \\num\n\
\t.endr\n\
\t.equ\t.L__gpr_num_xzr, 31\n\
\t.equ\t.L__gpr_num_wzr, 31\n";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
