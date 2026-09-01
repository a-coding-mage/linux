// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2022, Linaro Limited

// Dependencies from the original C includes:
// dt-bindings/sound/qcom,q6afe.h, linux/module.h, linux/platform_device.h,
// sound/soc.h, sound/soc-dapm.h, sound/pcm.h, sound/pcm_params.h,
// linux/soundwire/sdw.h, sound/jack.h, linux/input-event-codes.h,
// qdsp6/q6afe.h, qdsp6/q6apm.h, qdsp6/q6prm.h, qdsp6/q6dsp-common.h,
// common.h, sdw.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const I2S_MCLKFS: c_int = 256;

#[inline]
const fn I2S_MCLK_RATE(rate: c_int) -> c_int {
    rate * I2S_MCLKFS
}

#[inline]
const fn I2S_BIT_RATE(rate: c_int, channels: c_int, format: c_int) -> c_int {
    rate * channels * format
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
pub struct snd_soc_dapm_route {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub channels: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub driver_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct qcom_snd_tdm_slot_cfg {
    pub slots: c_int,
    pub slot_width: c_int,
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub no_pcm: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct qcom_snd_soc_common {
    pub driver_name: *const c_char,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub codec_dai_fmt: c_uint,
    pub codec_sysclk_set: bool,
    pub mi2s_mclk_enable: bool,
    pub mi2s_bclk_enable: bool,
    pub wcd_jack: bool,
    pub snd_prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct sc8280xp_snd_data {
    pub stream_prepared: [bool; AFE_PORT_MAX as usize],
    pub card: *mut snd_soc_card,
    pub jack: snd_soc_jack,
    pub dp_jack: [snd_soc_jack; 8],
    pub priv_: *const qcom_snd_soc_common,
    pub jack_setup: bool,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;

    static AFE_PORT_MAX: c_int;
    static ENOENT: c_int;
    static ENOTSUPP: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static GFP_KERNEL: c_uint;

    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_BC_FC: c_uint;
    static LPAIF_MI2S_BCLK: c_int;
    static LPAIF_MI2S_MCLK: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_CLOCK_OUT: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static PCM_CHANNEL_FL: c_uint;
    static PCM_CHANNEL_FR: c_uint;

    static WSA_CODEC_DMA_RX_0: c_int;
    static WSA_CODEC_DMA_RX_1: c_int;
    static DISPLAY_PORT_RX_0: c_int;
    static DISPLAY_PORT_RX_1: c_int;
    static DISPLAY_PORT_RX_7: c_int;
    static TX_CODEC_DMA_TX_0: c_int;
    static TX_CODEC_DMA_TX_1: c_int;
    static TX_CODEC_DMA_TX_2: c_int;
    static TX_CODEC_DMA_TX_3: c_int;
    static PRIMARY_MI2S_RX: c_int;
    static QUATERNARY_MI2S_TX: c_int;
    static QUINARY_MI2S_RX: c_int;
    static QUINARY_MI2S_TX: c_int;
    static SENARY_MI2S_RX: c_int;
    static SENARY_MI2S_TX: c_int;
    static LPI_MI2S_RX_0: c_int;
    static LPI_MI2S_TX_4: c_int;
    static PRIMARY_TDM_RX_0: c_int;
    static QUINARY_TDM_TX_7: c_int;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_codec_dai_count(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_rtd_to_codec_dai(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn qcom_snd_get_dai_tdm_slots(
        rtd: *mut snd_soc_pcm_runtime,
        cpu_cfg: *mut qcom_snd_tdm_slot_cfg,
        codec_cfg: *mut qcom_snd_tdm_slot_cfg,
    ) -> c_int;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn qcom_snd_apply_dai_tdm_slots_cfg(
        rtd: *mut snd_soc_pcm_runtime,
        cpu_cfg: *mut qcom_snd_tdm_slot_cfg,
        codec_cfg: *mut qcom_snd_tdm_slot_cfg,
    ) -> c_int;
    fn snd_soc_tdm_params_to_bclk(
        params: *mut snd_pcm_hw_params,
        slot_width: c_int,
        slots: c_int,
        slot_multiple: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_int, dir: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_limit_volume(card: *mut snd_soc_card, name: *const c_char, max: c_int) -> c_int;
    fn qcom_snd_dp_jack_setup(rtd: *mut snd_soc_pcm_runtime, jack: *mut snd_soc_jack, id: c_int) -> c_int;
    fn qcom_snd_wcd_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        jack_setup: *mut bool,
    ) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: c_uint,
        tx_slot: *const c_uint,
        rx_num: c_uint,
        rx_slot: *const c_uint,
    ) -> c_int;
    fn qcom_snd_sdw_prepare(substream: *mut snd_pcm_substream, prepared: *mut bool) -> c_int;
    fn qcom_snd_sdw_hw_free(substream: *mut snd_pcm_substream, prepared: *mut bool) -> c_int;
    fn qcom_snd_sdw_startup(substream: *mut snd_pcm_substream) -> c_int;
    fn qcom_snd_sdw_shutdown(substream: *mut snd_pcm_substream);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn qcom_snd_parse_of(card: *mut snd_soc_card) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn snd_soc_card_num_links(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_card_link(card: *mut snd_soc_card, index: c_int) -> *mut snd_soc_dai_link;
}

// Original C initializers use ASoC DAPM and control macros whose concrete
// struct expansions are supplied by external headers.
static mut sc8280xp_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    SND_SOC_DAPM_HP!("Headphone Jack", ptr::null()),
    SND_SOC_DAPM_MIC!("Mic Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP0 Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP1 Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP2 Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP3 Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP4 Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP5 Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP6 Jack", ptr::null()),
    SND_SOC_DAPM_SPK!("DP7 Jack", ptr::null()),
];

static max98090_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_PIN_SWITCH!("Headset Mic12"),
    SOC_DAPM_PIN_SWITCH!("Headphone"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic56"),
    SOC_DAPM_PIN_SWITCH!("Speaker"),
    SOC_DAPM_PIN_SWITCH!("Receiver"),
    SOC_DAPM_PIN_SWITCH!("Int Mic"),
];

static max98090_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_HP!("Headphone Jack", ptr::null()),
    SND_SOC_DAPM_MIC!("Mic Jack", ptr::null()),
    SND_SOC_DAPM_HP!("Headphone", ptr::null()),
    SND_SOC_DAPM_MIC!("Headset Mic12", ptr::null()),
    SND_SOC_DAPM_MIC!("Headset Mic56", ptr::null()),
    SND_SOC_DAPM_MIC!("Int Mic", ptr::null()),
    SND_SOC_DAPM_SPK!("Receiver", ptr::null()),
    SND_SOC_DAPM_SPK!("Speaker", ptr::null()),
];

#[inline]
unsafe fn sc8280xp_get_mclk_freq(params: *mut snd_pcm_hw_params) -> c_int {
    let rate = params_rate(params);

    match rate {
        11025 | 44100 | 88200 => return I2S_MCLK_RATE(44100),
        _ => {}
    }

    I2S_MCLK_RATE(rate)
}

#[inline]
unsafe fn sc8280xp_get_bclk_freq(params: *mut snd_pcm_hw_params) -> c_int {
    I2S_BIT_RATE(
        params_rate(params),
        params_channels(params),
        snd_pcm_format_width(params_format(params)),
    )
}

unsafe extern "C" fn sc8280xp_tdm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sc8280xp_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut snd_soc_dai;
    let mut cpu_cfg = core::mem::zeroed::<qcom_snd_tdm_slot_cfg>();
    let mut codec_cfg = core::mem::zeroed::<qcom_snd_tdm_slot_cfg>();
    let mut ret: c_int;

    ret = qcom_snd_get_dai_tdm_slots(rtd, &mut cpu_cfg, &mut codec_cfg);
    if ret != 0 {
        return if ret == -ENOENT { 0 } else { ret };
    }

    if cpu_cfg.slots == 0 {
        return 0;
    }

    ret = snd_soc_dai_set_fmt(cpu_dai, SND_SOC_DAIFMT_BP_FP);
    if ret != 0 && ret != -ENOTSUPP {
        return ret;
    }

    if (*(*data).priv_).codec_dai_fmt != 0 {
        let mut i = 0;
        while i < snd_soc_rtd_codec_dai_count(rtd) {
            codec_dai = snd_soc_rtd_to_codec_dai(rtd, i);
            ret = snd_soc_dai_set_fmt(codec_dai, (*(*data).priv_).codec_dai_fmt);
            if ret != 0 && ret != -ENOTSUPP {
                return ret;
            }
            i += 1;
        }
    }

    ret = qcom_snd_apply_dai_tdm_slots_cfg(rtd, &mut cpu_cfg, &mut codec_cfg);
    if ret != 0 {
        return ret;
    }

    let bclk_freq = snd_soc_tdm_params_to_bclk(params, cpu_cfg.slot_width, cpu_cfg.slots, 1);
    if bclk_freq <= 0 {
        return -EINVAL;
    }

    if (*(*data).priv_).mi2s_bclk_enable {
        ret = snd_soc_dai_set_sysclk(cpu_dai, LPAIF_MI2S_BCLK, bclk_freq, SND_SOC_CLOCK_IN);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err(
                (*rtd).dev,
                b"%s: failed to set cpu sysclk: %d\n\0".as_ptr() as *const c_char,
                b"sc8280xp_tdm_hw_params\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    }

    if (*(*data).priv_).codec_sysclk_set {
        let mut i = 0;
        while i < snd_soc_rtd_codec_dai_count(rtd) {
            codec_dai = snd_soc_rtd_to_codec_dai(rtd, i);
            ret = snd_soc_dai_set_sysclk(codec_dai, 0, bclk_freq, SND_SOC_CLOCK_IN);
            if ret != 0 && ret != -ENOTSUPP {
                dev_err(
                    (*rtd).dev,
                    b"%s: failed to set codec sysclk on %s: %d\n\0".as_ptr() as *const c_char,
                    b"sc8280xp_tdm_hw_params\0".as_ptr() as *const c_char,
                    (*codec_dai).name,
                    ret,
                );
                return ret;
            }
            i += 1;
        }
    }

    0
}

unsafe extern "C" fn sc8280xp_snd_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sc8280xp_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let card = (*rtd).card;
    let mut dp_jack: *mut snd_soc_jack = ptr::null_mut();
    let mut dp_pcm_id: c_int = 0;

    if (*cpu_dai).id == WSA_CODEC_DMA_RX_0 || (*cpu_dai).id == WSA_CODEC_DMA_RX_1 {
        /*
         * Set limit of -3 dB on Digital Volume and 0 dB on PA Volume
         * to reduce the risk of speaker damage until we have active
         * speaker protection in place.
         */
        snd_soc_limit_volume(card, b"WSA_RX0 Digital Volume\0".as_ptr() as *const c_char, 81);
        snd_soc_limit_volume(card, b"WSA_RX1 Digital Volume\0".as_ptr() as *const c_char, 81);
        snd_soc_limit_volume(card, b"SpkrLeft PA Volume\0".as_ptr() as *const c_char, 17);
        snd_soc_limit_volume(card, b"SpkrRight PA Volume\0".as_ptr() as *const c_char, 17);
    } else if (*cpu_dai).id == DISPLAY_PORT_RX_0 {
        /* DISPLAY_PORT dai ids are not contiguous */
        dp_pcm_id = 0;
        dp_jack = (*data).dp_jack.as_mut_ptr().add(dp_pcm_id as usize);
    } else if (*cpu_dai).id >= DISPLAY_PORT_RX_1 && (*cpu_dai).id <= DISPLAY_PORT_RX_7 {
        dp_pcm_id = (*cpu_dai).id - DISPLAY_PORT_RX_1 + 1;
        dp_jack = (*data).dp_jack.as_mut_ptr().add(dp_pcm_id as usize);
    }

    if !dp_jack.is_null() {
        return qcom_snd_dp_jack_setup(rtd, dp_jack, dp_pcm_id);
    }

    if (*(*data).priv_).wcd_jack {
        return qcom_snd_wcd_jack_setup(rtd, &mut (*data).jack, &mut (*data).jack_setup);
    }

    0
}

unsafe extern "C" fn sc8280xp_be_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    (*rate).min = 48000;
    (*rate).max = 48000;
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);
    (*channels).min = 2;
    (*channels).max = 2;
    if (*cpu_dai).id == TX_CODEC_DMA_TX_0
        || (*cpu_dai).id == TX_CODEC_DMA_TX_1
        || (*cpu_dai).id == TX_CODEC_DMA_TX_2
        || (*cpu_dai).id == TX_CODEC_DMA_TX_3
    {
        (*channels).min = 1;
    }

    0
}

unsafe extern "C" fn sc8280xp_snd_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sc8280xp_snd_data;
    let mclk_freq = sc8280xp_get_mclk_freq(params);
    let bclk_freq = sc8280xp_get_bclk_freq(params);
    let mut ret: c_int;
    let id = (*cpu_dai).id;

    if (id >= PRIMARY_MI2S_RX && id <= QUATERNARY_MI2S_TX)
        || (id >= QUINARY_MI2S_RX && id <= QUINARY_MI2S_TX)
        || (id >= SENARY_MI2S_RX && id <= SENARY_MI2S_TX)
        || (id >= LPI_MI2S_RX_0 && id <= LPI_MI2S_TX_4)
    {
        ret = snd_soc_dai_set_fmt(cpu_dai, SND_SOC_DAIFMT_BP_FP);
        if ret != 0 && ret != -ENOTSUPP {
            return ret;
        }

        if (*(*data).priv_).codec_dai_fmt != 0 {
            ret = snd_soc_dai_set_fmt(codec_dai, (*(*data).priv_).codec_dai_fmt);
            if ret != 0 && ret != -ENOTSUPP {
                return ret;
            }
        }

        if (*(*data).priv_).mi2s_mclk_enable {
            ret = snd_soc_dai_set_sysclk(cpu_dai, LPAIF_MI2S_MCLK, mclk_freq, SND_SOC_CLOCK_OUT);
            if ret != 0 {
                return ret;
            }
        }

        if (*(*data).priv_).mi2s_bclk_enable {
            ret = snd_soc_dai_set_sysclk(cpu_dai, LPAIF_MI2S_BCLK, bclk_freq, SND_SOC_CLOCK_OUT);
            if ret != 0 {
                return ret;
            }
        }

        if (*(*data).priv_).codec_sysclk_set {
            ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk_freq, SND_SOC_CLOCK_IN);
            if ret != 0 && ret != -ENOTSUPP {
                return ret;
            }
        }
    } else if id >= PRIMARY_TDM_RX_0 && id <= QUINARY_TDM_TX_7 {
        return sc8280xp_tdm_hw_params(substream, params);
    }

    0
}

/*
 * WSA and WSA2 are handled as a single interface with the
 * following channels mask:
 *  __________________________________________________
 *  | Bits  |     3    |     2    |   1     |     0   |
 *  ---------------------------------------------------
 *  | Line  | WSA2 Ch2 | WSA2 Ch1 | WSA Ch2 | WSA Ch1 |
 *  ---------------------------------------------------
 *
 * The Ayaneo Pocket S2 speakers are connected only to
 * the WSA2 interface and the WSA interface is not enabled.
 *
 * Set the channel mapping on the WSA2 channels only.
 */
static ayaneo_ps2_channels_mapping: [c_uint; 4] = [
    0,              /* WSA Ch1 */
    0,              /* WSA Ch2 */
    PCM_CHANNEL_FL, /* WSA2 Ch1 */
    PCM_CHANNEL_FR, /* WSA2 Ch2 */
];

unsafe extern "C" fn ayaneo_ps2_snd_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let channels = (*(*substream).runtime).channels;

    if (*cpu_dai).id != WSA_CODEC_DMA_RX_0 {
        return 0;
    }

    if channels != 2 {
        return -EINVAL;
    }

    snd_soc_dai_set_channel_map(
        cpu_dai,
        0,
        ptr::null(),
        ayaneo_ps2_channels_mapping.len() as c_uint,
        ayaneo_ps2_channels_mapping.as_ptr(),
    )
}

unsafe extern "C" fn sc8280xp_snd_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sc8280xp_snd_data;

    if let Some(snd_prepare) = (*(*data).priv_).snd_prepare {
        let ret = snd_prepare(substream);
        if ret != 0 {
            return ret;
        }
    }

    qcom_snd_sdw_prepare(
        substream,
        (*data).stream_prepared.as_mut_ptr().add((*cpu_dai).id as usize),
    )
}

unsafe extern "C" fn sc8280xp_snd_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut sc8280xp_snd_data;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    qcom_snd_sdw_hw_free(
        substream,
        (*data).stream_prepared.as_mut_ptr().add((*cpu_dai).id as usize),
    )
}

static sc8280xp_be_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(qcom_snd_sdw_startup),
    shutdown: Some(qcom_snd_sdw_shutdown),
    hw_params: Some(sc8280xp_snd_hw_params),
    hw_free: Some(sc8280xp_snd_hw_free),
    prepare: Some(sc8280xp_snd_prepare),
};

unsafe fn sc8280xp_add_be_ops(card: *mut snd_soc_card) {
    let mut i = 0;

    while i < snd_soc_card_num_links(card) {
        let link = snd_soc_card_link(card, i);
        if (*link).no_pcm == 1 {
            (*link).init = Some(sc8280xp_snd_init);
            (*link).be_hw_params_fixup = Some(sc8280xp_be_hw_params_fixup);
            (*link).ops = &sc8280xp_be_ops;
        }
        i += 1;
    }
}

unsafe extern "C" fn sc8280xp_platform_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let dev = &mut (*pdev).dev as *mut device;
    let card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    /* Allocate the private data */
    let data = devm_kzalloc(dev, core::mem::size_of::<sc8280xp_snd_data>(), GFP_KERNEL)
        as *mut sc8280xp_snd_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).priv_ = of_device_get_match_data(dev) as *const qcom_snd_soc_common;
    if (*data).priv_.is_null() {
        return -ENODEV;
    }

    (*card).owner = THIS_MODULE;
    (*card).dev = dev;
    dev_set_drvdata(dev, card as *mut c_void);
    snd_soc_card_set_drvdata(card, data as *mut c_void);
    (*card).dapm_widgets = (*(*data).priv_).dapm_widgets;
    (*card).num_dapm_widgets = (*(*data).priv_).num_dapm_widgets;
    (*card).dapm_routes = (*(*data).priv_).dapm_routes;
    (*card).num_dapm_routes = (*(*data).priv_).num_dapm_routes;
    (*card).controls = (*(*data).priv_).controls;
    (*card).num_controls = (*(*data).priv_).num_controls;

    ret = qcom_snd_parse_of(card);
    if ret != 0 {
        return ret;
    }

    (*card).driver_name = (*(*data).priv_).driver_name;
    sc8280xp_add_be_ops(card);
    devm_snd_soc_register_card(dev, card)
}

static mut ayaneo_ps2_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"ayaneo-ps2\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: Some(ayaneo_ps2_snd_prepare),
};

static eliza_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"eliza\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: None,
};

static hawi_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"hawi\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: true,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: true,
    wcd_jack: true,
    snd_prepare: None,
};

static kaanapali_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"kaanapali\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: None,
};

static qcs9100_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"sa8775p\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: false,
    snd_prepare: None,
};

static qcs615_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"qcs615\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: true,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: false,
    snd_prepare: None,
};

static qcm6490_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"qcm6490\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: None,
};

static qcs6490_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"qcs6490\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: None,
};

static qcs8275_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"qcs8300\0".as_ptr() as *const c_char,
    dapm_widgets: max98090_dapm_widgets.as_ptr(),
    num_dapm_widgets: 8,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: max98090_controls.as_ptr(),
    num_controls: 6,
    codec_dai_fmt: SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_BC_FC,
    codec_sysclk_set: true,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: false,
    snd_prepare: None,
};

static sc8280xp_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"sc8280xp\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: None,
};

static sm8450_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"sm8450\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S,
    codec_sysclk_set: false,
    /* I2S Connected to HDMI */
    mi2s_mclk_enable: true,
    mi2s_bclk_enable: true,
    wcd_jack: true,
    snd_prepare: None,
};

static sm8475_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"sm8475\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: None,
};

static sm8550_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"sm8550\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S,
    codec_sysclk_set: false,
    /* I2S Connected to HDMI */
    mi2s_mclk_enable: true,
    mi2s_bclk_enable: true,
    wcd_jack: true,
    snd_prepare: None,
};

static sm8650_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"sm8650\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S,
    codec_sysclk_set: false,
    /* I2S Connected to HDMI */
    mi2s_mclk_enable: true,
    mi2s_bclk_enable: true,
    wcd_jack: true,
    snd_prepare: None,
};

static sm8750_priv_data: qcom_snd_soc_common = qcom_snd_soc_common {
    driver_name: b"sm8750\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { sc8280xp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 10,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    controls: ptr::null(),
    num_controls: 0,
    codec_dai_fmt: 0,
    codec_sysclk_set: false,
    mi2s_mclk_enable: false,
    mi2s_bclk_enable: false,
    wcd_jack: true,
    snd_prepare: None,
};

static snd_sc8280xp_dt_match: [of_device_id; 18] = [
    of_device_id {
        compatible: b"ayaneo,pocket-s2-sndcard\0".as_ptr() as *const c_char,
        data: unsafe { &ayaneo_ps2_priv_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: b"qcom,eliza-sndcard\0".as_ptr() as *const c_char,
        data: &eliza_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,hawi-sndcard\0".as_ptr() as *const c_char,
        data: &hawi_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,kaanapali-sndcard\0".as_ptr() as *const c_char,
        data: &kaanapali_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,maili-sndcard\0".as_ptr() as *const c_char,
        data: &hawi_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,qcm6490-idp-sndcard\0".as_ptr() as *const c_char,
        data: &qcm6490_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,qcs615-sndcard\0".as_ptr() as *const c_char,
        data: &qcs615_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,qcs6490-rb3gen2-sndcard\0".as_ptr() as *const c_char,
        data: &qcs6490_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,qcs8275-sndcard\0".as_ptr() as *const c_char,
        data: &qcs8275_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,qcs9075-sndcard\0".as_ptr() as *const c_char,
        data: &qcs9100_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,qcs9100-sndcard\0".as_ptr() as *const c_char,
        data: &qcs9100_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,sc8280xp-sndcard\0".as_ptr() as *const c_char,
        data: &sc8280xp_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,sm8450-sndcard\0".as_ptr() as *const c_char,
        data: &sm8450_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,sm8475-sndcard\0".as_ptr() as *const c_char,
        data: &sm8475_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,sm8550-sndcard\0".as_ptr() as *const c_char,
        data: &sm8550_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,sm8650-sndcard\0".as_ptr() as *const c_char,
        data: &sm8650_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"qcom,sm8750-sndcard\0".as_ptr() as *const c_char,
        data: &sm8750_priv_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, snd_sc8280xp_dt_match);

static mut snd_sc8280xp_driver: platform_driver = platform_driver {
    probe: Some(sc8280xp_platform_probe),
    driver: device_driver {
        name: b"snd-sc8280xp\0".as_ptr() as *const c_char,
        of_match_table: snd_sc8280xp_dt_match.as_ptr(),
    },
};

// module_platform_driver(snd_sc8280xp_driver);
// MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org");
// MODULE_DESCRIPTION("SC8280XP ASoC Machine Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
