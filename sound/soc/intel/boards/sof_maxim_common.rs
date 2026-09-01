// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Translated from C implementation source. C include dependencies are expected
// to provide the referenced kernel/ASoC symbols, macros, and constants.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
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

unsafe extern "C" {
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_int) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn sof_dai_get_tdm_slots(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num_controls: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn soc_intel_is_cml() -> bool;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn fls(x: c_uint) -> c_int;
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0x0004;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0x0005;

const MAX_98373_DEV0_NAME: *const c_char = c"i2c-MX98373:00".as_ptr();
const MAX_98373_DEV1_NAME: *const c_char = c"i2c-MX98373:01".as_ptr();
const MAX_98373_CODEC_DAI: *const c_char = c"max98373-aif1".as_ptr();
const MAX_98373_ACPI_HID: *const c_char = c"MX98373".as_ptr();
const MAX_98390_DEV0_NAME: *const c_char = c"i2c-MX98390:00".as_ptr();
const MAX_98390_DEV1_NAME: *const c_char = c"i2c-MX98390:01".as_ptr();
const MAX_98390_DEV2_NAME: *const c_char = c"i2c-MX98390:02".as_ptr();
const MAX_98390_DEV3_NAME: *const c_char = c"i2c-MX98390:03".as_ptr();
const MAX_98390_CODEC_DAI: *const c_char = c"max98390-aif1".as_ptr();
const MAX_98390_ACPI_HID: *const c_char = c"MX98390".as_ptr();
const MAX_98357A_DEV0_NAME: *const c_char = c"MX98357A:00".as_ptr();
const MAX_98360A_DEV0_NAME: *const c_char = c"MX98360A:00".as_ptr();
const MAX_98357A_CODEC_DAI: *const c_char = c"HiFi".as_ptr();

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        $array.len() as c_int
    };
}

macro_rules! COMP_CODEC_CONF {
    ($name:expr) => {
        snd_soc_dai_link_component {
            name: $name,
            dai_name: core::ptr::null(),
        }
    };
}

macro_rules! SOC_DAPM_PIN_SWITCH {
    ($name:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_SPK {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

/*
 * Common structures and functions
 */
static maxim_2spk_kcontrols: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!("Left Spk"),
    SOC_DAPM_PIN_SWITCH!("Right Spk"),
];

static maxim_2spk_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_SPK!("Left Spk", core::ptr::null::<c_void>()),
    SND_SOC_DAPM_SPK!("Right Spk", core::ptr::null::<c_void>()),
];

/* helper function to get the number of specific codec */
unsafe fn get_num_codecs(hid: *const c_char) -> c_uint {
    let mut adev: *mut acpi_device;
    let mut dev_num: c_uint = 0;

    for_each_acpi_dev_match!(adev, hid, core::ptr::null::<c_void>(), -1, {
        dev_num += 1;
    });

    dev_num
}

/*
 * Maxim MAX98373
 */
const MAX_98373_PIN_NAME: usize = 16;

static max_98373_dapm_routes: [snd_soc_dapm_route; 2] = [
    /* speaker */
    snd_soc_dapm_route { sink: c"Left Spk".as_ptr(), control: core::ptr::null(), source: c"Left BE_OUT".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Spk".as_ptr(), control: core::ptr::null(), source: c"Right BE_OUT".as_ptr() },
];

static mut max_98373_codec_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98373_DEV0_NAME), name_prefix: c"Right".as_ptr() },
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98373_DEV1_NAME), name_prefix: c"Left".as_ptr() },
];

static mut max_98373_components: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component { /* For Right */ name: MAX_98373_DEV0_NAME, dai_name: MAX_98373_CODEC_DAI },
    snd_soc_dai_link_component { /* For Left */ name: MAX_98373_DEV1_NAME, dai_name: MAX_98373_CODEC_DAI },
];

/*
 * According to the definition of 'DAI Sel Mux' mixer in max98373.c, rx mask
 * should choose two channels from TDM slots, the LSB of rx mask is left channel
 * and the other one is right channel.
 */
#[repr(C)]
struct max_98373_tdm_mask_entry {
    rx: c_uint,
}

static max_98373_tdm_mask: [max_98373_tdm_mask_entry; 2] = [
    max_98373_tdm_mask_entry { rx: 0x3 },
    max_98373_tdm_mask_entry { rx: 0x3 },
];

/*
 * The tx mask indicates which channel(s) contains output IV-sense data and
 * others should set to Hi-Z. Here we get the channel number from codec's ACPI
 * device property "maxim,vmon-slot-no" and "maxim,imon-slot-no" to generate the
 * mask. Refer to the max98373_slot_config() function in max98373.c codec driver.
 */
unsafe fn max_98373_get_tx_mask(dev: *mut device) -> c_uint {
    let mut vmon_slot: c_int = 0;
    let mut imon_slot: c_int = 0;

    if device_property_read_u32(dev, c"maxim,vmon-slot-no".as_ptr(), &mut vmon_slot) != 0 {
        vmon_slot = 0;
    }

    if device_property_read_u32(dev, c"maxim,imon-slot-no".as_ptr(), &mut imon_slot) != 0 {
        imon_slot = 1;
    }

    dev_dbg(dev, c"vmon_slot %d imon_slot %d\n".as_ptr(), vmon_slot, imon_slot);

    ((0x1_u32 << vmon_slot) | (0x1_u32 << imon_slot)) as c_uint
}

unsafe extern "C" fn max_98373_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai_link = (*rtd).dai_link;
    let mut codec_dai: *mut snd_soc_dai;
    let mut i: c_int;
    let mut tdm_slots: c_int;
    let mut tx_mask: c_uint;
    let mut tx_mask_used: c_uint = 0x0;
    let mut ret: c_int = 0;

    for_each_rtd_codec_dais!(rtd, i, codec_dai, {
        if i as usize >= max_98373_tdm_mask.len() {
            dev_err((*codec_dai).dev, c"only 2 amps are supported\n".as_ptr());
            return -EINVAL;
        }

        match (*dai_link).dai_fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            x if x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
                /* get the tplg configured tdm slot number */
                tdm_slots = sof_dai_get_tdm_slots(rtd);
                if tdm_slots <= 0 {
                    dev_err(rtd.as_ref().unwrap().dev, c"invalid tdm slots %d\n".as_ptr(), tdm_slots);
                    return -EINVAL;
                }

                /* get the tx mask from ACPI device properties */
                tx_mask = max_98373_get_tx_mask((*codec_dai).dev);
                if tx_mask == 0 {
                    return -EINVAL;
                }

                if (tx_mask & tx_mask_used) != 0 {
                    dev_err(
                        (*codec_dai).dev,
                        c"invalid tx mask 0x%x, used 0x%x\n".as_ptr(),
                        tx_mask,
                        tx_mask_used,
                    );
                    return -EINVAL;
                }

                tx_mask_used |= tx_mask;

                /*
                 * check if tdm slot number is too small for channel
                 * allocation
                 */
                if fls(tx_mask) > tdm_slots {
                    dev_err(
                        (*codec_dai).dev,
                        c"slot mismatch, tx %d slots %d\n".as_ptr(),
                        fls(tx_mask),
                        tdm_slots,
                    );
                    return -EINVAL;
                }

                if fls(max_98373_tdm_mask[i as usize].rx) > tdm_slots {
                    dev_err(
                        (*codec_dai).dev,
                        c"slot mismatch, rx %d slots %d\n".as_ptr(),
                        fls(max_98373_tdm_mask[i as usize].rx),
                        tdm_slots,
                    );
                    return -EINVAL;
                }

                dev_dbg(
                    (*codec_dai).dev,
                    c"set tdm slot: tx 0x%x rx 0x%x slots %d width %d\n".as_ptr(),
                    tx_mask,
                    max_98373_tdm_mask[i as usize].rx,
                    tdm_slots,
                    params_width(params),
                );

                ret = snd_soc_dai_set_tdm_slot(
                    codec_dai,
                    tx_mask,
                    max_98373_tdm_mask[i as usize].rx,
                    tdm_slots,
                    params_width(params),
                );
                if ret < 0 {
                    dev_err((*codec_dai).dev, c"fail to set tdm slot, ret %d\n".as_ptr(), ret);
                    return ret;
                }
            }
            _ => {
                dev_dbg((*codec_dai).dev, c"codec is in I2S mode\n".as_ptr());
            }
        }
    });
    0
}

unsafe extern "C" fn max_98373_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let cpu_dai: *mut snd_soc_dai;
    let mut j: c_int;
    let mut ret: c_int = 0;

    /* set spk pin by playback only */
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        return 0;
    }

    cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    for_each_rtd_codec_dais!(rtd, j, codec_dai, {
        let dapm = snd_soc_component_to_dapm((*cpu_dai).component);
        let mut pin_name = [0 as c_char; MAX_98373_PIN_NAME];

        snprintf(
            pin_name.as_mut_ptr(),
            pin_name.len(),
            c"%s Spk".as_ptr(),
            (*(*codec_dai).component).name_prefix,
        );

        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                ret = snd_soc_dapm_enable_pin(dapm, pin_name.as_ptr());
                if ret == 0 {
                    snd_soc_dapm_sync(dapm);
                }
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                ret = snd_soc_dapm_disable_pin(dapm, pin_name.as_ptr());
                if ret == 0 {
                    snd_soc_dapm_sync(dapm);
                }
            }
            _ => {}
        }
    });

    ret
}

static max_98373_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(max_98373_hw_params),
    trigger: Some(max_98373_trigger),
};

unsafe extern "C" fn max_98373_spk_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let num_codecs = get_num_codecs(MAX_98373_ACPI_HID);
    let mut ret: c_int;

    match num_codecs {
        2 => {
            ret = snd_soc_dapm_new_controls(dapm, maxim_2spk_widgets.as_ptr(), ARRAY_SIZE!(maxim_2spk_widgets));
            if ret != 0 {
                dev_err((*rtd).dev, c"fail to add max98373 widgets, ret %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_add_card_controls(card, maxim_2spk_kcontrols.as_ptr(), ARRAY_SIZE!(maxim_2spk_kcontrols));
            if ret != 0 {
                dev_err((*rtd).dev, c"fail to add max98373 kcontrols, ret %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_dapm_add_routes(dapm, max_98373_dapm_routes.as_ptr(), ARRAY_SIZE!(max_98373_dapm_routes));
            if ret != 0 {
                dev_err((*rtd).dev, c"fail to add max98373 routes, ret %d\n".as_ptr(), ret);
                return ret;
            }
        }
        _ => {
            dev_err((*rtd).dev, c"max98373: invalid num_codecs %d\n".as_ptr(), num_codecs);
            return -EINVAL;
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn max_98373_dai_link(_dev: *mut device, link: *mut snd_soc_dai_link) {
    (*link).codecs = max_98373_components.as_mut_ptr();
    (*link).num_codecs = ARRAY_SIZE!(max_98373_components) as c_uint;
    (*link).init = Some(max_98373_spk_codec_init);
    (*link).ops = &max_98373_ops;
}
// EXPORT_SYMBOL_NS(max_98373_dai_link, "SND_SOC_INTEL_SOF_MAXIM_COMMON");

#[no_mangle]
pub unsafe extern "C" fn max_98373_set_codec_conf(card: *mut snd_soc_card) {
    (*card).codec_conf = max_98373_codec_conf.as_mut_ptr();
    (*card).num_configs = ARRAY_SIZE!(max_98373_codec_conf) as c_uint;
}
// EXPORT_SYMBOL_NS(max_98373_set_codec_conf, "SND_SOC_INTEL_SOF_MAXIM_COMMON");

/*
 * Maxim MAX98390
 */
static max_98390_dapm_routes: [snd_soc_dapm_route; 2] = [
    /* speaker */
    snd_soc_dapm_route { sink: c"Left Spk".as_ptr(), control: core::ptr::null(), source: c"Left BE_OUT".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Spk".as_ptr(), control: core::ptr::null(), source: c"Right BE_OUT".as_ptr() },
];

static max_98390_tt_kcontrols: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!("TL Spk"),
    SOC_DAPM_PIN_SWITCH!("TR Spk"),
];

static max_98390_tt_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_SPK!("TL Spk", core::ptr::null::<c_void>()),
    SND_SOC_DAPM_SPK!("TR Spk", core::ptr::null::<c_void>()),
];

static max_98390_tt_dapm_routes: [snd_soc_dapm_route; 2] = [
    /* Tweeter speaker */
    snd_soc_dapm_route { sink: c"TL Spk".as_ptr(), control: core::ptr::null(), source: c"Tweeter Left BE_OUT".as_ptr() },
    snd_soc_dapm_route { sink: c"TR Spk".as_ptr(), control: core::ptr::null(), source: c"Tweeter Right BE_OUT".as_ptr() },
];

static mut max_98390_cml_codec_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98390_DEV0_NAME), name_prefix: c"Left".as_ptr() },
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98390_DEV1_NAME), name_prefix: c"Right".as_ptr() },
];

static mut max_98390_codec_conf: [snd_soc_codec_conf; 4] = [
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98390_DEV0_NAME), name_prefix: c"Right".as_ptr() },
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98390_DEV1_NAME), name_prefix: c"Left".as_ptr() },
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98390_DEV2_NAME), name_prefix: c"Tweeter Right".as_ptr() },
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX_98390_DEV3_NAME), name_prefix: c"Tweeter Left".as_ptr() },
];

static mut max_98390_components: [snd_soc_dai_link_component; 4] = [
    snd_soc_dai_link_component { name: MAX_98390_DEV0_NAME, dai_name: MAX_98390_CODEC_DAI },
    snd_soc_dai_link_component { name: MAX_98390_DEV1_NAME, dai_name: MAX_98390_CODEC_DAI },
    snd_soc_dai_link_component { name: MAX_98390_DEV2_NAME, dai_name: MAX_98390_CODEC_DAI },
    snd_soc_dai_link_component { name: MAX_98390_DEV3_NAME, dai_name: MAX_98390_CODEC_DAI },
];

#[repr(C)]
struct max_98390_tdm_mask_entry {
    tx: c_uint,
    rx: c_uint,
}

static max_98390_tdm_mask: [max_98390_tdm_mask_entry; 4] = [
    max_98390_tdm_mask_entry { tx: 0x01, rx: 0x3 },
    max_98390_tdm_mask_entry { tx: 0x02, rx: 0x3 },
    max_98390_tdm_mask_entry { tx: 0x04, rx: 0x3 },
    max_98390_tdm_mask_entry { tx: 0x08, rx: 0x3 },
];

unsafe extern "C" fn max_98390_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai_link = (*rtd).dai_link;
    let mut codec_dai: *mut snd_soc_dai;
    let mut i: c_int;
    let mut ret: c_int;

    for_each_rtd_codec_dais!(rtd, i, codec_dai, {
        if i as usize >= max_98390_tdm_mask.len() {
            dev_err((*codec_dai).dev, c"invalid codec index %d\n".as_ptr(), i);
            return -ENODEV;
        }

        match (*dai_link).dai_fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            x if x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
                /* 4-slot TDM */
                ret = snd_soc_dai_set_tdm_slot(
                    codec_dai,
                    max_98390_tdm_mask[i as usize].tx,
                    max_98390_tdm_mask[i as usize].rx,
                    4,
                    params_width(params),
                );
                if ret < 0 {
                    dev_err((*codec_dai).dev, c"fail to set tdm slot, ret %d\n".as_ptr(), ret);
                    return ret;
                }
            }
            _ => {
                dev_dbg((*codec_dai).dev, c"codec is in I2S mode\n".as_ptr());
            }
        }
    });
    0
}

unsafe extern "C" fn max_98390_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let num_codecs = get_num_codecs(MAX_98390_ACPI_HID);
    let mut ret: c_int;

    match num_codecs {
        4 => {
            /* add widgets/controls/dapm for tweeter speakers */
            ret = snd_soc_dapm_new_controls(dapm, max_98390_tt_dapm_widgets.as_ptr(), ARRAY_SIZE!(max_98390_tt_dapm_widgets));
            if ret != 0 {
                dev_err((*rtd).dev, c"unable to add tweeter dapm widgets, ret %d\n".as_ptr(), ret);
                /* Don't need to add routes if widget addition failed */
                return ret;
            }

            ret = snd_soc_add_card_controls(card, max_98390_tt_kcontrols.as_ptr(), ARRAY_SIZE!(max_98390_tt_kcontrols));
            if ret != 0 {
                dev_err((*rtd).dev, c"unable to add tweeter controls, ret %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_dapm_add_routes(dapm, max_98390_tt_dapm_routes.as_ptr(), ARRAY_SIZE!(max_98390_tt_dapm_routes));
            if ret != 0 {
                dev_err((*rtd).dev, c"unable to add tweeter dapm routes, ret %d\n".as_ptr(), ret);
                return ret;
            }

            /* fallthrough */
            ret = snd_soc_dapm_new_controls(dapm, maxim_2spk_widgets.as_ptr(), ARRAY_SIZE!(maxim_2spk_widgets));
            if ret != 0 {
                dev_err((*rtd).dev, c"fail to add max98390 woofer widgets, ret %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_add_card_controls(card, maxim_2spk_kcontrols.as_ptr(), ARRAY_SIZE!(maxim_2spk_kcontrols));
            if ret != 0 {
                dev_err((*rtd).dev, c"fail to add max98390 woofer kcontrols, ret %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_dapm_add_routes(dapm, max_98390_dapm_routes.as_ptr(), ARRAY_SIZE!(max_98390_dapm_routes));
            if ret != 0 {
                dev_err((*rtd).dev, c"unable to add dapm routes, ret %d\n".as_ptr(), ret);
                return ret;
            }
        }
        2 => {
            /* add regular speakers dapm route */
            ret = snd_soc_dapm_new_controls(dapm, maxim_2spk_widgets.as_ptr(), ARRAY_SIZE!(maxim_2spk_widgets));
            if ret != 0 {
                dev_err((*rtd).dev, c"fail to add max98390 woofer widgets, ret %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_add_card_controls(card, maxim_2spk_kcontrols.as_ptr(), ARRAY_SIZE!(maxim_2spk_kcontrols));
            if ret != 0 {
                dev_err((*rtd).dev, c"fail to add max98390 woofer kcontrols, ret %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_dapm_add_routes(dapm, max_98390_dapm_routes.as_ptr(), ARRAY_SIZE!(max_98390_dapm_routes));
            if ret != 0 {
                dev_err((*rtd).dev, c"unable to add dapm routes, ret %d\n".as_ptr(), ret);
                return ret;
            }
        }
        _ => {
            dev_err((*rtd).dev, c"invalid codec number %d\n".as_ptr(), num_codecs);
            return -EINVAL;
        }
    }

    ret
}

static max_98390_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(max_98390_hw_params),
    trigger: None,
};

#[no_mangle]
pub unsafe extern "C" fn max_98390_dai_link(dev: *mut device, link: *mut snd_soc_dai_link) {
    let num_codecs = get_num_codecs(MAX_98390_ACPI_HID);

    (*link).codecs = max_98390_components.as_mut_ptr();

    match num_codecs {
        2 | 4 => {
            (*link).num_codecs = num_codecs;
        }
        _ => {
            dev_err(dev, c"invalid codec number %d for %s\n".as_ptr(), num_codecs, MAX_98390_ACPI_HID);
        }
    }

    (*link).init = Some(max_98390_init);
    (*link).ops = &max_98390_ops;
}
// EXPORT_SYMBOL_NS(max_98390_dai_link, "SND_SOC_INTEL_SOF_MAXIM_COMMON");

#[no_mangle]
pub unsafe extern "C" fn max_98390_set_codec_conf(dev: *mut device, card: *mut snd_soc_card) {
    let num_codecs = get_num_codecs(MAX_98390_ACPI_HID);

    (*card).codec_conf = max_98390_codec_conf.as_mut_ptr();

    match num_codecs {
        2 => {
            if soc_intel_is_cml() {
                (*card).codec_conf = max_98390_cml_codec_conf.as_mut_ptr();
            }

            /* fallthrough */
            (*card).num_configs = num_codecs;
        }
        4 => {
            (*card).num_configs = num_codecs;
        }
        _ => {
            dev_err(dev, c"invalid codec number %d for %s\n".as_ptr(), num_codecs, MAX_98390_ACPI_HID);
        }
    }
}
// EXPORT_SYMBOL_NS(max_98390_set_codec_conf, "SND_SOC_INTEL_SOF_MAXIM_COMMON");

/*
 * Maxim MAX98357A/MAX98360A
 */
static max_98357a_kcontrols: [snd_kcontrol_new; 1] = [
    SOC_DAPM_PIN_SWITCH!("Spk"),
];

static max_98357a_dapm_widgets: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_SPK!("Spk", core::ptr::null::<c_void>()),
];

static max_98357a_dapm_routes: [snd_soc_dapm_route; 1] = [
    /* speaker */
    snd_soc_dapm_route { sink: c"Spk".as_ptr(), control: core::ptr::null(), source: c"Speaker".as_ptr() },
];

static mut max_98357a_components: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component { name: MAX_98357A_DEV0_NAME, dai_name: MAX_98357A_CODEC_DAI },
];

static mut max_98360a_components: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component { name: MAX_98360A_DEV0_NAME, dai_name: MAX_98357A_CODEC_DAI },
];

unsafe extern "C" fn max_98357a_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, max_98357a_dapm_widgets.as_ptr(), ARRAY_SIZE!(max_98357a_dapm_widgets));
    if ret != 0 {
        dev_err((*rtd).dev, c"unable to add dapm controls, ret %d\n".as_ptr(), ret);
        /* Don't need to add routes if widget addition failed */
        return ret;
    }

    ret = snd_soc_add_card_controls(card, max_98357a_kcontrols.as_ptr(), ARRAY_SIZE!(max_98357a_kcontrols));
    if ret != 0 {
        dev_err((*rtd).dev, c"unable to add card controls, ret %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, max_98357a_dapm_routes.as_ptr(), ARRAY_SIZE!(max_98357a_dapm_routes));

    if ret != 0 {
        dev_err((*rtd).dev, c"unable to add dapm routes, ret %d\n".as_ptr(), ret);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn max_98357a_dai_link(link: *mut snd_soc_dai_link) {
    (*link).codecs = max_98357a_components.as_mut_ptr();
    (*link).num_codecs = ARRAY_SIZE!(max_98357a_components) as c_uint;
    (*link).init = Some(max_98357a_init);
}
// EXPORT_SYMBOL_NS(max_98357a_dai_link, "SND_SOC_INTEL_SOF_MAXIM_COMMON");

#[no_mangle]
pub unsafe extern "C" fn max_98360a_dai_link(link: *mut snd_soc_dai_link) {
    (*link).codecs = max_98360a_components.as_mut_ptr();
    (*link).num_codecs = ARRAY_SIZE!(max_98360a_components) as c_uint;
    (*link).init = Some(max_98357a_init);
}
// EXPORT_SYMBOL_NS(max_98360a_dai_link, "SND_SOC_INTEL_SOF_MAXIM_COMMON");

// MODULE_DESCRIPTION("ASoC Intel SOF Maxim helpers");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
