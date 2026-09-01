// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2024 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 * soc_sdw_bridge_cs35l56 - codec helper functions for handling CS35L56 Smart AMP
 */

// C dependencies: linux/module.h, linux/platform_device.h, sound/core.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h,
// sound/soc_sdw_utils.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_IB_IF: c_uint = 3 << 8;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 4 << 12;
const SOC_SDW_SIDECAR_AMPS: u64 = 1 << 2;

const BRIDGE_SPEAKER: &[u8] = b"Bridge Speaker\0";
const AMPL_SPK: &[u8] = b"AMPL SPK\0";
const AMPR_SPK: &[u8] = b"AMPR SPK\0";
const AMPL: &[u8] = b"AMPL\0";
const AMPR: &[u8] = b"AMPR\0";
const CS42L43_CS35L56: &[u8] = b"cs42l43-cs35l56\0";
const CS42L43_CODEC: &[u8] = b"cs42l43-codec\0";
const CS42L43_ASP: &[u8] = b"cs42l43-asp\0";
const SPI_CS35L56_LEFT: &[u8] = b"spi-cs35l56-left\0";
const SPI_CS35L56_RIGHT: &[u8] = b"spi-cs35l56-right\0";
const CS35L56_ASP1: &[u8] = b"cs35l56-asp1\0";

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_card {
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_context {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub num_codecs: c_int,
    pub codec_dais: *mut *mut snd_soc_dai,
    pub num_cpus: c_int,
    pub cpu_dais: *mut *mut snd_soc_dai,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub sname: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *const snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *const snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub c2c_params: *const snd_soc_pcm_stream,
    pub num_c2c_params: c_uint,
    pub dai_fmt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_mc_private {
    pub mc_quirk: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_codec_info {
    pub amp_num: c_int,
}

unsafe impl Sync for snd_soc_dapm_widget {}
unsafe impl Sync for snd_soc_dapm_route {}
unsafe impl Sync for snd_soc_pcm_stream {}
unsafe impl Sync for snd_soc_dai_link_component {}
unsafe impl Sync for snd_soc_dai_link {}

unsafe extern "C" {
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn asoc_sdw_cs35l56_volume_limit(
        card: *mut snd_soc_card,
        name_prefix: *const c_char,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
}

static BRIDGE_WIDGETS: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    id: 0,
    name: BRIDGE_SPEAKER.as_ptr() as *const c_char,
    sname: ptr::null(),
}];

static BRIDGE_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: BRIDGE_SPEAKER.as_ptr() as *const c_char,
        control: ptr::null(),
        source: AMPL_SPK.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: BRIDGE_SPEAKER.as_ptr() as *const c_char,
        control: ptr::null(),
        source: AMPR_SPK.as_ptr() as *const c_char,
    },
];

static BRIDGE_CS35L56_NAME_PREFIXES: [*const c_char; 2] = [
    AMPL.as_ptr() as *const c_char,
    AMPR.as_ptr() as *const c_char,
];

unsafe extern "C" fn asoc_sdw_bridge_cs35l56_asp_init(
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut ret: c_int;
    let rx_mask: c_uint = 3; // ASP RX1, RX2
    let tx_mask: c_uint = 3; // ASP TX1, TX2

    ret = snd_soc_dapm_new_controls(
        dapm,
        BRIDGE_WIDGETS.as_ptr(),
        BRIDGE_WIDGETS.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"widgets addition failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, BRIDGE_MAP.as_ptr(), BRIDGE_MAP.len() as c_int);
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"map addition failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* 4 x 16-bit sample slots and FSYNC=48000, BCLK=3.072 MHz */
    let mut i: c_int = 0;
    while i < (*rtd).num_codecs {
        let codec_dai: *mut snd_soc_dai = *(*rtd).codec_dais.add(i as usize);
        ret = asoc_sdw_cs35l56_volume_limit(card, (*(*codec_dai).component).name_prefix);
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_dai_set_tdm_slot(codec_dai, tx_mask, rx_mask, 4, 16);
        if ret < 0 {
            return ret;
        }

        ret = snd_soc_dai_set_sysclk(codec_dai, 0, 3072000, SND_SOC_CLOCK_IN);
        if ret < 0 {
            return ret;
        }

        i += 1;
    }

    i = 0;
    while i < (*rtd).num_cpus {
        let cpu_dai: *mut snd_soc_dai = *(*rtd).cpu_dais.add(i as usize);
        ret = snd_soc_dai_set_tdm_slot(cpu_dai, tx_mask, rx_mask, 4, 16);
        if ret < 0 {
            return ret;
        }

        i += 1;
    }

    0
}

static ASOC_SDW_BRIDGE_PARAMS: snd_soc_pcm_stream = snd_soc_pcm_stream {
    stream_name: ptr::null(),
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    rates: 0,
};

static ASOC_SDW_BRIDGE_DAI_CPUS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: CS42L43_CODEC.as_ptr() as *const c_char,
        dai_name: CS42L43_ASP.as_ptr() as *const c_char,
    }];

static ASOC_SDW_BRIDGE_DAI_CODECS: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component {
        name: SPI_CS35L56_LEFT.as_ptr() as *const c_char,
        dai_name: CS35L56_ASP1.as_ptr() as *const c_char,
    },
    snd_soc_dai_link_component {
        name: SPI_CS35L56_RIGHT.as_ptr() as *const c_char,
        dai_name: CS35L56_ASP1.as_ptr() as *const c_char,
    },
];

static ASOC_SDW_BRIDGE_DAI_PLATFORMS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: CS42L43_CODEC.as_ptr() as *const c_char,
        dai_name: ptr::null(),
    }];

static BRIDGE_DAI_TEMPLATE: snd_soc_dai_link = snd_soc_dai_link {
    name: CS42L43_CS35L56.as_ptr() as *const c_char,
    stream_name: ptr::null(),
    cpus: ASOC_SDW_BRIDGE_DAI_CPUS.as_ptr(),
    num_cpus: ASOC_SDW_BRIDGE_DAI_CPUS.len() as c_uint,
    codecs: ASOC_SDW_BRIDGE_DAI_CODECS.as_ptr(),
    num_codecs: ASOC_SDW_BRIDGE_DAI_CODECS.len() as c_uint,
    platforms: ASOC_SDW_BRIDGE_DAI_PLATFORMS.as_ptr(),
    num_platforms: ASOC_SDW_BRIDGE_DAI_PLATFORMS.len() as c_uint,
    init: Some(asoc_sdw_bridge_cs35l56_asp_init),
    c2c_params: &ASOC_SDW_BRIDGE_PARAMS,
    num_c2c_params: 1,
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_CBC_CFC,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_bridge_cs35l56_count_sidecar(
    ctx: *mut asoc_sdw_mc_private,
    num_dais: *mut c_int,
    num_devs: *mut c_int,
) -> c_int {
    if (*ctx).mc_quirk & SOC_SDW_SIDECAR_AMPS != 0 {
        *num_dais += 1;
        *num_devs += BRIDGE_CS35L56_NAME_PREFIXES.len() as c_int;
    }

    0
}
// EXPORT_SYMBOL_NS(asoc_sdw_bridge_cs35l56_count_sidecar, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_bridge_cs35l56_add_sidecar(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    codec_conf: *mut *mut snd_soc_codec_conf,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private =
        snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;

    if (*ctx).mc_quirk & SOC_SDW_SIDECAR_AMPS != 0 {
        **dai_links = BRIDGE_DAI_TEMPLATE;

        let mut i: usize = 0;
        while i < BRIDGE_CS35L56_NAME_PREFIXES.len() {
            (**codec_conf).dlc.name = (*(**dai_links).codecs.add(i)).name;
            (**codec_conf).name_prefix = BRIDGE_CS35L56_NAME_PREFIXES[i];
            *codec_conf = (*codec_conf).add(1);
            i += 1;
        }

        *dai_links = (*dai_links).add(1);
    }

    0
}
// EXPORT_SYMBOL_NS(asoc_sdw_bridge_cs35l56_add_sidecar, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_bridge_cs35l56_spk_init(
    card: *mut snd_soc_card,
    _dai_links: *mut snd_soc_dai_link,
    info: *mut asoc_sdw_codec_info,
    _playback: bool,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private =
        snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;

    if (*ctx).mc_quirk & SOC_SDW_SIDECAR_AMPS != 0 {
        (*info).amp_num += BRIDGE_CS35L56_NAME_PREFIXES.len() as c_int;
    }

    0
}
// EXPORT_SYMBOL_NS(asoc_sdw_bridge_cs35l56_spk_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
