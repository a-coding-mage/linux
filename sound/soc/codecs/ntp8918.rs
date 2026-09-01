// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the NTP8918 Audio Amplifier
 *
 * Copyright (c) 2024, SaluteDevices. All Rights Reserved.
 *
 * Author: Igor Prusov <ivprusov@salutedevices.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (((!0u32) << l) & ((!0u32) >> (31 - h)))
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 3;

const NTP8918_RATES: c_uint = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000;

const NTP8918_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

const NTP8918_INPUT_FMT: c_uint = 0x0;
const NTP8918_INPUT_FMT_MASTER_MODE: c_uint = BIT(0);
const NTP8918_INPUT_FMT_GSA_MODE: c_uint = BIT(1);
const NTP8918_GSA_FMT: c_uint = 0x1;
const NTP8918_GSA_BS_MASK: c_uint = GENMASK(3, 2);
const fn NTP8918_GSA_BS(x: c_uint) -> c_uint {
    x << 2
}
const NTP8918_GSA_RIGHT_J: c_uint = BIT(0);
const NTP8918_GSA_LSB: c_uint = BIT(1);
const NTP8918_MCLK_FREQ_CTRL: c_uint = 0x2;
const NTP8918_MCLK_FREQ_MCF: c_uint = GENMASK(1, 0);
const NTP8918_MASTER_VOL: c_uint = 0x0C;
const NTP8918_CHNL_A_VOL: c_uint = 0x17;
const NTP8918_CHNL_B_VOL: c_uint = 0x18;
const NTP8918_SOFT_MUTE: c_uint = 0x33;
const NTP8918_SOFT_MUTE_SM1: c_uint = BIT(0);
const NTP8918_SOFT_MUTE_SM2: c_uint = BIT(1);
const NTP8918_PWM_SWITCH: c_uint = 0x34;
const NTP8918_PWM_MASK_CTRL0: c_uint = 0x35;
const REG_MAX: c_uint = NTP8918_PWM_MASK_CTRL0;

const NTP8918_FW_NAME: *const c_char = b"eq_8918.bin\0".as_ptr() as *const c_char;
const NTP8918_FW_MAGIC: c_uint = 0x38393138; /* "8918" */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
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
pub struct ntp8918_priv {
    i2c: *mut i2c_client,
    bck: *mut clk,
    reset: *mut reset_control,
    format: c_uint,
}

#[repr(C)]
pub struct reg_sequence {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
    regmap: *mut regmap,
}

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
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
pub struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
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
pub struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    name: [c_char; 20],
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
    driver: device_driver,
}

unsafe extern "C" {
    fn reset_control_deassert(reset: *mut reset_control) -> c_int;
    fn reset_control_assert(reset: *mut reset_control) -> c_int;
    fn fsleep(usecs: c_uint);
    fn ntpfw_load(i2c: *mut i2c_client, name: *const c_char, magic: c_uint) -> c_int;
    fn dev_warn_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regmap_multi_reg_write_bypassed(
        map: *mut regmap,
        regs: *const reg_sequence,
        num_regs: c_int,
    ) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn snd_soc_component_cache_sync(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_reset_control_get_shared(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
}

/* static const DECLARE_TLV_DB_SCALE(ntp8918_master_vol_scale, -12550, 50, 0); */
static ntp8918_master_vol_scale: [c_uint; 4] = [0, (-12550i32) as c_uint, 50, 0];

/* SOC_SINGLE_RANGE_TLV and SOC_SINGLE expand to snd_kcontrol_new initializers. */
static ntp8918_vol_control: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn ntp8918_reset_gpio(ntp8918: *mut ntp8918_priv) {
    /*
     * Proper initialization sequence for NTP8918 amplifier requires driving
     * /RESET signal low during power up for at least 0.1us. The sequence is,
     * according to NTP8918 datasheet, 6.2 Timing Sequence 1:
     * Deassert for T2 >= 1ms...
     */
    unsafe {
        reset_control_deassert((*ntp8918).reset);
        fsleep(1000);

        /* ...Assert for T3 >= 0.1us... */
        reset_control_assert((*ntp8918).reset);
        fsleep(1);

        /* ...Deassert, and wait for T4 >= 0.5ms before sound on sequence. */
        reset_control_deassert((*ntp8918).reset);
        fsleep(500);
    }
}

static ntp8918_sound_off: [reg_sequence; 1] = [reg_sequence {
    reg: NTP8918_MASTER_VOL,
    def: 0,
}];

static ntp8918_sound_on: [reg_sequence; 1] = [reg_sequence {
    reg: NTP8918_MASTER_VOL,
    def: 0b11,
}];

unsafe extern "C" fn ntp8918_load_firmware(ntp8918: *mut ntp8918_priv) -> c_int {
    let ret: c_int;

    unsafe {
        ret = ntpfw_load((*ntp8918).i2c, NTP8918_FW_NAME, NTP8918_FW_MAGIC);
        if ret == -ENOENT {
            dev_warn_once(
                &mut (*(*ntp8918).i2c).dev,
                b"Could not find firmware %s\n\0".as_ptr() as *const c_char,
                NTP8918_FW_NAME,
            );
            return 0;
        }
    }

    ret
}

unsafe extern "C" fn ntp8918_snd_suspend(component: *mut snd_soc_component) -> c_int {
    let ntp8918 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8918_priv };

    unsafe {
        regcache_cache_only((*component).regmap, true);

        regmap_multi_reg_write_bypassed(
            (*component).regmap,
            ntp8918_sound_off.as_ptr(),
            ARRAY_SIZE(&ntp8918_sound_off) as c_int,
        );

        /*
         * According to NTP8918 datasheet, 6.2 Timing Sequence 1:
         * wait after sound off for T6 >= 0.5ms
         */
        fsleep(500);
        reset_control_assert((*ntp8918).reset);

        regcache_mark_dirty((*component).regmap);
        clk_disable_unprepare((*ntp8918).bck);
    }

    0
}

unsafe extern "C" fn ntp8918_snd_resume(component: *mut snd_soc_component) -> c_int {
    let ntp8918 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8918_priv };
    let mut ret: c_int;

    unsafe {
        ret = clk_prepare_enable((*ntp8918).bck);
        if ret != 0 {
            return ret;
        }

        ntp8918_reset_gpio(ntp8918);

        regmap_multi_reg_write_bypassed(
            (*component).regmap,
            ntp8918_sound_on.as_ptr(),
            ARRAY_SIZE(&ntp8918_sound_on) as c_int,
        );

        ret = ntp8918_load_firmware(ntp8918);
        if ret != 0 {
            dev_err(
                &mut (*(*ntp8918).i2c).dev,
                b"Failed to load firmware\n\0".as_ptr() as *const c_char,
            );
            return ret;
        }

        regcache_cache_only((*component).regmap, false);
        snd_soc_component_cache_sync(component);
    }

    0
}

unsafe extern "C" fn ntp8918_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let ntp8918 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8918_priv };
    let dev = unsafe { (*component).dev };

    unsafe {
        ret = snd_soc_add_component_controls(
            component,
            ntp8918_vol_control.as_ptr(),
            ARRAY_SIZE(&ntp8918_vol_control) as c_uint,
        );
        if ret != 0 {
            return dev_err_probe(dev, ret, b"Failed to add controls\n\0".as_ptr() as *const c_char);
        }

        ret = ntp8918_load_firmware(ntp8918);
        if ret != 0 {
            return dev_err_probe(dev, ret, b"Failed to load firmware\n\0".as_ptr() as *const c_char);
        }
    }

    0
}

/* SND_SOC_DAPM_DAC and SND_SOC_DAPM_OUTPUT expand to snd_soc_dapm_widget initializers. */
static ntp8918_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static ntp8918_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"OUT1\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AIFIN\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT2\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AIFIN\0".as_ptr() as *const c_char,
    },
];

static soc_component_ntp8918: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ntp8918_probe),
    suspend: Some(ntp8918_snd_suspend),
    resume: Some(ntp8918_snd_resume),
    dapm_widgets: ntp8918_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&ntp8918_dapm_widgets),
    dapm_routes: ntp8918_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&ntp8918_dapm_routes),
};

unsafe extern "C" fn ntp8918_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let ntp8918 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8918_priv };
    let mut input_fmt: c_uint = 0;
    let mut gsa_fmt: c_uint = 0;
    let gsa_fmt_mask: c_uint;
    let mcf: c_uint;
    let bclk: c_int;
    let mut ret: c_int;

    unsafe {
        bclk = snd_soc_params_to_bclk(params);
    }
    match bclk {
        3072000 | 2822400 => {
            mcf = 0;
        }
        6144000 => {
            mcf = 1;
        }
        2048000 => {
            mcf = 2;
        }
        _ => {
            return -EINVAL;
        }
    }

    unsafe {
        ret = snd_soc_component_update_bits(
            component,
            NTP8918_MCLK_FREQ_CTRL,
            NTP8918_MCLK_FREQ_MCF,
            mcf,
        );
    }
    if ret != 0 {
        return ret;
    }

    match unsafe { (*ntp8918).format } {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_RIGHT_J => {
            input_fmt |= NTP8918_INPUT_FMT_GSA_MODE;
            gsa_fmt |= NTP8918_GSA_RIGHT_J;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            input_fmt |= NTP8918_INPUT_FMT_GSA_MODE;
        }
        _ => {}
    }

    unsafe {
        ret = snd_soc_component_update_bits(
            component,
            NTP8918_INPUT_FMT,
            NTP8918_INPUT_FMT_MASTER_MODE | NTP8918_INPUT_FMT_GSA_MODE,
            input_fmt,
        );
    }

    if (input_fmt & NTP8918_INPUT_FMT_GSA_MODE) == 0 || ret < 0 {
        return ret;
    }

    match unsafe { params_width(params) } {
        24 => {
            gsa_fmt |= NTP8918_GSA_BS(0);
        }
        20 => {
            gsa_fmt |= NTP8918_GSA_BS(1);
        }
        18 => {
            gsa_fmt |= NTP8918_GSA_BS(2);
        }
        16 => {
            gsa_fmt |= NTP8918_GSA_BS(3);
        }
        _ => {
            return -EINVAL;
        }
    }

    gsa_fmt_mask = NTP8918_GSA_BS_MASK | NTP8918_GSA_RIGHT_J | NTP8918_GSA_LSB;
    unsafe { snd_soc_component_update_bits(component, NTP8918_GSA_FMT, gsa_fmt_mask, gsa_fmt) }
}

unsafe extern "C" fn ntp8918_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = unsafe { (*dai).component };
    let ntp8918 =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8918_priv };

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_LEFT_J => unsafe {
            (*ntp8918).format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
        },
        _ => {
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn ntp8918_digital_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _stream: c_int,
) -> c_int {
    let mute_mask: c_uint = NTP8918_SOFT_MUTE_SM1 | NTP8918_SOFT_MUTE_SM2;

    unsafe {
        snd_soc_component_update_bits(
            (*dai).component,
            NTP8918_SOFT_MUTE,
            mute_mask,
            if mute != 0 { mute_mask } else { 0 },
        )
    }
}

static ntp8918_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ntp8918_hw_params),
    set_fmt: Some(ntp8918_set_fmt),
    mute_stream: Some(ntp8918_digital_mute),
};

static mut ntp8918_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ntp8918-amplifier\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: NTP8918_RATES,
        formats: NTP8918_FORMATS,
    },
    ops: &ntp8918_dai_ops,
};

static ntp8918_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: REG_MAX,
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn ntp8918_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ntp8918: *mut ntp8918_priv;
    let mut ret: c_int;
    let regmap: *mut regmap;

    unsafe {
        ntp8918 = devm_kzalloc(
            &mut (*i2c).dev,
            size_of::<ntp8918_priv>(),
            GFP_KERNEL,
        ) as *mut ntp8918_priv;
        if ntp8918.is_null() {
            return -ENOMEM;
        }

        (*ntp8918).i2c = i2c;

        (*ntp8918).reset = devm_reset_control_get_shared(&mut (*i2c).dev, ptr::null());
        if IS_ERR((*ntp8918).reset as *const c_void) {
            return dev_err_probe(
                &mut (*i2c).dev,
                PTR_ERR((*ntp8918).reset as *const c_void),
                b"Failed to get reset\n\0".as_ptr() as *const c_char,
            );
        }

        dev_set_drvdata(&mut (*i2c).dev, ntp8918 as *mut c_void);

        ntp8918_reset_gpio(ntp8918);

        regmap = devm_regmap_init_i2c(i2c, &ntp8918_regmap);
        if IS_ERR(regmap as *const c_void) {
            return dev_err_probe(
                &mut (*i2c).dev,
                PTR_ERR(regmap as *const c_void),
                b"Failed to allocate regmap\n\0".as_ptr() as *const c_char,
            );
        }

        ret = devm_snd_soc_register_component(
            &mut (*i2c).dev,
            &soc_component_ntp8918,
            &raw mut ntp8918_dai,
            1,
        );
        if ret != 0 {
            return dev_err_probe(
                &mut (*i2c).dev,
                ret,
                b"Failed to register component\n\0".as_ptr() as *const c_char,
            );
        }

        (*ntp8918).bck = devm_clk_get_enabled(
            &mut (*i2c).dev,
            b"bck\0".as_ptr() as *const c_char,
        );
        if IS_ERR((*ntp8918).bck as *const c_void) {
            return dev_err_probe(
                &mut (*i2c).dev,
                PTR_ERR((*ntp8918).bck as *const c_void),
                b"failed to get bck clock\n\0".as_ptr() as *const c_char,
            );
        }
    }

    0
}

static ntp8918_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'n' as c_char,
            b't' as c_char,
            b'p' as c_char,
            b'8' as c_char,
            b'9' as c_char,
            b'1' as c_char,
            b'8' as c_char,
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
/* MODULE_DEVICE_TABLE(i2c, ntp8918_i2c_id); */

static ntp8918_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"neofidelity,ntp8918\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, ntp8918_of_match); */

static mut ntp8918_i2c_driver: i2c_driver = i2c_driver {
    probe: Some(ntp8918_i2c_probe),
    id_table: ntp8918_i2c_id.as_ptr(),
    driver: device_driver {
        name: b"ntp8918\0".as_ptr() as *const c_char,
        of_match_table: ntp8918_of_match.as_ptr(),
    },
};
/* module_i2c_driver(ntp8918_i2c_driver); */

/* MODULE_AUTHOR("Igor Prusov <ivprusov@salutedevices.com>"); */
/* MODULE_DESCRIPTION("NTP8918 Audio Amplifier Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
