// SPDX-License-Identifier: GPL-2.0
/*
 * mt2701-cs42448.rs  --  MT2701 CS42448 ALSA SoC machine driver
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Ir Lian <ir.lian@mediatek.com>
 *	   Garlic Tseng <garlic.tseng@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub trigger: [c_int; 2],
    pub ops: *const snd_soc_ops,
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub no_pcm: c_uint,
    pub dai_fmt: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub num_codecs: c_uint,
    pub num_cpus: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    #[cfg(CONFIG_OF)]
    pub of_match_table: *const of_device_id,
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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
struct mt2701_cs42448_private {
    i2s1_in_mux: c_int,
    i2s1_in_mux_sel_1: *mut gpio_desc,
    i2s1_in_mux_sel_2: *mut gpio_desc,
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 1;
const SND_SOC_DPCM_TRIGGER_POST: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_GATED: c_uint = 0;

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

static i2sin_mux_switch_text: [*const c_char; 4] = [
    b"ADC_SDOUT2\0".as_ptr() as *const c_char,
    b"ADC_SDOUT3\0".as_ptr() as *const c_char,
    b"I2S_IN_1\0".as_ptr() as *const c_char,
    b"I2S_IN_2\0".as_ptr() as *const c_char,
];

/* SOC_ENUM_SINGLE_EXT(4, i2sin_mux_switch_text) */
static i2sin_mux_enum: soc_enum = soc_enum { _private: [] };

unsafe extern "C" fn mt2701_cs42448_i2sin1_mux_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol);
    let priv_: *mut mt2701_cs42448_private =
        snd_soc_card_get_drvdata(card) as *mut mt2701_cs42448_private;

    (*ucontrol).value.integer.value[0] = (*priv_).i2s1_in_mux as i64;
    0
}

unsafe extern "C" fn mt2701_cs42448_i2sin1_mux_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol);
    let priv_: *mut mt2701_cs42448_private =
        snd_soc_card_get_drvdata(card) as *mut mt2701_cs42448_private;

    if (*ucontrol).value.integer.value[0] == (*priv_).i2s1_in_mux as i64 {
        return 0;
    }

    match (*ucontrol).value.integer.value[0] {
        0 => {
            gpiod_set_value((*priv_).i2s1_in_mux_sel_1, 0);
            gpiod_set_value((*priv_).i2s1_in_mux_sel_2, 0);
        }
        1 => {
            gpiod_set_value((*priv_).i2s1_in_mux_sel_1, 1);
            gpiod_set_value((*priv_).i2s1_in_mux_sel_2, 0);
        }
        2 => {
            gpiod_set_value((*priv_).i2s1_in_mux_sel_1, 0);
            gpiod_set_value((*priv_).i2s1_in_mux_sel_2, 1);
        }
        3 => {
            gpiod_set_value((*priv_).i2s1_in_mux_sel_1, 1);
            gpiod_set_value((*priv_).i2s1_in_mux_sel_2, 1);
        }
        _ => {
            dev_warn(
                (*card).dev,
                b"%s invalid setting\n\0".as_ptr() as *const c_char,
                b"mt2701_cs42448_i2sin1_mux_set\0".as_ptr() as *const c_char,
            );
        }
    }

    (*priv_).i2s1_in_mux = (*ucontrol).value.integer.value[0] as c_int;
    0
}

/* SND_SOC_DAPM_LINE/MIC initializers */
static mt2701_cs42448_asoc_card_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

/* SOC_DAPM_PIN_SWITCH and SOC_ENUM_EXT initializers */
static mt2701_cs42448_controls: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static mt2701_cs42448_sampling_rates: [c_uint; 1] = [48000];

static mt2701_cs42448_constraints_rates: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        count: mt2701_cs42448_sampling_rates.len() as c_uint,
        list: mt2701_cs42448_sampling_rates.as_ptr(),
        mask: 0,
    };

unsafe extern "C" fn mt2701_cs42448_fe_ops_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
    let err: c_int;

    err = snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mt2701_cs42448_constraints_rates,
    );
    if err < 0 {
        dev_err(
            (*(*(*substream).pcm).card).dev,
            b"%s snd_pcm_hw_constraint_list failed: 0x%x\n\0".as_ptr() as *const c_char,
            b"mt2701_cs42448_fe_ops_startup\0".as_ptr() as *const c_char,
            err,
        );
        return err;
    }
    0
}

static mt2701_cs42448_48k_fe_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt2701_cs42448_fe_ops_startup),
    hw_params: None,
};

unsafe extern "C" fn mt2701_cs42448_be_ops_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mclk_rate: c_uint;
    let rate: c_uint = params_rate(params);
    let div_mclk_over_bck: c_uint = if rate > 192000 { 2 } else { 4 };
    let div_bck_over_lrck: c_uint = 64;

    mclk_rate = rate
        .wrapping_mul(div_bck_over_lrck)
        .wrapping_mul(div_mclk_over_bck);

    /* mt2701 mclk */
    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_rate, SND_SOC_CLOCK_OUT);

    /* codec mclk */
    snd_soc_dai_set_sysclk(codec_dai, 0, mclk_rate, SND_SOC_CLOCK_IN);

    0
}

static mt2701_cs42448_be_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(mt2701_cs42448_be_ops_hw_params),
};

const DAI_LINK_FE_MULTI_CH_OUT: usize = 0;
const DAI_LINK_FE_PCM0_IN: usize = 1;
const DAI_LINK_FE_PCM1_IN: usize = 2;
const DAI_LINK_FE_BT_OUT: usize = 3;
const DAI_LINK_FE_BT_IN: usize = 4;
const DAI_LINK_BE_I2S0: usize = 5;
const DAI_LINK_BE_I2S1: usize = 6;
const DAI_LINK_BE_I2S2: usize = 7;
const DAI_LINK_BE_I2S3: usize = 8;
const DAI_LINK_BE_MRG_BT: usize = 9;

static mut fe_multi_ch_out_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: b"PCM_multi\0".as_ptr() as *const c_char,
        dai_name: ptr::null(),
        of_node: ptr::null_mut(),
    }];
static mut fe_multi_ch_out_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: ptr::null(),
        dai_name: ptr::null(),
        of_node: ptr::null_mut(),
    }];
static mut fe_multi_ch_out_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: ptr::null(),
        dai_name: ptr::null(),
        of_node: ptr::null_mut(),
    }];

static mut fe_pcm0_in_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"PCM0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_pcm0_in_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_pcm0_in_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut fe_pcm1_in_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"PCM1\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_pcm1_in_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_pcm1_in_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut fe_bt_out_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"PCM_BT_DL\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_bt_out_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_bt_out_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut fe_bt_in_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"PCM_BT_UL\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_bt_in_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut fe_bt_in_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut be_i2s0_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"I2S0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut be_i2s0_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"cs42448\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut be_i2s0_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut be_i2s1_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"I2S1\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut be_i2s1_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"cs42448\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut be_i2s1_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut be_i2s2_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"I2S2\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut be_i2s2_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"cs42448\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut be_i2s2_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut be_i2s3_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"I2S3\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut be_i2s3_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"cs42448\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut be_i2s3_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut be_mrg_bt_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"MRG BT\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut be_mrg_bt_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"bt-sco-pcm-wb\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut be_mrg_bt_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut mt2701_cs42448_dai_links: [snd_soc_dai_link; 10] = unsafe {
    [
        /* FE */
        snd_soc_dai_link {
            name: b"mt2701-cs42448-multi-ch-out\0".as_ptr() as *const c_char,
            stream_name: b"mt2701-cs42448-multi-ch-out\0".as_ptr() as *const c_char,
            trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
            ops: &mt2701_cs42448_48k_fe_ops,
            dynamic: 1,
            playback_only: 1,
            capture_only: 0,
            no_pcm: 0,
            dai_fmt: 0,
            platforms: fe_multi_ch_out_platforms.as_mut_ptr(),
            codecs: fe_multi_ch_out_codecs.as_mut_ptr(),
            cpus: fe_multi_ch_out_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-pcm0\0".as_ptr() as *const c_char,
            stream_name: b"mt2701-cs42448-pcm0-data-UL\0".as_ptr() as *const c_char,
            trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
            ops: &mt2701_cs42448_48k_fe_ops,
            dynamic: 1,
            playback_only: 0,
            capture_only: 1,
            no_pcm: 0,
            dai_fmt: 0,
            platforms: fe_pcm0_in_platforms.as_mut_ptr(),
            codecs: fe_pcm0_in_codecs.as_mut_ptr(),
            cpus: fe_pcm0_in_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-pcm1-data-UL\0".as_ptr() as *const c_char,
            stream_name: b"mt2701-cs42448-pcm1-data-UL\0".as_ptr() as *const c_char,
            trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
            ops: &mt2701_cs42448_48k_fe_ops,
            dynamic: 1,
            playback_only: 0,
            capture_only: 1,
            no_pcm: 0,
            dai_fmt: 0,
            platforms: fe_pcm1_in_platforms.as_mut_ptr(),
            codecs: fe_pcm1_in_codecs.as_mut_ptr(),
            cpus: fe_pcm1_in_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-pcm-BT-out\0".as_ptr() as *const c_char,
            stream_name: b"mt2701-cs42448-pcm-BT\0".as_ptr() as *const c_char,
            trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
            ops: ptr::null(),
            dynamic: 1,
            playback_only: 1,
            capture_only: 0,
            no_pcm: 0,
            dai_fmt: 0,
            platforms: fe_bt_out_platforms.as_mut_ptr(),
            codecs: fe_bt_out_codecs.as_mut_ptr(),
            cpus: fe_bt_out_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-pcm-BT-in\0".as_ptr() as *const c_char,
            stream_name: b"mt2701-cs42448-pcm-BT\0".as_ptr() as *const c_char,
            trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
            ops: ptr::null(),
            dynamic: 1,
            playback_only: 0,
            capture_only: 1,
            no_pcm: 0,
            dai_fmt: 0,
            platforms: fe_bt_in_platforms.as_mut_ptr(),
            codecs: fe_bt_in_codecs.as_mut_ptr(),
            cpus: fe_bt_in_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        /* BE */
        snd_soc_dai_link {
            name: b"mt2701-cs42448-I2S0\0".as_ptr() as *const c_char,
            stream_name: ptr::null(),
            trigger: [0, 0],
            ops: &mt2701_cs42448_be_ops,
            dynamic: 0,
            playback_only: 0,
            capture_only: 0,
            no_pcm: 1,
            dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED,
            platforms: be_i2s0_platforms.as_mut_ptr(),
            codecs: be_i2s0_codecs.as_mut_ptr(),
            cpus: be_i2s0_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-I2S1\0".as_ptr() as *const c_char,
            stream_name: ptr::null(),
            trigger: [0, 0],
            ops: &mt2701_cs42448_be_ops,
            dynamic: 0,
            playback_only: 0,
            capture_only: 0,
            no_pcm: 1,
            dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED,
            platforms: be_i2s1_platforms.as_mut_ptr(),
            codecs: be_i2s1_codecs.as_mut_ptr(),
            cpus: be_i2s1_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-I2S2\0".as_ptr() as *const c_char,
            stream_name: ptr::null(),
            trigger: [0, 0],
            ops: &mt2701_cs42448_be_ops,
            dynamic: 0,
            playback_only: 0,
            capture_only: 0,
            no_pcm: 1,
            dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED,
            platforms: be_i2s2_platforms.as_mut_ptr(),
            codecs: be_i2s2_codecs.as_mut_ptr(),
            cpus: be_i2s2_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-I2S3\0".as_ptr() as *const c_char,
            stream_name: ptr::null(),
            trigger: [0, 0],
            ops: &mt2701_cs42448_be_ops,
            dynamic: 0,
            playback_only: 0,
            capture_only: 0,
            no_pcm: 1,
            dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED,
            platforms: be_i2s3_platforms.as_mut_ptr(),
            codecs: be_i2s3_codecs.as_mut_ptr(),
            cpus: be_i2s3_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
        snd_soc_dai_link {
            name: b"mt2701-cs42448-MRG-BT\0".as_ptr() as *const c_char,
            stream_name: ptr::null(),
            trigger: [0, 0],
            ops: ptr::null(),
            dynamic: 0,
            playback_only: 0,
            capture_only: 0,
            no_pcm: 1,
            dai_fmt: 0,
            platforms: be_mrg_bt_platforms.as_mut_ptr(),
            codecs: be_mrg_bt_codecs.as_mut_ptr(),
            cpus: be_mrg_bt_cpus.as_mut_ptr(),
            num_platforms: 1,
            num_codecs: 1,
            num_cpus: 1,
        },
    ]
};

static mut mt2701_cs42448_soc_card: snd_soc_card = unsafe {
    snd_soc_card {
        name: b"mt2701-cs42448\0".as_ptr() as *const c_char,
        owner: THIS_MODULE,
        dai_link: mt2701_cs42448_dai_links.as_mut_ptr(),
        num_links: mt2701_cs42448_dai_links.len() as c_int,
        controls: mt2701_cs42448_controls.as_ptr(),
        num_controls: mt2701_cs42448_controls.len() as c_int,
        dapm_widgets: mt2701_cs42448_asoc_card_dapm_widgets.as_ptr(),
        num_dapm_widgets: mt2701_cs42448_asoc_card_dapm_widgets.len() as c_int,
        dev: ptr::null_mut(),
    }
};

unsafe extern "C" fn mt2701_cs42448_machine_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut mt2701_cs42448_soc_card;
    let mut ret: c_int;
    let mut i: usize;
    let platform_node: *mut device_node;
    let codec_node: *mut device_node;
    let codec_node_bt_mrg: *mut device_node;
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut mt2701_cs42448_private = devm_kzalloc(
        dev,
        core::mem::size_of::<mt2701_cs42448_private>(),
        GFP_KERNEL,
    ) as *mut mt2701_cs42448_private;
    let mut dai_link: *mut snd_soc_dai_link;

    if priv_.is_null() {
        return -ENOMEM;
    }

    platform_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"mediatek,platform\0".as_ptr() as *const c_char,
        0,
    );
    if platform_node.is_null() {
        dev_err(
            dev,
            b"Property 'platform' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }
    i = 0;
    while i < (*card).num_links as usize {
        dai_link = (*card).dai_link.add(i);
        if !(*(*dai_link).platforms).name.is_null() {
            i += 1;
            continue;
        }
        (*(*dai_link).platforms).of_node = platform_node;
        i += 1;
    }

    (*card).dev = dev;

    codec_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"mediatek,audio-codec\0".as_ptr() as *const c_char,
        0,
    );
    if codec_node.is_null() {
        dev_err(
            dev,
            b"Property 'audio-codec' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }
    i = 0;
    while i < (*card).num_links as usize {
        dai_link = (*card).dai_link.add(i);
        if !(*(*dai_link).codecs).name.is_null() {
            i += 1;
            continue;
        }
        (*(*dai_link).codecs).of_node = codec_node;
        i += 1;
    }

    codec_node_bt_mrg = of_parse_phandle(
        (*pdev).dev.of_node,
        b"mediatek,audio-codec-bt-mrg\0".as_ptr() as *const c_char,
        0,
    );
    if codec_node_bt_mrg.is_null() {
        dev_err(
            dev,
            b"Property 'audio-codec-bt-mrg' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }
    (*(*mt2701_cs42448_dai_links[DAI_LINK_BE_MRG_BT].codecs)).of_node = codec_node_bt_mrg;

    ret = snd_soc_of_parse_audio_routing(card, b"audio-routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to parse audio-routing: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    (*priv_).i2s1_in_mux_sel_1 = devm_gpiod_get_optional(
        dev,
        b"i2s1-in-sel-gpio1\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*priv_).i2s1_in_mux_sel_1 as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).i2s1_in_mux_sel_1 as *const c_void),
            b"error getting mux 1 selector\n\0".as_ptr() as *const c_char,
        );
    }

    (*priv_).i2s1_in_mux_sel_2 = devm_gpiod_get_optional(
        dev,
        b"i2s1-in-sel-gpio2\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*priv_).i2s1_in_mux_sel_2 as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).i2s1_in_mux_sel_2 as *const c_void),
            b"error getting mux 2 selector\n\0".as_ptr() as *const c_char,
        );
    }

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    ret = devm_snd_soc_register_card(dev, card);

    if ret != 0 {
        dev_err(
            dev,
            b"%s snd_soc_register_card fail %d\n\0".as_ptr() as *const c_char,
            b"mt2701_cs42448_machine_probe\0".as_ptr() as *const c_char,
            ret,
        );
    }
    ret
}

/* Original C condition: #ifdef CONFIG_OF */
#[cfg(CONFIG_OF)]
static mt2701_cs42448_machine_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt2701-cs42448-machine\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, mt2701_cs42448_machine_dt_match); */

static mut mt2701_cs42448_machine: platform_driver = platform_driver {
    driver: device_driver {
        name: b"mt2701-cs42448\0".as_ptr() as *const c_char,
        #[cfg(CONFIG_OF)]
        of_match_table: mt2701_cs42448_machine_dt_match.as_ptr(),
    },
    probe: Some(mt2701_cs42448_machine_probe),
};

/* module_platform_driver(mt2701_cs42448_machine); */

/* Module information */
/* MODULE_DESCRIPTION("MT2701 CS42448 ALSA SoC machine driver"); */
/* MODULE_AUTHOR("Ir Lian <ir.lian@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("mt2701 cs42448 soc card"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
