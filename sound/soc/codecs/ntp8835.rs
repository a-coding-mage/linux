// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the NTP8835/NTP8835C Audio Amplifiers
 *
 * Copyright (c) 2024, SaluteDevices. All Rights Reserved.
 *
 * Author: Igor Prusov <ivprusov@salutedevices.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(h: u32, l: u32) -> u32 {
    (!0u32 >> (31 - h)) & (!0u32 << l)
}

const NTP8835_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

const NTP8835_INPUT_FMT: c_uint = 0x0;
const NTP8835_INPUT_FMT_MASTER_MODE: c_uint = bit(0);
const NTP8835_INPUT_FMT_GSA_MODE: c_uint = bit(1);
const NTP8835_GSA_FMT: c_uint = 0x1;
const NTP8835_GSA_BS_MASK: c_uint = genmask(3, 2);
const fn NTP8835_GSA_BS(x: c_uint) -> c_uint {
    x << 2
}
const NTP8835_GSA_RIGHT_J: c_uint = bit(0);
const NTP8835_GSA_LSB: c_uint = bit(1);
const NTP8835_MCLK_FREQ_CTRL: c_uint = 0x2;
const NTP8835_MCLK_FREQ_MCF: c_uint = genmask(1, 0);
const NTP8835_SOFT_MUTE: c_uint = 0x26;
const NTP8835_SOFT_MUTE_SM1: c_uint = bit(0);
const NTP8835_SOFT_MUTE_SM2: c_uint = bit(1);
const NTP8835_SOFT_MUTE_SM3: c_uint = bit(2);
const NTP8835_PWM_SWITCH: c_uint = 0x27;
const NTP8835_PWM_SWITCH_POF1: c_uint = bit(0);
const NTP8835_PWM_SWITCH_POF2: c_uint = bit(1);
const NTP8835_PWM_SWITCH_POF3: c_uint = bit(2);
const NTP8835_PWM_MASK_CTRL0: c_uint = 0x28;
const NTP8835_PWM_MASK_CTRL0_OUT_LOW: c_uint = bit(1);
const NTP8835_PWM_MASK_CTRL0_FPMLD: c_uint = bit(2);
const NTP8835_MASTER_VOL: c_uint = 0x2e;
const NTP8835_CHNL_A_VOL: c_uint = 0x2f;
const NTP8835_CHNL_B_VOL: c_uint = 0x30;
const NTP8835_CHNL_C_VOL: c_uint = 0x31;
const REG_MAX: c_uint = NTP8835_CHNL_C_VOL;

static NTP8835_FW_NAME: &[u8] = b"eq_8835.bin\0";
const NTP8835_FW_MAGIC: c_uint = 0x38383335; /* "8835" */

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SND_SOC_NOPM: c_int = -1;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;

#[repr(C)]
struct i2c_client {
    dev: device,
}
#[repr(C)]
struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
struct clk {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
    regmap: *mut regmap,
}
#[repr(C)]
struct snd_kcontrol {
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
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    access: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}
#[repr(C)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_integer {
    min: i64,
    max: i64,
    step: i64,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_union,
}
#[repr(C)]
union snd_ctl_elem_value_union {
    integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [i64; 128],
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    access: c_uint,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct reg_sequence {
    reg: c_uint,
    def: c_uint,
}
#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
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
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    set_sysclk: Option<
        unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int,
    >,
}
#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
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
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
}
#[repr(C)]
struct i2c_device_id {
    name: [c_char; 20],
}
#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}
#[repr(C)]
struct i2c_driver {
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
    driver: device_driver,
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct ntp8835_priv {
    i2c: *mut i2c_client,
    reset: *mut reset_control,
    format: c_uint,
    mclk: *mut clk,
    mclk_rate: c_uint,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint);
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn fsleep(usecs: c_uint);
    fn ntpfw_load(i2c: *mut i2c_client, name: *const c_char, magic: c_uint) -> c_int;
    fn dev_warn_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regmap_multi_reg_write_bypassed(
        map: *mut regmap,
        regs: *const reg_sequence,
        num_regs: c_int,
    ) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn snd_soc_component_cache_sync(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_reset_control_get_shared(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> bool;
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

/* static const DECLARE_TLV_DB_RANGE(ntp8835_vol_scale, ...) */
static ntp8835_vol_scale: [c_uint; 0] = [];

unsafe extern "C" fn ntp8835_mute_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
        (*uinfo).access = SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE;
        (*uinfo).count = 1;

        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 1;
        (*uinfo).value.integer.step = 1;
    }

    0
}

unsafe extern "C" fn ntp8835_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { snd_kcontrol_chip(kcontrol) };
    let val: c_uint;

    val = unsafe { snd_soc_component_read(component, NTP8835_SOFT_MUTE) };

    unsafe {
        (*ucontrol).value.integer.value[0] = if val != 0 { 0 } else { 1 };
    }
    0
}

unsafe extern "C" fn ntp8835_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { snd_kcontrol_chip(kcontrol) };
    let val: c_uint;

    val = unsafe {
        if (*ucontrol).value.integer.value[0] != 0 {
            0
        } else {
            7
        }
    };

    unsafe {
        snd_soc_component_write(component, NTP8835_SOFT_MUTE, val);
    }

    0
}

static ntp8835_vol_control: [snd_kcontrol_new; 2] = [
    /* SOC_SINGLE_TLV("Playback Volume", NTP8835_MASTER_VOL, 0, 0xff, 0, ntp8835_vol_scale) */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Playback Volume\0".as_ptr() as *const c_char,
        info: None,
        access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
        get: None,
        put: None,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Playback Switch\0".as_ptr() as *const c_char,
        info: Some(ntp8835_mute_info),
        access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
        get: Some(ntp8835_mute_get),
        put: Some(ntp8835_mute_put),
    },
];

unsafe fn ntp8835_reset_gpio(ntp8835: *mut ntp8835_priv) {
    /*
     * Proper initialization sequence for NTP835 amplifier requires driving
     * /RESET signal low during power up for at least 0.1us. The sequence is,
     * according to NTP8835 datasheet, 6.2 Timing Sequence (recommended):
     * Deassert for T2 >= 1ms...
     */
    unsafe {
        reset_control_deassert((*ntp8835).reset);
        fsleep(1000);

        /* ...Assert for T3 >= 0.1us... */
        reset_control_assert((*ntp8835).reset);
        fsleep(1);

        /* ...Deassert, and wait for T4 >= 0.5ms before sound on sequence. */
        reset_control_deassert((*ntp8835).reset);
        fsleep(500);
    }
}

static ntp8835_sound_on: [reg_sequence; 3] = [
    reg_sequence {
        reg: NTP8835_PWM_MASK_CTRL0,
        def: NTP8835_PWM_MASK_CTRL0_FPMLD,
    },
    reg_sequence {
        reg: NTP8835_PWM_SWITCH,
        def: 0x00,
    },
    reg_sequence {
        reg: NTP8835_SOFT_MUTE,
        def: 0x00,
    },
];

static ntp8835_sound_off: [reg_sequence; 3] = [
    reg_sequence {
        reg: NTP8835_SOFT_MUTE,
        def: NTP8835_SOFT_MUTE_SM1 | NTP8835_SOFT_MUTE_SM2 | NTP8835_SOFT_MUTE_SM3,
    },
    reg_sequence {
        reg: NTP8835_PWM_SWITCH,
        def: NTP8835_PWM_SWITCH_POF1 | NTP8835_PWM_SWITCH_POF2 | NTP8835_PWM_SWITCH_POF3,
    },
    reg_sequence {
        reg: NTP8835_PWM_MASK_CTRL0,
        def: NTP8835_PWM_MASK_CTRL0_OUT_LOW | NTP8835_PWM_MASK_CTRL0_FPMLD,
    },
];

unsafe fn ntp8835_load_firmware(ntp8835: *mut ntp8835_priv) -> c_int {
    let ret: c_int;

    ret = unsafe {
        ntpfw_load(
            (*ntp8835).i2c,
            NTP8835_FW_NAME.as_ptr() as *const c_char,
            NTP8835_FW_MAGIC,
        )
    };
    if ret == -ENOENT {
        unsafe {
            dev_warn_once(
                &mut (*(*ntp8835).i2c).dev,
                b"Could not find firmware %s\n\0".as_ptr() as *const c_char,
                NTP8835_FW_NAME.as_ptr() as *const c_char,
            );
        }
        return 0;
    }

    ret
}

unsafe extern "C" fn ntp8835_snd_suspend(component: *mut snd_soc_component) -> c_int {
    let ntp8835: *mut ntp8835_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8835_priv };

    unsafe {
        regcache_cache_only((*component).regmap, true);

        regmap_multi_reg_write_bypassed(
            (*component).regmap,
            ntp8835_sound_off.as_ptr(),
            ntp8835_sound_off.len() as c_int,
        );

        /*
         * According to NTP8835 datasheet, 6.2 Timing Sequence (recommended):
         * wait after sound off for T6 >= 0.5ms
         */
        fsleep(500);
        reset_control_assert((*ntp8835).reset);

        regcache_mark_dirty((*component).regmap);
        clk_disable_unprepare((*ntp8835).mclk);
    }

    0
}

unsafe extern "C" fn ntp8835_snd_resume(component: *mut snd_soc_component) -> c_int {
    let ntp8835: *mut ntp8835_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8835_priv };
    let mut ret: c_int;

    unsafe {
        ntp8835_reset_gpio(ntp8835);
        ret = clk_prepare_enable((*ntp8835).mclk);
        if ret != 0 {
            return ret;
        }

        regmap_multi_reg_write_bypassed(
            (*component).regmap,
            ntp8835_sound_on.as_ptr(),
            ntp8835_sound_on.len() as c_int,
        );

        ret = ntp8835_load_firmware(ntp8835);
        if ret != 0 {
            dev_err(
                &mut (*(*ntp8835).i2c).dev,
                b"Failed to load firmware\n\0".as_ptr() as *const c_char,
            );
            return ret;
        }

        regcache_cache_only((*component).regmap, false);
        snd_soc_component_cache_sync(component);
    }

    0
}

unsafe extern "C" fn ntp8835_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let ntp8835: *mut ntp8835_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8835_priv };
    let dev: *mut device = unsafe { (*component).dev };

    ret = unsafe {
        snd_soc_add_component_controls(
            component,
            ntp8835_vol_control.as_ptr(),
            ntp8835_vol_control.len() as c_uint,
        )
    };
    if ret != 0 {
        return unsafe {
            dev_err_probe(dev, ret, b"Failed to add controls\n\0".as_ptr() as *const c_char)
        };
    }

    ret = unsafe { ntp8835_load_firmware(ntp8835) };
    if ret != 0 {
        return unsafe {
            dev_err_probe(dev, ret, b"Failed to load firmware\n\0".as_ptr() as *const c_char)
        };
    }

    0
}

/* static const struct snd_soc_dapm_widget ntp8835_dapm_widgets[] = {
 *     SND_SOC_DAPM_DAC("AIFIN", "Playback", SND_SOC_NOPM, 0, 0),
 *     SND_SOC_DAPM_OUTPUT("OUT1"),
 *     SND_SOC_DAPM_OUTPUT("OUT2"),
 *     SND_SOC_DAPM_OUTPUT("OUT3"),
 * };
 */
static ntp8835_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static ntp8835_dapm_routes: [snd_soc_dapm_route; 3] = [
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
    snd_soc_dapm_route {
        sink: b"OUT3\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AIFIN\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn ntp8835_set_component_sysclk(
    component: *mut snd_soc_component,
    _clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let ntp8835: *mut ntp8835_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8835_priv };

    unsafe {
        match freq {
            12288000 | 24576000 | 18432000 => {
                (*ntp8835).mclk_rate = freq;
            }
            _ => {
                (*ntp8835).mclk_rate = 0;
                dev_err(
                    (*component).dev,
                    b"Unsupported MCLK value: %u\0".as_ptr() as *const c_char,
                    freq,
                );
                return -EINVAL;
            }
        }
    }

    0
}

static soc_component_ntp8835: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ntp8835_probe),
    suspend: Some(ntp8835_snd_suspend),
    resume: Some(ntp8835_snd_resume),
    dapm_widgets: ntp8835_dapm_widgets.as_ptr(),
    num_dapm_widgets: ntp8835_dapm_widgets.len() as c_uint,
    dapm_routes: ntp8835_dapm_routes.as_ptr(),
    num_dapm_routes: ntp8835_dapm_routes.len() as c_uint,
    set_sysclk: Some(ntp8835_set_component_sysclk),
};

unsafe extern "C" fn ntp8835_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let ntp8835: *mut ntp8835_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8835_priv };
    let mut input_fmt: c_uint = 0;
    let mut gsa_fmt: c_uint = 0;
    let gsa_fmt_mask: c_uint;
    let mcf: c_uint;
    let mut ret: c_int;

    unsafe {
        match (*ntp8835).mclk_rate {
            12288000 => mcf = 0,
            24576000 => mcf = 1,
            18432000 => mcf = 2,
            _ => return -EINVAL,
        }

        ret = snd_soc_component_update_bits(
            component,
            NTP8835_MCLK_FREQ_CTRL,
            NTP8835_MCLK_FREQ_MCF,
            mcf,
        );
        if ret != 0 {
            return ret;
        }

        match (*ntp8835).format {
            SND_SOC_DAIFMT_I2S => {}
            SND_SOC_DAIFMT_RIGHT_J => {
                input_fmt |= NTP8835_INPUT_FMT_GSA_MODE;
                gsa_fmt |= NTP8835_GSA_RIGHT_J;
            }
            SND_SOC_DAIFMT_LEFT_J => {
                input_fmt |= NTP8835_INPUT_FMT_GSA_MODE;
            }
            _ => {}
        }

        ret = snd_soc_component_update_bits(
            component,
            NTP8835_INPUT_FMT,
            NTP8835_INPUT_FMT_MASTER_MODE | NTP8835_INPUT_FMT_GSA_MODE,
            input_fmt,
        );

        if (input_fmt & NTP8835_INPUT_FMT_GSA_MODE) == 0 || ret < 0 {
            return ret;
        }

        match params_width(params) {
            24 => {
                gsa_fmt |= NTP8835_GSA_BS(0);
            }
            20 => {
                gsa_fmt |= NTP8835_GSA_BS(1);
            }
            18 => {
                gsa_fmt |= NTP8835_GSA_BS(2);
            }
            16 => {
                gsa_fmt |= NTP8835_GSA_BS(3);
            }
            _ => return -EINVAL,
        }

        gsa_fmt_mask = NTP8835_GSA_BS_MASK | NTP8835_GSA_RIGHT_J | NTP8835_GSA_LSB;
        snd_soc_component_update_bits(component, NTP8835_GSA_FMT, gsa_fmt_mask, gsa_fmt)
    }
}

unsafe extern "C" fn ntp8835_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let ntp8835: *mut ntp8835_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ntp8835_priv };

    unsafe {
        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_LEFT_J => {
                (*ntp8835).format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
            }
            _ => return -EINVAL,
        }
    }
    0
}

static ntp8835_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ntp8835_hw_params),
    set_fmt: Some(ntp8835_set_fmt),
};

static mut ntp8835_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ntp8835-amplifier\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 3,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: NTP8835_FORMATS,
    },
    ops: &ntp8835_dai_ops,
};

static ntp8835_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: REG_MAX,
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn ntp8835_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ntp8835: *mut ntp8835_priv;
    let regmap: *mut regmap;
    let mut ret: c_int;

    unsafe {
        ntp8835 = devm_kzalloc(
            &mut (*i2c).dev,
            core::mem::size_of::<ntp8835_priv>(),
            GFP_KERNEL,
        ) as *mut ntp8835_priv;
        if ntp8835.is_null() {
            return -ENOMEM;
        }

        (*ntp8835).i2c = i2c;

        (*ntp8835).reset = devm_reset_control_get_shared(&mut (*i2c).dev, ptr::null());
        if IS_ERR((*ntp8835).reset as *const c_void) {
            return dev_err_probe(
                &mut (*i2c).dev,
                PTR_ERR((*ntp8835).reset as *const c_void),
                b"Failed to get reset\n\0".as_ptr() as *const c_char,
            );
        }

        ret = reset_control_deassert((*ntp8835).reset);
        if ret != 0 {
            return dev_err_probe(
                &mut (*i2c).dev,
                ret,
                b"Failed to deassert reset\n\0".as_ptr() as *const c_char,
            );
        }

        dev_set_drvdata(&mut (*i2c).dev, ntp8835 as *mut c_void);

        ntp8835_reset_gpio(ntp8835);

        regmap = devm_regmap_init_i2c(i2c, &ntp8835_regmap);
        if IS_ERR(regmap as *const c_void) {
            return dev_err_probe(
                &mut (*i2c).dev,
                PTR_ERR(regmap as *const c_void),
                b"Failed to allocate regmap\n\0".as_ptr() as *const c_char,
            );
        }

        ret = devm_snd_soc_register_component(
            &mut (*i2c).dev,
            &soc_component_ntp8835,
            &raw mut ntp8835_dai,
            1,
        );
        if ret != 0 {
            return dev_err_probe(
                &mut (*i2c).dev,
                ret,
                b"Failed to register component\n\0".as_ptr() as *const c_char,
            );
        }

        (*ntp8835).mclk = devm_clk_get_enabled(
            &mut (*i2c).dev,
            b"mclk\0".as_ptr() as *const c_char,
        );
        if IS_ERR((*ntp8835).mclk as *const c_void) {
            return dev_err_probe(
                &mut (*i2c).dev,
                PTR_ERR((*ntp8835).mclk as *const c_void),
                b"failed to get mclk\n\0".as_ptr() as *const c_char,
            );
        }
    }

    0
}

static ntp8835_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'n' as c_char,
            b't' as c_char,
            b'p' as c_char,
            b'8' as c_char,
            b'8' as c_char,
            b'3' as c_char,
            b'5' as c_char,
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
/* MODULE_DEVICE_TABLE(i2c, ntp8835_i2c_id); */

static ntp8835_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"neofidelity,ntp8835\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, ntp8835_of_match); */

static mut ntp8835_i2c_driver: i2c_driver = i2c_driver {
    probe: Some(ntp8835_i2c_probe),
    id_table: ntp8835_i2c_id.as_ptr(),
    driver: device_driver {
        name: b"ntp8835\0".as_ptr() as *const c_char,
        of_match_table: ntp8835_of_match.as_ptr(),
    },
};
/* module_i2c_driver(ntp8835_i2c_driver); */

/* MODULE_AUTHOR("Igor Prusov <ivprusov@salutedevices.com>"); */
/* MODULE_DESCRIPTION("NTP8835 Audio Amplifier Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
