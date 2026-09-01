// SPDX-License-Identifier: GPL-2.0-or-later

/*
 * Based on code by Hu Jin
 * Copyright (C) 2014 Asahi Kasei Microdevices Corporation
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

/* Dependencies from Linux/ASoC headers:
 * linux/delay.h, linux/gpio/consumer.h, linux/i2c.h, linux/module.h,
 * linux/of.h, linux/pm_runtime.h, linux/regulator/consumer.h,
 * sound/soc.h, sound/tlv.h
 */

type u8 = u8;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

/* Registers and fields */
const AK4375_00_POWER_MANAGEMENT1: c_uint = 0x00;
const PMPLL: c_uint = BIT(0); /* 0: PLL off, 1: PLL on */
const AK4375_01_POWER_MANAGEMENT2: c_uint = 0x01;
const PMCP1: c_uint = BIT(0); /* Charge Pump 1: LDO1 and DAC */
const PMCP2: c_uint = BIT(1); /* Charge Pump 2: Class-G HP Amp */
const PMLDO1P: c_uint = BIT(4);
const PMLDO1N: c_uint = BIT(5);
const PMLDO: c_uint = PMLDO1P | PMLDO1N;
const AK4375_02_POWER_MANAGEMENT3: c_uint = 0x02;
const AK4375_03_POWER_MANAGEMENT4: c_uint = 0x03;
const AK4375_04_OUTPUT_MODE_SETTING: c_uint = 0x04;
const AK4375_05_CLOCK_MODE_SELECT: c_uint = 0x05;
const FS_MASK: c_uint = GENMASK(4, 0);
const FS_8KHZ: c_uint = 0x00;
const FS_11_025KHZ: c_uint = 0x01;
const FS_16KHZ: c_uint = 0x04;
const FS_22_05KHZ: c_uint = 0x05;
const FS_32KHZ: c_uint = 0x08;
const FS_44_1KHZ: c_uint = 0x09;
const FS_48KHZ: c_uint = 0x0a;
const FS_88_2KHZ: c_uint = 0x0d;
const FS_96KHZ: c_uint = 0x0e;
const FS_176_4KHZ: c_uint = 0x11;
const FS_192KHZ: c_uint = 0x12;
const CM_MASK: c_uint = GENMASK(6, 5); /* For SRC Bypass mode */
const CM_0: c_uint = 0x0 << 5;
const CM_1: c_uint = 0x1 << 5;
const CM_2: c_uint = 0x2 << 5;
const CM_3: c_uint = 0x3 << 5;
const AK4375_06_DIGITAL_FILTER_SELECT: c_uint = 0x06;
const DADFSEL: c_uint = BIT(5); /* 0: in SRC Bypass mode, 1: in SRC mode */
const DASL: c_uint = BIT(6);
const DASD: c_uint = BIT(7);
const AK4375_07_DAC_MONO_MIXING: c_uint = 0x07;
const DACMUTE_MASK: c_uint = GENMASK(5, 4) | GENMASK(1, 0); /* Clear to mute */
const AK4375_08_JITTER_CLEANER_SETTING1: c_uint = 0x08;
const AK4375_09_JITTER_CLEANER_SETTING2: c_uint = 0x09;
const AK4375_0A_JITTER_CLEANER_SETTING3: c_uint = 0x0a;
const SELDAIN: c_uint = BIT(1); /* 0: SRC Bypass mode, 1: SRC mode */
const XCKSEL: c_uint = BIT(6); /* 0: PLL0, 1: MCKI */
const XCKCPSEL: c_uint = BIT(7); /* Should be equal to SELDAIN and XCKSEL */
const AK4375_0B_LCH_OUTPUT_VOLUME: c_uint = 0x0b;
const AK4375_0C_RCH_OUTPUT_VOLUME: c_uint = 0x0c;
const AK4375_0D_HP_VOLUME_CONTROL: c_uint = 0x0d;
const AK4375_0E_PLL_CLK_SOURCE_SELECT: c_uint = 0x0e;
const PLS: c_uint = BIT(0); /* 0: MCKI, 1: BCLK */
const AK4375_0F_PLL_REF_CLK_DIVIDER1: c_uint = 0x0f; /* Reference clock divider [15:8] bits */
const AK4375_10_PLL_REF_CLK_DIVIDER2: c_uint = 0x10; /* Reference clock divider [7:0] bis */
const AK4375_11_PLL_FB_CLK_DIVIDER1: c_uint = 0x11; /* Feedback clock divider [15:8] bits */
const AK4375_12_PLL_FB_CLK_DIVIDER2: c_uint = 0x12; /* Feedback clock divider [7:0] bits */
const AK4375_13_SRC_CLK_SOURCE: c_uint = 0x13; /* SRC Bypass: SRCCKS=XCKSEL=SELDAIN=0 */
const SRCCKS: c_uint = BIT(0); /* SRC Clock source 0: MCKI, 1: PLL0 */
const DIV: c_uint = BIT(4);
const AK4375_14_DAC_CLK_DIVIDER: c_uint = 0x14;
const AK4375_15_AUDIO_IF_FORMAT: c_uint = 0x15;
const DEVICEID_MASK: c_uint = GENMASK(7, 5);
const AK4375_24_MODE_CONTROL: c_uint = 0x24;

const AK4375_PLL_FREQ_OUT_112896000: c_uint = 112896000; /* 44.1 kHz base rate */
const AK4375_PLL_FREQ_OUT_122880000: c_uint = 122880000; /* 32 and 48 kHz base rates */

const DEVICEID_AK4375: c_uint = 0x00;
const DEVICEID_AK4375A: c_uint = 0x01;
const DEVICEID_AK4376A: c_uint = 0x02;
const DEVICEID_AK4377: c_uint = 0x03;
const DEVICEID_AK4331: c_uint = 0x07;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct gpio_desc { _private: [u8; 0] }
#[repr(C)] struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct regulator_bulk_data {
    supply: *const c_char,
}

#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct ak4375_drvdata {
    dai_drv: *mut snd_soc_dai_driver,
    comp_drv: *const snd_soc_component_driver,
}

#[repr(C)]
struct ak4375_priv {
    dev: *mut device,
    regmap: *mut regmap,
    pdn_gpiod: *mut gpio_desc,
    supplies: [regulator_bulk_data; 2],
    rate: c_uint,
    pld: c_uint,
    mute_save: u8,
}

/* External kernel/ASoC data structures are declared with the fields used here. */
#[repr(C)] struct soc_enum { _private: [u8; 0] }
#[repr(C)] struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)] struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] struct regmap_config { _private: [u8; 0] }
#[repr(C)] struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] struct of_device_id { _private: [u8; 0] }
#[repr(C)] struct i2c_driver { _private: [u8; 0] }

static supply_names: [*const c_char; 2] = [b"avdd\0".as_ptr().cast(), b"tvdd\0".as_ptr().cast()];

static ak4375_reg_defaults: [reg_default; 23] = [
    reg_default { reg: 0x00, def: 0x00 }, reg_default { reg: 0x01, def: 0x00 }, reg_default { reg: 0x02, def: 0x00 },
    reg_default { reg: 0x03, def: 0x00 }, reg_default { reg: 0x04, def: 0x00 }, reg_default { reg: 0x05, def: 0x00 },
    reg_default { reg: 0x06, def: 0x00 }, reg_default { reg: 0x07, def: 0x00 }, reg_default { reg: 0x08, def: 0x00 },
    reg_default { reg: 0x09, def: 0x00 }, reg_default { reg: 0x0a, def: 0x00 }, reg_default { reg: 0x0b, def: 0x19 },
    reg_default { reg: 0x0c, def: 0x19 }, reg_default { reg: 0x0d, def: 0x75 }, reg_default { reg: 0x0e, def: 0x01 },
    reg_default { reg: 0x0f, def: 0x00 }, reg_default { reg: 0x10, def: 0x00 }, reg_default { reg: 0x11, def: 0x00 },
    reg_default { reg: 0x12, def: 0x00 }, reg_default { reg: 0x13, def: 0x00 }, reg_default { reg: 0x14, def: 0x00 },
    reg_default { reg: 0x15, def: 0x00 }, reg_default { reg: 0x24, def: 0x00 },
];

/*
 * Output Digital volume control:
 * from -12.5 to 3 dB in 0.5 dB steps (mute instead of -12.5 dB)
 */
static dac_tlv: [c_uint; 4] = [0, (-1250i32) as c_uint, 50, 0];

/*
 * HP-Amp Analog volume control:
 * from -4.2 to 6 dB in 2 dB steps (mute instead of -4.2 dB)
 */
static hpg_tlv: [c_uint; 4] = [0, (-4200i32) as c_uint, 20, 0];

static ak4375_ovolcn_select_texts: [*const c_char; 2] = [b"Dependent\0".as_ptr().cast(), b"Independent\0".as_ptr().cast()];
static ak4375_mdac_select_texts: [*const c_char; 2] = [b"x1\0".as_ptr().cast(), b"x1/2\0".as_ptr().cast()];
static ak4375_cpmode_select_texts: [*const c_char; 3] = [
    b"Automatic Switching\0".as_ptr().cast(),
    b"+-VDD Operation\0".as_ptr().cast(),
    b"+-1/2VDD Operation\0".as_ptr().cast(),
];

/*
 * DASD, DASL bits Digital Filter Setting
 * 0, 0 : Sharp Roll-Off Filter
 * 0, 1 : Slow Roll-Off Filter
 * 1, 0 : Short delay Sharp Roll-Off Filter
 * 1, 1 : Short delay Slow Roll-Off Filter
 */
static ak4375_digfil_select_texts: [*const c_char; 4] = [
    b"Sharp Roll-Off Filter\0".as_ptr().cast(),
    b"Slow Roll-Off Filter\0".as_ptr().cast(),
    b"Short delay Sharp Roll-Off Filter\0".as_ptr().cast(),
    b"Short delay Slow Roll-Off Filter\0".as_ptr().cast(),
];

/* SOC_ENUM_* and SOC_* control initializers are supplied by ASoC headers in C. */
static ak4375_ovolcn_enum: soc_enum = soc_enum { _private: [] };
static ak4375_mdacl_enum: soc_enum = soc_enum { _private: [] };
static ak4375_mdacr_enum: soc_enum = soc_enum { _private: [] };
static ak4375_cpmode_enum: soc_enum = soc_enum { _private: [] };
static ak4375_digfil_enum: soc_enum = soc_enum { _private: [] };

static ak4375_snd_controls: [snd_kcontrol_new; 8] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static ak4375_hpl_mixer_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static ak4375_hpr_mixer_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn usleep_range(min: c_uint, max: c_uint);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
}

const SND_SOC_DAPM_PRE_PMU: c_int = 1 << 0;
const SND_SOC_DAPM_POST_PMU: c_int = 1 << 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 1 << 2;
const SND_SOC_DAPM_POST_PMD: c_int = 1 << 3;
const SND_SOC_NOPM: c_int = -1;
const GPIOD_OUT_LOW: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const SNDRV_PCM_RATE_88200: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_176400: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 0;

unsafe extern "C" fn ak4375_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, AK4375_00_POWER_MANAGEMENT1, PMPLL, PMPLL);
            snd_soc_component_update_bits(component, AK4375_01_POWER_MANAGEMENT2, PMCP1, PMCP1);
            usleep_range(6500, 7000);
            snd_soc_component_update_bits(component, AK4375_01_POWER_MANAGEMENT2, PMLDO, PMLDO);
            usleep_range(1000, 2000);
        }
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_update_bits(component, AK4375_01_POWER_MANAGEMENT2, PMCP2, PMCP2);
            usleep_range(4500, 5000);
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, AK4375_01_POWER_MANAGEMENT2, PMCP2, 0x0);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, AK4375_01_POWER_MANAGEMENT2, PMLDO, 0x0);
            snd_soc_component_update_bits(component, AK4375_01_POWER_MANAGEMENT2, PMCP1, 0x0);
            snd_soc_component_update_bits(component, AK4375_00_POWER_MANAGEMENT1, PMPLL, 0x0);
        }
        _ => {}
    }

    0
}

/* SND_SOC_DAPM_* widget initializers are represented as opaque ASoC descriptors. */
static ak4375_dapm_widgets: [snd_soc_dapm_widget_desc; 6] = [
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
];

static ak4375_intercon: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr().cast(), control: ptr::null(), source: b"SDTI\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HPL Mixer\0".as_ptr().cast(), control: b"LDACL Switch\0".as_ptr().cast(), source: b"DAC\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HPL Mixer\0".as_ptr().cast(), control: b"RDACL Switch\0".as_ptr().cast(), source: b"DAC\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HPR Mixer\0".as_ptr().cast(), control: b"LDACR Switch\0".as_ptr().cast(), source: b"DAC\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HPR Mixer\0".as_ptr().cast(), control: b"RDACR Switch\0".as_ptr().cast(), source: b"DAC\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HPL\0".as_ptr().cast(), control: ptr::null(), source: b"HPL Mixer\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HPR\0".as_ptr().cast(), control: ptr::null(), source: b"HPR Mixer\0".as_ptr().cast() },
];

unsafe extern "C" fn ak4375_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let ak4375 = snd_soc_component_get_drvdata(component) as *mut ak4375_priv;
    let freq_in: c_uint;
    let freq_out: c_uint;

    (*ak4375).rate = params_rate(params);

    if (*ak4375).rate <= 96000 {
        (*ak4375).pld = 0;
    } else {
        (*ak4375).pld = 1;
    }

    freq_in = 32u32.wrapping_mul((*ak4375).rate) / ((*ak4375).pld + 1);

    if ((*ak4375).rate % 8000) == 0 {
        freq_out = AK4375_PLL_FREQ_OUT_122880000;
    } else {
        freq_out = AK4375_PLL_FREQ_OUT_112896000;
    }

    snd_soc_dai_set_pll(dai, 0, 0, freq_in, freq_out)
}

unsafe extern "C" fn ak4375_dai_set_pll(
    dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*dai).component;
    let ak4375 = snd_soc_component_get_drvdata(component) as *mut ak4375_priv;
    let mclk: c_uint;
    let plm: c_uint;
    let mdiv: c_uint;
    let div: c_uint;
    let cms: u8;
    let mut fs: u8;
    let mut cm: u8;

    cms = snd_soc_component_read(component, AK4375_05_CLOCK_MODE_SELECT) as u8;
    fs = cms & !(FS_MASK as u8);
    cm = cms & !(CM_MASK as u8);

    match (*ak4375).rate {
        8000 => fs |= FS_8KHZ as u8,
        11025 => fs |= FS_11_025KHZ as u8,
        16000 => fs |= FS_16KHZ as u8,
        22050 => fs |= FS_22_05KHZ as u8,
        32000 => fs |= FS_32KHZ as u8,
        44100 => fs |= FS_44_1KHZ as u8,
        48000 => fs |= FS_48KHZ as u8,
        88200 => fs |= FS_88_2KHZ as u8,
        96000 => fs |= FS_96KHZ as u8,
        176400 => fs |= FS_176_4KHZ as u8,
        192000 => fs |= FS_192KHZ as u8,
        _ => return -EINVAL,
    }

    if (*ak4375).rate <= 24000 {
        cm |= CM_1 as u8;
        mclk = 512u32.wrapping_mul((*ak4375).rate);
        mdiv = freq_out / mclk - 1;
        div = 0;
    } else if (*ak4375).rate <= 96000 {
        cm |= CM_0 as u8;
        mclk = 256u32.wrapping_mul((*ak4375).rate);
        mdiv = freq_out / mclk - 1;
        div = 0;
    } else {
        cm |= CM_3 as u8;
        mclk = 128u32.wrapping_mul((*ak4375).rate);
        mdiv = 4;
        div = 1;
    }

    /* Writing both fields in one go seems to make playback choppy on start */
    snd_soc_component_update_bits(component, AK4375_05_CLOCK_MODE_SELECT, FS_MASK, fs as c_uint);
    snd_soc_component_update_bits(component, AK4375_05_CLOCK_MODE_SELECT, CM_MASK, cm as c_uint);

    snd_soc_component_write(
        component,
        AK4375_0F_PLL_REF_CLK_DIVIDER1,
        ((*ak4375).pld & 0xff00) >> 8,
    );
    snd_soc_component_write(component, AK4375_10_PLL_REF_CLK_DIVIDER2, (*ak4375).pld & 0x00ff);

    plm = freq_out / freq_in - 1;
    snd_soc_component_write(component, AK4375_11_PLL_FB_CLK_DIVIDER1, (plm & 0xff00) >> 8);
    snd_soc_component_write(component, AK4375_12_PLL_FB_CLK_DIVIDER2, plm & 0x00ff);

    snd_soc_component_update_bits(component, AK4375_13_SRC_CLK_SOURCE, DIV, div);

    /* SRCCKS bit: force to 1 for SRC PLL source clock */
    snd_soc_component_update_bits(component, AK4375_13_SRC_CLK_SOURCE, SRCCKS, SRCCKS);

    snd_soc_component_write(component, AK4375_14_DAC_CLK_DIVIDER, mdiv);

    dev_dbg(
        (*ak4375).dev,
        b"rate=%d mclk=%d f_in=%d f_out=%d PLD=%d PLM=%d MDIV=%d DIV=%d\n\0".as_ptr().cast(),
        (*ak4375).rate,
        mclk,
        freq_in,
        freq_out,
        (*ak4375).pld,
        plm,
        mdiv,
        div,
    );

    0
}

unsafe extern "C" fn ak4375_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let ak4375 = snd_soc_component_get_drvdata(component) as *mut ak4375_priv;
    let mut val: u8 = snd_soc_component_read(component, AK4375_07_DAC_MONO_MIXING) as u8;

    dev_dbg((*ak4375).dev, b"mute=%d val=%d\n\0".as_ptr().cast(), mute, val as c_int);

    if mute != 0 {
        (*ak4375).mute_save = val & (DACMUTE_MASK as u8);
        val &= !(DACMUTE_MASK as u8);
    } else {
        val |= (*ak4375).mute_save;
    }

    snd_soc_component_write(component, AK4375_07_DAC_MONO_MIXING, val as c_uint);

    0
}

const AK4375_RATES: c_uint = SNDRV_PCM_RATE_8000_48000 |
    SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000;

const AK4375_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE;

static ak4375_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { _private: [] };

static mut ak4375_dai: snd_soc_dai_driver = snd_soc_dai_driver { _private: [] };

unsafe fn ak4375_power_off(ak4375: *mut ak4375_priv) {
    gpiod_set_value_cansleep((*ak4375).pdn_gpiod, 0);
    usleep_range(1000, 2000);

    regulator_bulk_disable(ARRAY_SIZE(&(*ak4375).supplies) as c_int, (*ak4375).supplies.as_mut_ptr());
}

unsafe fn ak4375_power_on(ak4375: *mut ak4375_priv) -> c_int {
    let ret: c_int;

    ret = regulator_bulk_enable(ARRAY_SIZE(&(*ak4375).supplies) as c_int, (*ak4375).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*ak4375).dev, b"Failed to enable regulators: %d\n\0".as_ptr().cast(), ret);
        return ret;
    }

    usleep_range(3000, 4000);

    gpiod_set_value_cansleep((*ak4375).pdn_gpiod, 1);
    usleep_range(1000, 2000);

    0
}

unsafe extern "C" fn ak4375_runtime_suspend(dev: *mut device) -> c_int {
    let ak4375 = dev_get_drvdata(dev) as *mut ak4375_priv;

    regcache_cache_only((*ak4375).regmap, true);
    ak4375_power_off(ak4375);

    0
}

unsafe extern "C" fn ak4375_runtime_resume(dev: *mut device) -> c_int {
    let ak4375 = dev_get_drvdata(dev) as *mut ak4375_priv;
    let ret: c_int;

    ret = ak4375_power_on(ak4375);
    if ret < 0 {
        return ret;
    }

    regcache_cache_only((*ak4375).regmap, false);
    regcache_mark_dirty((*ak4375).regmap);

    regcache_sync((*ak4375).regmap)
}

static soc_codec_dev_ak4375: snd_soc_component_driver = snd_soc_component_driver { _private: [] };

static ak4375_regmap: regmap_config = regmap_config { _private: [] };

static ak4375_drvdata: ak4375_drvdata = ak4375_drvdata {
    dai_drv: unsafe { &raw mut ak4375_dai },
    comp_drv: &soc_codec_dev_ak4375,
};

static ak4375_pm: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn ak4375_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ak4375: *mut ak4375_priv;
    let drvdata: *const ak4375_drvdata;
    let mut deviceid: c_uint = 0;
    let mut ret: c_int;
    let mut i: c_int;

    ak4375 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<ak4375_priv>(), GFP_KERNEL) as *mut ak4375_priv;
    if ak4375.is_null() {
        return -ENOMEM;
    }

    (*ak4375).regmap = devm_regmap_init_i2c(i2c, &ak4375_regmap);
    if IS_ERR((*ak4375).regmap.cast()) {
        return PTR_ERR((*ak4375).regmap.cast());
    }

    i2c_set_clientdata(i2c, ak4375.cast());
    (*ak4375).dev = &mut (*i2c).dev;

    drvdata = of_device_get_match_data(&mut (*i2c).dev) as *const ak4375_drvdata;

    i = 0;
    while i < ARRAY_SIZE(&supply_names) as c_int {
        (*ak4375).supplies[i as usize].supply = supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        (*ak4375).dev,
        ARRAY_SIZE(&(*ak4375).supplies) as c_int,
        (*ak4375).supplies.as_mut_ptr(),
    );
    if ret < 0 {
        dev_err((*ak4375).dev, b"Failed to get regulators: %d\n\0".as_ptr().cast(), ret);
        return ret;
    }

    (*ak4375).pdn_gpiod = devm_gpiod_get_optional((*ak4375).dev, b"pdn\0".as_ptr().cast(), GPIOD_OUT_LOW);
    if IS_ERR((*ak4375).pdn_gpiod.cast()) {
        return dev_err_probe(
            (*ak4375).dev,
            PTR_ERR((*ak4375).pdn_gpiod.cast()),
            b"failed to get pdn\n\0".as_ptr().cast(),
        );
    }

    ret = ak4375_power_on(ak4375);
    if ret < 0 {
        return ret;
    }

    /* Don't read deviceid from cache */
    regcache_cache_bypass((*ak4375).regmap, true);

    ret = regmap_read((*ak4375).regmap, AK4375_15_AUDIO_IF_FORMAT, &mut deviceid);
    if ret < 0 {
        dev_err((*ak4375).dev, b"unable to read DEVICEID!\n\0".as_ptr().cast());
        return ret;
    }

    regcache_cache_bypass((*ak4375).regmap, false);

    deviceid = (deviceid & DEVICEID_MASK) >> 5;

    match deviceid {
        DEVICEID_AK4331 => {
            dev_err((*ak4375).dev, b"found untested AK4331\n\0".as_ptr().cast());
            return -EINVAL;
        }
        DEVICEID_AK4375 => {
            dev_dbg((*ak4375).dev, b"found AK4375\n\0".as_ptr().cast());
        }
        DEVICEID_AK4375A => {
            dev_dbg((*ak4375).dev, b"found AK4375A\n\0".as_ptr().cast());
        }
        DEVICEID_AK4376A => {
            dev_err((*ak4375).dev, b"found unsupported AK4376/A!\n\0".as_ptr().cast());
            return -EINVAL;
        }
        DEVICEID_AK4377 => {
            dev_err((*ak4375).dev, b"found unsupported AK4377!\n\0".as_ptr().cast());
            return -EINVAL;
        }
        _ => {
            dev_err((*ak4375).dev, b"unrecognized DEVICEID!\n\0".as_ptr().cast());
            return -EINVAL;
        }
    }

    pm_runtime_set_active((*ak4375).dev);
    pm_runtime_enable((*ak4375).dev);

    ret = devm_snd_soc_register_component((*ak4375).dev, (*drvdata).comp_drv, (*drvdata).dai_drv, 1);
    if ret < 0 {
        dev_err((*ak4375).dev, b"Failed to register CODEC: %d\n\0".as_ptr().cast(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn ak4375_i2c_remove(i2c: *mut i2c_client) {
    pm_runtime_disable(&mut (*i2c).dev);
}

static ak4375_of_match: [of_device_id; 2] = [
    of_device_id { _private: [] },
    of_device_id { _private: [] },
];
/* MODULE_DEVICE_TABLE(of, ak4375_of_match); */

static mut ak4375_i2c_driver: i2c_driver = i2c_driver { _private: [] };
/* module_i2c_driver(ak4375_i2c_driver); */

/* MODULE_AUTHOR("Vincent Knecht <vincent.knecht@mailoo.org>"); */
/* MODULE_DESCRIPTION("ASoC AK4375 DAC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
