/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file incorporates work covered by the following copyright notice:
 * Copyright (c) 2020 Intel Corporation
 * Copyright(c) 2024 Advanced Micro Devices, Inc.
 */

// C includes: <sound/soc.h>, <sound/soc-acpi.h>

pub const SOC_SDW_MAX_DAI_NUM: usize = 8;
pub const SOC_SDW_MAX_AUX_NUM: usize = 2;
pub const SOC_SDW_MAX_NO_PROPS: usize = 2;
#[inline]
pub const fn SOC_SDW_JACK_JDSRC(quirk: u64) -> u64 { quirk & 0x0f }

/* If a CODEC has an optional speaker output, this quirk will enable it */
pub const SOC_SDW_CODEC_SPKR: u64 = 1u64 << 15;
/*
 * If the CODEC has additional devices attached directly to it.
 *
 * For the cs42l43:
 *   - 0 - No speaker output
 *   - SOC_SDW_CODEC_SPKR - CODEC internal speaker
 *   - SOC_SDW_SIDECAR_AMPS - 2x Sidecar amplifiers + CODEC internal speaker
 *   - SOC_SDW_CODEC_SPKR | SOF_SIDECAR_AMPS - Not currently supported
 */
pub const SOC_SDW_SIDECAR_AMPS: u64 = 1u64 << 16;
pub const SOC_SDW_CODEC_MIC: u64 = 1u64 << 17;

pub const SOC_SDW_UNUSED_DAI_ID: i32 = -1;
pub const SOC_SDW_JACK_OUT_DAI_ID: i32 = 0;
pub const SOC_SDW_JACK_IN_DAI_ID: i32 = 1;
pub const SOC_SDW_AMP_OUT_DAI_ID: i32 = 2;
pub const SOC_SDW_AMP_IN_DAI_ID: i32 = 3;
pub const SOC_SDW_DMIC_DAI_ID: i32 = 4;
pub const SOC_SDW_DAI_TYPE_JACK: i32 = 0;
pub const SOC_SDW_DAI_TYPE_AMP: i32 = 1;
pub const SOC_SDW_DAI_TYPE_MIC: i32 = 2;

pub type u8 = ::core::primitive::u8;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub const ACPI_ID_LEN: usize = 16;
pub const SNDRV_PCM_STREAM_LAST: usize = 1;

pub struct asoc_sdw_codec_info;
pub struct snd_soc_card;
pub struct snd_soc_jack;
pub struct device;
pub struct snd_soc_dai_link;
pub struct snd_kcontrol_new;
pub struct snd_soc_dapm_widget;
pub struct snd_soc_pcm_runtime;
pub struct snd_soc_dai;
pub struct snd_soc_ops;
pub struct list_head;
pub struct snd_soc_codec_conf;
pub struct snd_pcm_substream;
pub struct snd_pcm_hw_params;
pub struct snd_soc_acpi_link_adr;
pub struct snd_soc_dai_link_component;
pub struct snd_soc_acpi_endpoint;
pub struct snd_soc_aux_dev;

#[repr(C)]
pub struct asoc_sdw_mc_private {
    pub card: snd_soc_card,
    pub sdw_headset: snd_soc_jack,
    pub headset_codec_dev: *mut device, /* only one headset per card */
    pub amp_dev1: *mut device,
    pub amp_dev2: *mut device,
    pub append_dai_type: bool,
    pub ignore_internal_dmic: bool,
    pub private: *mut core::ffi::c_void,
    pub mc_quirk: usize,
    pub codec_info_list_count: i32,
}

#[repr(C)]
pub struct asoc_sdw_dai_info {
    pub direction: [bool; 2], /* playback & capture support */
    pub codec_name: *const core::ffi::c_char,
    pub dai_name: *const core::ffi::c_char,
    pub component_name: *const core::ffi::c_char,
    pub dai_type: i32,
    pub dailink: [i32; 2], /* dailink id for each direction */
    pub controls: *const snd_kcontrol_new,
    pub num_controls: i32,
    pub widgets: *const snd_soc_dapm_widget,
    pub num_widgets: i32,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link, *mut asoc_sdw_codec_info, bool) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link) -> i32>,
    pub rtd_init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> i32>,
    pub rtd_init_done: bool,
    pub quirk: usize,
    pub quirk_exclude: bool,
}

#[repr(C)] pub struct asoc_sdw_aux_info { pub codec_name: *const core::ffi::c_char }

#[repr(C)]
pub struct asoc_sdw_codec_info {
    pub vendor_id: i32, pub part_id: i32, pub version_id: i32,
    pub name_prefix: *const core::ffi::c_char, pub amp_num: i32,
    pub acpi_id: [u8; ACPI_ID_LEN], pub ignore_internal_dmic: bool,
    pub ops: *const snd_soc_ops, pub dais: [asoc_sdw_dai_info; SOC_SDW_MAX_DAI_NUM],
    pub dai_num: i32, pub auxs: [asoc_sdw_aux_info; SOC_SDW_MAX_AUX_NUM], pub aux_num: i32,
    /* Force AMP-style name_prefix handling (append AMP index) even if MIC/Jack DAIs exist */
    pub is_amp: bool,
    pub codec_card_late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> i32>,
    pub count_sidecar: Option<unsafe extern "C" fn(*mut asoc_sdw_mc_private, *mut i32, *mut i32) -> i32>,
    pub add_sidecar: Option<unsafe extern "C" fn(*mut snd_soc_card, *mut *mut snd_soc_dai_link, *mut *mut snd_soc_codec_conf) -> i32>,
}

#[repr(C)]
pub struct asoc_sdw_endpoint {
    pub list: list_head, pub link_mask: u32, pub codec_name: *const core::ffi::c_char,
    pub name_prefix: *const core::ffi::c_char, pub include_sidecar: bool,
    pub codec_info: *mut asoc_sdw_codec_info, pub dai_info: *const asoc_sdw_dai_info,
}

#[repr(C)]
pub struct asoc_sdw_dailink {
    pub initialised: bool, pub group_id: u8,
    pub link_mask: [u32; SNDRV_PCM_STREAM_LAST + 1],
    pub num_devs: [i32; SNDRV_PCM_STREAM_LAST + 1], pub endpoints: list_head,
}

extern "C" {
    pub static mut codec_info_list: [asoc_sdw_codec_info; 0];
    pub fn asoc_sdw_get_codec_info_list_count() -> i32;
    pub fn asoc_sdw_startup(substream: *mut snd_pcm_substream) -> i32;
    pub fn asoc_sdw_prepare(substream: *mut snd_pcm_substream) -> i32;
    // Duplicate declaration retained from the C header.
    pub fn asoc_sdw_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32;
    pub fn asoc_sdw_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> i32;
    pub fn asoc_sdw_hw_free(substream: *mut snd_pcm_substream) -> i32;
    pub fn asoc_sdw_shutdown(substream: *mut snd_pcm_substream);
    pub fn asoc_sdw_get_codec_name(dev: *mut device, dai_info: *const asoc_sdw_dai_info, adr_link: *const snd_soc_acpi_link_adr, adr_index: i32) -> *const core::ffi::c_char;
    pub fn asoc_sdw_find_codec_info_part(adr: u64) -> *mut asoc_sdw_codec_info;
    pub fn asoc_sdw_find_codec_info_acpi(acpi_id: *const u8) -> *mut asoc_sdw_codec_info;
    pub fn asoc_sdw_find_codec_info_dai(dai_name: *const core::ffi::c_char, dai_index: *mut i32) -> *mut asoc_sdw_codec_info;
    pub fn asoc_sdw_mc_find_codec_dai_used(card: *mut snd_soc_card, dai_name: *const core::ffi::c_char) -> *mut snd_soc_dai_link;
    pub fn asoc_sdw_mc_dailink_exit_loop(card: *mut snd_soc_card);
    pub fn asoc_sdw_card_late_probe(card: *mut snd_soc_card) -> i32;
    pub fn asoc_sdw_init_dai_link(dev: *mut device, dai_links: *mut snd_soc_dai_link, be_id: *mut i32, name: *mut core::ffi::c_char, playback: i32, capture: i32, cpus: *mut snd_soc_dai_link_component, cpus_num: i32, platform_component: *mut snd_soc_dai_link_component, num_platforms: i32, codecs: *mut snd_soc_dai_link_component, codecs_num: i32, no_pcm: i32, init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> i32>, ops: *const snd_soc_ops);
    pub fn asoc_sdw_init_simple_dai_link(dev: *mut device, dai_links: *mut snd_soc_dai_link, be_id: *mut i32, name: *mut core::ffi::c_char, playback: i32, capture: i32, cpu_dai_name: *const core::ffi::c_char, platform_comp_name: *const core::ffi::c_char, codec_name: *const core::ffi::c_char, codec_dai_name: *const core::ffi::c_char, no_pcm: i32, init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> i32>, ops: *const snd_soc_ops) -> i32;
    pub fn asoc_sdw_count_sdw_endpoints(card: *mut snd_soc_card, num_devs: *mut i32, num_ends: *mut i32, num_aux: *mut i32) -> i32;
    pub fn asoc_sdw_find_dailink(dailinks: *mut asoc_sdw_dailink, new: *const snd_soc_acpi_endpoint) -> *mut asoc_sdw_dailink;
    pub fn asoc_sdw_get_dai_type(type_: u32) -> i32;
    pub fn asoc_sdw_parse_sdw_endpoints(dev: *mut device, ctx: *mut asoc_sdw_mc_private, soc_aux: *mut snd_soc_aux_dev, soc_dais: *mut asoc_sdw_dailink, soc_ends: *mut asoc_sdw_endpoint, num_devs: *mut i32) -> i32;
    pub fn asoc_sdw_rtd_init(rtd: *mut snd_soc_pcm_runtime) -> i32;
    pub fn asoc_sdw_dmic_init(rtd: *mut snd_soc_pcm_runtime) -> i32;
    pub fn asoc_sdw_rt711_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_rt711_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> i32;
    pub fn asoc_sdw_rt_sdca_jack_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_rt_sdca_jack_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> i32;
    pub static soc_sdw_rt1308_i2s_ops: snd_soc_ops;
    pub fn asoc_sdw_rt_amp_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_rt_amp_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> i32;
    pub fn asoc_sdw_cs42l43_spk_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_es9356_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_es9356_amp_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_es9356_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> i32;
    pub fn asoc_sdw_bridge_cs35l56_count_sidecar(ctx: *mut asoc_sdw_mc_private, num_dais: *mut i32, num_devs: *mut i32) -> i32;
    pub fn asoc_sdw_bridge_cs35l56_add_sidecar(card: *mut snd_soc_card, dai_links: *mut *mut snd_soc_dai_link, codec_conf: *mut *mut snd_soc_codec_conf) -> i32;
    pub fn asoc_sdw_bridge_cs35l56_spk_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_cs_amp_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_cs_spk_feedback_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs35l56_volume_limit(card: *mut snd_soc_card, name_prefix: *const core::ffi::c_char) -> i32;
    pub fn asoc_sdw_maxim_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_rt_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_rt_sdca_jack_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_rt_amp_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_rt700_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_rt711_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_rt_mf_sdca_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_rt5682_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs42l42_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs42l43_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs42l43_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs42l43_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs42l45_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs42l45_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs47l47_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs47l47_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_cs_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_maxim_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_ti_amp_init(card: *mut snd_soc_card, dai_links: *mut snd_soc_dai_link, info: *mut asoc_sdw_codec_info, playback: bool) -> i32;
    pub fn asoc_sdw_ti_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_ti_tac5xx2_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_ti_amp_initial_settings(card: *mut snd_soc_card, name_prefix: *const core::ffi::c_char) -> i32;
    pub fn asoc_sdw_ti_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_ti_sdca_jack_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_es9356_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_es9356_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
    pub fn asoc_sdw_es9356_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
