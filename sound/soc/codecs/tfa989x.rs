// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Stephan Gerhold
 *
 * Register definitions/sequences taken from various tfa98xx kernel drivers:
 * Copyright (C) 2014-2020 NXP Semiconductors, All Rights Reserved.
 * Copyright (C) 2013 Sony Mobile Communications Inc.
 */

/* Depends on Linux kernel GPIO, I2C, module, regmap, regulator and ASoC APIs. */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const fn bit(n: c_uint) -> c_uint {
    1u32 << n
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const TFA989X_STATUSREG: c_uint = 0x00;
const TFA989X_BATTERYVOLTAGE: c_uint = 0x01;
const TFA989X_TEMPERATURE: c_uint = 0x02;
const TFA989X_REVISIONNUMBER: c_uint = 0x03;
const TFA989X_REVISIONNUMBER_REV_MSK: c_uint = genmask(7, 0); /* device revision */
const TFA989X_I2SREG: c_uint = 0x04;
const TFA989X_I2SREG_RCV: c_uint = 2; /* receiver mode */
const TFA989X_I2SREG_CHSA: c_uint = 6; /* amplifier input select */
const TFA989X_I2SREG_CHSA_MSK: c_uint = genmask(7, 6);
const TFA989X_I2SREG_I2SSR: c_uint = 12; /* sample rate */
const TFA989X_I2SREG_I2SSR_MSK: c_uint = genmask(15, 12);
const TFA989X_BAT_PROT: c_uint = 0x05;
const TFA989X_AUDIO_CTR: c_uint = 0x06;
const TFA989X_DCDCBOOST: c_uint = 0x07;
const TFA989X_SPKR_CALIBRATION: c_uint = 0x08;
const TFA989X_SYS_CTRL: c_uint = 0x09;
const TFA989X_SYS_CTRL_PWDN: c_uint = 0; /* power down */
const TFA989X_SYS_CTRL_I2CR: c_uint = 1; /* I2C reset */
const TFA989X_SYS_CTRL_CFE: c_uint = 2; /* enable CoolFlux DSP */
const TFA989X_SYS_CTRL_AMPE: c_uint = 3; /* enable amplifier */
const TFA989X_SYS_CTRL_DCA: c_uint = 4; /* enable boost */
const TFA989X_SYS_CTRL_SBSL: c_uint = 5; /* DSP configured */
const TFA989X_SYS_CTRL_AMPC: c_uint = 6; /* amplifier enabled by DSP */
const TFA989X_I2S_SEL_REG: c_uint = 0x0a;
const TFA989X_I2S_SEL_REG_SPKR_MSK: c_uint = genmask(10, 9); /* speaker impedance */
const TFA989X_I2S_SEL_REG_DCFG_MSK: c_uint = genmask(14, 11); /* DCDC compensation */
const TFA989X_HIDE_UNHIDE_KEY: c_uint = 0x40;
const TFA989X_PWM_CONTROL: c_uint = 0x41;
const TFA989X_CURRENTSENSE1: c_uint = 0x46;
const TFA989X_CURRENTSENSE2: c_uint = 0x47;
const TFA989X_CURRENTSENSE3: c_uint = 0x48;
const TFA989X_CURRENTSENSE4: c_uint = 0x49;

const TFA9890_REVISION: c_uint = 0x80;
const TFA9895_REVISION: c_uint = 0x12;
const TFA9897_REVISION: c_uint = 0x97;

const REGCACHE_RBTREE: c_uint = 2;
const SND_SOC_NOPM: c_int = -1;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 2;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0x0000_01ff;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
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
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 4],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct tfa989x_rev {
    rev: c_uint,
    init: Option<unsafe extern "C" fn(regmap: *mut regmap) -> c_int>,
}

#[repr(C)]
struct tfa989x {
    rev: *const tfa989x_rev,
    vddd_supply: *mut regulator,
    rcv_gpiod: *mut gpio_desc,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    writeable_reg: Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    volatile_reg: Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    cache_type: c_uint,
}

#[repr(C)]
struct soc_enum {
    reg: c_uint,
    shift_l: c_uint,
    items: c_uint,
    texts: *const *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    name: *const c_char,
    private_value: usize,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    id: c_int,
    name: *const c_char,
    reg: c_int,
    shift: c_uint,
    invert: c_uint,
    kcontrol_news: *const snd_kcontrol_new,
    num_kcontrols: c_int,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    formats: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct reg_sequence {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(i2c: *mut i2c_client) -> c_int>,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_put_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_get_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write(
        map: *mut regmap,
        regs: *const reg_sequence,
        num_regs: c_int,
    ) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(data: *mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn ptr_err<T>(ptr: *mut T) -> isize {
    ptr as isize
}

unsafe extern "C" fn tfa989x_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg > TFA989X_REVISIONNUMBER
}

unsafe extern "C" fn tfa989x_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg < TFA989X_REVISIONNUMBER
}

static tfa989x_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    writeable_reg: Some(tfa989x_writeable_reg),
    volatile_reg: Some(tfa989x_volatile_reg),
    cache_type: REGCACHE_RBTREE,
};

static CHSA_LEFT: &[u8] = b"Left\0";
static CHSA_RIGHT: &[u8] = b"Right\0";
/* "DSP" */
static chsa_text: [*const c_char; 2] = [
    CHSA_LEFT.as_ptr() as *const c_char,
    CHSA_RIGHT.as_ptr() as *const c_char,
];
static chsa_enum: soc_enum = soc_enum {
    reg: TFA989X_I2SREG,
    shift_l: TFA989X_I2SREG_CHSA,
    items: 2,
    texts: chsa_text.as_ptr(),
};
static AMP_INPUT: &[u8] = b"Amp Input\0";
static chsa_mux: snd_kcontrol_new = snd_kcontrol_new {
    name: AMP_INPUT.as_ptr() as *const c_char,
    private_value: &chsa_enum as *const soc_enum as usize,
    get: None,
    put: None,
};

static OUT: &[u8] = b"OUT\0";
static POWER: &[u8] = b"POWER\0";
static AMPE: &[u8] = b"AMPE\0";
static AIFINL: &[u8] = b"AIFINL\0";
static AIFINR: &[u8] = b"AIFINR\0";
static HIFI_PLAYBACK: &[u8] = b"HiFi Playback\0";

static tfa989x_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    snd_soc_dapm_widget {
        id: 0,
        name: OUT.as_ptr() as *const c_char,
        reg: 0,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 1,
        name: POWER.as_ptr() as *const c_char,
        reg: TFA989X_SYS_CTRL as c_int,
        shift: TFA989X_SYS_CTRL_PWDN,
        invert: 1,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 2,
        name: AMPE.as_ptr() as *const c_char,
        reg: TFA989X_SYS_CTRL as c_int,
        shift: TFA989X_SYS_CTRL_AMPE,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 3,
        name: AMP_INPUT.as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &chsa_mux,
        num_kcontrols: 1,
    },
    snd_soc_dapm_widget {
        id: 4,
        name: AIFINL.as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 4,
        name: AIFINR.as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
];

static tfa989x_dapm_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: OUT.as_ptr() as *const c_char,
        control: ptr::null(),
        source: AMPE.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: AMPE.as_ptr() as *const c_char,
        control: ptr::null(),
        source: POWER.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: AMPE.as_ptr() as *const c_char,
        control: ptr::null(),
        source: AMP_INPUT.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: AMP_INPUT.as_ptr() as *const c_char,
        control: CHSA_LEFT.as_ptr() as *const c_char,
        source: AIFINL.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: AMP_INPUT.as_ptr() as *const c_char,
        control: CHSA_RIGHT.as_ptr() as *const c_char,
        source: AIFINR.as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn tfa989x_put_mode(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tfa989x = snd_soc_component_get_drvdata(component) as *mut tfa989x;

    gpiod_set_value_cansleep(
        (*tfa989x).rcv_gpiod,
        (*ucontrol).value.enumerated.item[0] as c_int,
    );

    snd_soc_put_enum_double(kcontrol, ucontrol)
}

static MODE_SPEAKER: &[u8] = b"Speaker\0";
static MODE_RECEIVER: &[u8] = b"Receiver\0";
static mode_text: [*const c_char; 2] = [
    MODE_SPEAKER.as_ptr() as *const c_char,
    MODE_RECEIVER.as_ptr() as *const c_char,
];
static mode_enum: soc_enum = soc_enum {
    reg: TFA989X_I2SREG,
    shift_l: TFA989X_I2SREG_RCV,
    items: 2,
    texts: mode_text.as_ptr(),
};
static MODE: &[u8] = b"Mode\0";
static tfa989x_mode_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: MODE.as_ptr() as *const c_char,
    private_value: &mode_enum as *const soc_enum as usize,
    get: Some(snd_soc_get_enum_double),
    put: Some(tfa989x_put_mode),
}];

unsafe extern "C" fn tfa989x_probe(component: *mut snd_soc_component) -> c_int {
    let tfa989x = snd_soc_component_get_drvdata(component) as *mut tfa989x;

    if (*(*tfa989x).rev).rev == TFA9897_REVISION {
        return snd_soc_add_component_controls(
            component,
            tfa989x_mode_controls.as_ptr(),
            tfa989x_mode_controls.len() as c_uint,
        );
    }

    0
}

static tfa989x_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tfa989x_probe),
    dapm_widgets: tfa989x_dapm_widgets.as_ptr(),
    num_dapm_widgets: tfa989x_dapm_widgets.len() as c_uint,
    dapm_routes: tfa989x_dapm_routes.as_ptr(),
    num_dapm_routes: tfa989x_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

static tfa989x_rates: [c_uint; 9] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000];

fn tfa989x_find_sample_rate(rate: c_uint) -> c_int {
    let mut i: c_int = 0;

    while i < tfa989x_rates.len() as c_int {
        if tfa989x_rates[i as usize] == rate {
            return i;
        }
        i += 1;
    }

    -EINVAL
}

unsafe extern "C" fn tfa989x_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let sr: c_int;

    sr = tfa989x_find_sample_rate(params_rate(params));
    if sr < 0 {
        return sr;
    }

    snd_soc_component_update_bits(
        component,
        TFA989X_I2SREG,
        TFA989X_I2SREG_I2SSR_MSK,
        (sr as c_uint) << TFA989X_I2SREG_I2SSR,
    )
}

static tfa989x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tfa989x_hw_params),
};

static TFA989X_HIFI: &[u8] = b"tfa989x-hifi\0";
static mut tfa989x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: TFA989X_HIFI.as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: HIFI_PLAYBACK.as_ptr() as *const c_char,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
        rates: SNDRV_PCM_RATE_8000_48000,
        rate_min: 8000,
        rate_max: 48000,
        channels_min: 1,
        channels_max: 2,
    },
    ops: &tfa989x_dai_ops,
};

unsafe extern "C" fn tfa9890_init(regmap: *mut regmap) -> c_int {
    let mut ret: c_int;

    /* temporarily allow access to hidden registers */
    ret = regmap_write(regmap, TFA989X_HIDE_UNHIDE_KEY, 0x5a6b);
    if ret != 0 {
        return ret;
    }

    /* update PLL registers */
    ret = regmap_set_bits(regmap, 0x59, 0x3);
    if ret != 0 {
        return ret;
    }

    /* hide registers again */
    ret = regmap_write(regmap, TFA989X_HIDE_UNHIDE_KEY, 0x0000);
    if ret != 0 {
        return ret;
    }

    regmap_write(regmap, TFA989X_CURRENTSENSE2, 0x7BE1)
}

static tfa9890_rev: tfa989x_rev = tfa989x_rev {
    rev: TFA9890_REVISION,
    init: Some(tfa9890_init),
};

static tfa9895_reg_init: [reg_sequence; 6] = [
    /* some other registers must be set for optimal amplifier behaviour */
    reg_sequence {
        reg: TFA989X_BAT_PROT,
        def: 0x13ab,
    },
    reg_sequence {
        reg: TFA989X_AUDIO_CTR,
        def: 0x001f,
    },
    /* peak voltage protection is always on, but may be written */
    reg_sequence {
        reg: TFA989X_SPKR_CALIBRATION,
        def: 0x3c4e,
    },
    /* TFA989X_SYSCTRL_DCA = 0 */
    reg_sequence {
        reg: TFA989X_SYS_CTRL,
        def: 0x024d,
    },
    reg_sequence {
        reg: TFA989X_PWM_CONTROL,
        def: 0x0308,
    },
    reg_sequence {
        reg: TFA989X_CURRENTSENSE4,
        def: 0x0e82,
    },
];

unsafe extern "C" fn tfa9895_init(regmap: *mut regmap) -> c_int {
    regmap_multi_reg_write(
        regmap,
        tfa9895_reg_init.as_ptr(),
        tfa9895_reg_init.len() as c_int,
    )
}

static tfa9895_rev: tfa989x_rev = tfa989x_rev {
    rev: TFA9895_REVISION,
    init: Some(tfa9895_init),
};

unsafe extern "C" fn tfa9897_init(regmap: *mut regmap) -> c_int {
    let mut ret: c_int;

    /* Reduce slewrate by clearing iddqtestbst to avoid booster damage */
    ret = regmap_write(regmap, TFA989X_CURRENTSENSE3, 0x0300);
    if ret != 0 {
        return ret;
    }

    /* Enable clipping */
    ret = regmap_clear_bits(regmap, TFA989X_CURRENTSENSE4, 0x1);
    if ret != 0 {
        return ret;
    }

    /* Set required TDM configuration */
    regmap_write(regmap, 0x14, 0x0)
}

static tfa9897_rev: tfa989x_rev = tfa989x_rev {
    rev: TFA9897_REVISION,
    init: Some(tfa9897_init),
};

/*
 * Note: At the moment this driver bypasses the "CoolFlux DSP" built into the
 * TFA989X amplifiers. Unfortunately, there seems to be absolutely
 * no documentation for it - the public "short datasheets" do not provide
 * any information about the DSP or available registers.
 *
 * Usually the TFA989X amplifiers are configured through proprietary userspace
 * libraries. There are also some (rather complex) kernel drivers but even those
 * rely on obscure firmware blobs for configuration (so-called "containers").
 * They seem to contain different "profiles" with tuned speaker settings, sample
 * rates and volume steps (which would be better exposed as separate ALSA mixers).
 *
 * Bypassing the DSP disables volume control (and perhaps some speaker
 * optimization?), but at least allows using the speaker without obscure
 * kernel drivers and firmware.
 *
 * Ideally NXP (or now Goodix) should release proper documentation for these
 * amplifiers so that support for the "CoolFlux DSP" can be implemented properly.
 */
unsafe fn tfa989x_dsp_bypass(regmap: *mut regmap) -> c_int {
    let mut ret: c_int;

    /* Clear CHSA to bypass DSP and take input from I2S 1 left channel */
    ret = regmap_clear_bits(regmap, TFA989X_I2SREG, TFA989X_I2SREG_CHSA_MSK);
    if ret != 0 {
        return ret;
    }

    /* Set DCDC compensation to off and speaker impedance to 8 ohm */
    ret = regmap_update_bits(
        regmap,
        TFA989X_I2S_SEL_REG,
        TFA989X_I2S_SEL_REG_DCFG_MSK | TFA989X_I2S_SEL_REG_SPKR_MSK,
        TFA989X_I2S_SEL_REG_SPKR_MSK,
    );
    if ret != 0 {
        return ret;
    }

    /* Set DCDC to follower mode and disable CoolFlux DSP */
    regmap_clear_bits(
        regmap,
        TFA989X_SYS_CTRL,
        bit(TFA989X_SYS_CTRL_DCA) | bit(TFA989X_SYS_CTRL_CFE) | bit(TFA989X_SYS_CTRL_AMPC),
    )
}

unsafe extern "C" fn tfa989x_regulator_disable(data: *mut c_void) {
    let tfa989x = data as *mut tfa989x;

    regulator_disable((*tfa989x).vddd_supply);
}

unsafe extern "C" fn tfa989x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let mut rev: *const tfa989x_rev;
    let tfa989x: *mut tfa989x;
    let regmap: *mut regmap;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    rev = device_get_match_data(dev) as *const tfa989x_rev;
    if rev.is_null() {
        dev_err(dev, b"unknown device revision\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    tfa989x = devm_kzalloc(dev, size_of::<tfa989x>(), GFP_KERNEL) as *mut tfa989x;
    if tfa989x.is_null() {
        return -ENOMEM;
    }

    (*tfa989x).rev = rev;
    i2c_set_clientdata(i2c, tfa989x as *mut c_void);

    (*tfa989x).vddd_supply = devm_regulator_get(dev, b"vddd\0".as_ptr() as *const c_char);
    if is_err((*tfa989x).vddd_supply) {
        return dev_err_probe(
            dev,
            ptr_err((*tfa989x).vddd_supply),
            b"Failed to get vddd regulator\n\0".as_ptr() as *const c_char,
        );
    }

    if (*(*tfa989x).rev).rev == TFA9897_REVISION {
        (*tfa989x).rcv_gpiod =
            devm_gpiod_get_optional(dev, b"rcv\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
        if is_err((*tfa989x).rcv_gpiod) {
            return ptr_err((*tfa989x).rcv_gpiod) as c_int;
        }
    }

    regmap = devm_regmap_init_i2c(i2c, &tfa989x_regmap);
    if is_err(regmap) {
        return ptr_err(regmap) as c_int;
    }

    ret = regulator_enable((*tfa989x).vddd_supply);
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to enable vddd regulator: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = devm_add_action_or_reset(dev, Some(tfa989x_regulator_disable), tfa989x as *mut c_void);
    if ret != 0 {
        return ret;
    }

    /* Bypass regcache for reset and init sequence */
    regcache_cache_bypass(regmap, true);

    /* Dummy read to generate i2c clocks, required on some devices */
    regmap_read(regmap, TFA989X_REVISIONNUMBER, &mut val);

    ret = regmap_read(regmap, TFA989X_REVISIONNUMBER, &mut val);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to read revision number: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    val &= TFA989X_REVISIONNUMBER_REV_MSK;
    if val != (*rev).rev {
        dev_err(
            dev,
            b"invalid revision number, expected %#x, got %#x\n\0".as_ptr() as *const c_char,
            (*rev).rev,
            val,
        );
        return -ENODEV;
    }

    ret = regmap_write(regmap, TFA989X_SYS_CTRL, bit(TFA989X_SYS_CTRL_I2CR));
    if ret != 0 {
        dev_err(
            dev,
            b"failed to reset I2C registers: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = ((*rev).init.unwrap())(regmap);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to initialize registers: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = tfa989x_dsp_bypass(regmap);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to enable DSP bypass: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }
    regcache_cache_bypass(regmap, false);

    devm_snd_soc_register_component(dev, &tfa989x_component, &mut tfa989x_dai, 1)
}

static tfa989x_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: b"nxp,tfa9890\0".as_ptr() as *const c_char,
        data: &tfa9890_rev as *const tfa989x_rev as *const c_void,
    },
    of_device_id {
        compatible: b"nxp,tfa9895\0".as_ptr() as *const c_char,
        data: &tfa9895_rev as *const tfa989x_rev as *const c_void,
    },
    of_device_id {
        compatible: b"nxp,tfa9897\0".as_ptr() as *const c_char,
        data: &tfa9897_rev as *const tfa989x_rev as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, tfa989x_of_match); */

static mut tfa989x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tfa989x\0".as_ptr() as *const c_char,
        of_match_table: tfa989x_of_match.as_ptr(),
    },
    probe: Some(tfa989x_i2c_probe),
};
/* module_i2c_driver(tfa989x_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC NXP/Goodix TFA989X (TFA1) driver"); */
/* MODULE_AUTHOR("Stephan Gerhold <stephan@gerhold.net>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
