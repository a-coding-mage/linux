/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Huawei Ltd.
 * Author: Jiang Liu <liuj97@gmail.com>
 *
 * Copyright (C) 2014 Zi Shen Lim <zlim.lnx@gmail.com>
 */

// C includes: linux/build_bug.h, linux/types.h, asm/insn-def.h
// The declarations below are omitted when building as an assembler source.

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_hint_cr_op {
    AARCH64_INSN_HINT_NOP = 0x0 << 5, AARCH64_INSN_HINT_YIELD = 0x1 << 5,
    AARCH64_INSN_HINT_WFE = 0x2 << 5, AARCH64_INSN_HINT_WFI = 0x3 << 5,
    AARCH64_INSN_HINT_SEV = 0x4 << 5, AARCH64_INSN_HINT_SEVL = 0x5 << 5,
    AARCH64_INSN_HINT_XPACLRI = 0x07 << 5, AARCH64_INSN_HINT_PACIA_1716 = 0x08 << 5,
    AARCH64_INSN_HINT_PACIB_1716 = 0x0A << 5, AARCH64_INSN_HINT_AUTIA_1716 = 0x0C << 5,
    AARCH64_INSN_HINT_AUTIB_1716 = 0x0E << 5, AARCH64_INSN_HINT_PACIAZ = 0x18 << 5,
    AARCH64_INSN_HINT_PACIASP = 0x19 << 5, AARCH64_INSN_HINT_PACIBZ = 0x1A << 5,
    AARCH64_INSN_HINT_PACIBSP = 0x1B << 5, AARCH64_INSN_HINT_AUTIAZ = 0x1C << 5,
    AARCH64_INSN_HINT_AUTIASP = 0x1D << 5, AARCH64_INSN_HINT_AUTIBZ = 0x1E << 5,
    AARCH64_INSN_HINT_AUTIBSP = 0x1F << 5, AARCH64_INSN_HINT_ESB = 0x10 << 5,
    AARCH64_INSN_HINT_PSB = 0x11 << 5, AARCH64_INSN_HINT_TSB = 0x12 << 5,
    AARCH64_INSN_HINT_CSDB = 0x14 << 5, AARCH64_INSN_HINT_CLEARBHB = 0x16 << 5,
    AARCH64_INSN_HINT_BTI = 0x20 << 5, AARCH64_INSN_HINT_BTIC = 0x22 << 5,
    AARCH64_INSN_HINT_BTIJ = 0x24 << 5, AARCH64_INSN_HINT_BTIJC = 0x26 << 5,
}

macro_rules! c_enum {
    ($name:ident { $($item:ident),* $(,)? }) => {
        #[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum $name { $($item),* }
    };
}
c_enum!(aarch64_insn_imm_type { AARCH64_INSN_IMM_ADR, AARCH64_INSN_IMM_26, AARCH64_INSN_IMM_19, AARCH64_INSN_IMM_16, AARCH64_INSN_IMM_14, AARCH64_INSN_IMM_12, AARCH64_INSN_IMM_9, AARCH64_INSN_IMM_7, AARCH64_INSN_IMM_6, AARCH64_INSN_IMM_S, AARCH64_INSN_IMM_R, AARCH64_INSN_IMM_N, AARCH64_INSN_IMM_MAX });
c_enum!(aarch64_insn_register_type { AARCH64_INSN_REGTYPE_RT, AARCH64_INSN_REGTYPE_RN, AARCH64_INSN_REGTYPE_RT2, AARCH64_INSN_REGTYPE_RM, AARCH64_INSN_REGTYPE_RD, AARCH64_INSN_REGTYPE_RA, AARCH64_INSN_REGTYPE_RS });
#[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum aarch64_insn_register { AARCH64_INSN_REG_0=0, AARCH64_INSN_REG_1=1, AARCH64_INSN_REG_2=2, AARCH64_INSN_REG_3=3, AARCH64_INSN_REG_4=4, AARCH64_INSN_REG_5=5, AARCH64_INSN_REG_6=6, AARCH64_INSN_REG_7=7, AARCH64_INSN_REG_8=8, AARCH64_INSN_REG_9=9, AARCH64_INSN_REG_10=10, AARCH64_INSN_REG_11=11, AARCH64_INSN_REG_12=12, AARCH64_INSN_REG_13=13, AARCH64_INSN_REG_14=14, AARCH64_INSN_REG_15=15, AARCH64_INSN_REG_16=16, AARCH64_INSN_REG_17=17, AARCH64_INSN_REG_18=18, AARCH64_INSN_REG_19=19, AARCH64_INSN_REG_20=20, AARCH64_INSN_REG_21=21, AARCH64_INSN_REG_22=22, AARCH64_INSN_REG_23=23, AARCH64_INSN_REG_24=24, AARCH64_INSN_REG_25=25, AARCH64_INSN_REG_26=26, AARCH64_INSN_REG_27=27, AARCH64_INSN_REG_28=28, AARCH64_INSN_REG_29=29, AARCH64_INSN_REG_FP=29, AARCH64_INSN_REG_30=30, AARCH64_INSN_REG_LR=30, AARCH64_INSN_REG_ZR=31, AARCH64_INSN_REG_SP=31 }
#[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum aarch64_insn_special_register { AARCH64_INSN_SPCLREG_SPSR_EL1=0xC200, AARCH64_INSN_SPCLREG_ELR_EL1=0xC201, AARCH64_INSN_SPCLREG_SP_EL0=0xC208, AARCH64_INSN_SPCLREG_SPSEL=0xC210, AARCH64_INSN_SPCLREG_CURRENTEL=0xC212, AARCH64_INSN_SPCLREG_DAIF=0xDA11, AARCH64_INSN_SPCLREG_NZCV=0xDA10, AARCH64_INSN_SPCLREG_FPCR=0xDA20, AARCH64_INSN_SPCLREG_DSPSR_EL0=0xDA28, AARCH64_INSN_SPCLREG_DLR_EL0=0xDA29, AARCH64_INSN_SPCLREG_SPSR_EL2=0xE200, AARCH64_INSN_SPCLREG_ELR_EL2=0xE201, AARCH64_INSN_SPCLREG_SP_EL1=0xE208, AARCH64_INSN_SPCLREG_SPSR_INQ=0xE218, AARCH64_INSN_SPCLREG_SPSR_ABT=0xE219, AARCH64_INSN_SPCLREG_SPSR_UND=0xE21A, AARCH64_INSN_SPCLREG_SPSR_FIQ=0xE21B, AARCH64_INSN_SPCLREG_SPSR_EL3=0xF200, AARCH64_INSN_SPCLREG_ELR_EL3=0xF201, AARCH64_INSN_SPCLREG_SP_EL2=0xF210 }
#[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum aarch64_insn_system_register { AARCH64_INSN_SYSREG_TPIDR_EL1=0x4684, AARCH64_INSN_SYSREG_TPIDR_EL2=0x6682, AARCH64_INSN_SYSREG_SP_EL0=0x4208 }
c_enum!(aarch64_insn_variant { AARCH64_INSN_VARIANT_32BIT, AARCH64_INSN_VARIANT_64BIT });
#[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum aarch64_insn_condition { AARCH64_INSN_COND_EQ=0x0, AARCH64_INSN_COND_NE=0x1, AARCH64_INSN_COND_CS=0x2, AARCH64_INSN_COND_CC=0x3, AARCH64_INSN_COND_MI=0x4, AARCH64_INSN_COND_PL=0x5, AARCH64_INSN_COND_VS=0x6, AARCH64_INSN_COND_VC=0x7, AARCH64_INSN_COND_HI=0x8, AARCH64_INSN_COND_LS=0x9, AARCH64_INSN_COND_GE=0xa, AARCH64_INSN_COND_LT=0xb, AARCH64_INSN_COND_GT=0xc, AARCH64_INSN_COND_LE=0xd, AARCH64_INSN_COND_AL=0xe }
c_enum!(aarch64_insn_branch_type { AARCH64_INSN_BRANCH_NOLINK, AARCH64_INSN_BRANCH_LINK, AARCH64_INSN_BRANCH_RETURN, AARCH64_INSN_BRANCH_COMP_ZERO, AARCH64_INSN_BRANCH_COMP_NONZERO });
c_enum!(aarch64_insn_size_type { AARCH64_INSN_SIZE_8, AARCH64_INSN_SIZE_16, AARCH64_INSN_SIZE_32, AARCH64_INSN_SIZE_64 });
c_enum!(aarch64_insn_ldst_type { AARCH64_INSN_LDST_LOAD_REG_OFFSET, AARCH64_INSN_LDST_STORE_REG_OFFSET, AARCH64_INSN_LDST_LOAD_IMM_OFFSET, AARCH64_INSN_LDST_STORE_IMM_OFFSET, AARCH64_INSN_LDST_LOAD_PAIR_PRE_INDEX, AARCH64_INSN_LDST_STORE_PAIR_PRE_INDEX, AARCH64_INSN_LDST_LOAD_PAIR_POST_INDEX, AARCH64_INSN_LDST_STORE_PAIR_POST_INDEX, AARCH64_INSN_LDST_LOAD_ACQ, AARCH64_INSN_LDST_LOAD_EX, AARCH64_INSN_LDST_LOAD_ACQ_EX, AARCH64_INSN_LDST_STORE_REL, AARCH64_INSN_LDST_STORE_EX, AARCH64_INSN_LDST_STORE_REL_EX, AARCH64_INSN_LDST_SIGNED_LOAD_IMM_OFFSET, AARCH64_INSN_LDST_SIGNED_LOAD_REG_OFFSET });
c_enum!(aarch64_insn_adsb_type { AARCH64_INSN_ADSB_ADD, AARCH64_INSN_ADSB_SUB, AARCH64_INSN_ADSB_ADD_SETFLAGS, AARCH64_INSN_ADSB_SUB_SETFLAGS });
c_enum!(aarch64_insn_extend_type { AARCH64_INSN_EXTEND_UXTB, AARCH64_INSN_EXTEND_UXTH, AARCH64_INSN_EXTEND_UXTW, AARCH64_INSN_EXTEND_UXTX, AARCH64_INSN_EXTEND_SXTB, AARCH64_INSN_EXTEND_SXTH, AARCH64_INSN_EXTEND_SXTW, AARCH64_INSN_EXTEND_SXTX });
c_enum!(aarch64_insn_movewide_type { AARCH64_INSN_MOVEWIDE_ZERO, AARCH64_INSN_MOVEWIDE_KEEP, AARCH64_INSN_MOVEWIDE_INVERSE });
c_enum!(aarch64_insn_bitfield_type { AARCH64_INSN_BITFIELD_MOVE, AARCH64_INSN_BITFIELD_MOVE_UNSIGNED, AARCH64_INSN_BITFIELD_MOVE_SIGNED });
c_enum!(aarch64_insn_data1_type { AARCH64_INSN_DATA1_REVERSE_16, AARCH64_INSN_DATA1_REVERSE_32, AARCH64_INSN_DATA1_REVERSE_64 });
c_enum!(aarch64_insn_data2_type { AARCH64_INSN_DATA2_UDIV, AARCH64_INSN_DATA2_SDIV, AARCH64_INSN_DATA2_LSLV, AARCH64_INSN_DATA2_LSRV, AARCH64_INSN_DATA2_ASRV, AARCH64_INSN_DATA2_RORV });
c_enum!(aarch64_insn_data3_type { AARCH64_INSN_DATA3_MADD, AARCH64_INSN_DATA3_MSUB });
c_enum!(aarch64_insn_logic_type { AARCH64_INSN_LOGIC_AND, AARCH64_INSN_LOGIC_BIC, AARCH64_INSN_LOGIC_ORR, AARCH64_INSN_LOGIC_ORN, AARCH64_INSN_LOGIC_EOR, AARCH64_INSN_LOGIC_EON, AARCH64_INSN_LOGIC_AND_SETFLAGS, AARCH64_INSN_LOGIC_BIC_SETFLAGS });
c_enum!(aarch64_insn_prfm_type { AARCH64_INSN_PRFM_TYPE_PLD, AARCH64_INSN_PRFM_TYPE_PLI, AARCH64_INSN_PRFM_TYPE_PST });
c_enum!(aarch64_insn_prfm_target { AARCH64_INSN_PRFM_TARGET_L1, AARCH64_INSN_PRFM_TARGET_L2, AARCH64_INSN_PRFM_TARGET_L3 });
c_enum!(aarch64_insn_prfm_policy { AARCH64_INSN_PRFM_POLICY_KEEP, AARCH64_INSN_PRFM_POLICY_STRM });
c_enum!(aarch64_insn_adr_type { AARCH64_INSN_ADR_TYPE_ADRP, AARCH64_INSN_ADR_TYPE_ADR });
c_enum!(aarch64_insn_mem_atomic_op { AARCH64_INSN_MEM_ATOMIC_ADD, AARCH64_INSN_MEM_ATOMIC_CLR, AARCH64_INSN_MEM_ATOMIC_EOR, AARCH64_INSN_MEM_ATOMIC_SET, AARCH64_INSN_MEM_ATOMIC_SWP });
c_enum!(aarch64_insn_mem_order_type { AARCH64_INSN_MEM_ORDER_NONE, AARCH64_INSN_MEM_ORDER_ACQ, AARCH64_INSN_MEM_ORDER_REL, AARCH64_INSN_MEM_ORDER_ACQREL });
c_enum!(aarch64_insn_mb_type { AARCH64_INSN_MB_SY, AARCH64_INSN_MB_ST, AARCH64_INSN_MB_LD, AARCH64_INSN_MB_ISH, AARCH64_INSN_MB_ISHST, AARCH64_INSN_MB_ISHLD, AARCH64_INSN_MB_NSH, AARCH64_INSN_MB_NSHST, AARCH64_INSN_MB_NSHLD, AARCH64_INSN_MB_OSH, AARCH64_INSN_MB_OSHST, AARCH64_INSN_MB_OSHLD });

macro_rules! insn_func {
    ($is:ident, $get:ident, $mask:expr, $val:expr) => {
        pub const fn $is(code: u32) -> bool { (code & ($mask as u32)) == ($val as u32) }
        pub const fn $get() -> u32 { $val as u32 }
    };
}

// __AARCH64_INSN_FUNCS(abbr, mask, val), expanded explicitly because Rust
// identifiers cannot be concatenated in a declarative macro.
insn_func!(aarch64_insn_is_class_branch_sys, aarch64_insn_get_class_branch_sys_value, 0x1c000000, 0x14000000);
insn_func!(aarch64_insn_is_adr, aarch64_insn_get_adr_value, 0x9F000000, 0x10000000); insn_func!(aarch64_insn_is_adrp, aarch64_insn_get_adrp_value, 0x9F000000, 0x90000000);
insn_func!(aarch64_insn_is_prfm, aarch64_insn_get_prfm_value, 0x3FC00000, 0x39800000); insn_func!(aarch64_insn_is_prfm_lit, aarch64_insn_get_prfm_lit_value, 0xFF000000, 0xD8000000);
insn_func!(aarch64_insn_is_store_imm, aarch64_insn_get_store_imm_value, 0x3FC00000, 0x39000000); insn_func!(aarch64_insn_is_load_imm, aarch64_insn_get_load_imm_value, 0x3FC00000, 0x39400000); insn_func!(aarch64_insn_is_signed_load_imm, aarch64_insn_get_signed_load_imm_value, 0x3FC00000, 0x39800000);
insn_func!(aarch64_insn_is_store_pre, aarch64_insn_get_store_pre_value, 0x3FE00C00, 0x38000C00); insn_func!(aarch64_insn_is_load_pre, aarch64_insn_get_load_pre_value, 0x3FE00C00, 0x38400C00); insn_func!(aarch64_insn_is_store_post, aarch64_insn_get_store_post_value, 0x3FE00C00, 0x38000400); insn_func!(aarch64_insn_is_load_post, aarch64_insn_get_load_post_value, 0x3FE00C00, 0x38400400);
insn_func!(aarch64_insn_is_str_reg, aarch64_insn_get_str_reg_value, 0x3FE0EC00, 0x38206800); insn_func!(aarch64_insn_is_str_imm, aarch64_insn_get_str_imm_value, 0x3FC00000, 0x39000000);
insn_func!(aarch64_insn_is_ldadd, aarch64_insn_get_ldadd_value, 0x3F20FC00, 0x38200000); insn_func!(aarch64_insn_is_ldclr, aarch64_insn_get_ldclr_value, 0x3F20FC00, 0x38201000); insn_func!(aarch64_insn_is_ldeor, aarch64_insn_get_ldeor_value, 0x3F20FC00, 0x38202000); insn_func!(aarch64_insn_is_ldset, aarch64_insn_get_ldset_value, 0x3F20FC00, 0x38203000); insn_func!(aarch64_insn_is_swp, aarch64_insn_get_swp_value, 0x3F20FC00, 0x38208000); insn_func!(aarch64_insn_is_cas, aarch64_insn_get_cas_value, 0x3FA07C00, 0x08A07C00);
insn_func!(aarch64_insn_is_ldr_reg, aarch64_insn_get_ldr_reg_value, 0x3FE0EC00, 0x38606800); insn_func!(aarch64_insn_is_signed_ldr_reg, aarch64_insn_get_signed_ldr_reg_value, 0x3FE0FC00, 0x38A0E800); insn_func!(aarch64_insn_is_ldr_imm, aarch64_insn_get_ldr_imm_value, 0x3FC00000, 0x39400000); insn_func!(aarch64_insn_is_ldr_lit, aarch64_insn_get_ldr_lit_value, 0xBF000000, 0x18000000); insn_func!(aarch64_insn_is_ldrsw_lit, aarch64_insn_get_ldrsw_lit_value, 0xFF000000, 0x98000000);
insn_func!(aarch64_insn_is_exclusive, aarch64_insn_get_exclusive_value, 0x3F800000, 0x08000000); insn_func!(aarch64_insn_is_load_acq, aarch64_insn_get_load_acq_value, 0x3FDFFC00, 0x08DFFC00); insn_func!(aarch64_insn_is_store_rel, aarch64_insn_get_store_rel_value, 0x3FDFFC00, 0x089FFC00); insn_func!(aarch64_insn_is_load_ex, aarch64_insn_get_load_ex_value, 0x3FC00000, 0x08400000); insn_func!(aarch64_insn_is_store_ex, aarch64_insn_get_store_ex_value, 0x3FC00000, 0x08000000); insn_func!(aarch64_insn_is_mops, aarch64_insn_get_mops_value, 0x3B200C00, 0x19000400);
insn_func!(aarch64_insn_is_stp, aarch64_insn_get_stp_value, 0x7FC00000, 0x29000000); insn_func!(aarch64_insn_is_ldp, aarch64_insn_get_ldp_value, 0x7FC00000, 0x29400000); insn_func!(aarch64_insn_is_stp_post, aarch64_insn_get_stp_post_value, 0x7FC00000, 0x28800000); insn_func!(aarch64_insn_is_ldp_post, aarch64_insn_get_ldp_post_value, 0x7FC00000, 0x28C00000); insn_func!(aarch64_insn_is_stp_pre, aarch64_insn_get_stp_pre_value, 0x7FC00000, 0x29800000); insn_func!(aarch64_insn_is_ldp_pre, aarch64_insn_get_ldp_pre_value, 0x7FC00000, 0x29C00000);
insn_func!(aarch64_insn_is_add_imm,aarch64_insn_get_add_imm_value,0x7F000000,0x11000000); insn_func!(aarch64_insn_is_adds_imm,aarch64_insn_get_adds_imm_value,0x7F000000,0x31000000); insn_func!(aarch64_insn_is_sub_imm,aarch64_insn_get_sub_imm_value,0x7F000000,0x51000000); insn_func!(aarch64_insn_is_subs_imm,aarch64_insn_get_subs_imm_value,0x7F000000,0x71000000);
insn_func!(aarch64_insn_is_movn,aarch64_insn_get_movn_value,0x7F800000,0x12800000); insn_func!(aarch64_insn_is_sbfm,aarch64_insn_get_sbfm_value,0x7F800000,0x13000000); insn_func!(aarch64_insn_is_bfm,aarch64_insn_get_bfm_value,0x7F800000,0x33000000); insn_func!(aarch64_insn_is_movz,aarch64_insn_get_movz_value,0x7F800000,0x52800000); insn_func!(aarch64_insn_is_ubfm,aarch64_insn_get_ubfm_value,0x7F800000,0x53000000); insn_func!(aarch64_insn_is_movk,aarch64_insn_get_movk_value,0x7F800000,0x72800000);
insn_func!(aarch64_insn_is_add,aarch64_insn_get_add_value,0x7F200000,0x0B000000); insn_func!(aarch64_insn_is_adds,aarch64_insn_get_adds_value,0x7F200000,0x2B000000); insn_func!(aarch64_insn_is_sub,aarch64_insn_get_sub_value,0x7F200000,0x4B000000); insn_func!(aarch64_insn_is_subs,aarch64_insn_get_subs_value,0x7F200000,0x6B000000); insn_func!(aarch64_insn_is_add_ext,aarch64_insn_get_add_ext_value,0x7FE00000,0x0B200000); insn_func!(aarch64_insn_is_adds_ext,aarch64_insn_get_adds_ext_value,0x7FE00000,0x2B200000); insn_func!(aarch64_insn_is_sub_ext,aarch64_insn_get_sub_ext_value,0x7FE00000,0x4B200000); insn_func!(aarch64_insn_is_subs_ext,aarch64_insn_get_subs_ext_value,0x7FE00000,0x6B200000);
insn_func!(aarch64_insn_is_madd,aarch64_insn_get_madd_value,0x7FE08000,0x1B000000); insn_func!(aarch64_insn_is_msub,aarch64_insn_get_msub_value,0x7FE08000,0x1B008000); insn_func!(aarch64_insn_is_udiv,aarch64_insn_get_udiv_value,0x7FE0FC00,0x1AC00800); insn_func!(aarch64_insn_is_sdiv,aarch64_insn_get_sdiv_value,0x7FE0FC00,0x1AC00C00); insn_func!(aarch64_insn_is_lslv,aarch64_insn_get_lslv_value,0x7FE0FC00,0x1AC02000); insn_func!(aarch64_insn_is_lsrv,aarch64_insn_get_lsrv_value,0x7FE0FC00,0x1AC02400); insn_func!(aarch64_insn_is_asrv,aarch64_insn_get_asrv_value,0x7FE0FC00,0x1AC02800); insn_func!(aarch64_insn_is_rorv,aarch64_insn_get_rorv_value,0x7FE0FC00,0x1AC02C00);
insn_func!(aarch64_insn_is_rev16,aarch64_insn_get_rev16_value,0x7FFFFC00,0x5AC00400); insn_func!(aarch64_insn_is_rev32,aarch64_insn_get_rev32_value,0x7FFFFC00,0x5AC00800); insn_func!(aarch64_insn_is_rev64,aarch64_insn_get_rev64_value,0x7FFFFC00,0x5AC00C00);
insn_func!(aarch64_insn_is_and,aarch64_insn_get_and_value,0x7F200000,0x0A000000); insn_func!(aarch64_insn_is_bic,aarch64_insn_get_bic_value,0x7F200000,0x0A200000); insn_func!(aarch64_insn_is_orr,aarch64_insn_get_orr_value,0x7F200000,0x2A000000); insn_func!(aarch64_insn_is_mov_reg,aarch64_insn_get_mov_reg_value,0x7FE0FFE0,0x2A0003E0); insn_func!(aarch64_insn_is_orn,aarch64_insn_get_orn_value,0x7F200000,0x2A200000); insn_func!(aarch64_insn_is_eor,aarch64_insn_get_eor_value,0x7F200000,0x4A000000); insn_func!(aarch64_insn_is_eon,aarch64_insn_get_eon_value,0x7F200000,0x4A200000); insn_func!(aarch64_insn_is_ands,aarch64_insn_get_ands_value,0x7F200000,0x6A000000); insn_func!(aarch64_insn_is_bics,aarch64_insn_get_bics_value,0x7F200000,0x6A200000);
insn_func!(aarch64_insn_is_and_imm,aarch64_insn_get_and_imm_value,0x7F800000,0x12000000); insn_func!(aarch64_insn_is_orr_imm,aarch64_insn_get_orr_imm_value,0x7F800000,0x32000000); insn_func!(aarch64_insn_is_eor_imm,aarch64_insn_get_eor_imm_value,0x7F800000,0x52000000); insn_func!(aarch64_insn_is_ands_imm,aarch64_insn_get_ands_imm_value,0x7F800000,0x72000000); insn_func!(aarch64_insn_is_extr,aarch64_insn_get_extr_value,0x7FA00000,0x13800000);
insn_func!(aarch64_insn_is_b,aarch64_insn_get_b_value,0xFC000000,0x14000000); insn_func!(aarch64_insn_is_bl,aarch64_insn_get_bl_value,0xFC000000,0x94000000); insn_func!(aarch64_insn_is_cbz,aarch64_insn_get_cbz_value,0x7F000000,0x34000000); insn_func!(aarch64_insn_is_cbnz,aarch64_insn_get_cbnz_value,0x7F000000,0x35000000); insn_func!(aarch64_insn_is_tbz,aarch64_insn_get_tbz_value,0x7F000000,0x36000000); insn_func!(aarch64_insn_is_tbnz,aarch64_insn_get_tbnz_value,0x7F000000,0x37000000); insn_func!(aarch64_insn_is_bcond,aarch64_insn_get_bcond_value,0xFF000000,0x54000000);
insn_func!(aarch64_insn_is_svc,aarch64_insn_get_svc_value,0xFFE0001F,0xD4000001); insn_func!(aarch64_insn_is_hvc,aarch64_insn_get_hvc_value,0xFFE0001F,0xD4000002); insn_func!(aarch64_insn_is_smc,aarch64_insn_get_smc_value,0xFFE0001F,0xD4000003); insn_func!(aarch64_insn_is_brk,aarch64_insn_get_brk_value,0xFFE0001F,0xD4200000); insn_func!(aarch64_insn_is_exception,aarch64_insn_get_exception_value,0xFF000000,0xD4000000); insn_func!(aarch64_insn_is_hint,aarch64_insn_get_hint_value,0xFFFFF01F,0xD503201F);
insn_func!(aarch64_insn_is_br,aarch64_insn_get_br_value,0xFFFFFC1F,0xD61F0000); insn_func!(aarch64_insn_is_br_auth,aarch64_insn_get_br_auth_value,0xFEFFF800,0xD61F0800); insn_func!(aarch64_insn_is_blr,aarch64_insn_get_blr_value,0xFFFFFC1F,0xD63F0000); insn_func!(aarch64_insn_is_blr_auth,aarch64_insn_get_blr_auth_value,0xFEFFF800,0xD63F0800); insn_func!(aarch64_insn_is_ret,aarch64_insn_get_ret_value,0xFFFFFC1F,0xD65F0000); insn_func!(aarch64_insn_is_ret_auth,aarch64_insn_get_ret_auth_value,0xFFFFFBFF,0xD65F0BFF); insn_func!(aarch64_insn_is_eret,aarch64_insn_get_eret_value,0xFFFFFFFF,0xD69F03E0); insn_func!(aarch64_insn_is_eret_auth,aarch64_insn_get_eret_auth_value,0xFFFFFBFF,0xD69F0BFF);
insn_func!(aarch64_insn_is_mrs,aarch64_insn_get_mrs_value,0xFFF00000,0xD5300000); insn_func!(aarch64_insn_is_msr_imm,aarch64_insn_get_msr_imm_value,0xFFF8F01F,0xD500401F); insn_func!(aarch64_insn_is_msr_reg,aarch64_insn_get_msr_reg_value,0xFFF00000,0xD5100000); insn_func!(aarch64_insn_is_dmb,aarch64_insn_get_dmb_value,0xFFFFF0FF,0xD50330BF); insn_func!(aarch64_insn_is_dsb_base,aarch64_insn_get_dsb_base_value,0xFFFFF0FF,0xD503309F); insn_func!(aarch64_insn_is_dsb_nxs,aarch64_insn_get_dsb_nxs_value,0xFFFFF3FF,0xD503323F); insn_func!(aarch64_insn_is_isb,aarch64_insn_get_isb_value,0xFFFFF0FF,0xD50330DF); insn_func!(aarch64_insn_is_sb,aarch64_insn_get_sb_value,0xFFFFFFFF,0xD50330FF); insn_func!(aarch64_insn_is_clrex,aarch64_insn_get_clrex_value,0xFFFFF0FF,0xD503305F); insn_func!(aarch64_insn_is_ssbb,aarch64_insn_get_ssbb_value,0xFFFFFFFF,0xD503309F); insn_func!(aarch64_insn_is_pssbb,aarch64_insn_get_pssbb_value,0xFFFFFFFF,0xD503349F); insn_func!(aarch64_insn_is_bti,aarch64_insn_get_bti_value,0xFFFFFF3F,0xD503241f);

pub fn aarch64_insn_is_steppable_hint(insn: u32) -> bool {
    if !aarch64_insn_is_hint(insn) { return false; }
    match insn & 0xFE0 {
        0x0E0 | 0x100 | 0x140 | 0x300 | 0x320 | 0x340 | 0x360 | 0x400 | 0x440 | 0x480 | 0x4C0 | 0x000 => true,
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
