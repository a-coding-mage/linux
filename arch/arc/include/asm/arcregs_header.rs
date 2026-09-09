/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. */

// Build Configuration Registers
pub const ARC_REG_AUX_DCCM: u32 = 0x18;
pub const ARC_REG_ERP_CTRL: u32 = 0x3f;
pub const ARC_REG_DCCM_BASE_BUILD: u32 = 0x61;
pub const ARC_REG_CRC_BCR: u32 = 0x62;
pub const ARC_REG_VECBASE_BCR: u32 = 0x68;
pub const ARC_REG_PERIBASE_BCR: u32 = 0x69;
pub const ARC_REG_FP_BCR: u32 = 0x6b;
pub const ARC_REG_DPFP_BCR: u32 = 0x6c;
pub const ARC_REG_ERP_BUILD: u32 = 0xc7;
pub const ARC_REG_FP_V2_BCR: u32 = 0xc8;
pub const ARC_REG_SLC_BCR: u32 = 0xce;
pub const ARC_REG_DCCM_BUILD: u32 = 0x74;
pub const ARC_REG_AP_BCR: u32 = 0x76;
pub const ARC_REG_ICCM_BUILD: u32 = 0x78;
pub const ARC_REG_XY_MEM_BCR: u32 = 0x79;
pub const ARC_REG_MAC_BCR: u32 = 0x7a;
pub const ARC_REG_MPY_BCR: u32 = 0x7b;
pub const ARC_REG_SWAP_BCR: u32 = 0x7c;
pub const ARC_REG_NORM_BCR: u32 = 0x7d;
pub const ARC_REG_MIXMAX_BCR: u32 = 0x7e;
pub const ARC_REG_BARREL_BCR: u32 = 0x7f;
pub const ARC_REG_D_UNCACH_BCR: u32 = 0x6a;
pub const ARC_REG_BPU_BCR: u32 = 0xc0;
pub const ARC_REG_ISA_CFG_BCR: u32 = 0xc1;
pub const ARC_REG_LPB_BUILD: u32 = 0xe9;
pub const ARC_REG_RTT_BCR: u32 = 0xf2;
pub const ARC_REG_IRQ_BCR: u32 = 0xf3;
pub const ARC_REG_MICRO_ARCH_BCR: u32 = 0xf9;
pub const ARC_REG_SMART_BCR: u32 = 0xff;
pub const ARC_REG_CLUSTER_BCR: u32 = 0xcf;
pub const ARC_REG_AUX_ICCM: u32 = 0x208;
pub const ARC_REG_LPB_CTRL: u32 = 0x488;
pub const ARC_REG_FPU_CTRL: u32 = 0x300;
pub const ARC_REG_FPU_STATUS: u32 = 0x301;
pub const ARC_REG_STATUS32: u32 = 0x0a;

pub const STATUS_AE_BIT: u32 = 5;
pub const STATUS_DE_BIT: u32 = 6;
pub const STATUS_U_BIT: u32 = 7;
pub const STATUS_Z_BIT: u32 = 11;
pub const STATUS_L_BIT: u32 = 12;
pub const STATUS_AE_MASK: u32 = 1 << STATUS_AE_BIT;
pub const STATUS_DE_MASK: u32 = 1 << STATUS_DE_BIT;
pub const STATUS_U_MASK: u32 = 1 << STATUS_U_BIT;
pub const STATUS_Z_MASK: u32 = 1 << STATUS_Z_BIT;
pub const STATUS_L_MASK: u32 = 1 << STATUS_L_BIT;

// CONFIG_ISA_ARCOMPACT selects the ARCompact exception vector values.
pub const ECR_V_MEM_ERR: u32 = 0x01;
pub const ECR_V_INSN_ERR: u32 = 0x02;
#[cfg(feature = "isa_arcompact")]
pub const ECR_V_MACH_CHK: u32 = 0x20;
#[cfg(not(feature = "isa_arcompact"))]
pub const ECR_V_MACH_CHK: u32 = 0x03;
#[cfg(feature = "isa_arcompact")]
pub const ECR_V_ITLB_MISS: u32 = 0x21;
#[cfg(not(feature = "isa_arcompact"))]
pub const ECR_V_ITLB_MISS: u32 = 0x04;
#[cfg(feature = "isa_arcompact")]
pub const ECR_V_DTLB_MISS: u32 = 0x22;
#[cfg(not(feature = "isa_arcompact"))]
pub const ECR_V_DTLB_MISS: u32 = 0x05;
#[cfg(feature = "isa_arcompact")]
pub const ECR_V_PROTV: u32 = 0x23;
#[cfg(not(feature = "isa_arcompact"))]
pub const ECR_V_PROTV: u32 = 0x06;
#[cfg(feature = "isa_arcompact")]
pub const ECR_V_TRAP: u32 = 0x25;
#[cfg(not(feature = "isa_arcompact"))]
pub const ECR_V_TRAP: u32 = 0x09;
#[cfg(not(feature = "isa_arcompact"))]
pub const ECR_V_MISALIGN: u32 = 0x0d;

pub const ECR_C_PROTV_INST_FETCH: u32 = 0;
pub const ECR_C_PROTV_LOAD: u32 = 1;
pub const ECR_C_PROTV_STORE: u32 = 2;
pub const ECR_C_PROTV_XCHG: u32 = 3;
pub const ECR_C_PROTV_MISALIG_DATA: u32 = 4;
pub const ECR_C_BIT_PROTV_MISALIG_DATA: u32 = 10;
pub const ECR_C_MCHK_DUP_TLB: u32 = 1;
pub const ECR_C_BIT_DTLB_LD_MISS: u32 = 8;
pub const ECR_C_BIT_DTLB_ST_MISS: u32 = 9;

pub const AUX_IDENTITY: u32 = 4;
pub const AUX_EXEC_CTRL: u32 = 8;
pub const AUX_INTR_VEC_BASE: u32 = 0x25;
pub const AUX_VOL: u32 = 0x5e;
pub const ARC_AUX_FP_STAT: u32 = 0x300;
pub const ARC_AUX_DPFP_1L: u32 = 0x301;
pub const ARC_AUX_DPFP_1H: u32 = 0x302;
pub const ARC_AUX_DPFP_2L: u32 = 0x303;
pub const ARC_AUX_DPFP_2H: u32 = 0x304;
pub const ARC_AUX_DPFP_STAT: u32 = 0x305;
pub const ARC_AUX_DSP_BUILD: u32 = 0x7a;
pub const ARC_AUX_ACC0_LO: u32 = 0x580;
pub const ARC_AUX_ACC0_GLO: u32 = 0x581;
pub const ARC_AUX_ACC0_HI: u32 = 0x582;
pub const ARC_AUX_ACC0_GHI: u32 = 0x583;
pub const ARC_AUX_DSP_BFLY0: u32 = 0x598;
pub const ARC_AUX_DSP_CTRL: u32 = 0x59f;
pub const ARC_AUX_DSP_FFT_CTRL: u32 = 0x59e;
pub const ARC_AUX_AGU_BUILD: u32 = 0xcc;
pub const ARC_AUX_AGU_AP0: u32 = 0x5c0;
pub const ARC_AUX_AGU_AP1: u32 = 0x5c1;
pub const ARC_AUX_AGU_AP2: u32 = 0x5c2;
pub const ARC_AUX_AGU_AP3: u32 = 0x5c3;
pub const ARC_AUX_AGU_OS0: u32 = 0x5d0;
pub const ARC_AUX_AGU_OS1: u32 = 0x5d1;
pub const ARC_AUX_AGU_MOD0: u32 = 0x5e0;
pub const ARC_AUX_AGU_MOD1: u32 = 0x5e1;
pub const ARC_AUX_AGU_MOD2: u32 = 0x5e2;
pub const ARC_AUX_AGU_MOD3: u32 = 0x5e3;

#[inline]
pub const fn TO_KB(bytes: u32) -> u32 { bytes >> 10 }
#[inline]
pub const fn TO_MB(bytes: u32) -> u32 { TO_KB(bytes) >> 10 }

// C bitfields are represented by their single 32-bit storage word. Field
// order and widths are retained from the source; accessors belong to users.
macro_rules! bcr_word_types {
    ($($name:ident),* $(,)?) => { $(#[repr(C)] #[derive(Copy, Clone)] pub struct $name { pub bits: u32 })* };
}
bcr_word_types!(
    bcr_identity, bcr_isa_arcv2, bcr_uarch_build, bcr_mmu_3, bcr_mmu_4,
    bcr_cache, bcr_slc_cfg, bcr_clust_cfg, bcr_volatile, bcr_mpy,
    bcr_iccm_arcompact, bcr_iccm_arcv2, bcr_dccm_arcompact, bcr_dccm_arcv2,
    bcr_fp_arcompact, bcr_fp_arcv2, bcr_actionpoint, bcr_bpu_arcompact,
    bcr_bpu_arcv2, bcr_erp, ctl_erp, bcr_lpb, bcr_generic
);

// IS_ENABLED(CONFIG_ISA_ARCV2/CONFIG_ISA_ARCOMPACT), supplied by the build.
#[cfg(feature = "isa_arcv2")]
#[inline]
pub const fn is_isa_arcv2() -> i32 { 1 }
#[cfg(not(feature = "isa_arcv2"))]
#[inline]
pub const fn is_isa_arcv2() -> i32 { 0 }
#[cfg(feature = "isa_arcompact")]
#[inline]
pub const fn is_isa_arcompact() -> i32 { 1 }
#[cfg(not(feature = "isa_arcompact"))]
#[inline]
pub const fn is_isa_arcompact() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
