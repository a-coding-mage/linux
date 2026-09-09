/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The original header provides assembler-only local register-number symbols.
 * Rust has no direct equivalent for assembler .equ/.irp directives, so the
 * expanded values are represented as constants here.
 */

pub const L__GPR_NUM_ZERO: i32 = 0;

pub const L__GPR_NUM_R1: i32 = 1;
pub const L__GPR_NUM_R2: i32 = 2;
pub const L__GPR_NUM_R3: i32 = 3;
pub const L__GPR_NUM_R4: i32 = 4;
pub const L__GPR_NUM_R5: i32 = 5;
pub const L__GPR_NUM_R6: i32 = 6;
pub const L__GPR_NUM_R7: i32 = 7;
pub const L__GPR_NUM_R8: i32 = 8;
pub const L__GPR_NUM_R9: i32 = 9;
pub const L__GPR_NUM_R10: i32 = 10;
pub const L__GPR_NUM_R11: i32 = 11;
pub const L__GPR_NUM_R12: i32 = 12;
pub const L__GPR_NUM_R13: i32 = 13;
pub const L__GPR_NUM_R14: i32 = 14;
pub const L__GPR_NUM_R15: i32 = 15;
pub const L__GPR_NUM_R16: i32 = 16;
pub const L__GPR_NUM_R17: i32 = 17;
pub const L__GPR_NUM_R18: i32 = 18;
pub const L__GPR_NUM_R19: i32 = 19;
pub const L__GPR_NUM_R20: i32 = 20;
pub const L__GPR_NUM_R21: i32 = 21;
pub const L__GPR_NUM_R22: i32 = 22;
pub const L__GPR_NUM_R23: i32 = 23;
pub const L__GPR_NUM_R24: i32 = 24;
pub const L__GPR_NUM_R25: i32 = 25;
pub const L__GPR_NUM_R26: i32 = 26;
pub const L__GPR_NUM_R27: i32 = 27;
pub const L__GPR_NUM_R28: i32 = 28;
pub const L__GPR_NUM_R29: i32 = 29;
pub const L__GPR_NUM_R30: i32 = 30;
pub const L__GPR_NUM_R31: i32 = 31;

/* ABI names of registers */
pub const L__GPR_NUM_RA: i32 = 1;
pub const L__GPR_NUM_TP: i32 = 2;
pub const L__GPR_NUM_SP: i32 = 3;

pub const L__GPR_NUM_A0: i32 = 4;
pub const L__GPR_NUM_A1: i32 = 5;
pub const L__GPR_NUM_A2: i32 = 6;
pub const L__GPR_NUM_A3: i32 = 7;
pub const L__GPR_NUM_A4: i32 = 8;
pub const L__GPR_NUM_A5: i32 = 9;
pub const L__GPR_NUM_A6: i32 = 10;
pub const L__GPR_NUM_A7: i32 = 11;

pub const L__GPR_NUM_T0: i32 = 12;
pub const L__GPR_NUM_T1: i32 = 13;
pub const L__GPR_NUM_T2: i32 = 14;
pub const L__GPR_NUM_T3: i32 = 15;
pub const L__GPR_NUM_T4: i32 = 16;
pub const L__GPR_NUM_T5: i32 = 17;
pub const L__GPR_NUM_T6: i32 = 18;
pub const L__GPR_NUM_T7: i32 = 19;
pub const L__GPR_NUM_T8: i32 = 20;

pub const L__GPR_NUM_S9: i32 = 22;
pub const L__GPR_NUM_FP: i32 = 22;

pub const L__GPR_NUM_S0: i32 = 23;
pub const L__GPR_NUM_S1: i32 = 24;
pub const L__GPR_NUM_S2: i32 = 25;
pub const L__GPR_NUM_S3: i32 = 26;
pub const L__GPR_NUM_S4: i32 = 27;
pub const L__GPR_NUM_S5: i32 = 28;
pub const L__GPR_NUM_S6: i32 = 29;
pub const L__GPR_NUM_S7: i32 = 30;
pub const L__GPR_NUM_S8: i32 = 31;

/*
 * Non-assembler form of the original __DEFINE_ASM_GPR_NUMS macro.  It is
 * preserved as the exact assembler source emitted by that macro.
 */
pub const __DEFINE_ASM_GPR_NUMS: &str = concat!(
    "\t.equ\t.L__gpr_num_zero, 0\n",
    "\t.irp\tnum,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31\n",
    "\t.equ\t.L__gpr_num_$r\\num, \\num\n",
    "\t.endr\n",
    "\t.equ\t.L__gpr_num_$ra, 1\n",
    "\t.equ\t.L__gpr_num_$tp, 2\n",
    "\t.equ\t.L__gpr_num_$sp, 3\n",
    "\t.irp\tnum,0,1,2,3,4,5,6,7\n",
    "\t.equ\t.L__gpr_num_$a\\num, 4 + \\num\n",
    "\t.endr\n",
    "\t.irp\tnum,0,1,2,3,4,5,6,7,8\n",
    "\t.equ\t.L__gpr_num_$t\\num, 12 + \\num\n",
    "\t.endr\n",
    "\t.equ\t.L__gpr_num_$s9, 22\n",
    "\t.equ\t.L__gpr_num_$fp, 22\n",
    "\t.irp\tnum,0,1,2,3,4,5,6,7,8\n",
    "\t.equ\t.L__gpr_num_$s\\num, 23 + \\num\n",
    "\t.endr\n",
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
