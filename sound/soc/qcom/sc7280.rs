// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
//
// ALSA SoC Machine driver for sc7280

// C dependencies translated as external Rust dependencies:
// dt-bindings/sound/qcom,lpass.h
// dt-bindings/sound/qcom,q6afe.h
// linux/input.h
// linux/module.h
// linux/platform_device.h
// sound/core.h
// sound/jack.h
// sound/pcm.h
// sound/soc.h
// sound/rt5682s.h
// linux/soundwire/sdw.h
// sound/pcm_params.h
// ../codecs/rt5682.h
// ../codecs/rt5682s.h
// common.h
// lpass.h
// qdsp6/q6afe.h
// sdw.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

const DEFAULT_MCLK_RATE: u32 = 19200000;
const RT5682_PLL_FREQ: u32 = 48000 * 512;
const MI2S_BCLK_RATE: u32 = 1536000;

const LPASS_MAX_PORTS: usize = 128;

type c_int = i32;
type c_uint = u32;
type c_char = i8;
type bool_ = bool;

const NULL: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut core::ffi::c_void,
    ) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut core::ffi::c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut core::ffi::c_void);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_uint,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn qcom_snd_sdw_prepare(substream: *mut snd_pcm_substream, prepared: *mut bool_) -> c_int;
    fn qcom_snd_sdw_hw_free(substream: *mut snd_pcm_substream, prepared: *mut bool_) -> c_int;
    fn qcom_snd_sdw_shutdown(substream: *mut snd_pcm_substream);
    fn qcom_snd_sdw_startup(substream: *mut snd_pcm_substream) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_mask;
    fn snd_mask_set_format(mask: *mut snd_mask, val: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn qcom_snd_parse_of(card: *mut snd_soc_card) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
struct module {
    _private: [u8; 0],
}

#[repr(C)]
struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_jack {
    private_data: *mut core::ffi::c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_jack)>,
}

#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_card {
    owner: *mut module,
    driver_name: *const c_char,
    dev: *mut device,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dai_link: *mut snd_soc_dai_link,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai_link {
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    ops: *const snd_soc_ops,
    no_pcm: c_int,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: driver,
}

#[repr(C)]
struct sc7280_snd_data {
    card: snd_soc_card,
    pri_mi2s_clk_count: u32,
    hs_jack: snd_soc_jack,
    hdmi_jack: snd_soc_jack,
    jack_setup: bool_,
    stream_prepared: [bool_; LPASS_MAX_PORTS],
}

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_LINEOUT: c_int = 0x0004;
const SND_JACK_MECHANICAL: c_int = 0x0008;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const SND_JACK_BTN_4: c_int = 0x0400;
const SND_JACK_BTN_5: c_int = 0x0200;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;

const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 0x246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;

const ENOTSUPP: c_int = 524;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const MI2S_PRIMARY: c_int = 0;
const MI2S_SECONDARY: c_int = 1;
const LPASS_CDC_DMA_RX0: c_int = 2;
const LPASS_CDC_DMA_TX3: c_int = 3;
const TX_CODEC_DMA_TX_3: c_int = 4;
const LPASS_CDC_DMA_VA_TX0: c_int = 5;
const RX_CODEC_DMA_RX_0: c_int = 6;
const SECONDARY_MI2S_RX: c_int = 7;
const VA_CODEC_DMA_TX_0: c_int = 8;
const LPASS_DP_RX: c_int = 9;

const LPASS_MCLK0: c_int = 0;
const Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT: c_int = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_uint = 2;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;

const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;

const RT5682S_PLL2: c_int = 0;
const RT5682S_PLL_S_MCLK: c_int = 0;
const RT5682S_SCLK_S_PLL2: c_int = 0;

unsafe extern "C" fn sc7280_jack_free(jack: *mut snd_jack) {
    let component = (*jack).private_data as *mut snd_soc_component;

    snd_soc_component_set_jack(component, core::ptr::null_mut(), core::ptr::null_mut());
}

static mut sc7280_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn sc7280_headset_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let pdata = snd_soc_card_get_drvdata(card) as *mut sc7280_snd_data;
    let mut codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let component = (*codec_dai).component;
    let jack: *mut snd_jack;
    let mut rval: c_int;
    let mut i: c_int;

    if !(*pdata).jack_setup {
        rval = snd_soc_card_jack_new_pins(
            card,
            b"Headset Jack\0".as_ptr() as *const c_char,
            SND_JACK_HEADSET
                | SND_JACK_LINEOUT
                | SND_JACK_MECHANICAL
                | SND_JACK_BTN_0
                | SND_JACK_BTN_1
                | SND_JACK_BTN_2
                | SND_JACK_BTN_3
                | SND_JACK_BTN_4
                | SND_JACK_BTN_5,
            &mut (*pdata).hs_jack,
            sc7280_jack_pins.as_mut_ptr(),
            sc7280_jack_pins.len() as c_uint,
        );

        if rval < 0 {
            dev_err((*card).dev, b"Unable to add Headset Jack\n\0".as_ptr() as *const c_char);
            return rval;
        }

        jack = (*pdata).hs_jack.jack;

        snd_jack_set_key(jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key(jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key(jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key(jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

        (*jack).private_data = component as *mut core::ffi::c_void;
        (*jack).private_free = Some(sc7280_jack_free);
        (*pdata).jack_setup = true;
    }
    match (*cpu_dai).id {
        MI2S_PRIMARY | LPASS_CDC_DMA_RX0 | LPASS_CDC_DMA_TX3 | TX_CODEC_DMA_TX_3 => {
            i = 0;
            while i < 1 {
                codec_dai = snd_soc_rtd_to_codec(rtd, i);
                rval = snd_soc_component_set_jack(component, &mut (*pdata).hs_jack, NULL);
                if rval != 0 && rval != -ENOTSUPP {
                    dev_err(
                        (*card).dev,
                        b"Failed to set jack: %d\n\0".as_ptr() as *const c_char,
                        rval,
                    );
                    return rval;
                }
                i += 1;
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn sc7280_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let pdata = snd_soc_card_get_drvdata(card) as *mut sc7280_snd_data;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;
    let jack: *mut snd_jack;
    let rval: c_int;

    rval = snd_soc_card_jack_new(
        card,
        b"HDMI Jack\0".as_ptr() as *const c_char,
        SND_JACK_LINEOUT,
        &mut (*pdata).hdmi_jack,
    );

    if rval < 0 {
        dev_err((*card).dev, b"Unable to add HDMI Jack\n\0".as_ptr() as *const c_char);
        return rval;
    }

    jack = (*pdata).hdmi_jack.jack;
    (*jack).private_data = component as *mut core::ffi::c_void;
    (*jack).private_free = Some(sc7280_jack_free);

    snd_soc_component_set_jack(component, &mut (*pdata).hdmi_jack, NULL)
}

unsafe extern "C" fn sc7280_rt5682_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let card = (*rtd).card;
    let data = snd_soc_card_get_drvdata(card) as *mut sc7280_snd_data;
    let mut ret: c_int;

    (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_add(1);
    if (*data).pri_mi2s_clk_count == 1 {
        snd_soc_dai_set_sysclk(
            cpu_dai,
            LPASS_MCLK0,
            DEFAULT_MCLK_RATE,
            SNDRV_PCM_STREAM_PLAYBACK,
        );
    }
    snd_soc_dai_set_fmt(
        codec_dai,
        SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S,
    );

    ret = snd_soc_dai_set_pll(
        codec_dai,
        RT5682S_PLL2,
        RT5682S_PLL_S_MCLK,
        DEFAULT_MCLK_RATE,
        RT5682_PLL_FREQ,
    );
    if ret != 0 {
        dev_err((*rtd).dev, b"can't set codec pll: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5682S_SCLK_S_PLL2,
        RT5682_PLL_FREQ,
        SND_SOC_CLOCK_IN,
    );

    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"snd_soc_dai_set_sysclk err = %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn sc7280_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    match (*cpu_dai).id {
        MI2S_PRIMARY | LPASS_CDC_DMA_TX3 | TX_CODEC_DMA_TX_3 => return sc7280_headset_init(rtd),
        LPASS_CDC_DMA_RX0
        | LPASS_CDC_DMA_VA_TX0
        | MI2S_SECONDARY
        | RX_CODEC_DMA_RX_0
        | SECONDARY_MI2S_RX
        | VA_CODEC_DMA_TX_0 => return 0,
        LPASS_DP_RX => return sc7280_hdmi_init(rtd),
        _ => {
            dev_err(
                (*rtd).dev,
                b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const c_char,
                b"sc7280_init\0".as_ptr() as *const c_char,
                (*cpu_dai).id,
            );
        }
    }

    -EINVAL
}

unsafe extern "C" fn sc7280_snd_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);

    if (*(*rtd).dai_link).no_pcm == 0 {
        snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 2, 2);
        snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_RATE, 48000, 48000);
    }

    0
}

unsafe extern "C" fn sc7280_snd_swr_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sc7280_snd_data;

    qcom_snd_sdw_prepare(
        substream,
        &mut (*data).stream_prepared[(*cpu_dai).id as usize],
    )
}

unsafe extern "C" fn sc7280_snd_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    match (*cpu_dai).id {
        LPASS_CDC_DMA_RX0
        | LPASS_CDC_DMA_TX3
        | RX_CODEC_DMA_RX_0
        | TX_CODEC_DMA_TX_3
        | VA_CODEC_DMA_TX_0 => return sc7280_snd_swr_prepare(substream),
        _ => {}
    }

    0
}

unsafe extern "C" fn sc7280_snd_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sc7280_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    qcom_snd_sdw_hw_free(
        substream,
        &mut (*data).stream_prepared[(*cpu_dai).id as usize],
    )
}

unsafe extern "C" fn sc7280_snd_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let data = snd_soc_card_get_drvdata(card) as *mut sc7280_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    match (*cpu_dai).id {
        MI2S_PRIMARY => {
            (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_sub(1);
            if (*data).pri_mi2s_clk_count == 0 {
                snd_soc_dai_set_sysclk(cpu_dai, LPASS_MCLK0, 0, SNDRV_PCM_STREAM_PLAYBACK);
            }
        }
        SECONDARY_MI2S_RX => {
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT,
                0,
                SNDRV_PCM_STREAM_PLAYBACK,
            );
        }
        _ => {}
    }

    qcom_snd_sdw_shutdown(substream);
}

unsafe extern "C" fn sc7280_snd_startup(substream: *mut snd_pcm_substream) -> c_int {
    let fmt: c_uint = SND_SOC_DAIFMT_CBC_CFC;
    let mut codec_dai_fmt: c_uint = SND_SOC_DAIFMT_CBC_CFC;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int = 0;

    match (*cpu_dai).id {
        MI2S_PRIMARY => {
            ret = sc7280_rt5682_init(rtd);
            if ret != 0 {
                return ret;
            }
        }
        SECONDARY_MI2S_RX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;

            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT,
                MI2S_BCLK_RATE,
                SNDRV_PCM_STREAM_PLAYBACK,
            );

            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        _ => {}
    }

    qcom_snd_sdw_startup(substream)
}

static sc7280_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(sc7280_snd_startup),
    hw_params: Some(sc7280_snd_hw_params),
    hw_free: Some(sc7280_snd_hw_free),
    prepare: Some(sc7280_snd_prepare),
    shutdown: Some(sc7280_snd_shutdown),
};

// SND_SOC_DAPM_HP("Headphone Jack", NULL), SND_SOC_DAPM_MIC("Headset Mic", NULL)
static sc7280_snd_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

// SOC_DAPM_PIN_SWITCH("Headphone Jack"), SOC_DAPM_PIN_SWITCH("Headset Mic")
static sc7280_snd_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn sc7280_snd_be_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    (*rate).min = 48000;
    (*rate).max = (*rate).min;
    (*channels).min = 2;
    (*channels).max = (*channels).min;
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);

    0
}

unsafe extern "C" fn sc7280_snd_platform_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card;
    let data: *mut sc7280_snd_data;
    let dev = &mut (*pdev).dev as *mut device;
    let mut link: *mut snd_soc_dai_link;
    let mut ret: c_int;
    let mut i: c_int;

    data = devm_kzalloc(
        dev,
        core::mem::size_of::<sc7280_snd_data>(),
        GFP_KERNEL,
    ) as *mut sc7280_snd_data;
    if data.is_null() {
        return -ENOMEM;
    }

    card = &mut (*data).card;
    snd_soc_card_set_drvdata(card, data as *mut core::ffi::c_void);

    (*card).owner = THIS_MODULE;
    (*card).driver_name = b"SC7280\0".as_ptr() as *const c_char;
    (*card).dev = dev;

    (*card).dapm_widgets = sc7280_snd_widgets.as_ptr();
    (*card).num_dapm_widgets = sc7280_snd_widgets.len() as c_uint;
    (*card).controls = sc7280_snd_controls.as_ptr();
    (*card).num_controls = sc7280_snd_controls.len() as c_uint;

    ret = qcom_snd_parse_of(card);
    if ret != 0 {
        return ret;
    }

    // for_each_card_prelinks(card, i, link)
    i = 0;
    while i < 0 {
        link = core::ptr::null_mut();
        (*link).init = Some(sc7280_init);
        (*link).ops = &sc7280_ops;
        if (*link).no_pcm == 1 {
            (*link).be_hw_params_fixup = Some(sc7280_snd_be_hw_params_fixup);
        }
        i += 1;
    }

    devm_snd_soc_register_card(dev, card)
}

static sc7280_snd_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"google,sc7280-herobrine\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, sc7280_snd_device_id);

static mut sc7280_snd_driver: platform_driver = platform_driver {
    probe: Some(sc7280_snd_platform_probe),
    driver: driver {
        name: b"msm-snd-sc7280\0".as_ptr() as *const c_char,
        of_match_table: sc7280_snd_device_id.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
};
// module_platform_driver(sc7280_snd_driver);

// MODULE_DESCRIPTION("sc7280 ASoC Machine Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
