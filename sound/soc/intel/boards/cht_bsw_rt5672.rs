// SPDX-License-Identifier: GPL-2.0-only
/*
 *  cht_bsw_rt5672.rs - ASoc Machine driver for Intel Cherryview-based platforms
 *                      Cherrytrail and Braswell, with RT5672 codec.
 *
 *  Copyright (C) 2014 Intel Corp
 *  Author: Subhransu S. Prusty <subhransu.s.prusty@intel.com>
 *          Mengdong Lin <mengdong.lin@intel.com>
 */

// C includes translated as external dependency intent:
// linux/gpio/consumer.h, linux/input.h, linux/module.h,
// linux/platform_device.h, linux/slab.h, linux/clk.h, linux/string.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/jack.h,
// sound/soc-acpi.h, ../../codecs/rt5670.h,
// ../atom/sst-atom-controls.h, ../common/soc-intel-quirks.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* The platform clock #3 outputs 19.2Mhz clock to codec as I2S MCLK */
const CHT_PLAT_CLK_3_HZ: c_uint = 19200000;
const CHT_CODEC_DAI: &[u8] = b"rt5670-aif1\0";

extern "C" {
    static mut snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_fixup_dai_links_platform_name(card: *mut snd_soc_card, platform_name: *const c_char) -> c_int;
    fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;

    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int);
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_int, val: c_uint) -> c_int;

    fn devm_acpi_dev_add_driver_gpios(dev: *mut device, gpios: *const acpi_gpio_mapping) -> c_int;
    fn rt5670_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: c_uint, clk_src: c_uint);
    fn rt5670_set_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn rt5670_jack_suspend(component: *mut snd_soc_component);
    fn rt5670_jack_resume(component: *mut snd_soc_component);
    fn rt5670_components() -> *const c_char;

    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: c_long) -> *mut acpi_device;
    fn acpi_dev_name(adev: *mut acpi_device) -> *const c_char;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn soc_intel_is_byt() -> bool;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const SND_ACPI_I2C_ID_LEN: usize = 32;
const SND_JACK_MICROPHONE: c_int = 0x0001;
const SND_JACK_HEADPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_MICROPHONE | SND_JACK_HEADPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const RT5670_PLL1_S_MCLK: c_int = 0;
const RT5670_SCLK_S_PLL1: c_int = 1;
const RT5670_SCLK_S_RCCLK: c_int = 2;
const RT5670_DA_STEREO_FILTER: c_uint = 1 << 0;
const RT5670_DA_MONO_L_FILTER: c_uint = 1 << 1;
const RT5670_DA_MONO_R_FILTER: c_uint = 1 << 2;
const RT5670_AD_STEREO_FILTER: c_uint = 1 << 3;
const RT5670_AD_MONO_L_FILTER: c_uint = 1 << 4;
const RT5670_AD_MONO_R_FILTER: c_uint = 1 << 5;
const RT5670_CLK_SEL_I2S1_ASRC: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const MERR_DPCM_AUDIO: usize = 0;
const MERR_DPCM_DEEP_BUFFER: usize = 1;

#[repr(C)]
pub struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
pub struct cht_mc_private {
    headset: snd_soc_jack,
    codec_name: [c_char; SND_ACPI_I2C_ID_LEN],
    mclk: *mut clk,
    use_ssp0: bool,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    name: *const c_char,
    event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    event_flags: c_int,
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    name: *const c_char,
}

#[repr(C)]
pub struct acpi_gpio_params {
    crs_entry_index: c_uint,
    line_index: c_uint,
    active_low: bool,
}

#[repr(C)]
pub struct acpi_gpio_mapping {
    name: *const c_char,
    data: *const acpi_gpio_params,
    size: c_uint,
}

#[repr(C)]
pub struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    id: c_int,
    nonatomic: bool,
    dynamic: c_int,
    playback_only: c_int,
    no_pcm: c_int,
    ops: *const snd_soc_ops,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    name: *mut c_char,
    dai_name: *mut c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    owner: *mut c_void,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    dev: *mut device,
    components: *const c_char,
    name: *const c_char,
    driver_name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct device {
    platform_data: *mut c_void,
    driver: *mut device_driver,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    id: *const c_char,
    mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    acpi_ipc_irq_index: c_int,
    platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
    name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

fn snd_soc_dapm_event_on(event: c_int) -> bool {
    event == SND_SOC_DAPM_PRE_PMU
}

fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

fn ptr_err<T>(ptr: *mut T) -> c_long {
    ptr as c_long
}

unsafe extern "C" fn platform_clock_control(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let codec_dai: *mut snd_soc_dai;
    let ctx = snd_soc_card_get_drvdata(card) as *mut cht_mc_private;
    let mut ret: c_int;

    codec_dai = snd_soc_card_get_codec_dai(card, CHT_CODEC_DAI.as_ptr() as *const c_char);
    if codec_dai.is_null() {
        dev_err((*card).dev, b"Codec dai not found; Unable to set platform clock\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    if snd_soc_dapm_event_on(event) {
        ret = clk_prepare_enable((*ctx).mclk);
        if ret < 0 {
            dev_err((*card).dev, b"could not configure MCLK state: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        /* set codec PLL source to the 19.2MHz platform clock (MCLK) */
        ret = snd_soc_dai_set_pll(codec_dai, 0, RT5670_PLL1_S_MCLK, CHT_PLAT_CLK_3_HZ, 48000 * 512);
        if ret < 0 {
            dev_err((*card).dev, b"can't set codec pll: %d\n\0".as_ptr() as *const c_char, ret);
            clk_disable_unprepare((*ctx).mclk);
            return ret;
        }

        /* set codec sysclk source to PLL */
        ret = snd_soc_dai_set_sysclk(codec_dai, RT5670_SCLK_S_PLL1, 48000 * 512, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*card).dev, b"can't set codec sysclk: %d\n\0".as_ptr() as *const c_char, ret);
            clk_disable_unprepare((*ctx).mclk);
            return ret;
        }
    } else {
        /* Set codec sysclk source to its internal clock because codec
         * PLL will be off when idle and MCLK will also be off by ACPI
         * when codec is runtime suspended. Codec needs clock for jack
         * detection and button press.
         */
        ret = snd_soc_dai_set_sysclk(codec_dai, RT5670_SCLK_S_RCCLK, 48000 * 512, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*card).dev, b"failed to set codec sysclk: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        clk_disable_unprepare((*ctx).mclk);
    }
    0
}

static mut CHT_BSW_HEADSET_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
    snd_soc_jack_pin { pin: b"Headphone\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
];

static CHT_DAPM_WIDGETS: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { name: b"Headphone\0".as_ptr() as *const c_char, event: None, event_flags: 0, dapm: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Headset Mic\0".as_ptr() as *const c_char, event: None, event_flags: 0, dapm: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Int Mic\0".as_ptr() as *const c_char, event: None, event_flags: 0, dapm: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Ext Spk\0".as_ptr() as *const c_char, event: None, event_flags: 0, dapm: ptr::null_mut() },
    snd_soc_dapm_widget {
        name: b"Platform Clock\0".as_ptr() as *const c_char,
        event: Some(platform_clock_control),
        event_flags: SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
        dapm: ptr::null_mut(),
    },
];

static CHT_AUDIO_MAP: [snd_soc_dapm_route; 14] = [
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN1N\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC L1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC R1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPOLP\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPOLN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPORP\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPORN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
];

static CHT_AUDIO_SSP0_MAP: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"AIF1 Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp0 Tx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ssp0 Tx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"modem_out\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"modem_in\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp0 Rx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ssp0 Rx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIF1 Capture\0".as_ptr() as *const c_char },
];

static CHT_AUDIO_SSP2_MAP: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: b"AIF1 Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp2 Tx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ssp2 Tx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"codec_out0\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ssp2 Tx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"codec_out1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"codec_in0\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp2 Rx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"codec_in1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp2 Rx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ssp2 Rx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIF1 Capture\0".as_ptr() as *const c_char },
];

static CHT_MC_CONTROLS: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { name: b"Headphone\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Int Mic\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Ext Spk\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn cht_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    /* set codec PLL source to the 19.2MHz platform clock (MCLK) */
    ret = snd_soc_dai_set_pll(codec_dai, 0, RT5670_PLL1_S_MCLK, CHT_PLAT_CLK_3_HZ, params_rate(params) * 512);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set codec pll: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* set codec sysclk source to PLL */
    ret = snd_soc_dai_set_sysclk(codec_dai, RT5670_SCLK_S_PLL1, params_rate(params) * 512, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set codec sysclk: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

static HEADSET_GPIOS: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 0, line_index: 0, active_low: false };

static CHT_RT5672_GPIOS: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping { name: b"headset-gpios\0".as_ptr() as *const c_char, data: &HEADSET_GPIOS, size: 1 },
    acpi_gpio_mapping { name: ptr::null(), data: ptr::null(), size: 0 },
];

unsafe extern "C" fn cht_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let mut ret: c_int;
    let dapm = snd_soc_card_to_dapm((*runtime).card);
    let codec_dai = snd_soc_rtd_to_codec(runtime, 0);
    let component = (*codec_dai).component;
    let ctx = snd_soc_card_get_drvdata((*runtime).card) as *mut cht_mc_private;

    if devm_acpi_dev_add_driver_gpios((*component).dev, CHT_RT5672_GPIOS.as_ptr()) != 0 {
        dev_warn((*runtime).dev, b"Unable to add GPIO mapping table\n\0".as_ptr() as *const c_char);
    }

    /* Select codec ASRC clock source to track I2S1 clock, because codec
     * is in slave mode and 100fs I2S format (BCLK = 100 * LRCLK) cannot
     * be supported by RT5672. Otherwise, ASRC will be disabled and cause
     * noise.
     */
    rt5670_sel_asrc_clk_src(
        component,
        RT5670_DA_STEREO_FILTER
            | RT5670_DA_MONO_L_FILTER
            | RT5670_DA_MONO_R_FILTER
            | RT5670_AD_STEREO_FILTER
            | RT5670_AD_MONO_L_FILTER
            | RT5670_AD_MONO_R_FILTER,
        RT5670_CLK_SEL_I2S1_ASRC,
    );

    if (*ctx).use_ssp0 {
        ret = snd_soc_dapm_add_routes(dapm, CHT_AUDIO_SSP0_MAP.as_ptr(), CHT_AUDIO_SSP0_MAP.len() as c_int);
    } else {
        ret = snd_soc_dapm_add_routes(dapm, CHT_AUDIO_SSP2_MAP.as_ptr(), CHT_AUDIO_SSP2_MAP.len() as c_int);
    }
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*runtime).card,
        b"Headset\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2,
        &mut (*ctx).headset,
        CHT_BSW_HEADSET_PINS.as_mut_ptr(),
        CHT_BSW_HEADSET_PINS.len() as c_uint,
    );
    if ret != 0 {
        return ret;
    }

    snd_jack_set_key((*ctx).headset.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*ctx).headset.jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key((*ctx).headset.jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);

    rt5670_set_jack_detect(component, &mut (*ctx).headset);

    /*
     * The firmware might enable the clock at boot (this information
     * may or may not be reflected in the enable clock register).
     * To change the rate we must disable the clock first to cover
     * these cases. Due to Common Clock Framework restrictions that
     * do not allow to disable a clock that has not been enabled, we
     * need to enable the clock first.
     */
    ret = clk_prepare_enable((*ctx).mclk);
    if ret == 0 {
        clk_disable_unprepare((*ctx).mclk);
    }

    ret = clk_set_rate((*ctx).mclk, CHT_PLAT_CLK_3_HZ);
    if ret != 0 {
        dev_err((*runtime).dev, b"unable to set MCLK rate\n\0".as_ptr() as *const c_char);
        return ret;
    }

    0
}

unsafe extern "C" fn cht_codec_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let ctx = snd_soc_card_get_drvdata((*rtd).card) as *mut cht_mc_private;
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut ret: c_int;
    let bits: c_int;

    /* The DSP will convert the FE rate to 48k, stereo, 24bits */
    (*rate).min = 48000;
    (*rate).max = 48000;
    (*channels).min = 2;
    (*channels).max = 2;

    if (*ctx).use_ssp0 {
        /* set SSP0 to 16-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);
        bits = 16;
    } else {
        /* set SSP2 to 24-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);
        bits = 24;
    }

    /*
     * The default mode for the cpu-dai is TDM 4 slot. The default mode
     * for the codec-dai is I2S. So we need to either set the cpu-dai to
     * I2S mode to match the codec-dai, or set the codec-dai to TDM 4 slot
     * (or program both to yet another mode).
     * One board, the Lenovo Miix 2 10, uses not 1 but 2 codecs connected
     * to SSP2. The second piggy-backed, output-only codec is inside the
     * keyboard-dock (which has extra speakers). Unlike the main rt5672
     * codec, we cannot configure this codec, it is hard coded to use
     * 2 channel 24 bit I2S. For this to work we must use I2S mode on this
     * board. Since we only support 2 channels anyways, there is no need
     * for TDM on any cht-bsw-rt5672 designs. So we use I2S 2ch everywhere.
     */
    ret = snd_soc_dai_set_fmt(
        snd_soc_rtd_to_cpu(rtd, 0),
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP,
    );
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set format to I2S, err %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, bits);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set I2S config, err %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn cht_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

static CHT_AIF1_OPS: snd_soc_ops = snd_soc_ops {
    startup: Some(cht_aif1_startup),
    hw_params: None,
};

static CHT_BE_SSP2_OPS: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(cht_aif1_hw_params),
};

// SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY()));
// SND_SOC_DAILINK_DEF(media, DAILINK_COMP_ARRAY(COMP_CPU("media-cpu-dai")));
// SND_SOC_DAILINK_DEF(deepbuffer, DAILINK_COMP_ARRAY(COMP_CPU("deepbuffer-cpu-dai")));
// SND_SOC_DAILINK_DEF(ssp2_port, DAILINK_COMP_ARRAY(COMP_CPU("ssp2-port")));
// SND_SOC_DAILINK_DEF(ssp2_codec, DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10EC5670:00", "rt5670-aif1")));
// SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("sst-mfld-platform")));

static mut DUMMY: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null_mut(), dai_name: ptr::null_mut() }];
static mut MEDIA: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null_mut(), dai_name: b"media-cpu-dai\0".as_ptr() as *mut c_char }];
static mut DEEPBUFFER: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null_mut(), dai_name: b"deepbuffer-cpu-dai\0".as_ptr() as *mut c_char }];
static mut SSP2_PORT: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: ptr::null_mut(), dai_name: b"ssp2-port\0".as_ptr() as *mut c_char }];
static mut SSP2_CODEC: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-10EC5670:00\0".as_ptr() as *mut c_char,
    dai_name: b"rt5670-aif1\0".as_ptr() as *mut c_char,
}];
static mut PLATFORM: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"sst-mfld-platform\0".as_ptr() as *mut c_char, dai_name: ptr::null_mut() }];

static mut CHT_DAILINK: [snd_soc_dai_link; 3] = [
    /* Front End DAI links */
    snd_soc_dai_link {
        name: b"Audio Port\0".as_ptr() as *const c_char,
        stream_name: b"Audio\0".as_ptr() as *const c_char,
        id: 0,
        nonatomic: true,
        dynamic: 1,
        playback_only: 0,
        no_pcm: 0,
        ops: &CHT_AIF1_OPS,
        init: None,
        be_hw_params_fixup: None,
        cpus: unsafe { MEDIA.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { DUMMY.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { PLATFORM.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: b"Deep-Buffer Audio Port\0".as_ptr() as *const c_char,
        stream_name: b"Deep-Buffer Audio\0".as_ptr() as *const c_char,
        id: 0,
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        no_pcm: 0,
        ops: &CHT_AIF1_OPS,
        init: None,
        be_hw_params_fixup: None,
        cpus: unsafe { DEEPBUFFER.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { DUMMY.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { PLATFORM.as_mut_ptr() },
        num_platforms: 1,
    },
    /* Back End DAI links */
    snd_soc_dai_link {
        /* SSP2 - Codec */
        name: b"SSP2-Codec\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        id: 0,
        nonatomic: false,
        dynamic: 0,
        playback_only: 0,
        no_pcm: 1,
        ops: &CHT_BE_SSP2_OPS,
        init: Some(cht_codec_init),
        be_hw_params_fixup: Some(cht_codec_fixup),
        cpus: unsafe { SSP2_PORT.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { SSP2_CODEC.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { PLATFORM.as_mut_ptr() },
        num_platforms: 1,
    },
];

unsafe extern "C" fn cht_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let mut component: *mut snd_soc_component;
    let ctx = snd_soc_card_get_drvdata(card) as *mut cht_mc_private;

    // for_each_card_components(card, component)
    component = ptr::null_mut();
    while !component.is_null() {
        if strncmp((*component).name, (*ctx).codec_name.as_ptr(), size_of::<[c_char; SND_ACPI_I2C_ID_LEN]>()) == 0 {
            dev_dbg((*component).dev, b"disabling jack detect before going to suspend.\n\0".as_ptr() as *const c_char);
            rt5670_jack_suspend(component);
            break;
        }
    }
    0
}

unsafe extern "C" fn cht_resume_post(card: *mut snd_soc_card) -> c_int {
    let mut component: *mut snd_soc_component;
    let ctx = snd_soc_card_get_drvdata(card) as *mut cht_mc_private;

    // for_each_card_components(card, component)
    component = ptr::null_mut();
    while !component.is_null() {
        if strncmp((*component).name, (*ctx).codec_name.as_ptr(), size_of::<[c_char; SND_ACPI_I2C_ID_LEN]>()) == 0 {
            dev_dbg((*component).dev, b"enabling jack detect for resume.\n\0".as_ptr() as *const c_char);
            rt5670_jack_resume(component);
            break;
        }
    }

    0
}

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: &[u8] = b"bytcht rt5672\0"; /* card name will be 'sof-bytcht rt5672' */
const SOF_DRIVER_NAME: &[u8] = b"SOF\0";

const CARD_NAME: &[u8] = b"cht-bsw-rt5672\0";
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

/* SoC card */
static mut SND_SOC_CARD_CHT: snd_soc_card = snd_soc_card {
    owner: ptr::null_mut(),
    dai_link: unsafe { CHT_DAILINK.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: CHT_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: CHT_AUDIO_MAP.as_ptr(),
    num_dapm_routes: 14,
    controls: CHT_MC_CONTROLS.as_ptr(),
    num_controls: 4,
    suspend_pre: Some(cht_suspend_pre),
    resume_post: Some(cht_resume_post),
    dev: ptr::null_mut(),
    components: ptr::null(),
    name: ptr::null(),
    driver_name: ptr::null(),
};

const RT5672_I2C_DEFAULT: &[u8] = b"i2c-10EC5670:00\0";

unsafe extern "C" fn snd_cht_mc_probe(pdev: *mut platform_device) -> c_int {
    let mut ret_val: c_int = 0;
    let drv: *mut cht_mc_private;
    let mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    let platform_name: *const c_char;
    let dev = &mut (*pdev).dev as *mut device;
    let adev: *mut acpi_device;
    let sof_parent: bool;
    let mut dai_index: c_int = 0;
    let mut i: c_int;

    drv = devm_kzalloc(dev, size_of::<cht_mc_private>(), GFP_KERNEL) as *mut cht_mc_private;
    if drv.is_null() {
        return -ENOMEM;
    }

    strscpy((*drv).codec_name.as_mut_ptr(), RT5672_I2C_DEFAULT.as_ptr() as *const c_char, (*drv).codec_name.len());

    /* find index of codec dai */
    i = 0;
    while i < CHT_DAILINK.len() as c_int {
        if CHT_DAILINK[i as usize].num_codecs != 0
            && strcmp((*CHT_DAILINK[i as usize].codecs).name, RT5672_I2C_DEFAULT.as_ptr() as *const c_char) == 0
        {
            dai_index = i;
            break;
        }
        i += 1;
    }

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            (*drv).codec_name.as_mut_ptr(),
            (*drv).codec_name.len(),
            b"i2c-%s\0".as_ptr() as *const c_char,
            acpi_dev_name(adev),
        );
        (*CHT_DAILINK[dai_index as usize].codecs).name = (*drv).codec_name.as_mut_ptr();
    } else {
        dev_err(dev, b"Error cannot find '%s' dev\n\0".as_ptr() as *const c_char, (*mach).id);
        return -ENOENT;
    }

    acpi_dev_put(adev);

    /* Use SSP0 on Bay Trail CR devices */
    if soc_intel_is_byt() && (*mach).mach_params.acpi_ipc_irq_index == 0 {
        (*CHT_DAILINK[dai_index as usize].cpus).dai_name = b"ssp0-port\0".as_ptr() as *mut c_char;
        (*drv).use_ssp0 = true;
    }

    /* override platform name, if required */
    SND_SOC_CARD_CHT.dev = dev;
    platform_name = (*mach).mach_params.platform;

    ret_val = snd_soc_fixup_dai_links_platform_name(&mut SND_SOC_CARD_CHT, platform_name);
    if ret_val != 0 {
        return ret_val;
    }

    SND_SOC_CARD_CHT.components = rt5670_components();

    (*drv).mclk = devm_clk_get(dev, b"pmc_plt_clk_3\0".as_ptr() as *const c_char);
    if is_err((*drv).mclk) {
        dev_err(
            dev,
            b"Failed to get MCLK from pmc_plt_clk_3: %ld\n\0".as_ptr() as *const c_char,
            ptr_err((*drv).mclk),
        );
        return ptr_err((*drv).mclk) as c_int;
    }
    snd_soc_card_set_drvdata(&mut SND_SOC_CARD_CHT, drv as *mut c_void);

    sof_parent = snd_soc_acpi_sof_parent(dev);

    /* set card and driver name */
    if sof_parent {
        SND_SOC_CARD_CHT.name = SOF_CARD_NAME.as_ptr() as *const c_char;
        SND_SOC_CARD_CHT.driver_name = SOF_DRIVER_NAME.as_ptr() as *const c_char;
    } else {
        SND_SOC_CARD_CHT.name = CARD_NAME.as_ptr() as *const c_char;
        SND_SOC_CARD_CHT.driver_name = DRIVER_NAME;
    }

    /* set pm ops */
    if sof_parent {
        (*(*pdev).dev.driver).pm = &snd_soc_pm_ops;
    }

    /* register the soc card */
    ret_val = devm_snd_soc_register_card(dev, &mut SND_SOC_CARD_CHT);
    if ret_val != 0 {
        dev_err(dev, b"snd_soc_register_card failed %d\n\0".as_ptr() as *const c_char, ret_val);
        return ret_val;
    }
    platform_set_drvdata(pdev, &mut SND_SOC_CARD_CHT as *mut snd_soc_card as *mut c_void);
    ret_val
}

static mut SND_CHT_MC_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"cht-bsw-rt5672\0".as_ptr() as *const c_char,
        pm: ptr::null(),
    },
    probe: Some(snd_cht_mc_probe),
};

// module_platform_driver(snd_cht_mc_driver);

// MODULE_DESCRIPTION("ASoC Intel(R) Baytrail CR Machine driver");
// MODULE_AUTHOR("Subhransu S. Prusty, Mengdong Lin");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:cht-bsw-rt5672");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
