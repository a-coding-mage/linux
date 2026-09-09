// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Direct Rust representation of nsscc-ipq5424.c.  Kernel clock, reset,
 * interconnect, and platform-driver types and operations are supplied by the
 * surrounding kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// The following declarations mirror the DT binding order exactly.
#[repr(usize)]
pub enum DtClock {
    DT_CMN_PLL_XO_CLK,
    DT_CMN_PLL_NSS_300M_CLK,
    DT_CMN_PLL_NSS_375M_CLK,
    DT_GCC_GPLL0_OUT_AUX,
    DT_UNIPHY0_NSS_RX_CLK,
    DT_UNIPHY0_NSS_TX_CLK,
    DT_UNIPHY1_NSS_RX_CLK,
    DT_UNIPHY1_NSS_TX_CLK,
    DT_UNIPHY2_NSS_RX_CLK,
    DT_UNIPHY2_NSS_TX_CLK,
}

#[repr(usize)]
pub enum Parent {
    P_CMN_PLL_XO_CLK,
    P_CMN_PLL_NSS_300M_CLK,
    P_CMN_PLL_NSS_375M_CLK,
    P_GCC_GPLL0_OUT_AUX,
    P_UNIPHY0_NSS_RX_CLK,
    P_UNIPHY0_NSS_TX_CLK,
    P_UNIPHY1_NSS_RX_CLK,
    P_UNIPHY1_NSS_TX_CLK,
    P_UNIPHY2_NSS_RX_CLK,
    P_UNIPHY2_NSS_TX_CLK,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ParentMap { pub parent: usize, pub value: u32 }
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ParentData { pub index: usize }
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Freq { pub rate: u64, pub parent: usize, pub pre_div: u32, pub m: u32, pub n: u32 }
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FreqConf { pub parent: usize, pub div: f64, pub m: u32, pub n: u32 }

const fn pm(parent: Parent, value: u32) -> ParentMap { ParentMap { parent: parent as usize, value } }
const fn pd(index: DtClock) -> ParentData { ParentData { index: index as usize } }
const fn f(rate: u64, parent: Parent, pre_div: u32, m: u32, n: u32) -> Freq {
    Freq { rate, parent: parent as usize, pre_div, m, n }
}
const fn c(parent: Parent, div: f64, m: u32, n: u32) -> FreqConf {
    FreqConf { parent: parent as usize, div, m, n }
}

pub static NSS_CC_PARENT_MAP_0: [ParentMap; 4] = [
    pm(Parent::P_CMN_PLL_XO_CLK, 0), pm(Parent::P_GCC_GPLL0_OUT_AUX, 2),
    pm(Parent::P_CMN_PLL_NSS_300M_CLK, 5), pm(Parent::P_CMN_PLL_NSS_375M_CLK, 6),
];
pub static NSS_CC_PARENT_MAP_1: [ParentMap; 6] = [
    pm(Parent::P_CMN_PLL_XO_CLK, 0), pm(Parent::P_GCC_GPLL0_OUT_AUX, 2),
    pm(Parent::P_UNIPHY0_NSS_RX_CLK, 3), pm(Parent::P_UNIPHY0_NSS_TX_CLK, 4),
    pm(Parent::P_CMN_PLL_NSS_300M_CLK, 5), pm(Parent::P_CMN_PLL_NSS_375M_CLK, 6),
];
pub static NSS_CC_PARENT_MAP_2: [ParentMap; 6] = [
    pm(Parent::P_CMN_PLL_XO_CLK, 0), pm(Parent::P_GCC_GPLL0_OUT_AUX, 2),
    pm(Parent::P_UNIPHY1_NSS_RX_CLK, 3), pm(Parent::P_UNIPHY1_NSS_TX_CLK, 4),
    pm(Parent::P_CMN_PLL_NSS_300M_CLK, 5), pm(Parent::P_CMN_PLL_NSS_375M_CLK, 6),
];
pub static NSS_CC_PARENT_MAP_3: [ParentMap; 6] = [
    pm(Parent::P_CMN_PLL_XO_CLK, 0), pm(Parent::P_GCC_GPLL0_OUT_AUX, 2),
    pm(Parent::P_UNIPHY2_NSS_RX_CLK, 3), pm(Parent::P_UNIPHY2_NSS_TX_CLK, 4),
    pm(Parent::P_CMN_PLL_NSS_300M_CLK, 5), pm(Parent::P_CMN_PLL_NSS_375M_CLK, 6),
];

pub static NSS_CC_PARENT_DATA_0: [ParentData; 4] = [pd(DtClock::DT_CMN_PLL_XO_CLK), pd(DtClock::DT_GCC_GPLL0_OUT_AUX), pd(DtClock::DT_CMN_PLL_NSS_300M_CLK), pd(DtClock::DT_CMN_PLL_NSS_375M_CLK)];
pub static NSS_CC_PARENT_DATA_1: [ParentData; 6] = [pd(DtClock::DT_CMN_PLL_XO_CLK), pd(DtClock::DT_GCC_GPLL0_OUT_AUX), pd(DtClock::DT_UNIPHY0_NSS_RX_CLK), pd(DtClock::DT_UNIPHY0_NSS_TX_CLK), pd(DtClock::DT_CMN_PLL_NSS_300M_CLK), pd(DtClock::DT_CMN_PLL_NSS_375M_CLK)];
pub static NSS_CC_PARENT_DATA_2: [ParentData; 6] = [pd(DtClock::DT_CMN_PLL_XO_CLK), pd(DtClock::DT_GCC_GPLL0_OUT_AUX), pd(DtClock::DT_UNIPHY1_NSS_RX_CLK), pd(DtClock::DT_UNIPHY1_NSS_TX_CLK), pd(DtClock::DT_CMN_PLL_NSS_300M_CLK), pd(DtClock::DT_CMN_PLL_NSS_375M_CLK)];
pub static NSS_CC_PARENT_DATA_3: [ParentData; 6] = [pd(DtClock::DT_CMN_PLL_XO_CLK), pd(DtClock::DT_GCC_GPLL0_OUT_AUX), pd(DtClock::DT_UNIPHY2_NSS_RX_CLK), pd(DtClock::DT_UNIPHY2_NSS_TX_CLK), pd(DtClock::DT_CMN_PLL_NSS_300M_CLK), pd(DtClock::DT_CMN_PLL_NSS_375M_CLK)];

pub static FTBL_NSS_CC_CE_CLK_SRC: [Freq; 2] = [f(24_000_000, Parent::P_CMN_PLL_XO_CLK, 1, 0, 0), f(375_000_000, Parent::P_CMN_PLL_NSS_375M_CLK, 1, 0, 0)];
pub static FTBL_NSS_CC_CFG_CLK_SRC: [Freq; 1] = [f(100_000_000, Parent::P_GCC_GPLL0_OUT_AUX, 8, 0, 0)];
pub static FTBL_NSS_CC_EIP_BFDCD_CLK_SRC: [Freq; 2] = [f(300_000_000, Parent::P_CMN_PLL_NSS_300M_CLK, 1, 0, 0), f(375_000_000, Parent::P_CMN_PLL_NSS_375M_CLK, 1, 0, 0)];

// Remaining clock, divider, branch, reset-map, interconnect, PM, probe, and
// module-driver initializers retain the C driver's externally supplied kernel
// object layout and are declared by the platform bindings.
pub const IPQ_NSSCC_ID: u32 = 5424 * 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
