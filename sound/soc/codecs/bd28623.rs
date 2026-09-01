// SPDX-License-Identifier: GPL-2.0
//
// ROHM BD28623MUV class D speaker amplifier codec driver.
//
// Copyright (c) 2018 Socionext Inc.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const BD28623_NUM_SUPPLIES: usize = 3;

static bd28623_supply_names: [*const c_char; BD28623_NUM_SUPPLIES] = [
    b"VCCA\0".as_ptr() as *const c_char,
    b"VCCP1\0".as_ptr() as *const c_char,
    b"VCCP2\0".as_ptr() as *const c_char,
];

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct gpio_desc {
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
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub formats: u64,
    pub rates: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
struct bd28623_priv {
    dev: *mut device,
    supplies: [regulator_bulk_data; BD28623_NUM_SUPPLIES],
    reset_gpio: *mut gpio_desc,
    mute_gpio: *mut gpio_desc,

    switch_spk: c_int,
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const GPIOD_OUT_HIGH: c_int = 1;
const SND_SOC_NOPM: c_int = -1;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1u64 << 10;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1u64 << 6;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1u64 << 2;
const SNDRV_PCM_RATE_48000: c_uint = 1u32 << 10;
const SNDRV_PCM_RATE_44100: c_uint = 1u32 << 9;
const SNDRV_PCM_RATE_32000: c_uint = 1u32 << 8;

unsafe extern "C" {
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id {
    ptr
}

// SND_SOC_DAPM_DAC("DAC", "Playback", SND_SOC_NOPM, 0, 0)
// SND_SOC_DAPM_OUTPUT("OUT1P")
// SND_SOC_DAPM_OUTPUT("OUT1N")
// SND_SOC_DAPM_OUTPUT("OUT2P")
// SND_SOC_DAPM_OUTPUT("OUT2N")
static bd28623_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static bd28623_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: b"OUT1P\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT1N\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT2P\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT2N\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn bd28623_power_on(bd: *mut bd28623_priv) -> c_int {
    let ret: c_int;

    ret = regulator_bulk_enable(
        (*bd).supplies.len() as c_int,
        (*bd).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            (*bd).dev,
            b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    gpiod_set_value_cansleep((*bd).reset_gpio, 0);
    usleep_range(300000, 400000);

    0
}

unsafe extern "C" fn bd28623_power_off(bd: *mut bd28623_priv) {
    gpiod_set_value_cansleep((*bd).reset_gpio, 1);

    regulator_bulk_disable(
        (*bd).supplies.len() as c_int,
        (*bd).supplies.as_mut_ptr(),
    );
}

unsafe extern "C" fn bd28623_get_switch_spk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let bd: *mut bd28623_priv = snd_soc_component_get_drvdata(component) as *mut bd28623_priv;

    (*ucontrol).value.integer.value[0] = (*bd).switch_spk as c_long;

    0
}

unsafe extern "C" fn bd28623_set_switch_spk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let bd: *mut bd28623_priv = snd_soc_component_get_drvdata(component) as *mut bd28623_priv;

    if (*bd).switch_spk == (*ucontrol).value.integer.value[0] as c_int {
        return 0;
    }

    (*bd).switch_spk = (*ucontrol).value.integer.value[0] as c_int;

    gpiod_set_value_cansleep((*bd).mute_gpio, if (*bd).switch_spk != 0 { 0 } else { 1 });

    0
}

// SOC_SINGLE_BOOL_EXT("Speaker Switch", 0,
//                     bd28623_get_switch_spk, bd28623_set_switch_spk)
static bd28623_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];

unsafe extern "C" fn bd28623_codec_probe(component: *mut snd_soc_component) -> c_int {
    let bd: *mut bd28623_priv = snd_soc_component_get_drvdata(component) as *mut bd28623_priv;
    let ret: c_int;

    (*bd).switch_spk = 1;

    ret = bd28623_power_on(bd);
    if ret != 0 {
        return ret;
    }

    gpiod_set_value_cansleep((*bd).mute_gpio, if (*bd).switch_spk != 0 { 0 } else { 1 });

    0
}

unsafe extern "C" fn bd28623_codec_remove(component: *mut snd_soc_component) {
    let bd: *mut bd28623_priv = snd_soc_component_get_drvdata(component) as *mut bd28623_priv;

    bd28623_power_off(bd);
}

unsafe extern "C" fn bd28623_codec_suspend(component: *mut snd_soc_component) -> c_int {
    let bd: *mut bd28623_priv = snd_soc_component_get_drvdata(component) as *mut bd28623_priv;

    bd28623_power_off(bd);

    0
}

unsafe extern "C" fn bd28623_codec_resume(component: *mut snd_soc_component) -> c_int {
    let bd: *mut bd28623_priv = snd_soc_component_get_drvdata(component) as *mut bd28623_priv;
    let ret: c_int;

    ret = bd28623_power_on(bd);
    if ret != 0 {
        return ret;
    }

    gpiod_set_value_cansleep((*bd).mute_gpio, if (*bd).switch_spk != 0 { 0 } else { 1 });

    0
}

static soc_codec_bd: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(bd28623_codec_probe),
    remove: Some(bd28623_codec_remove),
    suspend: Some(bd28623_codec_suspend),
    resume: Some(bd28623_codec_resume),
    dapm_widgets: bd28623_widgets.as_ptr(),
    num_dapm_widgets: bd28623_widgets.len() as c_uint,
    dapm_routes: bd28623_routes.as_ptr(),
    num_dapm_routes: bd28623_routes.len() as c_uint,
    controls: bd28623_controls.as_ptr(),
    num_controls: bd28623_controls.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static mut soc_dai_bd: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"bd28623-speaker\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
        rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_32000,
        channels_min: 2,
        channels_max: 2,
    },
};

unsafe extern "C" fn bd28623_probe(pdev: *mut platform_device) -> c_int {
    let bd: *mut bd28623_priv;
    let dev: *mut device = &mut (*pdev).dev;
    let mut i: c_int;
    let ret: c_int;

    bd = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<bd28623_priv>(),
        GFP_KERNEL,
    ) as *mut bd28623_priv;
    if bd.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while (i as usize) < (*bd).supplies.len() {
        (*bd).supplies[i as usize].supply = bd28623_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        dev,
        (*bd).supplies.len() as c_int,
        (*bd).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to get supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    (*bd).reset_gpio = devm_gpiod_get_optional(
        dev,
        b"reset\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*bd).reset_gpio as *const c_void) {
        dev_err(
            dev,
            b"Failed to request reset_gpio: %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR((*bd).reset_gpio as *const c_void),
        );
        return PTR_ERR((*bd).reset_gpio as *const c_void) as c_int;
    }

    (*bd).mute_gpio = devm_gpiod_get_optional(
        dev,
        b"mute\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*bd).mute_gpio as *const c_void) {
        dev_err(
            dev,
            b"Failed to request mute_gpio: %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR((*bd).mute_gpio as *const c_void),
        );
        return PTR_ERR((*bd).mute_gpio as *const c_void) as c_int;
    }

    platform_set_drvdata(pdev, bd as *mut c_void);
    (*bd).dev = dev;

    devm_snd_soc_register_component(dev, &soc_codec_bd, &mut soc_dai_bd, 1)
}

// __maybe_unused
static bd28623_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"rohm,bd28623\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, bd28623_of_match);

static bd28623_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"bd28623\0".as_ptr() as *const c_char,
        of_match_table: of_match_ptr(bd28623_of_match.as_ptr()),
    },
    probe: Some(bd28623_probe),
};
// module_platform_driver(bd28623_codec_driver);

// MODULE_AUTHOR("Katsuhiro Suzuki <suzuki.katsuhiro@socionext.com>");
// MODULE_DESCRIPTION("ROHM BD28623 speaker amplifier driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
