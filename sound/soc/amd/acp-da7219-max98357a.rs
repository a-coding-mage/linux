// SPDX-License-Identifier: MIT
//
// Machine driver for AMD ACP Audio engine using DA7219, RT5682 & MAX98357 codec
//
// Copyright 2017-2021 Advanced Micro Devices, Inc.

// C includes translated as external dependencies:
// sound/core.h, sound/soc.h, sound/pcm.h, sound/pcm_params.h, sound/soc-dapm.h,
// sound/jack.h, linux/clk.h, linux/module.h, linux/regulator/machine.h,
// linux/regulator/driver.h, linux/i2c.h, linux/input.h, linux/acpi.h,
// "acp.h", "../codecs/da7219.h", "../codecs/rt5682.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

const CZ_PLAT_CLK: c_uint = 48000000;
const DUAL_CHANNEL: c_uint = 2;
const RT5682_PLL_FREQ: c_uint = 48000 * 512;

const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_MICROPHONE: c_uint = 0x0002;
const SND_JACK_LINEOUT: c_uint = 0x0004;
const SND_JACK_HEADSET: c_uint = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_uint = 0x4000;
const SND_JACK_BTN_1: c_uint = 0x2000;
const SND_JACK_BTN_2: c_uint = 0x1000;
const SND_JACK_BTN_3: c_uint = 0x0800;

const KEY_PLAYPAUSE: c_uint = 164;
const KEY_VOLUMEUP: c_uint = 115;
const KEY_VOLUMEDOWN: c_uint = 114;
const KEY_VOICECOMMAND: c_uint = 246;

const DA7219_CLKSRC_MCLK: c_int = 0;
const DA7219_SYSCLK_PLL: c_int = 1;
const DA7219_PLL_FREQ_OUT_98304: c_uint = 98304000;
const RT5682_SCLK_S_PLL2: c_int = 0;
const RT5682_PLL2: c_int = 2;
const RT5682_PLL2_S_MCLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;

const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 10;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 11;
const I2S_SP_INSTANCE: c_int = 1;
const I2S_BT_INSTANCE: c_int = 2;
const CAP_CHANNEL0: c_int = 0;
const CAP_CHANNEL1: c_int = 1;

const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_TRIGGER_ORDER_LDC: c_uint = 1;

const REGULATOR_VOLTAGE: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct device_driver {
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_hw {
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hw,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct acp_platform_info {
    pub play_i2s_instance: c_int,
    pub cap_i2s_instance: c_int,
    pub capture_channel: c_int,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub trigger_stop: c_uint,
    pub ops: *const snd_soc_ops,
    pub cpus: *const snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *const snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_uint,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct regulator_consumer_supply {
    pub supply: *const c_char,
    pub dev_name: *const c_char,
}

#[repr(C)]
pub struct regulation_constraints {
    pub always_on: c_uint,
}

#[repr(C)]
pub struct regulator_init_data {
    pub constraints: regulation_constraints,
    pub num_consumer_supplies: c_uint,
    pub consumer_supplies: *mut regulator_consumer_supply,
}

#[repr(C)]
pub struct regulator_config {
    pub init_data: *mut regulator_init_data,
    pub dev: *mut device,
}

#[repr(C)]
pub struct regulator_ops {
    _empty: [u8; 0],
}

#[repr(C)]
pub struct regulator_desc {
    pub name: *const c_char,
    pub type_: c_uint,
    pub owner: *mut c_void,
    pub ops: *const regulator_ops,
    pub fixed_uV: c_uint,
    pub n_voltages: c_uint,
}

#[repr(C)]
pub struct regulator_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;
    static mut acp_bt_uart_enable: bool;

    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_uint, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn acpi_match_device(ids: *const acpi_device_id, dev: *mut device) -> *const acpi_device_id;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn devm_regulator_register(dev: *mut device, desc: *const regulator_desc, config: *const regulator_config) -> *mut regulator_dev;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
}

static mut cz_jack: snd_soc_jack = snd_soc_jack { jack: null_mut() };
static mut cz_jack_pins: [snd_soc_jack_pin; 3] = [
    snd_soc_jack_pin { pin: b"Headphone Jack\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
    snd_soc_jack_pin { pin: b"Line Out\0".as_ptr() as *const c_char, mask: SND_JACK_LINEOUT },
];

static mut da7219_dai_wclk: *mut clk = null_mut();
static mut da7219_dai_bclk: *mut clk = null_mut();
static mut rt5682_dai_wclk: *mut clk = null_mut();
static mut rt5682_dai_bclk: *mut clk = null_mut();

unsafe extern "C" fn cz_da7219_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut ret: c_int;
    let card = (*rtd).card;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;

    dev_info((*rtd).dev, b"codec dai name = %s\n\0".as_ptr() as *const c_char, (*codec_dai).name);

    ret = snd_soc_dai_set_sysclk(codec_dai, DA7219_CLKSRC_MCLK, CZ_PLAT_CLK, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set codec sysclk: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_PLL, CZ_PLAT_CLK, DA7219_PLL_FREQ_OUT_98304);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set codec pll: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    da7219_dai_wclk = devm_clk_get((*component).dev, b"da7219-dai-wclk\0".as_ptr() as *const c_char);
    if IS_ERR(da7219_dai_wclk as *const c_void) {
        return PTR_ERR(da7219_dai_wclk as *const c_void) as c_int;
    }

    da7219_dai_bclk = devm_clk_get((*component).dev, b"da7219-dai-bclk\0".as_ptr() as *const c_char);
    if IS_ERR(da7219_dai_bclk as *const c_void) {
        return PTR_ERR(da7219_dai_bclk as *const c_void) as c_int;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &mut cz_jack,
        cz_jack_pins.as_mut_ptr(),
        cz_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, b"HP jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    snd_soc_component_set_jack(component, &mut cz_jack, null_mut());
    0
}

unsafe extern "C" fn da7219_clk_enable(substream: *mut snd_pcm_substream) -> c_int {
    let mut ret: c_int = 0;
    let rtd = snd_soc_substream_to_rtd(substream);

    /*
     * Set wclk to 48000 because the rate constraint of this driver is
     * 48000. ADAU7002 spec: "The ADAU7002 requires a BCLK rate that is
     * minimum of 64x the LRCLK sample rate." DA7219 is the only clk
     * source so for all codecs we have to limit bclk to 64X lrclk.
     */
    clk_set_rate(da7219_dai_wclk, 48000);
    clk_set_rate(da7219_dai_bclk, 48000 * 64);
    ret = clk_prepare_enable(da7219_dai_bclk);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't enable master clock %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret
}

unsafe extern "C" fn da7219_clk_disable() {
    clk_disable_unprepare(da7219_dai_bclk);
}

unsafe extern "C" fn cz_rt5682_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut ret: c_int;
    let card = (*rtd).card;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;

    dev_info((*codec_dai).dev, b"codec dai name = %s\n\0".as_ptr() as *const c_char, (*codec_dai).name);

    /* Set codec sysclk */
    ret = snd_soc_dai_set_sysclk(codec_dai, RT5682_SCLK_S_PLL2, RT5682_PLL_FREQ, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*codec_dai).dev, b"Failed to set rt5682 SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    /* set codec PLL */
    ret = snd_soc_dai_set_pll(codec_dai, RT5682_PLL2, RT5682_PLL2_S_MCLK, CZ_PLAT_CLK, RT5682_PLL_FREQ);
    if ret < 0 {
        dev_err((*codec_dai).dev, b"can't set rt5682 PLL: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    rt5682_dai_wclk = devm_clk_get((*component).dev, b"rt5682-dai-wclk\0".as_ptr() as *const c_char);
    if IS_ERR(rt5682_dai_wclk as *const c_void) {
        return PTR_ERR(rt5682_dai_wclk as *const c_void) as c_int;
    }

    rt5682_dai_bclk = devm_clk_get((*component).dev, b"rt5682-dai-bclk\0".as_ptr() as *const c_char);
    if IS_ERR(rt5682_dai_bclk as *const c_void) {
        return PTR_ERR(rt5682_dai_bclk as *const c_void) as c_int;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &mut cz_jack,
        cz_jack_pins.as_mut_ptr(),
        cz_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, b"HP jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key(cz_jack.jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    ret = snd_soc_component_set_jack(component, &mut cz_jack, null_mut());
    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

unsafe extern "C" fn rt5682_clk_enable(substream: *mut snd_pcm_substream) -> c_int {
    let mut ret: c_int;
    let rtd = snd_soc_substream_to_rtd(substream);

    /*
     * Set wclk to 48000 because the rate constraint of this driver is
     * 48000. ADAU7002 spec: "The ADAU7002 requires a BCLK rate that is
     * minimum of 64x the LRCLK sample rate." RT5682 is the only clk
     * source so for all codecs we have to limit bclk to 64X lrclk.
     */
    ret = clk_set_rate(rt5682_dai_wclk, 48000);
    if ret != 0 {
        dev_err((*rtd).dev, b"Error setting wclk rate: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = clk_set_rate(rt5682_dai_bclk, 48000 * 64);
    if ret != 0 {
        dev_err((*rtd).dev, b"Error setting bclk rate: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = clk_prepare_enable(rt5682_dai_wclk);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't enable wclk %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
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

unsafe fn constrain_stereo_48000(substream: *mut snd_pcm_substream) -> (*mut snd_soc_pcm_runtime, *mut acp_platform_info) {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let machine = snd_soc_card_get_drvdata(card) as *mut acp_platform_info;

    /*
     * On this platform for PCM device we support stereo
     */
    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    (rtd, machine)
}

unsafe extern "C" fn cz_da7219_play_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).play_i2s_instance = I2S_SP_INSTANCE;
    da7219_clk_enable(substream)
}

unsafe extern "C" fn cz_da7219_cap_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).cap_i2s_instance = I2S_SP_INSTANCE;
    (*machine).capture_channel = CAP_CHANNEL1;
    da7219_clk_enable(substream)
}

unsafe extern "C" fn cz_max_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).play_i2s_instance = I2S_BT_INSTANCE;
    da7219_clk_enable(substream)
}

unsafe extern "C" fn cz_dmic0_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).cap_i2s_instance = I2S_BT_INSTANCE;
    da7219_clk_enable(substream)
}

unsafe extern "C" fn cz_dmic1_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).cap_i2s_instance = I2S_SP_INSTANCE;
    (*machine).capture_channel = CAP_CHANNEL0;
    da7219_clk_enable(substream)
}

unsafe extern "C" fn cz_da7219_shutdown(_substream: *mut snd_pcm_substream) {
    da7219_clk_disable();
}

unsafe extern "C" fn cz_rt5682_play_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).play_i2s_instance = I2S_SP_INSTANCE;
    rt5682_clk_enable(substream)
}

unsafe extern "C" fn cz_rt5682_cap_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).cap_i2s_instance = I2S_SP_INSTANCE;
    (*machine).capture_channel = CAP_CHANNEL1;
    rt5682_clk_enable(substream)
}

unsafe extern "C" fn cz_rt5682_max_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).play_i2s_instance = I2S_BT_INSTANCE;
    rt5682_clk_enable(substream)
}

unsafe extern "C" fn cz_rt5682_dmic0_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).cap_i2s_instance = I2S_BT_INSTANCE;
    rt5682_clk_enable(substream)
}

unsafe extern "C" fn cz_rt5682_dmic1_startup(substream: *mut snd_pcm_substream) -> c_int {
    let (_, machine) = constrain_stereo_48000(substream);
    (*machine).cap_i2s_instance = I2S_SP_INSTANCE;
    (*machine).capture_channel = CAP_CHANNEL0;
    rt5682_clk_enable(substream)
}

unsafe extern "C" fn cz_rt5682_shutdown(_substream: *mut snd_pcm_substream) {
    rt5682_clk_disable();
}

static cz_da7219_play_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_da7219_play_startup), shutdown: Some(cz_da7219_shutdown) };
static cz_da7219_cap_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_da7219_cap_startup), shutdown: Some(cz_da7219_shutdown) };
static cz_max_play_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_max_startup), shutdown: Some(cz_da7219_shutdown) };
static cz_dmic0_cap_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_dmic0_startup), shutdown: Some(cz_da7219_shutdown) };
static cz_dmic1_cap_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_dmic1_startup), shutdown: Some(cz_da7219_shutdown) };
static cz_rt5682_play_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_rt5682_play_startup), shutdown: Some(cz_rt5682_shutdown) };
static cz_rt5682_cap_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_rt5682_cap_startup), shutdown: Some(cz_rt5682_shutdown) };
static cz_rt5682_max_play_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_rt5682_max_startup), shutdown: Some(cz_rt5682_shutdown) };
static cz_rt5682_dmic0_cap_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_rt5682_dmic0_startup), shutdown: Some(cz_rt5682_shutdown) };
static cz_rt5682_dmic1_cap_ops: snd_soc_ops = snd_soc_ops { startup: Some(cz_rt5682_dmic1_startup), shutdown: Some(cz_rt5682_shutdown) };

static designware1: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"designware-i2s.1\0".as_ptr() as *const c_char, dai_name: null() }];
static designware2: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"designware-i2s.2\0".as_ptr() as *const c_char, dai_name: null() }];
static designware3: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"designware-i2s.3\0".as_ptr() as *const c_char, dai_name: null() }];
static dlgs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"i2c-DLGS7219:00\0".as_ptr() as *const c_char, dai_name: b"da7219-hifi\0".as_ptr() as *const c_char }];
static rt5682: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"i2c-10EC5682:00\0".as_ptr() as *const c_char, dai_name: b"rt5682-aif1\0".as_ptr() as *const c_char }];
static mx: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"MX98357A:00\0".as_ptr() as *const c_char, dai_name: b"HiFi\0".as_ptr() as *const c_char }];
static adau: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"ADAU7002:00\0".as_ptr() as *const c_char, dai_name: b"adau7002-hifi\0".as_ptr() as *const c_char }];
static platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp_audio_dma.0\0".as_ptr() as *const c_char, dai_name: null() }];

macro_rules! dai_link_reg {
    ($cpu:ident, $codec:ident, $platform:ident) => {
        cpus: $cpu.as_ptr(),
        num_cpus: $cpu.len() as c_uint,
        codecs: $codec.as_ptr(),
        num_codecs: $codec.len() as c_uint,
        platforms: $platform.as_ptr(),
        num_platforms: $platform.len() as c_uint
    };
}

static mut cz_dai_7219_98357: [snd_soc_dai_link; 5] = [
    snd_soc_dai_link { name: b"amd-da7219-play\0".as_ptr() as *const c_char, stream_name: b"Playback\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: Some(cz_da7219_init), playback_only: 1, capture_only: 0, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_da7219_play_ops, dai_link_reg!(designware1, dlgs, platform) },
    snd_soc_dai_link { name: b"amd-da7219-cap\0".as_ptr() as *const c_char, stream_name: b"Capture\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 0, capture_only: 1, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_da7219_cap_ops, dai_link_reg!(designware2, dlgs, platform) },
    snd_soc_dai_link { name: b"amd-max98357-play\0".as_ptr() as *const c_char, stream_name: b"HiFi Playback\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 1, capture_only: 0, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_max_play_ops, dai_link_reg!(designware3, mx, platform) },
    /* C panel DMIC */
    snd_soc_dai_link { name: b"dmic0\0".as_ptr() as *const c_char, stream_name: b"DMIC0 Capture\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 0, capture_only: 1, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_dmic0_cap_ops, dai_link_reg!(designware3, adau, platform) },
    /* A/B panel DMIC */
    snd_soc_dai_link { name: b"dmic1\0".as_ptr() as *const c_char, stream_name: b"DMIC1 Capture\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 0, capture_only: 1, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_dmic1_cap_ops, dai_link_reg!(designware2, adau, platform) },
];

static mut cz_dai_5682_98357: [snd_soc_dai_link; 5] = [
    snd_soc_dai_link { name: b"amd-rt5682-play\0".as_ptr() as *const c_char, stream_name: b"Playback\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: Some(cz_rt5682_init), playback_only: 1, capture_only: 0, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_rt5682_play_ops, dai_link_reg!(designware1, rt5682, platform) },
    snd_soc_dai_link { name: b"amd-rt5682-cap\0".as_ptr() as *const c_char, stream_name: b"Capture\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 0, capture_only: 1, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_rt5682_cap_ops, dai_link_reg!(designware2, rt5682, platform) },
    snd_soc_dai_link { name: b"amd-max98357-play\0".as_ptr() as *const c_char, stream_name: b"HiFi Playback\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 1, capture_only: 0, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_rt5682_max_play_ops, dai_link_reg!(designware3, mx, platform) },
    /* C panel DMIC */
    snd_soc_dai_link { name: b"dmic0\0".as_ptr() as *const c_char, stream_name: b"DMIC0 Capture\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 0, capture_only: 1, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_rt5682_dmic0_cap_ops, dai_link_reg!(designware3, adau, platform) },
    /* A/B panel DMIC */
    snd_soc_dai_link { name: b"dmic1\0".as_ptr() as *const c_char, stream_name: b"DMIC1 Capture\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP, init: None, playback_only: 0, capture_only: 1, trigger_stop: SND_SOC_TRIGGER_ORDER_LDC, ops: &cz_rt5682_dmic1_cap_ops, dai_link_reg!(designware2, adau, platform) },
];

static cz_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { id: 0, name: b"Headphones\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { id: 1, name: b"Speakers\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { id: 2, name: b"Line Out\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { id: 3, name: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { id: 3, name: b"Int Mic\0".as_ptr() as *const c_char },
];

static cz_audio_route: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route { sink: b"Headphones\0".as_ptr() as *const c_char, control: null(), source: b"HPL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphones\0".as_ptr() as *const c_char, control: null(), source: b"HPR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MIC\0".as_ptr() as *const c_char, control: null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speakers\0".as_ptr() as *const c_char, control: null(), source: b"Speaker\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PDM_DAT\0".as_ptr() as *const c_char, control: null(), source: b"Int Mic\0".as_ptr() as *const c_char },
];

static cz_rt5682_audio_route: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route { sink: b"Headphones\0".as_ptr() as *const c_char, control: null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphones\0".as_ptr() as *const c_char, control: null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speakers\0".as_ptr() as *const c_char, control: null(), source: b"Speaker\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PDM_DAT\0".as_ptr() as *const c_char, control: null(), source: b"Int Mic\0".as_ptr() as *const c_char },
];

static cz_mc_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { name: b"Headphones\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Speakers\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Line Out\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Int Mic\0".as_ptr() as *const c_char },
];

static mut cz_card: snd_soc_card = snd_soc_card {
    name: b"acpd7219m98357\0".as_ptr() as *const c_char,
    owner: null_mut(),
    dai_link: unsafe { cz_dai_7219_98357.as_mut_ptr() },
    num_links: 5,
    dapm_widgets: cz_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: cz_audio_route.as_ptr(),
    num_dapm_routes: 5,
    controls: cz_mc_controls.as_ptr(),
    num_controls: 5,
    dev: null_mut(),
};

static mut cz_rt5682_card: snd_soc_card = snd_soc_card {
    name: b"acpr5682m98357\0".as_ptr() as *const c_char,
    owner: null_mut(),
    dai_link: unsafe { cz_dai_5682_98357.as_mut_ptr() },
    num_links: 5,
    dapm_widgets: cz_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: cz_rt5682_audio_route.as_ptr(),
    num_dapm_routes: 5,
    controls: cz_mc_controls.as_ptr(),
    num_controls: 5,
    dev: null_mut(),
};

unsafe extern "C" fn acp_soc_is_rltk_max(dev: *mut device) -> *mut c_void {
    let mut match_: *const acpi_device_id;

    match_ = acpi_match_device((*(*dev).driver).acpi_match_table, dev);
    if match_.is_null() {
        return null_mut();
    }
    (*match_).driver_data as *mut c_void
}

static mut acp_da7219_supplies: [regulator_consumer_supply; 4] = [
    regulator_consumer_supply { supply: b"VDD\0".as_ptr() as *const c_char, dev_name: b"i2c-DLGS7219:00\0".as_ptr() as *const c_char },
    regulator_consumer_supply { supply: b"VDDMIC\0".as_ptr() as *const c_char, dev_name: b"i2c-DLGS7219:00\0".as_ptr() as *const c_char },
    regulator_consumer_supply { supply: b"VDDIO\0".as_ptr() as *const c_char, dev_name: b"i2c-DLGS7219:00\0".as_ptr() as *const c_char },
    regulator_consumer_supply { supply: b"IOVDD\0".as_ptr() as *const c_char, dev_name: b"ADAU7002:00\0".as_ptr() as *const c_char },
];

static mut acp_da7219_data: regulator_init_data = regulator_init_data {
    constraints: regulation_constraints { always_on: 1 },
    num_consumer_supplies: 4,
    consumer_supplies: unsafe { acp_da7219_supplies.as_mut_ptr() },
};

static mut acp_da7219_cfg: regulator_config = regulator_config {
    init_data: unsafe { &mut acp_da7219_data },
    dev: null_mut(),
};

static acp_da7219_ops: regulator_ops = regulator_ops { _empty: [] };

static acp_da7219_desc: regulator_desc = regulator_desc {
    name: b"reg-fixed-1.8V\0".as_ptr() as *const c_char,
    type_: REGULATOR_VOLTAGE,
    owner: null_mut(),
    ops: &acp_da7219_ops,
    fixed_uV: 1800000, /* 1.8V */
    n_voltages: 1,
};

unsafe extern "C" fn cz_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut card: *mut snd_soc_card;
    let mut machine: *mut acp_platform_info;
    let mut rdev: *mut regulator_dev;
    let dev = &mut (*pdev).dev as *mut device;

    card = acp_soc_is_rltk_max(dev) as *mut snd_soc_card;
    if card.is_null() {
        return -ENODEV;
    }
    if strcmp((*card).name, b"acpd7219m98357\0".as_ptr() as *const c_char) == 0 {
        acp_da7219_cfg.dev = &mut (*pdev).dev;
        rdev = devm_regulator_register(&mut (*pdev).dev, &acp_da7219_desc, &acp_da7219_cfg);
        if IS_ERR(rdev as *const c_void) {
            dev_err(&mut (*pdev).dev, b"Failed to register regulator: %d\n\0".as_ptr() as *const c_char, PTR_ERR(rdev as *const c_void) as c_int);
            return -EINVAL;
        }
    }

    machine = devm_kzalloc(&mut (*pdev).dev, size_of::<acp_platform_info>(), GFP_KERNEL) as *mut acp_platform_info;
    if machine.is_null() {
        return -ENOMEM;
    }
    (*card).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, card as *mut c_void);
    snd_soc_card_set_drvdata(card, machine as *mut c_void);
    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"devm_snd_soc_register_card(%s) failed\n\0".as_ptr() as *const c_char,
            (*card).name,
        );
    }
    acp_bt_uart_enable = !device_property_read_bool(&mut (*pdev).dev, b"bt-pad-enable\0".as_ptr() as *const c_char);
    0
}

// Original C condition: #ifdef CONFIG_ACPI
static cz_audio_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id { id: [65, 77, 68, 55, 50, 49, 57, 0, 0], driver_data: unsafe { &cz_card as *const _ as c_ulong } },
    acpi_device_id { id: [65, 77, 68, 73, 53, 54, 56, 50, 0], driver_data: unsafe { &cz_rt5682_card as *const _ as c_ulong } },
    acpi_device_id { id: [0; 9], driver_data: 0 },
];
// MODULE_DEVICE_TABLE(acpi, cz_audio_acpi_match);

static cz_pcm_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"cz-da7219-max98357a\0".as_ptr() as *const c_char,
        acpi_match_table: cz_audio_acpi_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(cz_probe),
};

// module_platform_driver(cz_pcm_driver);
// MODULE_AUTHOR("akshu.agrawal@amd.com");
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("DA7219, RT5682 & MAX98357A audio support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
