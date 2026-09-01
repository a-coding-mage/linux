// SPDX-License-Identifier: GPL-2.0
//
// Ingenic JZ4760 CODEC driver
//
// Copyright (C) 2021, Christophe Branchereau <cbranchereau@gmail.com>
// Copyright (C) 2021, Paul Cercueil <paul@crapouillou.net>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u32 = u32;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const fn field_shift(mask: c_uint) -> c_uint {
    mask.trailing_zeros()
}

const fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint {
    (val << field_shift(mask)) & mask
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const USEC_PER_SEC: c_uint = 1_000_000;

const true_: bool = true;
const false_: bool = false;

const ICDC_RGADW_OFFSET: usize = 0x00;
const ICDC_RGDATA_OFFSET: usize = 0x04;

/* ICDC internal register access control register(RGADW) */
const ICDC_RGADW_RGWR: c_uint = BIT(16);
const ICDC_RGADW_RGADDR_MASK: c_uint = GENMASK(14, 8);
const ICDC_RGADW_RGDIN_MASK: c_uint = GENMASK(7, 0);

/* ICDC internal register data output register (RGDATA)*/
const ICDC_RGDATA_IRQ: c_uint = BIT(8);
const ICDC_RGDATA_RGDOUT_MASK: c_uint = GENMASK(7, 0);

/* Internal register space, accessed through regmap */
const JZ4760_CODEC_REG_SR: c_uint = 0;
const JZ4760_CODEC_REG_AICR: c_uint = 1;
const JZ4760_CODEC_REG_CR1: c_uint = 2;
const JZ4760_CODEC_REG_CR2: c_uint = 3;
const JZ4760_CODEC_REG_CR3: c_uint = 4;
const JZ4760_CODEC_REG_CR4: c_uint = 5;
const JZ4760_CODEC_REG_CCR1: c_uint = 6;
const JZ4760_CODEC_REG_CCR2: c_uint = 7;
const JZ4760_CODEC_REG_PMR1: c_uint = 8;
const JZ4760_CODEC_REG_PMR2: c_uint = 9;
const JZ4760_CODEC_REG_ICR: c_uint = 10;
const JZ4760_CODEC_REG_IFR: c_uint = 11;
const JZ4760_CODEC_REG_GCR1: c_uint = 12;
const JZ4760_CODEC_REG_GCR2: c_uint = 13;
const JZ4760_CODEC_REG_GCR3: c_uint = 14;
const JZ4760_CODEC_REG_GCR4: c_uint = 15;
const JZ4760_CODEC_REG_GCR5: c_uint = 16;
const JZ4760_CODEC_REG_GCR6: c_uint = 17;
const JZ4760_CODEC_REG_GCR7: c_uint = 18;
const JZ4760_CODEC_REG_GCR8: c_uint = 19;
const JZ4760_CODEC_REG_GCR9: c_uint = 20;
const JZ4760_CODEC_REG_AGC1: c_uint = 21;
const JZ4760_CODEC_REG_AGC2: c_uint = 22;
const JZ4760_CODEC_REG_AGC3: c_uint = 23;
const JZ4760_CODEC_REG_AGC4: c_uint = 24;
const JZ4760_CODEC_REG_AGC5: c_uint = 25;
const JZ4760_CODEC_REG_MIX1: c_uint = 26;
const JZ4760_CODEC_REG_MIX2: c_uint = 27;

const REG_AICR_DAC_ADWL_MASK: c_uint = GENMASK(7, 6);
const REG_AICR_DAC_SERIAL: c_uint = BIT(3);
const REG_AICR_DAC_I2S: c_uint = BIT(1);

const REG_AICR_ADC_ADWL_MASK: c_uint = GENMASK(5, 4);

const REG_AICR_ADC_SERIAL: c_uint = BIT(2);
const REG_AICR_ADC_I2S: c_uint = BIT(0);

const REG_CR1_HP_LOAD: c_uint = BIT(7);
const REG_CR1_HP_MUTE: c_uint = BIT(5);
const REG_CR1_LO_MUTE_OFFSET: c_uint = 4;
const REG_CR1_BTL_MUTE_OFFSET: c_uint = 3;
const REG_CR1_OUTSEL_OFFSET: c_uint = 0;
const REG_CR1_OUTSEL_MASK: c_uint = GENMASK(1, REG_CR1_OUTSEL_OFFSET);

const REG_CR2_DAC_MONO: c_uint = BIT(7);
const REG_CR2_DAC_MUTE: c_uint = BIT(5);
const REG_CR2_DAC_NOMAD: c_uint = BIT(1);
const REG_CR2_DAC_RIGHT_ONLY: c_uint = BIT(0);

const REG_CR3_ADC_INSEL_OFFSET: c_uint = 2;
const REG_CR3_ADC_INSEL_MASK: c_uint = GENMASK(3, REG_CR3_ADC_INSEL_OFFSET);
const REG_CR3_MICSTEREO_OFFSET: c_uint = 1;
const REG_CR3_MICDIFF_OFFSET: c_uint = 0;

const REG_CR4_ADC_HPF_OFFSET: c_uint = 7;
const REG_CR4_ADC_RIGHT_ONLY: c_uint = BIT(0);

const REG_CCR1_CRYSTAL_MASK: c_uint = GENMASK(3, 0);

const REG_CCR2_DAC_FREQ_MASK: c_uint = GENMASK(7, 4);
const REG_CCR2_ADC_FREQ_MASK: c_uint = GENMASK(3, 0);

const REG_PMR1_SB: c_uint = BIT(7);
const REG_PMR1_SB_SLEEP: c_uint = BIT(6);
const REG_PMR1_SB_AIP_OFFSET: c_uint = 5;
const REG_PMR1_SB_LINE_OFFSET: c_uint = 4;
const REG_PMR1_SB_MIC1_OFFSET: c_uint = 3;
const REG_PMR1_SB_MIC2_OFFSET: c_uint = 2;
const REG_PMR1_SB_BYPASS_OFFSET: c_uint = 1;
const REG_PMR1_SB_MICBIAS_OFFSET: c_uint = 0;

const REG_PMR2_SB_ADC_OFFSET: c_uint = 4;
const REG_PMR2_SB_HP_OFFSET: c_uint = 3;
const REG_PMR2_SB_BTL_OFFSET: c_uint = 2;
const REG_PMR2_SB_LOUT_OFFSET: c_uint = 1;
const REG_PMR2_SB_DAC_OFFSET: c_uint = 0;

const REG_ICR_INT_FORM_MASK: c_uint = GENMASK(7, 6);
const REG_ICR_ALL_MASK: c_uint = GENMASK(5, 0);
const REG_ICR_JACK_MASK: c_uint = BIT(5);
const REG_ICR_SCMC_MASK: c_uint = BIT(4);
const REG_ICR_RUP_MASK: c_uint = BIT(3);
const REG_ICR_RDO_MASK: c_uint = BIT(2);
const REG_ICR_GUP_MASK: c_uint = BIT(1);
const REG_ICR_GDO_MASK: c_uint = BIT(0);

const REG_IFR_ALL_MASK: c_uint = GENMASK(5, 0);
const REG_IFR_JACK: c_uint = BIT(6);
const REG_IFR_JACK_EVENT: c_uint = BIT(5);
const REG_IFR_SCMC: c_uint = BIT(4);
const REG_IFR_RUP: c_uint = BIT(3);
const REG_IFR_RDO: c_uint = BIT(2);
const REG_IFR_GUP: c_uint = BIT(1);
const REG_IFR_GDO: c_uint = BIT(0);

const REG_GCR_GAIN_OFFSET: c_uint = 0;
const REG_GCR_GAIN_MAX: c_uint = 0x1f;

const REG_GCR_RL: c_uint = BIT(7);

const REG_GCR_GIM1_MASK: c_uint = GENMASK(5, 3);
const REG_GCR_GIM2_MASK: c_uint = GENMASK(2, 0);
const REG_GCR_GIM_GAIN_MAX: c_uint = 7;

const REG_AGC1_EN: c_uint = BIT(7);
const REG_AGC1_TARGET_MASK: c_uint = GENMASK(5, 2);

const REG_AGC2_NG_THR_MASK: c_uint = GENMASK(6, 4);
const REG_AGC2_HOLD_MASK: c_uint = GENMASK(3, 0);

const REG_AGC3_ATK_MASK: c_uint = GENMASK(7, 4);
const REG_AGC3_DCY_MASK: c_uint = GENMASK(3, 0);

const REG_AGC4_AGC_MAX_MASK: c_uint = GENMASK(4, 0);

const REG_AGC5_AGC_MIN_MASK: c_uint = GENMASK(4, 0);

const REG_MIX1_MIX_REC_MASK: c_uint = GENMASK(7, 6);
const REG_MIX1_GIMIX_MASK: c_uint = GENMASK(4, 0);

const REG_MIX2_DAC_MIX_MASK: c_uint = GENMASK(7, 6);
const REG_MIX2_GOMIX_MASK: c_uint = GENMASK(4, 0);

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
struct clk {
    _private: [u8; 0],
}
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_widget_data {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}
#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget_data,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    suspend_bias_off: c_uint,
    use_pmdown_time: c_uint,
}
#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    reg_defaults_raw: *const u8,
    num_reg_defaults_raw: c_uint,
    cache_type: c_uint,
}
#[repr(C)]
struct platform_driver_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}
#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: platform_driver_driver,
}

type snd_soc_bias_level = c_uint;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 3;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;

const SNDRV_PCM_FORMAT_S16_LE: c_int = 0;
const SNDRV_PCM_FORMAT_S18_3LE: c_int = 1;
const SNDRV_PCM_FORMAT_S20_3LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_3LE: c_int = 3;

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S18_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S18_3LE;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S20_3LE;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S24_3LE;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;

const SND_SOC_DAPM_PRE_PMU: c_int = 1 << 0;
const SND_SOC_DAPM_POST_PMU: c_int = 1 << 1;
const SND_SOC_DAPM_POST_PMD: c_int = 1 << 2;
const SND_SOC_NOPM: c_int = -1;
const REGCACHE_FLAT: c_uint = 0;

/* codec private data */
#[repr(C)]
struct jz_codec {
    dev: *mut device,
    regmap: *mut regmap,
    base: *mut c_void,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn msleep(msecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn readl(addr: *const c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, context: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn readl_poll_timeout(addr: *const c_void, val: *mut u32, cond: unsafe extern "C" fn(u32) -> bool, sleep_us: c_uint, timeout_us: c_uint) -> c_int;
    fn regmap_read_poll_timeout(map: *mut regmap, reg: c_uint, val: *mut c_uint, mask: c_uint, sleep_us: c_uint, timeout_us: c_uint) -> c_int;
}

unsafe extern "C" fn jz4760_codec_set_bias_level(
    codec: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let regmap = (*jz_codec).regmap;

    match level {
        SND_SOC_BIAS_PREPARE => {
            /* Reset all interrupt flags. */
            regmap_write(regmap, JZ4760_CODEC_REG_IFR, REG_IFR_ALL_MASK);

            regmap_clear_bits(regmap, JZ4760_CODEC_REG_PMR1, REG_PMR1_SB);
            msleep(250);
            regmap_clear_bits(regmap, JZ4760_CODEC_REG_PMR1, REG_PMR1_SB_SLEEP);
            msleep(400);
        }
        SND_SOC_BIAS_STANDBY => {
            regmap_set_bits(regmap, JZ4760_CODEC_REG_PMR1, REG_PMR1_SB_SLEEP);
            regmap_set_bits(regmap, JZ4760_CODEC_REG_PMR1, REG_PMR1_SB);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn jz4760_codec_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec = (*dai).component;
    let dapm = snd_soc_component_to_dapm(codec);
    let mut ret: c_int = 0;

    /*
     * SYSCLK output from the codec to the AIC is required to keep the
     * DMA transfer going during playback when all audible outputs have
     * been disabled.
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = snd_soc_dapm_force_enable_pin(dapm, c"SYSCLK".as_ptr());
    }
    ret
}

unsafe extern "C" fn jz4760_codec_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let codec = (*dai).component;
    let dapm = snd_soc_component_to_dapm(codec);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_dapm_disable_pin(dapm, c"SYSCLK".as_ptr());
    }
}

unsafe extern "C" fn jz4760_codec_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec = (*dai).component;
    let dapm = snd_soc_component_to_dapm(codec);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
                snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_ON);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /* do nothing */
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn jz4760_codec_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let codec = (*dai).component;
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let gain_bit: c_uint = if mute != 0 { REG_IFR_GDO } else { REG_IFR_GUP };
    let mut val: c_uint = 0;
    let mut reg: c_uint = 0;
    let change: c_int;
    let err: c_int;

    change = snd_soc_component_update_bits(
        codec,
        JZ4760_CODEC_REG_CR2,
        REG_CR2_DAC_MUTE,
        if mute != 0 { REG_CR2_DAC_MUTE } else { 0 },
    );
    if change == 1 {
        regmap_read((*jz_codec).regmap, JZ4760_CODEC_REG_PMR2, &mut val);

        if (val & BIT(REG_PMR2_SB_DAC_OFFSET)) != 0 {
            return 1;
        }

        err = regmap_read_poll_timeout(
            (*jz_codec).regmap,
            JZ4760_CODEC_REG_IFR,
            &mut val,
            gain_bit,
            1000,
            1 * USEC_PER_SEC,
        );
        if err != 0 {
            dev_err(
                (*jz_codec).dev,
                c"Timeout while setting digital mute: %d".as_ptr(),
                err,
            );
            return err;
        }

        /* clear GUP/GDO flag */
        regmap_write((*jz_codec).regmap, JZ4760_CODEC_REG_IFR, gain_bit);
    }

    regmap_read((*jz_codec).regmap, JZ4760_CODEC_REG_CR2, &mut reg);

    0
}

/* unit: 0.01dB */
/* static const DECLARE_TLV_DB_MINMAX_MUTE(dac_tlv, -3100, 100); */
/* static const DECLARE_TLV_DB_SCALE(adc_tlv, 0, 100, 0); */
/* static const DECLARE_TLV_DB_MINMAX(out_tlv, -2500, 100); */
/* static const DECLARE_TLV_DB_SCALE(linein_tlv, -2500, 100, 0); */
/* static const DECLARE_TLV_DB_MINMAX(mixer_tlv, -3100, 0); */

/* Unconditional controls. */
/* The snd_kcontrol_new arrays below are generated by ALSA SOC_* C macros. */
static jz4760_codec_snd_controls: [snd_kcontrol_new; 0] = [];
static jz4760_codec_pcm_playback_controls: [snd_kcontrol_new; 0] = [];
static jz4760_codec_hp_playback_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn hpout_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let codec = snd_soc_dapm_to_component((*w).dapm);
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let mut val: c_uint = 0;
    let err: c_int;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* unmute HP */
            regmap_clear_bits((*jz_codec).regmap, JZ4760_CODEC_REG_CR1, REG_CR1_HP_MUTE);
        }

        SND_SOC_DAPM_POST_PMU => {
            /* wait for ramp-up complete (RUP) */
            err = regmap_read_poll_timeout(
                (*jz_codec).regmap,
                JZ4760_CODEC_REG_IFR,
                &mut val,
                REG_IFR_RUP,
                1000,
                1 * USEC_PER_SEC,
            );
            if err != 0 {
                dev_err((*jz_codec).dev, c"RUP timeout: %d".as_ptr(), err);
                return err;
            }

            /* clear RUP flag */
            regmap_set_bits((*jz_codec).regmap, JZ4760_CODEC_REG_IFR, REG_IFR_RUP);
        }

        SND_SOC_DAPM_POST_PMD => {
            /* mute HP */
            regmap_set_bits((*jz_codec).regmap, JZ4760_CODEC_REG_CR1, REG_CR1_HP_MUTE);

            err = regmap_read_poll_timeout(
                (*jz_codec).regmap,
                JZ4760_CODEC_REG_IFR,
                &mut val,
                REG_IFR_RDO,
                1000,
                1 * USEC_PER_SEC,
            );
            if err != 0 {
                dev_err((*jz_codec).dev, c"RDO timeout: %d".as_ptr(), err);
                return err;
            }

            /* clear RDO flag */
            regmap_set_bits((*jz_codec).regmap, JZ4760_CODEC_REG_IFR, REG_IFR_RDO);
        }
        _ => {}
    }

    0
}

static jz4760_codec_hp_texts: [*const c_char; 4] = [
    c"PCM".as_ptr(),
    c"Line In".as_ptr(),
    c"Mic 1".as_ptr(),
    c"Mic 2".as_ptr(),
];

static jz4760_codec_hp_values: [c_uint; 4] = [3, 2, 0, 1];

/* static SOC_VALUE_ENUM_SINGLE_DECL(jz4760_codec_hp_enum, ...); */
/* static const struct snd_kcontrol_new jz4760_codec_hp_source = SOC_DAPM_ENUM("Route", jz4760_codec_hp_enum); */

static jz4760_codec_cap_texts: [*const c_char; 3] = [
    c"Line In".as_ptr(),
    c"Mic 1".as_ptr(),
    c"Mic 2".as_ptr(),
];

static jz4760_codec_cap_values: [c_uint; 3] = [2, 0, 1];

/* static SOC_VALUE_ENUM_SINGLE_DECL(jz4760_codec_cap_enum, ...); */
/* static const struct snd_kcontrol_new jz4760_codec_cap_source = SOC_DAPM_ENUM("Route", jz4760_codec_cap_enum); */
/* static const struct snd_kcontrol_new jz4760_codec_mic_controls[] = SOC_DAPM_SINGLE(...); */
/* static const struct snd_kcontrol_new jz4760_codec_line_out_switch = SOC_DAPM_SINGLE(...); */
/* static const struct snd_kcontrol_new jz4760_codec_btl_out_switch = SOC_DAPM_SINGLE(...); */

/* DAPM widgets generated by SND_SOC_DAPM_* C macros. */
static jz4760_codec_dapm_widgets: [snd_soc_dapm_widget_data; 0] = [];

/* Unconditional routes. */
static jz4760_codec_dapm_routes: [snd_soc_dapm_route; 35] = [
    snd_soc_dapm_route { sink: c"Mic 1".as_ptr(), control: ptr::null(), source: c"MIC1P".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic Diff".as_ptr(), control: ptr::null(), source: c"MIC1N".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic 1".as_ptr(), control: ptr::null(), source: c"Mic Diff".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic 2".as_ptr(), control: ptr::null(), source: c"MIC2P".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic Diff".as_ptr(), control: ptr::null(), source: c"MIC2N".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic 2".as_ptr(), control: ptr::null(), source: c"Mic Diff".as_ptr() },
    snd_soc_dapm_route { sink: c"Line In".as_ptr(), control: ptr::null(), source: c"LLINEIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Line In".as_ptr(), control: ptr::null(), source: c"RLINEIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic".as_ptr(), control: c"Stereo Capture Switch".as_ptr(), source: c"Mic 1".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic".as_ptr(), control: c"Stereo Capture Switch".as_ptr(), source: c"Mic 2".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphones Source".as_ptr(), control: c"Mic 1".as_ptr(), source: c"Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphones Source".as_ptr(), control: c"Mic 2".as_ptr(), source: c"Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Source".as_ptr(), control: c"Mic 1".as_ptr(), source: c"Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Source".as_ptr(), control: c"Mic 2".as_ptr(), source: c"Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Source".as_ptr(), control: c"Line In".as_ptr(), source: c"Line In".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Source".as_ptr(), control: c"Mic 1".as_ptr(), source: c"Mic 1".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture Source".as_ptr(), control: c"Mic 2".as_ptr(), source: c"Mic 2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: ptr::null(), source: c"Capture Source".as_ptr() },
    snd_soc_dapm_route { sink: c"Line In Bypass".as_ptr(), control: ptr::null(), source: c"Line In".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphones Source".as_ptr(), control: c"Mic 1".as_ptr(), source: c"Mic 1".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphones Source".as_ptr(), control: c"Mic 2".as_ptr(), source: c"Mic 2".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphones Source".as_ptr(), control: c"Line In".as_ptr(), source: c"Line In Bypass".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphones Source".as_ptr(), control: c"PCM".as_ptr(), source: c"Headphones Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"HP Out".as_ptr(), control: ptr::null(), source: c"Headphones Source".as_ptr() },
    snd_soc_dapm_route { sink: c"LHPOUT".as_ptr(), control: ptr::null(), source: c"HP Out".as_ptr() },
    snd_soc_dapm_route { sink: c"RHPOUT".as_ptr(), control: ptr::null(), source: c"HP Out".as_ptr() },
    snd_soc_dapm_route { sink: c"Line Out".as_ptr(), control: c"Switch".as_ptr(), source: c"HP Out".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT".as_ptr(), control: ptr::null(), source: c"Line Out".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT".as_ptr(), control: ptr::null(), source: c"Line Out".as_ptr() },
    snd_soc_dapm_route { sink: c"BTL Out".as_ptr(), control: c"Switch".as_ptr(), source: c"Line Out".as_ptr() },
    snd_soc_dapm_route { sink: c"BTLP".as_ptr(), control: ptr::null(), source: c"BTL Out".as_ptr() },
    snd_soc_dapm_route { sink: c"BTLN".as_ptr(), control: ptr::null(), source: c"BTL Out".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM Playback".as_ptr(), control: c"Volume".as_ptr(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphones Playback".as_ptr(), control: c"Volume".as_ptr(), source: c"PCM Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"SYSCLK".as_ptr(), control: ptr::null(), source: c"DAC".as_ptr() },
];

unsafe fn jz4760_codec_codec_init_regs(codec: *mut snd_soc_component) {
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let regmap = (*jz_codec).regmap;

    /* Collect updates for later sending. */
    regcache_cache_only(regmap, true);

    /* default Amp output to PCM */
    regmap_set_bits(regmap, JZ4760_CODEC_REG_CR1, REG_CR1_OUTSEL_MASK);

    /* Disable stereo mic */
    regmap_clear_bits(regmap, JZ4760_CODEC_REG_CR3, BIT(REG_CR3_MICSTEREO_OFFSET));

    /* Set mic 1 as default source for ADC */
    regmap_clear_bits(regmap, JZ4760_CODEC_REG_CR3, REG_CR3_ADC_INSEL_MASK);

    /* ADC/DAC: serial + i2s */
    regmap_set_bits(
        regmap,
        JZ4760_CODEC_REG_AICR,
        REG_AICR_ADC_SERIAL | REG_AICR_ADC_I2S | REG_AICR_DAC_SERIAL | REG_AICR_DAC_I2S,
    );

    /* The generated IRQ is a high level */
    regmap_clear_bits(regmap, JZ4760_CODEC_REG_ICR, REG_ICR_INT_FORM_MASK);
    regmap_update_bits(
        regmap,
        JZ4760_CODEC_REG_ICR,
        REG_ICR_ALL_MASK,
        REG_ICR_JACK_MASK | REG_ICR_RUP_MASK | REG_ICR_RDO_MASK | REG_ICR_GUP_MASK | REG_ICR_GDO_MASK,
    );

    /* 12M oscillator */
    regmap_clear_bits(regmap, JZ4760_CODEC_REG_CCR1, REG_CCR1_CRYSTAL_MASK);

    /* 0: 16ohm/220uF, 1: 10kohm/1uF */
    regmap_clear_bits(regmap, JZ4760_CODEC_REG_CR1, REG_CR1_HP_LOAD);

    /* default to NOMAD */
    regmap_set_bits((*jz_codec).regmap, JZ4760_CODEC_REG_CR2, REG_CR2_DAC_NOMAD);

    /* disable automatic gain */
    regmap_clear_bits(regmap, JZ4760_CODEC_REG_AGC1, REG_AGC1_EN);

    /* Independent L/R DAC gain control */
    regmap_clear_bits(regmap, JZ4760_CODEC_REG_GCR5, REG_GCR_RL);

    /* Send collected updates. */
    regcache_cache_only(regmap, false);
    regcache_sync(regmap);
}

unsafe extern "C" fn jz4760_codec_codec_probe(codec: *mut snd_soc_component) -> c_int {
    jz4760_codec_codec_init_regs(codec);

    0
}

static jz4760_codec_soc_codec_dev: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(jz4760_codec_codec_probe),
    set_bias_level: Some(jz4760_codec_set_bias_level),
    controls: jz4760_codec_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&jz4760_codec_snd_controls) as c_uint,
    dapm_widgets: jz4760_codec_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&jz4760_codec_dapm_widgets) as c_uint,
    dapm_routes: jz4760_codec_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&jz4760_codec_dapm_routes) as c_uint,
    suspend_bias_off: 1,
    use_pmdown_time: 1,
};

static jz4760_codec_sample_rates: [c_uint; 11] = [
    96000, 48000, 44100, 32000, 24000, 22050, 16000, 12000, 11025, 9600, 8000,
];

unsafe extern "C" fn jz4760_codec_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec = snd_soc_component_get_drvdata((*dai).component) as *mut jz_codec;
    let mut rate: c_uint;
    let bit_width: c_uint;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            bit_width = 0;
        }
        SNDRV_PCM_FORMAT_S18_3LE => {
            bit_width = 1;
        }
        SNDRV_PCM_FORMAT_S20_3LE => {
            bit_width = 2;
        }
        SNDRV_PCM_FORMAT_S24_3LE => {
            bit_width = 3;
        }
        _ => {
            return -EINVAL;
        }
    }

    rate = 0;
    while (rate as usize) < ARRAY_SIZE(&jz4760_codec_sample_rates) {
        if jz4760_codec_sample_rates[rate as usize] == params_rate(params) {
            break;
        }
        rate += 1;
    }

    if (rate as usize) == ARRAY_SIZE(&jz4760_codec_sample_rates) {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits(
            (*codec).regmap,
            JZ4760_CODEC_REG_AICR,
            REG_AICR_DAC_ADWL_MASK,
            FIELD_PREP(REG_AICR_DAC_ADWL_MASK, bit_width),
        );
        regmap_update_bits(
            (*codec).regmap,
            JZ4760_CODEC_REG_CCR2,
            REG_CCR2_DAC_FREQ_MASK,
            FIELD_PREP(REG_CCR2_DAC_FREQ_MASK, rate),
        );
    } else {
        regmap_update_bits(
            (*codec).regmap,
            JZ4760_CODEC_REG_AICR,
            REG_AICR_ADC_ADWL_MASK,
            FIELD_PREP(REG_AICR_ADC_ADWL_MASK, bit_width),
        );
        regmap_update_bits(
            (*codec).regmap,
            JZ4760_CODEC_REG_CCR2,
            REG_CCR2_ADC_FREQ_MASK,
            FIELD_PREP(REG_CCR2_ADC_FREQ_MASK, rate),
        );
    }

    0
}

static jz4760_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(jz4760_codec_startup),
    shutdown: Some(jz4760_codec_shutdown),
    hw_params: Some(jz4760_codec_hw_params),
    trigger: Some(jz4760_codec_pcm_trigger),
    mute_stream: Some(jz4760_codec_mute_stream),
    no_capture_mute: 1,
};

const JZ_CODEC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE;

static mut jz4760_codec_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"jz4760-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: JZ_CODEC_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: JZ_CODEC_FORMATS,
    },
    ops: &jz4760_codec_dai_ops,
};

unsafe extern "C" fn jz4760_codec_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    reg == JZ4760_CODEC_REG_SR || reg == JZ4760_CODEC_REG_IFR
}

unsafe extern "C" fn jz4760_codec_writeable(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        JZ4760_CODEC_REG_SR => false,
        _ => true,
    }
}

unsafe extern "C" fn jz4760_codec_io_wait_cond(reg: u32) -> bool {
    (reg & ICDC_RGADW_RGWR) == 0
}

unsafe fn jz4760_codec_io_wait(codec: *mut jz_codec) -> c_int {
    let mut reg: u32 = 0;

    readl_poll_timeout(
        ((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *const c_void,
        &mut reg,
        jz4760_codec_io_wait_cond,
        1000,
        1 * USEC_PER_SEC,
    )
}

unsafe extern "C" fn jz4760_codec_reg_read(
    context: *mut c_void,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let codec = context as *mut jz_codec;
    let mut i: c_uint;
    let mut tmp: u32;
    let ret: c_int;

    ret = jz4760_codec_io_wait(codec);
    if ret != 0 {
        return ret;
    }

    tmp = readl(((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *const c_void);
    tmp &= !ICDC_RGADW_RGADDR_MASK;
    tmp |= FIELD_PREP(ICDC_RGADW_RGADDR_MASK, reg);
    writel(tmp, ((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *mut c_void);

    /* wait 6+ cycles */
    i = 0;
    while i < 6 {
        *val = readl(((*codec).base as *mut u8).add(ICDC_RGDATA_OFFSET) as *const c_void)
            & ICDC_RGDATA_RGDOUT_MASK;
        i += 1;
    }

    0
}

unsafe extern "C" fn jz4760_codec_reg_write(
    context: *mut c_void,
    reg: c_uint,
    val: c_uint,
) -> c_int {
    let codec = context as *mut jz_codec;
    let mut ret: c_int;

    ret = jz4760_codec_io_wait(codec);
    if ret != 0 {
        return ret;
    }

    writel(
        ICDC_RGADW_RGWR | FIELD_PREP(ICDC_RGADW_RGADDR_MASK, reg) | val,
        ((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *mut c_void,
    );

    ret = jz4760_codec_io_wait(codec);
    if ret != 0 {
        return ret;
    }

    0
}

static jz4760_codec_reg_defaults: [u8; 28] = [
    0x00, 0xFC, 0x1B, 0x20, 0x00, 0x80, 0x00, 0x00,
    0xFF, 0x1F, 0x3F, 0x00, 0x06, 0x06, 0x06, 0x06,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x07, 0x44,
    0x1F, 0x00, 0x00, 0x00,
];

static jz4760_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 8,

    max_register: JZ4760_CODEC_REG_MIX2,
    volatile_reg: Some(jz4760_codec_volatile),
    writeable_reg: Some(jz4760_codec_writeable),

    reg_read: Some(jz4760_codec_reg_read),
    reg_write: Some(jz4760_codec_reg_write),

    reg_defaults_raw: jz4760_codec_reg_defaults.as_ptr(),
    num_reg_defaults_raw: ARRAY_SIZE(&jz4760_codec_reg_defaults) as c_uint,
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn jz4760_codec_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let codec: *mut jz_codec;
    let clk: *mut clk;
    let ret: c_int;

    codec = devm_kzalloc(dev, core::mem::size_of::<jz_codec>(), GFP_KERNEL) as *mut jz_codec;
    if codec.is_null() {
        return -ENOMEM;
    }

    (*codec).dev = dev;

    (*codec).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*codec).base) {
        return PTR_ERR((*codec).base);
    }

    (*codec).regmap = devm_regmap_init(
        dev,
        ptr::null(),
        codec as *mut c_void,
        &jz4760_codec_regmap_config,
    );
    if IS_ERR((*codec).regmap as *const c_void) {
        return PTR_ERR((*codec).regmap as *const c_void);
    }

    clk = devm_clk_get_enabled(dev, c"aic".as_ptr());
    if IS_ERR(clk as *const c_void) {
        return PTR_ERR(clk as *const c_void);
    }

    platform_set_drvdata(pdev, codec as *mut c_void);

    ret = devm_snd_soc_register_component(
        dev,
        &jz4760_codec_soc_codec_dev,
        &mut jz4760_codec_dai,
        1,
    );
    if ret != 0 {
        dev_err(dev, c"Failed to register codec: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

static jz4760_codec_of_matches: [of_device_id; 2] = [
    of_device_id { compatible: c"ingenic,jz4760-codec".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, jz4760_codec_of_matches); */

static mut jz4760_codec_driver: platform_driver = platform_driver {
    probe: Some(jz4760_codec_probe),
    driver: platform_driver_driver {
        name: c"jz4760-codec".as_ptr(),
        of_match_table: jz4760_codec_of_matches.as_ptr(),
    },
};
/* module_platform_driver(jz4760_codec_driver); */

/* MODULE_DESCRIPTION("JZ4760 SoC internal codec driver"); */
/* MODULE_AUTHOR("Christophe Branchereau <cbranchereau@gmail.com>"); */
/* MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
