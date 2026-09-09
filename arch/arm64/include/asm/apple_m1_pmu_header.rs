// SPDX-License-Identifier: GPL-2.0

// Translated from the C header. The original dependencies supplied the
// `sys_reg`, `BIT`, and `GENMASK` macros.

/* Counters */
pub const SYS_IMP_APL_PMC0_EL1: u64 = sys_reg(3, 2, 15, 0, 0);
pub const SYS_IMP_APL_PMC1_EL1: u64 = sys_reg(3, 2, 15, 1, 0);
pub const SYS_IMP_APL_PMC2_EL1: u64 = sys_reg(3, 2, 15, 2, 0);
pub const SYS_IMP_APL_PMC3_EL1: u64 = sys_reg(3, 2, 15, 3, 0);
pub const SYS_IMP_APL_PMC4_EL1: u64 = sys_reg(3, 2, 15, 4, 0);
pub const SYS_IMP_APL_PMC5_EL1: u64 = sys_reg(3, 2, 15, 5, 0);
pub const SYS_IMP_APL_PMC6_EL1: u64 = sys_reg(3, 2, 15, 6, 0);
pub const SYS_IMP_APL_PMC7_EL1: u64 = sys_reg(3, 2, 15, 7, 0);
pub const SYS_IMP_APL_PMC8_EL1: u64 = sys_reg(3, 2, 15, 9, 0);
pub const SYS_IMP_APL_PMC9_EL1: u64 = sys_reg(3, 2, 15, 10, 0);

/* Core PMC control register */
pub const SYS_IMP_APL_PMCR0_EL1: u64 = sys_reg(3, 1, 15, 0, 0);
pub const PMCR0_CNT_ENABLE_0_7: u64 = GENMASK(7, 0);
pub const PMCR0_IMODE: u64 = GENMASK(10, 8);
pub const PMCR0_IMODE_OFF: u64 = 0;
pub const PMCR0_IMODE_PMI: u64 = 1;
pub const PMCR0_IMODE_AIC: u64 = 2;
pub const PMCR0_IMODE_HALT: u64 = 3;
pub const PMCR0_IMODE_FIQ: u64 = 4;
pub const PMCR0_IACT: u64 = BIT(11);
pub const PMCR0_PMI_ENABLE_0_7: u64 = GENMASK(19, 12);
pub const PMCR0_STOP_CNT_ON_PMI: u64 = BIT(20);
pub const PMCR0_CNT_GLOB_L2C_EVT: u64 = BIT(21);
pub const PMCR0_DEFER_PMI_TO_ERET: u64 = BIT(22);
pub const PMCR0_ALLOW_CNT_EN_EL0: u64 = BIT(30);
pub const PMCR0_CNT_ENABLE_8_9: u64 = GENMASK(33, 32);
pub const PMCR0_PMI_ENABLE_8_9: u64 = GENMASK(45, 44);

pub const SYS_IMP_APL_PMCR1_EL1: u64 = sys_reg(3, 1, 15, 1, 0);
pub const SYS_IMP_APL_PMCR1_EL12: u64 = sys_reg(3, 1, 15, 7, 2);
pub const PMCR1_COUNT_A64_EL0_0_7: u64 = GENMASK(15, 8);
pub const PMCR1_COUNT_A64_EL1_0_7: u64 = GENMASK(23, 16);
pub const PMCR1_COUNT_A64_EL0_8_9: u64 = GENMASK(41, 40);
pub const PMCR1_COUNT_A64_EL1_8_9: u64 = GENMASK(49, 48);

pub const SYS_IMP_APL_PMCR2_EL1: u64 = sys_reg(3, 1, 15, 2, 0);
pub const SYS_IMP_APL_PMCR3_EL1: u64 = sys_reg(3, 1, 15, 3, 0);
pub const SYS_IMP_APL_PMCR4_EL1: u64 = sys_reg(3, 1, 15, 4, 0);

pub const SYS_IMP_APL_PMESR0_EL1: u64 = sys_reg(3, 1, 15, 5, 0);
pub const PMESR0_EVT_CNT_2: u64 = GENMASK(7, 0);
pub const PMESR0_EVT_CNT_3: u64 = GENMASK(15, 8);
pub const PMESR0_EVT_CNT_4: u64 = GENMASK(23, 16);
pub const PMESR0_EVT_CNT_5: u64 = GENMASK(31, 24);

pub const SYS_IMP_APL_PMESR1_EL1: u64 = sys_reg(3, 1, 15, 6, 0);
pub const PMESR1_EVT_CNT_6: u64 = GENMASK(7, 0);
pub const PMESR1_EVT_CNT_7: u64 = GENMASK(15, 8);
pub const PMESR1_EVT_CNT_8: u64 = GENMASK(23, 16);
pub const PMESR1_EVT_CNT_9: u64 = GENMASK(31, 24);

pub const SYS_IMP_APL_PMSR_EL1: u64 = sys_reg(3, 1, 15, 13, 0);
pub const PMSR_OVERFLOW: u64 = GENMASK(9, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
