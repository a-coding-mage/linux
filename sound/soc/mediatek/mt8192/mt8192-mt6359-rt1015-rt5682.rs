// SPDX-License-Identifier: GPL-2.0
//
// mt8192-mt6359-rt1015-rt5682.c  --
//	MT8192-MT6359-RT1015-RT6358 ALSA SoC machine driver
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const DRIVER_NAME: *const c_char = b"mt8192_mt6359\0".as_ptr() as *const c_char;

const RT1015_CODEC_DAI: *const c_char = b"rt1015-aif\0".as_ptr() as *const c_char;
const RT1015_DEV0_NAME: *const c_char = b"rt1015.1-0028\0".as_ptr() as *const c_char;
const RT1015_DEV1_NAME: *const c_char = b"rt1015.1-0029\0".as_ptr() as *const c_char;

const RT1015_RT5682_CARD_NAME: *const c_char =
    b"mt8192_mt6359_rt1015_rt5682\0".as_ptr() as *const c_char;
const RT1015P_RT5682_CARD_NAME: *const c_char =
    b"mt8192_mt6359_rt1015p_rt5682\0".as_ptr() as *const c_char;
const RT1015P_RT5682S_CARD_NAME: *const c_char =
    b"mt8192_mt6359_rt1015p_rt5682s\0".as_ptr() as *const c_char;

const RT1015_RT5682_OF_NAME: *const c_char =
    b"mediatek,mt8192_mt6359_rt1015_rt5682\0".as_ptr() as *const c_char;
const RT1015P_RT5682_OF_NAME: *const c_char =
    b"mediatek,mt8192_mt6359_rt1015p_rt5682\0".as_ptr() as *const c_char;
const RT1015P_RT5682S_OF_NAME: *const c_char =
    b"mediatek,mt8192_mt6359_rt1015p_rt5682s\0".as_ptr() as *const c_char;

const MT8192_JACK_HEADSET: usize = 0;
const MT8192_JACK_HDMI: usize = 1;
const MT8192_JACK_MAX: usize = 2;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub platform_priv: *mut mt8192_afe_private,
}
#[repr(C)]
pub struct mt8192_afe_private {
    pub topckgen: *mut regmap,
    pub mtkaif_calibration_num_phase: c_int,
    pub mtkaif_chosen_phase: [c_int; 3],
    pub mtkaif_phase_cycle: [c_int; 3],
    pub mtkaif_protocol: c_int,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}
#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mtk_soc_card_data {
    pub card_data: *mut mtk_platform_card_data,
}
#[repr(C)]
pub struct mtk_platform_card_data {
    pub card: *mut snd_soc_card,
    pub num_jacks: c_int,
    pub jacks: [snd_soc_jack; MT8192_JACK_MAX],
    pub pcm_constraints: *const mtk_pcm_constraints_data,
    pub num_pcm_constraints: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub list: *const c_uint,
    pub count: c_uint,
}
#[repr(C)]
pub struct mtk_pcm_constraints_data {
    pub channels: *const snd_pcm_hw_constraint_list,
    pub rates: *const snd_pcm_hw_constraint_list,
}
#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}
#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub trigger: [c_int; 2],
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub no_pcm: c_uint,
    pub ignore_suspend: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
    pub ignore: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
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
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}
#[repr(C)]
pub struct mtk_soundcard_pdata {
    pub card_name: *const c_char,
    pub card_data: *mut mtk_platform_card_data,
    pub soc_probe: Option<unsafe extern "C" fn(*mut mtk_soc_card_data, bool) -> c_int>,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}
#[repr(C)]
pub struct dev_pm_ops {
    pub poweroff: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}
#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mtk_soundcard_common_playback_ops: snd_soc_ops;
    static mtk_soundcard_common_capture_ops: snd_soc_ops;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
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
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
    ) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn mt8192_afe_gpio_request(dev: *mut device, enable: bool, dai: c_int, uplink: c_int) -> c_int;
    fn mt6359_mtkaif_calibration_enable(cmpnt: *mut snd_soc_component);
    fn mt6359_mtkaif_calibration_disable(cmpnt: *mut snd_soc_component);
    fn mt6359_set_mtkaif_calibration_phase(
        cmpnt: *mut snd_soc_component,
        phase_1: c_int,
        phase_2: c_int,
        phase_3: c_int,
    );
    fn mt6359_set_mtkaif_protocol(cmpnt: *mut snd_soc_component, protocol: c_int);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut mtk_soc_card_data;
    fn mt8192_dai_i2s_set_share(
        afe: *mut mtk_base_afe,
        master: *const c_char,
        slave: *const c_char,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        typ: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, typ: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(
        cmpnt: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        typ: c_int,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut c_void;
    fn snd_mask_reset_range(mask: *mut c_void, start: c_uint, end: c_uint);
    fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snd_soc_of_get_dai_link_codecs(
        dev: *mut device,
        node: *mut device_node,
        link: *mut snd_soc_dai_link,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn mt8192_afe_gpio_init(dev: *mut device) -> c_int;
    fn snd_soc_poweroff(dev: *mut device) -> c_int;
    fn snd_soc_resume(dev: *mut device) -> c_int;
    fn mtk_soundcard_common_probe(pdev: *mut platform_device) -> c_int;
}

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const SND_JACK_AVOUT: c_int = 0x0040;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const RT1015_PLL_S_BCLK: c_int = 1;
const RT1015_SCLK_S_PLL: c_int = 1;
const RT5682_PLL1: c_int = 1;
const RT5682_PLL1_S_BCLK1: c_int = 1;
const RT5682_SCLK_S_PLL1: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_CLOCK_OUT: c_int = 1;
const AFE_PCM_NAME: *const c_char = b"AFE_PCM\0".as_ptr() as *const c_char;
const MT8192_DAI_ADDA: c_int = 0;
const MT8192_DAI_ADDA_CH34: c_int = 1;
const AFE_AUD_PAD_TOP: c_uint = 0;
const CKSYS_AUD_TOP_CFG: c_uint = 0;
const CKSYS_AUD_TOP_MON: c_uint = 0;
const MT6359_MTKAIF_PROTOCOL_2_CLK_P2: c_int = 2;
const MTKAIF_PROTOCOL_2_CLK_P2: c_int = 2;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_LAST: c_uint = 64;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SND_SOC_DPCM_TRIGGER_PRE: c_int = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0x0003;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0100;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x3000;
const MTK_CONSTRAINT_PLAYBACK: usize = 0;
const MTK_CONSTRAINT_CAPTURE: usize = 1;
const EINVAL: c_int = 22;

/* Headset jack detection DAPM pins */
static mut mt8192_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn mt8192_rt1015_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 128;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let mut ret: c_int;

    let mut i: c_uint = 0;
    while i < (*(*rtd).card).num_links as c_uint {
        let codec_dai = snd_soc_rtd_to_codec(rtd, i as c_int);
        ret = snd_soc_dai_set_pll(
            codec_dai,
            0,
            RT1015_PLL_S_BCLK,
            params_rate(params).wrapping_mul(64),
            params_rate(params).wrapping_mul(256),
        );
        if ret != 0 {
            dev_err((*card).dev, b"failed to set pll\n\0".as_ptr() as *const c_char);
            return ret;
        }

        ret = snd_soc_dai_set_sysclk(
            codec_dai,
            RT1015_SCLK_S_PLL,
            params_rate(params).wrapping_mul(256),
            SND_SOC_CLOCK_IN,
        );
        if ret != 0 {
            dev_err((*card).dev, b"failed to set sysclk\n\0".as_ptr() as *const c_char);
            return ret;
        }
        i += 1;
    }

    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

unsafe extern "C" fn mt8192_rt5682x_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 128;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let bitwidth = snd_pcm_format_width(params_format(params));
    let mut ret: c_int;

    if bitwidth < 0 {
        dev_err(
            (*card).dev,
            b"invalid bit width: %d\n\0".as_ptr() as *const c_char,
            bitwidth,
        );
        return bitwidth;
    }

    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x00, 0x0, 0x2, bitwidth);
    if ret != 0 {
        dev_err((*card).dev, b"failed to set tdm slot\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = snd_soc_dai_set_pll(
        codec_dai,
        RT5682_PLL1,
        RT5682_PLL1_S_BCLK1,
        params_rate(params).wrapping_mul(64),
        params_rate(params).wrapping_mul(512),
    );
    if ret != 0 {
        dev_err((*card).dev, b"failed to set pll\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5682_SCLK_S_PLL1,
        params_rate(params).wrapping_mul(512),
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err((*card).dev, b"failed to set sysclk\n\0".as_ptr() as *const c_char);
        return ret;
    }

    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8192_rt1015_i2s_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8192_rt1015_i2s_hw_params),
};

static mt8192_rt5682x_i2s_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8192_rt5682x_i2s_hw_params),
};

unsafe extern "C" fn mt8192_mt6359_mtkaif_calibration(
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let afe = snd_soc_component_get_drvdata(cmpnt_afe);
    let afe_priv = (*afe).platform_priv;
    let mut phase: c_int;
    let mut monitor: c_uint = 0;
    let mut test_done_1: c_int;
    let mut test_done_2: c_int;
    let mut test_done_3: c_int;
    let mut cycle_1: c_int;
    let mut cycle_2: c_int;
    let mut cycle_3: c_int;
    let mut prev_cycle_1: c_int = 0;
    let mut prev_cycle_2: c_int = 0;
    let mut prev_cycle_3: c_int = 0;
    let chosen_phase_1: c_int;
    let chosen_phase_2: c_int;
    let chosen_phase_3: c_int;
    let mut counter: c_int;
    let mut mtkaif_calib_ok: bool;

    pm_runtime_get_sync((*afe).dev);
    mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA, 1);
    mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA, 0);
    mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA_CH34, 1);
    mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA_CH34, 0);

    mt6359_mtkaif_calibration_enable(cmpnt_codec);

    /* set clock protocol 2 */
    regmap_update_bits((*afe).regmap, AFE_AUD_PAD_TOP, 0xff, 0x38);
    regmap_update_bits((*afe).regmap, AFE_AUD_PAD_TOP, 0xff, 0x39);

    /* set test type to synchronizer pulse */
    regmap_update_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, 0xffff, 0x4);

    mtkaif_calib_ok = true;
    (*afe_priv).mtkaif_calibration_num_phase = 42; /* mt6359: 0 ~ 42 */
    (*afe_priv).mtkaif_chosen_phase[0] = -1;
    (*afe_priv).mtkaif_chosen_phase[1] = -1;
    (*afe_priv).mtkaif_chosen_phase[2] = -1;

    phase = 0;
    while phase <= (*afe_priv).mtkaif_calibration_num_phase && mtkaif_calib_ok {
        mt6359_set_mtkaif_calibration_phase(cmpnt_codec, phase, phase, phase);
        regmap_update_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, 0x1, 0x1);

        test_done_1 = 0;
        test_done_2 = 0;
        test_done_3 = 0;
        cycle_1 = -1;
        cycle_2 = -1;
        cycle_3 = -1;
        counter = 0;
        while test_done_1 == 0 || test_done_2 == 0 || test_done_3 == 0 {
            regmap_read((*afe_priv).topckgen, CKSYS_AUD_TOP_MON, &mut monitor);

            test_done_1 = ((monitor >> 28) & 0x1) as c_int;
            test_done_2 = ((monitor >> 29) & 0x1) as c_int;
            test_done_3 = ((monitor >> 30) & 0x1) as c_int;
            if test_done_1 == 1 {
                cycle_1 = (monitor & 0xf) as c_int;
            }
            if test_done_2 == 1 {
                cycle_2 = ((monitor >> 4) & 0xf) as c_int;
            }
            if test_done_3 == 1 {
                cycle_3 = ((monitor >> 8) & 0xf) as c_int;
            }

            /* handle if never test done */
            counter += 1;
            if counter > 10000 {
                dev_err(
                    (*afe).dev,
                    b"%s(), test fail, cycle_1 %d, cycle_2 %d, cycle_3 %d, monitor 0x%x\n\0"
                        .as_ptr() as *const c_char,
                    b"mt8192_mt6359_mtkaif_calibration\0".as_ptr() as *const c_char,
                    cycle_1,
                    cycle_2,
                    cycle_3,
                    monitor,
                );
                mtkaif_calib_ok = false;
                break;
            }
        }

        if phase == 0 {
            prev_cycle_1 = cycle_1;
            prev_cycle_2 = cycle_2;
            prev_cycle_3 = cycle_3;
        }

        if cycle_1 != prev_cycle_1 && (*afe_priv).mtkaif_chosen_phase[0] < 0 {
            (*afe_priv).mtkaif_chosen_phase[0] = phase - 1;
            (*afe_priv).mtkaif_phase_cycle[0] = prev_cycle_1;
        }
        if cycle_2 != prev_cycle_2 && (*afe_priv).mtkaif_chosen_phase[1] < 0 {
            (*afe_priv).mtkaif_chosen_phase[1] = phase - 1;
            (*afe_priv).mtkaif_phase_cycle[1] = prev_cycle_2;
        }
        if cycle_3 != prev_cycle_3 && (*afe_priv).mtkaif_chosen_phase[2] < 0 {
            (*afe_priv).mtkaif_chosen_phase[2] = phase - 1;
            (*afe_priv).mtkaif_phase_cycle[2] = prev_cycle_3;
        }

        regmap_update_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, 0x1, 0x0);

        if (*afe_priv).mtkaif_chosen_phase[0] >= 0
            && (*afe_priv).mtkaif_chosen_phase[1] >= 0
            && (*afe_priv).mtkaif_chosen_phase[2] >= 0
        {
            break;
        }
        phase += 1;
    }

    chosen_phase_1 = if (*afe_priv).mtkaif_chosen_phase[0] < 0 {
        0
    } else {
        (*afe_priv).mtkaif_chosen_phase[0]
    };
    chosen_phase_2 = if (*afe_priv).mtkaif_chosen_phase[1] < 0 {
        0
    } else {
        (*afe_priv).mtkaif_chosen_phase[1]
    };
    chosen_phase_3 = if (*afe_priv).mtkaif_chosen_phase[2] < 0 {
        0
    } else {
        (*afe_priv).mtkaif_chosen_phase[2]
    };

    mt6359_set_mtkaif_calibration_phase(cmpnt_codec, chosen_phase_1, chosen_phase_2, chosen_phase_3);

    /* disable rx fifo */
    regmap_update_bits((*afe).regmap, AFE_AUD_PAD_TOP, 0xff, 0x38);

    mt6359_mtkaif_calibration_disable(cmpnt_codec);

    mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA, 1);
    mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA, 0);
    mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA_CH34, 1);
    mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA_CH34, 0);
    pm_runtime_put((*afe).dev);

    dev_dbg(
        (*afe).dev,
        b"%s(), mtkaif_chosen_phase[0/1/2]:%d/%d/%d\n\0".as_ptr() as *const c_char,
        b"mt8192_mt6359_mtkaif_calibration\0".as_ptr() as *const c_char,
        (*afe_priv).mtkaif_chosen_phase[0],
        (*afe_priv).mtkaif_chosen_phase[1],
        (*afe_priv).mtkaif_chosen_phase[2],
    );

    0
}

unsafe extern "C" fn mt8192_mt6359_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let afe = snd_soc_component_get_drvdata(cmpnt_afe);
    let afe_priv = (*afe).platform_priv;

    /* set mtkaif protocol */
    mt6359_set_mtkaif_protocol(cmpnt_codec, MT6359_MTKAIF_PROTOCOL_2_CLK_P2);
    (*afe_priv).mtkaif_protocol = MTKAIF_PROTOCOL_2_CLK_P2;

    /* mtkaif calibration */
    mt8192_mt6359_mtkaif_calibration(rtd);

    0
}

unsafe extern "C" fn mt8192_rt5682_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let jack = &mut (*(*soc_card_data).card_data).jacks[MT8192_JACK_HEADSET] as *mut snd_soc_jack;
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe);
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int;

    ret = mt8192_dai_i2s_set_share(
        afe,
        b"I2S8\0".as_ptr() as *const c_char,
        b"I2S9\0".as_ptr() as *const c_char,
    );
    if ret != 0 {
        dev_err((*rtd).dev, b"Failed to set up shared clocks\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        mt8192_jack_pins.as_mut_ptr(),
        mt8192_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"Headset Jack creation failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    snd_soc_component_set_jack(cmpnt_codec, jack, ptr::null_mut())
}

unsafe extern "C" fn mt8192_mt6359_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let jack = &mut (*(*soc_card_data).card_data).jacks[MT8192_JACK_HDMI] as *mut snd_soc_jack;
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let ret = snd_soc_card_jack_new(
        (*rtd).card,
        b"HDMI Jack\0".as_ptr() as *const c_char,
        SND_JACK_AVOUT,
        jack,
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"HDMI Jack creation failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    snd_soc_component_set_jack(cmpnt_codec, jack, ptr::null_mut())
}

unsafe extern "C" fn mt8192_i2s_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    /* fix BE i2s format to S24_LE, clean param mask first */
    snd_mask_reset_range(
        hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT),
        0,
        SNDRV_PCM_FORMAT_LAST,
    );

    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);

    0
}

macro_rules! comp {
    ($name:expr, $dai:expr) => {
        snd_soc_dai_link_component {
            name: $name.as_ptr() as *const c_char,
            dai_name: $dai.as_ptr() as *const c_char,
            of_node: ptr::null_mut(),
        }
    };
}

/* FE and BE DAI link component definitions translated from SND_SOC_DAILINK_DEFS. */
static mut playback1_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL1\0", b"\0")];
static mut playback12_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL12\0", b"\0")];
static mut playback2_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL2\0", b"\0")];
static mut playback3_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL3\0", b"\0")];
static mut playback4_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL4\0", b"\0")];
static mut playback5_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL5\0", b"\0")];
static mut playback6_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL6\0", b"\0")];
static mut playback7_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL7\0", b"\0")];
static mut playback8_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL8\0", b"\0")];
static mut playback9_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"DL9\0", b"\0")];
static mut capture1_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL1\0", b"\0")];
static mut capture2_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL2\0", b"\0")];
static mut capture3_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL3\0", b"\0")];
static mut capture4_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL4\0", b"\0")];
static mut capture5_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL5\0", b"\0")];
static mut capture6_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL6\0", b"\0")];
static mut capture7_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL7\0", b"\0")];
static mut capture8_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL8\0", b"\0")];
static mut capture_mono1_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL_MONO_1\0", b"\0")];
static mut capture_mono2_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL_MONO_2\0", b"\0")];
static mut capture_mono3_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"UL_MONO_3\0", b"\0")];
static mut playback_hdmi_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"HDMI\0", b"\0")];
static mut primary_codec_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"ADDA\0", b"\0")];
static mut primary_codec_codecs: [snd_soc_dai_link_component; 2] = [
    comp!(b"mt6359-sound\0", b"mt6359-snd-codec-aif1\0"),
    comp!(b"dmic-codec\0", b"dmic-hifi\0"),
];
static mut primary_codec_ch34_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"ADDA_CH34\0", b"\0")];
static mut primary_codec_ch34_codecs: [snd_soc_dai_link_component; 1] =
    [comp!(b"mt6359-sound\0", b"mt6359-snd-codec-aif2\0")];
static mut ap_dmic_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"AP_DMIC\0", b"\0")];
static mut ap_dmic_ch34_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"AP_DMIC_CH34\0", b"\0")];
static mut i2s0_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S0\0", b"\0")];
static mut i2s1_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S1\0", b"\0")];
static mut i2s2_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S2\0", b"\0")];
static mut i2s3_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S3\0", b"\0")];
static mut i2s5_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S5\0", b"\0")];
static mut i2s6_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S6\0", b"\0")];
static mut i2s7_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S7\0", b"\0")];
static mut i2s8_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S8\0", b"\0")];
static mut i2s9_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"I2S9\0", b"\0")];
static mut connsys_i2s_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"CONNSYS_I2S\0", b"\0")];
static mut pcm1_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"PCM 1\0", b"\0")];
static mut pcm2_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"PCM 2\0", b"\0")];
static mut tdm_cpus: [snd_soc_dai_link_component; 1] = [comp!(b"TDM\0", b"\0")];
static mut tdm_codecs: [snd_soc_dai_link_component; 1] = [comp!(b"\0", b"i2s-hifi\0")];

macro_rules! dai_link {
    ($name:expr, $stream:expr, $cpus:ident) => {
        snd_soc_dai_link {
            name: $name.as_ptr() as *const c_char,
            stream_name: $stream.as_ptr() as *const c_char,
            trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
            dynamic: 0,
            playback_only: 0,
            capture_only: 0,
            no_pcm: 0,
            ignore_suspend: 0,
            init: None,
            be_hw_params_fixup: None,
            ops: ptr::null(),
            dai_fmt: 0,
            ignore: 0,
            cpus: $cpus.as_mut_ptr(),
            num_cpus: $cpus.len() as c_uint,
            codecs: ptr::null_mut(),
            num_codecs: 0,
            platforms: ptr::null_mut(),
            num_platforms: 0,
        }
    };
}

static mut mt8192_mt6359_dai_links: [snd_soc_dai_link; 40] = unsafe {
    [
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_1\0", b"Playback_1\0", playback1_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_12\0", b"Playback_12\0", playback12_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_2\0", b"Playback_2\0", playback2_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ops: &mtk_soundcard_common_playback_ops, ..dai_link!(b"Playback_3\0", b"Playback_3\0", playback3_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_4\0", b"Playback_4\0", playback4_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_5\0", b"Playback_5\0", playback5_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_6\0", b"Playback_6\0", playback6_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_7\0", b"Playback_7\0", playback7_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_8\0", b"Playback_8\0", playback8_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"Playback_9\0", b"Playback_9\0", playback9_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, ..dai_link!(b"Capture_1\0", b"Capture_1\0", capture1_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_playback_ops, ..dai_link!(b"Capture_2\0", b"Capture_2\0", capture2_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_3\0", b"Capture_3\0", capture3_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_4\0", b"Capture_4\0", capture4_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_5\0", b"Capture_5\0", capture5_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_6\0", b"Capture_6\0", capture6_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_7\0", b"Capture_7\0", capture7_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_8\0", b"Capture_8\0", capture8_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_Mono_1\0", b"Capture_Mono_1\0", capture_mono1_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_Mono_2\0", b"Capture_Mono_2\0", capture_mono2_cpus) },
        snd_soc_dai_link { dynamic: 1, capture_only: 1, ..dai_link!(b"Capture_Mono_3\0", b"Capture_Mono_3\0", capture_mono3_cpus) },
        snd_soc_dai_link { dynamic: 1, playback_only: 1, ..dai_link!(b"playback_hdmi\0", b"Playback_HDMI\0", playback_hdmi_cpus) },
        snd_soc_dai_link { no_pcm: 1, ignore_suspend: 1, init: Some(mt8192_mt6359_init), cpus: primary_codec_cpus.as_mut_ptr(), num_cpus: 1, codecs: primary_codec_codecs.as_mut_ptr(), num_codecs: 2, ..dai_link!(b"Primary Codec\0", b"\0", primary_codec_cpus) },
        snd_soc_dai_link { no_pcm: 1, ignore_suspend: 1, cpus: primary_codec_ch34_cpus.as_mut_ptr(), num_cpus: 1, codecs: primary_codec_ch34_codecs.as_mut_ptr(), num_codecs: 1, ..dai_link!(b"Primary Codec CH34\0", b"\0", primary_codec_ch34_cpus) },
        snd_soc_dai_link { no_pcm: 1, capture_only: 1, ignore_suspend: 1, ..dai_link!(b"AP_DMIC\0", b"\0", ap_dmic_cpus) },
        snd_soc_dai_link { no_pcm: 1, capture_only: 1, ignore_suspend: 1, ..dai_link!(b"AP_DMIC_CH34\0", b"\0", ap_dmic_ch34_cpus) },
        snd_soc_dai_link { no_pcm: 1, capture_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ..dai_link!(b"I2S0\0", b"\0", i2s0_cpus) },
        snd_soc_dai_link { no_pcm: 1, playback_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ..dai_link!(b"I2S1\0", b"\0", i2s1_cpus) },
        snd_soc_dai_link { no_pcm: 1, capture_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ..dai_link!(b"I2S2\0", b"\0", i2s2_cpus) },
        snd_soc_dai_link { no_pcm: 1, playback_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ..dai_link!(b"I2S3\0", b"\0", i2s3_cpus) },
        snd_soc_dai_link { no_pcm: 1, playback_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ..dai_link!(b"I2S5\0", b"\0", i2s5_cpus) },
        snd_soc_dai_link { no_pcm: 1, capture_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ..dai_link!(b"I2S6\0", b"\0", i2s6_cpus) },
        snd_soc_dai_link { no_pcm: 1, playback_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ..dai_link!(b"I2S7\0", b"\0", i2s7_cpus) },
        snd_soc_dai_link { no_pcm: 1, capture_only: 1, ignore_suspend: 1, init: Some(mt8192_rt5682_init), be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ops: &mt8192_rt5682x_i2s_ops, ..dai_link!(b"I2S8\0", b"\0", i2s8_cpus) },
        snd_soc_dai_link { no_pcm: 1, playback_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ops: &mt8192_rt5682x_i2s_ops, ..dai_link!(b"I2S9\0", b"\0", i2s9_cpus) },
        snd_soc_dai_link { no_pcm: 1, capture_only: 1, ignore_suspend: 1, ..dai_link!(b"CONNSYS_I2S\0", b"\0", connsys_i2s_cpus) },
        snd_soc_dai_link { no_pcm: 1, ignore_suspend: 1, ..dai_link!(b"PCM 1\0", b"\0", pcm1_cpus) },
        snd_soc_dai_link { no_pcm: 1, ignore_suspend: 1, ..dai_link!(b"PCM 2\0", b"\0", pcm2_cpus) },
        snd_soc_dai_link { no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_CBP_CFP, playback_only: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8192_i2s_hw_params_fixup), ignore: 1, init: Some(mt8192_mt6359_hdmi_init), cpus: tdm_cpus.as_mut_ptr(), num_cpus: 1, codecs: tdm_codecs.as_mut_ptr(), num_codecs: 1, ..dai_link!(b"TDM\0", b"\0", tdm_cpus) },
        dai_link!(b"\0", b"\0", tdm_cpus),
    ]
};

static mt8192_mt6359_rt1015_rt5682_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { name: b"Left Spk\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Right Spk\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Headphone Jack\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"TDM Out\0".as_ptr() as *const c_char },
];

static mt8192_mt6359_rt1015_rt5682_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Left SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Right SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"TDM Out\0".as_ptr() as *const c_char, control: ptr::null(), source: b"TDM\0".as_ptr() as *const c_char },
];

static mt8192_mt6359_rt1015_rt5682_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { name: b"Left Spk\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Right Spk\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headphone Jack\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() as *const c_char },
];

static mut rt1015_amp_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: comp!(b"rt1015.1-0028\0", b"\0"), name_prefix: b"Left\0".as_ptr() as *const c_char },
    snd_soc_codec_conf { dlc: comp!(b"rt1015.1-0029\0", b"\0"), name_prefix: b"Right\0".as_ptr() as *const c_char },
];

static mut mt8192_mt6359_rt1015_rt5682_card: snd_soc_card = unsafe {
    snd_soc_card {
        name: RT1015_RT5682_CARD_NAME,
        driver_name: DRIVER_NAME,
        owner: THIS_MODULE,
        dev: ptr::null_mut(),
        dai_link: mt8192_mt6359_dai_links.as_mut_ptr(),
        num_links: mt8192_mt6359_dai_links.len() as c_int,
        controls: mt8192_mt6359_rt1015_rt5682_controls.as_ptr(),
        num_controls: mt8192_mt6359_rt1015_rt5682_controls.len() as c_int,
        dapm_widgets: mt8192_mt6359_rt1015_rt5682_widgets.as_ptr(),
        num_dapm_widgets: mt8192_mt6359_rt1015_rt5682_widgets.len() as c_int,
        dapm_routes: mt8192_mt6359_rt1015_rt5682_routes.as_ptr(),
        num_dapm_routes: mt8192_mt6359_rt1015_rt5682_routes.len() as c_int,
        codec_conf: rt1015_amp_conf.as_mut_ptr(),
        num_configs: rt1015_amp_conf.len() as c_int,
    }
};

static mt8192_mt6359_rt1015p_rt5682x_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { name: b"Speakers\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Headphone Jack\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Headset Mic\0".as_ptr() as *const c_char },
];

static mt8192_mt6359_rt1015p_rt5682x_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"Speakers\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Speaker\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
];

static mt8192_mt6359_rt1015p_rt5682x_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { name: b"Speakers\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headphone Jack\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() as *const c_char },
];

static mut mt8192_mt6359_rt1015p_rt5682x_card: snd_soc_card = unsafe {
    snd_soc_card {
        name: ptr::null(),
        driver_name: DRIVER_NAME,
        owner: THIS_MODULE,
        dev: ptr::null_mut(),
        dai_link: mt8192_mt6359_dai_links.as_mut_ptr(),
        num_links: mt8192_mt6359_dai_links.len() as c_int,
        controls: mt8192_mt6359_rt1015p_rt5682x_controls.as_ptr(),
        num_controls: mt8192_mt6359_rt1015p_rt5682x_controls.len() as c_int,
        dapm_widgets: mt8192_mt6359_rt1015p_rt5682x_widgets.as_ptr(),
        num_dapm_widgets: mt8192_mt6359_rt1015p_rt5682x_widgets.len() as c_int,
        dapm_routes: mt8192_mt6359_rt1015p_rt5682x_routes.as_ptr(),
        num_dapm_routes: mt8192_mt6359_rt1015p_rt5682x_routes.len() as c_int,
        codec_conf: ptr::null_mut(),
        num_configs: 0,
    }
};

unsafe extern "C" fn mt8192_mt6359_card_set_be_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    node: *mut device_node,
    link_name: *mut c_char,
) -> c_int {
    let mut ret: c_int;

    if !node.is_null() && strcmp((*link).name, link_name) == 0 {
        ret = snd_soc_of_get_dai_link_codecs(dev, node, link);
        if ret < 0 {
            dev_err_probe(dev, ret, b"get dai link codecs fail\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }

    0
}

unsafe extern "C" fn mt8192_mt6359_legacy_probe(
    soc_card_data: *mut mtk_soc_card_data,
) -> c_int {
    let card_data = (*soc_card_data).card_data;
    let card = (*card_data).card;
    let dev = (*card).dev;
    let mut hdmi_codec: *mut device_node;
    let mut headset_codec: *mut device_node;
    let mut speaker_codec: *mut device_node;
    let mut ret: c_int = 0;

    hdmi_codec = of_parse_phandle(
        (*dev).of_node,
        b"mediatek,hdmi-codec\0".as_ptr() as *const c_char,
        0,
    );
    if hdmi_codec.is_null() {
        dev_dbg(dev, b"The machine has no hdmi-codec\n\0".as_ptr() as *const c_char);
    }

    speaker_codec = of_get_child_by_name(
        (*dev).of_node,
        b"speaker-codecs\0".as_ptr() as *const c_char,
    );
    if speaker_codec.is_null() {
        ret = -EINVAL;
        dev_err_probe(
            dev,
            ret,
            b"Property 'speaker-codecs' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        of_node_put(hdmi_codec);
        return ret;
    }

    headset_codec = of_get_child_by_name(
        (*dev).of_node,
        b"headset-codec\0".as_ptr() as *const c_char,
    );
    if headset_codec.is_null() {
        ret = -EINVAL;
        dev_err_probe(
            dev,
            ret,
            b"Property 'headset-codec' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        of_node_put(speaker_codec);
        of_node_put(hdmi_codec);
        return ret;
    }

    let mut i = 0;
    while i < (*card).num_links {
        let dai_link = (*card).dai_link.add(i as usize);
        ret = mt8192_mt6359_card_set_be_link(
            dev,
            dai_link,
            speaker_codec,
            b"I2S3\0".as_ptr() as *mut c_char,
        );
        if ret != 0 {
            dev_err_probe(
                dev,
                ret,
                b"%s set speaker_codec fail\n\0".as_ptr() as *const c_char,
                (*dai_link).name,
            );
            break;
        }

        ret = mt8192_mt6359_card_set_be_link(
            dev,
            dai_link,
            headset_codec,
            b"I2S8\0".as_ptr() as *mut c_char,
        );
        if ret != 0 {
            dev_err_probe(
                dev,
                ret,
                b"%s set headset_codec fail\n\0".as_ptr() as *const c_char,
                (*dai_link).name,
            );
            break;
        }

        ret = mt8192_mt6359_card_set_be_link(
            dev,
            dai_link,
            headset_codec,
            b"I2S9\0".as_ptr() as *mut c_char,
        );
        if ret != 0 {
            dev_err_probe(
                dev,
                ret,
                b"%s set headset_codec fail\n\0".as_ptr() as *const c_char,
                (*dai_link).name,
            );
            break;
        }

        if !hdmi_codec.is_null()
            && strcmp((*dai_link).name, b"TDM\0".as_ptr() as *const c_char) == 0
        {
            (*(*dai_link).codecs).of_node = hdmi_codec;
            (*dai_link).ignore = 0;
        }

        if (*dai_link).num_codecs != 0
            && strcmp((*(*dai_link).codecs.add(0)).dai_name, RT1015_CODEC_DAI) == 0
        {
            (*dai_link).ops = &mt8192_rt1015_i2s_ops;
        }

        i += 1;
    }

    of_node_put(headset_codec);
    of_node_put(speaker_codec);
    of_node_put(hdmi_codec);
    ret
}

unsafe extern "C" fn mt8192_mt6359_soc_card_probe(
    soc_card_data: *mut mtk_soc_card_data,
    legacy: bool,
) -> c_int {
    let card_data = (*soc_card_data).card_data;
    let card = (*card_data).card;
    let mut ret: c_int;

    if legacy {
        ret = mt8192_mt6359_legacy_probe(soc_card_data);
        if ret != 0 {
            return ret;
        }
    } else {
        let mut i = 0;
        while i < (*card).num_links {
            let dai_link = (*card).dai_link.add(i as usize);
            if (*dai_link).num_codecs != 0
                && strcmp((*(*dai_link).codecs.add(0)).dai_name, RT1015_CODEC_DAI) == 0
            {
                (*dai_link).ops = &mt8192_rt1015_i2s_ops;
            }
            i += 1;
        }
    }

    ret = mt8192_afe_gpio_init((*card).dev);
    if ret != 0 {
        return dev_err_probe(
            (*card).dev,
            ret,
            b"%s init gpio error\n\0".as_ptr() as *const c_char,
            b"mt8192_mt6359_soc_card_probe\0".as_ptr() as *const c_char,
        );
    }

    0
}

static mt8192_pcm_playback_channels: [c_uint; 2] = [1, 2];
static mt8192_pcm_playback_rates: [c_uint; 1] = [48000];

static mt8192_pcm_capture_channels: [c_uint; 3] = [1, 2, 4];
static mt8192_pcm_capture_rates: [c_uint; 6] = [8000, 16000, 32000, 48000, 96000, 192000];

static mt8192_pcm_playback_channels_constraint: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        list: mt8192_pcm_playback_channels.as_ptr(),
        count: mt8192_pcm_playback_channels.len() as c_uint,
    };
static mt8192_pcm_playback_rates_constraint: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        list: mt8192_pcm_playback_rates.as_ptr(),
        count: mt8192_pcm_playback_rates.len() as c_uint,
    };
static mt8192_pcm_capture_channels_constraint: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        list: mt8192_pcm_capture_channels.as_ptr(),
        count: mt8192_pcm_capture_channels.len() as c_uint,
    };
static mt8192_pcm_capture_rates_constraint: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        list: mt8192_pcm_capture_rates.as_ptr(),
        count: mt8192_pcm_capture_rates.len() as c_uint,
    };

static mt8192_pcm_constraints: [mtk_pcm_constraints_data; MTK_CONSTRAINT_CAPTURE + 1] = [
    mtk_pcm_constraints_data {
        channels: &mt8192_pcm_playback_channels_constraint,
        rates: &mt8192_pcm_playback_rates_constraint,
    },
    mtk_pcm_constraints_data {
        channels: &mt8192_pcm_capture_channels_constraint,
        rates: &mt8192_pcm_capture_rates_constraint,
    },
];

static mut mt8192_mt6359_rt1015_rt5682_card_data: mtk_platform_card_data =
    mtk_platform_card_data {
        card: unsafe { &mut mt8192_mt6359_rt1015_rt5682_card },
        num_jacks: MT8192_JACK_MAX as c_int,
        jacks: [snd_soc_jack { jack: ptr::null_mut() }, snd_soc_jack { jack: ptr::null_mut() }],
        pcm_constraints: mt8192_pcm_constraints.as_ptr(),
        num_pcm_constraints: mt8192_pcm_constraints.len() as c_int,
    };

static mt8192_mt6359_rt1015_rt5682_pdata: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: RT1015_RT5682_CARD_NAME,
    card_data: unsafe { &mut mt8192_mt6359_rt1015_rt5682_card_data },
    soc_probe: Some(mt8192_mt6359_soc_card_probe),
};

static mut mt8192_mt6359_rt1015p_rt5682_card_data: mtk_platform_card_data =
    mtk_platform_card_data {
        card: unsafe { &mut mt8192_mt6359_rt1015p_rt5682x_card },
        num_jacks: MT8192_JACK_MAX as c_int,
        jacks: [snd_soc_jack { jack: ptr::null_mut() }, snd_soc_jack { jack: ptr::null_mut() }],
        pcm_constraints: mt8192_pcm_constraints.as_ptr(),
        num_pcm_constraints: mt8192_pcm_constraints.len() as c_int,
    };

static mt8192_mt6359_rt1015p_rt5682_pdata: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: RT1015P_RT5682_CARD_NAME,
    card_data: unsafe { &mut mt8192_mt6359_rt1015p_rt5682_card_data },
    soc_probe: Some(mt8192_mt6359_soc_card_probe),
};

static mut mt8192_mt6359_rt1015p_rt5682s_card_data: mtk_platform_card_data =
    mtk_platform_card_data {
        card: unsafe { &mut mt8192_mt6359_rt1015p_rt5682x_card },
        num_jacks: MT8192_JACK_MAX as c_int,
        jacks: [snd_soc_jack { jack: ptr::null_mut() }, snd_soc_jack { jack: ptr::null_mut() }],
        pcm_constraints: mt8192_pcm_constraints.as_ptr(),
        num_pcm_constraints: mt8192_pcm_constraints.len() as c_int,
    };

static mt8192_mt6359_rt1015p_rt5682s_pdata: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: RT1015P_RT5682S_CARD_NAME,
    card_data: unsafe { &mut mt8192_mt6359_rt1015p_rt5682s_card_data },
    soc_probe: Some(mt8192_mt6359_soc_card_probe),
};

/* #ifdef CONFIG_OF */
static mt8192_mt6359_dt_match: [of_device_id; 4] = [
    of_device_id {
        compatible: RT1015_RT5682_OF_NAME,
        data: &mt8192_mt6359_rt1015_rt5682_pdata as *const _ as *const c_void,
    },
    of_device_id {
        compatible: RT1015P_RT5682_OF_NAME,
        data: &mt8192_mt6359_rt1015p_rt5682_pdata as *const _ as *const c_void,
    },
    of_device_id {
        compatible: RT1015P_RT5682S_OF_NAME,
        data: &mt8192_mt6359_rt1015p_rt5682s_pdata as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, mt8192_mt6359_dt_match); */
/* #endif */

static mt8192_mt6359_pm_ops: dev_pm_ops = dev_pm_ops {
    poweroff: Some(snd_soc_poweroff),
    restore: Some(snd_soc_resume),
};

static mt8192_mt6359_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: DRIVER_NAME,
        /* #ifdef CONFIG_OF */
        of_match_table: mt8192_mt6359_dt_match.as_ptr(),
        /* #endif */
        pm: &mt8192_mt6359_pm_ops,
    },
    probe: Some(mtk_soundcard_common_probe),
};

/* module_platform_driver(mt8192_mt6359_driver); */

/* Module information */
/* MODULE_DESCRIPTION("MT8192-MT6359 ALSA SoC machine driver"); */
/* MODULE_AUTHOR("Jiaxin Yu <jiaxin.yu@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("mt8192_mt6359 soc card"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
