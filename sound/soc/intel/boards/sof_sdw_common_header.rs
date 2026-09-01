/* SPDX-License-Identifier: GPL-2.0-only
 *  Copyright (c) 2020 Intel Corporation
 */

/*
 *  sof_sdw_common.h - prototypes for common helpers
 */

// Dependencies from the original header:
// linux/bits.h, linux/types.h, sound/soc.h, sound/soc_sdw_utils.h,
// and "sof_hdmi_common.h".

use core::ffi::c_int;

pub const MAX_HDMI_NUM: u32 = 4;
pub const SOC_SDW_MAX_CPU_DAIS: u32 = 16;
pub const SOC_SDW_INTEL_BIDIR_PDI_BASE: u32 = 2;

/* 8 combinations with 4 links + unused group 0 */
pub const SDW_MAX_GROUPS: u32 = 9;

pub const SOF_PRE_TGL_HDMI_COUNT: u32 = 3;
pub const SOF_TGL_HDMI_COUNT: u32 = 4;

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    let high = if h >= 31 {
        u32::MAX
    } else {
        (1u32 << (h + 1)) - 1
    };
    let low = if l == 0 { 0 } else { (1u32 << l) - 1 };
    high & !low
}

pub const SOF_I2S_SSP0: u32 = BIT(0);
pub const SOF_I2S_SSP1: u32 = BIT(1);
pub const SOF_I2S_SSP2: u32 = BIT(2);
pub const SOF_I2S_SSP3: u32 = BIT(3);
pub const SOF_I2S_SSP4: u32 = BIT(4);
pub const SOF_I2S_SSP5: u32 = BIT(5);

/* Deprecated and no longer supported by the code */
pub const SOC_SDW_FOUR_SPK: u32 = BIT(4);
pub const SOF_SDW_TGL_HDMI: u32 = BIT(5);
pub const SOC_SDW_PCH_DMIC: u32 = BIT(6);

pub const fn SOF_SSP_PORT(x: u32) -> u32 {
    ((x & GENMASK(5, 0)) << 7)
}

pub const fn SOF_SSP_GET_PORT(quirk: u32) -> u32 {
    ((quirk >> 7) & GENMASK(5, 0))
}

/* Deprecated and no longer supported by the code */
pub const SOC_SDW_NO_AGGREGATION: u32 = BIT(14);

/* BT audio offload: reserve 3 bits for future */
pub const SOF_BT_OFFLOAD_SSP_SHIFT: u32 = 18;
pub const SOF_BT_OFFLOAD_SSP_MASK: u32 = GENMASK(20, 18);

pub const fn SOF_BT_OFFLOAD_SSP(quirk: u32) -> u32 {
    ((quirk << SOF_BT_OFFLOAD_SSP_SHIFT) & SOF_BT_OFFLOAD_SSP_MASK)
}

pub const SOF_SSP_BT_OFFLOAD_PRESENT: u32 = BIT(21);

#[repr(C)]
pub struct intel_mc_ctx {
    pub hdmi: sof_hdmi_private,
    /* To store SDW Pin index for each SoundWire link */
    pub sdw_pin_index: [u32; SDW_INTEL_MAX_LINKS],
}

/* generic HDMI support */
unsafe extern "C" {
    pub fn sof_sdw_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;

    pub fn sof_sdw_hdmi_card_late_probe(card: *mut snd_soc_card) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
