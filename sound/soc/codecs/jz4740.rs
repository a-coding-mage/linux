// SPDX-License-Identifier: GPL-2.0
//
// JZ4740 CODEC driver
//
// Copyright (C) 2009-2010, Lars-Peter Clausen <lars@metafoo.de>

// Translated from Linux C implementation source. External Linux/ASoC symbols,
// types, constants, and descriptor-construction macros are expected from the
// surrounding kernel Rust bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const JZ4740_REG_CODEC_1: c_uint = 0x0;
const JZ4740_REG_CODEC_2: c_uint = 0x4;

const JZ4740_CODEC_1_LINE_ENABLE: c_uint = BIT(29);
const JZ4740_CODEC_1_MIC_ENABLE: c_uint = BIT(28);
const JZ4740_CODEC_1_SW1_ENABLE: c_uint = BIT(27);
const JZ4740_CODEC_1_ADC_ENABLE: c_uint = BIT(26);
const JZ4740_CODEC_1_SW2_ENABLE: c_uint = BIT(25);
const JZ4740_CODEC_1_DAC_ENABLE: c_uint = BIT(24);
const JZ4740_CODEC_1_VREF_DISABLE: c_uint = BIT(20);
const JZ4740_CODEC_1_VREF_AMP_DISABLE: c_uint = BIT(19);
const JZ4740_CODEC_1_VREF_PULLDOWN: c_uint = BIT(18);
const JZ4740_CODEC_1_VREF_LOW_CURRENT: c_uint = BIT(17);
const JZ4740_CODEC_1_VREF_HIGH_CURRENT: c_uint = BIT(16);
const JZ4740_CODEC_1_HEADPHONE_DISABLE: c_uint = BIT(14);
const JZ4740_CODEC_1_HEADPHONE_AMP_CHANGE_ANY: c_uint = BIT(13);
const JZ4740_CODEC_1_HEADPHONE_CHARGE: c_uint = BIT(12);
const JZ4740_CODEC_1_HEADPHONE_PULLDOWN: c_uint = BIT(11) | BIT(10);
const JZ4740_CODEC_1_HEADPHONE_POWERDOWN_M: c_uint = BIT(9);
const JZ4740_CODEC_1_HEADPHONE_POWERDOWN: c_uint = BIT(8);
const JZ4740_CODEC_1_SUSPEND: c_uint = BIT(1);
const JZ4740_CODEC_1_RESET: c_uint = BIT(0);

const JZ4740_CODEC_1_LINE_ENABLE_OFFSET: c_uint = 29;
const JZ4740_CODEC_1_MIC_ENABLE_OFFSET: c_uint = 28;
const JZ4740_CODEC_1_SW1_ENABLE_OFFSET: c_uint = 27;
const JZ4740_CODEC_1_ADC_ENABLE_OFFSET: c_uint = 26;
const JZ4740_CODEC_1_SW2_ENABLE_OFFSET: c_uint = 25;
const JZ4740_CODEC_1_DAC_ENABLE_OFFSET: c_uint = 24;
const JZ4740_CODEC_1_HEADPHONE_DISABLE_OFFSET: c_uint = 14;
const JZ4740_CODEC_1_HEADPHONE_POWERDOWN_OFFSET: c_uint = 8;

const JZ4740_CODEC_2_INPUT_VOLUME_MASK: c_uint = 0x1f0000;
const JZ4740_CODEC_2_SAMPLE_RATE_MASK: c_uint = 0x000f00;
const JZ4740_CODEC_2_MIC_BOOST_GAIN_MASK: c_uint = 0x000030;
const JZ4740_CODEC_2_HEADPHONE_VOLUME_MASK: c_uint = 0x000003;

const JZ4740_CODEC_2_INPUT_VOLUME_OFFSET: c_uint = 16;
const JZ4740_CODEC_2_SAMPLE_RATE_OFFSET: c_uint = 8;
const JZ4740_CODEC_2_MIC_BOOST_GAIN_OFFSET: c_uint = 4;
const JZ4740_CODEC_2_HEADPHONE_VOLUME_OFFSET: c_uint = 0;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
    _private: [u8; 0],
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct jz4740_codec {
    pub regmap: *mut regmap,
}

static jz4740_codec_reg_defaults: [reg_default; 2] = [
    reg_default {
        reg: JZ4740_REG_CODEC_1,
        def: 0x021b2302,
    },
    reg_default {
        reg: JZ4740_REG_CODEC_2,
        def: 0x00170803,
    },
];

DECLARE_TLV_DB_RANGE!(
    jz4740_mic_tlv,
    0,
    2,
    TLV_DB_SCALE_ITEM!(0, 600, 0),
    3,
    3,
    TLV_DB_SCALE_ITEM!(2000, 0, 0)
);

DECLARE_TLV_DB_SCALE!(jz4740_out_tlv, 0, 200, 0);
DECLARE_TLV_DB_SCALE!(jz4740_in_tlv, -3450, 150, 0);

static jz4740_codec_controls: [snd_kcontrol_new; 4] = [
    SOC_SINGLE_TLV!(
        "Master Playback Volume\0",
        JZ4740_REG_CODEC_2,
        JZ4740_CODEC_2_HEADPHONE_VOLUME_OFFSET,
        3,
        0,
        jz4740_out_tlv
    ),
    SOC_SINGLE_TLV!(
        "Master Capture Volume\0",
        JZ4740_REG_CODEC_2,
        JZ4740_CODEC_2_INPUT_VOLUME_OFFSET,
        31,
        0,
        jz4740_in_tlv
    ),
    SOC_SINGLE!(
        "Master Playback Switch\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_HEADPHONE_DISABLE_OFFSET,
        1,
        1
    ),
    SOC_SINGLE_TLV!(
        "Mic Capture Volume\0",
        JZ4740_REG_CODEC_2,
        JZ4740_CODEC_2_MIC_BOOST_GAIN_OFFSET,
        3,
        0,
        jz4740_mic_tlv
    ),
];

static jz4740_codec_output_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!(
        "Bypass Switch\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_SW1_ENABLE_OFFSET,
        1,
        0
    ),
    SOC_DAPM_SINGLE!(
        "DAC Switch\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_SW2_ENABLE_OFFSET,
        1,
        0
    ),
];

static jz4740_codec_input_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!(
        "Line Capture Switch\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_LINE_ENABLE_OFFSET,
        1,
        0
    ),
    SOC_DAPM_SINGLE!(
        "Mic Capture Switch\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_MIC_ENABLE_OFFSET,
        1,
        0
    ),
];

static jz4740_codec_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    SND_SOC_DAPM_ADC!(
        "ADC\0",
        "Capture\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_ADC_ENABLE_OFFSET,
        0
    ),
    SND_SOC_DAPM_DAC!(
        "DAC\0",
        "Playback\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_DAC_ENABLE_OFFSET,
        0
    ),
    SND_SOC_DAPM_MIXER!(
        "Output Mixer\0",
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_HEADPHONE_POWERDOWN_OFFSET,
        1,
        jz4740_codec_output_controls,
        ARRAY_SIZE!(jz4740_codec_output_controls)
    ),
    SND_SOC_DAPM_MIXER_NAMED_CTL!(
        "Input Mixer\0",
        SND_SOC_NOPM,
        0,
        0,
        jz4740_codec_input_controls,
        ARRAY_SIZE!(jz4740_codec_input_controls)
    ),
    SND_SOC_DAPM_MIXER!("Line Input\0", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("LOUT\0"),
    SND_SOC_DAPM_OUTPUT!("ROUT\0"),
    SND_SOC_DAPM_INPUT!("MIC\0"),
    SND_SOC_DAPM_INPUT!("LIN\0"),
    SND_SOC_DAPM_INPUT!("RIN\0"),
];

static jz4740_codec_dapm_routes: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route { sink: c"Line Input".as_ptr(), control: ptr::null(), source: c"LIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Line Input".as_ptr(), control: ptr::null(), source: c"RIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Line Capture Switch".as_ptr(), source: c"Line Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Mic Capture Switch".as_ptr(), source: c"MIC".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: ptr::null(), source: c"Input Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: c"Bypass Switch".as_ptr(), source: c"Input Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: c"DAC Switch".as_ptr(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT".as_ptr(), control: ptr::null(), source: c"Output Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT".as_ptr(), control: ptr::null(), source: c"Output Mixer".as_ptr() },
];

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn udelay(usecs: c_uint);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn jz4740_codec_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let jz4740_codec =
        snd_soc_component_get_drvdata((*dai).component) as *mut jz4740_codec;
    let mut val: u32;

    match params_rate(params) {
        8000 => val = 0,
        11025 => val = 1,
        12000 => val = 2,
        16000 => val = 3,
        22050 => val = 4,
        24000 => val = 5,
        32000 => val = 6,
        44100 => val = 7,
        48000 => val = 8,
        _ => return -EINVAL,
    }

    val <<= JZ4740_CODEC_2_SAMPLE_RATE_OFFSET;

    regmap_update_bits(
        (*jz4740_codec).regmap,
        JZ4740_REG_CODEC_2,
        JZ4740_CODEC_2_SAMPLE_RATE_MASK,
        val,
    );

    0
}

static jz4740_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(jz4740_codec_hw_params),
};

static mut jz4740_codec_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"jz4740-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8,
    },
    ops: &jz4740_codec_dai_ops,
    symmetric_rate: 1,
};

unsafe fn jz4740_codec_wakeup(regmap: *mut regmap) {
    regmap_set_bits(regmap, JZ4740_REG_CODEC_1, JZ4740_CODEC_1_RESET);
    udelay(2);

    regmap_clear_bits(
        regmap,
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_SUSPEND | JZ4740_CODEC_1_RESET,
    );

    regcache_sync(regmap);
}

unsafe extern "C" fn jz4740_codec_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let jz4740_codec = snd_soc_component_get_drvdata(component) as *mut jz4740_codec;
    let regmap = (*jz4740_codec).regmap;
    let mask: c_uint;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            mask = JZ4740_CODEC_1_VREF_DISABLE
                | JZ4740_CODEC_1_VREF_AMP_DISABLE
                | JZ4740_CODEC_1_HEADPHONE_POWERDOWN_M;

            regmap_clear_bits(regmap, JZ4740_REG_CODEC_1, mask);
        }
        SND_SOC_BIAS_STANDBY => {
            /* The only way to clear the suspend flag is to reset the codec */
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                jz4740_codec_wakeup(regmap);
            }

            mask = JZ4740_CODEC_1_VREF_DISABLE
                | JZ4740_CODEC_1_VREF_AMP_DISABLE
                | JZ4740_CODEC_1_HEADPHONE_POWERDOWN_M;

            regmap_set_bits(regmap, JZ4740_REG_CODEC_1, mask);
        }
        SND_SOC_BIAS_OFF => {
            mask = JZ4740_CODEC_1_SUSPEND;
            regmap_set_bits(regmap, JZ4740_REG_CODEC_1, mask);
            regcache_mark_dirty(regmap);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn jz4740_codec_dev_probe(component: *mut snd_soc_component) -> c_int {
    let jz4740_codec = snd_soc_component_get_drvdata(component) as *mut jz4740_codec;

    regmap_update_bits(
        (*jz4740_codec).regmap,
        JZ4740_REG_CODEC_1,
        JZ4740_CODEC_1_SW2_ENABLE,
        JZ4740_CODEC_1_SW2_ENABLE,
    );

    0
}

static soc_codec_dev_jz4740_codec: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(jz4740_codec_dev_probe),
    set_bias_level: Some(jz4740_codec_set_bias_level),
    controls: jz4740_codec_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(jz4740_codec_controls),
    dapm_widgets: jz4740_codec_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(jz4740_codec_dapm_widgets),
    dapm_routes: jz4740_codec_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(jz4740_codec_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static jz4740_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: JZ4740_REG_CODEC_2,
    reg_defaults: jz4740_codec_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(jz4740_codec_reg_defaults),
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn jz4740_codec_probe(pdev: *mut platform_device) -> c_int {
    let ret: c_int;
    let jz4740_codec: *mut jz4740_codec;
    let base: *mut c_void;

    jz4740_codec = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<jz4740_codec>(),
        GFP_KERNEL,
    ) as *mut jz4740_codec;
    if jz4740_codec.is_null() {
        return -ENOMEM;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*jz4740_codec).regmap =
        devm_regmap_init_mmio(&mut (*pdev).dev, base, &jz4740_codec_regmap_config);
    if IS_ERR((*jz4740_codec).regmap as *const c_void) {
        return PTR_ERR((*jz4740_codec).regmap as *const c_void);
    }

    platform_set_drvdata(pdev, jz4740_codec as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_codec_dev_jz4740_codec,
        &raw mut jz4740_codec_dai,
        1,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Failed to register codec\n".as_ptr());
    }

    ret
}

static jz4740_codec_of_matches: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ingenic,jz4740-codec".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, jz4740_codec_of_matches);

static mut jz4740_codec_driver: platform_driver = platform_driver {
    probe: Some(jz4740_codec_probe),
    driver: device_driver {
        name: c"jz4740-codec".as_ptr(),
        of_match_table: jz4740_codec_of_matches.as_ptr(),
    },
};

module_platform_driver!(jz4740_codec_driver);

MODULE_DESCRIPTION!("JZ4740 SoC internal codec driver");
MODULE_AUTHOR!("Lars-Peter Clausen <lars@metafoo.de>");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("platform:jz4740-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
