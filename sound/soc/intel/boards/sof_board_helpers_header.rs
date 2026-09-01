/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2023 Intel Corporation.
 */

// C dependencies removed from executable Rust:
// <sound/soc.h>, <sound/soc-acpi-intel-ssp-common.h>, "sof_hdmi_common.h"

use core::ffi::{c_int, c_ulong};

const fn bit(nr: u32) -> c_ulong {
    (1 as c_ulong) << nr
}

const fn genmask(h: u32, l: u32) -> c_ulong {
    (!0 as c_ulong) >> (c_ulong::BITS - 1 - h) & ((!0 as c_ulong) << l)
}

/*
 * Common board quirks: from bit 8 to 31, LSB 8 bits reserved for machine
 *                      drivers
 */

/* SSP port number for headphone codec: 3 bits */
pub const SOF_SSP_PORT_CODEC_SHIFT: c_ulong = 8;
pub const SOF_SSP_PORT_CODEC_MASK: c_ulong = genmask(10, 8);
pub const fn SOF_SSP_PORT_CODEC(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_SSP_PORT_CODEC_SHIFT) & SOF_SSP_PORT_CODEC_MASK
}

/* SSP port number for speaker amplifier: 3 bits */
pub const SOF_SSP_PORT_AMP_SHIFT: c_ulong = 11;
pub const SOF_SSP_PORT_AMP_MASK: c_ulong = genmask(13, 11);
pub const fn SOF_SSP_PORT_AMP(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_SSP_PORT_AMP_SHIFT) & SOF_SSP_PORT_AMP_MASK
}

/* SSP port number for BT audio offload: 3 bits */
pub const SOF_SSP_PORT_BT_OFFLOAD_SHIFT: c_ulong = 14;
pub const SOF_SSP_PORT_BT_OFFLOAD_MASK: c_ulong = genmask(16, 14);
pub const fn SOF_SSP_PORT_BT_OFFLOAD(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_SSP_PORT_BT_OFFLOAD_SHIFT) & SOF_SSP_PORT_BT_OFFLOAD_MASK
}

/* SSP port mask for HDMI capture: 6 bits */
pub const SOF_SSP_MASK_HDMI_CAPTURE_SHIFT: c_ulong = 17;
pub const SOF_SSP_MASK_HDMI_CAPTURE_MASK: c_ulong = genmask(22, 17);
pub const fn SOF_SSP_MASK_HDMI_CAPTURE(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_SSP_MASK_HDMI_CAPTURE_SHIFT) & SOF_SSP_MASK_HDMI_CAPTURE_MASK
}

/* Number of idisp HDMI BE link: 3 bits */
pub const SOF_NUM_IDISP_HDMI_SHIFT: c_ulong = 23;
pub const SOF_NUM_IDISP_HDMI_MASK: c_ulong = genmask(25, 23);
pub const fn SOF_NUM_IDISP_HDMI(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_NUM_IDISP_HDMI_SHIFT) & SOF_NUM_IDISP_HDMI_MASK
}

/* Board uses BT audio offload */
pub const SOF_BT_OFFLOAD_PRESENT: c_ulong = bit(26);

pub const SOF_LINK_NONE: c_int = 0;
pub const SOF_LINK_CODEC: c_int = 1;
pub const SOF_LINK_DMIC01: c_int = 2;
pub const SOF_LINK_DMIC16K: c_int = 3;
pub const SOF_LINK_IDISP_HDMI: c_int = 4;
pub const SOF_LINK_AMP: c_int = 5;
pub const SOF_LINK_BT_OFFLOAD: c_int = 6;
pub const SOF_LINK_HDMI_IN: c_int = 7;
pub const SOF_LINK_HDA: c_int = 8;

pub const SOF_LINK_ORDER_MASK: c_ulong = 0xF;
pub const SOF_LINK_ORDER_SHIFT: c_ulong = 4;

pub const fn SOF_LINK_ORDER(
    k1: c_ulong,
    k2: c_ulong,
    k3: c_ulong,
    k4: c_ulong,
    k5: c_ulong,
    k6: c_ulong,
    k7: c_ulong,
) -> c_ulong {
    (((k1) & SOF_LINK_ORDER_MASK) << (SOF_LINK_ORDER_SHIFT * 0))
        | (((k2) & SOF_LINK_ORDER_MASK) << (SOF_LINK_ORDER_SHIFT * 1))
        | (((k3) & SOF_LINK_ORDER_MASK) << (SOF_LINK_ORDER_SHIFT * 2))
        | (((k4) & SOF_LINK_ORDER_MASK) << (SOF_LINK_ORDER_SHIFT * 3))
        | (((k5) & SOF_LINK_ORDER_MASK) << (SOF_LINK_ORDER_SHIFT * 4))
        | (((k6) & SOF_LINK_ORDER_MASK) << (SOF_LINK_ORDER_SHIFT * 5))
        | (((k7) & SOF_LINK_ORDER_MASK) << (SOF_LINK_ORDER_SHIFT * 6))
}

pub const SOF_LINK_IDS_MASK: c_ulong = 0xF;
pub const SOF_LINK_IDS_SHIFT: c_ulong = 4;

pub const fn SOF_LINK_IDS(
    k1: c_ulong,
    k2: c_ulong,
    k3: c_ulong,
    k4: c_ulong,
    k5: c_ulong,
    k6: c_ulong,
    k7: c_ulong,
) -> c_ulong {
    (((k1) & SOF_LINK_IDS_MASK) << (SOF_LINK_IDS_SHIFT * 0))
        | (((k2) & SOF_LINK_IDS_MASK) << (SOF_LINK_IDS_SHIFT * 1))
        | (((k3) & SOF_LINK_IDS_MASK) << (SOF_LINK_IDS_SHIFT * 2))
        | (((k4) & SOF_LINK_IDS_MASK) << (SOF_LINK_IDS_SHIFT * 3))
        | (((k5) & SOF_LINK_IDS_MASK) << (SOF_LINK_IDS_SHIFT * 4))
        | (((k6) & SOF_LINK_IDS_MASK) << (SOF_LINK_IDS_SHIFT * 5))
        | (((k7) & SOF_LINK_IDS_MASK) << (SOF_LINK_IDS_SHIFT * 6))
}

/*
 * sof_da7219_private: private data for da7219 machine driver
 *
 * @mclk_en: true for mclk pin is connected
 * @pll_bypass: true for PLL bypass mode
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_da7219_private {
    pub mclk_en: bool,
    pub pll_bypass: bool,
}

/*
 * sof_rt5682_private: private data for rt5682 machine driver
 *
 * @mclk: mclk clock data
 * @is_legacy_cpu: true for BYT/CHT boards
 * @mclk_en: true for mclk pin is connected
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_rt5682_private {
    pub mclk: *mut clk,
    pub is_legacy_cpu: bool,
    pub mclk_en: bool,
}

/*
 * sof_card_private: common data for machine drivers
 *
 * @headset_jack: headset jack data
 * @hdmi: init data for hdmi dai link
 * @codec_type: type of headset codec
 * @amp_type: type of speaker amplifier
 * @dmic_be_num: number of Intel PCH DMIC BE link
 * @hdmi_num: number of Intel HDMI BE link
 * @ssp_codec: ssp port number of headphone BE link
 * @ssp_amp: ssp port number of speaker BE link
 * @ssp_bt: ssp port number of BT offload BE link
 * @ssp_mask_hdmi_in: ssp port mask of HDMI-IN BE link
 * @bt_offload_present: true to create BT offload BE link
 * @hda_codec_present: true to create HDA codec BE links
 * @codec_link: pointer to headset codec dai link
 * @amp_link: pointer to speaker amplifier dai link
 * @link_order_overwrite: custom DAI link order
 * @link_id_overwrite: custom DAI link ID
 * @da7219: private data for da7219 machine driver
 * @rt5682: private data for rt5682 machine driver
 */
#[repr(C)]
pub struct sof_card_private {
    pub headset_jack: snd_soc_jack,
    pub hdmi: sof_hdmi_private,

    pub codec_type: snd_soc_acpi_intel_codec,
    pub amp_type: snd_soc_acpi_intel_codec,

    pub dmic_be_num: c_int,
    pub hdmi_num: c_int,

    pub ssp_codec: c_int,
    pub ssp_amp: c_int,
    pub ssp_bt: c_int,
    pub ssp_mask_hdmi_in: c_ulong,

    pub bt_offload_present: bool,
    pub hda_codec_present: bool,

    pub codec_link: *mut snd_soc_dai_link,
    pub amp_link: *mut snd_soc_dai_link,

    pub link_order_overwrite: c_ulong,
    /*
     * A variable stores id for all BE DAI links, use SOF_LINK_IDS macro to
     * build the value; use DAI link array index as id if zero.
     */
    pub link_id_overwrite: c_ulong,

    pub u: sof_card_private_union,
}

#[repr(C)]
pub union sof_card_private_union {
    pub da7219: sof_da7219_private,
    pub rt5682: sof_rt5682_private,
}

extern "C" {
    pub fn sof_intel_board_card_late_probe(card: *mut snd_soc_card) -> c_int;
    pub fn sof_intel_board_set_dai_link(
        dev: *mut device,
        card: *mut snd_soc_card,
        ctx: *mut sof_card_private,
    ) -> c_int;
    pub fn sof_intel_board_get_ctx(
        dev: *mut device,
        board_quirk: c_ulong,
    ) -> *mut sof_card_private;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
