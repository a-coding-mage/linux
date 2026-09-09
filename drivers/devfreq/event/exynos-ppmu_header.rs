/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * exynos_ppmu.h - Exynos PPMU header file
 *
 * Copyright (c) 2015 Samsung Electronics Co., Ltd.
 * Author : Chanwoo Choi <cw00.choi@samsung.com>
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ppmu_state {
    PPMU_DISABLE = 0,
    PPMU_ENABLE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ppmu_counter {
    PPMU_PMNCNT0 = 0,
    PPMU_PMNCNT1,
    PPMU_PMNCNT2,
    PPMU_PMNCNT3,
    PPMU_PMNCNT_MAX,
}

/* PPMUv1.1 Definitions */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ppmu_event_type {
    PPMU_RO_BUSY_CYCLE_CNT = 0x0,
    PPMU_WO_BUSY_CYCLE_CNT = 0x1,
    PPMU_RW_BUSY_CYCLE_CNT = 0x2,
    PPMU_RO_REQUEST_CNT = 0x3,
    PPMU_WO_REQUEST_CNT = 0x4,
    PPMU_RO_DATA_CNT = 0x5,
    PPMU_WO_DATA_CNT = 0x6,
    PPMU_RO_LATENCY = 0x12,
    PPMU_WO_LATENCY = 0x16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ppmu_reg {
    PPMU_PMNC = 0x00,
    PPMU_CNTENS = 0x10,
    PPMU_CNTENC = 0x20,
    PPMU_INTENS = 0x30,
    PPMU_INTENC = 0x40,
    PPMU_FLAG = 0x50,
    PPMU_CCNT = 0x100,
    PPMU_PMCNT0 = 0x110,
    PPMU_PMCNT1 = 0x120,
    PPMU_PMCNT2 = 0x130,
    PPMU_PMCNT3_HIGH = 0x140,
    PPMU_PMCNT3_LOW = 0x150,
    PPMU_BEVT0SEL = 0x1000,
    PPMU_BEVT1SEL = 0x1100,
    PPMU_BEVT2SEL = 0x1200,
    PPMU_BEVT3SEL = 0x1300,
    PPMU_COUNTER_RESET = 0x1810,
    PPMU_READ_OVERFLOW_CNT = 0x1810,
    PPMU_READ_UNDERFLOW_CNT = 0x1814,
    PPMU_WRITE_OVERFLOW_CNT = 0x1850,
    PPMU_WRITE_UNDERFLOW_CNT = 0x1854,
    PPMU_READ_PENDING_CNT = 0x1880,
    PPMU_WRITE_PENDING_CNT = 0x1884,
}

pub const PPMU_PMNC_CC_RESET_SHIFT: u32 = 2;
pub const PPMU_PMNC_COUNTER_RESET_SHIFT: u32 = 1;
pub const PPMU_PMNC_ENABLE_SHIFT: u32 = 0;
pub const PPMU_PMNC_START_MODE_MASK: u32 = 1u32 << 16;
pub const PPMU_PMNC_CC_DIVIDER_MASK: u32 = 1u32 << 3;
pub const PPMU_PMNC_CC_RESET_MASK: u32 = 1u32 << 2;
pub const PPMU_PMNC_COUNTER_RESET_MASK: u32 = 1u32 << 1;
pub const PPMU_PMNC_ENABLE_MASK: u32 = 1u32 << 0;

pub const PPMU_CCNT_MASK: u32 = 1u32 << 31;
pub const PPMU_PMCNT3_MASK: u32 = 1u32 << 3;
pub const PPMU_PMCNT2_MASK: u32 = 1u32 << 2;
pub const PPMU_PMCNT1_MASK: u32 = 1u32 << 1;
pub const PPMU_PMCNT0_MASK: u32 = 1u32 << 0;

#[inline]
pub const fn PPMU_PMNCT(x: u32) -> u32 { PPMU_PMCNT0 as u32 + 0x10 * x }
#[inline]
pub const fn PPMU_BEVTxSEL(x: u32) -> u32 { PPMU_BEVT0SEL as u32 + 0x100 * x }

/* PPMU_V2.0 definitions */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ppmu_v2_mode {
    PPMU_V2_MODE_MANUAL = 0,
    PPMU_V2_MODE_AUTO = 1,
    PPMU_V2_MODE_CIG = 2, /* CIG (Conditional Interrupt Generation) */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ppmu_v2_event_type {
    PPMU_V2_RO_DATA_CNT = 0x4,
    PPMU_V2_WO_DATA_CNT = 0x5,
    PPMU_V2_EVT3_RW_DATA_CNT = 0x22, /* Only for Event3 */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ppmu_V2_reg {
    PPMU_V2_PMNC = 0x04,
    PPMU_V2_CNTENS = 0x08,
    PPMU_V2_CNTENC = 0x0c,
    PPMU_V2_INTENS = 0x10,
    PPMU_V2_INTENC = 0x14,
    PPMU_V2_FLAG = 0x18,
    PPMU_V2_CCNT = 0x48,
    PPMU_V2_PMCNT0 = 0x34,
    PPMU_V2_PMCNT1 = 0x38,
    PPMU_V2_PMCNT2 = 0x3c,
    PPMU_V2_PMCNT3_LOW = 0x40,
    PPMU_V2_PMCNT3_HIGH = 0x44,
    PPMU_V2_CIG_CFG0 = 0x1c,
    PPMU_V2_CIG_CFG1 = 0x20,
    PPMU_V2_CIG_CFG2 = 0x24,
    PPMU_V2_CIG_RESULT = 0x28,
    PPMU_V2_CNT_RESET = 0x2c,
    PPMU_V2_CNT_AUTO = 0x30,
    PPMU_V2_CH_EV0_TYPE = 0x200,
    PPMU_V2_CH_EV1_TYPE = 0x204,
    PPMU_V2_CH_EV2_TYPE = 0x208,
    PPMU_V2_CH_EV3_TYPE = 0x20c,
    PPMU_V2_SM_ID_V = 0x220,
    PPMU_V2_SM_ID_A = 0x224,
    PPMU_V2_SM_OTHERS_V = 0x228,
    PPMU_V2_SM_OTHERS_A = 0x22c,
    PPMU_V2_INTERRUPT_RESET = 0x260,
}

pub const PPMU_V2_PMNC_START_MODE_SHIFT: u32 = 20;
pub const PPMU_V2_PMNC_START_MODE_MASK: u32 = 0x3 << PPMU_V2_PMNC_START_MODE_SHIFT;

#[inline]
pub const fn PPMU_V2_PMNCT(x: u32) -> u32 { PPMU_V2_PMCNT0 as u32 + 0x4 * x }
#[inline]
pub const fn PPMU_V2_CH_EVx_TYPE(x: u32) -> u32 { PPMU_V2_CH_EV0_TYPE as u32 + 0x4 * x }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
