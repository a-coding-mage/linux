/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2022,2024 NXP
 */

// Dependency: linux/types.h supplies the C integer types used by this header.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum phy_hdmi_mode {
    PHY_HDMI_MODE_TMDS,
    PHY_HDMI_MODE_FRL,
}

/**
 * struct phy_configure_opts_hdmi - HDMI configuration set
 * @bpc: Bits per color channel.
 * @tmds_char_rate: HDMI TMDS Character Rate in Hertz.
 * @frl.rate_per_lane: HDMI FRL Rate per Lane in Gbps.
 * @frl.lanes: HDMI FRL lanes count.
 *
 * This structure is used to represent the configuration state of a HDMI phy.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_configure_opts_hdmi {
    pub bpc: u32,
    pub union_: phy_configure_opts_hdmi__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union phy_configure_opts_hdmi__bindgen_ty_1 {
    pub tmds_char_rate: u64,
    pub frl: phy_configure_opts_hdmi__bindgen_ty_1__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_configure_opts_hdmi__bindgen_ty_1__bindgen_ty_1 {
    pub rate_per_lane: u8,
    pub lanes: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
