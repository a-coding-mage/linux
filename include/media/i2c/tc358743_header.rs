/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tc358743 - Toshiba HDMI to CSI-2 bridge
 *
 * Copyright 2015 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

/*
 * References (c = chapter, p = page):
 * REF_01 - Toshiba, TC358743XBG (H2C), Functional Specification, Rev 0.60
 * REF_02 - Toshiba, TC358743XBG_HDMI-CSI_Tv11p_nm.xls
 */

/* C header guard: _TC358743_ */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tc358743_ddc5v_delays {
    DDC5V_DELAY_0_MS,
    DDC5V_DELAY_50_MS,
    DDC5V_DELAY_100_MS,
    DDC5V_DELAY_200_MS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tc358743_hdmi_detection_delay {
    HDMI_MODE_DELAY_0_MS,
    HDMI_MODE_DELAY_25_MS,
    HDMI_MODE_DELAY_50_MS,
    HDMI_MODE_DELAY_100_MS,
}

#[repr(C)]
pub struct tc358743_platform_data {
    /* System clock connected to REFCLK (pin H5) */
    pub refclk_hz: u32, /* 26 MHz, 27 MHz or 42 MHz */

    /* DDC +5V debounce delay to avoid spurious interrupts when the cable
     * is connected.
     * Sets DDC5V_MODE in register DDC_CTL.
     * Default: DDC5V_DELAY_0_MS
     */
    pub ddc5v_delay: tc358743_ddc5v_delays,

    pub enable_hdcp: bool,

    /*
     * The FIFO size is 512x32, so Toshiba recommend to set the default FIFO
     * level to somewhere in the middle (e.g. 300), so it can cover speed
     * mismatches in input and output ports.
     */
    pub fifo_level: u16,

    /* Bps pr lane is (refclk_hz / pll_prd) * pll_fbd */
    pub pll_prd: u16,
    pub pll_fbd: u16,

    /* CSI
     * Calculate CSI parameters with REF_02 for the highest resolution your
     * CSI interface can handle. The driver will adjust the number of CSI
     * lanes in use according to the pixel clock.
     *
     * The values in brackets are calculated with REF_02 when the number of
     * bps pr lane is 823.5 MHz, and can serve as a starting point.
     */
    pub lineinitcnt: u32,    /* (0x00001770) */
    pub lptxtimecnt: u32,    /* (0x00000005) */
    pub tclk_headercnt: u32, /* (0x00001d04) */
    pub tclk_trailcnt: u32,  /* (0x00000000) */
    pub ths_headercnt: u32,  /* (0x00000505) */
    pub twakeup: u32,        /* (0x00004650) */
    pub tclk_postcnt: u32,   /* (0x00000000) */
    pub ths_trailcnt: u32,   /* (0x00000004) */
    pub hstxvregcnt: u32,    /* (0x00000005) */

    /* DVI->HDMI detection delay to avoid unnecessary switching between DVI
     * and HDMI mode.
     * Sets HDMI_DET_V in register HDMI_DET.
     * Default: HDMI_MODE_DELAY_0_MS
     */
    pub hdmi_detection_delay: tc358743_hdmi_detection_delay,

    /* Reset PHY automatically when TMDS clock goes from DC to AC.
     * Sets PHY_AUTO_RST2 in register PHY_CTL2.
     * Default: false
     */
    pub hdmi_phy_auto_reset_tmds_detected: bool,

    /* Reset PHY automatically when TMDS clock passes 21 MHz.
     * Sets PHY_AUTO_RST3 in register PHY_CTL2.
     * Default: false
     */
    pub hdmi_phy_auto_reset_tmds_in_range: bool,

    /* Reset PHY automatically when TMDS clock is detected.
     * Sets PHY_AUTO_RST4 in register PHY_CTL2.
     * Default: false
     */
    pub hdmi_phy_auto_reset_tmds_valid: bool,

    /* Reset HDMI PHY automatically when hsync period is out of range.
     * Sets H_PI_RST in register HV_RST.
     * Default: false
     */
    pub hdmi_phy_auto_reset_hsync_out_of_range: bool,

    /* Reset HDMI PHY automatically when vsync period is out of range.
     * Sets V_PI_RST in register HV_RST.
     * Default: false
     */
    pub hdmi_phy_auto_reset_vsync_out_of_range: bool,
}

/* custom controls */
/* Audio sample rate in Hz */
pub const TC358743_CID_AUDIO_SAMPLING_RATE: u32 = V4L2_CID_USER_TC358743_BASE + 0;
/* Audio present status */
pub const TC358743_CID_AUDIO_PRESENT: u32 = V4L2_CID_USER_TC358743_BASE + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
