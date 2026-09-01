// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2023 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 *  soc_sdw_cs_amp - Helpers to handle CS35L56 from generic machine driver
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const CS_AMP_CHANNELS_PER_AMP: c_uint = 4;
const CS35L56_SPK_VOLUME_0DB: c_int = 400; /* 0dB Max */

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_ch_map {
    pub cpu: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *const snd_soc_dai_link,
}

#[repr(C)]
pub struct asoc_sdw_codec_info {
    pub amp_num: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

unsafe extern "C" {
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn snd_soc_limit_volume(
        card: *mut snd_soc_card,
        name: *const c_char,
        max: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_link_to_codec(
        link: *const snd_soc_dai_link,
        index: c_int,
    ) -> *const snd_soc_dai_link_component;
    fn snd_soc_find_dai(dlc: *const snd_soc_dai_link_component) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn WARN_ON(condition: bool) -> c_int;

    /*
     * Rust translation hook for for_each_rtd_codec_dais(rtd, i, codec_dai).
     * The original iterator macro is supplied by the ASoC headers.
     */
    fn for_each_rtd_codec_dais_get(
        rtd: *mut snd_soc_pcm_runtime,
        index: c_int,
    ) -> *mut snd_soc_dai;

    /*
     * Rust translation hooks for for_each_rtd_ch_maps(rtd, i, ch_map).
     * The original iterator macro is supplied by the ASoC headers.
     */
    fn for_each_rtd_ch_maps_len(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn for_each_rtd_ch_maps_get(
        rtd: *mut snd_soc_pcm_runtime,
        index: c_int,
    ) -> *const snd_soc_dai_link_ch_map;
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    if h >= 31 {
        c_uint::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !(if l == 0 { 0 } else { (1u32 << l) - 1 })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs35l56_volume_limit(
    card: *mut snd_soc_card,
    name_prefix: *const c_char,
) -> c_int {
    let volume_ctl_name: *mut c_char;
    let ret: c_int;

    volume_ctl_name = kasprintf(GFP_KERNEL, c"%s Speaker Volume".as_ptr(), name_prefix);
    if volume_ctl_name.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_limit_volume(card, volume_ctl_name, CS35L56_SPK_VOLUME_0DB);
    if ret != 0 {
        dev_err(
            (*card).dev,
            c"%s limit set failed: %d\n".as_ptr(),
            volume_ctl_name,
            ret,
        );
    }

    kfree(volume_ctl_name as *const c_void);
    ret
}

// EXPORT_SYMBOL_NS(asoc_sdw_cs35l56_volume_limit, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut widget_name: [c_char; 16] = [0; 16];
    let route = snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: core::ptr::null(),
        source: widget_name.as_mut_ptr(),
    };
    let mut codec_dai: *mut snd_soc_dai;
    let mut i: c_int = 0;
    let mut ret: c_int;

    /* for_each_rtd_codec_dais(rtd, i, codec_dai) */
    while {
        codec_dai = for_each_rtd_codec_dais_get(rtd, i);
        !codec_dai.is_null()
    } {
        if strstr((*codec_dai).name, c"cs35l56".as_ptr()).is_null() {
            i += 1;
            continue;
        }

        snprintf(
            widget_name.as_mut_ptr(),
            core::mem::size_of_val(&widget_name),
            c"%s SPK".as_ptr(),
            (*(*codec_dai).component).name_prefix,
        );

        ret = asoc_sdw_cs35l56_volume_limit(card, (*(*codec_dai).component).name_prefix);
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_dapm_add_routes(dapm, &route, 1);
        if ret != 0 {
            return ret;
        }

        i += 1;
    }

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_cs_spk_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs_spk_feedback_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let dai_link: *const snd_soc_dai_link = (*rtd).dai_link;
    let mut ch_map: *const snd_soc_dai_link_ch_map;
    let mut codec_dlc: *const snd_soc_dai_link_component;
    let mut codec_dai: *mut snd_soc_dai;
    let mut ch_slot: [u8; 8] = [0; 8];
    let amps_per_bus: c_uint;
    let ch_per_amp: c_uint;
    let mut mask: c_uint;
    let mut i: c_int;
    let mut ret: c_int;

    WARN_ON((*dai_link).num_cpus as usize > ch_slot.len());

    /*
     * CS35L56 has 4 TX channels. When the capture is aggregated the
     * same bus slots will be allocated to all the amps on a bus. Only
     * one amp on that bus can be transmitting in each slot so divide
     * the available 4 slots between all the amps on a bus.
     */
    amps_per_bus = (*dai_link).num_codecs / (*dai_link).num_cpus;
    if (amps_per_bus == 0) || (amps_per_bus > CS_AMP_CHANNELS_PER_AMP) {
        dev_err(
            (*(*rtd).card).dev,
            c"Illegal num_codecs:%u / num_cpus:%u\n".as_ptr(),
            (*dai_link).num_codecs,
            (*dai_link).num_cpus,
        );
        return -EINVAL;
    }

    ch_per_amp = CS_AMP_CHANNELS_PER_AMP / amps_per_bus;

    i = 0;
    /* for_each_rtd_ch_maps(rtd, i, ch_map) */
    while i < for_each_rtd_ch_maps_len(rtd) {
        ch_map = for_each_rtd_ch_maps_get(rtd, i);
        codec_dlc = snd_soc_link_to_codec((*rtd).dai_link, i);
        codec_dai = snd_soc_find_dai(codec_dlc);
        mask = genmask(ch_per_amp - 1, 0) << ch_slot[(*ch_map).cpu as usize];

        ret = snd_soc_dai_set_tdm_slot(codec_dai, 0, mask, 4, 32);
        if ret < 0 {
            dev_err(
                (*(*rtd).card).dev,
                c"Failed to set TDM slot:%d\n".as_ptr(),
                ret,
            );
            return ret;
        }

        ch_slot[(*ch_map).cpu as usize] =
            ch_slot[(*ch_map).cpu as usize].wrapping_add(ch_per_amp as u8);
        i += 1;
    }

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_cs_spk_feedback_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs_amp_init(
    _card: *mut snd_soc_card,
    _dai_links: *mut snd_soc_dai_link,
    info: *mut asoc_sdw_codec_info,
    playback: bool,
) -> c_int {
    /* Do init on playback link only. */
    if !playback {
        return 0;
    }

    (*info).amp_num = (*info).amp_num.wrapping_add(1);

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_cs_amp_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
