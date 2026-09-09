/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm9090.h -- Platform data for WM9090
 *
 * Copyright 2009, 2010 Wolfson Microelectronics. PLC.
 */

// C bit-fields are represented by their containing unsigned-int storage.
#[repr(C)]
pub struct wm9090_platform_data {
    /* Line inputs 1 & 2 can optionally be differential */
    pub bitfields: u32,

    /* AGC configuration.  This is intended to protect the speaker
     * against overdriving and will therefore depend on the
     * hardware setup with incorrect runtime configuration
     * potentially causing hardware damage.
     */
    pub agc: [u16; 3],
}

impl wm9090_platform_data {
    pub const LIN1_DIFF_SHIFT: u32 = 0;
    pub const LIN2_DIFF_SHIFT: u32 = 1;
    pub const AGC_ENA_SHIFT: u32 = 2;

    #[inline]
    pub fn lin1_diff(&self) -> u32 {
        (self.bitfields >> Self::LIN1_DIFF_SHIFT) & 1
    }

    #[inline]
    pub fn lin2_diff(&self) -> u32 {
        (self.bitfields >> Self::LIN2_DIFF_SHIFT) & 1
    }

    #[inline]
    pub fn agc_ena(&self) -> u32 {
        (self.bitfields >> Self::AGC_ENA_SHIFT) & 1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
