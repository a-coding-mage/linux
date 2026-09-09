/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of the 32-bit ARM BPF JIT encoding header. */

/* ARM 32bit Registers */
pub const ARM_R0: u32 = 0; pub const ARM_R1: u32 = 1; pub const ARM_R2: u32 = 2; pub const ARM_R3: u32 = 3;
pub const ARM_R4: u32 = 4; pub const ARM_R5: u32 = 5; pub const ARM_R6: u32 = 6; pub const ARM_R7: u32 = 7;
pub const ARM_R8: u32 = 8; pub const ARM_R9: u32 = 9; pub const ARM_R10: u32 = 10;
pub const ARM_FP: u32 = 11; pub const ARM_IP: u32 = 12; pub const ARM_SP: u32 = 13;
pub const ARM_LR: u32 = 14; pub const ARM_PC: u32 = 15;

pub const ARM_COND_EQ: u32 = 0x0; pub const ARM_COND_NE: u32 = 0x1;
pub const ARM_COND_CS: u32 = 0x2; pub const ARM_COND_HS: u32 = ARM_COND_CS;
pub const ARM_COND_CC: u32 = 0x3; pub const ARM_COND_LO: u32 = ARM_COND_CC;
pub const ARM_COND_MI: u32 = 0x4; pub const ARM_COND_PL: u32 = 0x5;
pub const ARM_COND_VS: u32 = 0x6; pub const ARM_COND_VC: u32 = 0x7;
pub const ARM_COND_HI: u32 = 0x8; pub const ARM_COND_LS: u32 = 0x9;
pub const ARM_COND_GE: u32 = 0xa; pub const ARM_COND_LT: u32 = 0xb;
pub const ARM_COND_GT: u32 = 0xc; pub const ARM_COND_LE: u32 = 0xd;
pub const ARM_COND_AL: u32 = 0xe;

pub const SRTYPE_LSL: u32 = 0; pub const SRTYPE_LSR: u32 = 1;
pub const SRTYPE_ASR: u32 = 2; pub const SRTYPE_ROR: u32 = 3; pub const SRTYPE_ASL: u32 = SRTYPE_LSL;

pub const ARM_INST_ADD_R: u32 = 0x00800000; pub const ARM_INST_ADDS_R: u32 = 0x00900000;
pub const ARM_INST_ADC_R: u32 = 0x00a00000; pub const ARM_INST_ADC_I: u32 = 0x02a00000;
pub const ARM_INST_ADD_I: u32 = 0x02800000; pub const ARM_INST_ADDS_I: u32 = 0x02900000;
pub const ARM_INST_AND_R: u32 = 0x00000000; pub const ARM_INST_ANDS_R: u32 = 0x00100000; pub const ARM_INST_AND_I: u32 = 0x02000000;
pub const ARM_INST_BIC_R: u32 = 0x01c00000; pub const ARM_INST_BIC_I: u32 = 0x03c00000;
pub const ARM_INST_B: u32 = 0x0a000000; pub const ARM_INST_BX: u32 = 0x012fff10; pub const ARM_INST_BLX_R: u32 = 0x012fff30;
pub const ARM_INST_CMP_R: u32 = 0x01500000; pub const ARM_INST_CMP_I: u32 = 0x03500000;
pub const ARM_INST_EOR_R: u32 = 0x00200000; pub const ARM_INST_EOR_I: u32 = 0x02200000;
pub const ARM_INST_LDST__U: u32 = 0x00800000; pub const ARM_INST_LDST__IMM12: u32 = 0x00000fff;
pub const ARM_INST_LDRB_I: u32 = 0x05500000; pub const ARM_INST_LDRB_R: u32 = 0x07d00000;
pub const ARM_INST_LDRSB_I: u32 = 0x015000d0; pub const ARM_INST_LDRD_I: u32 = 0x014000d0;
pub const ARM_INST_LDRH_I: u32 = 0x015000b0; pub const ARM_INST_LDRH_R: u32 = 0x019000b0;
pub const ARM_INST_LDRSH_I: u32 = 0x015000f0; pub const ARM_INST_LDR_I: u32 = 0x05100000; pub const ARM_INST_LDR_R: u32 = 0x07900000;
pub const ARM_INST_LDM: u32 = 0x08900000; pub const ARM_INST_LDM_IA: u32 = 0x08b00000;
pub const ARM_INST_LSL_I: u32 = 0x01a00000; pub const ARM_INST_LSL_R: u32 = 0x01a00010;
pub const ARM_INST_LSR_I: u32 = 0x01a00020; pub const ARM_INST_LSR_R: u32 = 0x01a00030;
pub const ARM_INST_ASR_I: u32 = 0x01a00040; pub const ARM_INST_ASR_R: u32 = 0x01a00050;
pub const ARM_INST_MOV_R: u32 = 0x01a00000; pub const ARM_INST_MOVS_R: u32 = 0x01b00000;
pub const ARM_INST_MOV_I: u32 = 0x03a00000; pub const ARM_INST_MOVW: u32 = 0x03000000; pub const ARM_INST_MOVT: u32 = 0x03400000;
pub const ARM_INST_MUL: u32 = 0x00000090; pub const ARM_INST_POP: u32 = 0x08bd0000; pub const ARM_INST_PUSH: u32 = 0x092d0000;
pub const ARM_INST_ORR_R: u32 = 0x01800000; pub const ARM_INST_ORRS_R: u32 = 0x01900000; pub const ARM_INST_ORR_I: u32 = 0x03800000;
pub const ARM_INST_REV: u32 = 0x06bf0f30; pub const ARM_INST_REV16: u32 = 0x06bf0fb0;
pub const ARM_INST_RSB_I: u32 = 0x02600000; pub const ARM_INST_RSBS_I: u32 = 0x02700000; pub const ARM_INST_RSC_I: u32 = 0x02e00000;
pub const ARM_INST_SUB_R: u32 = 0x00400000; pub const ARM_INST_SUBS_R: u32 = 0x00500000; pub const ARM_INST_RSB_R: u32 = 0x00600000;
pub const ARM_INST_SUB_I: u32 = 0x02400000; pub const ARM_INST_SUBS_I: u32 = 0x02500000; pub const ARM_INST_SBC_I: u32 = 0x02c00000;
pub const ARM_INST_SBC_R: u32 = 0x00c00000; pub const ARM_INST_SBCS_R: u32 = 0x00d00000;
pub const ARM_INST_STR_I: u32 = 0x05000000; pub const ARM_INST_STRB_I: u32 = 0x05400000; pub const ARM_INST_STRD_I: u32 = 0x014000f0; pub const ARM_INST_STRH_I: u32 = 0x014000b0;
pub const ARM_INST_TST_R: u32 = 0x01100000; pub const ARM_INST_TST_I: u32 = 0x03100000;
pub const ARM_INST_UDIV: u32 = 0x0730f010; pub const ARM_INST_SDIV: u32 = 0x0710f010; pub const ARM_INST_UMULL: u32 = 0x00800090;
pub const ARM_INST_MLS: u32 = 0x00600090; pub const ARM_INST_UXTH: u32 = 0x06ff0070; pub const ARM_INST_UDF: u32 = 0xe7fddef1;

macro_rules! al3_r { ($op:expr,$rd:expr,$rn:expr,$rm:expr) => { ($op | ($rd << 12) | ($rn << 16) | $rm) }; }
macro_rules! al3_i { ($op:expr,$rd:expr,$rn:expr,$imm:expr) => { ($op | ($rd << 12) | ($rn << 16) | $imm) }; }
macro_rules! al3_sr { ($inst:expr) => { ($inst | (1 << 4)) }; }

macro_rules! ARM_ADD_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_ADD_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_ADDS_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_ADDS_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_ADD_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_ADD_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_ADDS_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_ADDS_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_ADC_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_ADC_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_ADC_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_ADC_I | ($rd << 12) | ($rn << 16) | $imm }; }

macro_rules! ARM_AND_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_AND_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_ANDS_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_ANDS_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_AND_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_AND_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_BIC_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_BIC_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_BIC_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_BIC_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_B { ($imm24:expr) => { ARM_INST_B | ($imm24 & 0xffffff) }; }
macro_rules! ARM_BX { ($rm:expr) => { ARM_INST_BX | $rm }; }
macro_rules! ARM_BLX_R { ($rm:expr) => { ARM_INST_BLX_R | $rm }; }
macro_rules! ARM_CMP_R { ($rn:expr,$rm:expr) => { ARM_INST_CMP_R | ($rn << 16) | $rm }; }
macro_rules! ARM_CMP_I { ($rn:expr,$imm:expr) => { ARM_INST_CMP_I | ($rn << 16) | $imm }; }
macro_rules! ARM_EOR_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_EOR_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_EOR_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_EOR_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_LDR_R { ($rt:expr,$rn:expr,$rm:expr) => { ARM_INST_LDR_R | ARM_INST_LDST__U | ($rt << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_LDR_R_SI { ($rt:expr,$rn:expr,$rm:expr,$typ:expr,$imm:expr) => { ARM_INST_LDR_R | ARM_INST_LDST__U | ($rt << 12) | ($rn << 16) | ($imm << 7) | ($typ << 5) | $rm }; }
macro_rules! ARM_LDRB_R { ($rt:expr,$rn:expr,$rm:expr) => { ARM_INST_LDRB_R | ARM_INST_LDST__U | ($rt << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_LDRH_R { ($rt:expr,$rn:expr,$rm:expr) => { ARM_INST_LDRH_R | ARM_INST_LDST__U | ($rt << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_LDM { ($rn:expr,$regs:expr) => { ARM_INST_LDM | ($rn << 16) | $regs }; }
macro_rules! ARM_LDM_IA { ($rn:expr,$regs:expr) => { ARM_INST_LDM_IA | ($rn << 16) | $regs }; }
macro_rules! ARM_LSL_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_LSL_R | ($rd << 12) | $rn | ($rm << 8) }; }
macro_rules! ARM_LSL_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_LSL_I | ($rd << 12) | $rn | ($imm << 7) }; }
macro_rules! ARM_LSR_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_LSR_R | ($rd << 12) | $rn | ($rm << 8) }; }
macro_rules! ARM_LSR_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_LSR_I | ($rd << 12) | $rn | ($imm << 7) }; }
macro_rules! ARM_ASR_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_ASR_R | ($rd << 12) | $rn | ($rm << 8) }; }
macro_rules! ARM_ASR_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_ASR_I | ($rd << 12) | $rn | ($imm << 7) }; }
macro_rules! ARM_MOV_R { ($rd:expr,$rm:expr) => { ARM_INST_MOV_R | ($rd << 12) | $rm }; }
macro_rules! ARM_MOVS_R { ($rd:expr,$rm:expr) => { ARM_INST_MOVS_R | ($rd << 12) | $rm }; }
macro_rules! ARM_MOV_I { ($rd:expr,$imm:expr) => { ARM_INST_MOV_I | ($rd << 12) | $imm }; }
macro_rules! ARM_MOV_SR { ($rd:expr,$rm:expr,$typ:expr,$rs:expr) => { ARM_MOV_R!($rd,$rm) | (1 << 4) | ($typ << 5) | ($rs << 8) }; }
macro_rules! ARM_MOV_SI { ($rd:expr,$rm:expr,$typ:expr,$imm6:expr) => { ARM_MOV_R!($rd,$rm) | ($typ << 5) | ($imm6 << 7) }; }
macro_rules! ARM_MOVW { ($rd:expr,$imm:expr) => { ARM_INST_MOVW | (($imm >> 12) << 16) | ($rd << 12) | ($imm & 0x0fff) }; }
macro_rules! ARM_MOVT { ($rd:expr,$imm:expr) => { ARM_INST_MOVT | (($imm >> 12) << 16) | ($rd << 12) | ($imm & 0x0fff) }; }
macro_rules! ARM_MUL { ($rd:expr,$rm:expr,$rn:expr) => { ARM_INST_MUL | ($rd << 16) | ($rm << 8) | $rn }; }
macro_rules! ARM_POP { ($regs:expr) => { ARM_INST_POP | $regs }; }
macro_rules! ARM_PUSH { ($regs:expr) => { ARM_INST_PUSH | $regs }; }
macro_rules! ARM_ORR_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_ORR_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_ORR_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_ORR_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_ORR_SR { ($rd:expr,$rn:expr,$rm:expr,$typ:expr,$rs:expr) => { ARM_ORR_R!($rd,$rn,$rm) | (1 << 4) | ($typ << 5) | ($rs << 8) }; }
macro_rules! ARM_ORRS_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_ORRS_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_ORRS_SR { ($rd:expr,$rn:expr,$rm:expr,$typ:expr,$rs:expr) => { ARM_ORRS_R!($rd,$rn,$rm) | (1 << 4) | ($typ << 5) | ($rs << 8) }; }
macro_rules! ARM_ORR_SI { ($rd:expr,$rn:expr,$rm:expr,$typ:expr,$imm6:expr) => { ARM_ORR_R!($rd,$rn,$rm) | ($typ << 5) | ($imm6 << 7) }; }
macro_rules! ARM_ORRS_SI { ($rd:expr,$rn:expr,$rm:expr,$typ:expr,$imm6:expr) => { ARM_ORR_R!($rd,$rn,$rm) | ($typ << 5) | ($imm6 << 7) }; }
macro_rules! ARM_REV { ($rd:expr,$rm:expr) => { ARM_INST_REV | ($rd << 12) | $rm }; }
macro_rules! ARM_REV16 { ($rd:expr,$rm:expr) => { ARM_INST_REV16 | ($rd << 12) | $rm }; }
macro_rules! ARM_RSB_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_RSB_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_RSBS_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_RSBS_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_RSC_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_RSC_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_SUB_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_SUB_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_SUBS_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_SUBS_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_RSB_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_RSB_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_SBC_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_SBC_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_SBCS_R { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_SBCS_R | ($rd << 12) | ($rn << 16) | $rm }; }
macro_rules! ARM_SUB_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_SUB_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_SUBS_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_SUBS_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_SBC_I { ($rd:expr,$rn:expr,$imm:expr) => { ARM_INST_SBC_I | ($rd << 12) | ($rn << 16) | $imm }; }
macro_rules! ARM_TST_R { ($rn:expr,$rm:expr) => { ARM_INST_TST_R | ($rn << 16) | $rm }; }
macro_rules! ARM_TST_I { ($rn:expr,$imm:expr) => { ARM_INST_TST_I | ($rn << 16) | $imm }; }
macro_rules! ARM_UDIV { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_UDIV | ($rd << 16) | $rn | ($rm << 8) }; }
macro_rules! ARM_SDIV { ($rd:expr,$rn:expr,$rm:expr) => { ARM_INST_SDIV | ($rd << 16) | $rn | ($rm << 8) }; }
macro_rules! ARM_UMULL { ($rd_lo:expr,$rd_hi:expr,$rn:expr,$rm:expr) => { ARM_INST_UMULL | ($rd_hi << 16) | ($rd_lo << 12) | ($rm << 8) | $rn }; }
macro_rules! ARM_MLS { ($rd:expr,$rn:expr,$rm:expr,$ra:expr) => { ARM_INST_MLS | ($rd << 16) | $rn | ($rm << 8) | ($ra << 12) }; }
macro_rules! ARM_UXTH { ($rd:expr,$rm:expr) => { ARM_INST_UXTH | ($rd << 12) | $rm }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
