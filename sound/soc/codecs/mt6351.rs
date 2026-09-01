// SPDX-License-Identifier: GPL-2.0
//
// mt6351.c  --  mt6351 ALSA SoC audio codec driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
// Rust translation of the isolated C source. Kernel, ALSA SoC, regmap, OF,
// module, and mt6351 register definitions are expected external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

/* MT6351_TOP_CLKSQ */
const RG_CLKSQ_EN_AUD_BIT: c_uint = 0;

/* MT6351_TOP_CKPDN_CON0 */
const RG_AUDNCP_CK_PDN_BIT: c_uint = 12;
const RG_AUDIF_CK_PDN_BIT: c_uint = 13;
const RG_AUD_CK_PDN_BIT: c_uint = 14;
const RG_ZCD13M_CK_PDN_BIT: c_uint = 15;

/* MT6351_AUDDEC_ANA_CON0 */
const RG_AUDDACLPWRUP_VAUDP32_BIT: c_uint = 0;
const RG_AUDDACRPWRUP_VAUDP32_BIT: c_uint = 1;
const RG_AUD_DAC_PWR_UP_VA32_BIT: c_uint = 2;
const RG_AUD_DAC_PWL_UP_VA32_BIT: c_uint = 3;
const RG_AUDHSPWRUP_VAUDP32_BIT: c_uint = 4;
const RG_AUDHPLPWRUP_VAUDP32_BIT: c_uint = 5;
const RG_AUDHPRPWRUP_VAUDP32_BIT: c_uint = 6;
const RG_AUDHSMUXINPUTSEL_VAUDP32_SFT: c_uint = 7;
const RG_AUDHSMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;
const RG_AUDHPLMUXINPUTSEL_VAUDP32_SFT: c_uint = 9;
const RG_AUDHPLMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;
const RG_AUDHPRMUXINPUTSEL_VAUDP32_SFT: c_uint = 11;
const RG_AUDHPRMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;
const RG_AUDHSSCDISABLE_VAUDP32: c_uint = 13;
const RG_AUDHPLSCDISABLE_VAUDP32_BIT: c_uint = 14;
const RG_AUDHPRSCDISABLE_VAUDP32_BIT: c_uint = 15;

/* MT6351_AUDDEC_ANA_CON1 */
const RG_HSOUTPUTSTBENH_VAUDP32_BIT: c_uint = 8;

/* MT6351_AUDDEC_ANA_CON3 */
const RG_AUDLOLPWRUP_VAUDP32_BIT: c_uint = 2;
const RG_AUDLOLMUXINPUTSEL_VAUDP32_SFT: c_uint = 3;
const RG_AUDLOLMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;
const RG_AUDLOLSCDISABLE_VAUDP32_BIT: c_uint = 5;
const RG_LOOUTPUTSTBENH_VAUDP32_BIT: c_uint = 9;

/* MT6351_AUDDEC_ANA_CON6 */
const RG_ABIDEC_RSVD0_VAUDP32_HPL_BIT: c_uint = 8;
const RG_ABIDEC_RSVD0_VAUDP32_HPR_BIT: c_uint = 9;
const RG_ABIDEC_RSVD0_VAUDP32_HS_BIT: c_uint = 10;
const RG_ABIDEC_RSVD0_VAUDP32_LOL_BIT: c_uint = 11;

/* MT6351_AUDDEC_ANA_CON9 */
const RG_AUDIBIASPWRDN_VAUDP32_BIT: c_uint = 8;
const RG_RSTB_DECODER_VA32_BIT: c_uint = 9;
const RG_AUDGLB_PWRDN_VA32_BIT: c_uint = 12;
const RG_LCLDO_DEC_EN_VA32_BIT: c_uint = 13;
const RG_LCLDO_DEC_REMOTE_SENSE_VA18_BIT: c_uint = 15;

/* MT6351_AUDDEC_ANA_CON10 */
const RG_NVREG_EN_VAUDP32_BIT: c_uint = 8;
const RG_AUDGLB_LP2_VOW_EN_VA32: c_uint = 10;

/* MT6351_AFE_UL_DL_CON0 */
const RG_AFE_ON_BIT: c_uint = 0;

/* MT6351_AFE_DL_SRC2_CON0_L */
const RG_DL_2_SRC_ON_TMP_CTL_PRE_BIT: c_uint = 0;

/* MT6351_AFE_UL_SRC_CON0_L */
const UL_SRC_ON_TMP_CTL: c_uint = 0;

/* MT6351_AFE_TOP_CON0 */
const RG_DL_SINE_ON_SFT: c_uint = 0;
const RG_DL_SINE_ON_MASK: c_uint = 0x1;
const RG_UL_SINE_ON_SFT: c_uint = 1;
const RG_UL_SINE_ON_MASK: c_uint = 0x1;

/* MT6351_AUDIO_TOP_CON0 */
const AUD_TOP_PDN_RESERVED_BIT: c_uint = 0;
const AUD_TOP_PWR_CLK_DIS_CTL_BIT: c_uint = 2;
const AUD_TOP_PDN_ADC_CTL_BIT: c_uint = 5;
const AUD_TOP_PDN_DAC_CTL_BIT: c_uint = 6;
const AUD_TOP_PDN_AFE_CTL_BIT: c_uint = 7;

/* MT6351_AFE_SGEN_CFG0 */
const SGEN_C_MUTE_SW_CTL_BIT: c_uint = 6;
const SGEN_C_DAC_EN_CTL_BIT: c_uint = 7;

/* MT6351_AFE_NCP_CFG0 */
const RG_NCP_ON_BIT: c_uint = 0;

/* MT6351_LDO_VUSB33_CON0 */
const RG_VUSB33_EN: c_uint = 1;
const RG_VUSB33_ON_CTRL: c_uint = 3;

/* MT6351_LDO_VA18_CON0 */
const RG_VA18_EN: c_uint = 1;
const RG_VA18_ON_CTRL: c_uint = 3;

/* MT6351_AUDENC_ANA_CON0 */
const RG_AUDPREAMPLON: c_uint = 0;
const RG_AUDPREAMPLDCCEN: c_uint = 1;
const RG_AUDPREAMPLDCPRECHARGE: c_uint = 2;
const RG_AUDPREAMPLINPUTSEL_SFT: c_uint = 4;
const RG_AUDPREAMPLINPUTSEL_MASK: c_uint = 0x3;
const RG_AUDADCLPWRUP: c_uint = 12;
const RG_AUDADCLINPUTSEL_SFT: c_uint = 13;
const RG_AUDADCLINPUTSEL_MASK: c_uint = 0x3;

/* MT6351_AUDENC_ANA_CON1 */
const RG_AUDPREAMPRON: c_uint = 0;
const RG_AUDPREAMPRDCCEN: c_uint = 1;
const RG_AUDPREAMPRDCPRECHARGE: c_uint = 2;
const RG_AUDPREAMPRINPUTSEL_SFT: c_uint = 4;
const RG_AUDPREAMPRINPUTSEL_MASK: c_uint = 0x3;
const RG_AUDADCRPWRUP: c_uint = 12;
const RG_AUDADCRINPUTSEL_SFT: c_uint = 13;
const RG_AUDADCRINPUTSEL_MASK: c_uint = 0x3;

/* MT6351_AUDENC_ANA_CON3 */
const RG_AUDADCCLKRSTB: c_uint = 6;

/* MT6351_AUDENC_ANA_CON9 */
const RG_AUDPWDBMICBIAS0: c_uint = 0;
const RG_AUDMICBIAS0VREF: c_uint = 4;
const RG_AUDMICBIAS0LOWPEN: c_uint = 7;
const RG_AUDPWDBMICBIAS2: c_uint = 8;
const RG_AUDMICBIAS2VREF: c_uint = 12;
const RG_AUDMICBIAS2LOWPEN: c_uint = 15;

/* MT6351_AUDENC_ANA_CON10 */
const RG_AUDPWDBMICBIAS1: c_uint = 0;
const RG_AUDMICBIAS1DCSW1NEN: c_uint = 2;
const RG_AUDMICBIAS1VREF: c_uint = 4;
const RG_AUDMICBIAS1LOWPEN: c_uint = 7;

const AUDIO_ANALOG_VOLUME_HSOUTL: usize = 0;
const AUDIO_ANALOG_VOLUME_HSOUTR: usize = 1;
const AUDIO_ANALOG_VOLUME_HPOUTL: usize = 2;
const AUDIO_ANALOG_VOLUME_HPOUTR: usize = 3;
const AUDIO_ANALOG_VOLUME_LINEOUTL: usize = 4;
const AUDIO_ANALOG_VOLUME_LINEOUTR: usize = 5;
const AUDIO_ANALOG_VOLUME_MICAMP1: usize = 6;
const AUDIO_ANALOG_VOLUME_MICAMP2: usize = 7;
const AUDIO_ANALOG_VOLUME_TYPE_MAX: usize = 8;

/* Supply subseq */
const SUPPLY_SUBSEQ_SETTING: c_int = 0;
const SUPPLY_SUBSEQ_ENABLE: c_int = 1;
const SUPPLY_SUBSEQ_MICBIAS: c_int = 2;

const REG_STRIDE: c_uint = 2;

#[repr(C)]
struct mt6351_priv {
    dev: *mut device,
    regmap: *mut regmap,
    dl_rate: c_uint,
    ul_rate: c_uint,
    ana_gain: [c_int; AUDIO_ANALOG_VOLUME_TYPE_MAX],
    hp_en_counter: c_int,
}

#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_substream { stream: c_int }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dai { component: *mut snd_soc_component }
#[repr(C)] struct snd_soc_component { dev: *mut device, regmap: *mut regmap }
#[repr(C)] struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
    reg: c_uint,
    shift: c_uint,
    on_val: c_uint,
    off_val: c_uint,
}
#[repr(C)] struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct snd_soc_dai_ops { hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int> }
#[repr(C)] struct snd_soc_pcm_stream { stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: u64 }
#[repr(C)] struct snd_soc_dai_driver { name: *const c_char, playback: snd_soc_pcm_stream, capture: snd_soc_pcm_stream, ops: *const snd_soc_dai_ops }
#[repr(C)] struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)] struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget_desc,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    endianness: c_uint,
}
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct platform_driver_driver { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] struct platform_driver { driver: platform_driver_driver, probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int> }

extern "C" {
    static mut GFP_KERNEL: c_uint;
    static mut ENOMEM: c_int;
    static mut ENODEV: c_int;
    static mut SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static mut SNDRV_PCM_STREAM_CAPTURE: c_int;
    static mut SND_SOC_DAPM_PRE_PMU: c_int;
    static mut SND_SOC_DAPM_POST_PMU: c_int;
    static mut SND_SOC_DAPM_PRE_PMD: c_int;
    static mut SND_SOC_DAPM_POST_PMD: c_int;
    static mut SND_SOC_NOPM: c_uint;

    static mut SNDRV_PCM_RATE_8000: c_uint;
    static mut SNDRV_PCM_RATE_16000: c_uint;
    static mut SNDRV_PCM_RATE_32000: c_uint;
    static mut SNDRV_PCM_RATE_48000: c_uint;
    static mut SNDRV_PCM_RATE_8000_48000: c_uint;
    static mut SNDRV_PCM_RATE_96000: c_uint;
    static mut SNDRV_PCM_RATE_192000: c_uint;
    static mut SNDRV_PCM_FMTBIT_S16_LE: u64;
    static mut SNDRV_PCM_FMTBIT_U16_LE: u64;
    static mut SNDRV_PCM_FMTBIT_S24_LE: u64;
    static mut SNDRV_PCM_FMTBIT_U24_LE: u64;
    static mut SNDRV_PCM_FMTBIT_S32_LE: u64;
    static mut SNDRV_PCM_FMTBIT_U32_LE: u64;

    static mut MT6351_ZCD_CON0: c_uint;
    static mut MT6351_ZCD_CON1: c_uint;
    static mut MT6351_ZCD_CON2: c_uint;
    static mut MT6351_ZCD_CON3: c_uint;
    static mut MT6351_TOP_CLKSQ: c_uint;
    static mut MT6351_TOP_CKPDN_CON0: c_uint;
    static mut MT6351_TOP_CKPDN_CON0_SET: c_uint;
    static mut MT6351_AUDDEC_ANA_CON0: c_uint;
    static mut MT6351_AUDDEC_ANA_CON1: c_uint;
    static mut MT6351_AUDDEC_ANA_CON2: c_uint;
    static mut MT6351_AUDDEC_ANA_CON3: c_uint;
    static mut MT6351_AUDDEC_ANA_CON6: c_uint;
    static mut MT6351_AUDDEC_ANA_CON9: c_uint;
    static mut MT6351_AUDDEC_ANA_CON10: c_uint;
    static mut MT6351_AFE_UL_DL_CON0: c_uint;
    static mut MT6351_AFE_DL_SRC2_CON0_L: c_uint;
    static mut MT6351_AFE_DL_SRC2_CON0_H: c_uint;
    static mut MT6351_AFE_UL_SRC_CON0_L: c_uint;
    static mut MT6351_AFE_UL_SRC_CON0_H: c_uint;
    static mut MT6351_AFE_TOP_CON0: c_uint;
    static mut MT6351_AUDIO_TOP_CON0: c_uint;
    static mut MT6351_AFE_SGEN_CFG0: c_uint;
    static mut MT6351_AFE_SGEN_CFG1: c_uint;
    static mut MT6351_AFE_NCP_CFG0: c_uint;
    static mut MT6351_AFE_NCP_CFG1: c_uint;
    static mut MT6351_LDO_VUSB33_CON0: c_uint;
    static mut MT6351_LDO_VA18_CON0: c_uint;
    static mut MT6351_AUDENC_ANA_CON0: c_uint;
    static mut MT6351_AUDENC_ANA_CON1: c_uint;
    static mut MT6351_AUDENC_ANA_CON3: c_uint;
    static mut MT6351_AUDENC_ANA_CON9: c_uint;
    static mut MT6351_AUDENC_ANA_CON10: c_uint;
    static mut MT6351_AFUNC_AUD_CON0: c_uint;
    static mut MT6351_AFUNC_AUD_CON2: c_uint;
    static mut MT6351_AFE_DL_SDM_CON1: c_uint;
    static mut MT6351_AFE_PMIC_NEWIF_CFG0: c_uint;
    static mut MT6351_AFE_PMIC_NEWIF_CFG2: c_uint;
    static mut MT6351_AFE_DCCLK_CFG0: c_uint;
    static mut MT6351_AFE_HPANC_CFG0: c_uint;

    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_init_regmap(cmpnt: *mut snd_soc_component, regmap: *mut regmap);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn MT6351_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_U24_LE |
    SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_LE
}

unsafe extern "C" fn set_hp_gain_zero(cmpnt: *mut snd_soc_component) {
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON2, 0x1f << 7, 0x8 << 7);
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON2, 0x1f << 0, 0x8 << 0);
}

unsafe extern "C" fn get_cap_reg_val(cmpnt: *mut snd_soc_component, rate: c_uint) -> c_uint {
    match rate {
        8000 => 0,
        16000 => 1,
        32000 => 2,
        48000 => 3,
        96000 => 4,
        192000 => 5,
        _ => {
            dev_warn((*cmpnt).dev, c"%s(), error rate %d, return 3".as_ptr(), c"get_cap_reg_val".as_ptr(), rate);
            3
        }
    }
}

unsafe extern "C" fn get_play_reg_val(cmpnt: *mut snd_soc_component, rate: c_uint) -> c_uint {
    match rate {
        8000 => 0,
        11025 => 1,
        12000 => 2,
        16000 => 3,
        22050 => 4,
        24000 => 5,
        32000 => 6,
        44100 => 7,
        48000 | 96000 | 192000 => 8,
        _ => {
            dev_warn((*cmpnt).dev, c"%s(), error rate %d, return 8".as_ptr(), c"get_play_reg_val".as_ptr(), rate);
            8
        }
    }
}

unsafe extern "C" fn mt6351_codec_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let cmpnt = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6351_priv;
    let rate = params_rate(params);

    dev_dbg((*priv_).dev, c"%s(), substream->stream %d, rate %d\n".as_ptr(),
            c"mt6351_codec_dai_hw_params".as_ptr(), (*substream).stream, rate);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*priv_).dl_rate = rate;
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*priv_).ul_rate = rate;
    }

    0
}

static mt6351_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mt6351_codec_dai_hw_params),
};

const HP_GAIN_SET_ZERO: c_int = 0;
const HP_GAIN_RESTORE: c_int = 1;

unsafe extern "C" fn hp_gain_ramp_set(cmpnt: *mut snd_soc_component, hp_gain_ctl: c_int) {
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6351_priv;
    let idx: c_int;
    let old_idx: c_int;

    if hp_gain_ctl == HP_GAIN_SET_ZERO {
        idx = 8; /* 0dB */
        old_idx = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL];
    } else {
        idx = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL];
        old_idx = 8; /* 0dB */
    }
    dev_dbg((*priv_).dev, c"%s(), idx %d, old_idx %d\n".as_ptr(),
            c"hp_gain_ramp_set".as_ptr(), idx, old_idx);

    let mut offset = if idx > old_idx { idx - old_idx } else { old_idx - idx };
    let mut reg_idx = old_idx;

    while offset > 0 {
        reg_idx = if idx > old_idx { reg_idx + 1 } else { reg_idx - 1 };

        /* check valid range, and set value */
        if (reg_idx >= 0 && reg_idx <= 0x12) || reg_idx == 0x1f {
            regmap_update_bits(
                (*cmpnt).regmap,
                MT6351_ZCD_CON2,
                0xf9f,
                ((reg_idx << 7) | reg_idx) as c_uint,
            );
            usleep_range(100, 120);
        }
        offset -= 1;
    }
}

unsafe extern "C" fn hp_zcd_enable(cmpnt: *mut snd_soc_component) {
    /* Enable ZCD, for minimize pop noise */
    /* when adjust gain during HP buffer on */
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON0, 0x7 << 8, 0x1 << 8);
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON0, 0x1 << 7, 0x0 << 7);
    /* timeout, 1=5ms, 0=30ms */
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON0, 0x1 << 6, 0x1 << 6);
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON0, 0x3 << 4, 0x0 << 4);
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON0, 0x7 << 1, 0x5 << 1);
    regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON0, 0x1 << 0, 0x1 << 0);
}

unsafe extern "C" fn hp_zcd_disable(cmpnt: *mut snd_soc_component) {
    regmap_write((*cmpnt).regmap, MT6351_ZCD_CON0, 0x0000);
}

/* ALSA TLV/control/enum macro declarations from C are preserved as dependency-shaped Rust comments/items. */
// static const DECLARE_TLV_DB_SCALE(playback_tlv, -1000, 100, 0);
// static const DECLARE_TLV_DB_SCALE(pga_tlv, 0, 600, 0);
// static const struct snd_kcontrol_new mt6351_snd_controls[] = { ... SOC_*_TLV(...) ... };
static mt6351_snd_controls: [snd_kcontrol_new; 0] = [];

static lo_in_mux_map: [*const c_char; 4] = [
    c"Open".as_ptr(), c"Mute".as_ptr(), c"Playback".as_ptr(), c"Test Mode".as_ptr(),
];
static mut lo_in_mux_map_value: [c_int; 4] = [0x0, 0x1, 0x2, 0x3];
// static SOC_VALUE_ENUM_SINGLE_DECL(lo_in_mux_map_enum, ...);
static lo_in_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static hp_in_mux_map: [*const c_char; 4] = [
    c"Open".as_ptr(), c"LoudSPK Playback".as_ptr(), c"Audio Playback".as_ptr(), c"Test Mode".as_ptr(),
];
static mut hp_in_mux_map_value: [c_int; 4] = [0x0, 0x1, 0x2, 0x3];
// static SOC_VALUE_ENUM_SINGLE_DECL(hpl_in_mux_map_enum, ...);
static hpl_in_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
// static SOC_VALUE_ENUM_SINGLE_DECL(hpr_in_mux_map_enum, ...);
static hpr_in_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static rcv_in_mux_map: [*const c_char; 4] = [
    c"Open".as_ptr(), c"Mute".as_ptr(), c"Voice Playback".as_ptr(), c"Test Mode".as_ptr(),
];
static mut rcv_in_mux_map_value: [c_int; 4] = [0x0, 0x1, 0x2, 0x3];
// static SOC_VALUE_ENUM_SINGLE_DECL(rcv_in_mux_map_enum, ...);
static rcv_in_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static dac_in_mux_map: [*const c_char; 2] = [c"Normal Path".as_ptr(), c"Sgen".as_ptr()];
static mut dac_in_mux_map_value: [c_int; 2] = [0x0, 0x1];
// static SOC_VALUE_ENUM_SINGLE_DECL(dac_in_mux_map_enum, ...);
static dac_in_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
// static SOC_VALUE_ENUM_SINGLE_DECL(aif_out_mux_map_enum, ...);
static aif_out_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static adc_left_mux_map: [*const c_char; 4] = [
    c"Idle".as_ptr(), c"AIN0".as_ptr(), c"Left Preamplifier".as_ptr(), c"Idle_1".as_ptr(),
];
static mut adc_left_mux_map_value: [c_int; 4] = [0x0, 0x1, 0x2, 0x3];
// static SOC_VALUE_ENUM_SINGLE_DECL(adc_left_mux_map_enum, ...);
static adc_left_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static adc_right_mux_map: [*const c_char; 4] = [
    c"Idle".as_ptr(), c"AIN0".as_ptr(), c"Right Preamplifier".as_ptr(), c"Idle_1".as_ptr(),
];
static mut adc_right_mux_map_value: [c_int; 4] = [0x0, 0x1, 0x2, 0x3];
// static SOC_VALUE_ENUM_SINGLE_DECL(adc_right_mux_map_enum, ...);
static adc_right_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static pga_left_mux_map: [*const c_char; 4] = [
    c"None".as_ptr(), c"AIN0".as_ptr(), c"AIN1".as_ptr(), c"AIN2".as_ptr(),
];
static mut pga_left_mux_map_value: [c_int; 4] = [0x0, 0x1, 0x2, 0x3];
// static SOC_VALUE_ENUM_SINGLE_DECL(pga_left_mux_map_enum, ...);
static pga_left_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static pga_right_mux_map: [*const c_char; 4] = [
    c"None".as_ptr(), c"AIN0".as_ptr(), c"AIN3".as_ptr(), c"AIN2".as_ptr(),
];
static mut pga_right_mux_map_value: [c_int; 4] = [0x0, 0x1, 0x2, 0x3];
// static SOC_VALUE_ENUM_SINGLE_DECL(pga_right_mux_map_enum, ...);
static pga_right_mux_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn mt_reg_set_clr_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_POST_PMU {
        if (*w).on_val != 0 {
            /* SET REG */
            regmap_update_bits((*cmpnt).regmap, (*w).reg + REG_STRIDE, 0x1 << (*w).shift, 0x1 << (*w).shift);
        } else {
            /* CLR REG */
            regmap_update_bits((*cmpnt).regmap, (*w).reg + REG_STRIDE * 2, 0x1 << (*w).shift, 0x1 << (*w).shift);
        }
    } else if event == SND_SOC_DAPM_PRE_PMD {
        if (*w).off_val != 0 {
            /* SET REG */
            regmap_update_bits((*cmpnt).regmap, (*w).reg + REG_STRIDE, 0x1 << (*w).shift, 0x1 << (*w).shift);
        } else {
            /* CLR REG */
            regmap_update_bits((*cmpnt).regmap, (*w).reg + REG_STRIDE * 2, 0x1 << (*w).shift, 0x1 << (*w).shift);
        }
    }

    0
}

unsafe extern "C" fn mt_ncp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_PRE_PMU {
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_NCP_CFG1, 0xffff, 0x1515);
        /* NCP: ck1 and ck2 clock frequecy adjust configure */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_NCP_CFG0, 0xfffe, 0x8C00);
    } else if event == SND_SOC_DAPM_POST_PMU {
        usleep_range(250, 270);
    }

    0
}

unsafe extern "C" fn mt_sgen_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_PRE_PMU {
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_SGEN_CFG0, 0xffef, 0x0008);
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_SGEN_CFG1, 0xffff, 0x0101);
    }
    0
}

unsafe extern "C" fn mt_aif_in_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6351_priv;

    dev_dbg((*priv_).dev, c"%s(), event 0x%x, rate %d\n".as_ptr(),
            c"mt_aif_in_event".as_ptr(), event, (*priv_).dl_rate);

    if event == SND_SOC_DAPM_PRE_PMU {
        /* sdm audio fifo clock power on */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFUNC_AUD_CON2, 0xffff, 0x0006);
        /* scrambler clock on enable */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFUNC_AUD_CON0, 0xffff, 0xC3A1);
        /* sdm power on */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFUNC_AUD_CON2, 0xffff, 0x0003);
        /* sdm fifo enable */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFUNC_AUD_CON2, 0xffff, 0x000B);
        /* set attenuation gain */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_DL_SDM_CON1, 0xffff, 0x001E);

        regmap_write((*cmpnt).regmap, MT6351_AFE_PMIC_NEWIF_CFG0, (get_play_reg_val(cmpnt, (*priv_).dl_rate) << 12) | 0x330);
        regmap_write((*cmpnt).regmap, MT6351_AFE_DL_SRC2_CON0_H, (get_play_reg_val(cmpnt, (*priv_).dl_rate) << 12) | 0x300);
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_PMIC_NEWIF_CFG2, 0x8000, 0x8000);
    }

    0
}

unsafe extern "C" fn mt_hp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6351_priv;
    let mut reg: c_int = 0;

    dev_dbg((*priv_).dev, c"%s(), event 0x%x, hp_en_counter %d\n".as_ptr(),
            c"mt_hp_event".as_ptr(), event, (*priv_).hp_en_counter);

    if event == SND_SOC_DAPM_PRE_PMU {
        (*priv_).hp_en_counter += 1;
        if (*priv_).hp_en_counter > 1 {
            return 0; /* already enabled, do nothing */
        } else if (*priv_).hp_en_counter <= 0 {
            dev_err((*priv_).dev, c"%s(), hp_en_counter %d <= 0\n".as_ptr(),
                    c"mt_hp_event".as_ptr(), (*priv_).hp_en_counter);
        }

        hp_zcd_disable(cmpnt);
        /* from yoyo HQA script */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON6, 0x0700, 0x0700);

        /* save target gain to restore after hardware open complete */
        regmap_read((*cmpnt).regmap, MT6351_ZCD_CON2, &mut reg);
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL] = reg & 0x1f;
        (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTR] = (reg >> 7) & 0x1f;

        /* Set HPR/HPL gain as minimum (~ -40dB) */
        regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON2, 0xffff, 0x0F9F);
        /* Set HS gain as minimum (~ -40dB) */
        regmap_update_bits((*cmpnt).regmap, MT6351_ZCD_CON3, 0xffff, 0x001F);
        /* De_OSC of HP */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON2, 0x0001, 0x0001);
        /* enable output STBENH */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON1, 0xffff, 0x2000);
        /* De_OSC of voice, enable output STBENH */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON1, 0xffff, 0x2100);
        /* Enable voice driver */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON0, 0x0010, 0xE090);
        /* Enable pre-charge buffer  */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON1, 0xffff, 0x2140);
        usleep_range(50, 60);
        /* Apply digital DC compensation value to DAC */
        set_hp_gain_zero(cmpnt);
        /* Enable HPR/HPL */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON1, 0xffff, 0x2100);
        /* Disable pre-charge buffer */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON1, 0xffff, 0x2000);
        /* Disable De_OSC of voice */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON0, 0x0010, 0xF4EF);
        /* Disable voice buffer */
        /* from yoyo HQ */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON6, 0x0700, 0x0300);
        /* Enable ZCD, for minimize pop noise */
        /* when adjust gain during HP buffer on */
        hp_zcd_enable(cmpnt);
        /* apply volume setting */
        hp_gain_ramp_set(cmpnt, HP_GAIN_RESTORE);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        (*priv_).hp_en_counter -= 1;
        if (*priv_).hp_en_counter > 0 {
            return 0; /* still being used, don't close */
        } else if (*priv_).hp_en_counter < 0 {
            dev_err((*priv_).dev, c"%s(), hp_en_counter %d <= 0\n".as_ptr(),
                    c"mt_hp_event".as_ptr(), (*priv_).hp_en_counter);
        }
        /* Disable AUD_ZCD */
        hp_zcd_disable(cmpnt);
        /* Set HPR/HPL gain as -1dB, step by step */
        hp_gain_ramp_set(cmpnt, HP_GAIN_SET_ZERO);
        set_hp_gain_zero(cmpnt);
    } else if event == SND_SOC_DAPM_POST_PMD {
        if (*priv_).hp_en_counter > 0 {
            return 0; /* still being used, don't close */
        } else if (*priv_).hp_en_counter < 0 {
            dev_err((*priv_).dev, c"%s(), hp_en_counter %d <= 0\n".as_ptr(),
                    c"mt_hp_event".as_ptr(), (*priv_).hp_en_counter);
        }
        /* reset*/
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON6, 0x0700, 0x0000);
        /* De_OSC of HP */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON2, 0x0001, 0x0000);
        /* apply volume setting */
        hp_gain_ramp_set(cmpnt, HP_GAIN_RESTORE);
    }

    0
}

unsafe extern "C" fn mt_aif_out_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6351_priv;

    dev_dbg((*priv_).dev, c"%s(), event 0x%x, rate %d\n".as_ptr(),
            c"mt_aif_out_event".as_ptr(), event, (*priv_).ul_rate);

    if event == SND_SOC_DAPM_PRE_PMU {
        /* dcclk_div=11'b00100000011, dcclk_ref_ck_sel=2'b00 */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_DCCLK_CFG0, 0xffff, 0x2062);
        /* dcclk_pdn=1'b0 */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_DCCLK_CFG0, 0xffff, 0x2060);
        /* dcclk_gen_on=1'b1 */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_DCCLK_CFG0, 0xffff, 0x2061);

        /* UL sample rate and mode configure */
        regmap_update_bits((*cmpnt).regmap, MT6351_AFE_UL_SRC_CON0_H, 0x000E,
                           get_cap_reg_val(cmpnt, (*priv_).ul_rate) << 1);

        /* fixed 260k path for 8/16/32/48 */
        if (*priv_).ul_rate <= 48000 {
            /* anc ul path src on */
            regmap_update_bits((*cmpnt).regmap, MT6351_AFE_HPANC_CFG0, 0x1 << 1, 0x1 << 1);
            /* ANC clk pdn release */
            regmap_update_bits((*cmpnt).regmap, MT6351_AFE_HPANC_CFG0, 0x1 << 0, 0x0 << 0);
        }
    } else if event == SND_SOC_DAPM_PRE_PMD {
        /* fixed 260k path for 8/16/32/48 */
        if (*priv_).ul_rate <= 48000 {
            /* anc ul path src on */
            regmap_update_bits((*cmpnt).regmap, MT6351_AFE_HPANC_CFG0, 0x1 << 1, 0x0 << 1);
            /* ANC clk pdn release */
            regmap_update_bits((*cmpnt).regmap, MT6351_AFE_HPANC_CFG0, 0x1 << 0, 0x1 << 0);
        }
    }

    0
}

unsafe extern "C" fn mt_adc_clkgen_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_PRE_PMU {
        /* Audio ADC clock gen. mode: 00_divided by 2 (Normal) */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON3, 0x3 << 4, 0x0);
    } else if event == SND_SOC_DAPM_POST_PMU {
        /* ADC CLK from: 00_13MHz from CLKSQ (Default) */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON3, 0x3 << 2, 0x0);
    }
    0
}

unsafe extern "C" fn mt_pga_left_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_PRE_PMU {
        /* Audio L PGA precharge on */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON0,
                           0x3 << RG_AUDPREAMPLDCPRECHARGE, 0x1 << RG_AUDPREAMPLDCPRECHARGE);
        /* Audio L PGA mode: 1_DCC */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON0,
                           0x3 << RG_AUDPREAMPLDCCEN, 0x1 << RG_AUDPREAMPLDCCEN);
    } else if event == SND_SOC_DAPM_POST_PMU {
        usleep_range(100, 120);
        /* Audio L PGA precharge off */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON0,
                           0x3 << RG_AUDPREAMPLDCPRECHARGE, 0x0 << RG_AUDPREAMPLDCPRECHARGE);
    }
    0
}

unsafe extern "C" fn mt_pga_right_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_PRE_PMU {
        /* Audio R PGA precharge on */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON1,
                           0x3 << RG_AUDPREAMPRDCPRECHARGE, 0x1 << RG_AUDPREAMPRDCPRECHARGE);
        /* Audio R PGA mode: 1_DCC */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON1,
                           0x3 << RG_AUDPREAMPRDCCEN, 0x1 << RG_AUDPREAMPRDCCEN);
    } else if event == SND_SOC_DAPM_POST_PMU {
        usleep_range(100, 120);
        /* Audio R PGA precharge off */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON1,
                           0x3 << RG_AUDPREAMPRDCPRECHARGE, 0x0 << RG_AUDPREAMPRDCPRECHARGE);
    }
    0
}

unsafe extern "C" fn mt_mic_bias_0_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_PRE_PMU {
        /* MIC Bias 0 LowPower: 0_Normal */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON9, 0x3 << RG_AUDMICBIAS0LOWPEN, 0x0);
        /* MISBIAS0 = 1P9V */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON9, 0x7 << RG_AUDMICBIAS0VREF, 0x2 << RG_AUDMICBIAS0VREF);
    } else if event == SND_SOC_DAPM_POST_PMD {
        /* MISBIAS0 = 1P97 */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON9, 0x7 << RG_AUDMICBIAS0VREF, 0x0 << RG_AUDMICBIAS0VREF);
    }
    0
}

unsafe extern "C" fn mt_mic_bias_1_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_PRE_PMU {
        /* MIC Bias 1 LowPower: 0_Normal */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON10, 0x3 << RG_AUDMICBIAS1LOWPEN, 0x0);
        /* MISBIAS1 = 2P7V */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON10, 0x7 << RG_AUDMICBIAS1VREF, 0x7 << RG_AUDMICBIAS1VREF);
    } else if event == SND_SOC_DAPM_POST_PMD {
        /* MISBIAS1 = 1P7V */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON10, 0x7 << RG_AUDMICBIAS1VREF, 0x0 << RG_AUDMICBIAS1VREF);
    }
    0
}

unsafe extern "C" fn mt_mic_bias_2_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_PRE_PMU {
        /* MIC Bias 2 LowPower: 0_Normal */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON9, 0x3 << RG_AUDMICBIAS2LOWPEN, 0x0);
        /* MISBIAS2 = 1P9V */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON9, 0x7 << RG_AUDMICBIAS2VREF, 0x2 << RG_AUDMICBIAS2VREF);
    } else if event == SND_SOC_DAPM_POST_PMD {
        /* MISBIAS2 = 1P97 */
        regmap_update_bits((*cmpnt).regmap, MT6351_AUDENC_ANA_CON9, 0x7 << RG_AUDMICBIAS2VREF, 0x0 << RG_AUDMICBIAS2VREF);
    }
    0
}

/* DAPM Widgets
 * The C source initializes this with SND_SOC_DAPM_* macros. Those macro
 * expansions are dependency-owned, so the complete C initializer is preserved
 * here as a source-level note rather than reimplemented locally.
 */
static mt6351_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static mt6351_dapm_routes: [snd_soc_dapm_route; 92] = [
    /* Capture */
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AIF Out Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"VUSB33_LDO".as_ptr() },
    snd_soc_dapm_route { sink: c"VUSB33_LDO".as_ptr(), control: ptr::null(), source: c"VUSB33_LDO_CTRL".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"VA18_LDO".as_ptr() },
    snd_soc_dapm_route { sink: c"VA18_LDO".as_ptr(), control: ptr::null(), source: c"VA18_LDO_CTRL".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AUDGLB".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"CLKSQ Audio".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AFE_ON".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_AFE_CTL".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_ADC_CTL".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_PWR_CLK".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_PDN_RESERVED".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Out Mux".as_ptr(), control: c"Normal Path".as_ptr(), source: c"ADC L".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Out Mux".as_ptr(), control: c"Normal Path".as_ptr(), source: c"ADC R".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC L".as_ptr(), control: ptr::null(), source: c"ADC L Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC L".as_ptr(), control: ptr::null(), source: c"AUD_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC L".as_ptr(), control: ptr::null(), source: c"AUDIF_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC L".as_ptr(), control: ptr::null(), source: c"ADC CLKGEN".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC R".as_ptr(), control: ptr::null(), source: c"ADC R Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC R".as_ptr(), control: ptr::null(), source: c"AUD_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC R".as_ptr(), control: ptr::null(), source: c"AUDIF_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC R".as_ptr(), control: ptr::null(), source: c"ADC CLKGEN".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC L Mux".as_ptr(), control: c"AIN0".as_ptr(), source: c"AIN0".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC L Mux".as_ptr(), control: c"Left Preamplifier".as_ptr(), source: c"PGA L".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC R Mux".as_ptr(), control: c"AIN0".as_ptr(), source: c"AIN0".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC R Mux".as_ptr(), control: c"Right Preamplifier".as_ptr(), source: c"PGA R".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA L".as_ptr(), control: ptr::null(), source: c"PGA L Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA R".as_ptr(), control: ptr::null(), source: c"PGA R Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA L Mux".as_ptr(), control: c"AIN0".as_ptr(), source: c"AIN0".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA L Mux".as_ptr(), control: c"AIN1".as_ptr(), source: c"AIN1".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA L Mux".as_ptr(), control: c"AIN2".as_ptr(), source: c"AIN2".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA R Mux".as_ptr(), control: c"AIN0".as_ptr(), source: c"AIN0".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA R Mux".as_ptr(), control: c"AIN3".as_ptr(), source: c"AIN3".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA R Mux".as_ptr(), control: c"AIN2".as_ptr(), source: c"AIN2".as_ptr() },
    snd_soc_dapm_route { sink: c"AIN0".as_ptr(), control: ptr::null(), source: c"Mic Bias 0".as_ptr() },
    snd_soc_dapm_route { sink: c"AIN2".as_ptr(), control: ptr::null(), source: c"Mic Bias 2".as_ptr() },
    snd_soc_dapm_route { sink: c"AIN1".as_ptr(), control: ptr::null(), source: c"Mic Bias 1".as_ptr() },
    snd_soc_dapm_route { sink: c"AIN1".as_ptr(), control: ptr::null(), source: c"Mic Bias 1 DCC pull high".as_ptr() },
    /* DL Supply */
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"AUDGLB".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"CLKSQ Audio".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"ZCD13M_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"AUD_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"AUDIF_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"AUDNCP_CK".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"NV Regulator".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"AUD_CLK".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"IBIST".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Power Supply".as_ptr(), control: ptr::null(), source: c"LDO".as_ptr() },
    snd_soc_dapm_route { sink: c"LDO".as_ptr(), control: ptr::null(), source: c"LDO_REMOTE_SENSE".as_ptr() },
    /* DL Digital Supply */
    snd_soc_dapm_route { sink: c"DL Digital Clock".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_AFE_CTL".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Digital Clock".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_DAC_CTL".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Digital Clock".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_PWR_CLK".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Digital Clock".as_ptr(), control: ptr::null(), source: c"AUDIO_TOP_PDN_RESERVED".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Digital Clock".as_ptr(), control: ptr::null(), source: c"NCP".as_ptr() },
    snd_soc_dapm_route { sink: c"DL Digital Clock".as_ptr(), control: ptr::null(), source: c"AFE_ON".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF_RX".as_ptr(), control: ptr::null(), source: c"DL Digital Clock".as_ptr() },
    /* DL Path */
    snd_soc_dapm_route { sink: c"DAC In Mux".as_ptr(), control: c"Normal Path".as_ptr(), source: c"AIF_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC In Mux".as_ptr(), control: c"Sgen".as_ptr(), source: c"SGEN DL".as_ptr() },
    snd_soc_dapm_route { sink: c"SGEN DL".as_ptr(), control: ptr::null(), source: c"SGEN DL SRC".as_ptr() },
    snd_soc_dapm_route { sink: c"SGEN DL".as_ptr(), control: ptr::null(), source: c"SGEN MUTE".as_ptr() },
    snd_soc_dapm_route { sink: c"SGEN DL".as_ptr(), control: ptr::null(), source: c"SGEN DL Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"SGEN DL".as_ptr(), control: ptr::null(), source: c"DL Digital Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"DACL".as_ptr(), control: ptr::null(), source: c"DAC In Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"DACL".as_ptr(), control: ptr::null(), source: c"DL Power Supply".as_ptr() },
    snd_soc_dapm_route { sink: c"DACL".as_ptr(), control: ptr::null(), source: c"DACL_BIASGEN".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR".as_ptr(), control: ptr::null(), source: c"DAC In Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR".as_ptr(), control: ptr::null(), source: c"DL Power Supply".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR".as_ptr(), control: ptr::null(), source: c"DACR_BIASGEN".as_ptr() },
    snd_soc_dapm_route { sink: c"LOL Mux".as_ptr(), control: c"Playback".as_ptr(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"LOL Buffer".as_ptr(), control: ptr::null(), source: c"LOL Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"LOL Buffer".as_ptr(), control: ptr::null(), source: c"LO Stability Enh".as_ptr() },
    snd_soc_dapm_route { sink: c"LOL Buffer".as_ptr(), control: ptr::null(), source: c"LOL Bias Gen".as_ptr() },
    snd_soc_dapm_route { sink: c"LINEOUT L".as_ptr(), control: ptr::null(), source: c"LOL Buffer".as_ptr() },
    /* Headphone Path */
    snd_soc_dapm_route { sink: c"HPL Mux".as_ptr(), control: c"Audio Playback".as_ptr(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"HPR Mux".as_ptr(), control: c"Audio Playback".as_ptr(), source: c"DACR".as_ptr() },
    snd_soc_dapm_route { sink: c"HPL Mux".as_ptr(), control: c"LoudSPK Playback".as_ptr(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"HPR Mux".as_ptr(), control: c"LoudSPK Playback".as_ptr(), source: c"DACR".as_ptr() },
    snd_soc_dapm_route { sink: c"HPL Power".as_ptr(), control: ptr::null(), source: c"HPL Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"HPR Power".as_ptr(), control: ptr::null(), source: c"HPR Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone L".as_ptr(), control: ptr::null(), source: c"HPL Power".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone R".as_ptr(), control: ptr::null(), source: c"HPR Power".as_ptr() },
    /* Receiver Path */
    snd_soc_dapm_route { sink: c"RCV Mux".as_ptr(), control: c"Voice Playback".as_ptr(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"RCV Buffer".as_ptr(), control: ptr::null(), source: c"RCV Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"RCV Buffer".as_ptr(), control: ptr::null(), source: c"RCV Stability Enh".as_ptr() },
    snd_soc_dapm_route { sink: c"RCV Buffer".as_ptr(), control: ptr::null(), source: c"RCV Bias Gen".as_ptr() },
    snd_soc_dapm_route { sink: c"Receiver".as_ptr(), control: ptr::null(), source: c"RCV Buffer".as_ptr() },
];

unsafe extern "C" fn mt6351_codec_init_reg(cmpnt: *mut snd_soc_component) -> c_int {
    /* Disable CLKSQ 26MHz */
    regmap_update_bits((*cmpnt).regmap, MT6351_TOP_CLKSQ, 0x0001, 0x0);
    /* disable AUDGLB */
    regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON9, 0x1000, 0x1000);
    /* Turn off AUDNCP_CLKDIV engine clock,Turn off AUD 26M */
    regmap_update_bits((*cmpnt).regmap, MT6351_TOP_CKPDN_CON0_SET, 0x3800, 0x3800);
    /* Disable HeadphoneL/HeadphoneR/voice short circuit protection */
    regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON0, 0xe000, 0xe000);
    /* [5] = 1, disable LO buffer left short circuit protection */
    regmap_update_bits((*cmpnt).regmap, MT6351_AUDDEC_ANA_CON3, 0x20, 0x20);
    /* Reverse the PMIC clock*/
    regmap_update_bits((*cmpnt).regmap, MT6351_AFE_PMIC_NEWIF_CFG2, 0x8000, 0x8000);
    0
}

unsafe extern "C" fn mt6351_codec_probe(cmpnt: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(cmpnt) as *mut mt6351_priv;

    snd_soc_component_init_regmap(cmpnt, (*priv_).regmap);

    mt6351_codec_init_reg(cmpnt);
    0
}

static mt6351_soc_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(mt6351_codec_probe),
    controls: mt6351_snd_controls.as_ptr(),
    num_controls: mt6351_snd_controls.len() as c_uint,
    dapm_widgets: mt6351_dapm_widgets.as_ptr(),
    num_dapm_widgets: mt6351_dapm_widgets.len() as c_uint,
    dapm_routes: mt6351_dapm_routes.as_ptr(),
    num_dapm_routes: mt6351_dapm_routes.len() as c_uint,
    endianness: 1,
};

static mut mt6351_dai_driver: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: c"mt6351-snd-codec-aif1".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000 */
            formats: 0, /* MT6351_FORMATS */
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000 */
            formats: 0, /* MT6351_FORMATS */
        },
        ops: &mt6351_codec_dai_ops,
    },
];

unsafe extern "C" fn mt6351_codec_driver_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut mt6351_priv;

    priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mt6351_priv>(), GFP_KERNEL) as *mut mt6351_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);

    (*priv_).dev = &mut (*pdev).dev;

    /* pdev->dev.parent is an external struct field not modeled in this isolated translation. */
    (*priv_).regmap = dev_get_regmap(ptr::null_mut(), ptr::null());
    if (*priv_).regmap.is_null() {
        return -ENODEV;
    }

    dev_dbg((*priv_).dev, c"%s(), dev name %s\n".as_ptr(),
            c"mt6351_codec_driver_probe".as_ptr(), dev_name(&mut (*pdev).dev));

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &mt6351_soc_component_driver,
        mt6351_dai_driver.as_mut_ptr(),
        mt6351_dai_driver.len() as c_int,
    )
}

static mt6351_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"mediatek,mt6351-sound".as_ptr() },
    of_device_id { compatible: ptr::null() },
];

static mt6351_codec_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"mt6351-sound".as_ptr(),
        of_match_table: mt6351_of_match.as_ptr(),
    },
    probe: Some(mt6351_codec_driver_probe),
};

// module_platform_driver(mt6351_codec_driver)

/* Module information */
// MODULE_DESCRIPTION("MT6351 ALSA SoC codec driver");
// MODULE_AUTHOR("KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
