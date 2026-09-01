// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rockchip machine ASoC driver for boards using MAX98357A/RT5514/DA7219
 *
 * Copyright (c) 2016, ROCKCHIP CORPORATION.  All rights reserved.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{copy_nonoverlapping, null, null_mut};

const DRV_NAME: *const c_char = b"rk3399-gru-sound\0".as_ptr() as *const c_char;

const SOUND_FS: c_uint = 256;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

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
const KEY_VOICECOMMAND: c_uint = 0x246;

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;

const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const RT5514_SCLK_S_MCLK: c_int = 1;
const DA7219_SYSCLK_MCLK: c_int = 0;

const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_runtime_hw {
    pub formats: u64,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_runtime_hw,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
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
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub mask: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
}

type c_uchar = u8;

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_routes: *mut snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    /* CONFIG_PM: .pm = &snd_soc_pm_ops */
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver_private,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static i2c_bus_type: bus_type;
    static spi_bus_type: bus_type;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn msleep(msecs: c_uint);
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_device_is_available(np: *mut device_node) -> c_int;
    fn of_device_is_compatible(np: *mut device_node, compatible: *const c_char) -> c_int;
    fn bus_find_device_by_of_node(
        bus: *const bus_type,
        np: *mut device_node,
    ) -> *mut device;
    fn put_device(dev: *mut device);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint)
        -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

static mut dmic_wakeup_delay: c_uint = 0;

static mut rockchip_sound_jack: snd_soc_jack = snd_soc_jack { jack: null_mut() };

/* Headset jack detection DAPM pins */
static mut rockchip_sound_jack_pins: [snd_soc_jack_pin; 3] = [
    snd_soc_jack_pin {
        pin: b"Headphones\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Line Out\0".as_ptr() as *const c_char,
        mask: SND_JACK_LINEOUT,
    },
];

static rockchip_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    snd_soc_dapm_widget { id: 0, name: b"Headphones\0".as_ptr() as *const c_char, reg: -1, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Speakers\0".as_ptr() as *const c_char, reg: -1, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Headset Mic\0".as_ptr() as *const c_char, reg: -1, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Line Out\0".as_ptr() as *const c_char, reg: -1, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Int Mic\0".as_ptr() as *const c_char, reg: -1, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"HDMI\0".as_ptr() as *const c_char, reg: -1, shift: 0, mask: 0, on_val: 0, off_val: 0 },
];

static rockchip_controls: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new { iface: 0, name: b"Headphones Switch\0".as_ptr() as *const c_char },
    snd_kcontrol_new { iface: 0, name: b"Speakers Switch\0".as_ptr() as *const c_char },
    snd_kcontrol_new { iface: 0, name: b"Headset Mic Switch\0".as_ptr() as *const c_char },
    snd_kcontrol_new { iface: 0, name: b"Line Out Switch\0".as_ptr() as *const c_char },
    snd_kcontrol_new { iface: 0, name: b"Int Mic Switch\0".as_ptr() as *const c_char },
    snd_kcontrol_new { iface: 0, name: b"HDMI Switch\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn rockchip_sound_max98357a_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mclk = params_rate(params).wrapping_mul(SOUND_FS);
    let ret = snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), 0, mclk, 0);
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"%s() error setting sysclk to %u: %d\n\0".as_ptr() as *const c_char,
            b"rockchip_sound_max98357a_hw_params\0".as_ptr() as *const c_char,
            mclk,
            ret,
        );
        return ret;
    }
    0
}

unsafe extern "C" fn rockchip_sound_rt5514_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mclk = params_rate(params).wrapping_mul(SOUND_FS);

    let mut ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"Can't set cpu clock out %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5514_SCLK_S_MCLK, mclk, SND_SOC_CLOCK_IN);
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"%s() error setting sysclk to %u: %d\n\0".as_ptr() as *const c_char,
            b"rockchip_sound_rt5514_hw_params\0".as_ptr() as *const c_char,
            params_rate(params).wrapping_mul(512),
            ret,
        );
        return ret;
    }

    /* Wait for DMIC stable */
    msleep(dmic_wakeup_delay);
    0
}

unsafe extern "C" fn rockchip_sound_da7219_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mclk: c_uint;

    /* in bypass mode, the mclk has to be one of the frequencies below */
    match params_rate(params) {
        8000 | 16000 | 24000 | 32000 | 48000 | 64000 | 96000 => mclk = 12288000,
        11025 | 22050 | 44100 | 88200 => mclk = 11289600,
        _ => return -EINVAL,
    }

    let mut ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"Can't set cpu clock out %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"Can't set codec clock in %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_MCLK, 0, 0);
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"Can't set pll sysclk mclk %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    0
}

static mut cdn_dp_card_jack: snd_soc_jack = snd_soc_jack { jack: null_mut() };

unsafe extern "C" fn rockchip_sound_cdndp_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let card = (*rtd).card;

    /* Enable jack detection. */
    let ret = snd_soc_card_jack_new(
        card,
        b"DP Jack\0".as_ptr() as *const c_char,
        SND_JACK_LINEOUT,
        &mut cdn_dp_card_jack,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"Can't create DP Jack %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    snd_soc_component_set_jack(component, &mut cdn_dp_card_jack, null_mut())
}

unsafe extern "C" fn rockchip_sound_da7219_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    /* We need default MCLK and PLL settings for the accessory detection */
    let mut ret = snd_soc_dai_set_sysclk(codec_dai, 0, 12288000, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"Init can't set codec clock in %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_MCLK, 0, 0);
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"Init can't set pll sysclk mclk %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* Enable Headset and 4 Buttons Jack detection */
    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &mut rockchip_sound_jack,
        rockchip_sound_jack_pins.as_mut_ptr(),
        rockchip_sound_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"New Headset Jack failed! (%d)\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    snd_jack_set_key(rockchip_sound_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(rockchip_sound_jack.jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key(rockchip_sound_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key(rockchip_sound_jack.jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    snd_soc_component_set_jack(component, &mut rockchip_sound_jack, null_mut());
    0
}

unsafe extern "C" fn rockchip_sound_dmic_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mclk = params_rate(params).wrapping_mul(SOUND_FS);
    let ret = snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), 0, mclk, 0);
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"%s() error setting sysclk to %u: %d\n\0".as_ptr() as *const c_char,
            b"rockchip_sound_dmic_hw_params\0".as_ptr() as *const c_char,
            mclk,
            ret,
        );
        return ret;
    }

    /* Wait for DMIC stable */
    msleep(dmic_wakeup_delay);
    0
}

unsafe extern "C" fn rockchip_sound_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE;
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_RATE, 8000, 96000)
}

static rockchip_sound_max98357a_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(rockchip_sound_startup),
    hw_params: Some(rockchip_sound_max98357a_hw_params),
};

static rockchip_sound_rt5514_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(rockchip_sound_startup),
    hw_params: Some(rockchip_sound_rt5514_hw_params),
};

static rockchip_sound_da7219_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(rockchip_sound_startup),
    hw_params: Some(rockchip_sound_da7219_hw_params),
};

static rockchip_sound_dmic_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(rockchip_sound_startup),
    hw_params: Some(rockchip_sound_dmic_hw_params),
};

static mut rockchip_sound_card: snd_soc_card = snd_soc_card {
    name: b"rk3399-gru-sound\0".as_ptr() as *const c_char,
    owner: null_mut(),
    dev: null_mut(),
    dai_link: null_mut(),
    num_links: 0,
    dapm_widgets: rockchip_dapm_widgets.as_ptr(),
    num_dapm_widgets: rockchip_dapm_widgets.len() as c_int,
    controls: rockchip_controls.as_ptr(),
    num_controls: rockchip_controls.len() as c_int,
    dapm_routes: null_mut(),
    num_dapm_routes: 0,
};

const DAILINK_CDNDP: usize = 0;
const DAILINK_DA7219: usize = 1;
const DAILINK_DMIC: usize = 2;
const DAILINK_MAX98357A: usize = 3;
const DAILINK_RT5514: usize = 4;
const DAILINK_RT5514_DSP: usize = 5;

static mut cdndp_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];
static mut cdndp_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: b"spdif-hifi\0".as_ptr() as *const c_char, of_node: null_mut() }];
static mut cdndp_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];

static mut da7219_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];
static mut da7219_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: b"da7219-hifi\0".as_ptr() as *const c_char, of_node: null_mut() }];
static mut da7219_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];

static mut dmic_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];
static mut dmic_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: b"dmic-hifi\0".as_ptr() as *const c_char, of_node: null_mut() }];
static mut dmic_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];

static mut max98357a_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];
static mut max98357a_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: b"HiFi\0".as_ptr() as *const c_char, of_node: null_mut() }];
static mut max98357a_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];

static mut rt5514_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];
static mut rt5514_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: b"rt5514-aif1\0".as_ptr() as *const c_char, of_node: null_mut() }];
static mut rt5514_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];

static mut rt5514_dsp_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];
static mut rt5514_dsp_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];
static mut rt5514_dsp_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: null(), dai_name: null(), of_node: null_mut() }];

static mut rockchip_dais: [snd_soc_dai_link; 6] = [
    snd_soc_dai_link { name: b"DP\0".as_ptr() as *const c_char, stream_name: b"DP PCM\0".as_ptr() as *const c_char, init: Some(rockchip_sound_cdndp_init), ops: null(), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, cpus: cdndp_cpus.as_mut_ptr(), num_cpus: 1, codecs: cdndp_codecs.as_mut_ptr(), num_codecs: 1, platforms: cdndp_platforms.as_mut_ptr(), num_platforms: 1 },
    /* set da7219 as slave */
    snd_soc_dai_link { name: b"DA7219\0".as_ptr() as *const c_char, stream_name: b"DA7219 PCM\0".as_ptr() as *const c_char, init: Some(rockchip_sound_da7219_init), ops: &rockchip_sound_da7219_ops, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, cpus: da7219_cpus.as_mut_ptr(), num_cpus: 1, codecs: da7219_codecs.as_mut_ptr(), num_codecs: 1, platforms: da7219_platforms.as_mut_ptr(), num_platforms: 1 },
    snd_soc_dai_link { name: b"DMIC\0".as_ptr() as *const c_char, stream_name: b"DMIC PCM\0".as_ptr() as *const c_char, init: None, ops: &rockchip_sound_dmic_ops, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, cpus: dmic_cpus.as_mut_ptr(), num_cpus: 1, codecs: dmic_codecs.as_mut_ptr(), num_codecs: 1, platforms: dmic_platforms.as_mut_ptr(), num_platforms: 1 },
    /* set max98357a as slave */
    snd_soc_dai_link { name: b"MAX98357A\0".as_ptr() as *const c_char, stream_name: b"MAX98357A PCM\0".as_ptr() as *const c_char, init: None, ops: &rockchip_sound_max98357a_ops, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, cpus: max98357a_cpus.as_mut_ptr(), num_cpus: 1, codecs: max98357a_codecs.as_mut_ptr(), num_codecs: 1, platforms: max98357a_platforms.as_mut_ptr(), num_platforms: 1 },
    /* set rt5514 as slave */
    snd_soc_dai_link { name: b"RT5514\0".as_ptr() as *const c_char, stream_name: b"RT5514 PCM\0".as_ptr() as *const c_char, init: None, ops: &rockchip_sound_rt5514_ops, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, cpus: rt5514_cpus.as_mut_ptr(), num_cpus: 1, codecs: rt5514_codecs.as_mut_ptr(), num_codecs: 1, platforms: rt5514_platforms.as_mut_ptr(), num_platforms: 1 },
    /* RT5514 DSP for voice wakeup via spi bus */
    snd_soc_dai_link { name: b"RT5514 DSP\0".as_ptr() as *const c_char, stream_name: b"Wake on Voice\0".as_ptr() as *const c_char, init: None, ops: null(), dai_fmt: 0, cpus: rt5514_dsp_cpus.as_mut_ptr(), num_cpus: 1, codecs: rt5514_dsp_codecs.as_mut_ptr(), num_codecs: 1, platforms: rt5514_dsp_platforms.as_mut_ptr(), num_platforms: 1 },
];

static rockchip_sound_cdndp_routes: [snd_soc_dapm_route; 1] = [
    /* Output */
    snd_soc_dapm_route { sink: b"HDMI\0".as_ptr() as *const c_char, control: null(), source: b"TX\0".as_ptr() as *const c_char },
];

static rockchip_sound_da7219_routes: [snd_soc_dapm_route; 3] = [
    /* Output */
    snd_soc_dapm_route { sink: b"Headphones\0".as_ptr() as *const c_char, control: null(), source: b"HPL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphones\0".as_ptr() as *const c_char, control: null(), source: b"HPR\0".as_ptr() as *const c_char },
    /* Input */
    snd_soc_dapm_route { sink: b"MIC\0".as_ptr() as *const c_char, control: null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
];

static rockchip_sound_dmic_routes: [snd_soc_dapm_route; 1] = [
    /* Input */
    snd_soc_dapm_route { sink: b"DMic\0".as_ptr() as *const c_char, control: null(), source: b"Int Mic\0".as_ptr() as *const c_char },
];

static rockchip_sound_max98357a_routes: [snd_soc_dapm_route; 1] = [
    /* Output */
    snd_soc_dapm_route { sink: b"Speakers\0".as_ptr() as *const c_char, control: null(), source: b"Speaker\0".as_ptr() as *const c_char },
];

static rockchip_sound_rt5514_routes: [snd_soc_dapm_route; 2] = [
    /* Input */
    snd_soc_dapm_route { sink: b"DMIC1L\0".as_ptr() as *const c_char, control: null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC1R\0".as_ptr() as *const c_char, control: null(), source: b"Int Mic\0".as_ptr() as *const c_char },
];

#[repr(C)]
struct rockchip_sound_route {
    routes: *const snd_soc_dapm_route,
    num_routes: c_int,
}

static rockchip_routes: [rockchip_sound_route; 6] = [
    rockchip_sound_route { routes: rockchip_sound_cdndp_routes.as_ptr(), num_routes: rockchip_sound_cdndp_routes.len() as c_int },
    rockchip_sound_route { routes: rockchip_sound_da7219_routes.as_ptr(), num_routes: rockchip_sound_da7219_routes.len() as c_int },
    rockchip_sound_route { routes: rockchip_sound_dmic_routes.as_ptr(), num_routes: rockchip_sound_dmic_routes.len() as c_int },
    rockchip_sound_route { routes: rockchip_sound_max98357a_routes.as_ptr(), num_routes: rockchip_sound_max98357a_routes.len() as c_int },
    rockchip_sound_route { routes: rockchip_sound_rt5514_routes.as_ptr(), num_routes: rockchip_sound_rt5514_routes.len() as c_int },
    rockchip_sound_route { routes: null(), num_routes: 0 },
];

#[repr(C)]
struct dailink_match_data {
    compatible: *const c_char,
    bus_type: *const bus_type,
}

static dailink_match: [dailink_match_data; 6] = [
    dailink_match_data { compatible: b"rockchip,rk3399-cdn-dp\0".as_ptr() as *const c_char, bus_type: null() },
    dailink_match_data { compatible: b"dlg,da7219\0".as_ptr() as *const c_char, bus_type: null() },
    dailink_match_data { compatible: b"dmic-codec\0".as_ptr() as *const c_char, bus_type: null() },
    dailink_match_data { compatible: b"maxim,max98357a\0".as_ptr() as *const c_char, bus_type: null() },
    dailink_match_data { compatible: b"realtek,rt5514\0".as_ptr() as *const c_char, bus_type: unsafe { &i2c_bus_type } },
    dailink_match_data { compatible: b"realtek,rt5514\0".as_ptr() as *const c_char, bus_type: unsafe { &spi_bus_type } },
];

unsafe extern "C" fn rockchip_sound_codec_node_match(np_codec: *mut device_node) -> c_int {
    for i in 0..dailink_match.len() {
        if of_device_is_compatible(np_codec, dailink_match[i].compatible) == 0 {
            continue;
        }

        if !dailink_match[i].bus_type.is_null() {
            let dev = bus_find_device_by_of_node(dailink_match[i].bus_type, np_codec);
            if dev.is_null() {
                continue;
            }
            put_device(dev);
        }

        return i as c_int;
    }
    -1
}

unsafe extern "C" fn rockchip_sound_of_parse_dais(
    dev: *mut device,
    card: *mut snd_soc_card,
) -> c_int {
    let np_cpu0: *mut device_node;
    let np_cpu1: *mut device_node;
    let mut np_cpu: *mut device_node;
    let mut np_codec: *mut device_node;
    let mut dai: *mut snd_soc_dai_link;
    let routes: *mut snd_soc_dapm_route;
    let mut index: c_int;
    let mut num_routes: c_int;

    (*card).dai_link = devm_kzalloc(dev, size_of_val(&rockchip_dais), GFP_KERNEL)
        as *mut snd_soc_dai_link;
    if (*card).dai_link.is_null() {
        return -ENOMEM;
    }

    num_routes = 0;
    for i in 0..rockchip_routes.len() {
        num_routes += rockchip_routes[i].num_routes;
    }
    routes = devm_kcalloc(
        dev,
        num_routes as usize,
        size_of::<snd_soc_dapm_route>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dapm_route;
    if routes.is_null() {
        return -ENOMEM;
    }
    (*card).dapm_routes = routes;

    np_cpu0 = of_parse_phandle((*dev).of_node, b"rockchip,cpu\0".as_ptr() as *const c_char, 0);
    np_cpu1 = of_parse_phandle((*dev).of_node, b"rockchip,cpu\0".as_ptr() as *const c_char, 1);

    (*card).num_dapm_routes = 0;
    (*card).num_links = 0;
    for i in 0..rockchip_dais.len() {
        np_codec = of_parse_phandle(
            (*dev).of_node,
            b"rockchip,codec\0".as_ptr() as *const c_char,
            i as c_int,
        );
        if np_codec.is_null() {
            break;
        }

        if of_device_is_available(np_codec) == 0 {
            continue;
        }

        index = rockchip_sound_codec_node_match(np_codec);
        if index < 0 {
            continue;
        }

        match index as usize {
            DAILINK_CDNDP => np_cpu = np_cpu1,
            DAILINK_RT5514_DSP => np_cpu = np_codec,
            _ => np_cpu = np_cpu0,
        }

        if np_cpu.is_null() {
            dev_err(
                dev,
                b"Missing 'rockchip,cpu' for %s\n\0".as_ptr() as *const c_char,
                rockchip_dais[index as usize].name,
            );
            return -EINVAL;
        }

        dai = (*card).dai_link.add((*card).num_links as usize);
        (*card).num_links += 1;
        *dai = rockchip_dais[index as usize];

        if (*(*dai).codecs).name.is_null() {
            (*(*dai).codecs).of_node = np_codec;
        }
        (*(*dai).platforms).of_node = np_cpu;
        (*(*dai).cpus).of_node = np_cpu;

        if (*card).num_dapm_routes + rockchip_routes[index as usize].num_routes > num_routes {
            dev_err(dev, b"Too many routes\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        copy_nonoverlapping(
            rockchip_routes[index as usize].routes,
            routes.add((*card).num_dapm_routes as usize),
            rockchip_routes[index as usize].num_routes as usize,
        );
        (*card).num_dapm_routes += rockchip_routes[index as usize].num_routes;
    }

    0
}

unsafe extern "C" fn rockchip_sound_probe(pdev: *mut platform_device) -> c_int {
    let card = &mut rockchip_sound_card as *mut snd_soc_card;

    let mut ret = rockchip_sound_of_parse_dais(&mut (*pdev).dev, card);
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Failed to parse dais: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* Set DMIC wakeup delay */
    ret = device_property_read_u32(
        &mut (*pdev).dev,
        b"dmic-wakeup-delay-ms\0".as_ptr() as *const c_char,
        &mut dmic_wakeup_delay,
    );
    if ret != 0 {
        dmic_wakeup_delay = 0;
        dev_dbg(
            &mut (*pdev).dev,
            b"no optional property 'dmic-wakeup-delay-ms' found, default: no delay\n\0".as_ptr() as *const c_char,
        );
    }

    (*card).dev = &mut (*pdev).dev;
    devm_snd_soc_register_card(&mut (*pdev).dev, card)
}

static rockchip_sound_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"rockchip,rk3399-gru-sound\0".as_ptr() as *const c_char },
    of_device_id { compatible: null() },
];
/* MODULE_DEVICE_TABLE(of, rockchip_sound_of_match); */

static mut rockchip_sound_driver: platform_driver = platform_driver {
    probe: Some(rockchip_sound_probe),
    driver: driver_private {
        name: DRV_NAME,
        of_match_table: rockchip_sound_of_match.as_ptr(),
        /* CONFIG_PM: .pm = &snd_soc_pm_ops */
    },
};

/* module_platform_driver(rockchip_sound_driver); */

/* MODULE_AUTHOR("Xing Zheng <zhengxing@rock-chips.com>"); */
/* MODULE_DESCRIPTION("Rockchip ASoC Machine Driver"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:" DRV_NAME); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
