// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek MT8365 Sound Card driver
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Nicolas Belin <nbelin@baylibre.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_SOC_DPCM_TRIGGER_POST: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;

#[repr(C)]
struct pinctrl {
    _private: [u8; 0],
}

#[repr(C)]
struct pinctrl_state {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct module {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    private_data: *mut c_void,
    stream: c_int,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
struct mtk_soc_card_data {
    card_data: *mut mtk_platform_card_data,
    mach_priv: *mut mt8365_mt6357_priv,
}

#[repr(C)]
struct mtk_platform_card_data {
    card: *mut snd_soc_card,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    id: c_int,
    trigger: [c_uint; 2],
    dynamic: c_uint,
    playback_only: c_uint,
    capture_only: c_uint,
    dpcm_merged_rate: c_uint,
    no_pcm: c_uint,
    dai_fmt: c_uint,
    ops: *const snd_soc_ops,
}

#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    owner: *mut module,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    dev: *mut device,
}

#[repr(C)]
struct mtk_soundcard_pdata {
    card_name: *const c_char,
    card_data: *const mtk_platform_card_data,
    soc_probe: Option<unsafe extern "C" fn(*mut mtk_soc_card_data, bool_) -> c_int>,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    driver: platform_driver_inner,
    probe: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut mtk_soc_card_data;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut mtk_soc_card_data);
    fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;
    fn pinctrl_lookup_state(pinctrl: *mut pinctrl, name: *const c_char) -> *mut pinctrl_state;
    fn pinctrl_select_state(pinctrl: *mut pinctrl, state: *mut pinctrl_state) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn parse_dai_link_info(card: *mut snd_soc_card) -> c_int;
    fn clean_card_reference(card: *mut snd_soc_card);
    fn mtk_soundcard_common_probe(pdev: *mut c_void) -> c_int;
}

#[repr(C)]
enum pinctrl_pin_state {
    PIN_STATE_DEFAULT,
    PIN_STATE_DMIC,
    PIN_STATE_MISO_OFF,
    PIN_STATE_MISO_ON,
    PIN_STATE_MOSI_OFF,
    PIN_STATE_MOSI_ON,
    PIN_STATE_MAX,
}

const PIN_STATE_DEFAULT: usize = pinctrl_pin_state::PIN_STATE_DEFAULT as usize;
const PIN_STATE_MISO_OFF: usize = pinctrl_pin_state::PIN_STATE_MISO_OFF as usize;
const PIN_STATE_MISO_ON: usize = pinctrl_pin_state::PIN_STATE_MISO_ON as usize;
const PIN_STATE_MOSI_OFF: usize = pinctrl_pin_state::PIN_STATE_MOSI_OFF as usize;
const PIN_STATE_MOSI_ON: usize = pinctrl_pin_state::PIN_STATE_MOSI_ON as usize;
const PIN_STATE_MAX: usize = pinctrl_pin_state::PIN_STATE_MAX as usize;

static mt8365_mt6357_pin_str: [*const c_char; PIN_STATE_MAX] = [
    b"default\0".as_ptr() as *const c_char,
    b"dmic\0".as_ptr() as *const c_char,
    b"miso_off\0".as_ptr() as *const c_char,
    b"miso_on\0".as_ptr() as *const c_char,
    b"mosi_off\0".as_ptr() as *const c_char,
    b"mosi_on\0".as_ptr() as *const c_char,
];

#[repr(C)]
struct mt8365_mt6357_priv {
    pinctrl: *mut pinctrl,
    pin_states: [*mut pinctrl_state; PIN_STATE_MAX],
}

const DAI_LINK_DL1_PLAYBACK: c_int = 0;
const DAI_LINK_DL2_PLAYBACK: c_int = 1;
const DAI_LINK_AWB_CAPTURE: c_int = 2;
const DAI_LINK_VUL_CAPTURE: c_int = 3;
const DAI_LINK_2ND_I2S_INTF: c_int = 4;
const DAI_LINK_DMIC: c_int = 5;
const DAI_LINK_INT_ADDA: c_int = 6;
const DAI_LINK_NUM: c_int = 7;

/* SND_SOC_DAPM_OUTPUT("HDMI Out") */
static mt8365_mt6357_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget { _private: [] }];

static mt8365_mt6357_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"HDMI Out\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"2ND I2S Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DMIC In\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"MICBIAS0\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn mt8365_mt6357_int_adda_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let priv_ = (*soc_card_data).mach_priv;
    let mut ret: c_int = 0;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if IS_ERR((*priv_).pin_states[PIN_STATE_MOSI_ON] as *const c_void) {
            return ret;
        }

        ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_STATE_MOSI_ON]);
        if ret != 0 {
            dev_err(
                (*(*rtd).card).dev,
                b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
                b"mt8365_mt6357_int_adda_startup\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        if IS_ERR((*priv_).pin_states[PIN_STATE_MISO_ON] as *const c_void) {
            return ret;
        }

        ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_STATE_MISO_ON]);
        if ret != 0 {
            dev_err(
                (*(*rtd).card).dev,
                b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
                b"mt8365_mt6357_int_adda_startup\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    0
}

unsafe extern "C" fn mt8365_mt6357_int_adda_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let priv_ = (*soc_card_data).mach_priv;
    let mut ret: c_int = 0;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if IS_ERR((*priv_).pin_states[PIN_STATE_MOSI_OFF] as *const c_void) {
            return;
        }

        ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_STATE_MOSI_OFF]);
        if ret != 0 {
            dev_err(
                (*(*rtd).card).dev,
                b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
                b"mt8365_mt6357_int_adda_shutdown\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        if IS_ERR((*priv_).pin_states[PIN_STATE_MISO_OFF] as *const c_void) {
            return;
        }

        ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_STATE_MISO_OFF]);
        if ret != 0 {
            dev_err(
                (*(*rtd).card).dev,
                b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
                b"mt8365_mt6357_int_adda_shutdown\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }
}

static mt8365_mt6357_int_adda_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8365_mt6357_int_adda_startup),
    shutdown: Some(mt8365_mt6357_int_adda_shutdown),
};

/*
 * SND_SOC_DAILINK_DEFS(playback1,
 *                      DAILINK_COMP_ARRAY(COMP_CPU("DL1")),
 *                      DAILINK_COMP_ARRAY(COMP_DUMMY()),
 *                      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 * SND_SOC_DAILINK_DEFS(playback2,
 *                      DAILINK_COMP_ARRAY(COMP_CPU("DL2")),
 *                      DAILINK_COMP_ARRAY(COMP_DUMMY()),
 *                      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 * SND_SOC_DAILINK_DEFS(awb_capture,
 *                      DAILINK_COMP_ARRAY(COMP_CPU("AWB")),
 *                      DAILINK_COMP_ARRAY(COMP_DUMMY()),
 *                      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 * SND_SOC_DAILINK_DEFS(vul,
 *                      DAILINK_COMP_ARRAY(COMP_CPU("VUL")),
 *                      DAILINK_COMP_ARRAY(COMP_DUMMY()),
 *                      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 *
 * SND_SOC_DAILINK_DEFS(i2s3,
 *                      DAILINK_COMP_ARRAY(COMP_CPU("2ND I2S")),
 *                      DAILINK_COMP_ARRAY(COMP_DUMMY()),
 *                      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 * SND_SOC_DAILINK_DEFS(dmic,
 *                      DAILINK_COMP_ARRAY(COMP_CPU("DMIC")),
 *                      DAILINK_COMP_ARRAY(COMP_DUMMY()),
 *                      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 * SND_SOC_DAILINK_DEFS(primary_codec,
 *                      DAILINK_COMP_ARRAY(COMP_CPU("INT ADDA")),
 *                      DAILINK_COMP_ARRAY(COMP_CODEC("mt6357-sound", "mt6357-snd-codec-aif1")),
 *                      DAILINK_COMP_ARRAY(COMP_EMPTY()));
 */

/* Digital audio interface glue - connects codec <---> CPU */
static mut mt8365_mt6357_dais: [snd_soc_dai_link; DAI_LINK_NUM as usize] = [
    snd_soc_dai_link {
        name: b"DL1_FE\0".as_ptr() as *const c_char,
        stream_name: b"MultiMedia1_PLayback\0".as_ptr() as *const c_char,
        id: DAI_LINK_DL1_PLAYBACK,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        dpcm_merged_rate: 1,
        no_pcm: 0,
        dai_fmt: 0,
        ops: ptr::null(),
    },
    snd_soc_dai_link {
        name: b"DL2_FE\0".as_ptr() as *const c_char,
        stream_name: b"MultiMedia2_PLayback\0".as_ptr() as *const c_char,
        id: DAI_LINK_DL2_PLAYBACK,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 1,
        capture_only: 0,
        dpcm_merged_rate: 1,
        no_pcm: 0,
        dai_fmt: 0,
        ops: ptr::null(),
    },
    snd_soc_dai_link {
        name: b"AWB_FE\0".as_ptr() as *const c_char,
        stream_name: b"DL1_AWB_Record\0".as_ptr() as *const c_char,
        id: DAI_LINK_AWB_CAPTURE,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        dpcm_merged_rate: 1,
        no_pcm: 0,
        dai_fmt: 0,
        ops: ptr::null(),
    },
    snd_soc_dai_link {
        name: b"VUL_FE\0".as_ptr() as *const c_char,
        stream_name: b"MultiMedia1_Capture\0".as_ptr() as *const c_char,
        id: DAI_LINK_VUL_CAPTURE,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        dynamic: 1,
        playback_only: 0,
        capture_only: 1,
        dpcm_merged_rate: 1,
        no_pcm: 0,
        dai_fmt: 0,
        ops: ptr::null(),
    },
    snd_soc_dai_link {
        name: b"I2S_OUT_BE\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        id: DAI_LINK_2ND_I2S_INTF,
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 0,
        dpcm_merged_rate: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ops: ptr::null(),
    },
    snd_soc_dai_link {
        name: b"DMIC_BE\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        id: DAI_LINK_DMIC,
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 1,
        dpcm_merged_rate: 0,
        no_pcm: 1,
        dai_fmt: 0,
        ops: ptr::null(),
    },
    snd_soc_dai_link {
        name: b"MTK_Codec\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        id: DAI_LINK_INT_ADDA,
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 0,
        capture_only: 0,
        dpcm_merged_rate: 0,
        no_pcm: 1,
        dai_fmt: 0,
        ops: &mt8365_mt6357_int_adda_ops,
    },
];

unsafe extern "C" fn mt8365_mt6357_gpio_probe(card: *mut snd_soc_card) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata(card);
    let priv_ = (*soc_card_data).mach_priv;
    let dev = (*card).dev;
    let mut ret: c_int;

    (*priv_).pinctrl = devm_pinctrl_get(dev);
    if IS_ERR((*priv_).pinctrl as *const c_void) {
        ret = PTR_ERR((*priv_).pinctrl as *const c_void);
        return dev_err_probe(
            dev,
            ret,
            b"Failed to get pinctrl\n\0".as_ptr() as *const c_char,
        );
    }

    let mut i = PIN_STATE_DEFAULT;
    while i < PIN_STATE_MAX {
        (*priv_).pin_states[i] =
            pinctrl_lookup_state((*priv_).pinctrl, mt8365_mt6357_pin_str[i]);
        if IS_ERR((*priv_).pin_states[i] as *const c_void) {
            dev_info(
                dev,
                b"No pin state for %s\n\0".as_ptr() as *const c_char,
                mt8365_mt6357_pin_str[i],
            );
        } else {
            ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[i]);
            if ret != 0 {
                dev_err_probe(
                    dev,
                    ret,
                    b"Failed to select pin state %s\n\0".as_ptr() as *const c_char,
                    mt8365_mt6357_pin_str[i],
                );
                return ret;
            }
        }
        i += 1;
    }
    0
}

static mut mt8365_mt6357_soc_card: snd_soc_card = snd_soc_card {
    name: b"mt8365-evk\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { mt8365_mt6357_dais.as_mut_ptr() },
    num_links: (size_of::<[snd_soc_dai_link; DAI_LINK_NUM as usize]>()
        / size_of::<snd_soc_dai_link>()) as c_int,
    dapm_widgets: mt8365_mt6357_widgets.as_ptr(),
    num_dapm_widgets: (size_of::<[snd_soc_dapm_widget; 1]>() / size_of::<snd_soc_dapm_widget>())
        as c_int,
    dapm_routes: mt8365_mt6357_routes.as_ptr(),
    num_dapm_routes: (size_of::<[snd_soc_dapm_route; 2]>() / size_of::<snd_soc_dapm_route>())
        as c_int,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn mt8365_mt6357_dev_probe(
    soc_card_data: *mut mtk_soc_card_data,
    _legacy: bool_,
) -> c_int {
    let card_data = (*soc_card_data).card_data;
    let card = (*card_data).card;
    let dev = (*card).dev;
    let mach_priv: *mut mt8365_mt6357_priv;
    let mut ret: c_int;

    ret = parse_dai_link_info(card);
    if ret != 0 {
        clean_card_reference(card);
        return ret;
    }

    mach_priv = devm_kzalloc(dev, size_of::<mt8365_mt6357_priv>(), GFP_KERNEL)
        as *mut mt8365_mt6357_priv;
    if mach_priv.is_null() {
        return -ENOMEM;
    }
    (*soc_card_data).mach_priv = mach_priv;
    snd_soc_card_set_drvdata(card, soc_card_data);
    mt8365_mt6357_gpio_probe(card);
    0
}

static mt8365_mt6357_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8365_mt6357_soc_card },
};

static mt8365_mt6357_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: b"mt8365-mt6357\0".as_ptr() as *const c_char,
    card_data: &mt8365_mt6357_card_data,
    soc_probe: Some(mt8365_mt6357_dev_probe),
};

static mt8365_mt6357_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8365-mt6357\0".as_ptr() as *const c_char,
        data: &mt8365_mt6357_card as *const mtk_soundcard_pdata as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, mt8365_mt6357_dt_match); */

static mt8365_mt6357_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"mt8365_mt6357\0".as_ptr() as *const c_char,
        of_match_table: mt8365_mt6357_dt_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(mtk_soundcard_common_probe),
};

/* module_platform_driver(mt8365_mt6357_driver); */

/* Module information */
/* MODULE_DESCRIPTION("MT8365 EVK SoC machine driver"); */
/* MODULE_AUTHOR("Nicolas Belin <nbelin@baylibre.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform: mt8365_mt6357"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
