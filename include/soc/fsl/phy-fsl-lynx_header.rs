/* SPDX-License-Identifier: GPL-2.0+ */
/* Copyright 2023-2026 NXP */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lynx_lane_mode {
    LANE_MODE_UNKNOWN,
    LANE_MODE_1000BASEX_SGMII,
    LANE_MODE_2500BASEX,
    LANE_MODE_QSGMII,
    LANE_MODE_10G_QXGMII,
    LANE_MODE_10GBASER,
    LANE_MODE_USXGMII,
    LANE_MODE_25GBASER,
    LANE_MODE_MAX,
}

pub const fn lynx_lane_mode_uses_gmii_mac(mode: lynx_lane_mode) -> bool {
    match mode {
        lynx_lane_mode::LANE_MODE_1000BASEX_SGMII
        | lynx_lane_mode::LANE_MODE_2500BASEX
        | lynx_lane_mode::LANE_MODE_QSGMII
        | lynx_lane_mode::LANE_MODE_10G_QXGMII => true,
        _ => false,
    }
}

pub const fn lynx_lane_mode_uses_xgmii_mac(mode: lynx_lane_mode) -> bool {
    match mode {
        lynx_lane_mode::LANE_MODE_10GBASER | lynx_lane_mode::LANE_MODE_USXGMII => true,
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
