// SPDX-License-Identifier: GPL-2.0-only
/*
 * rx51.rs  --  SoC audio for Nokia RX-51
 *
 * Copyright (C) 2008 - 2009 Nokia Corporation
 *
 * Contact: Peter Ujfalusi <peter.ujfalusi@ti.com>
 *          Eduardo Valentin <eduardo.valentin@nokia.com>
 *          Jarkko Nikula <jarkko.nikula@bitmer.com>
 */

// Requires: linux/delay.h
// Requires: linux/platform_device.h
// Requires: linux/gpio/consumer.h
// Requires: linux/module.h
// Requires: sound/core.h
// Requires: sound/jack.h
// Requires: sound/pcm.h
// Requires: sound/soc.h
// Requires: linux/platform_data/asoc-ti-mcbsp.h
// Requires: omap-mcbsp.h

// Forward declarations of external types
#[repr(C)]
pub struct gpio_desc {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [u32; 128],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _opaque: [u8; 0],
}

// Jack function constants
const RX51_JACK_DISABLED: i32 = 0;
const RX51_JACK_TVOUT: i32 = 1;    // tv-out with stereo output
const RX51_JACK_HP: i32 = 2;       // headphone: stereo output, no mic
const RX51_JACK_HS: i32 = 3;       // headset: stereo output with mic

#[repr(C)]
pub struct rx51_audio_pdata {
    pub tvout_selection_gpio: *mut gpio_desc,
    pub eci_sw_gpio: *mut gpio_desc,
    pub speaker_amp_gpio: *mut gpio_desc,
}

static mut rx51_spk_func: i32 = 0;
static mut rx51_dmic_func: i32 = 0;
static mut rx51_jack_func: i32 = 0;

// External C functions
extern "C" {
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut core::ffi::c_void;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const i8);
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const i8);
    fn gpiod_set_value(desc: *mut gpio_desc, value: i32);
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: u32,
        val: u32,
    ) -> i32;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream)
        -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        idx: i32,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: i32,
        freq: u32,
        dir: i32,
    ) -> i32;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut core::ffi::c_void;
    fn gpiod_set_raw_value_cansleep(desc: *mut gpio_desc, value: i32);
    fn omap_mcbsp_st_add_controls(
        rtd: *mut snd_soc_pcm_runtime,
        port: i32,
    ) -> i32;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const i8,
        type_: u32,
        jack: *mut snd_soc_jack,
    ) -> i32;
    fn snd_soc_jack_add_gpios(
        jack: *mut snd_soc_jack,
        count: usize,
        gpios: *mut snd_soc_jack_gpio,
    ) -> i32;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut core::ffi::c_void);
    fn devm_gpiod_get(
        dev: *mut device,
        con_id: *const i8,
        flags: u32,
    ) -> *mut gpio_desc;
    fn devm_snd_soc_register_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> i32;
    fn of_machine_is_compatible(compat: *const i8) -> i32;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const i8,
        index: i32,
    ) -> *mut device_node;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const i8, ...) -> i32;
    fn snd_soc_limit_volume(card: *mut snd_soc_card, name: *const i8, max: i32);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *mut core::ffi::c_void) -> i32;
    fn PTR_ERR(ptr: *mut core::ffi::c_void) -> i32;
}

unsafe fn rx51_ext_control(dapm: *mut snd_soc_dapm_context) {
    let card = snd_soc_dapm_to_card(dapm);
    let pdata = snd_soc_card_get_drvdata(card) as *mut rx51_audio_pdata;
    let mut hp = 0;
    let mut hs = 0;
    let mut tvout = 0;

    match rx51_jack_func {
        RX51_JACK_TVOUT => {
            tvout = 1;
            hp = 1;
        }
        RX51_JACK_HS => {
            hs = 1;
            hp = 1;
        }
        RX51_JACK_HP => {
            hp = 1;
        }
        _ => {}
    }

    snd_soc_dapm_mutex_lock(dapm);

    if rx51_spk_func != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"Ext Spk\0".as_ptr() as *const i8);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"Ext Spk\0".as_ptr() as *const i8);
    }
    if rx51_dmic_func != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"DMic\0".as_ptr() as *const i8);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"DMic\0".as_ptr() as *const i8);
    }
    if hp != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"Headphone Jack\0".as_ptr() as *const i8);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"Headphone Jack\0".as_ptr() as *const i8);
    }
    if hs != 0 {
        snd_soc_dapm_enable_pin_unlocked(dapm, b"HS Mic\0".as_ptr() as *const i8);
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, b"HS Mic\0".as_ptr() as *const i8);
    }

    gpiod_set_value((*pdata).tvout_selection_gpio, tvout);

    snd_soc_dapm_sync_unlocked(dapm);

    snd_soc_dapm_mutex_unlock(dapm);
}

unsafe extern "C" fn rx51_startup(substream: *mut snd_pcm_substream) -> i32 {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let dapm = snd_soc_card_to_dapm((*rtd).card);

    snd_pcm_hw_constraint_single(runtime, 10, 2);
    rx51_ext_control(dapm);

    0
}

unsafe extern "C" fn rx51_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    snd_soc_dai_set_sysclk(codec_dai, 0, 19200000, 0)
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
}

static rx51_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(rx51_startup),
    hw_params: Some(rx51_hw_params),
};

unsafe extern "C" fn rx51_get_spk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    (*ucontrol).value.enumerated.item[0] = rx51_spk_func as u32;

    0
}

unsafe extern "C" fn rx51_set_spk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    let dapm = snd_soc_card_to_dapm(card);

    if rx51_spk_func == (*ucontrol).value.enumerated.item[0] as i32 {
        return 0;
    }

    rx51_spk_func = (*ucontrol).value.enumerated.item[0] as i32;
    rx51_ext_control(dapm);

    1
}

unsafe extern "C" fn rx51_spk_event(
    w: *mut snd_soc_dapm_widget,
    k: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let dapm = (*w).dapm;
    let card = snd_soc_dapm_to_card(dapm);
    let pdata = snd_soc_card_get_drvdata(card) as *mut rx51_audio_pdata;

    gpiod_set_raw_value_cansleep(
        (*pdata).speaker_amp_gpio,
        if SND_SOC_DAPM_EVENT_ON(event) != 0 { 1 } else { 0 },
    );

    0
}

unsafe extern "C" fn rx51_get_input(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    (*ucontrol).value.enumerated.item[0] = rx51_dmic_func as u32;

    0
}

unsafe extern "C" fn rx51_set_input(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    let dapm = snd_soc_card_to_dapm(card);

    if rx51_dmic_func == (*ucontrol).value.enumerated.item[0] as i32 {
        return 0;
    }

    rx51_dmic_func = (*ucontrol).value.enumerated.item[0] as i32;
    rx51_ext_control(dapm);

    1
}

unsafe extern "C" fn rx51_get_jack(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    (*ucontrol).value.enumerated.item[0] = rx51_jack_func as u32;

    0
}

unsafe extern "C" fn rx51_set_jack(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    let dapm = snd_soc_card_to_dapm(card);

    if rx51_jack_func == (*ucontrol).value.enumerated.item[0] as i32 {
        return 0;
    }

    rx51_jack_func = (*ucontrol).value.enumerated.item[0] as i32;
    rx51_ext_control(dapm);

    1
}

static mut rx51_av_jack: snd_soc_jack = unsafe { core::mem::zeroed() };

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const i8,
    pub report: i32,
    pub invert: i32,
    pub debounce_time: i32,
    pub gpiod_dev: *mut device,
    pub idx: i32,
}

static mut rx51_av_jack_gpios: [snd_soc_jack_gpio; 1] = [snd_soc_jack_gpio {
    name: b"jack-detection\0".as_ptr() as *const i8,
    report: 4,
    invert: 1,
    debounce_time: 200,
    gpiod_dev: core::ptr::null_mut(),
    idx: 0,
}];

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _opaque: [u8; 0],
}

// DAPM widgets array
// SND_SOC_DAPM_SPK("Ext Spk", rx51_spk_event),
// SND_SOC_DAPM_MIC("DMic", NULL),
// SND_SOC_DAPM_HP("Headphone Jack", NULL),
// SND_SOC_DAPM_MIC("HS Mic", NULL),
// SND_SOC_DAPM_LINE("FM Transmitter", NULL),
// SND_SOC_DAPM_SPK("Earphone", NULL),
static aic34_dapm_widgets: &[&str] = &[
    "Ext Spk", "DMic", "Headphone Jack", "HS Mic", "FM Transmitter", "Earphone",
];

// DAPM routes
static audio_map: &[(&str, &str, &str)] = &[
    ("Ext Spk", "", "HPLOUT"),
    ("Ext Spk", "", "HPROUT"),
    ("Ext Spk", "", "HPLCOM"),
    ("Ext Spk", "", "HPRCOM"),
    ("FM Transmitter", "", "LLOUT"),
    ("FM Transmitter", "", "RLOUT"),
    ("Headphone Jack", "", "TPA6130A2 HPLEFT"),
    ("Headphone Jack", "", "TPA6130A2 HPRIGHT"),
    ("TPA6130A2 LEFTIN", "", "LLOUT"),
    ("TPA6130A2 RIGHTIN", "", "RLOUT"),
    ("DMic Rate 64", "", "DMic"),
    ("DMic", "", "Mic Bias"),
    ("b LINE2R", "", "MONO_LOUT"),
    ("Earphone", "", "b HPLOUT"),
    ("LINE1L", "", "HS Mic"),
    ("HS Mic", "", "b Mic Bias"),
];

static spk_function: &[&str] = &["Off", "On"];
static input_function: &[&str] = &["ADC", "Digital Mic"];
static jack_function: &[&str] = &["Off", "TV-OUT", "Headphone", "Headset"];

#[repr(C)]
pub struct soc_enum {
    // Macro-expanded: SOC_ENUM_SINGLE_EXT
}

// aic34_rx51_controls defined via macros:
// SOC_ENUM_EXT("Speaker Function", rx51_enum[0], rx51_get_spk, rx51_set_spk),
// SOC_ENUM_EXT("Input Select", rx51_enum[1], rx51_get_input, rx51_set_input),
// SOC_ENUM_EXT("Jack Function", rx51_enum[2], rx51_get_jack, rx51_set_jack),
// SOC_DAPM_PIN_SWITCH("FM Transmitter"),
// SOC_DAPM_PIN_SWITCH("Earphone"),

unsafe extern "C" fn rx51_aic34_init(rtd: *mut snd_soc_pcm_runtime) -> i32 {
    let card = (*rtd).card;
    let mut err: i32;

    snd_soc_limit_volume(card, b"TPA6130A2 Headphone Playback Volume\0".as_ptr() as *const i8, 42);

    err = omap_mcbsp_st_add_controls(rtd, 2);
    if err < 0 {
        dev_err((*card).dev, b"Failed to add MCBSP controls\n\0".as_ptr() as *const i8);
        return err;
    }

    err = snd_soc_card_jack_new(
        card,
        b"AV Jack\0".as_ptr() as *const i8,
        12,
        &mut rx51_av_jack,
    );
    if err != 0 {
        dev_err((*card).dev, b"Failed to add AV Jack\n\0".as_ptr() as *const i8);
        return err;
    }

    rx51_av_jack_gpios[0].gpiod_dev = (*card).dev;
    rx51_av_jack_gpios[0].idx = 0;

    err = snd_soc_jack_add_gpios(
        &mut rx51_av_jack,
        1,
        rx51_av_jack_gpios.as_mut_ptr(),
    );
    if err != 0 {
        dev_err((*card).dev, b"Failed to add GPIOs\n\0".as_ptr() as *const i8);
        return err;
    }

    err
}

// DAI link definitions via SND_SOC_DAILINK_DEFS macro
#[repr(C)]
pub struct snd_soc_dai_link {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_aux_dev {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    _opaque: [u8; 0],
}

// rx51_dai is expanded from SND_SOC_DAILINK_DEFS macro
static mut rx51_dai: [snd_soc_dai_link; 1] = [unsafe { core::mem::zeroed() }];

// rx51_aux_dev
static mut rx51_aux_dev: [snd_soc_aux_dev; 2] = [unsafe { core::mem::zeroed() }; 2];

// rx51_codec_conf
static mut rx51_codec_conf: [snd_soc_codec_conf; 2] = [unsafe { core::mem::zeroed() }; 2];

// rx51_sound_card
static mut rx51_sound_card: snd_soc_card = unsafe { core::mem::zeroed() };

unsafe extern "C" fn rx51_soc_probe(pdev: *mut platform_device) -> i32 {
    let mut pdata: *mut rx51_audio_pdata;
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    let card = &mut rx51_sound_card;
    let mut err: i32;

    if of_machine_is_compatible(b"nokia,omap3-n900\0".as_ptr() as *const i8) == 0 {
        return -19;
    }

    (*card).dev = dev;

    if !np.is_null() {
        let mut dai_node: *mut device_node;

        dai_node = of_parse_phandle(np, b"nokia,cpu-dai\0".as_ptr() as *const i8, 0);
        if dai_node.is_null() {
            dev_err(dev, b"McBSP node is not provided\n\0".as_ptr() as *const i8);
            return -22;
        }
        (*(*card).dai_link.add(0)).cpus.dai_name = core::ptr::null_mut();
        (*(*card).dai_link.add(0)).platforms.name = core::ptr::null_mut();
        (*(*card).dai_link.add(0)).cpus.of_node = dai_node;
        (*(*card).dai_link.add(0)).platforms.of_node = dai_node;

        dai_node = of_parse_phandle(np, b"nokia,audio-codec\0".as_ptr() as *const i8, 0);
        if dai_node.is_null() {
            dev_err(dev, b"Codec node is not provided\n\0".as_ptr() as *const i8);
            return -22;
        }
        (*(*card).dai_link.add(0)).codecs.name = core::ptr::null_mut();
        (*(*card).dai_link.add(0)).codecs.of_node = dai_node;

        dai_node = of_parse_phandle(np, b"nokia,audio-codec\0".as_ptr() as *const i8, 1);
        if dai_node.is_null() {
            dev_err(dev, b"Auxiliary Codec node is not provided\n\0".as_ptr() as *const i8);
            return -22;
        }
        (*rx51_aux_dev.as_mut_ptr()).dlc.name = core::ptr::null_mut();
        (*rx51_aux_dev.as_mut_ptr()).dlc.of_node = dai_node;
        (*rx51_codec_conf.as_mut_ptr()).dlc.name = core::ptr::null_mut();
        (*rx51_codec_conf.as_mut_ptr()).dlc.of_node = dai_node;

        dai_node = of_parse_phandle(np, b"nokia,headphone-amplifier\0".as_ptr() as *const i8, 0);
        if dai_node.is_null() {
            dev_err(dev, b"Headphone amplifier node is not provided\n\0".as_ptr() as *const i8);
            return -22;
        }
        (*rx51_aux_dev.as_mut_ptr().add(1)).dlc.name = core::ptr::null_mut();
        (*rx51_aux_dev.as_mut_ptr().add(1)).dlc.of_node = dai_node;
        (*rx51_codec_conf.as_mut_ptr().add(1)).dlc.name = core::ptr::null_mut();
        (*rx51_codec_conf.as_mut_ptr().add(1)).dlc.of_node = dai_node;
    }

    pdata = devm_kzalloc(dev, core::mem::size_of::<rx51_audio_pdata>(), 0x10) as *mut rx51_audio_pdata;
    if pdata.is_null() {
        return -12;
    }

    snd_soc_card_set_drvdata(card, pdata as *mut core::ffi::c_void);

    (*pdata).tvout_selection_gpio = devm_gpiod_get(
        dev,
        b"tvout-selection\0".as_ptr() as *const i8,
        1,
    );
    if IS_ERR((*pdata).tvout_selection_gpio as *mut core::ffi::c_void) != 0 {
        return dev_err_probe(
            dev,
            PTR_ERR((*pdata).tvout_selection_gpio as *mut core::ffi::c_void),
            b"could not get tvout selection gpio\n\0".as_ptr() as *const i8,
        );
    }

    (*pdata).eci_sw_gpio = devm_gpiod_get(dev, b"eci-switch\0".as_ptr() as *const i8, 2);
    if IS_ERR((*pdata).eci_sw_gpio as *mut core::ffi::c_void) != 0 {
        return dev_err_probe(
            dev,
            PTR_ERR((*pdata).eci_sw_gpio as *mut core::ffi::c_void),
            b"could not get eci switch gpio\n\0".as_ptr() as *const i8,
        );
    }

    (*pdata).speaker_amp_gpio = devm_gpiod_get(
        dev,
        b"speaker-amplifier\0".as_ptr() as *const i8,
        1,
    );
    if IS_ERR((*pdata).speaker_amp_gpio as *mut core::ffi::c_void) != 0 {
        return dev_err_probe(
            dev,
            PTR_ERR((*pdata).speaker_amp_gpio as *mut core::ffi::c_void),
            b"could not get speaker enable gpio\n\0".as_ptr() as *const i8,
        );
    }

    err = devm_snd_soc_register_card(dev, card);
    if err != 0 {
        return dev_err_probe(
            dev,
            err,
            b"snd_soc_register_card() failed\n\0".as_ptr() as *const i8,
        );
    }

    0
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: [i8; 128],
    pub data: *mut core::ffi::c_void,
}

static rx51_audio_of_match: &[of_device_id] = &[
    of_device_id {
        compatible: [
            b'n' as i8, b'o' as i8, b'k' as i8, b'i' as i8, b'a' as i8, b',' as i8, b'n' as i8,
            b'9' as i8, b'0' as i8, b'0' as i8, b'-' as i8, b'a' as i8, b'u' as i8, b'd' as i8,
            b'i' as i8, b'o' as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ],
        data: core::ptr::null_mut(),
    },
    of_device_id {
        compatible: [0; 128],
        data: core::ptr::null_mut(),
    },
];

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const i8,
    pub of_match_table: *const of_device_id,
}

static rx51_soc_driver: platform_driver = platform_driver {
    probe: Some(rx51_soc_probe),
    driver: platform_driver_driver {
        name: b"rx51-audio\0".as_ptr() as *const i8,
        of_match_table: rx51_audio_of_match.as_ptr(),
    },
};

fn SND_SOC_DAPM_EVENT_ON(event: i32) -> i32 {
    if (event & 1) != 0 { 1 } else { 0 }
}

// module_platform_driver(rx51_soc_driver);
// MODULE_AUTHOR("Nokia Corporation");
// MODULE_DESCRIPTION("ALSA SoC Nokia RX-51");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:rx51-audio");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
