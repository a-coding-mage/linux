// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017, Maxim Integrated
//
// Rust translation of soc/codecs/max98373-i2c.c. C include dependencies are
// represented by extern declarations and opaque C-compatible types.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u32 = ::std::os::raw::c_uint;
type c_int = ::std::os::raw::c_int;
type c_uint = ::std::os::raw::c_uint;
type c_ulong = ::std::os::raw::c_ulong;
type bool_t = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 2;

const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0100;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;

const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 10;

const MAX98373_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const MAX98373_FORMATS: c_ulong =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct max98373_cache {
    pub reg: c_uint,
    pub val: c_uint,
}

#[repr(C)]
pub struct max98373_priv {
    pub regmap: *mut regmap,
    pub ch_size: c_int,
    pub tdm_mode: bool,
    pub interleave_mode: bool,
    pub cache_num: c_int,
    pub cache: *mut max98373_cache,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const ::std::os::raw::c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const ::std::os::raw::c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [::std::os::raw::c_char; 20],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const ::std::os::raw::c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [::std::os::raw::c_char; 16],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::std::os::raw::c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

extern "C" {
    static soc_codec_dev_max98373: u8;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut max98373_priv;
    fn dev_get_drvdata(dev: *mut device) -> *mut max98373_priv;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut max98373_priv);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut max98373_priv;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, flags: c_uint) -> *mut max98373_cache;
    fn device_property_read_bool(dev: *mut device, propname: *const ::std::os::raw::c_char) -> bool;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *mut regmap) -> bool;
    fn PTR_ERR(ptr: *mut regmap) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn max98373_reset(max98373: *mut max98373_priv, dev: *mut device);
    fn max98373_slot_config(dev: *mut device, max98373: *mut max98373_priv);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const u8,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const ::std::os::raw::c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const ::std::os::raw::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const ::std::os::raw::c_char, ...);
}

/* Register and bitfield constants are supplied by max98373.h in the original C
 * source. They are intentionally referenced here as external translation
 * dependencies rather than redefined from another file.
 */

static max98373_i2c_cache_reg: [u32; 3] = [
    MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK,
    MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK,
    MAX98373_R20B6_BDE_CUR_STATE_READBACK,
];

static max98373_reg: [reg_default; 91] = [
    reg_default { reg: MAX98373_R2000_SW_RESET, def: 0x00 },
    reg_default { reg: MAX98373_R2001_INT_RAW1, def: 0x00 },
    reg_default { reg: MAX98373_R2002_INT_RAW2, def: 0x00 },
    reg_default { reg: MAX98373_R2003_INT_RAW3, def: 0x00 },
    reg_default { reg: MAX98373_R2004_INT_STATE1, def: 0x00 },
    reg_default { reg: MAX98373_R2005_INT_STATE2, def: 0x00 },
    reg_default { reg: MAX98373_R2006_INT_STATE3, def: 0x00 },
    reg_default { reg: MAX98373_R2007_INT_FLAG1, def: 0x00 },
    reg_default { reg: MAX98373_R2008_INT_FLAG2, def: 0x00 },
    reg_default { reg: MAX98373_R2009_INT_FLAG3, def: 0x00 },
    reg_default { reg: MAX98373_R200A_INT_EN1, def: 0x00 },
    reg_default { reg: MAX98373_R200B_INT_EN2, def: 0x00 },
    reg_default { reg: MAX98373_R200C_INT_EN3, def: 0x00 },
    reg_default { reg: MAX98373_R200D_INT_FLAG_CLR1, def: 0x00 },
    reg_default { reg: MAX98373_R200E_INT_FLAG_CLR2, def: 0x00 },
    reg_default { reg: MAX98373_R200F_INT_FLAG_CLR3, def: 0x00 },
    reg_default { reg: MAX98373_R2010_IRQ_CTRL, def: 0x00 },
    reg_default { reg: MAX98373_R2014_THERM_WARN_THRESH, def: 0x10 },
    reg_default { reg: MAX98373_R2015_THERM_SHDN_THRESH, def: 0x27 },
    reg_default { reg: MAX98373_R2016_THERM_HYSTERESIS, def: 0x01 },
    reg_default { reg: MAX98373_R2017_THERM_FOLDBACK_SET, def: 0xC0 },
    reg_default { reg: MAX98373_R2018_THERM_FOLDBACK_EN, def: 0x00 },
    reg_default { reg: MAX98373_R201E_PIN_DRIVE_STRENGTH, def: 0x55 },
    reg_default { reg: MAX98373_R2020_PCM_TX_HIZ_EN_1, def: 0xFE },
    reg_default { reg: MAX98373_R2021_PCM_TX_HIZ_EN_2, def: 0xFF },
    reg_default { reg: MAX98373_R2022_PCM_TX_SRC_1, def: 0x00 },
    reg_default { reg: MAX98373_R2023_PCM_TX_SRC_2, def: 0x00 },
    reg_default { reg: MAX98373_R2024_PCM_DATA_FMT_CFG, def: 0xC0 },
    reg_default { reg: MAX98373_R2025_AUDIO_IF_MODE, def: 0x00 },
    reg_default { reg: MAX98373_R2026_PCM_CLOCK_RATIO, def: 0x04 },
    reg_default { reg: MAX98373_R2027_PCM_SR_SETUP_1, def: 0x08 },
    reg_default { reg: MAX98373_R2028_PCM_SR_SETUP_2, def: 0x88 },
    reg_default { reg: MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1, def: 0x00 },
    reg_default { reg: MAX98373_R202A_PCM_TO_SPK_MONO_MIX_2, def: 0x00 },
    reg_default { reg: MAX98373_R202B_PCM_RX_EN, def: 0x00 },
    reg_default { reg: MAX98373_R202C_PCM_TX_EN, def: 0x00 },
    reg_default { reg: MAX98373_R202E_ICC_RX_CH_EN_1, def: 0x00 },
    reg_default { reg: MAX98373_R202F_ICC_RX_CH_EN_2, def: 0x00 },
    reg_default { reg: MAX98373_R2030_ICC_TX_HIZ_EN_1, def: 0xFF },
    reg_default { reg: MAX98373_R2031_ICC_TX_HIZ_EN_2, def: 0xFF },
    reg_default { reg: MAX98373_R2032_ICC_LINK_EN_CFG, def: 0x30 },
    reg_default { reg: MAX98373_R2034_ICC_TX_CNTL, def: 0x00 },
    reg_default { reg: MAX98373_R2035_ICC_TX_EN, def: 0x00 },
    reg_default { reg: MAX98373_R2036_SOUNDWIRE_CTRL, def: 0x05 },
    reg_default { reg: MAX98373_R203D_AMP_DIG_VOL_CTRL, def: 0x00 },
    reg_default { reg: MAX98373_R203E_AMP_PATH_GAIN, def: 0x08 },
    reg_default { reg: MAX98373_R203F_AMP_DSP_CFG, def: 0x02 },
    reg_default { reg: MAX98373_R2040_TONE_GEN_CFG, def: 0x00 },
    reg_default { reg: MAX98373_R2041_AMP_CFG, def: 0x03 },
    reg_default { reg: MAX98373_R2042_AMP_EDGE_RATE_CFG, def: 0x00 },
    reg_default { reg: MAX98373_R2043_AMP_EN, def: 0x00 },
    reg_default { reg: MAX98373_R2046_IV_SENSE_ADC_DSP_CFG, def: 0x04 },
    reg_default { reg: MAX98373_R2047_IV_SENSE_ADC_EN, def: 0x00 },
    reg_default { reg: MAX98373_R2051_MEAS_ADC_SAMPLING_RATE, def: 0x00 },
    reg_default { reg: MAX98373_R2052_MEAS_ADC_PVDD_FLT_CFG, def: 0x00 },
    reg_default { reg: MAX98373_R2053_MEAS_ADC_THERM_FLT_CFG, def: 0x00 },
    reg_default { reg: MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK, def: 0x00 },
    reg_default { reg: MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK, def: 0x00 },
    reg_default { reg: MAX98373_R2056_MEAS_ADC_PVDD_CH_EN, def: 0x00 },
    reg_default { reg: MAX98373_R2090_BDE_LVL_HOLD, def: 0x00 },
    reg_default { reg: MAX98373_R2091_BDE_GAIN_ATK_REL_RATE, def: 0x00 },
    reg_default { reg: MAX98373_R2092_BDE_CLIPPER_MODE, def: 0x00 },
    reg_default { reg: MAX98373_R2097_BDE_L1_THRESH, def: 0x00 },
    reg_default { reg: MAX98373_R2098_BDE_L2_THRESH, def: 0x00 },
    reg_default { reg: MAX98373_R2099_BDE_L3_THRESH, def: 0x00 },
    reg_default { reg: MAX98373_R209A_BDE_L4_THRESH, def: 0x00 },
    reg_default { reg: MAX98373_R209B_BDE_THRESH_HYST, def: 0x00 },
    reg_default { reg: MAX98373_R20A8_BDE_L1_CFG_1, def: 0x00 },
    reg_default { reg: MAX98373_R20A9_BDE_L1_CFG_2, def: 0x00 },
    reg_default { reg: MAX98373_R20AA_BDE_L1_CFG_3, def: 0x00 },
    reg_default { reg: MAX98373_R20AB_BDE_L2_CFG_1, def: 0x00 },
    reg_default { reg: MAX98373_R20AC_BDE_L2_CFG_2, def: 0x00 },
    reg_default { reg: MAX98373_R20AD_BDE_L2_CFG_3, def: 0x00 },
    reg_default { reg: MAX98373_R20AE_BDE_L3_CFG_1, def: 0x00 },
    reg_default { reg: MAX98373_R20AF_BDE_L3_CFG_2, def: 0x00 },
    reg_default { reg: MAX98373_R20B0_BDE_L3_CFG_3, def: 0x00 },
    reg_default { reg: MAX98373_R20B1_BDE_L4_CFG_1, def: 0x00 },
    reg_default { reg: MAX98373_R20B2_BDE_L4_CFG_2, def: 0x00 },
    reg_default { reg: MAX98373_R20B3_BDE_L4_CFG_3, def: 0x00 },
    reg_default { reg: MAX98373_R20B4_BDE_INFINITE_HOLD_RELEASE, def: 0x00 },
    reg_default { reg: MAX98373_R20B5_BDE_EN, def: 0x00 },
    reg_default { reg: MAX98373_R20B6_BDE_CUR_STATE_READBACK, def: 0x00 },
    reg_default { reg: MAX98373_R20D1_DHT_CFG, def: 0x01 },
    reg_default { reg: MAX98373_R20D2_DHT_ATTACK_CFG, def: 0x02 },
    reg_default { reg: MAX98373_R20D3_DHT_RELEASE_CFG, def: 0x03 },
    reg_default { reg: MAX98373_R20D4_DHT_EN, def: 0x00 },
    reg_default { reg: MAX98373_R20E0_LIMITER_THRESH_CFG, def: 0x00 },
    reg_default { reg: MAX98373_R20E1_LIMITER_ATK_REL_RATES, def: 0x00 },
    reg_default { reg: MAX98373_R20E2_LIMITER_EN, def: 0x00 },
    reg_default { reg: MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG, def: 0x00 },
    reg_default { reg: MAX98373_R20FF_GLOBAL_SHDN, def: 0x00 },
    reg_default { reg: MAX98373_R21FF_REV_ID, def: 0x42 },
];

unsafe extern "C" fn max98373_dai_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let max98373 = snd_soc_component_get_drvdata(component);
    let mut format: c_uint = 0;
    let mut invert: c_uint = 0;

    dev_dbg((*component).dev, c"%s: fmt 0x%08X\n".as_ptr(), c"max98373_dai_set_fmt".as_ptr(), fmt);

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => invert = MAX98373_PCM_MODE_CFG_PCM_BCLKEDGE,
        _ => {
            dev_err((*component).dev, c"DAI invert mode unsupported\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2026_PCM_CLOCK_RATIO,
        MAX98373_PCM_MODE_CFG_PCM_BCLKEDGE,
        invert,
    );

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => format = MAX98373_PCM_FORMAT_I2S,
        SND_SOC_DAIFMT_LEFT_J => format = MAX98373_PCM_FORMAT_LJ,
        SND_SOC_DAIFMT_DSP_A => format = MAX98373_PCM_FORMAT_TDM_MODE1,
        SND_SOC_DAIFMT_DSP_B => format = MAX98373_PCM_FORMAT_TDM_MODE0,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2024_PCM_DATA_FMT_CFG,
        MAX98373_PCM_MODE_CFG_FORMAT_MASK,
        format << MAX98373_PCM_MODE_CFG_FORMAT_SHIFT,
    );

    0
}

/* BCLKs per LRCLK */
static bclk_sel_table: [c_int; 10] = [32, 48, 64, 96, 128, 192, 256, 384, 512, 320];

fn max98373_get_bclk_sel(bclk: c_int) -> c_int {
    /* match BCLKs per LRCLK */
    for i in 0..bclk_sel_table.len() {
        if bclk_sel_table[i] == bclk {
            return i as c_int + 2;
        }
    }
    0
}

unsafe extern "C" fn max98373_set_clock(
    component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let max98373 = snd_soc_component_get_drvdata(component);
    /* BCLK/LRCLK ratio calculation */
    let blr_clk_ratio = params_channels(params) * (*max98373).ch_size;
    let value: c_int;

    if !(*max98373).tdm_mode {
        /* BCLK configuration */
        value = max98373_get_bclk_sel(blr_clk_ratio);
        if value == 0 {
            dev_err((*component).dev, c"format unsupported %d\n".as_ptr(), params_format(params));
            return -EINVAL;
        }

        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2026_PCM_CLOCK_RATIO,
            MAX98373_PCM_CLK_SETUP_BSEL_MASK,
            value as c_uint,
        );
    }
    0
}

unsafe extern "C" fn max98373_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let max98373 = snd_soc_component_get_drvdata(component);
    let mut sampling_rate: c_uint = 0;
    let chan_sz: c_uint;

    /* pcm mode configuration */
    match snd_pcm_format_width(params_format(params)) {
        16 => chan_sz = MAX98373_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98373_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98373_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err((*component).dev, c"format unsupported %d\n".as_ptr(), params_format(params));
            return -EINVAL;
        }
    }

    (*max98373).ch_size = snd_pcm_format_width(params_format(params));

    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2024_PCM_DATA_FMT_CFG,
        MAX98373_PCM_MODE_CFG_CHANSZ_MASK,
        chan_sz,
    );

    dev_dbg((*component).dev, c"format supported %d".as_ptr(), params_format(params));

    /* sampling rate configuration */
    match params_rate(params) {
        8000 => sampling_rate = MAX98373_PCM_SR_SET1_SR_8000,
        11025 => sampling_rate = MAX98373_PCM_SR_SET1_SR_11025,
        12000 => sampling_rate = MAX98373_PCM_SR_SET1_SR_12000,
        16000 => sampling_rate = MAX98373_PCM_SR_SET1_SR_16000,
        22050 => sampling_rate = MAX98373_PCM_SR_SET1_SR_22050,
        24000 => sampling_rate = MAX98373_PCM_SR_SET1_SR_24000,
        32000 => sampling_rate = MAX98373_PCM_SR_SET1_SR_32000,
        44100 => sampling_rate = MAX98373_PCM_SR_SET1_SR_44100,
        48000 => sampling_rate = MAX98373_PCM_SR_SET1_SR_48000,
        88200 => sampling_rate = MAX98373_PCM_SR_SET1_SR_88200,
        96000 => sampling_rate = MAX98373_PCM_SR_SET1_SR_96000,
        _ => {
            dev_err((*component).dev, c"rate %d not supported\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }

    /* set DAI_SR to correct LRCLK frequency */
    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2027_PCM_SR_SETUP_1,
        MAX98373_PCM_SR_SET1_SR_MASK,
        sampling_rate,
    );
    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2028_PCM_SR_SETUP_2,
        MAX98373_PCM_SR_SET2_SR_MASK,
        sampling_rate << MAX98373_PCM_SR_SET2_SR_SHIFT,
    );

    /* set sampling rate of IV */
    if (*max98373).interleave_mode && sampling_rate > MAX98373_PCM_SR_SET1_SR_16000 {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2028_PCM_SR_SETUP_2,
            MAX98373_PCM_SR_SET2_IVADC_SR_MASK,
            sampling_rate - 3,
        );
    } else {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2028_PCM_SR_SETUP_2,
            MAX98373_PCM_SR_SET2_IVADC_SR_MASK,
            sampling_rate,
        );
    }

    max98373_set_clock(component, params)
}

unsafe extern "C" fn max98373_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let max98373 = snd_soc_component_get_drvdata(component);
    let mut bsel: c_int = 0;
    let chan_sz: c_uint;
    let mut mask: c_uint;
    let mut slot_found: c_int;

    if tx_mask == 0 && rx_mask == 0 && slots == 0 && slot_width == 0 {
        (*max98373).tdm_mode = false;
    } else {
        (*max98373).tdm_mode = true;
    }

    /* BCLK configuration */
    bsel = max98373_get_bclk_sel(slots * slot_width);
    if bsel == 0 {
        dev_err((*component).dev, c"BCLK %d not supported\n".as_ptr(), slots * slot_width);
        return -EINVAL;
    }

    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2026_PCM_CLOCK_RATIO,
        MAX98373_PCM_CLK_SETUP_BSEL_MASK,
        bsel as c_uint,
    );

    /* Channel size configuration */
    match slot_width {
        16 => chan_sz = MAX98373_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98373_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98373_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err((*component).dev, c"format unsupported %d\n".as_ptr(), slot_width);
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2024_PCM_DATA_FMT_CFG,
        MAX98373_PCM_MODE_CFG_CHANSZ_MASK,
        chan_sz,
    );

    /* Rx slot configuration */
    slot_found = 0;
    mask = rx_mask;
    for x in 0..16 {
        if (mask & 0x1) != 0 {
            if slot_found == 0 {
                regmap_update_bits(
                    (*max98373).regmap,
                    MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1,
                    MAX98373_PCM_TO_SPK_CH0_SRC_MASK,
                    x,
                );
            } else {
                regmap_write((*max98373).regmap, MAX98373_R202A_PCM_TO_SPK_MONO_MIX_2, x);
            }
            slot_found += 1;
            if slot_found > 1 {
                break;
            }
        }
        mask >>= 1;
    }

    /* Tx slot Hi-Z configuration */
    regmap_write((*max98373).regmap, MAX98373_R2020_PCM_TX_HIZ_EN_1, (!tx_mask) & 0xFF);
    regmap_write(
        (*max98373).regmap,
        MAX98373_R2021_PCM_TX_HIZ_EN_2,
        ((!tx_mask) & 0xFF00) >> 8,
    );

    0
}

static max98373_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(max98373_dai_set_fmt),
    hw_params: Some(max98373_dai_hw_params),
    set_tdm_slot: Some(max98373_dai_tdm_slot),
};

unsafe extern "C" fn max98373_readable_register(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        MAX98373_R2000_SW_RESET
        | MAX98373_R2001_INT_RAW1..=MAX98373_R200C_INT_EN3
        | MAX98373_R2010_IRQ_CTRL
        | MAX98373_R2014_THERM_WARN_THRESH..=MAX98373_R2018_THERM_FOLDBACK_EN
        | MAX98373_R201E_PIN_DRIVE_STRENGTH..=MAX98373_R2036_SOUNDWIRE_CTRL
        | MAX98373_R203D_AMP_DIG_VOL_CTRL..=MAX98373_R2043_AMP_EN
        | MAX98373_R2046_IV_SENSE_ADC_DSP_CFG..=MAX98373_R2047_IV_SENSE_ADC_EN
        | MAX98373_R2051_MEAS_ADC_SAMPLING_RATE..=MAX98373_R2056_MEAS_ADC_PVDD_CH_EN
        | MAX98373_R2090_BDE_LVL_HOLD..=MAX98373_R2092_BDE_CLIPPER_MODE
        | MAX98373_R2097_BDE_L1_THRESH..=MAX98373_R209B_BDE_THRESH_HYST
        | MAX98373_R20A8_BDE_L1_CFG_1..=MAX98373_R20B3_BDE_L4_CFG_3
        | MAX98373_R20B5_BDE_EN..=MAX98373_R20B6_BDE_CUR_STATE_READBACK
        | MAX98373_R20D1_DHT_CFG..=MAX98373_R20D4_DHT_EN
        | MAX98373_R20E0_LIMITER_THRESH_CFG..=MAX98373_R20E2_LIMITER_EN
        | MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG..=MAX98373_R20FF_GLOBAL_SHDN
        | MAX98373_R21FF_REV_ID => true,
        _ => false,
    }
}

unsafe extern "C" fn max98373_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        MAX98373_R2000_SW_RESET..=MAX98373_R2009_INT_FLAG3
        | MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK
        | MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK
        | MAX98373_R20B6_BDE_CUR_STATE_READBACK
        | MAX98373_R20FF_GLOBAL_SHDN
        | MAX98373_R21FF_REV_ID => true,
        _ => false,
    }
}

static mut max98373_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"max98373-aif1".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MAX98373_RATES,
        formats: MAX98373_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"HiFi Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MAX98373_RATES,
        formats: MAX98373_FORMATS,
    },
    ops: &max98373_dai_ops,
}];

unsafe extern "C" fn max98373_suspend(dev: *mut device) -> c_int {
    let max98373 = dev_get_drvdata(dev);

    /* cache feedback register values before suspend */
    for i in 0..(*max98373).cache_num {
        regmap_read(
            (*max98373).regmap,
            (*(*max98373).cache.offset(i as isize)).reg,
            &mut (*(*max98373).cache.offset(i as isize)).val as *mut c_uint as *mut c_int,
        );
    }

    regcache_cache_only((*max98373).regmap, true);
    regcache_mark_dirty((*max98373).regmap);
    0
}

unsafe extern "C" fn max98373_resume(dev: *mut device) -> c_int {
    let max98373 = dev_get_drvdata(dev);
    let ret: c_int;

    regcache_cache_only((*max98373).regmap, false);
    max98373_reset(max98373, dev);
    ret = regcache_sync((*max98373).regmap);
    if ret != 0 {
        regcache_cache_only((*max98373).regmap, true);
        regcache_mark_dirty((*max98373).regmap);
        return ret;
    }

    0
}

static max98373_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(max98373_suspend),
    resume: Some(max98373_resume),
};

static max98373_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 8,
    max_register: MAX98373_R21FF_REV_ID,
    reg_defaults: max98373_reg.as_ptr(),
    num_reg_defaults: max98373_reg.len() as c_uint,
    readable_reg: Some(max98373_readable_register),
    volatile_reg: Some(max98373_volatile_reg),
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn max98373_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int = 0;
    let mut reg: c_int = 0;
    let mut max98373: *mut max98373_priv = ::std::ptr::null_mut();

    max98373 = devm_kzalloc(
        &mut (*i2c).dev,
        ::std::mem::size_of::<max98373_priv>(),
        GFP_KERNEL,
    );

    if max98373.is_null() {
        ret = -ENOMEM;
        return ret;
    }
    i2c_set_clientdata(i2c, max98373);

    /* update interleave mode info */
    if device_property_read_bool(&mut (*i2c).dev, c"maxim,interleave_mode".as_ptr()) {
        (*max98373).interleave_mode = true;
    } else {
        (*max98373).interleave_mode = false;
    }

    /* regmap initialization */
    (*max98373).regmap = devm_regmap_init_i2c(i2c, &max98373_regmap);
    if IS_ERR((*max98373).regmap) {
        ret = PTR_ERR((*max98373).regmap);
        dev_err(&mut (*i2c).dev, c"Failed to allocate regmap: %d\n".as_ptr(), ret);
        return ret;
    }

    (*max98373).cache_num = max98373_i2c_cache_reg.len() as c_int;
    (*max98373).cache = devm_kcalloc(
        &mut (*i2c).dev,
        (*max98373).cache_num,
        ::std::mem::size_of::<max98373_cache>(),
        GFP_KERNEL,
    );
    if (*max98373).cache.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    for i in 0..(*max98373).cache_num {
        (*(*max98373).cache.offset(i as isize)).reg = max98373_i2c_cache_reg[i as usize];
    }

    /* voltage/current slot & gpio configuration */
    max98373_slot_config(&mut (*i2c).dev, max98373);

    /* Check Revision ID */
    ret = regmap_read((*max98373).regmap, MAX98373_R21FF_REV_ID, &mut reg);
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            c"Failed to read: 0x%02X\n".as_ptr(),
            MAX98373_R21FF_REV_ID,
        );
        return ret;
    }
    dev_info(&mut (*i2c).dev, c"MAX98373 revisionID: 0x%02X\n".as_ptr(), reg);

    /* codec registration */
    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_codec_dev_max98373,
        max98373_dai.as_mut_ptr(),
        max98373_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(&mut (*i2c).dev, c"Failed to register codec: %d\n".as_ptr(), ret);
    }

    ret
}

static max98373_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"max98373\0\0\0\0\0\0\0\0\0\0\0\0" as [::std::os::raw::c_char; 20], driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];

/* MODULE_DEVICE_TABLE(i2c, max98373_i2c_id); */

/* #if defined(CONFIG_OF) */
static max98373_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"maxim,max98373".as_ptr() },
    of_device_id { compatible: ::std::ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, max98373_of_match); */
/* #endif */

/* #ifdef CONFIG_ACPI */
static max98373_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"MX98373\0\0\0\0\0\0\0\0\0" as [::std::os::raw::c_char; 16], driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, max98373_acpi_match); */
/* #endif */

static mut max98373_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"max98373".as_ptr(),
        of_match_table: max98373_of_match.as_ptr(),
        acpi_match_table: max98373_acpi_match.as_ptr(),
        pm: &max98373_pm,
    },
    probe: Some(max98373_i2c_probe),
    id_table: max98373_i2c_id.as_ptr(),
};

/* module_i2c_driver(max98373_i2c_driver) */

/* MODULE_DESCRIPTION("ALSA SoC MAX98373 driver"); */
/* MODULE_AUTHOR("Ryan Lee <ryans.lee@maximintegrated.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
