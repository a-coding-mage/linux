/* SPDX-License-Identifier: GPL-2.0-only */

// __ASSEMBLER__: the original header emits local assembler symbols for each
// architectural register number.  These Rust constants preserve those values.
pub const GPR_NUM_X0: u32 = 0;
pub const GPR_NUM_X1: u32 = 1;
pub const GPR_NUM_X2: u32 = 2;
pub const GPR_NUM_X3: u32 = 3;
pub const GPR_NUM_X4: u32 = 4;
pub const GPR_NUM_X5: u32 = 5;
pub const GPR_NUM_X6: u32 = 6;
pub const GPR_NUM_X7: u32 = 7;
pub const GPR_NUM_X8: u32 = 8;
pub const GPR_NUM_X9: u32 = 9;
pub const GPR_NUM_X10: u32 = 10;
pub const GPR_NUM_X11: u32 = 11;
pub const GPR_NUM_X12: u32 = 12;
pub const GPR_NUM_X13: u32 = 13;
pub const GPR_NUM_X14: u32 = 14;
pub const GPR_NUM_X15: u32 = 15;
pub const GPR_NUM_X16: u32 = 16;
pub const GPR_NUM_X17: u32 = 17;
pub const GPR_NUM_X18: u32 = 18;
pub const GPR_NUM_X19: u32 = 19;
pub const GPR_NUM_X20: u32 = 20;
pub const GPR_NUM_X21: u32 = 21;
pub const GPR_NUM_X22: u32 = 22;
pub const GPR_NUM_X23: u32 = 23;
pub const GPR_NUM_X24: u32 = 24;
pub const GPR_NUM_X25: u32 = 25;
pub const GPR_NUM_X26: u32 = 26;
pub const GPR_NUM_X27: u32 = 27;
pub const GPR_NUM_X28: u32 = 28;
pub const GPR_NUM_X29: u32 = 29;
pub const GPR_NUM_X30: u32 = 30;
pub const GPR_NUM_X31: u32 = 31;

pub const GPR_NUM_ZERO: u32 = 0;
pub const GPR_NUM_RA: u32 = 1;
pub const GPR_NUM_SP: u32 = 2;
pub const GPR_NUM_GP: u32 = 3;
pub const GPR_NUM_TP: u32 = 4;
pub const GPR_NUM_T0: u32 = 5;
pub const GPR_NUM_T1: u32 = 6;
pub const GPR_NUM_T2: u32 = 7;
pub const GPR_NUM_S0: u32 = 8;
pub const GPR_NUM_S1: u32 = 9;
pub const GPR_NUM_A0: u32 = 10;
pub const GPR_NUM_A1: u32 = 11;
pub const GPR_NUM_A2: u32 = 12;
pub const GPR_NUM_A3: u32 = 13;
pub const GPR_NUM_A4: u32 = 14;
pub const GPR_NUM_A5: u32 = 15;
pub const GPR_NUM_A6: u32 = 16;
pub const GPR_NUM_A7: u32 = 17;
pub const GPR_NUM_S2: u32 = 18;
pub const GPR_NUM_S3: u32 = 19;
pub const GPR_NUM_S4: u32 = 20;
pub const GPR_NUM_S5: u32 = 21;
pub const GPR_NUM_S6: u32 = 22;
pub const GPR_NUM_S7: u32 = 23;
pub const GPR_NUM_S8: u32 = 24;
pub const GPR_NUM_S9: u32 = 25;
pub const GPR_NUM_S10: u32 = 26;
pub const GPR_NUM_S11: u32 = 27;
pub const GPR_NUM_T3: u32 = 28;
pub const GPR_NUM_T4: u32 = 29;
pub const GPR_NUM_T5: u32 = 30;
pub const GPR_NUM_T6: u32 = 31;

// Non-assembler builds of the original header define this assembler text
// macro.  Keep the exact emitted text available as a Rust string constant.
pub const DEFINE_ASM_GPR_NUMS: &str = r#"\
\t.irp\tnum,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31\n
\t.equ\t.L__gpr_num_x\\num, \\num\n
\t.endr\n
\t.equ\t.L__gpr_num_zero,\t0\n
\t.equ\t.L__gpr_num_ra,\t\t1\n
\t.equ\t.L__gpr_num_sp,\t\t2\n
\t.equ\t.L__gpr_num_gp,\t\t3\n
\t.equ\t.L__gpr_num_tp,\t\t4\n
\t.equ\t.L__gpr_num_t0,\t\t5\n
\t.equ\t.L__gpr_num_t1,\t\t6\n
\t.equ\t.L__gpr_num_t2,\t\t7\n
\t.equ\t.L__gpr_num_s0,\t\t8\n
\t.equ\t.L__gpr_num_s1,\t\t9\n
\t.equ\t.L__gpr_num_a0,\t\t10\n
\t.equ\t.L__gpr_num_a1,\t\t11\n
\t.equ\t.L__gpr_num_a2,\t\t12\n
\t.equ\t.L__gpr_num_a3,\t\t13\n
\t.equ\t.L__gpr_num_a4,\t\t14\n
\t.equ\t.L__gpr_num_a5,\t\t15\n
\t.equ\t.L__gpr_num_a6,\t\t16\n
\t.equ\t.L__gpr_num_a7,\t\t17\n
\t.equ\t.L__gpr_num_s2,\t\t18\n
\t.equ\t.L__gpr_num_s3,\t\t19\n
\t.equ\t.L__gpr_num_s4,\t\t20\n
\t.equ\t.L__gpr_num_s5,\t\t21\n
\t.equ\t.L__gpr_num_s6,\t\t22\n
\t.equ\t.L__gpr_num_s7,\t\t23\n
\t.equ\t.L__gpr_num_s8,\t\t24\n
\t.equ\t.L__gpr_num_s9,\t\t25\n
\t.equ\t.L__gpr_num_s10,\t26\n
\t.equ\t.L__gpr_num_s11,\t27\n
\t.equ\t.L__gpr_num_t3,\t\t28\n
\t.equ\t.L__gpr_num_t4,\t\t29\n
\t.equ\t.L__gpr_num_t5,\t\t30\n
\t.equ\t.L__gpr_num_t6,\t\t31\n
"#;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
