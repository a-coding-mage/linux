// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * cx20442.c  --  CX20442 ALSA Soc Audio driver
 *
 * Copyright 2009 Janusz Krzysztofik <jkrzyszt@tis.icnet.pl>
 *
 * Initially based on sound/soc/codecs/wm8400.c
 * Copyright 2008, 2009 Wolfson Microelectronics PLC.
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = core::ffi::c_uchar;
type size_t = usize;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EPROBE_DEFER: c_int = 517;

const CX20442_PM: c_uint = 0x0;

const CX20442_TELIN: c_uint = 0;
const CX20442_TELOUT: c_uint = 1;
const CX20442_MIC: c_uint = 2;
const CX20442_SPKOUT: c_uint = 3;
const CX20442_AGC: c_uint = 4;

#[repr(C)]
pub struct tty_operations {
    pub write: Option<unsafe extern "C" fn(*mut tty_struct, *const c_char, c_int) -> c_int>,
}

#[repr(C)]
pub struct tty_struct {
    pub ops: *mut tty_operations,
    pub disc_data: *mut c_void,
    pub receive_room: c_int,
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct cx20442_codec {
    pub component: *mut snd_soc_component,
    pub ready: bool,
}

#[repr(C)]
struct cx20442_priv {
    tty: *mut tty_struct,
    por: *mut regulator,
    reg_cache: u8,
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
    capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    read: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint) -> c_uint>,
    write: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint, c_uint) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct tty_ldisc_ops {
    name: *const c_char,
    owner: *mut c_void,
    open: Option<unsafe extern "C" fn(*mut tty_struct) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut tty_struct)>,
    hangup: Option<unsafe extern "C" fn(*mut tty_struct)>,
    receive_buf: Option<unsafe extern "C" fn(*mut tty_struct, *const u8, *const u8, size_t)>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

const SND_SOC_NOPM: c_uint = 0;
const SNDRV_PCM_RATE_8000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;

extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;

    fn regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn regulator_put(regulator: *mut regulator);

    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn tty_hangup(tty: *mut tty_struct);

    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// DAPM widget constructor macros from <sound/soc.h>; preserved as dependency intent.
static cx20442_dapm_widgets: [snd_soc_dapm_widget; 17] = [
    SND_SOC_DAPM_OUTPUT(cstr!("TELOUT")),
    SND_SOC_DAPM_OUTPUT(cstr!("SPKOUT")),
    SND_SOC_DAPM_OUTPUT(cstr!("AGCOUT")),
    SND_SOC_DAPM_MIXER(cstr!("SPKOUT Mixer"), SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA(cstr!("TELOUT Amp"), CX20442_PM, CX20442_TELOUT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA(cstr!("SPKOUT Amp"), CX20442_PM, CX20442_SPKOUT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA(cstr!("SPKOUT AGC"), CX20442_PM, CX20442_AGC, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_DAC(cstr!("DAC"), cstr!("Playback"), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC(cstr!("ADC"), cstr!("Capture"), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MIXER(cstr!("Input Mixer"), SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MICBIAS(cstr!("TELIN Bias"), CX20442_PM, CX20442_TELIN, 0),
    SND_SOC_DAPM_MICBIAS(cstr!("MIC Bias"), CX20442_PM, CX20442_MIC, 0),
    SND_SOC_DAPM_PGA(cstr!("MIC AGC"), CX20442_PM, CX20442_AGC, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_INPUT(cstr!("TELIN")),
    SND_SOC_DAPM_INPUT(cstr!("MIC")),
    SND_SOC_DAPM_INPUT(cstr!("AGCIN")),
];

extern "Rust" {
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIXER(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const c_void,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_PGA(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const c_void,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_DAC(
        name: *const c_char,
        stream: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_ADC(
        name: *const c_char,
        stream: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MICBIAS(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint)
        -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
}

static cx20442_audio_map: [snd_soc_dapm_route; 15] = [
    snd_soc_dapm_route { sink: cstr!("TELOUT"), control: core::ptr::null(), source: cstr!("TELOUT Amp") },
    snd_soc_dapm_route { sink: cstr!("SPKOUT"), control: core::ptr::null(), source: cstr!("SPKOUT Mixer") },
    snd_soc_dapm_route { sink: cstr!("SPKOUT Mixer"), control: core::ptr::null(), source: cstr!("SPKOUT Amp") },
    snd_soc_dapm_route { sink: cstr!("TELOUT Amp"), control: core::ptr::null(), source: cstr!("DAC") },
    snd_soc_dapm_route { sink: cstr!("SPKOUT Amp"), control: core::ptr::null(), source: cstr!("DAC") },
    snd_soc_dapm_route { sink: cstr!("SPKOUT Mixer"), control: core::ptr::null(), source: cstr!("SPKOUT AGC") },
    snd_soc_dapm_route { sink: cstr!("SPKOUT AGC"), control: core::ptr::null(), source: cstr!("AGCIN") },
    snd_soc_dapm_route { sink: cstr!("AGCOUT"), control: core::ptr::null(), source: cstr!("MIC AGC") },
    snd_soc_dapm_route { sink: cstr!("MIC AGC"), control: core::ptr::null(), source: cstr!("MIC") },
    snd_soc_dapm_route { sink: cstr!("MIC Bias"), control: core::ptr::null(), source: cstr!("MIC") },
    snd_soc_dapm_route { sink: cstr!("Input Mixer"), control: core::ptr::null(), source: cstr!("MIC Bias") },
    snd_soc_dapm_route { sink: cstr!("TELIN Bias"), control: core::ptr::null(), source: cstr!("TELIN") },
    snd_soc_dapm_route { sink: cstr!("Input Mixer"), control: core::ptr::null(), source: cstr!("TELIN Bias") },
    snd_soc_dapm_route { sink: cstr!("ADC"), control: core::ptr::null(), source: cstr!("Input Mixer") },
];

unsafe extern "C" fn cx20442_read_reg_cache(
    component: *mut snd_soc_component,
    reg: c_uint,
) -> c_uint {
    let cx20442 = snd_soc_component_get_drvdata(component) as *mut cx20442_priv;

    if reg >= 1 {
        return (-EINVAL) as c_uint;
    }

    (*cx20442).reg_cache as c_uint
}

#[repr(C)]
enum v253_vls {
    V253_VLS_NONE = 0,
    V253_VLS_T,
    V253_VLS_L,
    V253_VLS_LT,
    V253_VLS_S,
    V253_VLS_ST,
    V253_VLS_M,
    V253_VLS_MST,
    V253_VLS_S1,
    V253_VLS_S1T,
    V253_VLS_MS1T,
    V253_VLS_M1,
    V253_VLS_M1ST,
    V253_VLS_M1S1T,
    V253_VLS_H,
    V253_VLS_HT,
    V253_VLS_MS,
    V253_VLS_MS1,
    V253_VLS_M1S,
    V253_VLS_M1S1,
    V253_VLS_TEST,
}

fn cx20442_pm_to_v253_vls(value: u8) -> c_int {
    match (value as c_uint) & !(1 << CX20442_AGC) {
        0 => v253_vls::V253_VLS_T as c_int,
        x if x == (1 << CX20442_SPKOUT)
            || x == (1 << CX20442_MIC)
            || x == ((1 << CX20442_SPKOUT) | (1 << CX20442_MIC)) =>
        {
            v253_vls::V253_VLS_M1S1 as c_int
        }
        x if x == (1 << CX20442_TELOUT)
            || x == (1 << CX20442_TELIN)
            || x == ((1 << CX20442_TELOUT) | (1 << CX20442_TELIN)) =>
        {
            v253_vls::V253_VLS_L as c_int
        }
        x if x == ((1 << CX20442_TELOUT) | (1 << CX20442_MIC)) => {
            v253_vls::V253_VLS_NONE as c_int
        }
        _ => -EINVAL,
    }
}

fn cx20442_pm_to_v253_vsp(value: u8) -> c_int {
    match (value as c_uint) & !(1 << CX20442_AGC) {
        x if x == (1 << CX20442_SPKOUT)
            || x == (1 << CX20442_MIC)
            || x == ((1 << CX20442_SPKOUT) | (1 << CX20442_MIC)) =>
        {
            (((value as c_uint) & (1 << CX20442_AGC)) != 0) as c_int
        }
        _ => {
            if ((value as c_uint) & (1 << CX20442_AGC)) != 0 {
                -EINVAL
            } else {
                0
            }
        }
    }
}

unsafe extern "C" fn cx20442_write(
    component: *mut snd_soc_component,
    reg: c_uint,
    value: c_uint,
) -> c_int {
    let cx20442 = snd_soc_component_get_drvdata(component) as *mut cx20442_priv;
    let mut vls: c_int;
    let mut vsp: c_int;
    let old: c_int;
    let mut len: c_int;
    let mut buf = [0 as c_char; 18];

    if reg >= 1 {
        return -EINVAL;
    }

    /*
     * tty and write pointers required for talking to the modem
     * are expected to be set by the line discipline initialization code
     */
    if (*cx20442).tty.is_null()
        || (*(*cx20442).tty).ops.is_null()
        || (*(*(*cx20442).tty).ops).write.is_none()
    {
        return -EIO;
    }

    old = (*cx20442).reg_cache as c_int;
    (*cx20442).reg_cache = value as u8;

    vls = cx20442_pm_to_v253_vls(value as u8);
    if vls < 0 {
        return vls;
    }

    vsp = cx20442_pm_to_v253_vsp(value as u8);
    if vsp < 0 {
        return vsp;
    }

    if vls == v253_vls::V253_VLS_T as c_int || vls == cx20442_pm_to_v253_vls(old as u8) {
        if vsp == cx20442_pm_to_v253_vsp(old as u8) {
            return 0;
        }
        len = snprintf(buf.as_mut_ptr(), buf.len(), cstr!("at+vsp=%d\r"), vsp);
    } else if vsp == cx20442_pm_to_v253_vsp(old as u8) {
        len = snprintf(buf.as_mut_ptr(), buf.len(), cstr!("at+vls=%d\r"), vls);
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            cstr!("at+vls=%d;+vsp=%d\r"),
            vls,
            vsp,
        );
    }

    if len > (buf.len() - 1) as c_int {
        return -ENOMEM;
    }

    dev_dbg((*component).dev, cstr!("%s: %s\n"), cstr!("cx20442_write"), buf.as_ptr());
    if ((*(*(*cx20442).tty).ops).write.unwrap())((*cx20442).tty, buf.as_ptr(), len) != len {
        return -EIO;
    }

    0
}

/*
 * Line discpline related code
 *
 * Any of the callback functions below can be used in two ways:
 * 1) registerd by a machine driver as one of line discipline operations,
 * 2) called from a machine's provided line discipline callback function
 *    in case when extra machine specific code must be run as well.
 */

/* Modem init: echo off, digital speaker off, quiet off, voice mode */
static v253_init: &[u8] = b"ate0m0q0+fclass=8\r\0";

/* Line discipline .open() */
unsafe extern "C" fn v253_open(tty: *mut tty_struct) -> c_int {
    let ret: c_int;
    let len = strlen(v253_init.as_ptr() as *const c_char) as c_int;

    /* Doesn't make sense without write callback */
    if (*tty).ops.is_null() || (*(*tty).ops).write.is_none() {
        return -EINVAL;
    }

    /* Won't work if no codec pointer has been passed by a card driver */
    if (*tty).disc_data.is_null() {
        return -ENODEV;
    }

    (*tty).receive_room = 16;
    if ((*(*tty).ops).write.unwrap())(tty, v253_init.as_ptr() as *const c_char, len) != len {
        ret = -EIO;
        (*tty).disc_data = core::ptr::null_mut();
        return ret;
    }
    /* Actual setup will be performed after the modem responds. */
    0
}

/* Line discipline .close() */
unsafe extern "C" fn v253_close(tty: *mut tty_struct) {
    let codec = (*tty).disc_data as *mut cx20442_codec;
    let component = (*codec).component;
    let cx20442: *mut cx20442_priv;

    (*tty).disc_data = core::ptr::null_mut();

    if component.is_null() {
        return;
    }

    cx20442 = snd_soc_component_get_drvdata(component) as *mut cx20442_priv;

    /* Prevent the codec driver from further accessing the modem */
    (*cx20442).tty = core::ptr::null_mut();
    (*codec).ready = false;
}

/* Line discipline .hangup() */
unsafe extern "C" fn v253_hangup(tty: *mut tty_struct) {
    v253_close(tty);
}

/* Line discipline .receive_buf() */
unsafe extern "C" fn v253_receive(
    tty: *mut tty_struct,
    _cp: *const u8,
    _fp: *const u8,
    _count: size_t,
) {
    let codec = (*tty).disc_data as *mut cx20442_codec;
    let component = (*codec).component;
    let cx20442: *mut cx20442_priv;

    if component.is_null() {
        return;
    }

    cx20442 = snd_soc_component_get_drvdata(component) as *mut cx20442_priv;

    if (*cx20442).tty.is_null() {
        /* First modem response, complete setup procedure */

        /* Set up codec driver access to modem controls */
        (*cx20442).tty = tty;
        (*codec).ready = true;
    }
}

#[no_mangle]
pub static mut v253_ops: tty_ldisc_ops = tty_ldisc_ops {
    name: cstr!("cx20442"),
    owner: unsafe { THIS_MODULE },
    open: Some(v253_open),
    close: Some(v253_close),
    hangup: Some(v253_hangup),
    receive_buf: Some(v253_receive),
};
// EXPORT_SYMBOL_GPL(v253_ops);

/*
 * Codec DAI
 */

static mut cx20442_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("cx20442-voice"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
};

unsafe extern "C" fn cx20442_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let cx20442 = snd_soc_component_get_drvdata(component) as *mut cx20442_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut err: c_int = 0;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) != snd_soc_bias_level::SND_SOC_BIAS_STANDBY {
                return err;
            }
            if IS_ERR((*cx20442).por as *const c_void) {
                err = PTR_ERR((*cx20442).por as *const c_void);
            } else {
                err = regulator_enable((*cx20442).por);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) != snd_soc_bias_level::SND_SOC_BIAS_PREPARE {
                return err;
            }
            if IS_ERR((*cx20442).por as *const c_void) {
                err = PTR_ERR((*cx20442).por as *const c_void);
            } else {
                err = regulator_disable((*cx20442).por);
            }
        }
        _ => {}
    }

    err
}

unsafe extern "C" fn cx20442_component_probe(component: *mut snd_soc_component) -> c_int {
    let cx20442: *mut cx20442_priv;

    cx20442 = kzalloc(core::mem::size_of::<cx20442_priv>(), 0) as *mut cx20442_priv;
    if cx20442.is_null() {
        return -ENOMEM;
    }

    (*cx20442).por = regulator_get((*component).dev, cstr!("POR"));
    if IS_ERR((*cx20442).por as *const c_void) {
        let mut err = PTR_ERR((*cx20442).por as *const c_void);

        dev_warn((*component).dev, cstr!("failed to get POR supply (%d)"), err);
        /*
         * When running on a non-dt platform and requested regulator
         * is not available, regulator_get() never returns
         * -EPROBE_DEFER as it is not able to justify if the regulator
         * may still appear later.  On the other hand, the board can
         * still set full constraints flag at late_initcall in order
         * to instruct regulator_get() to return a dummy one if
         * sufficient.  Hence, if we get -ENODEV here, let's convert
         * it to -EPROBE_DEFER and wait for the board to decide or
         * let Deferred Probe infrastructure handle this error.
         */
        if err == -ENODEV {
            err = -EPROBE_DEFER;
        }
        kfree(cx20442 as *mut c_void);
        return err;
    }

    (*cx20442).tty = core::ptr::null_mut();

    snd_soc_component_set_drvdata(component, cx20442 as *mut c_void);

    0
}

/* power down chip */
unsafe extern "C" fn cx20442_component_remove(component: *mut snd_soc_component) {
    let cx20442 = snd_soc_component_get_drvdata(component) as *mut cx20442_priv;

    if !(*cx20442).tty.is_null() {
        let tty = (*cx20442).tty;
        tty_hangup(tty);
    }

    if !IS_ERR((*cx20442).por as *const c_void) {
        /* should be already in STANDBY, hence disabled */
        regulator_put((*cx20442).por);
    }

    snd_soc_component_set_drvdata(component, core::ptr::null_mut());
    kfree(cx20442 as *mut c_void);
}

static cx20442_component_dev: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cx20442_component_probe),
    remove: Some(cx20442_component_remove),
    set_bias_level: Some(cx20442_set_bias_level),
    read: Some(cx20442_read_reg_cache),
    write: Some(cx20442_write),
    dapm_widgets: cx20442_dapm_widgets.as_ptr(),
    num_dapm_widgets: cx20442_dapm_widgets.len() as c_uint,
    dapm_routes: cx20442_audio_map.as_ptr(),
    num_dapm_routes: cx20442_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn cx20442_platform_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &cx20442_component_dev,
        &mut cx20442_dai,
        1,
    )
}

static mut cx20442_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: cstr!("cx20442-codec"),
    },
    probe: Some(cx20442_platform_probe),
};

// module_platform_driver(cx20442_platform_driver);

// MODULE_DESCRIPTION("ASoC CX20442-11 voice modem codec driver");
// MODULE_AUTHOR("Janusz Krzysztofik");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:cx20442-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
