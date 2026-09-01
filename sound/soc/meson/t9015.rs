// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const fn BIT(nr: u32) -> c_uint {
    1u32 << nr
}

const BLOCK_EN: c_uint = 0x00;
const LORN_EN: c_uint = 0;
const LORP_EN: c_uint = 1;
const LOLN_EN: c_uint = 2;
const LOLP_EN: c_uint = 3;
const DACR_EN: c_uint = 4;
const DACL_EN: c_uint = 5;
const DACR_INV: c_uint = 20;
const DACL_INV: c_uint = 21;
const DACR_SRC: c_uint = 22;
const DACL_SRC: c_uint = 23;
const REFP_BUF_EN: c_uint = BIT(12);
const BIAS_CURRENT_EN: c_uint = BIT(13);
const VMID_GEN_FAST: c_uint = BIT(14);
const VMID_GEN_EN: c_uint = BIT(15);
const I2S_MODE: c_uint = BIT(30);
const VOL_CTRL0: c_uint = 0x04;
const GAIN_H: c_uint = 31;
const GAIN_L: c_uint = 23;
const VOL_CTRL1: c_uint = 0x08;
const DAC_MONO: c_uint = 8;
const RAMP_RATE: c_uint = 10;
const VC_RAMP_MODE: c_uint = 12;
const MUTE_MODE: c_uint = 13;
const UNMUTE_MODE: c_uint = 14;
const DAC_SOFT_MUTE: c_uint = 15;
const DACR_VC: c_uint = 16;
const DACL_VC: c_uint = 24;
const LINEOUT_CFG: c_uint = 0x0c;
const LORN_POL: c_uint = 0;
const LORP_POL: c_uint = 4;
const LOLN_POL: c_uint = 8;
const LOLP_POL: c_uint = 12;
const POWER_CFG: c_uint = 0x10;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_POSSIBLE_DAIFMT_I2S: c_ulonglong = 0;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: c_ulonglong = 0;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: c_ulonglong = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulonglong = 0;
const SNDRV_PCM_FMTBIT_S20_LE: c_ulonglong = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulonglong = 0;
const SND_SOC_NOPM: c_int = 0;

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulonglong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct t9015 {
    pub avdd: *mut regulator,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

unsafe extern "C" {
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn mdelay(msecs: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn device_reset(dev: *mut device) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id;
}

unsafe extern "C" fn t9015_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = unsafe { (*dai).component };
    let val: c_uint;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            val = I2S_MODE;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            val = 0;
        }
        _ => {
            return -EINVAL;
        }
    }

    unsafe {
        snd_soc_component_update_bits(component, BLOCK_EN, I2S_MODE, val);
    }

    if ((fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S)
        && ((fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_LEFT_J)
    {
        return -EINVAL;
    }

    0
}

static t9015_dai_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_LEFT_J;

static t9015_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(t9015_dai_set_fmt),
    auto_selectable_formats: &t9015_dai_selectable_formats,
    num_auto_selectable_formats: 1,
};

static mut t9015_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"t9015-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_LE
            | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &t9015_dai_ops,
};

// static const DECLARE_TLV_DB_MINMAX_MUTE(dac_vol_tlv, -9525, 0);
static dac_vol_tlv: [c_uint; 0] = [];

static ramp_rate_txt: [*const c_char; 2] = [
    b"Fast\0".as_ptr() as *const c_char,
    b"Slow\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(ramp_rate_enum, VOL_CTRL1, RAMP_RATE, ramp_rate_txt);

static dacr_in_txt: [*const c_char; 2] = [
    b"Right\0".as_ptr() as *const c_char,
    b"Left\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(dacr_in_enum, BLOCK_EN, DACR_SRC, dacr_in_txt);

static dacl_in_txt: [*const c_char; 2] = [
    b"Left\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(dacl_in_enum, BLOCK_EN, DACL_SRC, dacl_in_txt);

static mono_txt: [*const c_char; 2] = [
    b"Stereo\0".as_ptr() as *const c_char,
    b"Mono\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(mono_enum, VOL_CTRL1, DAC_MONO, mono_txt);

// The following ALSA control and DAPM initializers are produced by kernel macros
// in C. They are preserved as dependency-shaped declarations for the translated
// file because their concrete struct layout is supplied by external headers.
static t9015_snd_controls: [snd_kcontrol_new; 7] = unsafe { core::mem::zeroed() };
static t9015_right_dac_mux: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static t9015_left_dac_mux: snd_kcontrol_new = unsafe { core::mem::zeroed() };
static t9015_dapm_widgets: [snd_soc_dapm_widget; 14] = unsafe { core::mem::zeroed() };

static t9015_dapm_routes: [snd_soc_dapm_route; 16] = [
    snd_soc_dapm_route { sink: b"Right IN\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left IN\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right DAC Sel\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"Right IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right DAC Sel\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"Left IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left DAC Sel\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"Right IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left DAC Sel\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"Left IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right DAC Sel\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left DAC Sel\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right- Driver\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right+ Driver\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left- Driver\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left+ Driver\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LORN\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right- Driver\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LORP\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right+ Driver\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LOLN\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left- Driver\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LOLP\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left+ Driver\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn t9015_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut t9015 };
    let dapm = unsafe { snd_soc_component_to_dapm(component) };
    let now = unsafe { snd_soc_dapm_get_bias_level(dapm) };
    let ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            unsafe {
                snd_soc_component_update_bits(component, BLOCK_EN, BIAS_CURRENT_EN, BIAS_CURRENT_EN);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            unsafe {
                snd_soc_component_update_bits(component, BLOCK_EN, BIAS_CURRENT_EN, 0);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            ret = unsafe { regulator_enable((*priv_).avdd) };
            if ret != 0 {
                unsafe {
                    dev_err((*component).dev, b"AVDD enable failed\n\0".as_ptr() as *const c_char);
                }
                return ret;
            }

            if now == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                unsafe {
                    snd_soc_component_update_bits(
                        component,
                        BLOCK_EN,
                        VMID_GEN_EN | VMID_GEN_FAST | REFP_BUF_EN,
                        VMID_GEN_EN | VMID_GEN_FAST | REFP_BUF_EN,
                    );

                    mdelay(200);
                    snd_soc_component_update_bits(component, BLOCK_EN, VMID_GEN_FAST, 0);
                }
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            unsafe {
                snd_soc_component_update_bits(
                    component,
                    BLOCK_EN,
                    VMID_GEN_EN | VMID_GEN_FAST | REFP_BUF_EN,
                    0,
                );

                regulator_disable((*priv_).avdd);
            }
        }
    }

    0
}

static t9015_codec_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(t9015_set_bias_level),
    controls: t9015_snd_controls.as_ptr(),
    num_controls: t9015_snd_controls.len() as c_uint,
    dapm_widgets: t9015_dapm_widgets.as_ptr(),
    num_dapm_widgets: t9015_dapm_widgets.len() as c_uint,
    dapm_routes: t9015_dapm_routes.as_ptr(),
    num_dapm_routes: t9015_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    endianness: 1,
};

static t9015_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: POWER_CFG,
};

unsafe extern "C" fn t9015_probe(pdev: *mut platform_device) -> c_int {
    let dev = unsafe { &mut (*pdev).dev as *mut device };
    let priv_: *mut t9015;
    let regs: *mut c_void;
    let regmap: *mut regmap;
    let pclk: *mut clk;
    let ret: c_int;

    priv_ = unsafe { devm_kzalloc(dev, core::mem::size_of::<t9015>(), GFP_KERNEL) as *mut t9015 };
    if priv_.is_null() {
        return -ENOMEM;
    }
    unsafe {
        platform_set_drvdata(pdev, priv_ as *mut c_void);
    }

    pclk = unsafe { devm_clk_get_enabled(dev, b"pclk\0".as_ptr() as *const c_char) };
    if unsafe { IS_ERR(pclk as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR(pclk as *const c_void),
                b"failed to get core clock\n\0".as_ptr() as *const c_char,
            )
        };
    }

    unsafe {
        (*priv_).avdd = devm_regulator_get(dev, b"AVDD\0".as_ptr() as *const c_char);
    }
    if unsafe { IS_ERR((*priv_).avdd as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*priv_).avdd as *const c_void),
                b"failed to AVDD\n\0".as_ptr() as *const c_char,
            )
        };
    }

    ret = unsafe { device_reset(dev) };
    if ret != 0 {
        return unsafe {
            dev_err_probe(
                dev,
                ret,
                b"failed to reset device\n\0".as_ptr() as *const c_char,
            )
        };
    }

    regs = unsafe { devm_platform_ioremap_resource(pdev, 0) };
    if unsafe { IS_ERR(regs as *const c_void) } {
        unsafe {
            dev_err(dev, b"register map failed\n\0".as_ptr() as *const c_char);
        }
        return unsafe { PTR_ERR(regs as *const c_void) };
    }

    regmap = unsafe { devm_regmap_init_mmio(dev, regs, &t9015_regmap_config) };
    if unsafe { IS_ERR(regmap as *const c_void) } {
        unsafe {
            dev_err(dev, b"regmap init failed\n\0".as_ptr() as *const c_char);
        }
        return unsafe { PTR_ERR(regmap as *const c_void) };
    }

    /*
     * Initialize output polarity:
     * ATM the output polarity is fixed but in the future it might useful
     * to add DT property to set this depending on the platform needs
     */
    unsafe {
        regmap_write(regmap, LINEOUT_CFG, 0x1111);

        devm_snd_soc_register_component(dev, &t9015_codec_driver, &mut t9015_dai, 1)
    }
}

static t9015_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"amlogic,t9015\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, t9015_ids);

static mut t9015_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"t9015-codec\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(t9015_ids.as_ptr()) },
    },
    probe: Some(t9015_probe),
};

// module_platform_driver(t9015_driver);
// MODULE_DESCRIPTION("ASoC Amlogic T9015 codec driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
