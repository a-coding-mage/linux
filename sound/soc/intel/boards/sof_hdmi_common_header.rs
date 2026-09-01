/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2023 Intel Corporation.
 */

/* Dependency intent from C header: #include <sound/soc.h> */

pub const IDISP_CODEC_MASK: u32 = 0x4;

/*
 * sof_hdmi_private: data for Intel HDMI dai link (idisp) initialization
 *
 * @hdmi_comp: ASoC component of idisp codec
 * @idisp_codec: true to indicate idisp codec is present
 */
#[repr(C)]
pub struct sof_hdmi_private {
    pub hdmi_comp: *mut snd_soc_component,
    pub idisp_codec: bool,
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
