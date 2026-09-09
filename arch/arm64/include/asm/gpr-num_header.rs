/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The original header emits assembler-local symbols when __ASSEMBLER__ is
 * defined.  Rust identifiers cannot contain the assembler symbol's leading
 * dot, so the corresponding numeric values are represented by constants.
 */
pub const __GPR_NUM_X0: u32 = 0;
pub const __GPR_NUM_W0: u32 = 0;
pub const __GPR_NUM_X1: u32 = 1;
pub const __GPR_NUM_W1: u32 = 1;
pub const __GPR_NUM_X2: u32 = 2;
pub const __GPR_NUM_W2: u32 = 2;
pub const __GPR_NUM_X3: u32 = 3;
pub const __GPR_NUM_W3: u32 = 3;
pub const __GPR_NUM_X4: u32 = 4;
pub const __GPR_NUM_W4: u32 = 4;
pub const __GPR_NUM_X5: u32 = 5;
pub const __GPR_NUM_W5: u32 = 5;
pub const __GPR_NUM_X6: u32 = 6;
pub const __GPR_NUM_W6: u32 = 6;
pub const __GPR_NUM_X7: u32 = 7;
pub const __GPR_NUM_W7: u32 = 7;
pub const __GPR_NUM_X8: u32 = 8;
pub const __GPR_NUM_W8: u32 = 8;
pub const __GPR_NUM_X9: u32 = 9;
pub const __GPR_NUM_W9: u32 = 9;
pub const __GPR_NUM_X10: u32 = 10;
pub const __GPR_NUM_W10: u32 = 10;
pub const __GPR_NUM_X11: u32 = 11;
pub const __GPR_NUM_W11: u32 = 11;
pub const __GPR_NUM_X12: u32 = 12;
pub const __GPR_NUM_W12: u32 = 12;
pub const __GPR_NUM_X13: u32 = 13;
pub const __GPR_NUM_W13: u32 = 13;
pub const __GPR_NUM_X14: u32 = 14;
pub const __GPR_NUM_W14: u32 = 14;
pub const __GPR_NUM_X15: u32 = 15;
pub const __GPR_NUM_W15: u32 = 15;
pub const __GPR_NUM_X16: u32 = 16;
pub const __GPR_NUM_W16: u32 = 16;
pub const __GPR_NUM_X17: u32 = 17;
pub const __GPR_NUM_W17: u32 = 17;
pub const __GPR_NUM_X18: u32 = 18;
pub const __GPR_NUM_W18: u32 = 18;
pub const __GPR_NUM_X19: u32 = 19;
pub const __GPR_NUM_W19: u32 = 19;
pub const __GPR_NUM_X20: u32 = 20;
pub const __GPR_NUM_W20: u32 = 20;
pub const __GPR_NUM_X21: u32 = 21;
pub const __GPR_NUM_W21: u32 = 21;
pub const __GPR_NUM_X22: u32 = 22;
pub const __GPR_NUM_W22: u32 = 22;
pub const __GPR_NUM_X23: u32 = 23;
pub const __GPR_NUM_W23: u32 = 23;
pub const __GPR_NUM_X24: u32 = 24;
pub const __GPR_NUM_W24: u32 = 24;
pub const __GPR_NUM_X25: u32 = 25;
pub const __GPR_NUM_W25: u32 = 25;
pub const __GPR_NUM_X26: u32 = 26;
pub const __GPR_NUM_W26: u32 = 26;
pub const __GPR_NUM_X27: u32 = 27;
pub const __GPR_NUM_W27: u32 = 27;
pub const __GPR_NUM_X28: u32 = 28;
pub const __GPR_NUM_W28: u32 = 28;
pub const __GPR_NUM_X29: u32 = 29;
pub const __GPR_NUM_W29: u32 = 29;
pub const __GPR_NUM_X30: u32 = 30;
pub const __GPR_NUM_W30: u32 = 30;
pub const __GPR_NUM_XZR: u32 = 31;
pub const __GPR_NUM_WZR: u32 = 31;

/* Equivalent of the C preprocessor string macro __DEFINE_ASM_GPR_NUMS. */
pub const __DEFINE_ASM_GPR_NUMS: &str = "\
\t.irp\tnum,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30\n\
\t.equ\t.L__gpr_num_x\\num, \\num\n\
\t.equ\t.L__gpr_num_w\\num, \\num\n\
\t.endr\n\
\t.equ\t.L__gpr_num_xzr, 31\n\
\t.equ\t.L__gpr_num_wzr, 31\n";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
