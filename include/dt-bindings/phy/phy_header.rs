/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * This header provides constants for the phy framework
 *
 * Copyright (C) 2014 STMicroelectronics
 * Author: Gabriel Fernandez <gabriel.fernandez@st.com>
 */

pub const PHY_NONE: i32 = 0;
pub const PHY_TYPE_SATA: i32 = 1;
pub const PHY_TYPE_PCIE: i32 = 2;
pub const PHY_TYPE_USB2: i32 = 3;
pub const PHY_TYPE_USB3: i32 = 4;
pub const PHY_TYPE_UFS: i32 = 5;
pub const PHY_TYPE_DP: i32 = 6;
pub const PHY_TYPE_XPCS: i32 = 7;
pub const PHY_TYPE_SGMII: i32 = 8;
pub const PHY_TYPE_QSGMII: i32 = 9;
pub const PHY_TYPE_DPHY: i32 = 10;
pub const PHY_TYPE_CPHY: i32 = 11;
pub const PHY_TYPE_USXGMII: i32 = 12;
pub const PHY_TYPE_XAUI: i32 = 13;

pub const PHY_POL_NORMAL: i32 = 0;
pub const PHY_POL_INVERT: i32 = 1;
pub const PHY_POL_AUTO: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
