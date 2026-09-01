/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// C dependencies:
// #include <dt-bindings/sound/qcom,q6afe.h>
// #include <sound/soc.h>

pub const LPASS_MAX_PORT: u32 = LPI_MI2S_TX_6 + 1;

#[repr(C)]
pub struct qcom_snd_tdm_slot_cfg {
    pub tx_mask: u32,
    pub rx_mask: u32,
    pub slots: u32,
    pub slot_width: u32,
}

#[repr(C)]
pub struct snd_soc_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn qcom_snd_parse_of(card: *mut snd_soc_card) -> i32;

    pub fn qcom_snd_get_dai_tdm_slots(
        rtd: *mut snd_soc_pcm_runtime,
        cpu_cfg: *mut qcom_snd_tdm_slot_cfg,
        codec_cfg: *mut qcom_snd_tdm_slot_cfg,
    ) -> i32;

    pub fn qcom_snd_apply_dai_tdm_slots_cfg(
        rtd: *mut snd_soc_pcm_runtime,
        cpu_cfg: *const qcom_snd_tdm_slot_cfg,
        codec_cfg: *const qcom_snd_tdm_slot_cfg,
    ) -> i32;

    pub fn qcom_snd_apply_dai_tdm_slots(rtd: *mut snd_soc_pcm_runtime) -> i32;

    pub fn qcom_snd_wcd_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        jack_setup: *mut bool,
    ) -> i32;

    pub fn qcom_snd_dp_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        dp_jack: *mut snd_soc_jack,
        id: i32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
