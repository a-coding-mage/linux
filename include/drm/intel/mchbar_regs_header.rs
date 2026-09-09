/* SPDX-License-Identifier: MIT */
/* Copyright © 2022 Intel Corporation */

/* MCHBAR mirror; the original C header's include guard is omitted. */

pub const MCHBAR_MIRROR_BASE: u32 = 0x10000;
pub const MCHBAR_MIRROR_END: u32 = 0x13fff;
pub const MCHBAR_MIRROR_BASE_SNB: u32 = 0x140000;
pub const MCHBAR_MIRROR_END_SNB: u32 = 0x147fff;
pub const MCHBAR_MIRROR_END_ICL_RKL: u32 = 0x14ffff;
pub const MCHBAR_MIRROR_END_TGL: u32 = 0x15ffff;

pub const CTG_STOLEN_RESERVED: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x34);
pub const ELK_STOLEN_RESERVED: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x48);
pub const G4X_STOLEN_RESERVED_ADDR1_MASK: u32 = 0xFFFF << 16;
pub const G4X_STOLEN_RESERVED_ADDR2_MASK: u32 = 0xFFF << 4;
pub const G4X_STOLEN_RESERVED_ENABLE: u32 = 1 << 0;

/* Pineview MCH register contains DDR3 setting */
pub const CSHRDDR3CTL: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x1a8);
pub const CSHRDDR3CTL_DDR3: u32 = 1 << 2;

/* 915-945 and GM965 MCH register controlling DRAM channel access */
pub const DCC: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x200);
pub const DCC_ADDRESSING_MODE_SINGLE_CHANNEL: u32 = 0 << 0;
pub const DCC_ADDRESSING_MODE_DUAL_CHANNEL_ASYMMETRIC: u32 = 1 << 0;
pub const DCC_ADDRESSING_MODE_DUAL_CHANNEL_INTERLEAVED: u32 = 2 << 0;
pub const DCC_ADDRESSING_MODE_MASK: u32 = 3 << 0;
pub const DCC_CHANNEL_XOR_DISABLE: u32 = 1 << 10;
pub const DCC_CHANNEL_XOR_BIT_17: u32 = 1 << 9;
pub const DCC2: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x204);
pub const DCC2_MODIFIED_ENHANCED_DISABLE: u32 = 1 << 20;

/* 965 MCH register controlling DRAM channel configuration */
pub const C0DRB3_BW: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x206);
pub const C1DRB3_BW: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x606);

/* Clocking configuration register */
pub const CLKCFG: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0xc00);
pub const CLKCFG_FSB_400: u32 = 0 << 0; /* hrawclk 100 */
pub const CLKCFG_FSB_400_ALT: u32 = 5 << 0; /* hrawclk 100 */
pub const CLKCFG_FSB_533: u32 = 1 << 0; /* hrawclk 133 */
pub const CLKCFG_FSB_667: u32 = 3 << 0; /* hrawclk 166 */
pub const CLKCFG_FSB_800: u32 = 2 << 0; /* hrawclk 200 */
pub const CLKCFG_FSB_1067: u32 = 6 << 0; /* hrawclk 266 */
pub const CLKCFG_FSB_1067_ALT: u32 = 0 << 0; /* hrawclk 266 */
pub const CLKCFG_FSB_1333: u32 = 7 << 0; /* hrawclk 333 */
pub const CLKCFG_FSB_1333_ALT: u32 = 4 << 0; /* hrawclk 333 */
pub const CLKCFG_FSB_1600_ALT: u32 = 6 << 0; /* hrawclk 400 */
pub const CLKCFG_FSB_MASK: u32 = 7 << 0;
pub const CLKCFG_MEM_533: u32 = 1 << 4;
pub const CLKCFG_MEM_667: u32 = 2 << 4;
pub const CLKCFG_MEM_800: u32 = 3 << 4;
pub const CLKCFG_MEM_MASK: u32 = 7 << 4;

pub const HPLLVCO_MOBILE: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0xc0f);
pub const HPLLVCO: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0xc38);
pub const TSC1: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x1001);
pub const TSE: u32 = 1 << 0;
pub const TR1: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x1006);
pub const TSFS: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x1020);
pub const TSFS_SLOPE_MASK: u32 = 0x0000ff00;
pub const TSFS_SLOPE_SHIFT: u32 = 8;
pub const TSFS_INTR_MASK: u32 = 0x000000ff;

/* Memory latency timer register; self-refresh latency unit is 0.5us. */
pub const MLTR_ILK: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x1222);
pub const MLTR_WM2_MASK: u32 = REG_GENMASK!(13, 8);
pub const MLTR_WM1_MASK: u32 = REG_GENMASK!(5, 0);

pub const CSIPLL0: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x2c10);
pub const DDRMPLL1: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x2c20);
pub const ILK_GDSR: u32 = _MMIO!(MCHBAR_MIRROR_BASE + 0x2ca4);
pub const ILK_GRDOM_FULL: u32 = 0 << 1;
pub const ILK_GRDOM_RENDER: u32 = 1 << 1;
pub const ILK_GRDOM_MEDIA: u32 = 3 << 1;
pub const ILK_GRDOM_MASK: u32 = 3 << 1;
pub const ILK_GRDOM_RESET_ENABLE: u32 = 1 << 0;

pub const BXT_D_CR_DRP0_DUNIT8: u32 = 0x1000;
pub const BXT_D_CR_DRP0_DUNIT9: u32 = 0x1200;
pub const BXT_D_CR_DRP0_DUNIT_START: u32 = 8;
pub const BXT_D_CR_DRP0_DUNIT_END: u32 = 11;
macro_rules! BXT_D_CR_DRP0_DUNIT {
    ($x:expr) => { _MMIO!(MCHBAR_MIRROR_BASE_SNB + _PICK_EVEN!(($x) - 8, BXT_D_CR_DRP0_DUNIT8, BXT_D_CR_DRP0_DUNIT9)) };
}
pub const BXT_DRAM_RANK_MASK: u32 = 0x3;
pub const BXT_DRAM_RANK_SINGLE: u32 = 0x1;
pub const BXT_DRAM_RANK_DUAL: u32 = 0x3;
pub const BXT_DRAM_WIDTH_MASK: u32 = 0x3 << 4;
pub const BXT_DRAM_WIDTH_SHIFT: u32 = 4;
pub const BXT_DRAM_WIDTH_X8: u32 = 0x0 << 4;
pub const BXT_DRAM_WIDTH_X16: u32 = 0x1 << 4;
pub const BXT_DRAM_WIDTH_X32: u32 = 0x2 << 4;
pub const BXT_DRAM_WIDTH_X64: u32 = 0x3 << 4;
pub const BXT_DRAM_SIZE_MASK: u32 = 0x7 << 6;
pub const BXT_DRAM_SIZE_SHIFT: u32 = 6;
pub const BXT_DRAM_SIZE_4GBIT: u32 = 0x0 << 6;
pub const BXT_DRAM_SIZE_6GBIT: u32 = 0x1 << 6;
pub const BXT_DRAM_SIZE_8GBIT: u32 = 0x2 << 6;
pub const BXT_DRAM_SIZE_12GBIT: u32 = 0x3 << 6;
pub const BXT_DRAM_SIZE_16GBIT: u32 = 0x4 << 6;
pub const BXT_DRAM_TYPE_MASK: u32 = 0x7 << 22;
pub const BXT_DRAM_TYPE_SHIFT: u32 = 22;
pub const BXT_DRAM_TYPE_DDR3: u32 = 0x0 << 22;
pub const BXT_DRAM_TYPE_LPDDR3: u32 = 0x1 << 22;
pub const BXT_DRAM_TYPE_LPDDR4: u32 = 0x2 << 22;
pub const BXT_DRAM_TYPE_DDR4: u32 = 0x4 << 22;

pub const MCHBAR_CH0_CR_TC_PRE_0_0_0_MCHBAR: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x4000);
pub const DG1_DRAM_T_RDPRE_MASK: u32 = REG_GENMASK!(16, 11);
pub const DG1_DRAM_T_RP_MASK: u32 = REG_GENMASK!(6, 0);
pub const MCHBAR_CH0_CR_TC_PRE_0_0_0_MCHBAR_HIGH: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x4004);
pub const DG1_DRAM_T_RCD_MASK: u32 = REG_GENMASK!(15, 9);
pub const DG1_DRAM_T_RAS_MASK: u32 = REG_GENMASK!(8, 1);

pub const SKL_MAD_INTER_CHANNEL_0_0_0_MCHBAR_MCMAIN: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5000);
pub const SKL_DRAM_DDR_TYPE_MASK: u32 = REG_GENMASK!(1, 0);
pub const SKL_DRAM_DDR_TYPE_DDR4: u32 = REG_FIELD_PREP!(SKL_DRAM_DDR_TYPE_MASK, 0);
pub const SKL_DRAM_DDR_TYPE_DDR3: u32 = REG_FIELD_PREP!(SKL_DRAM_DDR_TYPE_MASK, 1);
pub const SKL_DRAM_DDR_TYPE_LPDDR3: u32 = REG_FIELD_PREP!(SKL_DRAM_DDR_TYPE_MASK, 2);
pub const SKL_DRAM_DDR_TYPE_LPDDR4: u32 = REG_FIELD_PREP!(SKL_DRAM_DDR_TYPE_MASK, 3);

/* snb MCH registers for reading the DRAM channel configuration */
pub const MAD_DIMM_C0: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5004);
pub const MAD_DIMM_C1: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5008);
pub const MAD_DIMM_C2: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x500C);
pub const MAD_DIMM_ECC_MASK: u32 = 0x3 << 24;
pub const MAD_DIMM_ECC_OFF: u32 = 0x0 << 24;
pub const MAD_DIMM_ECC_IO_ON_LOGIC_OFF: u32 = 0x1 << 24;
pub const MAD_DIMM_ECC_IO_OFF_LOGIC_ON: u32 = 0x2 << 24;
pub const MAD_DIMM_ECC_ON: u32 = 0x3 << 24;
pub const MAD_DIMM_ENH_INTERLEAVE: u32 = 0x1 << 22;
pub const MAD_DIMM_RANK_INTERLEAVE: u32 = 0x1 << 21;
pub const MAD_DIMM_B_WIDTH_X16: u32 = 0x1 << 20; /* X8 chips if unset */
pub const MAD_DIMM_A_WIDTH_X16: u32 = 0x1 << 19; /* X8 chips if unset */
pub const MAD_DIMM_B_DUAL_RANK: u32 = 0x1 << 18;
pub const MAD_DIMM_A_DUAL_RANK: u32 = 0x1 << 17;
pub const MAD_DIMM_A_SELECT: u32 = 0x1 << 16;
/* DIMM sizes are in multiples of 256mb. */
pub const MAD_DIMM_B_SIZE_SHIFT: u32 = 8;
pub const MAD_DIMM_B_SIZE_MASK: u32 = 0xff << MAD_DIMM_B_SIZE_SHIFT;
pub const MAD_DIMM_A_SIZE_SHIFT: u32 = 0;
pub const MAD_DIMM_A_SIZE_MASK: u32 = 0xff << MAD_DIMM_A_SIZE_SHIFT;

pub const SKL_MAD_DIMM_CH0_0_0_0_MCHBAR_MCMAIN: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x500C);
pub const SKL_MAD_DIMM_CH1_0_0_0_MCHBAR_MCMAIN: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5010);
pub const SKL_DIMM_S_RANK_MASK: u32 = REG_GENMASK!(26, 26);
pub const SKL_DIMM_S_RANK_1: u32 = REG_FIELD_PREP!(SKL_DIMM_S_RANK_MASK, 0);
pub const SKL_DIMM_S_RANK_2: u32 = REG_FIELD_PREP!(SKL_DIMM_S_RANK_MASK, 1);
pub const SKL_DIMM_S_WIDTH_MASK: u32 = REG_GENMASK!(25, 24);
pub const SKL_DIMM_S_WIDTH_X8: u32 = REG_FIELD_PREP!(SKL_DIMM_S_WIDTH_MASK, 0);
pub const SKL_DIMM_S_WIDTH_X16: u32 = REG_FIELD_PREP!(SKL_DIMM_S_WIDTH_MASK, 1);
pub const SKL_DIMM_S_WIDTH_X32: u32 = REG_FIELD_PREP!(SKL_DIMM_S_WIDTH_MASK, 2);
pub const SKL_DIMM_S_SIZE_MASK: u32 = REG_GENMASK!(21, 16);
pub const SKL_DIMM_L_RANK_MASK: u32 = REG_GENMASK!(10, 10);
pub const SKL_DIMM_L_RANK_1: u32 = REG_FIELD_PREP!(SKL_DIMM_L_RANK_MASK, 0);
pub const SKL_DIMM_L_RANK_2: u32 = REG_FIELD_PREP!(SKL_DIMM_L_RANK_MASK, 1);
pub const SKL_DIMM_L_WIDTH_MASK: u32 = REG_GENMASK!(9, 8);
pub const SKL_DIMM_L_WIDTH_X8: u32 = REG_FIELD_PREP!(SKL_DIMM_L_WIDTH_MASK, 0);
pub const SKL_DIMM_L_WIDTH_X16: u32 = REG_FIELD_PREP!(SKL_DIMM_L_WIDTH_MASK, 1);
pub const SKL_DIMM_L_WIDTH_X32: u32 = REG_FIELD_PREP!(SKL_DIMM_L_WIDTH_MASK, 2);
pub const SKL_DIMM_L_SIZE_MASK: u32 = REG_GENMASK!(5, 0);
pub const ICL_DIMM_S_RANK_MASK: u32 = REG_GENMASK!(27, 26);
pub const ICL_DIMM_S_RANK_1: u32 = REG_FIELD_PREP!(ICL_DIMM_S_RANK_MASK, 0);
pub const ICL_DIMM_S_RANK_2: u32 = REG_FIELD_PREP!(ICL_DIMM_S_RANK_MASK, 1);
pub const ICL_DIMM_S_WIDTH_MASK: u32 = REG_GENMASK!(25, 24);
pub const ICL_DIMM_S_WIDTH_X8: u32 = REG_FIELD_PREP!(ICL_DIMM_S_WIDTH_MASK, 0);
pub const ICL_DIMM_S_WIDTH_X16: u32 = REG_FIELD_PREP!(ICL_DIMM_S_WIDTH_MASK, 1);
pub const ICL_DIMM_S_WIDTH_X32: u32 = REG_FIELD_PREP!(ICL_DIMM_S_WIDTH_MASK, 2);
pub const ICL_DIMM_S_SIZE_MASK: u32 = REG_GENMASK!(22, 16);
pub const ICL_DIMM_L_RANK_MASK: u32 = REG_GENMASK!(10, 9);
pub const ICL_DIMM_L_RANK_1: u32 = REG_FIELD_PREP!(ICL_DIMM_L_RANK_MASK, 0);
pub const ICL_DIMM_L_RANK_2: u32 = REG_FIELD_PREP!(ICL_DIMM_L_RANK_MASK, 1);
pub const ICL_DIMM_L_RANK_3: u32 = REG_FIELD_PREP!(ICL_DIMM_L_RANK_MASK, 2);
pub const ICL_DIMM_L_RANK_4: u32 = REG_FIELD_PREP!(ICL_DIMM_L_RANK_MASK, 3);
pub const ICL_DIMM_L_WIDTH_MASK: u32 = REG_GENMASK!(8, 7);
pub const ICL_DIMM_L_WIDTH_X8: u32 = REG_FIELD_PREP!(ICL_DIMM_L_WIDTH_MASK, 0);
pub const ICL_DIMM_L_WIDTH_X16: u32 = REG_FIELD_PREP!(ICL_DIMM_L_WIDTH_MASK, 1);
pub const ICL_DIMM_L_WIDTH_X32: u32 = REG_FIELD_PREP!(ICL_DIMM_L_WIDTH_MASK, 2);
pub const ICL_DIMM_L_SIZE_MASK: u32 = REG_GENMASK!(6, 0);

pub const SA_PERF_STATUS_0_0_0_MCHBAR_PC: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5918);
pub const DG1_QCLK_RATIO_MASK: u32 = REG_GENMASK!(9, 2);
pub const DG1_QCLK_REFERENCE: u32 = REG_BIT!(10);

/* *_PACKAGE_POWER_SKU - SKU power and timing parameters. */
pub const PCU_PACKAGE_POWER_SKU: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5930);
pub const PKG_PKG_TDP: u64 = GENMASK_ULL!(14, 0);
pub const PKG_MIN_PWR: u64 = GENMASK_ULL!(30, 16);
pub const PKG_MAX_PWR: u64 = GENMASK_ULL!(46, 32);
pub const PKG_MAX_WIN: u64 = GENMASK_ULL!(54, 48);
pub const PKG_MAX_WIN_X: u64 = GENMASK_ULL!(54, 53);
pub const PKG_MAX_WIN_Y: u64 = GENMASK_ULL!(52, 48);
pub const PCU_PACKAGE_POWER_SKU_UNIT: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5938);
pub const PKG_PWR_UNIT: u32 = REG_GENMASK!(3, 0);
pub const PKG_ENERGY_UNIT: u32 = REG_GENMASK!(12, 8);
pub const PKG_TIME_UNIT: u32 = REG_GENMASK!(19, 16);
pub const PCU_PACKAGE_ENERGY_STATUS: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x593c);
pub const GEN6_GT_PERF_STATUS: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5948);
pub const PCU_PACKAGE_TEMPERATURE: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5978);
pub const TEMP_MASK: u32 = REG_GENMASK!(7, 0);
pub const GEN6_RP_STATE_LIMITS: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5994);
pub const GEN6_RP_STATE_CAP: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5998);
pub const RP0_CAP_MASK: u32 = REG_GENMASK!(7, 0);
pub const RP1_CAP_MASK: u32 = REG_GENMASK!(15, 8);
pub const RPN_CAP_MASK: u32 = REG_GENMASK!(23, 16);
pub const GEN10_FREQ_INFO_REC: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5ef0);
pub const RPE_MASK: u32 = REG_GENMASK!(15, 8);
pub const PCU_PACKAGE_RAPL_LIMIT: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x59a0);
pub const PKG_PWR_LIM_1: u32 = REG_GENMASK!(14, 0);
pub const PKG_PWR_LIM_1_EN: u32 = REG_BIT!(15);
pub const PKG_PWR_LIM_1_TIME: u32 = REG_GENMASK!(23, 17);
pub const PKG_PWR_LIM_1_TIME_X: u32 = REG_GENMASK!(23, 22);
pub const PKG_PWR_LIM_1_TIME_Y: u32 = REG_GENMASK!(21, 17);

/* snb MCH registers for priority tuning */
pub const MCH_SSKPD: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5d10);
pub const SSKPD_NEW_WM0_MASK_HSW: u64 = REG_GENMASK64!(63, 56);
pub const SSKPD_WM4_MASK_HSW: u64 = REG_GENMASK64!(40, 32);
pub const SSKPD_WM3_MASK_HSW: u64 = REG_GENMASK64!(28, 20);
pub const SSKPD_WM2_MASK_HSW: u64 = REG_GENMASK64!(19, 12);
pub const SSKPD_WM1_MASK_HSW: u64 = REG_GENMASK64!(11, 4);
pub const SSKPD_OLD_WM0_MASK_HSW: u64 = REG_GENMASK64!(3, 0);
pub const SSKPD_WM3_MASK_SNB: u32 = REG_GENMASK!(29, 24);
pub const SSKPD_WM2_MASK_SNB: u32 = REG_GENMASK!(21, 16);
pub const SSKPD_WM1_MASK_SNB: u32 = REG_GENMASK!(13, 8);
pub const SSKPD_WM0_MASK_SNB: u32 = REG_GENMASK!(5, 0);

/* Memory controller frequency in MCHBAR for Haswell (possible SNB+) */
pub const DCLK: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5e04);
pub const SKL_MC_BIOS_DATA_0_0_0_MCHBAR_PCU: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5e04);
pub const DG1_GEAR_TYPE: u32 = REG_BIT!(16);

/* See hsw_read_dcomp() and hsw_write_dcomp() before using this register. */
pub const D_COMP_HSW: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x5f0c);
pub const D_COMP_RCOMP_IN_PROGRESS: u32 = 1 << 9;
pub const D_COMP_COMP_FORCE: u32 = 1 << 8;
pub const D_COMP_COMP_DISABLE: u32 = 1 << 0;
pub const BXT_GT_PERF_STATUS: u32 = _MMIO!(MCHBAR_MIRROR_BASE_SNB + 0x7070);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
