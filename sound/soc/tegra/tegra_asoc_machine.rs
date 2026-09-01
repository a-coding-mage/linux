// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra_asoc_machine.c - Universal ASoC machine driver for NVIDIA Tegra boards.
 *
 * Rust translation of the isolated C implementation source. Linux kernel and
 * ASoC declarations/macros from included headers remain external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_int,
    pub debounce_time: c_int,
    pub desc: *mut gpio_desc,
    pub data: *mut c_void,
    pub jack_status_check: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub components: *const c_char,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub fully_routed: bool,
    pub dev: *mut device,
    pub owner: *mut module,
    pub driver_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device_registered {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct tegra_asoc_data {
    pub mclk_rate: Option<unsafe extern "C" fn(c_uint) -> c_uint>,
    pub card: *mut snd_soc_card,
    pub hp_jack_name: *const c_char,
    pub add_common_dapm_widgets: bool,
    pub add_common_controls: bool,
    pub add_common_snd_ops: bool,
    pub add_mic_jack: bool,
    pub add_hp_jack: bool,
    pub add_headset_jack: bool,
    pub codec_dev_name: *const c_char,
    pub set_ac97: bool,
    pub mclk_id: c_uint,
}

#[repr(C)]
pub struct tegra_machine {
    pub asoc: *const tegra_asoc_data,
    pub mic_jack: *mut snd_soc_jack,
    pub hp_jack_gpio: *mut snd_soc_jack_gpio,
    pub gpiod_hp_mute: *mut gpio_desc,
    pub gpiod_hp_det: *mut gpio_desc,
    pub gpiod_mic_det: *mut gpio_desc,
    pub gpiod_spkr_en: *mut gpio_desc,
    pub gpiod_int_mic_en: *mut gpio_desc,
    pub gpiod_ext_mic_en: *mut gpio_desc,
    pub clk_pll_a: *mut clk,
    pub clk_pll_a_out0: *mut clk,
    pub clk_cdev1: *mut clk,
    pub set_baseclock: c_uint,
    pub set_mclk: c_uint,
}

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_SOC_CLOCK_IN: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_int = 1;
const GPIOD_IN: c_int = 2;
const GPIOD_OUT_LOW: c_int = 3;
const SND_SOC_DAIFMT_I2S: c_uint = 1 << 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 1 << 1;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 1 << 2;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 1 << 3;

extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: c_void;

    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, name: *const c_char) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_jack_add_gpios(
        jack: *mut snd_soc_jack,
        count: c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn of_machine_is_compatible(compat: *const c_char) -> bool;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_uint,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn of_node_put(of_node: *mut c_void);
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device_registered);
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *mut c_void,
        num: c_uint,
    ) -> *mut platform_device_registered;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_force_enable_pin(
        dapm: *mut snd_soc_dapm_context,
        pin: *const c_char,
    ) -> c_int;
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> isize {
    ptr as isize
}

unsafe fn ERR_PTR<T>(err: isize) -> *mut T {
    err as *mut T
}

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> c_int {
    (event != 0) as c_int
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

/* Headphones Jack */

static mut tegra_machine_hp_jack: snd_soc_jack = snd_soc_jack { _private: [] };

static mut tegra_machine_hp_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: cstr!("Headphone"),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: cstr!("Headphones"),
        mask: SND_JACK_HEADPHONE,
    },
];

static mut tegra_machine_hp_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: cstr!("Headphones detection"),
    report: SND_JACK_HEADPHONE,
    debounce_time: 150,
    desc: ptr::null_mut(),
    data: ptr::null_mut(),
    jack_status_check: None,
};

/* Headset Jack */

static mut tegra_machine_headset_jack: snd_soc_jack = snd_soc_jack { _private: [] };

static mut tegra_machine_headset_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: cstr!("Headset Mic"),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: cstr!("Headset Stereophone"),
        mask: SND_JACK_HEADPHONE,
    },
];

static mut tegra_machine_headset_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: cstr!("Headset detection"),
    report: SND_JACK_HEADSET,
    debounce_time: 150,
    desc: ptr::null_mut(),
    data: ptr::null_mut(),
    jack_status_check: None,
};

/* Mic Jack */
unsafe extern "C" fn coupled_mic_hp_check(data: *mut c_void) -> c_int {
    let machine = data as *mut tegra_machine;

    /* Detect mic insertion only if 3.5 jack is in */
    if gpiod_get_value_cansleep((*machine).gpiod_hp_det) != 0
        && gpiod_get_value_cansleep((*machine).gpiod_mic_det) != 0
    {
        return SND_JACK_MICROPHONE;
    }

    0
}

static mut tegra_machine_mic_jack: snd_soc_jack = snd_soc_jack { _private: [] };

static mut tegra_machine_mic_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: cstr!("Mic Jack"),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: cstr!("Headset Mic"),
        mask: SND_JACK_MICROPHONE,
    },
];

static mut tegra_machine_mic_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: cstr!("Mic detection"),
    report: SND_JACK_MICROPHONE,
    debounce_time: 150,
    desc: ptr::null_mut(),
    data: ptr::null_mut(),
    jack_status_check: None,
};

unsafe extern "C" fn tegra_machine_event(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let machine = snd_soc_card_get_drvdata(card) as *mut tegra_machine;

    if snd_soc_dapm_widget_name_cmp(w, cstr!("Int Spk")) == 0
        || snd_soc_dapm_widget_name_cmp(w, cstr!("Speakers")) == 0
    {
        gpiod_set_value_cansleep((*machine).gpiod_spkr_en, SND_SOC_DAPM_EVENT_ON(event));
    }

    if snd_soc_dapm_widget_name_cmp(w, cstr!("Mic Jack")) == 0
        || snd_soc_dapm_widget_name_cmp(w, cstr!("Headset Mic")) == 0
    {
        gpiod_set_value_cansleep((*machine).gpiod_ext_mic_en, SND_SOC_DAPM_EVENT_ON(event));
    }

    if snd_soc_dapm_widget_name_cmp(w, cstr!("Int Mic")) == 0
        || snd_soc_dapm_widget_name_cmp(w, cstr!("Internal Mic 2")) == 0
    {
        gpiod_set_value_cansleep((*machine).gpiod_int_mic_en, SND_SOC_DAPM_EVENT_ON(event));
    }

    if snd_soc_dapm_widget_name_cmp(w, cstr!("Headphone")) == 0
        || snd_soc_dapm_widget_name_cmp(w, cstr!("Headphone Jack")) == 0
    {
        gpiod_set_value_cansleep(
            (*machine).gpiod_hp_mute,
            (SND_SOC_DAPM_EVENT_ON(event) == 0) as c_int,
        );
    }

    0
}

/* SND_SOC_DAPM_* and SOC_DAPM_PIN_SWITCH initializers are header macros. */
static tegra_machine_dapm_widgets: [snd_soc_dapm_widget; 18] =
    [snd_soc_dapm_widget { dapm: ptr::null_mut() }; 18];

static tegra_machine_controls: [snd_kcontrol_new; 9] =
    [snd_kcontrol_new { _private: [] }; 9];

#[no_mangle]
pub unsafe extern "C" fn tegra_asoc_machine_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let machine = snd_soc_card_get_drvdata(card) as *mut tegra_machine;
    let jack_name: *const c_char;
    let mut err: c_int;

    if !(*machine).gpiod_hp_det.is_null() && (*(*machine).asoc).add_hp_jack {
        if !(*(*machine).asoc).hp_jack_name.is_null() {
            jack_name = (*(*machine).asoc).hp_jack_name;
        } else {
            jack_name = cstr!("Headphones Jack");
        }

        err = snd_soc_card_jack_new_pins(
            card,
            jack_name,
            SND_JACK_HEADPHONE,
            &mut tegra_machine_hp_jack,
            tegra_machine_hp_jack_pins.as_mut_ptr(),
            array_size(&tegra_machine_hp_jack_pins),
        );
        if err != 0 {
            dev_err(
                (*rtd).dev,
                cstr!("Headphones Jack creation failed: %d\n"),
                err,
            );
            return err;
        }

        tegra_machine_hp_jack_gpio.desc = (*machine).gpiod_hp_det;

        err = snd_soc_jack_add_gpios(
            &mut tegra_machine_hp_jack,
            1,
            &mut tegra_machine_hp_jack_gpio,
        );
        if err != 0 {
            dev_err((*rtd).dev, cstr!("HP GPIOs not added: %d\n"), err);
        }
    }

    if !(*machine).gpiod_hp_det.is_null() && (*(*machine).asoc).add_headset_jack {
        err = snd_soc_card_jack_new_pins(
            card,
            cstr!("Headset Jack"),
            SND_JACK_HEADSET,
            &mut tegra_machine_headset_jack,
            tegra_machine_headset_jack_pins.as_mut_ptr(),
            array_size(&tegra_machine_headset_jack_pins),
        );
        if err != 0 {
            dev_err((*rtd).dev, cstr!("Headset Jack creation failed: %d\n"), err);
            return err;
        }

        tegra_machine_headset_jack_gpio.desc = (*machine).gpiod_hp_det;

        err = snd_soc_jack_add_gpios(
            &mut tegra_machine_headset_jack,
            1,
            &mut tegra_machine_headset_jack_gpio,
        );
        if err != 0 {
            dev_err((*rtd).dev, cstr!("Headset GPIOs not added: %d\n"), err);
        }
    }

    if !(*machine).gpiod_mic_det.is_null() && (*(*machine).asoc).add_mic_jack {
        err = snd_soc_card_jack_new_pins(
            (*rtd).card,
            cstr!("Mic Jack"),
            SND_JACK_MICROPHONE,
            &mut tegra_machine_mic_jack,
            tegra_machine_mic_jack_pins.as_mut_ptr(),
            array_size(&tegra_machine_mic_jack_pins),
        );
        if err != 0 {
            dev_err((*rtd).dev, cstr!("Mic Jack creation failed: %d\n"), err);
            return err;
        }

        tegra_machine_mic_jack_gpio.data = machine as *mut c_void;
        tegra_machine_mic_jack_gpio.desc = (*machine).gpiod_mic_det;

        if of_property_read_bool((*card).dev.as_ref().unwrap().of_node, cstr!("nvidia,coupled-mic-hp-det")) {
            tegra_machine_mic_jack_gpio.desc = (*machine).gpiod_hp_det;
            tegra_machine_mic_jack_gpio.jack_status_check = Some(coupled_mic_hp_check);
        }

        err = snd_soc_jack_add_gpios(
            &mut tegra_machine_mic_jack,
            1,
            &mut tegra_machine_mic_jack_gpio,
        );
        if err != 0 {
            dev_err((*rtd).dev, cstr!("Mic GPIOs not added: %d\n"), err);
        }
    }

    0
}

unsafe extern "C" fn tegra_machine_mclk_rate_128(srate: c_uint) -> c_uint {
    128u32.wrapping_mul(srate)
}

unsafe extern "C" fn tegra_machine_mclk_rate_256(srate: c_uint) -> c_uint {
    256u32.wrapping_mul(srate)
}

unsafe extern "C" fn tegra_machine_mclk_rate_512(srate: c_uint) -> c_uint {
    512u32.wrapping_mul(srate)
}

unsafe extern "C" fn tegra_machine_mclk_rate_12mhz(srate: c_uint) -> c_uint {
    let mclk: c_uint;

    match srate {
        8000 | 16000 | 24000 | 32000 | 48000 | 64000 | 96000 => {
            mclk = 12288000;
        }
        11025 | 22050 | 44100 | 88200 => {
            mclk = 11289600;
        }
        _ => {
            mclk = 12000000;
        }
    }

    mclk
}

unsafe extern "C" fn tegra_machine_mclk_rate_6mhz(srate: c_uint) -> c_uint {
    let mclk: c_uint;

    match srate {
        8000 | 16000 | 64000 => {
            mclk = 8192000;
        }
        11025 | 22050 | 88200 => {
            mclk = 11289600;
        }
        96000 => {
            mclk = 12288000;
        }
        _ => {
            mclk = 256u32.wrapping_mul(srate);
        }
    }

    mclk
}

unsafe extern "C" fn tegra_machine_mclk_rate_cpcap(srate: c_uint) -> c_uint {
    let mclk: c_uint;

    match srate {
        11025 | 22050 | 44100 | 88200 => {
            mclk = 26000000;
        }
        _ => {
            mclk = 256u32.wrapping_mul(srate);
        }
    }

    mclk
}

unsafe extern "C" fn tegra_machine_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let card = (*rtd).card;
    let machine = snd_soc_card_get_drvdata(card) as *mut tegra_machine;
    let srate = params_rate(params);
    let mclk = ((*(*machine).asoc).mclk_rate.unwrap())(srate);
    let clk_id = (*(*machine).asoc).mclk_id;
    let new_baseclock: c_uint;
    let mut err: c_int;

    match srate {
        11025 | 22050 | 44100 | 88200 => {
            if of_machine_is_compatible(cstr!("nvidia,tegra20")) {
                new_baseclock = 56448000;
            } else if of_machine_is_compatible(cstr!("nvidia,tegra30")) {
                new_baseclock = 564480000;
            } else {
                new_baseclock = 282240000;
            }
        }
        8000 | 16000 | 32000 | 48000 | 64000 | 96000 => {
            if of_machine_is_compatible(cstr!("nvidia,tegra20")) {
                new_baseclock = 73728000;
            } else if of_machine_is_compatible(cstr!("nvidia,tegra30")) {
                new_baseclock = 552960000;
            } else {
                new_baseclock = 368640000;
            }
        }
        _ => {
            dev_err((*card).dev, cstr!("Invalid sound rate: %u\n"), srate);
            return -EINVAL;
        }
    }

    if new_baseclock != (*machine).set_baseclock || mclk != (*machine).set_mclk {
        (*machine).set_baseclock = 0;
        (*machine).set_mclk = 0;

        clk_disable_unprepare((*machine).clk_cdev1);

        err = clk_set_rate((*machine).clk_pll_a, new_baseclock);
        if err != 0 {
            dev_err((*card).dev, cstr!("Can't set pll_a rate: %d\n"), err);
            return err;
        }

        err = clk_set_rate((*machine).clk_pll_a_out0, mclk);
        if err != 0 {
            dev_err((*card).dev, cstr!("Can't set pll_a_out0 rate: %d\n"), err);
            return err;
        }

        /* Don't set cdev1/extern1 rate; it's locked to pll_a_out0 */

        err = clk_prepare_enable((*machine).clk_cdev1);
        if err != 0 {
            dev_err((*card).dev, cstr!("Can't enable cdev1: %d\n"), err);
            return err;
        }

        (*machine).set_baseclock = new_baseclock;
        (*machine).set_mclk = mclk;
    }

    err = snd_soc_dai_set_sysclk(codec_dai, clk_id, mclk, SND_SOC_CLOCK_IN);
    if err < 0 {
        dev_err((*card).dev, cstr!("codec_dai clock not set: %d\n"), err);
        return err;
    }

    0
}

static tegra_machine_snd_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(tegra_machine_hw_params),
};

unsafe extern "C" fn tegra_machine_node_release(of_node: *mut c_void) {
    of_node_put(of_node);
}

unsafe fn tegra_machine_parse_phandle(dev: *mut device, name: *const c_char) -> *mut device_node {
    let np: *mut device_node;
    let err: c_int;

    np = of_parse_phandle((*dev).of_node, name, 0);
    if np.is_null() {
        dev_err(dev, cstr!("Property '%s' missing or invalid\n"), name);
        return ERR_PTR(-EINVAL as isize);
    }

    err = devm_add_action_or_reset(dev, tegra_machine_node_release, np as *mut c_void);
    if err != 0 {
        return ERR_PTR(err as isize);
    }

    np
}

unsafe extern "C" fn tegra_machine_unregister_codec(pdev: *mut c_void) {
    platform_device_unregister(pdev as *mut platform_device_registered);
}

unsafe fn tegra_machine_register_codec(dev: *mut device, name: *const c_char) -> c_int {
    let pdev: *mut platform_device_registered;
    let err: c_int;

    if name.is_null() {
        return 0;
    }

    pdev = platform_device_register_simple(name, -1, ptr::null_mut(), 0);
    if IS_ERR(pdev) {
        return dev_err_probe(dev, PTR_ERR(pdev), cstr!("failed to register codec %s\n"), name);
    }

    err = devm_add_action_or_reset(dev, tegra_machine_unregister_codec, pdev as *mut c_void);
    if err != 0 {
        return err;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tegra_asoc_machine_probe(pdev: *mut platform_device) -> c_int {
    let np_codec: *mut device_node;
    let np_i2s: *mut device_node;
    let np_ac97: *mut device_node;
    let asoc: *const tegra_asoc_data;
    let dev = &mut (*pdev).dev as *mut device;
    let machine: *mut tegra_machine;
    let card: *mut snd_soc_card;
    let mut gpiod: *mut gpio_desc;
    let mut err: c_int;

    machine = devm_kzalloc(dev, core::mem::size_of::<tegra_machine>(), GFP_KERNEL) as *mut tegra_machine;
    if machine.is_null() {
        return -ENOMEM;
    }

    asoc = of_device_get_match_data(dev) as *const tegra_asoc_data;
    card = (*asoc).card;
    (*card).dev = dev;

    (*machine).asoc = asoc;
    (*machine).mic_jack = &mut tegra_machine_mic_jack;
    (*machine).hp_jack_gpio = &mut tegra_machine_hp_jack_gpio;
    snd_soc_card_set_drvdata(card, machine as *mut c_void);

    gpiod = devm_gpiod_get_optional(dev, cstr!("nvidia,hp-mute"), GPIOD_OUT_HIGH);
    (*machine).gpiod_hp_mute = gpiod;
    if IS_ERR(gpiod) {
        return dev_err_probe(dev, PTR_ERR(gpiod), cstr!("failed to get hp-mute GPIO\n"));
    }

    gpiod = devm_gpiod_get_optional(dev, cstr!("nvidia,hp-det"), GPIOD_IN);
    (*machine).gpiod_hp_det = gpiod;
    if IS_ERR(gpiod) {
        return dev_err_probe(dev, PTR_ERR(gpiod), cstr!("failed to get hp-det GPIO\n"));
    }

    gpiod = devm_gpiod_get_optional(dev, cstr!("nvidia,mic-det"), GPIOD_IN);
    (*machine).gpiod_mic_det = gpiod;
    if IS_ERR(gpiod) {
        return dev_err_probe(dev, PTR_ERR(gpiod), cstr!("failed to get mic-det GPIO\n"));
    }

    gpiod = devm_gpiod_get_optional(dev, cstr!("nvidia,spkr-en"), GPIOD_OUT_LOW);
    (*machine).gpiod_spkr_en = gpiod;
    if IS_ERR(gpiod) {
        return dev_err_probe(dev, PTR_ERR(gpiod), cstr!("failed to get spkr-en GPIO\n"));
    }

    gpiod = devm_gpiod_get_optional(dev, cstr!("nvidia,int-mic-en"), GPIOD_OUT_LOW);
    (*machine).gpiod_int_mic_en = gpiod;
    if IS_ERR(gpiod) {
        return dev_err_probe(dev, PTR_ERR(gpiod), cstr!("failed to get int-mic-en GPIO\n"));
    }

    gpiod = devm_gpiod_get_optional(dev, cstr!("nvidia,ext-mic-en"), GPIOD_OUT_LOW);
    (*machine).gpiod_ext_mic_en = gpiod;
    if IS_ERR(gpiod) {
        return dev_err_probe(dev, PTR_ERR(gpiod), cstr!("failed to get ext-mic-en GPIO\n"));
    }

    err = snd_soc_of_parse_card_name(card, cstr!("nvidia,model"));
    if err != 0 {
        return err;
    }

    if (*card).dapm_routes.is_null() {
        err = snd_soc_of_parse_audio_routing(card, cstr!("nvidia,audio-routing"));
        if err != 0 {
            return err;
        }
    }

    if (*asoc).set_ac97 {
        err = tegra_machine_register_codec(dev, (*asoc).codec_dev_name);
        if err != 0 {
            return err;
        }

        np_ac97 = tegra_machine_parse_phandle(dev, cstr!("nvidia,ac97-controller"));
        if IS_ERR(np_ac97) {
            return PTR_ERR(np_ac97) as c_int;
        }

        (*(*card).dai_link).cpus.as_mut().unwrap().of_node = np_ac97;
        (*(*card).dai_link).platforms.as_mut().unwrap().of_node = np_ac97;
    } else {
        np_codec = tegra_machine_parse_phandle(dev, cstr!("nvidia,audio-codec"));
        if IS_ERR(np_codec) {
            return PTR_ERR(np_codec) as c_int;
        }

        np_i2s = tegra_machine_parse_phandle(dev, cstr!("nvidia,i2s-controller"));
        if IS_ERR(np_i2s) {
            return PTR_ERR(np_i2s) as c_int;
        }

        (*(*card).dai_link).cpus.as_mut().unwrap().of_node = np_i2s;
        (*(*card).dai_link).codecs.as_mut().unwrap().of_node = np_codec;
        (*(*card).dai_link).platforms.as_mut().unwrap().of_node = np_i2s;
    }

    if (*asoc).add_common_controls {
        (*card).controls = tegra_machine_controls.as_ptr();
        (*card).num_controls = array_size(&tegra_machine_controls) as c_int;
    }

    if (*asoc).add_common_dapm_widgets {
        (*card).dapm_widgets = tegra_machine_dapm_widgets.as_ptr();
        (*card).num_dapm_widgets = array_size(&tegra_machine_dapm_widgets) as c_int;
    }

    if (*asoc).add_common_snd_ops {
        (*(*card).dai_link).ops = &tegra_machine_snd_ops;
    }

    if (*card).owner.is_null() {
        (*card).owner = THIS_MODULE;
    }
    if (*card).driver_name.is_null() {
        (*card).driver_name = cstr!("tegra");
    }

    (*machine).clk_pll_a = devm_clk_get(dev, cstr!("pll_a"));
    if IS_ERR((*machine).clk_pll_a) {
        return dev_err_probe(dev, PTR_ERR((*machine).clk_pll_a), cstr!("can't retrieve clk pll_a\n"));
    }

    (*machine).clk_pll_a_out0 = devm_clk_get(dev, cstr!("pll_a_out0"));
    if IS_ERR((*machine).clk_pll_a_out0) {
        return dev_err_probe(dev, PTR_ERR((*machine).clk_pll_a_out0), cstr!("can't retrieve clk pll_a_out0\n"));
    }

    (*machine).clk_cdev1 = devm_clk_get(dev, cstr!("mclk"));
    if IS_ERR((*machine).clk_cdev1) {
        return dev_err_probe(dev, PTR_ERR((*machine).clk_cdev1), cstr!("can't retrieve clk cdev1\n"));
    }

    /*
     * If clock parents are not set in DT, configure here to use clk_out_1
     * as mclk and extern1 as parent for Tegra30 and higher.
     */
    if !of_property_present((*dev).of_node, cstr!("assigned-clock-parents"))
        && !of_machine_is_compatible(cstr!("nvidia,tegra20"))
    {
        let clk_out_1: *mut clk;
        let clk_extern1: *mut clk;

        dev_warn(dev, cstr!("Configuring clocks for a legacy device-tree\n"));
        dev_warn(dev, cstr!("Please update DT to use assigned-clock-parents\n"));

        clk_extern1 = devm_clk_get(dev, cstr!("extern1"));
        if IS_ERR(clk_extern1) {
            return dev_err_probe(dev, PTR_ERR(clk_extern1), cstr!("can't retrieve clk extern1\n"));
        }

        err = clk_set_parent(clk_extern1, (*machine).clk_pll_a_out0);
        if err < 0 {
            return dev_err_probe(dev, err as isize, cstr!("set parent failed for clk extern1\n"));
        }

        clk_out_1 = devm_clk_get(dev, cstr!("pmc_clk_out_1"));
        if IS_ERR(clk_out_1) {
            return dev_err_probe(dev, PTR_ERR(clk_out_1), cstr!("can't retrieve pmc_clk_out_1\n"));
        }

        err = clk_set_parent(clk_out_1, clk_extern1);
        if err < 0 {
            return dev_err_probe(dev, err as isize, cstr!("set parent failed for pmc_clk_out_1\n"));
        }

        (*machine).clk_cdev1 = clk_out_1;
    }

    if (*asoc).set_ac97 {
        /*
         * AC97 rate is fixed at 24.576MHz and is used for both the
         * host controller and the external codec
         */
        err = clk_set_rate((*machine).clk_pll_a, 73728000);
        if err != 0 {
            return dev_err_probe(dev, err as isize, cstr!("can't set pll_a rate\n"));
        }

        err = clk_set_rate((*machine).clk_pll_a_out0, 24576000);
        if err != 0 {
            return dev_err_probe(dev, err as isize, cstr!("can't set pll_a_out0 rate\n"));
        }

        (*machine).set_baseclock = 73728000;
        (*machine).set_mclk = 24576000;
    }

    /*
     * FIXME: There is some unknown dependency between audio MCLK disable
     * and suspend-resume functionality on Tegra30, although audio MCLK is
     * only needed for audio.
     */
    err = clk_prepare_enable((*machine).clk_cdev1);
    if err != 0 {
        return dev_err_probe(dev, err as isize, cstr!("can't enable cdev1\n"));
    }

    err = devm_snd_soc_register_card(dev, card);
    if err != 0 {
        return err;
    }

    0
}

/* DAI link component definitions generated by SND_SOC_DAILINK_DEFS are external macro data in C. */
macro_rules! empty_dai_link {
    ($name:literal, $stream:literal, $init:expr, $fmt:expr) => {
        snd_soc_dai_link {
            name: cstr!($name),
            stream_name: cstr!($stream),
            init: $init,
            dai_fmt: $fmt,
            cpus: ptr::null_mut(),
            codecs: ptr::null_mut(),
            platforms: ptr::null_mut(),
            ops: ptr::null(),
        }
    };
}

/* WM8753 machine */
static mut tegra_wm8753_dai: snd_soc_dai_link = empty_dai_link!(
    "WM8753",
    "WM8753 PCM",
    None,
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_wm8753: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:wm8753"),
    dai_link: unsafe { &mut tegra_wm8753_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_wm8753_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_12mhz),
    card: unsafe { &mut snd_soc_tegra_wm8753 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: false,
    add_common_snd_ops: true,
    add_mic_jack: false,
    add_hp_jack: false,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* WM9712 machine */
unsafe extern "C" fn tegra_wm9712_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dapm = snd_soc_card_to_dapm((*rtd).card);
    snd_soc_dapm_force_enable_pin(dapm, cstr!("Mic Bias"))
}

static mut tegra_wm9712_dai: snd_soc_dai_link =
    empty_dai_link!("AC97 HiFi", "AC97 HiFi", Some(tegra_wm9712_init), 0);

static mut snd_soc_tegra_wm9712: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:wm9712"),
    dai_link: unsafe { &mut tegra_wm9712_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_wm9712_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: None,
    card: unsafe { &mut snd_soc_tegra_wm9712 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: false,
    add_common_snd_ops: false,
    add_mic_jack: false,
    add_hp_jack: false,
    add_headset_jack: false,
    codec_dev_name: cstr!("wm9712-codec"),
    set_ac97: true,
    mclk_id: 0,
};

/* MAX98090 machine */
static mut tegra_max98090_dai: snd_soc_dai_link = empty_dai_link!(
    "max98090",
    "max98090 PCM",
    Some(tegra_asoc_machine_init),
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_max98090: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:max98090"),
    dai_link: unsafe { &mut tegra_max98090_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_max98090_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_12mhz),
    card: unsafe { &mut snd_soc_tegra_max98090 },
    hp_jack_name: cstr!("Headphones"),
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: true,
    add_hp_jack: true,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* MAX98088 machine */
static mut tegra_max98088_dai: snd_soc_dai_link = empty_dai_link!(
    "MAX98088",
    "MAX98088 PCM",
    Some(tegra_asoc_machine_init),
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_max98088: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:max98088"),
    dai_link: unsafe { &mut tegra_max98088_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_max98088_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_12mhz),
    card: unsafe { &mut snd_soc_tegra_max98088 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: true,
    add_hp_jack: true,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* SGTL5000 machine */
static mut tegra_sgtl5000_dai: snd_soc_dai_link = empty_dai_link!(
    "sgtl5000",
    "HiFi",
    None,
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_sgtl5000: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:sgtl5000"),
    dai_link: unsafe { &mut tegra_sgtl5000_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_sgtl5000_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_12mhz),
    card: unsafe { &mut snd_soc_tegra_sgtl5000 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: false,
    add_common_snd_ops: true,
    add_mic_jack: false,
    add_hp_jack: false,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* TLV320AIC23 machine */
static trimslice_dapm_widgets: [snd_soc_dapm_widget; 2] =
    [snd_soc_dapm_widget { dapm: ptr::null_mut() }; 2];

static trimslice_audio_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: cstr!("Line Out"),
        control: ptr::null(),
        source: cstr!("LOUT"),
    },
    snd_soc_dapm_route {
        sink: cstr!("Line Out"),
        control: ptr::null(),
        source: cstr!("ROUT"),
    },
    snd_soc_dapm_route {
        sink: cstr!("LLINEIN"),
        control: ptr::null(),
        source: cstr!("Line In"),
    },
    snd_soc_dapm_route {
        sink: cstr!("RLINEIN"),
        control: ptr::null(),
        source: cstr!("Line In"),
    },
];

static mut tegra_tlv320aic23_dai: snd_soc_dai_link = empty_dai_link!(
    "TLV320AIC23",
    "AIC23",
    None,
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_trimslice: snd_soc_card = snd_soc_card {
    name: cstr!("tegra-trimslice"),
    components: cstr!("codec:tlv320aic23"),
    dai_link: unsafe { &mut tegra_tlv320aic23_dai },
    num_links: 1,
    dapm_widgets: trimslice_dapm_widgets.as_ptr(),
    num_dapm_widgets: 2,
    dapm_routes: trimslice_audio_map.as_ptr(),
    num_dapm_routes: 4,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_trimslice_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_128),
    card: unsafe { &mut snd_soc_tegra_trimslice },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: false,
    add_common_controls: false,
    add_common_snd_ops: true,
    add_mic_jack: false,
    add_hp_jack: false,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* RT5677 machine */
unsafe extern "C" fn tegra_rt5677_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let err: c_int;

    err = tegra_asoc_machine_init(rtd);
    if err != 0 {
        return err;
    }

    snd_soc_dapm_force_enable_pin(dapm, cstr!("MICBIAS1"));

    0
}

static mut tegra_rt5677_dai: snd_soc_dai_link = empty_dai_link!(
    "RT5677",
    "RT5677 PCM",
    Some(tegra_rt5677_init),
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_rt5677: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:rt5677"),
    dai_link: unsafe { &mut tegra_rt5677_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_rt5677_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_256),
    card: unsafe { &mut snd_soc_tegra_rt5677 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: true,
    add_hp_jack: true,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* RT5640 machine */
static mut tegra_rt5640_dai: snd_soc_dai_link = empty_dai_link!(
    "RT5640",
    "RT5640 PCM",
    Some(tegra_asoc_machine_init),
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_rt5640: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:rt5640"),
    dai_link: unsafe { &mut tegra_rt5640_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_rt5640_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_256),
    card: unsafe { &mut snd_soc_tegra_rt5640 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: false,
    add_hp_jack: true,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* RT5632 machine */
static mut tegra_rt5632_dai: snd_soc_dai_link = empty_dai_link!(
    "ALC5632",
    "ALC5632 PCM",
    Some(tegra_rt5677_init),
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_rt5632: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:rt5632"),
    dai_link: unsafe { &mut tegra_rt5632_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_rt5632_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_512),
    card: unsafe { &mut snd_soc_tegra_rt5632 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: false,
    add_hp_jack: false,
    add_headset_jack: true,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* RT5631 machine */
static mut tegra_rt5631_dai: snd_soc_dai_link = empty_dai_link!(
    "RT5631",
    "RT5631 PCM",
    Some(tegra_asoc_machine_init),
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
);

static mut snd_soc_tegra_rt5631: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:rt5631"),
    dai_link: unsafe { &mut tegra_rt5631_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_rt5631_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_6mhz),
    card: unsafe { &mut snd_soc_tegra_rt5631 },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: true,
    add_hp_jack: true,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

/* CPCAP machine */
static mut tegra_cpcap_dai: snd_soc_dai_link = empty_dai_link!(
    "CPCAP",
    "CPCAP PCM",
    Some(tegra_asoc_machine_init),
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP
);

static mut snd_soc_tegra_cpcap: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    components: cstr!("codec:cpcap"),
    dai_link: unsafe { &mut tegra_cpcap_dai },
    num_links: 1,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    fully_routed: true,
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    driver_name: ptr::null(),
};

static tegra_cpcap_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_machine_mclk_rate_cpcap),
    card: unsafe { &mut snd_soc_tegra_cpcap },
    hp_jack_name: ptr::null(),
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: false,
    add_hp_jack: false,
    add_headset_jack: false,
    codec_dev_name: ptr::null(),
    set_ac97: false,
    mclk_id: 0,
};

static tegra_machine_of_match: [of_device_id; 13] = [
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-trimslice"),
        data: unsafe { &tegra_trimslice_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-max98090"),
        data: unsafe { &tegra_max98090_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-max98088"),
        data: unsafe { &tegra_max98088_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-max98089"),
        data: unsafe { &tegra_max98088_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-sgtl5000"),
        data: unsafe { &tegra_sgtl5000_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-wm9712"),
        data: unsafe { &tegra_wm9712_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-wm8753"),
        data: unsafe { &tegra_wm8753_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-rt5677"),
        data: unsafe { &tegra_rt5677_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-rt5640"),
        data: unsafe { &tegra_rt5640_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-alc5632"),
        data: unsafe { &tegra_rt5632_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-rt5631"),
        data: unsafe { &tegra_rt5631_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: cstr!("nvidia,tegra-audio-cpcap"),
        data: unsafe { &tegra_cpcap_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, tegra_machine_of_match); */

static mut tegra_asoc_machine_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: cstr!("tegra-audio"),
        of_match_table: tegra_machine_of_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const _ as *const c_void },
    },
    probe: Some(tegra_asoc_machine_probe),
};

/* module_platform_driver(tegra_asoc_machine_driver); */
/* MODULE_AUTHOR("Anatol Pomozov <anatol@google.com>"); */
/* MODULE_AUTHOR("Andrey Danin <danindrey@mail.ru>"); */
/* MODULE_AUTHOR("Dmitry Osipenko <digetx@gmail.com>"); */
/* MODULE_AUTHOR("Ion Agorria <ion@agorria.com>"); */
/* MODULE_AUTHOR("Leon Romanovsky <leon@leon.nu>"); */
/* MODULE_AUTHOR("Lucas Stach <dev@lynxeye.de>"); */
/* MODULE_AUTHOR("Marc Dietrich <marvin24@gmx.de>"); */
/* MODULE_AUTHOR("Marcel Ziswiler <marcel@ziswiler.com>"); */
/* MODULE_AUTHOR("Mike Rapoport <mike@compulab.co.il>"); */
/* MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>"); */
/* MODULE_AUTHOR("Svyatoslav Ryhel <clamor95@gmail.com>"); */
/* MODULE_DESCRIPTION("Tegra machine ASoC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
