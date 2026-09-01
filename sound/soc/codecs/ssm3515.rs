// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Analog Devices' SSM3515 audio amp driver
//
// Copyright (C) The Asahi Linux Contributors

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type bool_ = bool;
type u8 = core::ffi::c_uchar;
type c_char = core::ffi::c_char;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;
type size_t = usize;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn __bf_shf(mask: c_uint) -> c_uint {
    mask.trailing_zeros()
}

const fn FIELD_PREP(mask: c_uint, val: c_int) -> c_uint {
    ((val as c_uint) << __bf_shf(mask)) & mask
}

const fn FIELD_GET(mask: c_uint, reg: c_int) -> c_uint {
    ((reg as c_uint) & mask) >> __bf_shf(mask)
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const SSM3515_PWR: c_uint = 0x00;
const SSM3515_PWR_APWDN_EN: c_uint = BIT(7);
const SSM3515_PWR_BSNS_PWDN: c_uint = BIT(6);
const SSM3515_PWR_S_RST: c_uint = BIT(1);
const SSM3515_PWR_SPWDN: c_uint = BIT(0);

const SSM3515_GEC: c_uint = 0x01;
const SSM3515_GEC_EDGE: c_uint = BIT(4);
const SSM3515_GEC_EDGE_SHIFT: c_uint = 4;
const SSM3515_GEC_ANA_GAIN: c_uint = GENMASK(1, 0);

const SSM3515_DAC: c_uint = 0x02;
const SSM3515_DAC_HV: c_uint = BIT(7);
const SSM3515_DAC_MUTE: c_uint = BIT(6);
const SSM3515_DAC_HPF: c_uint = BIT(5);
const SSM3515_DAC_LPM: c_uint = BIT(4);
const SSM3515_DAC_FS: c_uint = GENMASK(2, 0);

const SSM3515_DAC_VOL: c_uint = 0x03;

const SSM3515_SAI1: c_uint = 0x04;
const SSM3515_SAI1_DAC_POL: c_uint = BIT(7);
const SSM3515_SAI1_BCLK_POL: c_uint = BIT(6);
const SSM3515_SAI1_TDM_BCLKS: c_uint = GENMASK(5, 3);
const SSM3515_SAI1_FSYNC_MODE: c_uint = BIT(2);
const SSM3515_SAI1_SDATA_FMT: c_uint = BIT(1);
const SSM3515_SAI1_SAI_MODE: c_uint = BIT(0);

const SSM3515_SAI2: c_uint = 0x05;
const SSM3515_SAI2_DATA_WIDTH: c_uint = BIT(7);
const SSM3515_SAI2_AUTO_SLOT: c_uint = BIT(4);
const SSM3515_SAI2_TDM_SLOT: c_uint = GENMASK(3, 0);

const SSM3515_VBAT_OUT: c_uint = 0x06;

const SSM3515_STATUS: c_uint = 0x0a;
const SSM3515_STATUS_UVLO_REG: c_uint = BIT(6);
const SSM3515_STATUS_LIM_EG: c_uint = BIT(5);
const SSM3515_STATUS_CLIP: c_uint = BIT(4);
const SSM3515_STATUS_AMP_OC: c_uint = BIT(3);
const SSM3515_STATUS_OTF: c_uint = BIT(2);
const SSM3515_STATUS_OTW: c_uint = BIT(1);
const SSM3515_STATUS_BAT_WARN: c_uint = BIT(0);

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 0;
const SNDRV_PCM_FORMAT_S16: c_uint = 0;
const SNDRV_PCM_FORMAT_S24: c_uint = 1;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    cache_type: c_uint,
}

#[repr(C)]
struct ssm3515_data {
    dev: *mut device,
    regmap: *mut regmap,
}

#[repr(C)]
struct soc_enum {
    reg: c_uint,
    shift_l: c_uint,
    shift_r: c_uint,
    items: c_uint,
    texts: *const *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [usize; 8],
}

#[repr(C)]
struct snd_soc_dai_ops {
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [usize; 8],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct driver_private {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct i2c_driver {
    driver: driver_private,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

unsafe extern "C" {
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut core::ffi::c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut core::ffi::c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool_;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_reinit_cache(regmap: *mut regmap, config: *const regmap_config);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn ssm3515_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SSM3515_STATUS | SSM3515_VBAT_OUT => true,
        _ => false,
    }
}

static ssm3515_reg_defaults: [reg_default; 6] = [
    reg_default {
        reg: SSM3515_PWR,
        def: 0x81,
    },
    reg_default {
        reg: SSM3515_GEC,
        def: 0x01,
    },
    reg_default {
        reg: SSM3515_DAC,
        def: 0x32,
    },
    reg_default {
        reg: SSM3515_DAC_VOL,
        def: 0x40,
    },
    reg_default {
        reg: SSM3515_SAI1,
        def: 0x11,
    },
    reg_default {
        reg: SSM3515_SAI2,
        def: 0x00,
    },
];

static ssm3515_i2c_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    volatile_reg: Some(ssm3515_volatile_reg),
    max_register: 0xb,
    reg_defaults: ssm3515_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&ssm3515_reg_defaults) as c_uint,
    cache_type: REGCACHE_FLAT,
};

// The specced range is -71.25...24.00 dB with step size of 0.375 dB,
// and a mute item below that. This is represented by -71.62...24.00 dB
// with the mute item mapped onto the low end.
static ssm3515_dac_volume: [c_int; 2] = [-7162, 2400];

static ssm3515_ana_gain_text_0: &[u8] = b"8.4 V Span\0";
static ssm3515_ana_gain_text_1: &[u8] = b"12.6 V Span\0";
static ssm3515_ana_gain_text_2: &[u8] = b"14 V Span\0";
static ssm3515_ana_gain_text_3: &[u8] = b"15 V Span\0";

static ssm3515_ana_gain_text: [*const c_char; 4] = [
    ssm3515_ana_gain_text_0.as_ptr() as *const c_char,
    ssm3515_ana_gain_text_1.as_ptr() as *const c_char,
    ssm3515_ana_gain_text_2.as_ptr() as *const c_char,
    ssm3515_ana_gain_text_3.as_ptr() as *const c_char,
];

static ssm3515_ana_gain_enum: soc_enum = soc_enum {
    reg: SSM3515_GEC,
    shift_l: __bf_shf(SSM3515_GEC_ANA_GAIN),
    shift_r: __bf_shf(SSM3515_GEC_ANA_GAIN),
    items: ARRAY_SIZE(&ssm3515_ana_gain_text) as c_uint,
    texts: ssm3515_ana_gain_text.as_ptr(),
};

// Translated from SOC_SINGLE_TLV/SOC_SINGLE/SOC_ENUM control declarations.
static ssm3515_snd_controls: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new { _private: [0; 8] },
    snd_kcontrol_new { _private: [0; 8] },
    snd_kcontrol_new { _private: [0; 8] },
    snd_kcontrol_new { _private: [0; 8] },
    snd_kcontrol_new { _private: [0; 8] },
    snd_kcontrol_new { _private: [0; 8] },
];

unsafe extern "C" fn ssm3515_read_faults(component: *mut snd_soc_component) {
    let ret: c_int;

    ret = snd_soc_component_read(component, SSM3515_STATUS);
    if ret <= 0 {
        /*
         * If the read was erroneous, ASoC core has printed a message,
         * and that's all that's appropriate in handling the error here.
         */
        return;
    }

    dev_err(
        (*component).dev,
        b"device reports:%s%s%s%s%s%s%s\n\0".as_ptr() as *const c_char,
        if FIELD_GET(SSM3515_STATUS_UVLO_REG, ret) != 0 {
            b" voltage regulator fault\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        if FIELD_GET(SSM3515_STATUS_LIM_EG, ret) != 0 {
            b" limiter engaged\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        if FIELD_GET(SSM3515_STATUS_CLIP, ret) != 0 {
            b" clipping detected\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        if FIELD_GET(SSM3515_STATUS_AMP_OC, ret) != 0 {
            b" amp over-current fault\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        if FIELD_GET(SSM3515_STATUS_OTF, ret) != 0 {
            b" overtemperature fault\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        if FIELD_GET(SSM3515_STATUS_OTW, ret) != 0 {
            b" overtemperature warning\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        if FIELD_GET(SSM3515_STATUS_BAT_WARN, ret) != 0 {
            b" bat voltage low warning\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
    );
}

unsafe extern "C" fn ssm3515_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;

    /* Start out muted */
    ret = snd_soc_component_update_bits(
        component,
        SSM3515_DAC,
        SSM3515_DAC_MUTE,
        SSM3515_DAC_MUTE,
    );
    if ret < 0 {
        return ret;
    }

    /* Disable the 'master power-down' */
    ret = snd_soc_component_update_bits(component, SSM3515_PWR, SSM3515_PWR_SPWDN, 0);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn ssm3515_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let ret: c_int;

    ret = snd_soc_component_update_bits(
        (*dai).component,
        SSM3515_DAC,
        SSM3515_DAC_MUTE,
        FIELD_PREP(SSM3515_DAC_MUTE, mute),
    );
    if ret < 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn ssm3515_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut ret: c_int;
    let rateval: c_int;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16 | SNDRV_PCM_FORMAT_S24 => {
            ret = snd_soc_component_update_bits(
                component,
                SSM3515_SAI2,
                SSM3515_SAI2_DATA_WIDTH,
                FIELD_PREP(SSM3515_SAI2_DATA_WIDTH, (params_width(params) == 16) as c_int),
            );
            if ret < 0 {
                return ret;
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    match params_rate(params) {
        8000..=12000 => {
            rateval = 0;
        }
        16000..=24000 => {
            rateval = 1;
        }
        32000..=48000 => {
            rateval = 2;
        }
        64000..=96000 => {
            rateval = 3;
        }
        128000..=192000 => {
            rateval = 4;
        }
        48001..=63999 => {
            /* this is ...72000 but overlaps */
            rateval = 5;
        }
        _ => {
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        SSM3515_DAC,
        SSM3515_DAC_FS,
        FIELD_PREP(SSM3515_DAC_FS, rateval),
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn ssm3515_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut fpol_inv: bool_ = false; /* non-inverted: frame starts with low-to-high FSYNC */
    let ret: c_int;
    let mut sai1: u8 = 0;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_IB_IF => {
            sai1 |= SSM3515_SAI1_BCLK_POL as u8;
        }
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            fpol_inv = true;
            sai1 &= !(SSM3515_SAI1_SDATA_FMT as u8); /* 1 bit start delay */
        }
        SND_SOC_DAIFMT_LEFT_J => {
            fpol_inv = false;
            sai1 |= SSM3515_SAI1_SDATA_FMT as u8; /* no start delay */
        }
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_IF | SND_SOC_DAIFMT_IB_IF => {
            fpol_inv ^= true;
        }
        _ => {}
    }

    /* Set the serial input to 'TDM mode' */
    sai1 |= SSM3515_SAI1_SAI_MODE as u8;

    if fpol_inv {
        /*
         * We configure the codec in a 'TDM mode', in which the
         * FSYNC_MODE bit of SAI1 is supposed to select between
         * what the datasheet calls 'Pulsed FSYNC mode' and '50%
         * FSYNC mode'.
         *
         * Experiments suggest that this bit in fact simply selects
         * the FSYNC polarity, so go with that.
         */
        sai1 |= SSM3515_SAI1_FSYNC_MODE as u8;
    }

    ret = snd_soc_component_update_bits(
        component,
        SSM3515_SAI1,
        SSM3515_SAI1_BCLK_POL
            | SSM3515_SAI1_SDATA_FMT
            | SSM3515_SAI1_SAI_MODE
            | SSM3515_SAI1_FSYNC_MODE,
        sai1 as c_uint,
    );

    if ret < 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn ssm3515_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let slot: c_int;
    let tdm_bclks_val: c_int;
    let mut ret: c_int;

    if tx_mask == 0 || rx_mask != 0 {
        return -EINVAL;
    }

    slot = tx_mask.trailing_zeros() as c_int;

    if tx_mask & !BIT(slot as c_uint) != 0 {
        return -EINVAL;
    }

    match slot_width {
        16 => {
            tdm_bclks_val = 0;
        }
        24 => {
            tdm_bclks_val = 1;
        }
        32 => {
            tdm_bclks_val = 2;
        }
        48 => {
            tdm_bclks_val = 3;
        }
        64 => {
            tdm_bclks_val = 4;
        }
        _ => {
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        SSM3515_SAI1,
        SSM3515_SAI1_TDM_BCLKS,
        FIELD_PREP(SSM3515_SAI1_TDM_BCLKS, tdm_bclks_val),
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        SSM3515_SAI2,
        SSM3515_SAI2_TDM_SLOT,
        FIELD_PREP(SSM3515_SAI2_TDM_SLOT, slot),
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn ssm3515_hw_free(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    /*
     * We don't get live notification of faults, so at least at
     * this time, when playback is over, check if we have tripped
     * over anything and if so, log it.
     */
    ssm3515_read_faults((*dai).component);
    0
}

static ssm3515_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(ssm3515_mute),
    hw_params: Some(ssm3515_hw_params),
    set_fmt: Some(ssm3515_set_fmt),
    set_tdm_slot: Some(ssm3515_set_tdm_slot),
    hw_free: Some(ssm3515_hw_free),
};

static mut ssm3515_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"SSM3515 SAI\0".as_ptr() as *const c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &ssm3515_dai_ops,
};

static ssm3515_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { _private: [0; 8] },
    snd_soc_dapm_widget { _private: [0; 8] },
];

static ssm3515_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"OUT\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DAC\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Playback\0".as_ptr() as *const c_char,
    },
];

static ssm3515_asoc_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ssm3515_probe),
    controls: ssm3515_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&ssm3515_snd_controls) as c_uint,
    dapm_widgets: ssm3515_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&ssm3515_dapm_widgets) as c_uint,
    dapm_routes: ssm3515_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&ssm3515_dapm_routes) as c_uint,
    endianness: 1,
};

unsafe extern "C" fn ssm3515_i2c_probe(client: *mut i2c_client) -> c_int {
    let data: *mut ssm3515_data;
    let mut ret: c_int;

    data = devm_kzalloc(
        &mut (*client).dev,
        core::mem::size_of::<ssm3515_data>(),
        GFP_KERNEL,
    ) as *mut ssm3515_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).dev = &mut (*client).dev;
    i2c_set_clientdata(client, data as *mut core::ffi::c_void);

    (*data).regmap = devm_regmap_init_i2c(client, &ssm3515_i2c_regmap);
    if IS_ERR((*data).regmap as *const core::ffi::c_void) {
        return dev_err_probe(
            (*data).dev,
            PTR_ERR((*data).regmap as *const core::ffi::c_void),
            b"initializing register map\n\0".as_ptr() as *const c_char,
        );
    }

    /* Perform a reset */
    ret = regmap_update_bits(
        (*data).regmap,
        SSM3515_PWR,
        SSM3515_PWR_S_RST,
        SSM3515_PWR_S_RST,
    );
    if ret < 0 {
        return dev_err_probe(
            (*data).dev,
            ret,
            b"performing software reset\n\0".as_ptr() as *const c_char,
        );
    }
    regmap_reinit_cache((*data).regmap, &ssm3515_i2c_regmap);

    devm_snd_soc_register_component(
        (*data).dev,
        &ssm3515_asoc_component,
        &raw mut ssm3515_dai_driver,
        1,
    )
}

static ssm3515_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"adi,ssm3515\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, ssm3515_of_match);

static mut ssm3515_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_private {
        name: b"ssm3515\0".as_ptr() as *const c_char,
        of_match_table: ssm3515_of_match.as_ptr(),
    },
    probe: Some(ssm3515_i2c_probe),
};
// module_i2c_driver(ssm3515_i2c_driver);

// MODULE_AUTHOR("Martin Povišer <povik+lin@cutebit.org>");
// MODULE_DESCRIPTION("ASoC SSM3515 audio amp driver");
// MODULE_LICENSE("Dual MIT/GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
