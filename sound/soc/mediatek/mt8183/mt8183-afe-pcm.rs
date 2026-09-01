// SPDX-License-Identifier: GPL-2.0
//
// Mediatek ALSA SoC AFE platform driver for 8183
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies originally included from Linux, ALSA SoC, and local MediaTek
// headers are expected to be supplied by the surrounding translated tree.

const MTK_AFE_RATE_8K: c_uint = 0;
const MTK_AFE_RATE_11K: c_uint = 1;
const MTK_AFE_RATE_12K: c_uint = 2;
const MTK_AFE_RATE_384K: c_uint = 3;
const MTK_AFE_RATE_16K: c_uint = 4;
const MTK_AFE_RATE_22K: c_uint = 5;
const MTK_AFE_RATE_24K: c_uint = 6;
const MTK_AFE_RATE_130K: c_uint = 7;
const MTK_AFE_RATE_32K: c_uint = 8;
const MTK_AFE_RATE_44K: c_uint = 9;
const MTK_AFE_RATE_48K: c_uint = 10;
const MTK_AFE_RATE_88K: c_uint = 11;
const MTK_AFE_RATE_96K: c_uint = 12;
const MTK_AFE_RATE_176K: c_uint = 13;
const MTK_AFE_RATE_192K: c_uint = 14;
const MTK_AFE_RATE_260K: c_uint = 15;

const MTK_AFE_DAI_MEMIF_RATE_8K: c_uint = 0;
const MTK_AFE_DAI_MEMIF_RATE_16K: c_uint = 1;
const MTK_AFE_DAI_MEMIF_RATE_32K: c_uint = 2;
const MTK_AFE_DAI_MEMIF_RATE_48K: c_uint = 3;

const MTK_AFE_PCM_RATE_8K: c_uint = 0;
const MTK_AFE_PCM_RATE_16K: c_uint = 1;
const MTK_AFE_PCM_RATE_32K: c_uint = 2;
const MTK_AFE_PCM_RATE_48K: c_uint = 3;

extern "C" {
    static mtk_afe_fe_ops: snd_soc_dai_ops;
    static mtk_afe_pcm_platform: snd_soc_component_driver;

    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
    ) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(regmap: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(regmap: *mut regmap, enable: bool);
    fn regcache_mark_dirty(regmap: *mut regmap);
    fn regcache_sync(regmap: *mut regmap) -> c_int;
    fn regmap_attach_dev(
        dev: *mut device,
        regmap: *mut regmap,
        config: *const regmap_config,
    ) -> c_int;
    fn regmap_reinit_cache(regmap: *mut regmap, config: *const regmap_config) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn mt8183_afe_disable_clock(afe: *mut mtk_base_afe) -> c_int;
    fn mt8183_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn of_reserved_mem_device_release(data: *mut c_void);
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn of_reserved_mem_device_init(dev: *mut device) -> c_int;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn mt8183_init_clock(afe: *mut mtk_base_afe) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn syscon_node_to_regmap(np: *mut device_node) -> *mut regmap;
    fn devm_reset_control_get(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn reset_control_reset(rstc: *mut reset_control) -> c_int;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        data: *mut c_void,
    ) -> c_int;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn mtk_afe_combine_sub_dai(afe: *mut mtk_base_afe) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn mt8183_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
    fn mt8183_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int;
    fn mt8183_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int;
    fn mt8183_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int;
    fn mt8183_dai_hostless_register(afe: *mut mtk_base_afe) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_general_rate_transform(
    dev: *mut device,
    rate: c_uint,
) -> c_uint {
    match rate {
        8000 => MTK_AFE_RATE_8K,
        11025 => MTK_AFE_RATE_11K,
        12000 => MTK_AFE_RATE_12K,
        16000 => MTK_AFE_RATE_16K,
        22050 => MTK_AFE_RATE_22K,
        24000 => MTK_AFE_RATE_24K,
        32000 => MTK_AFE_RATE_32K,
        44100 => MTK_AFE_RATE_44K,
        48000 => MTK_AFE_RATE_48K,
        88200 => MTK_AFE_RATE_88K,
        96000 => MTK_AFE_RATE_96K,
        130000 => MTK_AFE_RATE_130K,
        176400 => MTK_AFE_RATE_176K,
        192000 => MTK_AFE_RATE_192K,
        260000 => MTK_AFE_RATE_260K,
        _ => {
            dev_warn(
                dev,
                c"%s(), rate %u invalid, use %d!!!\n".as_ptr(),
                c"mt8183_general_rate_transform".as_ptr(),
                rate,
                MTK_AFE_RATE_48K as c_int,
            );
            MTK_AFE_RATE_48K
        }
    }
}

unsafe extern "C" fn dai_memif_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => MTK_AFE_DAI_MEMIF_RATE_8K,
        16000 => MTK_AFE_DAI_MEMIF_RATE_16K,
        32000 => MTK_AFE_DAI_MEMIF_RATE_32K,
        48000 => MTK_AFE_DAI_MEMIF_RATE_48K,
        _ => {
            dev_warn(
                dev,
                c"%s(), rate %u invalid, use %d!!!\n".as_ptr(),
                c"dai_memif_rate_transform".as_ptr(),
                rate,
                MTK_AFE_DAI_MEMIF_RATE_16K as c_int,
            );
            MTK_AFE_DAI_MEMIF_RATE_16K
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_rate_transform(
    dev: *mut device,
    rate: c_uint,
    aud_blk: c_int,
) -> c_uint {
    match aud_blk {
        MT8183_MEMIF_MOD_DAI => dai_memif_rate_transform(dev, rate),
        _ => mt8183_general_rate_transform(dev, rate),
    }
}

static mt8183_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    period_bytes_min: 256,
    period_bytes_max: 4 * 48 * 1024,
    periods_min: 2,
    periods_max: 256,
    buffer_bytes_max: 8 * 48 * 1024,
    fifo_size: 0,
};

unsafe extern "C" fn mt8183_memif_fs(
    substream: *mut snd_pcm_substream,
    rate: c_uint,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME.as_ptr());
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;
    let id = (*snd_soc_rtd_to_cpu(rtd, 0)).id;

    mt8183_rate_transform((*afe).dev, rate, id) as c_int
}

unsafe extern "C" fn mt8183_irq_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME.as_ptr());
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;

    mt8183_general_rate_transform((*afe).dev, rate) as c_int
}

const MTK_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_PCM_DAI_RATES: c_uint =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;

const MTK_PCM_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

macro_rules! playback {
    ($name:expr, $channels_min:expr, $channels_max:expr, $rates:expr, $formats:expr) => {
        snd_soc_pcm_stream {
            stream_name: $name.as_ptr(),
            channels_min: $channels_min,
            channels_max: $channels_max,
            rates: $rates,
            formats: $formats,
        }
    };
}

macro_rules! capture {
    ($name:expr, $channels_min:expr, $channels_max:expr, $rates:expr, $formats:expr) => {
        snd_soc_pcm_stream {
            stream_name: $name.as_ptr(),
            channels_min: $channels_min,
            channels_max: $channels_max,
            rates: $rates,
            formats: $formats,
        }
    };
}

static mut mt8183_memif_dai_driver: [snd_soc_dai_driver; 9] = [
    /* FE DAIs: memory intefaces to CPU */
    snd_soc_dai_driver {
        name: c"DL1".as_ptr(),
        id: MT8183_MEMIF_DL1,
        playback: playback!(c"DL1", 1, 2, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"DL2".as_ptr(),
        id: MT8183_MEMIF_DL2,
        playback: playback!(c"DL2", 1, 2, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"DL3".as_ptr(),
        id: MT8183_MEMIF_DL3,
        playback: playback!(c"DL3", 1, 2, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"UL1".as_ptr(),
        id: MT8183_MEMIF_VUL12,
        capture: capture!(c"UL1", 1, 2, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"UL2".as_ptr(),
        id: MT8183_MEMIF_AWB,
        capture: capture!(c"UL2", 1, 2, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"UL3".as_ptr(),
        id: MT8183_MEMIF_VUL2,
        capture: capture!(c"UL3", 1, 2, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"UL4".as_ptr(),
        id: MT8183_MEMIF_AWB2,
        capture: capture!(c"UL4", 1, 2, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"UL_MONO_1".as_ptr(),
        id: MT8183_MEMIF_MOD_DAI,
        capture: capture!(c"UL_MONO_1", 1, 1, MTK_PCM_DAI_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"HDMI".as_ptr(),
        id: MT8183_MEMIF_HDMI,
        playback: playback!(c"HDMI", 2, 8, MTK_PCM_RATES, MTK_PCM_FORMATS),
        ops: unsafe { &mtk_afe_fe_ops },
        ..snd_soc_dai_driver::zeroed()
    },
];

/* dma widget & routes*/
static memif_ul1_ch1_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH1", AFE_CONN21, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"I2S0_CH1", AFE_CONN21, I_I2S0_CH1, 1, 0),
];

static memif_ul1_ch2_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH2", AFE_CONN22, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"I2S0_CH2", AFE_CONN21, I_I2S0_CH2, 1, 0),
];

static memif_ul2_ch1_mix: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH1", AFE_CONN5, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"DL1_CH1", AFE_CONN5, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"DL2_CH1", AFE_CONN5, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"DL3_CH1", AFE_CONN5, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"I2S2_CH1", AFE_CONN5, I_I2S2_CH1, 1, 0),
];

static memif_ul2_ch2_mix: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH2", AFE_CONN6, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"DL1_CH2", AFE_CONN6, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"DL2_CH2", AFE_CONN6, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"DL3_CH2", AFE_CONN6, I_DL3_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"I2S2_CH2", AFE_CONN6, I_I2S2_CH2, 1, 0),
];

static memif_ul3_ch1_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH1", AFE_CONN32, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"I2S2_CH1", AFE_CONN32, I_I2S2_CH1, 1, 0),
];

static memif_ul3_ch2_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH2", AFE_CONN33, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"I2S2_CH2", AFE_CONN33, I_I2S2_CH2, 1, 0),
];

static memif_ul4_ch1_mix: [snd_kcontrol_new; 1] =
    [SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH1", AFE_CONN38, I_ADDA_UL_CH1, 1, 0)];

static memif_ul4_ch2_mix: [snd_kcontrol_new; 1] =
    [SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH2", AFE_CONN39, I_ADDA_UL_CH2, 1, 0)];

static memif_ul_mono_1_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH1", AFE_CONN12, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH2", AFE_CONN12, I_ADDA_UL_CH2, 1, 0),
];

static mt8183_memif_widgets: [snd_soc_dapm_widget; 9] = [
    /* memif */
    SND_SOC_DAPM_MIXER!(c"UL1_CH1", SND_SOC_NOPM, 0, 0, memif_ul1_ch1_mix),
    SND_SOC_DAPM_MIXER!(c"UL1_CH2", SND_SOC_NOPM, 0, 0, memif_ul1_ch2_mix),
    SND_SOC_DAPM_MIXER!(c"UL2_CH1", SND_SOC_NOPM, 0, 0, memif_ul2_ch1_mix),
    SND_SOC_DAPM_MIXER!(c"UL2_CH2", SND_SOC_NOPM, 0, 0, memif_ul2_ch2_mix),
    SND_SOC_DAPM_MIXER!(c"UL3_CH1", SND_SOC_NOPM, 0, 0, memif_ul3_ch1_mix),
    SND_SOC_DAPM_MIXER!(c"UL3_CH2", SND_SOC_NOPM, 0, 0, memif_ul3_ch2_mix),
    SND_SOC_DAPM_MIXER!(c"UL4_CH1", SND_SOC_NOPM, 0, 0, memif_ul4_ch1_mix),
    SND_SOC_DAPM_MIXER!(c"UL4_CH2", SND_SOC_NOPM, 0, 0, memif_ul4_ch2_mix),
    SND_SOC_DAPM_MIXER!(c"UL_MONO_1_CH1", SND_SOC_NOPM, 0, 0, memif_ul_mono_1_mix),
];

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route {
            sink: $sink.as_ptr(),
            control: ptr::null(),
            source: $source.as_ptr(),
        }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route {
            sink: $sink.as_ptr(),
            control: $control.as_ptr(),
            source: $source.as_ptr(),
        }
    };
}

static mt8183_memif_routes: [snd_soc_dapm_route; 28] = [
    /* capture */
    route!(c"UL1", NULL, c"UL1_CH1"),
    route!(c"UL1", NULL, c"UL1_CH2"),
    route!(c"UL1_CH1", c"ADDA_UL_CH1", c"ADDA Capture"),
    route!(c"UL1_CH2", c"ADDA_UL_CH2", c"ADDA Capture"),
    route!(c"UL1_CH1", c"I2S0_CH1", c"I2S0"),
    route!(c"UL1_CH2", c"I2S0_CH2", c"I2S0"),
    route!(c"UL2", NULL, c"UL2_CH1"),
    route!(c"UL2", NULL, c"UL2_CH2"),
    route!(c"UL2_CH1", c"ADDA_UL_CH1", c"ADDA Capture"),
    route!(c"UL2_CH2", c"ADDA_UL_CH2", c"ADDA Capture"),
    route!(c"UL2_CH1", c"I2S2_CH1", c"I2S2"),
    route!(c"UL2_CH2", c"I2S2_CH2", c"I2S2"),
    route!(c"UL3", NULL, c"UL3_CH1"),
    route!(c"UL3", NULL, c"UL3_CH2"),
    route!(c"UL3_CH1", c"ADDA_UL_CH1", c"ADDA Capture"),
    route!(c"UL3_CH2", c"ADDA_UL_CH2", c"ADDA Capture"),
    route!(c"UL3_CH1", c"I2S2_CH1", c"I2S2"),
    route!(c"UL3_CH2", c"I2S2_CH2", c"I2S2"),
    route!(c"UL4", NULL, c"UL4_CH1"),
    route!(c"UL4", NULL, c"UL4_CH2"),
    route!(c"UL4_CH1", c"ADDA_UL_CH1", c"ADDA Capture"),
    route!(c"UL4_CH2", c"ADDA_UL_CH2", c"ADDA Capture"),
    route!(c"UL_MONO_1", NULL, c"UL_MONO_1_CH1"),
    route!(c"UL_MONO_1_CH1", c"ADDA_UL_CH1", c"ADDA Capture"),
    route!(c"UL_MONO_1_CH1", c"ADDA_UL_CH2", c"ADDA Capture"),
];

static mt8183_afe_pcm_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"mt8183-afe-pcm-dai".as_ptr(),
    ..snd_soc_component_driver::zeroed()
};

macro_rules! MT8183_MEMIF_BASE {
    ($idx:expr, $name:expr, $id:expr, $en_reg:expr, $base:expr, $cur:expr, $end:expr,
     $base_msb:expr, $cur_msb:expr, $end_msb:expr, $fs_reg:expr, $fs_shift:expr,
     $fs_maskbit:expr, $mono_reg:expr, $mono_shift:expr, $enable_shift:expr, $hd_shift:expr,
     $hd_align_mshift:expr) => {
        mtk_base_memif_data {
            name: $name.as_ptr(),
            id: $id,
            reg_ofs_base: $base,
            reg_ofs_cur: $cur,
            reg_ofs_end: $end,
            reg_ofs_base_msb: $base_msb,
            reg_ofs_cur_msb: $cur_msb,
            reg_ofs_end_msb: $end_msb,
            fs_reg: $fs_reg,
            fs_shift: $fs_shift,
            fs_maskbit: $fs_maskbit,
            mono_reg: $mono_reg,
            mono_shift: $mono_shift,
            enable_reg: $en_reg,
            enable_shift: $enable_shift,
            hd_reg: AFE_MEMIF_HD_MODE,
            hd_align_reg: AFE_MEMIF_HDALIGN,
            hd_shift: $hd_shift,
            hd_align_mshift: $hd_align_mshift,
            agent_disable_reg: -1,
            agent_disable_shift: -1,
            msb_reg: -1,
            msb_shift: -1,
        }
    };
}

/* For convenience with macros: missing register fields */
const MOD_DAI_DATA_SFT: c_int = -1;
const HDMI_MODE_SFT: c_int = -1;
const HDMI_MODE_MASK: c_int = -1;
const HDMI_DATA_SFT: c_int = -1;
const HDMI_ON_SFT: c_int = -1;

/* For convenience with macros: register name differences */
const AFE_VUL12_BASE: c_uint = AFE_VUL_D2_BASE;
const AFE_VUL12_CUR: c_uint = AFE_VUL_D2_CUR;
const AFE_VUL12_END: c_uint = AFE_VUL_D2_END;
const AFE_VUL12_BASE_MSB: c_uint = AFE_VUL_D2_BASE_MSB;
const AFE_VUL12_CUR_MSB: c_uint = AFE_VUL_D2_CUR_MSB;
const AFE_VUL12_END_MSB: c_uint = AFE_VUL_D2_END_MSB;
const AWB2_HD_ALIGN_SFT: c_int = AWB2_ALIGN_SFT;
const VUL12_DATA_SFT: c_int = VUL12_MONO_SFT;
const AFE_HDMI_BASE: c_uint = AFE_HDMI_OUT_BASE;
const AFE_HDMI_CUR: c_uint = AFE_HDMI_OUT_CUR;
const AFE_HDMI_END: c_uint = AFE_HDMI_OUT_END;
const AFE_HDMI_BASE_MSB: c_uint = AFE_HDMI_OUT_BASE_MSB;
const AFE_HDMI_CUR_MSB: c_uint = AFE_HDMI_OUT_CUR_MSB;
const AFE_HDMI_END_MSB: c_uint = AFE_HDMI_OUT_END_MSB;

static memif_data: [mtk_base_memif_data; MT8183_MEMIF_NUM as usize] = [
    MT8183_MEMIF_BASE!(MT8183_MEMIF_DL1, c"DL1", MT8183_MEMIF_DL1, AFE_DAC_CON0, AFE_DL1_BASE, AFE_DL1_CUR, AFE_DL1_END, AFE_DL1_BASE_MSB, AFE_DL1_CUR_MSB, AFE_DL1_END_MSB, AFE_DAC_CON1, DL1_MODE_SFT, DL1_MODE_MASK, AFE_DAC_CON1, DL1_DATA_SFT, DL1_ON_SFT, DL1_HD_SFT, DL1_HD_ALIGN_SFT),
    MT8183_MEMIF_BASE!(MT8183_MEMIF_DL2, c"DL2", MT8183_MEMIF_DL2, AFE_DAC_CON0, AFE_DL2_BASE, AFE_DL2_CUR, AFE_DL2_END, AFE_DL2_BASE_MSB, AFE_DL2_CUR_MSB, AFE_DL2_END_MSB, AFE_DAC_CON1, DL2_MODE_SFT, DL2_MODE_MASK, AFE_DAC_CON1, DL2_DATA_SFT, DL2_ON_SFT, DL2_HD_SFT, DL2_HD_ALIGN_SFT),
    MT8183_MEMIF_BASE!(MT8183_MEMIF_DL3, c"DL3", MT8183_MEMIF_DL3, AFE_DAC_CON0, AFE_DL3_BASE, AFE_DL3_CUR, AFE_DL3_END, AFE_DL3_BASE_MSB, AFE_DL3_CUR_MSB, AFE_DL3_END_MSB, AFE_DAC_CON2, DL3_MODE_SFT, DL3_MODE_MASK, AFE_DAC_CON1, DL3_DATA_SFT, DL3_ON_SFT, DL3_HD_SFT, DL3_HD_ALIGN_SFT),
    MT8183_MEMIF_BASE!(MT8183_MEMIF_VUL2, c"VUL2", MT8183_MEMIF_VUL2, AFE_DAC_CON0, AFE_VUL2_BASE, AFE_VUL2_CUR, AFE_VUL2_END, AFE_VUL2_BASE_MSB, AFE_VUL2_CUR_MSB, AFE_VUL2_END_MSB, AFE_DAC_CON2, VUL2_MODE_SFT, VUL2_MODE_MASK, AFE_DAC_CON2, VUL2_DATA_SFT, VUL2_ON_SFT, VUL2_HD_SFT, VUL2_HD_ALIGN_SFT),
    MT8183_MEMIF_BASE!(MT8183_MEMIF_AWB, c"AWB", MT8183_MEMIF_AWB, AFE_DAC_CON0, AFE_AWB_BASE, AFE_AWB_CUR, AFE_AWB_END, AFE_AWB_BASE_MSB, AFE_AWB_CUR_MSB, AFE_AWB_END_MSB, AFE_DAC_CON1, AWB_MODE_SFT, AWB_MODE_MASK, AFE_DAC_CON1, AWB_DATA_SFT, AWB_ON_SFT, AWB_HD_SFT, AWB_HD_ALIGN_SFT),
    MT8183_MEMIF_BASE!(MT8183_MEMIF_AWB2, c"AWB2", MT8183_MEMIF_AWB2, AFE_DAC_CON0, AFE_AWB2_BASE, AFE_AWB2_CUR, AFE_AWB2_END, AFE_AWB2_BASE_MSB, AFE_AWB2_CUR_MSB, AFE_AWB2_END_MSB, AFE_DAC_CON2, AWB2_MODE_SFT, AWB2_MODE_MASK, AFE_DAC_CON2, AWB2_DATA_SFT, AWB2_ON_SFT, AWB2_HD_SFT, AWB2_HD_ALIGN_SFT),
    MT8183_MEMIF_BASE!(MT8183_MEMIF_VUL12, c"VUL12", MT8183_MEMIF_VUL12, AFE_DAC_CON0, AFE_VUL12_BASE, AFE_VUL12_CUR, AFE_VUL12_END, AFE_VUL12_BASE_MSB, AFE_VUL12_CUR_MSB, AFE_VUL12_END_MSB, AFE_DAC_CON0, VUL12_MODE_SFT, VUL12_MODE_MASK, AFE_DAC_CON0, VUL12_DATA_SFT, VUL12_ON_SFT, VUL12_HD_SFT, VUL12_HD_ALIGN_SFT),
    MT8183_MEMIF_BASE!(MT8183_MEMIF_MOD_DAI, c"MOD_DAI", MT8183_MEMIF_MOD_DAI, AFE_DAC_CON0, AFE_MOD_DAI_BASE, AFE_MOD_DAI_CUR, AFE_MOD_DAI_END, AFE_MOD_DAI_BASE_MSB, AFE_MOD_DAI_CUR_MSB, AFE_MOD_DAI_END_MSB, AFE_DAC_CON1, MOD_DAI_MODE_SFT, MOD_DAI_MODE_MASK, -1, MOD_DAI_DATA_SFT, MOD_DAI_ON_SFT, MOD_DAI_HD_SFT, MOD_DAI_HD_ALIGN_SFT),
    /* enable control in tdm for sync start */
    MT8183_MEMIF_BASE!(MT8183_MEMIF_HDMI, c"HDMI", MT8183_MEMIF_HDMI, -1, AFE_HDMI_BASE, AFE_HDMI_CUR, AFE_HDMI_END, AFE_HDMI_BASE_MSB, AFE_HDMI_CUR_MSB, AFE_HDMI_END_MSB, -1, HDMI_MODE_SFT, HDMI_MODE_MASK, -1, HDMI_DATA_SFT, HDMI_ON_SFT, HDMI_HD_SFT, HDMI_HD_ALIGN_SFT),
];

macro_rules! MT8183_AFE_IRQ_BASE {
    ($id:expr, $fs_reg:expr, $fs_shift:expr, $fs_maskbit:expr, $cnt_reg:expr,
     $en_shift:expr, $clr_shift:expr) => {
        mtk_base_irq_data {
            id: $id,
            irq_cnt_reg: $cnt_reg,
            irq_cnt_shift: 0,
            irq_cnt_maskbit: 0x3ffff,
            irq_fs_reg: $fs_reg,
            irq_fs_shift: $fs_shift,
            irq_fs_maskbit: $fs_maskbit,
            irq_en_reg: AFE_IRQ_MCU_CON0,
            irq_en_shift: $en_shift,
            irq_clr_reg: AFE_IRQ_MCU_CLR,
            irq_clr_shift: $clr_shift,
        }
    };
}

static irq_data: [mtk_base_irq_data; MT8183_IRQ_NUM as usize] = [
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_0, AFE_IRQ_MCU_CON1 + 0 / 8 * 4, IRQ0_MCU_MODE_SFT, IRQ0_MCU_MODE_MASK, AFE_IRQ_MCU_CNT0, IRQ0_MCU_ON_SFT, IRQ0_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_1, AFE_IRQ_MCU_CON1 + 1 / 8 * 4, IRQ1_MCU_MODE_SFT, IRQ1_MCU_MODE_MASK, AFE_IRQ_MCU_CNT1, IRQ1_MCU_ON_SFT, IRQ1_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_2, AFE_IRQ_MCU_CON1 + 2 / 8 * 4, IRQ2_MCU_MODE_SFT, IRQ2_MCU_MODE_MASK, AFE_IRQ_MCU_CNT2, IRQ2_MCU_ON_SFT, IRQ2_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_3, AFE_IRQ_MCU_CON1 + 3 / 8 * 4, IRQ3_MCU_MODE_SFT, IRQ3_MCU_MODE_MASK, AFE_IRQ_MCU_CNT3, IRQ3_MCU_ON_SFT, IRQ3_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_4, AFE_IRQ_MCU_CON1 + 4 / 8 * 4, IRQ4_MCU_MODE_SFT, IRQ4_MCU_MODE_MASK, AFE_IRQ_MCU_CNT4, IRQ4_MCU_ON_SFT, IRQ4_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_5, AFE_IRQ_MCU_CON1 + 5 / 8 * 4, IRQ5_MCU_MODE_SFT, IRQ5_MCU_MODE_MASK, AFE_IRQ_MCU_CNT5, IRQ5_MCU_ON_SFT, IRQ5_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_6, AFE_IRQ_MCU_CON1 + 6 / 8 * 4, IRQ6_MCU_MODE_SFT, IRQ6_MCU_MODE_MASK, AFE_IRQ_MCU_CNT6, IRQ6_MCU_ON_SFT, IRQ6_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_7, AFE_IRQ_MCU_CON1 + 7 / 8 * 4, IRQ7_MCU_MODE_SFT, IRQ7_MCU_MODE_MASK, AFE_IRQ_MCU_CNT7, IRQ7_MCU_ON_SFT, IRQ7_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_8, -1, -1, -1, AFE_IRQ_MCU_CNT8, IRQ8_MCU_ON_SFT, IRQ8_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_11, AFE_IRQ_MCU_CON1 + 11 / 8 * 4, IRQ11_MCU_MODE_SFT, IRQ11_MCU_MODE_MASK, AFE_IRQ_MCU_CNT11, IRQ11_MCU_ON_SFT, IRQ11_MCU_CLR_SFT),
    MT8183_AFE_IRQ_BASE!(MT8183_IRQ_12, AFE_IRQ_MCU_CON1 + 12 / 8 * 4, IRQ12_MCU_MODE_SFT, IRQ12_MCU_MODE_MASK, AFE_IRQ_MCU_CNT12, IRQ12_MCU_ON_SFT, IRQ12_MCU_CLR_SFT),
];

unsafe extern "C" fn mt8183_is_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    /* these auto-gen reg has read-only bit, so put it as volatile */
    /* volatile reg cannot be cached, so cannot be set when power off */
    match reg {
        AUDIO_TOP_CON0..=AUDIO_TOP_CON1
        | AUDIO_TOP_CON3
        | AFE_DL1_CUR..=AFE_DL1_END
        | AFE_DL2_CUR..=AFE_DL2_END
        | AFE_AWB_END..=AFE_AWB_CUR
        | AFE_VUL_END..=AFE_VUL_CUR
        | AFE_MEMIF_MON0..=AFE_MEMIF_MON9
        | AFE_ADDA_SRC_DEBUG_MON0..=AFE_ADDA_SRC_DEBUG_MON1
        | AFE_ADDA_UL_SRC_MON0..=AFE_ADDA_UL_SRC_MON1
        | AFE_SIDETONE_MON
        | AFE_SIDETONE_CON0..=AFE_SIDETONE_COEFF
        | AFE_BUS_MON0
        | AFE_MRGIF_MON0..=AFE_I2S_MON
        | AFE_DAC_MON
        | AFE_VUL2_END..=AFE_VUL2_CUR
        | AFE_IRQ0_MCU_CNT_MON..=AFE_IRQ6_MCU_CNT_MON
        | AFE_MOD_DAI_END..=AFE_MOD_DAI_CUR
        | AFE_VUL_D2_END..=AFE_VUL_D2_CUR
        | AFE_DL3_CUR..=AFE_DL3_END
        | AFE_HDMI_OUT_CON0
        | AFE_HDMI_OUT_CUR..=AFE_HDMI_OUT_END
        | AFE_IRQ3_MCU_CNT_MON..=AFE_IRQ4_MCU_CNT_MON
        | AFE_IRQ_MCU_STATUS..=AFE_IRQ_MCU_CLR
        | AFE_IRQ_MCU_MON2
        | AFE_IRQ1_MCU_CNT_MON..=AFE_IRQ5_MCU_CNT_MON
        | AFE_IRQ7_MCU_CNT_MON
        | AFE_GAIN1_CUR
        | AFE_GAIN2_CUR
        | AFE_SRAM_DELSEL_CON0
        | AFE_SRAM_DELSEL_CON2..=AFE_SRAM_DELSEL_CON3
        | AFE_ASRC_2CH_CON12..=AFE_ASRC_2CH_CON13
        | PCM_INTF_CON2
        | FPGA_CFG0..=FPGA_CFG1
        | FPGA_CFG2..=FPGA_CFG3
        | AUDIO_TOP_DBG_MON0..=AUDIO_TOP_DBG_MON1
        | AFE_IRQ8_MCU_CNT_MON..=AFE_IRQ12_MCU_CNT_MON
        | AFE_CBIP_MON0
        | AFE_CBIP_SLV_MUX_MON0..=AFE_CBIP_SLV_DECODER_MON0
        | AFE_ADDA6_SRC_DEBUG_MON0
        | AFE_ADD6A_UL_SRC_MON0..=AFE_ADDA6_UL_SRC_MON1
        | AFE_DL1_CUR_MSB
        | AFE_DL2_CUR_MSB
        | AFE_AWB_CUR_MSB
        | AFE_VUL_CUR_MSB
        | AFE_VUL2_CUR_MSB
        | AFE_MOD_DAI_CUR_MSB
        | AFE_VUL_D2_CUR_MSB
        | AFE_DL3_CUR_MSB
        | AFE_HDMI_OUT_CUR_MSB
        | AFE_AWB2_END..=AFE_AWB2_CUR
        | AFE_AWB2_CUR_MSB
        | AFE_ADDA_DL_SDM_FIFO_MON..=AFE_ADDA_DL_SDM_OUT_MON
        | AFE_CONNSYS_I2S_MON..=AFE_ASRC_2CH_CON0
        | AFE_ASRC_2CH_CON2..=AFE_ASRC_2CH_CON5
        | AFE_ASRC_2CH_CON7..=AFE_ASRC_2CH_CON8
        | AFE_MEMIF_MON12..=AFE_MEMIF_MON24
        | AFE_ADDA_MTKAIF_MON0..=AFE_ADDA_MTKAIF_MON1
        | AFE_AUD_PAD_TOP
        | AFE_GENERAL1_ASRC_2CH_CON0
        | AFE_GENERAL1_ASRC_2CH_CON2..=AFE_GENERAL1_ASRC_2CH_CON5
        | AFE_GENERAL1_ASRC_2CH_CON7..=AFE_GENERAL1_ASRC_2CH_CON8
        | AFE_GENERAL1_ASRC_2CH_CON12..=AFE_GENERAL1_ASRC_2CH_CON13
        | AFE_GENERAL2_ASRC_2CH_CON0
        | AFE_GENERAL2_ASRC_2CH_CON2..=AFE_GENERAL2_ASRC_2CH_CON5
        | AFE_GENERAL2_ASRC_2CH_CON7..=AFE_GENERAL2_ASRC_2CH_CON8
        | AFE_GENERAL2_ASRC_2CH_CON12..=AFE_GENERAL2_ASRC_2CH_CON13 => true,
        _ => false,
    }
}

static mt8183_afe_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    volatile_reg: Some(mt8183_is_volatile_reg),
    max_register: AFE_MAX_REGISTER,
    num_reg_defaults_raw: AFE_MAX_REGISTER,
    cache_type: REGCACHE_FLAT,
    ..regmap_config::zeroed()
};

unsafe extern "C" fn mt8183_afe_irq_handler(
    _irq_id: c_int,
    dev: *mut c_void,
) -> irqreturn_t {
    let afe = dev as *mut mtk_base_afe;
    let mut status: c_uint = 0;
    let mut mcu_en: c_uint = 0;
    let mut irq_ret: irqreturn_t = IRQ_HANDLED;

    /* get irq that is sent to MCU */
    regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut mcu_en);

    let ret = regmap_read((*afe).regmap, AFE_IRQ_MCU_STATUS, &mut status);
    /* only care IRQ which is sent to MCU */
    let status_mcu = status & mcu_en & AFE_IRQ_STATUS_BITS;

    if ret != 0 || status_mcu == 0 {
        dev_err(
            (*afe).dev,
            c"%s(), irq status err, ret %d, status 0x%x, mcu_en 0x%x\n".as_ptr(),
            c"mt8183_afe_irq_handler".as_ptr(),
            ret,
            status,
            mcu_en,
        );

        irq_ret = IRQ_NONE;
    } else {
        let mut i = 0;
        while i < MT8183_MEMIF_NUM {
            let memif = (*afe).memif.add(i as usize);

            if (*memif).substream.is_null() {
                i += 1;
                continue;
            }

            if (*memif).irq_usage < 0 {
                i += 1;
                continue;
            }

            let irq = (*afe).irqs.add((*memif).irq_usage as usize);

            if (status_mcu & (1 << (*(*irq).irq_data).irq_en_shift)) != 0 {
                snd_pcm_period_elapsed((*memif).substream);
            }
            i += 1;
        }
    }

    /* clear irq */
    regmap_write((*afe).regmap, AFE_IRQ_MCU_CLR, status_mcu);

    irq_ret
}

unsafe extern "C" fn mt8183_afe_runtime_suspend(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8183_afe_private;
    let mut value: c_uint = 0;

    if !(*afe).regmap.is_null() && !(*afe_priv).pm_runtime_bypass_reg_ctl {
        /* disable AFE */
        regmap_update_bits((*afe).regmap, AFE_DAC_CON0, AFE_ON_MASK_SFT, 0x0);

        let ret = regmap_read_poll_timeout!(
            (*afe).regmap,
            AFE_DAC_MON,
            value,
            (value & AFE_ON_RETM_MASK_SFT) == 0,
            20,
            1 * 1000 * 1000
        );
        if ret != 0 {
            dev_warn(
                (*afe).dev,
                c"%s(), ret %d\n".as_ptr(),
                c"mt8183_afe_runtime_suspend".as_ptr(),
                ret,
            );
        }

        /* make sure all irq status are cleared, twice intended */
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_CLR, 0xffff, 0xffff);
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_CLR, 0xffff, 0xffff);

        /* cache only */
        regcache_cache_only((*afe).regmap, true);
        regcache_mark_dirty((*afe).regmap);
    }

    mt8183_afe_disable_clock(afe)
}

unsafe extern "C" fn mt8183_afe_runtime_resume(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8183_afe_private;

    let ret = mt8183_afe_enable_clock(afe);
    if ret != 0 {
        return ret;
    }

    if !(*afe).regmap.is_null() && !(*afe_priv).pm_runtime_bypass_reg_ctl {
        regcache_cache_only((*afe).regmap, false);
        regcache_sync((*afe).regmap);

        /* enable audio sys DCM for power saving */
        regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, 0x1 << 29, 0x1 << 29);

        /* force cpu use 8_24 format when writing 32bit data */
        regmap_update_bits(
            (*afe).regmap,
            AFE_MEMIF_MSB,
            CPU_HD_ALIGN_MASK_SFT,
            0 << CPU_HD_ALIGN_SFT,
        );

        /* set all output port to 24bit */
        regmap_write((*afe).regmap, AFE_CONN_24BIT, 0xffffffff);
        regmap_write((*afe).regmap, AFE_CONN_24BIT_1, 0xffffffff);

        /* enable AFE */
        regmap_update_bits((*afe).regmap, AFE_DAC_CON0, 0x1, 0x1);
    }

    0
}

unsafe extern "C" fn mt8183_dai_memif_register(afe: *mut mtk_base_afe) -> c_int {
    let dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mt8183_memif_dai_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mt8183_memif_dai_driver.len() as c_int;

    (*dai).dapm_widgets = mt8183_memif_widgets.as_ptr();
    (*dai).num_dapm_widgets = mt8183_memif_widgets.len() as c_int;
    (*dai).dapm_routes = mt8183_memif_routes.as_ptr();
    (*dai).num_dapm_routes = mt8183_memif_routes.len() as c_int;
    0
}

type dai_register_cb = unsafe extern "C" fn(*mut mtk_base_afe) -> c_int;
static dai_register_cbs: [dai_register_cb; 6] = [
    mt8183_dai_adda_register,
    mt8183_dai_i2s_register,
    mt8183_dai_pcm_register,
    mt8183_dai_tdm_register,
    mt8183_dai_hostless_register,
    mt8183_dai_memif_register,
];

unsafe extern "C" fn mt8183_afe_release_reserved_mem(data: *mut c_void) {
    of_reserved_mem_device_release(data);
}

unsafe extern "C" fn mt8183_afe_pcm_dev_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: c_int;

    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(34));
    if ret != 0 {
        return ret;
    }

    let afe = devm_kzalloc(dev, size_of::<mtk_base_afe>(), GFP_KERNEL) as *mut mtk_base_afe;
    if afe.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, afe as *mut c_void);

    (*afe).platform_priv =
        devm_kzalloc(dev, size_of::<mt8183_afe_private>(), GFP_KERNEL) as *mut c_void;
    if (*afe).platform_priv.is_null() {
        return -ENOMEM;
    }

    let afe_priv = (*afe).platform_priv as *mut mt8183_afe_private;
    (*afe).dev = dev;

    ret = of_reserved_mem_device_init(dev);
    if ret != 0 {
        dev_info(
            dev,
            c"no reserved memory found, pre-allocating buffers instead\n".as_ptr(),
        );
        (*afe).preallocate_buffers = true;
    } else {
        ret = devm_add_action_or_reset(dev, Some(mt8183_afe_release_reserved_mem), dev as *mut c_void);
        if ret != 0 {
            return ret;
        }
    }

    /* initial audio related clock */
    ret = mt8183_init_clock(afe);
    if ret != 0 {
        dev_err(dev, c"init clock error\n".as_ptr());
        return ret;
    }

    pm_runtime_enable(dev);

    /* regmap init */
    (*afe).regmap = syscon_node_to_regmap((*(*dev).parent).of_node);
    if IS_ERR((*afe).regmap as *const c_void) {
        dev_err(dev, c"could not get regmap from parent\n".as_ptr());
        ret = PTR_ERR((*afe).regmap as *const c_void);
        goto_err_pm_disable(dev, ret)
    } else {
        ret = regmap_attach_dev(dev, (*afe).regmap, &mt8183_afe_regmap_config);
        if ret != 0 {
            dev_warn(dev, c"regmap_attach_dev fail, ret %d\n".as_ptr(), ret);
            return goto_err_pm_disable(dev, ret);
        }

        let rstc = devm_reset_control_get(dev, c"audiosys".as_ptr());
        if IS_ERR(rstc as *const c_void) {
            ret = PTR_ERR(rstc as *const c_void);
            dev_err(dev, c"could not get audiosys reset:%d\n".as_ptr(), ret);
            return goto_err_pm_disable(dev, ret);
        }

        ret = reset_control_reset(rstc);
        if ret != 0 {
            dev_err(dev, c"failed to trigger audio reset:%d\n".as_ptr(), ret);
            return goto_err_pm_disable(dev, ret);
        }

        /* enable clock for regcache get default value from hw */
        (*afe_priv).pm_runtime_bypass_reg_ctl = true;
        ret = pm_runtime_resume_and_get(dev);
        if ret != 0 {
            (*afe_priv).pm_runtime_bypass_reg_ctl = false;
            return goto_err_pm_disable(dev, ret);
        }

        ret = regmap_reinit_cache((*afe).regmap, &mt8183_afe_regmap_config);
        pm_runtime_put_sync(dev);
        (*afe_priv).pm_runtime_bypass_reg_ctl = false;

        if ret != 0 {
            dev_err(dev, c"regmap_reinit_cache fail, ret %d\n".as_ptr(), ret);
            return goto_err_pm_disable(dev, ret);
        }

        regcache_cache_only((*afe).regmap, true);
        regcache_mark_dirty((*afe).regmap);

        /* init memif */
        (*afe).memif_size = MT8183_MEMIF_NUM;
        (*afe).memif = devm_kcalloc(
            dev,
            (*afe).memif_size as usize,
            size_of::<mtk_base_afe_memif>(),
            GFP_KERNEL,
        ) as *mut mtk_base_afe_memif;
        if (*afe).memif.is_null() {
            ret = -ENOMEM;
            return goto_err_pm_disable(dev, ret);
        }

        let mut i = 0;
        while i < (*afe).memif_size {
            (*(*afe).memif.add(i as usize)).data = &memif_data[i as usize];
            (*(*afe).memif.add(i as usize)).irq_usage = -1;
            i += 1;
        }

        (*(*afe).memif.add(MT8183_MEMIF_HDMI as usize)).irq_usage = MT8183_IRQ_8;
        (*(*afe).memif.add(MT8183_MEMIF_HDMI as usize)).const_irq = 1;

        mutex_init(&mut (*afe).irq_alloc_lock);

        /* init memif */
        /* irq initialize */
        (*afe).irqs_size = MT8183_IRQ_NUM;
        (*afe).irqs = devm_kcalloc(
            dev,
            (*afe).irqs_size as usize,
            size_of::<mtk_base_afe_irq>(),
            GFP_KERNEL,
        ) as *mut mtk_base_afe_irq;
        if (*afe).irqs.is_null() {
            ret = -ENOMEM;
            return goto_err_pm_disable(dev, ret);
        }

        i = 0;
        while i < (*afe).irqs_size {
            (*(*afe).irqs.add(i as usize)).irq_data = &irq_data[i as usize];
            i += 1;
        }

        /* request irq */
        let irq_id = platform_get_irq(pdev, 0);
        if irq_id < 0 {
            ret = irq_id;
            return goto_err_pm_disable(dev, ret);
        }

        ret = devm_request_irq(
            dev,
            irq_id,
            Some(mt8183_afe_irq_handler),
            IRQF_TRIGGER_NONE,
            c"asys-isr".as_ptr(),
            afe as *mut c_void,
        );
        if ret != 0 {
            dev_err(dev, c"could not request_irq for asys-isr\n".as_ptr());
            return goto_err_pm_disable(dev, ret);
        }

        /* init sub_dais */
        INIT_LIST_HEAD(&mut (*afe).sub_dais);

        i = 0;
        while (i as usize) < dai_register_cbs.len() {
            ret = dai_register_cbs[i as usize](afe);
            if ret != 0 {
                dev_warn(dev, c"dai register i %d fail, ret %d\n".as_ptr(), i, ret);
                return goto_err_pm_disable(dev, ret);
            }
            i += 1;
        }

        /* init dai_driver and component_driver */
        ret = mtk_afe_combine_sub_dai(afe);
        if ret != 0 {
            dev_warn(dev, c"mtk_afe_combine_sub_dai fail, ret %d\n".as_ptr(), ret);
            return goto_err_pm_disable(dev, ret);
        }

        (*afe).mtk_afe_hardware = &mt8183_afe_hardware;
        (*afe).memif_fs = Some(mt8183_memif_fs);
        (*afe).irq_fs = Some(mt8183_irq_fs);

        (*afe).runtime_resume = Some(mt8183_afe_runtime_resume);
        (*afe).runtime_suspend = Some(mt8183_afe_runtime_suspend);

        /* register component */
        ret = devm_snd_soc_register_component(dev, &mtk_afe_pcm_platform, ptr::null_mut(), 0);
        if ret != 0 {
            dev_warn(dev, c"err_platform\n".as_ptr());
            return goto_err_pm_disable(dev, ret);
        }

        ret = devm_snd_soc_register_component(
            dev,
            &mt8183_afe_pcm_dai_component,
            (*afe).dai_drivers,
            (*afe).num_dai_drivers,
        );
        if ret != 0 {
            dev_warn(dev, c"err_dai_component\n".as_ptr());
            return goto_err_pm_disable(dev, ret);
        }

        ret
    }
}

unsafe fn goto_err_pm_disable(dev: *mut device, ret: c_int) -> c_int {
    pm_runtime_disable(dev);
    ret
}

unsafe extern "C" fn mt8183_afe_pcm_dev_remove(pdev: *mut platform_device) {
    let dev = &mut (*pdev).dev as *mut device;

    pm_runtime_disable(dev);
    if !pm_runtime_status_suspended(dev) {
        mt8183_afe_runtime_suspend(dev);
    }
}

static mt8183_afe_pcm_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"mediatek,mt8183-audio".as_ptr(),
        ..of_device_id::zeroed()
    },
    of_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(of, mt8183_afe_pcm_dt_match);

static mt8183_afe_pm_ops: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(mt8183_afe_runtime_suspend, mt8183_afe_runtime_resume, NULL)
    runtime_suspend: Some(mt8183_afe_runtime_suspend),
    runtime_resume: Some(mt8183_afe_runtime_resume),
    ..dev_pm_ops::zeroed()
};

static mut mt8183_afe_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"mt8183-audio".as_ptr(),
        of_match_table: mt8183_afe_pcm_dt_match.as_ptr(),
        pm: pm_ptr!(&mt8183_afe_pm_ops),
        ..device_driver::zeroed()
    },
    probe: Some(mt8183_afe_pcm_dev_probe),
    remove: Some(mt8183_afe_pcm_dev_remove),
    ..platform_driver::zeroed()
};

module_platform_driver!(mt8183_afe_pcm_driver);

MODULE_DESCRIPTION!(c"Mediatek ALSA SoC AFE platform driver for 8183");
MODULE_AUTHOR!(c"KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
MODULE_LICENSE!(c"GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
