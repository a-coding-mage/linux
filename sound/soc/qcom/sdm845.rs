// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

// Translated from soc/qcom/sdm845.c. Includes are represented by the external
// declarations and constants expected from the surrounding kernel/ASoC tree.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const DRIVER_NAME: &[u8] = b"sdm845\0";
const DEFAULT_SAMPLE_RATE_48K: c_uint = 48000;
const DEFAULT_MCLK_RATE: c_uint = 24576000;
const TDM_BCLK_RATE: c_uint = 6144000;
const MI2S_BCLK_RATE: c_uint = 1536000;
const LEFT_SPK_TDM_TX_MASK: c_uint = 0x30;
const RIGHT_SPK_TDM_TX_MASK: c_uint = 0xC0;
const SPK_TDM_RX_MASK: c_uint = 0x03;
const NUM_TDM_SLOTS: c_int = 8;
const SLIM_MAX_TX_PORTS: usize = 16;
const SLIM_MAX_RX_PORTS: usize = 13;
const WCD934X_DEFAULT_MCLK_RATE: c_uint = 9600000;

const ENOTSUPP: c_int = 524;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const AFE_PORT_MAX: usize = 256;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 2;

const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;

const PRIMARY_MI2S_RX: c_int = 16;
const PRIMARY_MI2S_TX: c_int = 17;
const SECONDARY_MI2S_RX: c_int = 20;
const SECONDARY_MI2S_TX: c_int = 21;
const QUATERNARY_MI2S_RX: c_int = 26;
const QUATERNARY_TDM_RX_0: c_int = 34;
const QUATERNARY_TDM_TX_0: c_int = 35;
const SLIMBUS_0_RX: c_int = 160;
const SLIMBUS_6_TX: c_int = 173;

const Q6AFE_LPASS_CLK_ID_MCLK_1: c_int = 0;
const Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT: c_int = 0;
const Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT: c_int = 0;
const Q6AFE_LPASS_CLK_ID_QUAD_MI2S_IBIT: c_int = 0;
const Q6AFE_LPASS_CLK_ID_QUAD_TDM_IBIT: c_int = 0;

const RT5663_DA_STEREO_FILTER: c_uint = 0;
const RT5663_AD_STEREO_FILTER: c_uint = 0;
const RT5663_CLK_SEL_I2S1_ASRC: c_uint = 0;
const RT5663_SCLK_S_MCLK: c_int = 0;

#[repr(C)]
pub struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    dev: *mut device,
    card: *mut snd_soc_card,
    dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai {
    id: c_int,
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    driver_name: *const c_char,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    dev: *mut device,
    owner: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    no_pcm: c_int,
    ops: *const snd_soc_ops,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
}

#[repr(C)]
pub struct snd_jack {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_jack)>,
}

#[repr(C)]
pub struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    name: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    name: *const c_char,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: platform_driver_driver,
}

#[repr(C)]
pub struct sdm845_snd_data {
    jack: snd_soc_jack,
    jack_setup: bool,
    slim_port_setup: bool,
    stream_prepared: [bool; AFE_PORT_MAX],
    pri_mi2s_clk_count: u32,
    sec_mi2s_clk_count: u32,
    quat_tdm_clk_count: u32,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_dai_get_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: *mut u32,
        tx_slot: *mut u32,
        rx_num: *mut u32,
        rx_slot: *mut u32,
    ) -> c_int;
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: u32,
        tx_slot: *const c_uint,
        rx_num: u32,
        rx_slot: *const c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_set_format(mask: *mut snd_mask, val: c_int);
    fn qcom_snd_sdw_startup(substream: *mut snd_pcm_substream) -> c_int;
    fn qcom_snd_sdw_shutdown(substream: *mut snd_pcm_substream);
    fn qcom_snd_sdw_prepare(substream: *mut snd_pcm_substream, prepared: *mut bool) -> c_int;
    fn qcom_snd_sdw_hw_free(substream: *mut snd_pcm_substream, prepared: *mut bool) -> c_int;
    fn qcom_snd_parse_of(card: *mut snd_soc_card) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn rt5663_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: c_uint, clk_src: c_uint);
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

static mut SDM845_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

static mut TDM_SLOT_OFFSET: [c_uint; 8] = [0, 4, 8, 12, 16, 20, 24, 28];

unsafe extern "C" fn sdm845_slim_snd_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut snd_soc_dai;
    let mut rx_ch: [u32; SLIM_MAX_RX_PORTS] = [0; SLIM_MAX_RX_PORTS];
    let mut tx_ch: [u32; SLIM_MAX_TX_PORTS] = [0; SLIM_MAX_TX_PORTS];
    let mut rx_ch_cnt: u32 = 0;
    let mut tx_ch_cnt: u32 = 0;
    let mut ret: c_int = 0;

    // for_each_rtd_codec_dais(rtd, i, codec_dai)
    let mut i: c_int = 0;
    while {
        codec_dai = snd_soc_rtd_to_codec(rtd, i as c_uint);
        !codec_dai.is_null()
    } {
        ret = snd_soc_dai_get_channel_map(
            codec_dai,
            &mut tx_ch_cnt,
            tx_ch.as_mut_ptr(),
            &mut rx_ch_cnt,
            rx_ch.as_mut_ptr(),
        );

        if ret != 0 && ret != -ENOTSUPP {
            pr_err(b"failed to get codec chan map, err:%d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        } else if ret == -ENOTSUPP {
            i += 1;
            continue;
        }

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            ret = snd_soc_dai_set_channel_map(cpu_dai, 0, ptr::null(), rx_ch_cnt, rx_ch.as_ptr());
        } else {
            ret = snd_soc_dai_set_channel_map(cpu_dai, tx_ch_cnt, tx_ch.as_ptr(), 0, ptr::null());
        }
        if ret != 0 && ret != -ENOTSUPP {
            dev_err((*rtd).dev, b"failed to set cpu chan map, err:%d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn sdm845_tdm_snd_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut snd_soc_dai;
    let mut ret: c_int = 0;
    let channels: c_int;
    let slot_width: c_int;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => slot_width = 16,
        _ => {
            dev_err(
                (*rtd).dev,
                b"%s: invalid param format 0x%x\n\0".as_ptr() as *const c_char,
                b"sdm845_tdm_snd_hw_params\0".as_ptr() as *const c_char,
                params_format(params),
            );
            return -EINVAL;
        }
    }

    channels = params_channels(params);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0, 0x3, 8, slot_width);
        if ret < 0 {
            dev_err(
                (*rtd).dev,
                b"%s: failed to set tdm slot, err:%d\n\0".as_ptr() as *const c_char,
                b"sdm845_tdm_snd_hw_params\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        ret = snd_soc_dai_set_channel_map(
            cpu_dai,
            0,
            ptr::null(),
            channels as u32,
            TDM_SLOT_OFFSET.as_ptr(),
        );
        if ret < 0 {
            dev_err(
                (*rtd).dev,
                b"%s: failed to set channel map, err:%d\n\0".as_ptr() as *const c_char,
                b"sdm845_tdm_snd_hw_params\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    } else {
        ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0xf, 0, 8, slot_width);
        if ret < 0 {
            dev_err(
                (*rtd).dev,
                b"%s: failed to set tdm slot, err:%d\n\0".as_ptr() as *const c_char,
                b"sdm845_tdm_snd_hw_params\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        ret = snd_soc_dai_set_channel_map(
            cpu_dai,
            channels as u32,
            TDM_SLOT_OFFSET.as_ptr(),
            0,
            ptr::null(),
        );
        if ret < 0 {
            dev_err(
                (*rtd).dev,
                b"%s: failed to set channel map, err:%d\n\0".as_ptr() as *const c_char,
                b"sdm845_tdm_snd_hw_params\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    }

    // for_each_rtd_codec_dais(rtd, j, codec_dai)
    let mut j: c_int = 0;
    while {
        codec_dai = snd_soc_rtd_to_codec(rtd, j as c_uint);
        !codec_dai.is_null()
    } {
        if strcmp((*(*codec_dai).component).name_prefix, b"Left\0".as_ptr() as *const c_char) == 0 {
            ret = snd_soc_dai_set_tdm_slot(
                codec_dai,
                LEFT_SPK_TDM_TX_MASK,
                SPK_TDM_RX_MASK,
                NUM_TDM_SLOTS,
                slot_width,
            );
            if ret < 0 {
                dev_err((*rtd).dev, b"DEV0 TDM slot err:%d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }

        if strcmp((*(*codec_dai).component).name_prefix, b"Right\0".as_ptr() as *const c_char) == 0 {
            ret = snd_soc_dai_set_tdm_slot(
                codec_dai,
                RIGHT_SPK_TDM_TX_MASK,
                SPK_TDM_RX_MASK,
                NUM_TDM_SLOTS,
                slot_width,
            );
            if ret < 0 {
                dev_err((*rtd).dev, b"DEV1 TDM slot err:%d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
        j += 1;
    }

    ret
}

unsafe extern "C" fn sdm845_snd_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int = 0;

    match (*cpu_dai).id {
        PRIMARY_MI2S_RX | PRIMARY_MI2S_TX => {
            /*
             * Use ASRC for internal clocks, as PLL rate isn't multiple
             * of BCLK.
             */
            rt5663_sel_asrc_clk_src(
                (*codec_dai).component,
                RT5663_DA_STEREO_FILTER | RT5663_AD_STEREO_FILTER,
                RT5663_CLK_SEL_I2S1_ASRC,
            );
            ret = snd_soc_dai_set_sysclk(codec_dai, RT5663_SCLK_S_MCLK, DEFAULT_MCLK_RATE, SND_SOC_CLOCK_IN);
            if ret < 0 {
                dev_err((*rtd).dev, b"snd_soc_dai_set_sysclk err = %d\n\0".as_ptr() as *const c_char, ret);
            }
        }
        QUATERNARY_TDM_RX_0 | QUATERNARY_TDM_TX_0 => {
            ret = sdm845_tdm_snd_hw_params(substream, params);
        }
        id if id >= SLIMBUS_0_RX && id <= SLIMBUS_6_TX => {
            ret = sdm845_slim_snd_hw_params(substream, params);
        }
        QUATERNARY_MI2S_RX | SECONDARY_MI2S_RX => {}
        _ => {
            pr_err(
                b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const c_char,
                b"sdm845_snd_hw_params\0".as_ptr() as *const c_char,
                (*cpu_dai).id,
            );
        }
    }
    ret
}

unsafe extern "C" fn sdm845_jack_free(jack: *mut snd_jack) {
    let component = (*jack).private_data as *mut snd_soc_component;

    snd_soc_component_set_jack(component, ptr::null_mut(), ptr::null_mut());
}

unsafe extern "C" fn sdm845_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut component: *mut snd_soc_component;
    let card = (*rtd).card;
    let mut codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let pdata = snd_soc_card_get_drvdata(card) as *mut sdm845_snd_data;
    let link = (*rtd).dai_link;
    let mut jack: *mut snd_jack;
    /*
     * Codec SLIMBUS configuration
     * RX1, RX2, RX3, RX4, RX5, RX6, RX7, RX8, RX9, RX10, RX11, RX12, RX13
     * TX1, TX2, TX3, TX4, TX5, TX6, TX7, TX8, TX9, TX10, TX11, TX12, TX13
     * TX14, TX15, TX16
     */
    let rx_ch: [c_uint; SLIM_MAX_RX_PORTS] = [144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156];
    let tx_ch: [c_uint; SLIM_MAX_TX_PORTS] = [128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143];
    let mut rval: c_int;

    if !(*pdata).jack_setup {
        rval = snd_soc_card_jack_new_pins(
            card,
            b"Headset Jack\0".as_ptr() as *const c_char,
            SND_JACK_HEADSET | SND_JACK_HEADPHONE | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
            &mut (*pdata).jack,
            SDM845_JACK_PINS.as_mut_ptr(),
            SDM845_JACK_PINS.len() as c_uint,
        );

        if rval < 0 {
            dev_err((*card).dev, b"Unable to add Headphone Jack\n\0".as_ptr() as *const c_char);
            return rval;
        }

        jack = (*pdata).jack.jack;

        snd_jack_set_key(jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key(jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key(jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key(jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
        (*pdata).jack_setup = true;
    }

    match (*cpu_dai).id {
        PRIMARY_MI2S_RX => {
            jack = (*pdata).jack.jack;
            component = (*codec_dai).component;

            (*jack).private_data = component as *mut c_void;
            (*jack).private_free = Some(sdm845_jack_free);
            rval = snd_soc_component_set_jack(component, &mut (*pdata).jack, ptr::null_mut());
            if rval != 0 && rval != -ENOTSUPP {
                dev_warn((*card).dev, b"Failed to set jack: %d\n\0".as_ptr() as *const c_char, rval);
                return rval;
            }
        }
        id if id >= SLIMBUS_0_RX && id <= SLIMBUS_6_TX => {
            /* setting up wcd multiple times for slim port is redundant */
            if (*pdata).slim_port_setup || (*link).no_pcm == 0 {
                return 0;
            }

            // for_each_rtd_codec_dais(rtd, i, codec_dai)
            let mut i: c_int = 0;
            while {
                codec_dai = snd_soc_rtd_to_codec(rtd, i as c_uint);
                !codec_dai.is_null()
            } {
                rval = snd_soc_dai_set_channel_map(
                    codec_dai,
                    tx_ch.len() as u32,
                    tx_ch.as_ptr(),
                    rx_ch.len() as u32,
                    rx_ch.as_ptr(),
                );
                if rval != 0 && rval != -ENOTSUPP {
                    return rval;
                }

                snd_soc_dai_set_sysclk(
                    codec_dai,
                    0,
                    WCD934X_DEFAULT_MCLK_RATE,
                    SNDRV_PCM_STREAM_PLAYBACK,
                );

                rval = snd_soc_component_set_jack((*codec_dai).component, &mut (*pdata).jack, ptr::null_mut());
                if rval != 0 && rval != -ENOTSUPP {
                    dev_warn((*card).dev, b"Failed to set jack: %d\n\0".as_ptr() as *const c_char, rval);
                    return rval;
                }
                i += 1;
            }

            (*pdata).slim_port_setup = true;
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn sdm845_snd_startup(substream: *mut snd_pcm_substream) -> c_int {
    let fmt: c_uint = SND_SOC_DAIFMT_BP_FP;
    let mut codec_dai_fmt: c_uint = SND_SOC_DAIFMT_BC_FC;
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let data = snd_soc_card_get_drvdata(card) as *mut sdm845_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    match (*cpu_dai).id {
        PRIMARY_MI2S_RX | PRIMARY_MI2S_TX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF;
            (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_add(1);
            if (*data).pri_mi2s_clk_count == 1 {
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_MCLK_1, DEFAULT_MCLK_RATE, SNDRV_PCM_STREAM_PLAYBACK);
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT, MI2S_BCLK_RATE, SNDRV_PCM_STREAM_PLAYBACK);
            }
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        SECONDARY_MI2S_RX | SECONDARY_MI2S_TX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            (*data).sec_mi2s_clk_count = (*data).sec_mi2s_clk_count.wrapping_add(1);
            if (*data).sec_mi2s_clk_count == 1 {
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT, MI2S_BCLK_RATE, SNDRV_PCM_STREAM_CAPTURE);
            }
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        QUATERNARY_MI2S_RX => {
            codec_dai_fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S;
            snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_QUAD_MI2S_IBIT, MI2S_BCLK_RATE, SNDRV_PCM_STREAM_PLAYBACK);
            snd_soc_dai_set_fmt(cpu_dai, fmt);
            snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
        }
        QUATERNARY_TDM_RX_0 | QUATERNARY_TDM_TX_0 => {
            (*data).quat_tdm_clk_count = (*data).quat_tdm_clk_count.wrapping_add(1);
            if (*data).quat_tdm_clk_count == 1 {
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_QUAD_TDM_IBIT, TDM_BCLK_RATE, SNDRV_PCM_STREAM_PLAYBACK);
            }

            codec_dai_fmt |= SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_DSP_B;

            // for_each_rtd_codec_dais(rtd, j, codec_dai)
            let mut j: c_int = 0;
            while {
                codec_dai = snd_soc_rtd_to_codec(rtd, j as c_uint);
                !codec_dai.is_null()
            } {
                if strcmp((*(*codec_dai).component).name_prefix, b"Left\0".as_ptr() as *const c_char) == 0 {
                    ret = snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
                    if ret < 0 {
                        dev_err((*rtd).dev, b"Left TDM fmt err:%d\n\0".as_ptr() as *const c_char, ret);
                        return ret;
                    }
                }

                if strcmp((*(*codec_dai).component).name_prefix, b"Right\0".as_ptr() as *const c_char) == 0 {
                    ret = snd_soc_dai_set_fmt(codec_dai, codec_dai_fmt);
                    if ret < 0 {
                        dev_err((*rtd).dev, b"Right TDM slot err:%d\n\0".as_ptr() as *const c_char, ret);
                        return ret;
                    }
                }
                j += 1;
            }
        }
        id if id >= SLIMBUS_0_RX && id <= SLIMBUS_6_TX => {}
        _ => {
            pr_err(
                b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const c_char,
                b"sdm845_snd_startup\0".as_ptr() as *const c_char,
                (*cpu_dai).id,
            );
        }
    }
    qcom_snd_sdw_startup(substream)
}

unsafe extern "C" fn sdm845_snd_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let data = snd_soc_card_get_drvdata(card) as *mut sdm845_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    match (*cpu_dai).id {
        PRIMARY_MI2S_RX | PRIMARY_MI2S_TX => {
            (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_sub(1);
            if (*data).pri_mi2s_clk_count == 0 {
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_MCLK_1, 0, SNDRV_PCM_STREAM_PLAYBACK);
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT, 0, SNDRV_PCM_STREAM_PLAYBACK);
            }
        }
        SECONDARY_MI2S_RX | SECONDARY_MI2S_TX => {
            (*data).sec_mi2s_clk_count = (*data).sec_mi2s_clk_count.wrapping_sub(1);
            if (*data).sec_mi2s_clk_count == 0 {
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_SEC_MI2S_IBIT, 0, SNDRV_PCM_STREAM_CAPTURE);
            }
        }
        QUATERNARY_TDM_RX_0 | QUATERNARY_TDM_TX_0 => {
            (*data).quat_tdm_clk_count = (*data).quat_tdm_clk_count.wrapping_sub(1);
            if (*data).quat_tdm_clk_count == 0 {
                snd_soc_dai_set_sysclk(cpu_dai, Q6AFE_LPASS_CLK_ID_QUAD_TDM_IBIT, 0, SNDRV_PCM_STREAM_PLAYBACK);
            }
        }
        id if (id >= SLIMBUS_0_RX && id <= SLIMBUS_6_TX) || id == QUATERNARY_MI2S_RX => {}
        _ => {
            pr_err(
                b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const c_char,
                b"sdm845_snd_shutdown\0".as_ptr() as *const c_char,
                (*cpu_dai).id,
            );
        }
    }

    qcom_snd_sdw_shutdown(substream);
}

unsafe extern "C" fn sdm845_snd_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sdm845_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    qcom_snd_sdw_prepare(substream, &mut (*data).stream_prepared[(*cpu_dai).id as usize])
}

unsafe extern "C" fn sdm845_snd_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sdm845_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    qcom_snd_sdw_hw_free(substream, &mut (*data).stream_prepared[(*cpu_dai).id as usize])
}

static SDM845_BE_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(sdm845_snd_hw_params),
    hw_free: Some(sdm845_snd_hw_free),
    prepare: Some(sdm845_snd_prepare),
    startup: Some(sdm845_snd_startup),
    shutdown: Some(sdm845_snd_shutdown),
};

unsafe extern "C" fn sdm845_be_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    (*rate).max = DEFAULT_SAMPLE_RATE_48K;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);

    0
}

static SDM845_SND_WIDGETS: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { name: b"Headphone Jack\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Left Spk\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Right Spk\0".as_ptr() as *const c_char },
    snd_soc_dapm_widget { name: b"Int Mic\0".as_ptr() as *const c_char },
];

static SDM845_SND_CONTROLS: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { name: b"Headphone Jack\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() as *const c_char },
];

unsafe fn sdm845_add_ops(card: *mut snd_soc_card) {
    let mut link: *mut snd_soc_dai_link;

    // for_each_card_prelinks(card, i, link)
    let mut i: c_int = 0;
    while {
        link = (*card_prelink(card, i)).cast();
        !link.is_null()
    } {
        if (*link).no_pcm == 1 {
            (*link).ops = &SDM845_BE_OPS;
            (*link).be_hw_params_fixup = Some(sdm845_be_hw_params_fixup);
        }
        (*link).init = Some(sdm845_dai_init);
        i += 1;
    }
}

unsafe fn card_prelink(_card: *mut snd_soc_card, _i: c_int) -> *mut c_void {
    // External expansion point for for_each_card_prelinks(card, i, link).
    ptr::null_mut()
}

unsafe extern "C" fn sdm845_snd_platform_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card;
    let data: *mut sdm845_snd_data;
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: c_int;

    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    /* Allocate the private data */
    data = devm_kzalloc(dev, core::mem::size_of::<sdm845_snd_data>(), GFP_KERNEL) as *mut sdm845_snd_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*card).driver_name = DRIVER_NAME.as_ptr() as *const c_char;
    (*card).dapm_widgets = SDM845_SND_WIDGETS.as_ptr();
    (*card).num_dapm_widgets = SDM845_SND_WIDGETS.len() as c_int;
    (*card).controls = SDM845_SND_CONTROLS.as_ptr();
    (*card).num_controls = SDM845_SND_CONTROLS.len() as c_int;
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    dev_set_drvdata(dev, card as *mut c_void);
    ret = qcom_snd_parse_of(card);
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(card, data as *mut c_void);

    sdm845_add_ops(card);
    devm_snd_soc_register_card(dev, card)
}

static SDM845_SND_DEVICE_ID: [of_device_id; 4] = [
    of_device_id { compatible: b"qcom,sdm845-sndcard\0".as_ptr() as *const c_char },
    /* Do not grow the list for compatible devices */
    of_device_id { compatible: b"qcom,db845c-sndcard\0".as_ptr() as *const c_char },
    of_device_id { compatible: b"lenovo,yoga-c630-sndcard\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];

static mut SDM845_SND_DRIVER: platform_driver = platform_driver {
    probe: Some(sdm845_snd_platform_probe),
    driver: platform_driver_driver {
        name: b"msm-snd-sdm845\0".as_ptr() as *const c_char,
        of_match_table: SDM845_SND_DEVICE_ID.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, sdm845_snd_device_id);
// module_platform_driver(sdm845_snd_driver);
// MODULE_DESCRIPTION("sdm845 ASoC Machine Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
