// SPDX-License-Identifier: GPL-2.0-only
/*
 * n810.rs  --  SoC audio for Nokia N810
 *
 * Copyright (C) 2008 Nokia Corporation
 *
 * Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
 */

/* Dependencies translated from:
 * <linux/clk.h>
 * <linux/i2c.h>
 * <linux/platform_device.h>
 * <sound/core.h>
 * <sound/pcm.h>
 * <sound/soc.h>
 * <asm/mach-types.h>
 * <linux/gpio/consumer.h>
 * <linux/module.h>
 * <linux/platform_data/asoc-ti-mcbsp.h>
 * "omap-mcbsp.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
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
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    /* SND_SOC_DAILINK_REG(aic33) expands to framework-defined link fields. */
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char);
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char);
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        val: c_uint,
    ) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn of_have_populated_dt() -> bool;
    fn of_machine_is_compatible(compat: *const c_char) -> bool;
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn clk_put(clk: *mut clk);
    fn platform_device_del(pdev: *mut platform_device);
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
}

const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 10;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

const N810_JACK_DISABLED: c_int = 0;
const N810_JACK_HP: c_int = 1;
const N810_JACK_HS: c_int = 2;
const N810_JACK_MIC: c_int = 3;

static mut n810_headset_amp: *mut gpio_desc = ptr::null_mut();
static mut n810_speaker_amp: *mut gpio_desc = ptr::null_mut();

static mut sys_clkout2: *mut clk = ptr::null_mut();
static mut sys_clkout2_src: *mut clk = ptr::null_mut();
static mut func96m_clk: *mut clk = ptr::null_mut();

static mut n810_spk_func: c_int = 0;
static mut n810_jack_func: c_int = 0;
static mut n810_dmic_func: c_int = 0;

unsafe fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool {
    event != 0
}

unsafe extern "C" fn n810_ext_control(dapm: *mut snd_soc_dapm_context) {
    let mut hp: c_int = 0;
    let mut line1l: c_int = 0;

    match n810_jack_func {
        N810_JACK_HS => {
            line1l = 1;
            hp = 1;
        }
        N810_JACK_HP => {
            hp = 1;
        }
        N810_JACK_MIC => {
            line1l = 1;
        }
        _ => {}
    }

    snd_soc_dapm_mutex_lock(dapm);

    if n810_spk_func != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"Ext Spk\0".as_ptr() as *const c_char);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"Ext Spk\0".as_ptr() as *const c_char);
    }

    if hp != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"Headphone Jack\0".as_ptr() as *const c_char);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"Headphone Jack\0".as_ptr() as *const c_char);
    }
    if line1l != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"HS Mic\0".as_ptr() as *const c_char);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"HS Mic\0".as_ptr() as *const c_char);
    }

    if n810_dmic_func != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"DMic\0".as_ptr() as *const c_char);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"DMic\0".as_ptr() as *const c_char);
    }

    snd_soc_dapm_sync_unlocked(dapm);

    snd_soc_dapm_mutex_unlock(dapm);
}

unsafe extern "C" fn n810_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm((*rtd).card);

    snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 2);

    n810_ext_control(dapm);
    clk_prepare_enable(sys_clkout2)
}

unsafe extern "C" fn n810_shutdown(_substream: *mut snd_pcm_substream) {
    clk_disable_unprepare(sys_clkout2);
}

unsafe extern "C" fn n810_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let err: c_int;

    /* Set the codec system clock for DAC and ADC */
    err = snd_soc_dai_set_sysclk(codec_dai, 0, 12000000, SND_SOC_CLOCK_IN);

    err
}

static n810_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(n810_startup),
    hw_params: Some(n810_hw_params),
    shutdown: Some(n810_shutdown),
};

unsafe extern "C" fn n810_get_spk(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.enumerated.item[0] = n810_spk_func as c_uint;

    0
}

unsafe extern "C" fn n810_set_spk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);

    if n810_spk_func as c_uint == (*ucontrol).value.enumerated.item[0] {
        return 0;
    }

    n810_spk_func = (*ucontrol).value.enumerated.item[0] as c_int;
    n810_ext_control(dapm);

    1
}

unsafe extern "C" fn n810_get_jack(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.enumerated.item[0] = n810_jack_func as c_uint;

    0
}

unsafe extern "C" fn n810_set_jack(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);

    if n810_jack_func as c_uint == (*ucontrol).value.enumerated.item[0] {
        return 0;
    }

    n810_jack_func = (*ucontrol).value.enumerated.item[0] as c_int;
    n810_ext_control(dapm);

    1
}

unsafe extern "C" fn n810_get_input(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.enumerated.item[0] = n810_dmic_func as c_uint;

    0
}

unsafe extern "C" fn n810_set_input(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);

    if n810_dmic_func as c_uint == (*ucontrol).value.enumerated.item[0] {
        return 0;
    }

    n810_dmic_func = (*ucontrol).value.enumerated.item[0] as c_int;
    n810_ext_control(dapm);

    1
}

unsafe extern "C" fn n810_spk_event(
    _w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    if SND_SOC_DAPM_EVENT_ON(event) {
        gpiod_set_value(n810_speaker_amp, 1);
    } else {
        gpiod_set_value(n810_speaker_amp, 0);
    }

    0
}

unsafe extern "C" fn n810_jack_event(
    _w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    if SND_SOC_DAPM_EVENT_ON(event) {
        gpiod_set_value(n810_headset_amp, 1);
    } else {
        gpiod_set_value(n810_headset_amp, 0);
    }

    0
}

/* SND_SOC_DAPM_SPK/HP/MIC are framework macros; their expanded field layout is external. */
static aic33_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_SPK("Ext Spk", n810_spk_event),
    SND_SOC_DAPM_HP("Headphone Jack", n810_jack_event),
    SND_SOC_DAPM_MIC("DMic", None),
    SND_SOC_DAPM_MIC("HS Mic", None),
];

static audio_map: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPLOUT\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPROUT\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Ext Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"LLOUT\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Ext Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"RLOUT\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DMic Rate 64\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DMic\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DMic\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic Bias\0".as_ptr() as *const c_char,
    },
    /*
     * Note that the mic bias is coming from Retu/Vilma and we don't have
     * control over it atm. The analog HS mic is not working. <- TODO
     */
    snd_soc_dapm_route {
        sink: b"LINE1L\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HS Mic\0".as_ptr() as *const c_char,
    },
];

static spk_function: [*const c_char; 2] = [b"Off\0".as_ptr() as *const c_char, b"On\0".as_ptr() as *const c_char];
static jack_function: [*const c_char; 4] = [
    b"Off\0".as_ptr() as *const c_char,
    b"Headphone\0".as_ptr() as *const c_char,
    b"Headset\0".as_ptr() as *const c_char,
    b"Mic\0".as_ptr() as *const c_char,
];
static input_function: [*const c_char; 2] = [
    b"ADC\0".as_ptr() as *const c_char,
    b"Digital Mic\0".as_ptr() as *const c_char,
];
static n810_enum: [soc_enum; 3] = [
    SOC_ENUM_SINGLE_EXT(spk_function.len(), spk_function.as_ptr()),
    SOC_ENUM_SINGLE_EXT(jack_function.len(), jack_function.as_ptr()),
    SOC_ENUM_SINGLE_EXT(input_function.len(), input_function.as_ptr()),
];

static aic33_n810_controls: [snd_kcontrol_new; 3] = [
    SOC_ENUM_EXT(
        "Speaker Function",
        &n810_enum[0],
        n810_get_spk,
        n810_set_spk,
    ),
    SOC_ENUM_EXT(
        "Jack Function",
        &n810_enum[1],
        n810_get_jack,
        n810_set_jack,
    ),
    SOC_ENUM_EXT(
        "Input Select",
        &n810_enum[2],
        n810_get_input,
        n810_set_input,
    ),
];

/* Digital audio interface glue - connects codec <--> CPU */
SND_SOC_DAILINK_DEFS!(
    aic33,
    DAILINK_COMP_ARRAY!(COMP_CPU!("48076000.mcbsp")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(
        "tlv320aic3x-codec.1-0018",
        "tlv320aic3x-hifi"
    )),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!("48076000.mcbsp"))
);

static mut n810_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: b"TLV320AIC33\0".as_ptr() as *const c_char,
    stream_name: b"AIC33\0".as_ptr() as *const c_char,
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    ops: &n810_ops,
    /* SND_SOC_DAILINK_REG(aic33), */
};

/* Audio machine driver */
static mut snd_soc_n810: snd_soc_card = snd_soc_card {
    name: b"N810\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &mut n810_dai },
    num_links: 1,

    controls: aic33_n810_controls.as_ptr(),
    num_controls: aic33_n810_controls.len() as c_uint,
    dapm_widgets: aic33_dapm_widgets.as_ptr(),
    num_dapm_widgets: aic33_dapm_widgets.len() as c_uint,
    dapm_routes: audio_map.as_ptr(),
    num_dapm_routes: audio_map.len() as c_uint,
    fully_routed: true,
};

static mut n810_snd_device: *mut platform_device = ptr::null_mut();

unsafe extern "C" fn n810_soc_init() -> c_int {
    let mut err: c_int;
    let dev: *mut device;

    if !of_have_populated_dt()
        || (!of_machine_is_compatible(b"nokia,n810\0".as_ptr() as *const c_char)
            && !of_machine_is_compatible(b"nokia,n810-wimax\0".as_ptr() as *const c_char))
    {
        return -ENODEV;
    }

    n810_snd_device = platform_device_alloc(b"soc-audio\0".as_ptr() as *const c_char, -1);
    if n810_snd_device.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(n810_snd_device, &mut snd_soc_n810 as *mut _ as *mut c_void);
    err = platform_device_add(n810_snd_device);
    if err != 0 {
        goto_err1(err);
        return err;
    }

    dev = &mut (*n810_snd_device).dev;

    sys_clkout2_src = clk_get(dev, b"sys_clkout2_src\0".as_ptr() as *const c_char);
    if IS_ERR(sys_clkout2_src as *const c_void) {
        dev_err(
            dev,
            b"Could not get sys_clkout2_src clock\n\0".as_ptr() as *const c_char,
        );
        err = PTR_ERR(sys_clkout2_src as *const c_void);
        goto_err2(err);
        return err;
    }
    sys_clkout2 = clk_get(dev, b"sys_clkout2\0".as_ptr() as *const c_char);
    if IS_ERR(sys_clkout2 as *const c_void) {
        dev_err(dev, b"Could not get sys_clkout2\n\0".as_ptr() as *const c_char);
        err = PTR_ERR(sys_clkout2 as *const c_void);
        goto_err3(err);
        return err;
    }
    /*
     * Configure 12 MHz output on SYS_CLKOUT2. Therefore we must use
     * 96 MHz as its parent in order to get 12 MHz
     */
    func96m_clk = clk_get(dev, b"func_96m_ck\0".as_ptr() as *const c_char);
    if IS_ERR(func96m_clk as *const c_void) {
        dev_err(dev, b"Could not get func 96M clock\n\0".as_ptr() as *const c_char);
        err = PTR_ERR(func96m_clk as *const c_void);
        goto_err4(err);
        return err;
    }
    clk_set_parent(sys_clkout2_src, func96m_clk);
    clk_set_rate(sys_clkout2, 12000000);

    n810_headset_amp = devm_gpiod_get(
        &mut (*n810_snd_device).dev,
        b"headphone\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR(n810_headset_amp as *const c_void) {
        err = PTR_ERR(n810_headset_amp as *const c_void);
        goto_err4(err);
        return err;
    }

    n810_speaker_amp = devm_gpiod_get(
        &mut (*n810_snd_device).dev,
        b"speaker\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR(n810_speaker_amp as *const c_void) {
        err = PTR_ERR(n810_speaker_amp as *const c_void);
        goto_err4(err);
        return err;
    }

    return 0;
}

unsafe fn goto_err4(err: c_int) {
    clk_put(sys_clkout2);
    goto_err3(err);
}

unsafe fn goto_err3(err: c_int) {
    clk_put(sys_clkout2_src);
    goto_err2(err);
}

unsafe fn goto_err2(err: c_int) {
    platform_device_del(n810_snd_device);
    goto_err1(err);
}

unsafe fn goto_err1(_err: c_int) {
    platform_device_put(n810_snd_device);
}

unsafe extern "C" fn n810_soc_exit() {
    clk_put(sys_clkout2_src);
    clk_put(sys_clkout2);
    clk_put(func96m_clk);

    platform_device_unregister(n810_snd_device);
}

module_init!(n810_soc_init);
module_exit!(n810_soc_exit);

MODULE_AUTHOR!("Jarkko Nikula <jarkko.nikula@bitmer.com>");
MODULE_DESCRIPTION!("ALSA SoC Nokia N810");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
