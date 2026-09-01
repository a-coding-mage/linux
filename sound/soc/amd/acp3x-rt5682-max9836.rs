// SPDX-License-Identifier: GPL-2.0+
//
// Machine driver for AMD ACP Audio engine using DA7219 & MAX98357 codec.
//
// Copyright 2016 Advanced Micro Devices, Inc.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

const PCO_PLAT_CLK: c_uint = 48000000;
const RT5682_PLL_FREQ: c_uint = 48000 * 512;
const DUAL_CHANNEL: c_uint = 2;

const RT5682: usize = 0;
const MAX: usize = 1;
const EC: usize = 2;

type bool_c = c_int;

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct acp3x_platform_info {
    pub play_i2s_instance: c_int,
    pub cap_i2s_instance: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub playback_only: bool_c,
    pub capture_only: bool_c,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub private_value: c_ulong,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub invert: c_uint,
    pub kcontrol_news: *const snd_kcontrol_new,
}

type c_uchar = u8;

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_ulong,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn acpi_match_device(ids: *const acpi_device_id, dev: *mut device) -> *const acpi_device_id;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x3000;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x4000;
const SND_SOC_CLOCK_IN: c_int = 0;
const RT5682_PLL2: c_int = 1;
const RT5682_PLL2_S_MCLK: c_int = 2;
const RT5682_SCLK_S_PLL2: c_int = 2;
const RT1015_PLL_S_BCLK: c_int = 1;
const RT1015_SCLK_S_PLL: c_int = 1;
const I2S_SP_INSTANCE: c_int = 0;
const I2S_BT_INSTANCE: c_int = 2;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 10;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 12;
const SND_SOC_NOPM: c_int = -1;
const GPIOD_OUT_LOW: c_int = 0;
const GFP_KERNEL: c_uint = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

static mut pco_jack: snd_soc_jack = snd_soc_jack { jack: ptr::null_mut() };

static mut pco_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: cstr(b"Headphone Jack\0"),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: cstr(b"Headset Mic\0"),
        mask: SND_JACK_MICROPHONE,
    },
];

static mut rt5682_dai_wclk: *mut clk = ptr::null_mut();
static mut rt5682_dai_bclk: *mut clk = ptr::null_mut();
static mut dmic_sel: *mut gpio_desc = ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn soc_is_rltk_max(dev: *mut device) -> *mut c_void {
    let mut match_: *const acpi_device_id;

    match_ = acpi_match_device((*(*dev).driver).acpi_match_table, dev);
    if match_.is_null() {
        return ptr::null_mut();
    }
    (*match_).driver_data as *mut c_void
}

unsafe extern "C" fn acp3x_5682_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut ret: c_int;
    let card: *mut snd_soc_card = (*rtd).card;
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component: *mut snd_soc_component = (*codec_dai).component;

    dev_info((*rtd).dev, cstr(b"codec dai name = %s\n\0"), (*codec_dai).name);

    /* set rt5682 dai fmt */
    ret = snd_soc_dai_set_fmt(
        codec_dai,
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    );
    if ret < 0 {
        dev_err((*(*rtd).card).dev, cstr(b"Failed to set rt5682 dai fmt: %d\n\0"), ret);
        return ret;
    }

    /* set codec PLL */
    ret = snd_soc_dai_set_pll(codec_dai, RT5682_PLL2, RT5682_PLL2_S_MCLK, PCO_PLAT_CLK, RT5682_PLL_FREQ);
    if ret < 0 {
        dev_err((*rtd).dev, cstr(b"can't set rt5682 PLL: %d\n\0"), ret);
        return ret;
    }

    /* Set codec sysclk */
    ret = snd_soc_dai_set_sysclk(codec_dai, RT5682_SCLK_S_PLL2, RT5682_PLL_FREQ, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*rtd).dev, cstr(b"Failed to set rt5682 SYSCLK: %d\n\0"), ret);
        return ret;
    }

    /* Set tdm/i2s1 master bclk ratio */
    ret = snd_soc_dai_set_bclk_ratio(codec_dai, 64);
    if ret < 0 {
        dev_err((*rtd).dev, cstr(b"Failed to set rt5682 tdm bclk ratio: %d\n\0"), ret);
        return ret;
    }

    rt5682_dai_wclk = devm_clk_get((*component).dev, cstr(b"rt5682-dai-wclk\0"));
    if IS_ERR(rt5682_dai_wclk) {
        return PTR_ERR(rt5682_dai_wclk);
    }

    rt5682_dai_bclk = devm_clk_get((*component).dev, cstr(b"rt5682-dai-bclk\0"));
    if IS_ERR(rt5682_dai_bclk) {
        return PTR_ERR(rt5682_dai_bclk);
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        cstr(b"Headset Jack\0"),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &raw mut pco_jack,
        pco_jack_pins.as_mut_ptr(),
        pco_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, cstr(b"HP jack creation failed %d\n\0"), ret);
        return ret;
    }

    snd_jack_set_key(pco_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(pco_jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(pco_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(pco_jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    ret = snd_soc_component_set_jack(component, &raw mut pco_jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*rtd).dev, cstr(b"Headset Jack call-back failed: %d\n\0"), ret);
        return ret;
    }

    ret
}

unsafe extern "C" fn rt5682_clk_enable(substream: *mut snd_pcm_substream) -> c_int {
    let mut ret: c_int = 0;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);

    /* RT5682 will support only 48K output with 48M mclk */
    clk_set_rate(rt5682_dai_wclk, 48000);
    clk_set_rate(rt5682_dai_bclk, 48000 * 64);
    ret = clk_prepare_enable(rt5682_dai_wclk);
    if ret < 0 {
        dev_err((*rtd).dev, cstr(b"can't enable wclk %d\n\0"), ret);
        return ret;
    }

    ret
}

unsafe extern "C" fn acp3x_1015_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let srate: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    ret = 0;
    srate = params_rate(params) as c_int;

    i = 0;
    while i < (*rtd).card.cast::<snd_soc_card>().as_ref().map_or(0, |_| 0) {
        i += 1;
    }
    /* for_each_rtd_codec_dais(rtd, i, codec_dai) */
    i = 0;
    loop {
        codec_dai = snd_soc_rtd_to_codec(rtd, i);
        if codec_dai.is_null() {
            break;
        }
        if strcmp((*codec_dai).name, cstr(b"rt1015-aif\0")) != 0 {
            i += 1;
            continue;
        }

        ret = snd_soc_dai_set_pll(codec_dai, 0, RT1015_PLL_S_BCLK, (64 * srate) as c_uint, (256 * srate) as c_uint);
        if ret < 0 {
            return ret;
        }
        ret = snd_soc_dai_set_sysclk(codec_dai, RT1015_SCLK_S_PLL, (256 * srate) as c_uint, SND_SOC_CLOCK_IN);
        if ret < 0 {
            return ret;
        }
        i += 1;
    }
    ret
}

unsafe extern "C" fn rt5682_clk_disable() {
    clk_disable_unprepare(rt5682_dai_wclk);
}

static channels: [c_uint; 1] = [DUAL_CHANNEL];
static rates: [c_uint; 1] = [48000];

static constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates.len() as c_uint,
    list: rates.as_ptr(),
    mask: 0,
};

static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: channels.len() as c_uint,
    list: channels.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn acp3x_5682_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let card: *mut snd_soc_card = (*rtd).card;
    let machine: *mut acp3x_platform_info = snd_soc_card_get_drvdata(card) as *mut acp3x_platform_info;

    (*machine).play_i2s_instance = I2S_SP_INSTANCE;
    (*machine).cap_i2s_instance = I2S_SP_INSTANCE;

    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    rt5682_clk_enable(substream)
}

unsafe extern "C" fn acp3x_max_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let card: *mut snd_soc_card = (*rtd).card;
    let machine: *mut acp3x_platform_info = snd_soc_card_get_drvdata(card) as *mut acp3x_platform_info;

    (*machine).play_i2s_instance = I2S_BT_INSTANCE;

    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    rt5682_clk_enable(substream)
}

unsafe extern "C" fn acp3x_ec_dmic0_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let card: *mut snd_soc_card = (*rtd).card;
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let machine: *mut acp3x_platform_info = snd_soc_card_get_drvdata(card) as *mut acp3x_platform_info;

    (*machine).cap_i2s_instance = I2S_BT_INSTANCE;
    snd_soc_dai_set_bclk_ratio(codec_dai, 64);

    rt5682_clk_enable(substream)
}

static mut dmic_switch: c_int = 0;

unsafe extern "C" fn dmic_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    (*ucontrol).value.integer.value[0] = dmic_switch as c_long;
    0
}

unsafe extern "C" fn dmic_set(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    if !dmic_sel.is_null() {
        dmic_switch = (*ucontrol).value.integer.value[0] as c_int;
        gpiod_set_value(dmic_sel, dmic_switch);
    }
    0
}

unsafe extern "C" fn rt5682_shutdown(_substream: *mut snd_pcm_substream) {
    rt5682_clk_disable();
}

static acp3x_5682_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp3x_5682_startup),
    shutdown: Some(rt5682_shutdown),
    hw_params: None,
};

static acp3x_max_play_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp3x_max_startup),
    shutdown: Some(rt5682_shutdown),
    hw_params: Some(acp3x_1015_hw_params),
};

static acp3x_ec_cap0_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp3x_ec_dmic0_startup),
    shutdown: Some(rt5682_shutdown),
    hw_params: None,
};

static mut acp3x_i2s: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: cstr(b"acp3x_i2s_playcap.0\0"), dai_name: ptr::null() }];
static mut acp3x_bt: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: cstr(b"acp3x_i2s_playcap.2\0"), dai_name: ptr::null() }];
static mut rt5682: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: cstr(b"i2c-10EC5682:00\0"), dai_name: cstr(b"rt5682-aif1\0") }];
static mut max: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: cstr(b"MX98357A:00\0"), dai_name: cstr(b"HiFi\0") }];
static mut rt1015p: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: cstr(b"RTL1015:00\0"), dai_name: cstr(b"HiFi\0") }];
static mut rt1015: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component { name: cstr(b"i2c-10EC1015:00\0"), dai_name: cstr(b"rt1015-aif\0") },
    snd_soc_dai_link_component { name: cstr(b"i2c-10EC1015:01\0"), dai_name: cstr(b"rt1015-aif\0") },
];
static mut cros_ec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: cstr(b"GOOG0013:00\0"), dai_name: cstr(b"EC Codec I2S RX\0") }];
static mut platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: cstr(b"acp3x_rv_i2s_dma.0\0"), dai_name: ptr::null() }];

static mut rt1015_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf {
        dlc: snd_soc_dai_link_component { name: cstr(b"i2c-10EC1015:00\0"), dai_name: ptr::null() },
        name_prefix: cstr(b"Left\0"),
    },
    snd_soc_codec_conf {
        dlc: snd_soc_dai_link_component { name: cstr(b"i2c-10EC1015:01\0"), dai_name: ptr::null() },
        name_prefix: cstr(b"Right\0"),
    },
];

static mut acp3x_dai: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: cstr(b"acp3x-5682-play\0"),
        stream_name: cstr(b"Playback\0"),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        init: Some(acp3x_5682_init),
        ops: &acp3x_5682_ops,
        playback_only: 0,
        capture_only: 0,
        cpus: unsafe { acp3x_i2s.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { rt5682.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: cstr(b"acp3x-max98357-play\0"),
        stream_name: cstr(b"HiFi Playback\0"),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        init: None,
        ops: &acp3x_max_play_ops,
        playback_only: 1,
        capture_only: 0,
        cpus: unsafe { acp3x_bt.as_mut_ptr() },
        num_cpus: 1,
        codecs: ptr::null_mut(),
        num_codecs: 0,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: cstr(b"acp3x-ec-dmic0-capture\0"),
        stream_name: cstr(b"Capture DMIC0\0"),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        init: None,
        ops: &acp3x_ec_cap0_ops,
        playback_only: 0,
        capture_only: 1,
        cpus: unsafe { acp3x_bt.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { cros_ec.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
    },
];

static dmic_mux_text: [*const c_char; 2] = [cstr(b"Front Mic\0"), cstr(b"Rear Mic\0")];

static acp3x_dmic_enum: soc_enum = soc_enum { _private: [] };

static acp3x_dmic_mux_control: snd_kcontrol_new = snd_kcontrol_new {
    name: cstr(b"DMIC Select Mux\0"),
    private_value: &acp3x_dmic_enum as *const soc_enum as c_ulong,
    get: Some(dmic_get),
    put: Some(dmic_set),
};

static acp3x_5682_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { id: 0, name: cstr(b"Headphone Jack\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Spk\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Headset Mic\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Dmic Mux\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: &acp3x_dmic_mux_control },
];

static acp3x_5682_audio_route: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: cstr(b"Headphone Jack\0"), control: ptr::null(), source: cstr(b"HPOL\0") },
    snd_soc_dapm_route { sink: cstr(b"Headphone Jack\0"), control: ptr::null(), source: cstr(b"HPOR\0") },
    snd_soc_dapm_route { sink: cstr(b"IN1P\0"), control: ptr::null(), source: cstr(b"Headset Mic\0") },
    snd_soc_dapm_route { sink: cstr(b"Spk\0"), control: ptr::null(), source: cstr(b"Speaker\0") },
    snd_soc_dapm_route { sink: cstr(b"Dmic Mux\0"), control: cstr(b"Front Mic\0"), source: cstr(b"DMIC\0") },
    snd_soc_dapm_route { sink: cstr(b"Dmic Mux\0"), control: cstr(b"Rear Mic\0"), source: cstr(b"DMIC\0") },
];

static acp3x_5682_mc_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { name: cstr(b"Headphone Jack\0"), private_value: 0, get: None, put: None },
    snd_kcontrol_new { name: cstr(b"Spk\0"), private_value: 0, get: None, put: None },
    snd_kcontrol_new { name: cstr(b"Headset Mic\0"), private_value: 0, get: None, put: None },
];

static mut acp3x_5682: snd_soc_card = snd_soc_card {
    name: cstr(b"acp3xalc5682m98357\0"),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { acp3x_dai.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: acp3x_5682_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: acp3x_5682_audio_route.as_ptr(),
    num_dapm_routes: 6,
    controls: acp3x_5682_mc_controls.as_ptr(),
    num_controls: 3,
    codec_conf: ptr::null_mut(),
    num_configs: 0,
    dev: ptr::null_mut(),
};

static acp3x_1015_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { id: 0, name: cstr(b"Headphone Jack\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Headset Mic\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Dmic Mux\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: &acp3x_dmic_mux_control },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Left Spk\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Right Spk\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
];

static acp3x_1015_route: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: cstr(b"Headphone Jack\0"), control: ptr::null(), source: cstr(b"HPOL\0") },
    snd_soc_dapm_route { sink: cstr(b"Headphone Jack\0"), control: ptr::null(), source: cstr(b"HPOR\0") },
    snd_soc_dapm_route { sink: cstr(b"IN1P\0"), control: ptr::null(), source: cstr(b"Headset Mic\0") },
    snd_soc_dapm_route { sink: cstr(b"Dmic Mux\0"), control: cstr(b"Front Mic\0"), source: cstr(b"DMIC\0") },
    snd_soc_dapm_route { sink: cstr(b"Dmic Mux\0"), control: cstr(b"Rear Mic\0"), source: cstr(b"DMIC\0") },
    snd_soc_dapm_route { sink: cstr(b"Left Spk\0"), control: ptr::null(), source: cstr(b"Left SPO\0") },
    snd_soc_dapm_route { sink: cstr(b"Right Spk\0"), control: ptr::null(), source: cstr(b"Right SPO\0") },
];

static acp3x_mc_1015_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { name: cstr(b"Headphone Jack\0"), private_value: 0, get: None, put: None },
    snd_kcontrol_new { name: cstr(b"Headset Mic\0"), private_value: 0, get: None, put: None },
    snd_kcontrol_new { name: cstr(b"Left Spk\0"), private_value: 0, get: None, put: None },
    snd_kcontrol_new { name: cstr(b"Right Spk\0"), private_value: 0, get: None, put: None },
];

static mut acp3x_1015: snd_soc_card = snd_soc_card {
    name: cstr(b"acp3xalc56821015\0"),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { acp3x_dai.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: acp3x_1015_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: acp3x_1015_route.as_ptr(),
    num_dapm_routes: 7,
    controls: acp3x_mc_1015_controls.as_ptr(),
    num_controls: 4,
    codec_conf: unsafe { rt1015_conf.as_mut_ptr() },
    num_configs: 2,
    dev: ptr::null_mut(),
};

static acp3x_1015p_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { id: 0, name: cstr(b"Headphone Jack\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Headset Mic\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Dmic Mux\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: &acp3x_dmic_mux_control },
    snd_soc_dapm_widget { id: 0, name: cstr(b"Speakers\0"), reg: SND_SOC_NOPM, shift: 0, invert: 0, kcontrol_news: ptr::null() },
];

static acp3x_1015p_route: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: cstr(b"Headphone Jack\0"), control: ptr::null(), source: cstr(b"HPOL\0") },
    snd_soc_dapm_route { sink: cstr(b"Headphone Jack\0"), control: ptr::null(), source: cstr(b"HPOR\0") },
    snd_soc_dapm_route { sink: cstr(b"IN1P\0"), control: ptr::null(), source: cstr(b"Headset Mic\0") },
    snd_soc_dapm_route { sink: cstr(b"Dmic Mux\0"), control: cstr(b"Front Mic\0"), source: cstr(b"DMIC\0") },
    snd_soc_dapm_route { sink: cstr(b"Dmic Mux\0"), control: cstr(b"Rear Mic\0"), source: cstr(b"DMIC\0") },
    /* speaker */
    snd_soc_dapm_route { sink: cstr(b"Speakers\0"), control: ptr::null(), source: cstr(b"Speaker\0") },
];

static acp3x_mc_1015p_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { name: cstr(b"Speakers\0"), private_value: 0, get: None, put: None },
    snd_kcontrol_new { name: cstr(b"Headphone Jack\0"), private_value: 0, get: None, put: None },
    snd_kcontrol_new { name: cstr(b"Headset Mic\0"), private_value: 0, get: None, put: None },
];

static mut acp3x_1015p: snd_soc_card = snd_soc_card {
    name: cstr(b"acp3xalc56821015p\0"),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { acp3x_dai.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: acp3x_1015p_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: acp3x_1015p_route.as_ptr(),
    num_dapm_routes: 6,
    controls: acp3x_mc_1015p_controls.as_ptr(),
    num_controls: 3,
    codec_conf: ptr::null_mut(),
    num_configs: 0,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn card_spk_dai_link_present(links: *mut snd_soc_dai_link, card_name: *const c_char) {
    if strcmp(card_name, cstr(b"acp3xalc56821015\0")) == 0 {
        (*links.add(1)).codecs = rt1015.as_mut_ptr();
        (*links.add(1)).num_codecs = rt1015.len() as c_uint;
    } else if strcmp(card_name, cstr(b"acp3xalc56821015p\0")) == 0 {
        (*links.add(1)).codecs = rt1015p.as_mut_ptr();
        (*links.add(1)).num_codecs = rt1015p.len() as c_uint;
    } else {
        (*links.add(1)).codecs = max.as_mut_ptr();
        (*links.add(1)).num_codecs = max.len() as c_uint;
    }
}

unsafe extern "C" fn acp3x_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let card: *mut snd_soc_card;
    let machine: *mut acp3x_platform_info;
    let dev: *mut device = &raw mut (*pdev).dev;

    card = soc_is_rltk_max(dev) as *mut snd_soc_card;
    if card.is_null() {
        return -ENODEV;
    }

    machine = devm_kzalloc(&raw mut (*pdev).dev, core::mem::size_of::<acp3x_platform_info>(), GFP_KERNEL) as *mut acp3x_platform_info;
    if machine.is_null() {
        return -ENOMEM;
    }

    card_spk_dai_link_present((*card).dai_link, (*card).name);
    (*card).dev = &raw mut (*pdev).dev;
    platform_set_drvdata(pdev, card as *mut c_void);
    snd_soc_card_set_drvdata(card, machine as *mut c_void);

    dmic_sel = devm_gpiod_get(&raw mut (*pdev).dev, cstr(b"dmic\0"), GPIOD_OUT_LOW);
    if IS_ERR(dmic_sel) {
        dev_err(&raw mut (*pdev).dev, cstr(b"DMIC gpio failed err=%ld\n\0"), PTR_ERR(dmic_sel) as c_long);
        return PTR_ERR(dmic_sel);
    }

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);
    if ret != 0 {
        return dev_err_probe(
            &raw mut (*pdev).dev,
            ret,
            cstr(b"devm_snd_soc_register_card(%s) failed\n\0"),
            (*card).name,
        );
    }
    0
}

static acp3x_audio_acpi_match: [acpi_device_id; 4] = [
    acpi_device_id { id: *b"AMDI5682\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { &raw mut acp3x_5682 as c_ulong } },
    acpi_device_id { id: *b"AMDI1015\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { &raw mut acp3x_1015 as c_ulong } },
    acpi_device_id { id: *b"10021015\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { &raw mut acp3x_1015p as c_ulong } },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, acp3x_audio_acpi_match); */

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

static mut acp3x_audio: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: cstr(b"acp3x-alc5682-max98357\0"),
        acpi_match_table: acp3x_audio_acpi_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(acp3x_probe),
};

/* module_platform_driver(acp3x_audio); */

/* MODULE_AUTHOR("akshu.agrawal@amd.com"); */
/* MODULE_AUTHOR("Vishnuvardhanrao.Ravulapati@amd.com"); */
/* MODULE_AUTHOR("Vijendar.Mukunda@amd.com"); */
/* MODULE_DESCRIPTION("ALC5682 ALC1015, ALC1015P & MAX98357 audio support"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
