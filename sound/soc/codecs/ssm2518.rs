// SPDX-License-Identifier: GPL-2.0-only
/*
 * SSM2518 amplifier audio driver
 *
 * Copyright 2013 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const fn BIT(n: c_uint) -> c_uint {
    1_u32 << n
}

const SSM2518_REG_POWER1: c_uint = 0x00;
const SSM2518_REG_CLOCK: c_uint = 0x01;
const SSM2518_REG_SAI_CTRL1: c_uint = 0x02;
const SSM2518_REG_SAI_CTRL2: c_uint = 0x03;
const SSM2518_REG_CHAN_MAP: c_uint = 0x04;
const SSM2518_REG_LEFT_VOL: c_uint = 0x05;
const SSM2518_REG_RIGHT_VOL: c_uint = 0x06;
const SSM2518_REG_MUTE_CTRL: c_uint = 0x07;
const SSM2518_REG_FAULT_CTRL: c_uint = 0x08;
const SSM2518_REG_POWER2: c_uint = 0x09;
const SSM2518_REG_DRC_1: c_uint = 0x0a;
const SSM2518_REG_DRC_2: c_uint = 0x0b;
const SSM2518_REG_DRC_3: c_uint = 0x0c;
const SSM2518_REG_DRC_4: c_uint = 0x0d;
const SSM2518_REG_DRC_5: c_uint = 0x0e;
const SSM2518_REG_DRC_6: c_uint = 0x0f;
const SSM2518_REG_DRC_7: c_uint = 0x10;
const SSM2518_REG_DRC_8: c_uint = 0x11;
const SSM2518_REG_DRC_9: c_uint = 0x12;

const SSM2518_POWER1_RESET: c_uint = BIT(7);
const SSM2518_POWER1_NO_BCLK: c_uint = BIT(5);
const SSM2518_POWER1_MCS_MASK: c_uint = 0xf << 1;
const SSM2518_POWER1_MCS_64FS: c_uint = 0x0 << 1;
const SSM2518_POWER1_MCS_128FS: c_uint = 0x1 << 1;
const SSM2518_POWER1_MCS_256FS: c_uint = 0x2 << 1;
const SSM2518_POWER1_MCS_384FS: c_uint = 0x3 << 1;
const SSM2518_POWER1_MCS_512FS: c_uint = 0x4 << 1;
const SSM2518_POWER1_MCS_768FS: c_uint = 0x5 << 1;
const SSM2518_POWER1_MCS_100FS: c_uint = 0x6 << 1;
const SSM2518_POWER1_MCS_200FS: c_uint = 0x7 << 1;
const SSM2518_POWER1_MCS_400FS: c_uint = 0x8 << 1;
const SSM2518_POWER1_SPWDN: c_uint = BIT(0);

const SSM2518_CLOCK_ASR: c_uint = BIT(0);

const SSM2518_SAI_CTRL1_FMT_MASK: c_uint = 0x3 << 5;
const SSM2518_SAI_CTRL1_FMT_I2S: c_uint = 0x0 << 5;
const SSM2518_SAI_CTRL1_FMT_LJ: c_uint = 0x1 << 5;
const SSM2518_SAI_CTRL1_FMT_RJ_24BIT: c_uint = 0x2 << 5;
const SSM2518_SAI_CTRL1_FMT_RJ_16BIT: c_uint = 0x3 << 5;

const SSM2518_SAI_CTRL1_SAI_MASK: c_uint = 0x7 << 2;
const SSM2518_SAI_CTRL1_SAI_I2S: c_uint = 0x0 << 2;
const SSM2518_SAI_CTRL1_SAI_TDM_2: c_uint = 0x1 << 2;
const SSM2518_SAI_CTRL1_SAI_TDM_4: c_uint = 0x2 << 2;
const SSM2518_SAI_CTRL1_SAI_TDM_8: c_uint = 0x3 << 2;
const SSM2518_SAI_CTRL1_SAI_TDM_16: c_uint = 0x4 << 2;
const SSM2518_SAI_CTRL1_SAI_MONO: c_uint = 0x5 << 2;

const SSM2518_SAI_CTRL1_FS_MASK: c_uint = 0x3;
const SSM2518_SAI_CTRL1_FS_8000_12000: c_uint = 0x0;
const SSM2518_SAI_CTRL1_FS_16000_24000: c_uint = 0x1;
const SSM2518_SAI_CTRL1_FS_32000_48000: c_uint = 0x2;
const SSM2518_SAI_CTRL1_FS_64000_96000: c_uint = 0x3;

const SSM2518_SAI_CTRL2_BCLK_INTERAL: c_uint = BIT(7);
const SSM2518_SAI_CTRL2_LRCLK_PULSE: c_uint = BIT(6);
const SSM2518_SAI_CTRL2_LRCLK_INVERT: c_uint = BIT(5);
const SSM2518_SAI_CTRL2_MSB: c_uint = BIT(4);
const SSM2518_SAI_CTRL2_SLOT_WIDTH_MASK: c_uint = 0x3 << 2;
const SSM2518_SAI_CTRL2_SLOT_WIDTH_32: c_uint = 0x0 << 2;
const SSM2518_SAI_CTRL2_SLOT_WIDTH_24: c_uint = 0x1 << 2;
const SSM2518_SAI_CTRL2_SLOT_WIDTH_16: c_uint = 0x2 << 2;
const SSM2518_SAI_CTRL2_BCLK_INVERT: c_uint = BIT(1);

const SSM2518_CHAN_MAP_RIGHT_SLOT_OFFSET: c_int = 4;
const SSM2518_CHAN_MAP_RIGHT_SLOT_MASK: c_uint = 0xf0;
const SSM2518_CHAN_MAP_LEFT_SLOT_OFFSET: c_int = 0;
const SSM2518_CHAN_MAP_LEFT_SLOT_MASK: c_uint = 0x0f;

const SSM2518_MUTE_CTRL_ANA_GAIN: c_uint = BIT(5);
const SSM2518_MUTE_CTRL_MUTE_MASTER: c_uint = BIT(0);

const SSM2518_POWER2_APWDN: c_uint = BIT(0);

const SSM2518_DAC_MUTE: c_uint = BIT(6);
const SSM2518_DAC_FS_MASK: c_uint = 0x07;
const SSM2518_DAC_FS_8000: c_uint = 0x00;
const SSM2518_DAC_FS_16000: c_uint = 0x01;
const SSM2518_DAC_FS_32000: c_uint = 0x02;
const SSM2518_DAC_FS_64000: c_uint = 0x03;
const SSM2518_DAC_FS_128000: c_uint = 0x04;

const SSM2518_SYSCLK: c_int = 0;
const SSM2518_SYSCLK_SRC_MCLK: c_int = 0;
const SSM2518_SYSCLK_SRC_BCLK: c_int = 1;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32: c_uint = 0;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    dev: device,
}
#[repr(C)]
pub struct reg_default {
    reg: c_uint,
    def: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
    mask: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    val_bits: c_uint,
    reg_bits: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
}
#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}
#[repr(C)]
pub struct i2c_device_id {
    name: *const c_char,
}
#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
struct ssm2518 {
    regmap: *mut regmap,
    right_j: bool,
    sysclk: c_uint,
    constraints: *const snd_pcm_hw_constraint_list,
    enable_gpio: *mut gpio_desc,
}

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut ssm2518;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: bool);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> c_int;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static SSM2518_REG_DEFAULTS: [reg_default; 19] = [
    reg_default { reg: 0x00, def: 0x05 },
    reg_default { reg: 0x01, def: 0x00 },
    reg_default { reg: 0x02, def: 0x02 },
    reg_default { reg: 0x03, def: 0x00 },
    reg_default { reg: 0x04, def: 0x10 },
    reg_default { reg: 0x05, def: 0x40 },
    reg_default { reg: 0x06, def: 0x40 },
    reg_default { reg: 0x07, def: 0x81 },
    reg_default { reg: 0x08, def: 0x0c },
    reg_default { reg: 0x09, def: 0x99 },
    reg_default { reg: 0x0a, def: 0x7c },
    reg_default { reg: 0x0b, def: 0x5b },
    reg_default { reg: 0x0c, def: 0x57 },
    reg_default { reg: 0x0d, def: 0x89 },
    reg_default { reg: 0x0e, def: 0x8c },
    reg_default { reg: 0x0f, def: 0x77 },
    reg_default { reg: 0x10, def: 0x26 },
    reg_default { reg: 0x11, def: 0x1c },
    reg_default { reg: 0x12, def: 0x97 },
];

/* static const DECLARE_TLV_DB_MINMAX_MUTE(ssm2518_vol_tlv, -7125, 2400); */
static SSM2518_VOL_TLV: [c_uint; 0] = [];
/* static const DECLARE_TLV_DB_SCALE(ssm2518_compressor_tlv, -3400, 200, 0); */
static SSM2518_COMPRESSOR_TLV: [c_uint; 0] = [];
/* static const DECLARE_TLV_DB_SCALE(ssm2518_expander_tlv, -8100, 300, 0); */
static SSM2518_EXPANDER_TLV: [c_uint; 0] = [];
/* static const DECLARE_TLV_DB_SCALE(ssm2518_noise_gate_tlv, -9600, 300, 0); */
static SSM2518_NOISE_GATE_TLV: [c_uint; 0] = [];
/* static const DECLARE_TLV_DB_SCALE(ssm2518_post_drc_tlv, -2400, 300, 0); */
static SSM2518_POST_DRC_TLV: [c_uint; 0] = [];
/* static const DECLARE_TLV_DB_RANGE(ssm2518_limiter_tlv, ...); */
static SSM2518_LIMITER_TLV: [c_uint; 0] = [];

static SSM2518_DRC_PEAK_DETECTOR_ATTACK_TIME_TEXT: [&[u8]; 16] = [
    b"0 ms\0", b"0.1 ms\0", b"0.19 ms\0", b"0.37 ms\0", b"0.75 ms\0", b"1.5 ms\0",
    b"3 ms\0", b"6 ms\0", b"12 ms\0", b"24 ms\0", b"48 ms\0", b"96 ms\0", b"192 ms\0",
    b"384 ms\0", b"768 ms\0", b"1536 ms\0",
];

static SSM2518_DRC_PEAK_DETECTOR_RELEASE_TIME_TEXT: [&[u8]; 16] = [
    b"0 ms\0", b"1.5 ms\0", b"3 ms\0", b"6 ms\0", b"12 ms\0", b"24 ms\0", b"48 ms\0",
    b"96 ms\0", b"192 ms\0", b"384 ms\0", b"768 ms\0", b"1536 ms\0", b"3072 ms\0",
    b"6144 ms\0", b"12288 ms\0", b"24576 ms\0",
];

static SSM2518_DRC_HOLD_TIME_TEXT: [&[u8]; 13] = [
    b"0 ms\0", b"0.67 ms\0", b"1.33 ms\0", b"2.67 ms\0", b"5.33 ms\0",
    b"10.66 ms\0", b"21.32 ms\0", b"42.64 ms\0", b"85.28 ms\0", b"170.56 ms\0",
    b"341.12 ms\0", b"682.24 ms\0", b"1364 ms\0",
];

/* static SOC_ENUM_SINGLE_DECL(...) declarations preserved as external ALSA macro intent. */

static SSM2518_SND_CONTROLS: [snd_kcontrol_new; 0] = [];

static SSM2518_DAPM_WIDGETS: [snd_soc_dapm_widget; 0] = [];

static SSM2518_ROUTES: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"OUTL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DACL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUTR\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DACR\0".as_ptr() as *const c_char,
    },
];

#[repr(C)]
struct ssm2518_mcs_lut {
    rate: c_uint,
    sysclks: *const c_uint,
}

static SSM2518_SYSCLKS_2048000: [c_uint; 10] = [
    2048000, 4096000, 8192000, 12288000, 16384000, 24576000, 3200000, 6400000,
    12800000, 0,
];

static SSM2518_SYSCLKS_2822000: [c_uint; 10] = [
    2822000, 5644800, 11289600, 16934400, 22579200, 33868800, 4410000, 8820000,
    17640000, 0,
];

static SSM2518_SYSCLKS_3072000: [c_uint; 10] = [
    3072000, 6144000, 12288000, 16384000, 24576000, 38864000, 4800000, 9600000,
    19200000, 0,
];

static SSM2518_MCS_LUT: [ssm2518_mcs_lut; 10] = [
    ssm2518_mcs_lut { rate: 8000, sysclks: SSM2518_SYSCLKS_2048000.as_ptr() },
    ssm2518_mcs_lut { rate: 11025, sysclks: SSM2518_SYSCLKS_2822000.as_ptr() },
    ssm2518_mcs_lut { rate: 12000, sysclks: SSM2518_SYSCLKS_3072000.as_ptr() },
    ssm2518_mcs_lut { rate: 16000, sysclks: SSM2518_SYSCLKS_2048000.as_ptr() },
    ssm2518_mcs_lut { rate: 24000, sysclks: SSM2518_SYSCLKS_3072000.as_ptr() },
    ssm2518_mcs_lut { rate: 22050, sysclks: SSM2518_SYSCLKS_2822000.as_ptr() },
    ssm2518_mcs_lut { rate: 32000, sysclks: SSM2518_SYSCLKS_2048000.as_ptr() },
    ssm2518_mcs_lut { rate: 44100, sysclks: SSM2518_SYSCLKS_2822000.as_ptr() },
    ssm2518_mcs_lut { rate: 48000, sysclks: SSM2518_SYSCLKS_3072000.as_ptr() },
    ssm2518_mcs_lut { rate: 96000, sysclks: SSM2518_SYSCLKS_3072000.as_ptr() },
];

static SSM2518_RATES_2048000: [c_uint; 3] = [8000, 16000, 32000];
static SSM2518_CONSTRAINTS_2048000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: SSM2518_RATES_2048000.as_ptr(),
    count: SSM2518_RATES_2048000.len() as c_uint,
    mask: 0,
};

static SSM2518_RATES_2822000: [c_uint; 3] = [11025, 22050, 44100];
static SSM2518_CONSTRAINTS_2822000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: SSM2518_RATES_2822000.as_ptr(),
    count: SSM2518_RATES_2822000.len() as c_uint,
    mask: 0,
};

static SSM2518_RATES_3072000: [c_uint; 4] = [12000, 24000, 48000, 96000];
static SSM2518_CONSTRAINTS_3072000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: SSM2518_RATES_3072000.as_ptr(),
    count: SSM2518_RATES_3072000.len() as c_uint,
    mask: 0,
};

static SSM2518_RATES_12288000: [c_uint; 7] = [8000, 12000, 16000, 24000, 32000, 48000, 96000];
static SSM2518_CONSTRAINTS_12288000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: SSM2518_RATES_12288000.as_ptr(),
    count: SSM2518_RATES_12288000.len() as c_uint,
    mask: 0,
};

unsafe fn __ffs(word: c_uint) -> c_int {
    word.trailing_zeros() as c_int
}

unsafe extern "C" fn ssm2518_lookup_mcs(ssm2518: *mut ssm2518, rate: c_uint) -> c_int {
    let mut sysclks: *const c_uint = ptr::null();
    let mut i: c_int;

    i = 0;
    while (i as usize) < SSM2518_MCS_LUT.len() {
        if SSM2518_MCS_LUT[i as usize].rate == rate {
            sysclks = SSM2518_MCS_LUT[i as usize].sysclks;
            break;
        }
        i += 1;
    }

    if sysclks.is_null() {
        return -EINVAL;
    }

    i = 0;
    while *sysclks.add(i as usize) != 0 {
        if *sysclks.add(i as usize) == (*ssm2518).sysclk {
            return i;
        }
        i += 1;
    }

    -EINVAL
}

unsafe extern "C" fn ssm2518_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ssm2518: *mut ssm2518 = snd_soc_component_get_drvdata(component);
    let rate: c_uint = params_rate(params);
    let mut ctrl1: c_uint;
    let mut ctrl1_mask: c_uint;
    let mcs: c_int;
    let mut ret: c_int;

    mcs = ssm2518_lookup_mcs(ssm2518, rate);
    if mcs < 0 {
        return mcs;
    }

    ctrl1_mask = SSM2518_SAI_CTRL1_FS_MASK;

    if rate >= 8000 && rate <= 12000 {
        ctrl1 = SSM2518_SAI_CTRL1_FS_8000_12000;
    } else if rate >= 16000 && rate <= 24000 {
        ctrl1 = SSM2518_SAI_CTRL1_FS_16000_24000;
    } else if rate >= 32000 && rate <= 48000 {
        ctrl1 = SSM2518_SAI_CTRL1_FS_32000_48000;
    } else if rate >= 64000 && rate <= 96000 {
        ctrl1 = SSM2518_SAI_CTRL1_FS_64000_96000;
    } else {
        return -EINVAL;
    }

    if (*ssm2518).right_j {
        match params_width(params) {
            16 => ctrl1 |= SSM2518_SAI_CTRL1_FMT_RJ_16BIT,
            24 => ctrl1 |= SSM2518_SAI_CTRL1_FMT_RJ_24BIT,
            _ => return -EINVAL,
        }
        ctrl1_mask |= SSM2518_SAI_CTRL1_FMT_MASK;
    }

    /* Disable auto samplerate detection */
    ret = regmap_update_bits((*ssm2518).regmap, SSM2518_REG_CLOCK, SSM2518_CLOCK_ASR, SSM2518_CLOCK_ASR);
    if ret < 0 {
        return ret;
    }

    ret = regmap_update_bits((*ssm2518).regmap, SSM2518_REG_SAI_CTRL1, ctrl1_mask, ctrl1);
    if ret < 0 {
        return ret;
    }

    regmap_update_bits(
        (*ssm2518).regmap,
        SSM2518_REG_POWER1,
        SSM2518_POWER1_MCS_MASK,
        (mcs as c_uint) << 1,
    )
}

unsafe extern "C" fn ssm2518_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let ssm2518: *mut ssm2518 = snd_soc_component_get_drvdata((*dai).component);
    let val: c_uint;

    if mute != 0 {
        val = SSM2518_MUTE_CTRL_MUTE_MASTER;
    } else {
        val = 0;
    }

    regmap_update_bits(
        (*ssm2518).regmap,
        SSM2518_REG_MUTE_CTRL,
        SSM2518_MUTE_CTRL_MUTE_MASTER,
        val,
    )
}

unsafe extern "C" fn ssm2518_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let ssm2518: *mut ssm2518 = snd_soc_component_get_drvdata((*dai).component);
    let mut ctrl1: c_uint = 0;
    let mut ctrl2: c_uint = 0;
    let mut invert_fclk: bool;
    let ret: c_int;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => invert_fclk = false,
        SND_SOC_DAIFMT_IB_NF => {
            ctrl2 |= SSM2518_SAI_CTRL2_BCLK_INVERT;
            invert_fclk = false;
        }
        SND_SOC_DAIFMT_NB_IF => invert_fclk = true,
        SND_SOC_DAIFMT_IB_IF => {
            ctrl2 |= SSM2518_SAI_CTRL2_BCLK_INVERT;
            invert_fclk = true;
        }
        _ => return -EINVAL,
    }

    (*ssm2518).right_j = false;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl1 |= SSM2518_SAI_CTRL1_FMT_I2S,
        SND_SOC_DAIFMT_LEFT_J => {
            ctrl1 |= SSM2518_SAI_CTRL1_FMT_LJ;
            invert_fclk = !invert_fclk;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            ctrl1 |= SSM2518_SAI_CTRL1_FMT_RJ_24BIT;
            (*ssm2518).right_j = true;
            invert_fclk = !invert_fclk;
        }
        SND_SOC_DAIFMT_DSP_A => {
            ctrl2 |= SSM2518_SAI_CTRL2_LRCLK_PULSE;
            ctrl1 |= SSM2518_SAI_CTRL1_FMT_I2S;
            invert_fclk = false;
        }
        SND_SOC_DAIFMT_DSP_B => {
            ctrl2 |= SSM2518_SAI_CTRL2_LRCLK_PULSE;
            ctrl1 |= SSM2518_SAI_CTRL1_FMT_LJ;
            invert_fclk = false;
        }
        _ => return -EINVAL,
    }

    if invert_fclk {
        ctrl2 |= SSM2518_SAI_CTRL2_LRCLK_INVERT;
    }

    ret = regmap_write((*ssm2518).regmap, SSM2518_REG_SAI_CTRL1, ctrl1);
    if ret != 0 {
        return ret;
    }

    regmap_write((*ssm2518).regmap, SSM2518_REG_SAI_CTRL2, ctrl2)
}

unsafe extern "C" fn ssm2518_set_power(ssm2518: *mut ssm2518, enable: bool) -> c_int {
    let mut ret: c_int = 0;

    if !enable {
        ret = regmap_update_bits(
            (*ssm2518).regmap,
            SSM2518_REG_POWER1,
            SSM2518_POWER1_SPWDN,
            SSM2518_POWER1_SPWDN,
        );
        regcache_mark_dirty((*ssm2518).regmap);
    }

    if !(*ssm2518).enable_gpio.is_null() {
        gpiod_set_value_cansleep((*ssm2518).enable_gpio, enable);
    }

    regcache_cache_only((*ssm2518).regmap, !enable);

    if enable {
        ret = regmap_update_bits(
            (*ssm2518).regmap,
            SSM2518_REG_POWER1,
            SSM2518_POWER1_SPWDN | SSM2518_POWER1_RESET,
            0x00,
        );
        regcache_sync((*ssm2518).regmap);
    }

    ret
}

unsafe extern "C" fn ssm2518_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let ssm2518: *mut ssm2518 = snd_soc_component_get_drvdata(component);
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut ret: c_int = 0;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                ret = ssm2518_set_power(ssm2518, true);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            ret = ssm2518_set_power(ssm2518, false);
        }
    }

    ret
}

unsafe extern "C" fn ssm2518_set_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    width: c_int,
) -> c_int {
    let ssm2518: *mut ssm2518 = snd_soc_component_get_drvdata((*dai).component);
    let ctrl1: c_uint;
    let ctrl2: c_uint;
    let left_slot: c_int;
    let right_slot: c_int;
    let mut ret: c_int;

    if slots == 0 {
        return regmap_update_bits(
            (*ssm2518).regmap,
            SSM2518_REG_SAI_CTRL1,
            SSM2518_SAI_CTRL1_SAI_MASK,
            SSM2518_SAI_CTRL1_SAI_I2S,
        );
    }

    if tx_mask == 0 || rx_mask != 0 {
        return -EINVAL;
    }

    if slots == 1 {
        if tx_mask != 1 {
            return -EINVAL;
        }
        left_slot = 0;
        right_slot = 0;
    } else {
        /* We assume the left channel < right channel */
        left_slot = __ffs(tx_mask);
        tx_mask &= !(1_u32 << left_slot);
        if tx_mask == 0 {
            right_slot = left_slot;
        } else {
            right_slot = __ffs(tx_mask);
            tx_mask &= !(1_u32 << right_slot);
        }
    }

    if tx_mask != 0 || left_slot >= slots || right_slot >= slots {
        return -EINVAL;
    }

    match width {
        16 => ctrl2 = SSM2518_SAI_CTRL2_SLOT_WIDTH_16,
        24 => ctrl2 = SSM2518_SAI_CTRL2_SLOT_WIDTH_24,
        32 => ctrl2 = SSM2518_SAI_CTRL2_SLOT_WIDTH_32,
        _ => return -EINVAL,
    }

    match slots {
        1 => ctrl1 = SSM2518_SAI_CTRL1_SAI_MONO,
        2 => ctrl1 = SSM2518_SAI_CTRL1_SAI_TDM_2,
        4 => ctrl1 = SSM2518_SAI_CTRL1_SAI_TDM_4,
        8 => ctrl1 = SSM2518_SAI_CTRL1_SAI_TDM_8,
        16 => ctrl1 = SSM2518_SAI_CTRL1_SAI_TDM_16,
        _ => return -EINVAL,
    }

    ret = regmap_write(
        (*ssm2518).regmap,
        SSM2518_REG_CHAN_MAP,
        ((left_slot as c_uint) << SSM2518_CHAN_MAP_LEFT_SLOT_OFFSET)
            | ((right_slot as c_uint) << SSM2518_CHAN_MAP_RIGHT_SLOT_OFFSET),
    );
    if ret != 0 {
        return ret;
    }

    ret = regmap_update_bits(
        (*ssm2518).regmap,
        SSM2518_REG_SAI_CTRL1,
        SSM2518_SAI_CTRL1_SAI_MASK,
        ctrl1,
    );
    if ret != 0 {
        return ret;
    }

    regmap_update_bits(
        (*ssm2518).regmap,
        SSM2518_REG_SAI_CTRL2,
        SSM2518_SAI_CTRL2_SLOT_WIDTH_MASK,
        ctrl2,
    )
}

unsafe extern "C" fn ssm2518_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ssm2518: *mut ssm2518 = snd_soc_component_get_drvdata((*dai).component);

    if !(*ssm2518).constraints.is_null() {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            (*ssm2518).constraints,
        );
    }

    0
}

const SSM2518_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32;

static SSM2518_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ssm2518_startup),
    hw_params: Some(ssm2518_hw_params),
    mute_stream: Some(ssm2518_mute),
    set_fmt: Some(ssm2518_set_dai_fmt),
    set_tdm_slot: Some(ssm2518_set_tdm_slot),
    no_capture_mute: 1,
};

static mut SSM2518_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ssm2518-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SSM2518_FORMATS,
    },
    ops: &SSM2518_DAI_OPS,
};

unsafe extern "C" fn ssm2518_set_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let ssm2518: *mut ssm2518 = snd_soc_component_get_drvdata(component);
    let val: c_uint;

    if clk_id != SSM2518_SYSCLK {
        return -EINVAL;
    }

    match source {
        SSM2518_SYSCLK_SRC_MCLK => val = 0,
        SSM2518_SYSCLK_SRC_BCLK => {
            /*
             * In this case the bitclock is used as the system clock, and
             * the bitclock signal needs to be connected to the MCLK pin and
             * the BCLK pin is left unconnected
             */
            val = SSM2518_POWER1_NO_BCLK;
        }
        _ => return -EINVAL,
    }

    match freq {
        0 => (*ssm2518).constraints = ptr::null(),
        2048000 | 4096000 | 8192000 | 3200000 | 6400000 | 12800000 => {
            (*ssm2518).constraints = &SSM2518_CONSTRAINTS_2048000;
        }
        2822000 | 5644800 | 11289600 | 16934400 | 22579200 | 33868800 | 4410000 | 8820000
        | 17640000 => {
            (*ssm2518).constraints = &SSM2518_CONSTRAINTS_2822000;
        }
        3072000 | 6144000 | 38864000 | 4800000 | 9600000 | 19200000 => {
            (*ssm2518).constraints = &SSM2518_CONSTRAINTS_3072000;
        }
        12288000 | 16384000 | 24576000 => {
            (*ssm2518).constraints = &SSM2518_CONSTRAINTS_12288000;
        }
        _ => return -EINVAL,
    }

    (*ssm2518).sysclk = freq;

    regmap_update_bits(
        (*ssm2518).regmap,
        SSM2518_REG_POWER1,
        SSM2518_POWER1_NO_BCLK,
        val,
    )
}

static SSM2518_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(ssm2518_set_bias_level),
    set_sysclk: Some(ssm2518_set_sysclk),
    controls: SSM2518_SND_CONTROLS.as_ptr(),
    num_controls: SSM2518_SND_CONTROLS.len() as c_uint,
    dapm_widgets: SSM2518_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: SSM2518_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: SSM2518_ROUTES.as_ptr(),
    num_dapm_routes: SSM2518_ROUTES.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

static SSM2518_REGMAP_CONFIG: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 8,
    max_register: SSM2518_REG_DRC_9,
    cache_type: REGCACHE_RBTREE,
    reg_defaults: SSM2518_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: SSM2518_REG_DEFAULTS.len() as c_uint,
};

unsafe extern "C" fn ssm2518_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ssm2518: *mut ssm2518;
    let mut ret: c_int;

    ssm2518 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<ssm2518>(),
        GFP_KERNEL,
    ) as *mut ssm2518;
    if ssm2518.is_null() {
        return -ENOMEM;
    }

    /* Start with enabling the chip */
    (*ssm2518).enable_gpio = devm_gpiod_get_optional(&mut (*i2c).dev, ptr::null(), GPIOD_OUT_HIGH);
    ret = PTR_ERR_OR_ZERO((*ssm2518).enable_gpio as *const c_void);
    if ret != 0 {
        return ret;
    }

    gpiod_set_consumer_name((*ssm2518).enable_gpio, b"SSM2518 nSD\0".as_ptr() as *const c_char);

    i2c_set_clientdata(i2c, ssm2518 as *mut c_void);

    (*ssm2518).regmap = devm_regmap_init_i2c(i2c, &SSM2518_REGMAP_CONFIG);
    if IS_ERR((*ssm2518).regmap as *const c_void) {
        return PTR_ERR((*ssm2518).regmap as *const c_void);
    }

    /*
     * The reset bit is obviously volatile, but we need to be able to cache
     * the other bits in the register, so we can't just mark the whole
     * register as volatile. Since this is the only place where we'll ever
     * touch the reset bit just bypass the cache for this operation.
     */
    regcache_cache_bypass((*ssm2518).regmap, true);
    ret = regmap_write((*ssm2518).regmap, SSM2518_REG_POWER1, SSM2518_POWER1_RESET);
    regcache_cache_bypass((*ssm2518).regmap, false);
    if ret != 0 {
        return ret;
    }

    ret = regmap_update_bits((*ssm2518).regmap, SSM2518_REG_POWER2, SSM2518_POWER2_APWDN, 0x00);
    if ret != 0 {
        return ret;
    }

    ret = ssm2518_set_power(ssm2518, false);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &SSM2518_COMPONENT_DRIVER,
        &mut SSM2518_DAI,
        1,
    )
}

/* #ifdef CONFIG_OF */
static SSM2518_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"adi,ssm2518\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, ssm2518_dt_ids); */
/* #endif */

static SSM2518_I2C_IDS: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"ssm2518\0".as_ptr() as *const c_char,
    },
    i2c_device_id { name: ptr::null() },
];
/* MODULE_DEVICE_TABLE(i2c, ssm2518_i2c_ids); */

static mut SSM2518_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"ssm2518\0".as_ptr() as *const c_char,
        of_match_table: SSM2518_DT_IDS.as_ptr(),
    },
    probe: Some(ssm2518_i2c_probe),
    id_table: SSM2518_I2C_IDS.as_ptr(),
};
/* module_i2c_driver(ssm2518_driver); */

/* MODULE_DESCRIPTION("ASoC SSM2518 driver"); */
/* MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
