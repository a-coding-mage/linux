// SPDX-License-Identifier: GPL-2.0
/*
 * ASoC codec driver for spear platform
 *
 * sound/soc/codecs/sta529.c -- spear ALSA Soc codec driver
 *
 * Copyright (C) 2012 ST Microelectronics
 * Rajeev Kumar <rajeevkumar.linux@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Linux, ALSA, ASoC, regmap, I2C and module dependencies are provided externally. */

/* STA529 Register offsets */
const STA529_FFXCFG0: c_uint = 0x00;
const STA529_FFXCFG1: c_uint = 0x01;
const STA529_MVOL: c_uint = 0x02;
const STA529_LVOL: c_uint = 0x03;
const STA529_RVOL: c_uint = 0x04;
const STA529_TTF0: c_uint = 0x05;
const STA529_TTF1: c_uint = 0x06;
const STA529_TTP0: c_uint = 0x07;
const STA529_TTP1: c_uint = 0x08;
const STA529_S2PCFG0: c_uint = 0x0A;
const STA529_S2PCFG1: c_uint = 0x0B;
const STA529_P2SCFG0: c_uint = 0x0C;
const STA529_P2SCFG1: c_uint = 0x0D;
const STA529_PLLCFG0: c_uint = 0x14;
const STA529_PLLCFG1: c_uint = 0x15;
const STA529_PLLCFG2: c_uint = 0x16;
const STA529_PLLCFG3: c_uint = 0x17;
const STA529_PLLPFE: c_uint = 0x18;
const STA529_PLLST: c_uint = 0x19;
const STA529_ADCCFG: c_uint = 0x1E; /*mic_select*/
const STA529_CKOCFG: c_uint = 0x1F;
const STA529_MISC: c_uint = 0x20;
const STA529_PADST0: c_uint = 0x21;
const STA529_PADST1: c_uint = 0x22;
const STA529_FFXST: c_uint = 0x23;
const STA529_PWMIN1: c_uint = 0x2D;
const STA529_PWMIN2: c_uint = 0x2E;
const STA529_POWST: c_uint = 0x32;

const STA529_MAX_REGISTER: c_uint = 0x32;

const STA529_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;

const STA529_FORMAT: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
const S2PC_VALUE: c_uint = 0x98;
const CLOCK_OUT: c_uint = 0x60;
const DATA_FORMAT_MSK: c_uint = 0x0E;
const LEFT_J_DATA_FORMAT: u8 = 0x00;
const I2S_DATA_FORMAT: u8 = 0x02;
const RIGHT_J_DATA_FORMAT: u8 = 0x04;
const CODEC_MUTE_VAL: u8 = 0x80;

const POWER_CNTLMSAK: c_uint = 0x40;
const POWER_STDBY: c_uint = 0x40;
const FFX_MASK: c_uint = 0x80;
const FFX_OFF: c_uint = 0x80;
const POWER_UP: c_uint = 0x00;
const FFX_CLK_ENB: c_uint = 0x01;
const FFX_CLK_DIS: c_uint = 0x00;
const FFX_CLK_MSK: c_uint = 0x01;
const PLAY_FREQ_RANGE_MSK: c_uint = 0x70;
const CAP_FREQ_RANGE_MSK: c_uint = 0x0C;
const PDATA_LEN_MSK: c_uint = 0xC0;
const BCLK_TO_FS_MSK: c_uint = 0x30;
const AUDIO_MUTE_MSK: c_uint = 0x80;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: u32 = 0x000f;
const SND_SOC_DAIFMT_LEFT_J: u32 = 1;
const SND_SOC_DAIFMT_I2S: u32 = 2;
const SND_SOC_DAIFMT_RIGHT_J: u32 = 3;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_11025: c_uint = 1 << 1;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 3;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 5;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 6;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 2;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
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
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
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
pub struct sta529 {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static STA529_REG_DEFAULTS: [reg_default; 12] = [
    reg_default { reg: 0, def: 0x35 },  /* R0   - FFX Configuration reg 0 */
    reg_default { reg: 1, def: 0xc8 },  /* R1   - FFX Configuration reg 1 */
    reg_default { reg: 2, def: 0x50 },  /* R2   - Master Volume */
    reg_default { reg: 3, def: 0x00 },  /* R3   - Left Volume */
    reg_default { reg: 4, def: 0x00 },  /* R4  -  Right Volume */
    reg_default { reg: 10, def: 0xb2 }, /* R10  - S2P Config Reg 0 */
    reg_default { reg: 11, def: 0x41 }, /* R11  - S2P Config Reg 1 */
    reg_default { reg: 12, def: 0x92 }, /* R12  - P2S Config Reg 0 */
    reg_default { reg: 13, def: 0x41 }, /* R13  - P2S Config Reg 1 */
    reg_default { reg: 30, def: 0xd2 }, /* R30  - ADC Config Reg */
    reg_default { reg: 31, def: 0x40 }, /* R31  - clock Out Reg */
    reg_default { reg: 32, def: 0x21 }, /* R32  - Misc Register */
];

unsafe extern "C" fn sta529_readable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        STA529_FFXCFG0
        | STA529_FFXCFG1
        | STA529_MVOL
        | STA529_LVOL
        | STA529_RVOL
        | STA529_S2PCFG0
        | STA529_S2PCFG1
        | STA529_P2SCFG0
        | STA529_P2SCFG1
        | STA529_ADCCFG
        | STA529_CKOCFG
        | STA529_MISC => true,
        _ => false,
    }
}

static PWM_MODE_TEXT: [*const c_char; 4] = [
    b"Binary\0".as_ptr() as *const c_char,
    b"Headphone\0".as_ptr() as *const c_char,
    b"Ternary\0".as_ptr() as *const c_char,
    b"Phase-shift\0".as_ptr() as *const c_char,
];

/* DECLARE_TLV_DB_SCALE(out_gain_tlv, -9150, 50, 0) */
static OUT_GAIN_TLV: [c_uint; 4] = [0, 2, (-9150i32) as c_uint, 50];
/* DECLARE_TLV_DB_SCALE(master_vol_tlv, -12750, 50, 0) */
static MASTER_VOL_TLV: [c_uint; 4] = [0, 2, (-12750i32) as c_uint, 50];
/* SOC_ENUM_SINGLE_DECL(pwm_src, STA529_FFXCFG1, 4, pwm_mode_text) */
static PWM_SRC: soc_enum = soc_enum { _private: [] };

/* The SOC_* control macro expansions are supplied by the ASoC dependency. */
static STA529_SND_CONTROLS: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn sta529_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let sta529 = snd_soc_component_get_drvdata(component) as *mut sta529;
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON | snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, STA529_FFXCFG0, POWER_CNTLMSAK, POWER_UP);
            snd_soc_component_update_bits(component, STA529_MISC, FFX_CLK_MSK, FFX_CLK_ENB);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) as c_uint
                == snd_soc_bias_level::SND_SOC_BIAS_OFF as c_uint
            {
                regcache_sync((*sta529).regmap);
            }
            snd_soc_component_update_bits(component, STA529_FFXCFG0, POWER_CNTLMSAK, POWER_STDBY);
            /* Making FFX output to zero */
            snd_soc_component_update_bits(component, STA529_FFXCFG0, FFX_MASK, FFX_OFF);
            snd_soc_component_update_bits(component, STA529_MISC, FFX_CLK_MSK, FFX_CLK_DIS);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {}
    }

    0
}

unsafe extern "C" fn sta529_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let pdata: c_int;
    let play_freq_val: c_int;
    let record_freq_val: c_int;
    let bclk_to_fs_ratio: c_int;

    match params_width(params) {
        16 => {
            pdata = 1;
            bclk_to_fs_ratio = 0;
        }
        24 => {
            pdata = 2;
            bclk_to_fs_ratio = 1;
        }
        32 => {
            pdata = 3;
            bclk_to_fs_ratio = 2;
        }
        _ => {
            dev_err((*component).dev, b"Unsupported format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    match params_rate(params) {
        8000 | 11025 => {
            play_freq_val = 0;
            record_freq_val = 2;
        }
        16000 | 22050 => {
            play_freq_val = 1;
            record_freq_val = 0;
        }
        32000 | 44100 | 48000 => {
            play_freq_val = 2;
            record_freq_val = 0;
        }
        _ => {
            dev_err((*component).dev, b"Unsupported rate\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_component_update_bits(
            component,
            STA529_S2PCFG1,
            PDATA_LEN_MSK,
            (pdata << 6) as c_uint,
        );
        snd_soc_component_update_bits(
            component,
            STA529_S2PCFG1,
            BCLK_TO_FS_MSK,
            (bclk_to_fs_ratio << 4) as c_uint,
        );
        snd_soc_component_update_bits(
            component,
            STA529_MISC,
            PLAY_FREQ_RANGE_MSK,
            (play_freq_val << 4) as c_uint,
        );
    } else {
        snd_soc_component_update_bits(
            component,
            STA529_P2SCFG1,
            PDATA_LEN_MSK,
            (pdata << 6) as c_uint,
        );
        snd_soc_component_update_bits(
            component,
            STA529_P2SCFG1,
            BCLK_TO_FS_MSK,
            (bclk_to_fs_ratio << 4) as c_uint,
        );
        snd_soc_component_update_bits(
            component,
            STA529_MISC,
            CAP_FREQ_RANGE_MSK,
            (record_freq_val << 2) as c_uint,
        );
    }

    0
}

unsafe extern "C" fn sta529_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let mut val: u8 = 0;

    if mute != 0 {
        val |= CODEC_MUTE_VAL;
    }

    snd_soc_component_update_bits((*dai).component, STA529_FFXCFG0, AUDIO_MUTE_MSK, val as c_uint);

    0
}

unsafe extern "C" fn sta529_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: u32) -> c_int {
    let component = (*codec_dai).component;
    let mode: u8;

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_LEFT_J => {
            mode = LEFT_J_DATA_FORMAT;
        }
        SND_SOC_DAIFMT_I2S => {
            mode = I2S_DATA_FORMAT;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            mode = RIGHT_J_DATA_FORMAT;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, STA529_S2PCFG0, DATA_FORMAT_MSK, mode as c_uint);

    0
}

static STA529_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(sta529_hw_params),
    set_fmt: Some(sta529_set_dai_fmt),
    mute_stream: Some(sta529_mute),
    no_capture_mute: 1,
};

static mut STA529_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"sta529-audio\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: STA529_RATES,
        formats: STA529_FORMAT,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: STA529_RATES,
        formats: STA529_FORMAT,
    },
    ops: &STA529_DAI_OPS,
};

static STA529_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(sta529_set_bias_level),
    controls: STA529_SND_CONTROLS.as_ptr(),
    num_controls: STA529_SND_CONTROLS.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static STA529_REGMAP: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: STA529_MAX_REGISTER,
    readable_reg: Some(sta529_readable),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: STA529_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: STA529_REG_DEFAULTS.len() as c_uint,
};

unsafe extern "C" fn sta529_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let sta529: *mut sta529;
    let mut ret: c_int;

    sta529 = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<sta529>(),
        GFP_KERNEL,
    ) as *mut sta529;
    if sta529.is_null() {
        return -ENOMEM;
    }

    (*sta529).regmap = devm_regmap_init_i2c(i2c, &STA529_REGMAP);
    if IS_ERR((*sta529).regmap as *const c_void) {
        ret = PTR_ERR((*sta529).regmap as *const c_void);
        dev_err(
            &mut (*i2c).dev,
            b"Failed to allocate regmap: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    i2c_set_clientdata(i2c, sta529 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &STA529_COMPONENT_DRIVER,
        &raw mut STA529_DAI,
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to register CODEC: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

static STA529_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b's' as c_char,
            b't' as c_char,
            b'a' as c_char,
            b'5' as c_char,
            b'2' as c_char,
            b'9' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, sta529_i2c_id); */

static STA529_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"st,sta529\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, sta529_of_match); */

static STA529_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: i2c_driver_inner {
        name: b"sta529\0".as_ptr() as *const c_char,
        of_match_table: STA529_OF_MATCH.as_ptr(),
    },
    probe: Some(sta529_i2c_probe),
    id_table: STA529_I2C_ID.as_ptr(),
};

/* module_i2c_driver(sta529_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC STA529 codec driver"); */
/* MODULE_AUTHOR("Rajeev Kumar <rajeevkumar.linux@gmail.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
