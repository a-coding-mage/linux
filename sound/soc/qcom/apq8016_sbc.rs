// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 The Linux Foundation. All rights reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u32 = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
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
pub struct snd_soc_dai {
    pub id: c_int,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub no_pcm: bool_,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut module,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub components: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct apq8016_sbc_data {
    pub card: snd_soc_card,
    pub mic_iomux: *mut c_void,
    pub spkr_iomux: *mut c_void,
    pub jack: snd_soc_jack,
    pub jack_setup: bool_,
    pub mi2s_clk_count: [c_int; MI2S_COUNT as usize],
}

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    static mut MI2S_PRIMARY: c_int;
    static mut MI2S_SECONDARY: c_int;
    static mut MI2S_TERTIARY: c_int;
    static mut MI2S_QUATERNARY: c_int;
    static mut PRIMARY_MI2S_RX: c_int;
    static mut PRIMARY_MI2S_TX: c_int;
    static mut SECONDARY_MI2S_RX: c_int;
    static mut SECONDARY_MI2S_TX: c_int;
    static mut TERTIARY_MI2S_RX: c_int;
    static mut TERTIARY_MI2S_TX: c_int;
    static mut QUATERNARY_MI2S_RX: c_int;
    static mut QUATERNARY_MI2S_TX: c_int;

    static mut SND_JACK_MICROPHONE: c_int;
    static mut SND_JACK_HEADPHONE: c_int;
    static mut SND_JACK_HEADSET: c_int;
    static mut SND_JACK_BTN_0: c_int;
    static mut SND_JACK_BTN_1: c_int;
    static mut SND_JACK_BTN_2: c_int;
    static mut SND_JACK_BTN_3: c_int;
    static mut SND_JACK_BTN_4: c_int;
    static mut KEY_PLAYPAUSE: c_int;
    static mut KEY_VOICECOMMAND: c_int;
    static mut KEY_VOLUMEUP: c_int;
    static mut KEY_VOLUMEDOWN: c_int;
    static mut EINVAL: c_int;
    static mut ENOTSUPP: c_int;
    static mut ENOMEM: c_int;
    static mut GFP_KERNEL: c_uint;
    static mut SND_SOC_CLOCK_IN: c_int;
    static mut SND_SOC_DAIFMT_BP_FP: c_uint;
    static mut LPAIF_BIT_CLK: c_int;
    static mut SNDRV_PCM_HW_PARAM_RATE: c_int;
    static mut SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static mut SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static mut SNDRV_PCM_FORMAT_S16_LE: c_int;

    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn device_get_match_data(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn qcom_snd_parse_of(card: *mut snd_soc_card) -> c_int;
    fn devm_platform_ioremap_resource_byname(
        pdev: *mut platform_device,
        name: *const c_char,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

const MI2S_COUNT: usize = 4;

const MIC_CTRL_TER_WS_SLAVE_SEL: u32 = BIT(21);
const MIC_CTRL_QUA_WS_SLAVE_SEL_10: u32 = BIT(17);
const MIC_CTRL_TLMM_SCLK_EN: u32 = BIT(1);
const SPKR_CTL_PRI_WS_SLAVE_SEL_11: u32 = BIT(17) | BIT(16);
const SPKR_CTL_TLMM_MCLK_EN: u32 = BIT(1);
const SPKR_CTL_TLMM_SCLK_EN: u32 = BIT(2);
const SPKR_CTL_TLMM_DATA1_EN: u32 = BIT(3);
const SPKR_CTL_TLMM_WS_OUT_SEL_MASK: u32 = GENMASK(7, 6);
const SPKR_CTL_TLMM_WS_OUT_SEL_SEC: u32 = BIT(6);
const SPKR_CTL_TLMM_WS_EN_SEL_MASK: u32 = GENMASK(19, 18);
const SPKR_CTL_TLMM_WS_EN_SEL_SEC: u32 = BIT(18);
const DEFAULT_MCLK_RATE: u32 = 9600000;
const MI2S_BCLK_RATE: u32 = 1536000;

static mut apq8016_sbc_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Mic Jack".as_ptr(),
        mask: unsafe { SND_JACK_MICROPHONE },
    },
    snd_soc_jack_pin {
        pin: c"Headphone Jack".as_ptr(),
        mask: unsafe { SND_JACK_HEADPHONE },
    },
];

unsafe extern "C" fn apq8016_dai_init(rtd: *mut snd_soc_pcm_runtime, mi2s: c_int) -> c_int {
    let mut codec_dai: *mut snd_soc_dai;
    let mut component: *mut snd_soc_component;
    let card: *mut snd_soc_card = (*rtd).card;
    let pdata: *mut apq8016_sbc_data = snd_soc_card_get_drvdata(card) as *mut apq8016_sbc_data;
    let mut i: c_int;
    let mut rval: c_int;
    let mut value: u32;

    if mi2s == MI2S_PRIMARY {
        writel(
            readl((*pdata).spkr_iomux) | SPKR_CTL_PRI_WS_SLAVE_SEL_11,
            (*pdata).spkr_iomux,
        );
    } else if mi2s == MI2S_QUATERNARY {
        /* Configure the Quat MI2S to TLMM */
        writel(
            readl((*pdata).mic_iomux)
                | MIC_CTRL_QUA_WS_SLAVE_SEL_10
                | MIC_CTRL_TLMM_SCLK_EN,
            (*pdata).mic_iomux,
        );
    } else if mi2s == MI2S_SECONDARY {
        /* Clear TLMM_WS_OUT_SEL and TLMM_WS_EN_SEL fields */
        value = readl((*pdata).spkr_iomux)
            & !(SPKR_CTL_TLMM_WS_OUT_SEL_MASK | SPKR_CTL_TLMM_WS_EN_SEL_MASK);
        /* Configure the Sec MI2S to TLMM */
        writel(
            value
                | SPKR_CTL_TLMM_MCLK_EN
                | SPKR_CTL_TLMM_SCLK_EN
                | SPKR_CTL_TLMM_DATA1_EN
                | SPKR_CTL_TLMM_WS_OUT_SEL_SEC
                | SPKR_CTL_TLMM_WS_EN_SEL_SEC,
            (*pdata).spkr_iomux,
        );
    } else if mi2s == MI2S_TERTIARY {
        writel(
            readl((*pdata).mic_iomux) | MIC_CTRL_TER_WS_SLAVE_SEL | MIC_CTRL_TLMM_SCLK_EN,
            (*pdata).mic_iomux,
        );
    } else {
        dev_err((*card).dev, c"unsupported cpu dai configuration\n".as_ptr());
        return -EINVAL;
    }

    if !(*pdata).jack_setup {
        let jack: *mut snd_jack;

        rval = snd_soc_card_jack_new_pins(
            card,
            c"Headset Jack".as_ptr(),
            SND_JACK_HEADSET
                | SND_JACK_HEADPHONE
                | SND_JACK_BTN_0
                | SND_JACK_BTN_1
                | SND_JACK_BTN_2
                | SND_JACK_BTN_3
                | SND_JACK_BTN_4,
            &mut (*pdata).jack,
            apq8016_sbc_jack_pins.as_mut_ptr(),
            apq8016_sbc_jack_pins.len() as c_uint,
        );

        if rval < 0 {
            dev_err((*card).dev, c"Unable to add Headphone Jack\n".as_ptr());
            return rval;
        }

        jack = (*pdata).jack.jack;

        snd_jack_set_key(jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key(jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key(jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key(jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
        (*pdata).jack_setup = true;
    }

    /*
     * C used for_each_rtd_codec_dais(rtd, i, codec_dai). The iterator source is
     * external to this file; preserve the translated loop body for that macro.
     */
    i = 0;
    while for_each_rtd_codec_dais_next(rtd, &mut i, &mut codec_dai) {
        component = (*codec_dai).component;
        /* Set default mclk for internal codec */
        rval = snd_soc_component_set_sysclk(
            component,
            0,
            0,
            DEFAULT_MCLK_RATE,
            SND_SOC_CLOCK_IN,
        );
        if rval != 0 && rval != -ENOTSUPP {
            dev_warn((*card).dev, c"Failed to set mclk: %d\n".as_ptr(), rval);
            return rval;
        }
        rval = snd_soc_component_set_jack(component, &mut (*pdata).jack, ptr::null_mut());
        if rval != 0 && rval != -ENOTSUPP {
            dev_warn((*card).dev, c"Failed to set jack: %d\n".as_ptr(), rval);
            return rval;
        }
    }

    0
}

unsafe extern "C" {
    fn for_each_rtd_codec_dais_next(
        rtd: *mut snd_soc_pcm_runtime,
        i: *mut c_int,
        codec_dai: *mut *mut snd_soc_dai,
    ) -> bool_;
    fn for_each_card_prelinks_next(
        card: *mut snd_soc_card,
        i: *mut c_int,
        link: *mut *mut snd_soc_dai_link,
    ) -> bool_;
}

unsafe extern "C" fn apq8016_sbc_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);

    apq8016_dai_init(rtd, (*cpu_dai).id)
}

unsafe extern "C" fn apq8016_sbc_add_ops(card: *mut snd_soc_card) {
    let mut link: *mut snd_soc_dai_link;
    let mut i: c_int = 0;

    while for_each_card_prelinks_next(card, &mut i, &mut link) {
        (*link).init = Some(apq8016_sbc_dai_init);
    }
}

unsafe extern "C" fn qdsp6_dai_get_lpass_id(cpu_dai: *mut snd_soc_dai) -> c_int {
    if (*cpu_dai).id == PRIMARY_MI2S_RX || (*cpu_dai).id == PRIMARY_MI2S_TX {
        MI2S_PRIMARY
    } else if (*cpu_dai).id == SECONDARY_MI2S_RX || (*cpu_dai).id == SECONDARY_MI2S_TX {
        MI2S_SECONDARY
    } else if (*cpu_dai).id == TERTIARY_MI2S_RX || (*cpu_dai).id == TERTIARY_MI2S_TX {
        MI2S_TERTIARY
    } else if (*cpu_dai).id == QUATERNARY_MI2S_RX || (*cpu_dai).id == QUATERNARY_MI2S_TX {
        MI2S_QUATERNARY
    } else {
        -EINVAL
    }
}

unsafe extern "C" fn msm8916_qdsp6_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);

    snd_soc_dai_set_fmt(cpu_dai, SND_SOC_DAIFMT_BP_FP);
    apq8016_dai_init(rtd, qdsp6_dai_get_lpass_id(cpu_dai))
}

unsafe extern "C" fn msm8916_qdsp6_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let card: *mut snd_soc_card = (*rtd).card;
    let data: *mut apq8016_sbc_data = snd_soc_card_get_drvdata(card) as *mut apq8016_sbc_data;
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut mi2s: c_int;
    let ret: c_int;

    mi2s = qdsp6_dai_get_lpass_id(cpu_dai);
    if mi2s < 0 {
        return mi2s;
    }

    (*data).mi2s_clk_count[mi2s as usize] += 1;
    if (*data).mi2s_clk_count[mi2s as usize] > 1 {
        return 0;
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, LPAIF_BIT_CLK, MI2S_BCLK_RATE, 0);
    if ret != 0 {
        dev_err((*card).dev, c"Failed to enable LPAIF bit clk: %d\n".as_ptr(), ret);
    }
    ret
}

unsafe extern "C" fn msm8916_qdsp6_shutdown(substream: *mut snd_pcm_substream) {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let card: *mut snd_soc_card = (*rtd).card;
    let data: *mut apq8016_sbc_data = snd_soc_card_get_drvdata(card) as *mut apq8016_sbc_data;
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut mi2s: c_int;
    let ret: c_int;

    mi2s = qdsp6_dai_get_lpass_id(cpu_dai);
    if mi2s < 0 {
        return;
    }

    (*data).mi2s_clk_count[mi2s as usize] -= 1;
    if (*data).mi2s_clk_count[mi2s as usize] > 0 {
        return;
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, LPAIF_BIT_CLK, 0, 0);
    if ret != 0 {
        dev_err((*card).dev, c"Failed to disable LPAIF bit clk: %d\n".as_ptr(), ret);
    }
}

static msm8916_qdsp6_be_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(msm8916_qdsp6_startup),
    shutdown: Some(msm8916_qdsp6_shutdown),
};

unsafe extern "C" fn msm8916_qdsp6_be_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let fmt: *mut snd_mask = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);

    0
}

unsafe extern "C" fn msm8916_qdsp6_add_ops(card: *mut snd_soc_card) {
    let mut link: *mut snd_soc_dai_link;
    let mut i: c_int = 0;

    /* Make it obvious to userspace that QDSP6 is used */
    (*card).components = c"qdsp6".as_ptr();

    while for_each_card_prelinks_next(card, &mut i, &mut link) {
        if (*link).no_pcm {
            (*link).init = Some(msm8916_qdsp6_dai_init);
            (*link).ops = &msm8916_qdsp6_be_ops;
            (*link).be_hw_params_fixup = Some(msm8916_qdsp6_be_hw_params_fixup);
        }
    }
}

/* SOC_DAPM_PIN_SWITCH entries are macro-generated kernel control definitions. */
static apq8016_sbc_snd_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

/* SND_SOC_DAPM_* entries are macro-generated kernel DAPM widget definitions. */
static apq8016_sbc_dapm_widgets: [snd_soc_dapm_widget; 7] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

unsafe extern "C" fn apq8016_sbc_platform_probe(pdev: *mut platform_device) -> c_int {
    let add_ops: Option<unsafe extern "C" fn(*mut snd_soc_card)>;
    let dev: *mut device = &mut (*pdev).dev;
    let card: *mut snd_soc_card;
    let data: *mut apq8016_sbc_data;
    let mut ret: c_int;

    add_ops = core::mem::transmute(device_get_match_data(&mut (*pdev).dev));
    if add_ops.is_none() {
        return -EINVAL;
    }

    data = devm_kzalloc(dev, core::mem::size_of::<apq8016_sbc_data>(), GFP_KERNEL)
        as *mut apq8016_sbc_data;
    if data.is_null() {
        return -ENOMEM;
    }

    card = &mut (*data).card;
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dapm_widgets = apq8016_sbc_dapm_widgets.as_ptr();
    (*card).num_dapm_widgets = apq8016_sbc_dapm_widgets.len() as c_uint;
    (*card).controls = apq8016_sbc_snd_controls.as_ptr();
    (*card).num_controls = apq8016_sbc_snd_controls.len() as c_uint;

    ret = qcom_snd_parse_of(card);
    if ret != 0 {
        return ret;
    }

    (*data).mic_iomux = devm_platform_ioremap_resource_byname(pdev, c"mic-iomux".as_ptr());
    if IS_ERR((*data).mic_iomux) {
        return PTR_ERR((*data).mic_iomux);
    }

    (*data).spkr_iomux = devm_platform_ioremap_resource_byname(pdev, c"spkr-iomux".as_ptr());
    if IS_ERR((*data).spkr_iomux) {
        return PTR_ERR((*data).spkr_iomux);
    }

    snd_soc_card_set_drvdata(card, data as *mut c_void);

    add_ops.unwrap()(card);
    devm_snd_soc_register_card(&mut (*pdev).dev, card)
}

static apq8016_sbc_device_id: [of_device_id; 3] = [
    of_device_id {
        compatible: c"qcom,apq8016-sbc-sndcard".as_ptr(),
        data: apq8016_sbc_add_ops as *const c_void,
    },
    of_device_id {
        compatible: c"qcom,msm8916-qdsp6-sndcard".as_ptr(),
        data: msm8916_qdsp6_add_ops as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, apq8016_sbc_device_id); */

static mut apq8016_sbc_platform_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: c"qcom-apq8016-sbc".as_ptr(),
        of_match_table: apq8016_sbc_device_id.as_ptr(),
    },
    probe: Some(apq8016_sbc_platform_probe),
};

/* module_platform_driver(apq8016_sbc_platform_driver); */

/* MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org"); */
/* MODULE_DESCRIPTION("APQ8016 ASoC Machine Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
