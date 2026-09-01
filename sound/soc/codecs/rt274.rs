// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt274.rs -- source-level Rust translation of rt274.c.
 *
 * Original dependency intent:
 * linux/module.h, moduleparam.h, init.h, delay.h, pm.h, i2c.h,
 * platform_device.h, spi/spi.h, dmi.h, acpi.h, workqueue.h,
 * sound/core.h, pcm.h, pcm_params.h, soc.h, soc-dapm.h, initval.h,
 * tlv.h, jack.h, rl6347a.h, rt274.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub enum device {}
pub enum regmap {}
pub enum snd_soc_component {}
pub enum i2c_client {}
pub enum snd_soc_jack {}
pub enum work_struct {}
pub enum snd_kcontrol_new {}
pub enum snd_soc_dapm_widget {}
pub enum snd_soc_dapm_route {}
pub enum snd_pcm_substream {}
pub enum snd_pcm_hw_params {}
pub enum snd_soc_dai {}
pub enum snd_soc_dapm_context {}
pub enum snd_soc_dai_ops {}
pub enum snd_soc_dai_driver {}
pub enum snd_soc_component_driver {}
pub enum regmap_config {}
pub enum of_device_id {}
pub enum i2c_device_id {}
pub enum acpi_device_id {}
pub enum i2c_driver {}

pub type bool_ = bool;
pub type irqreturn_t = c_int;
pub type snd_soc_bias_level = c_int;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct rt274_priv {
    pub index_cache: *mut reg_default,
    pub index_cache_size: c_int,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub i2c: *mut i2c_client,
    pub jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub sys_clk: c_int,
    pub clk_id: c_int,
    pub fs: c_int,
    pub master: bool,
}

const RT274_VENDOR_ID: c_uint = 0x10ec0274;

macro_rules! ext_const {
    ($($name:ident),+ $(,)?) => {
        $(extern "C" { static $name: c_uint; })+
    };
}

ext_const!(
    AC_NODE_ROOT, AC_PAR_VENDOR_ID, RT274_GET_HP_SENSE, RT274_GET_MIC_SENSE,
    RT274_PROC_COEF, RT274_MIC, RT274_HP_OUT, RT274_DAC_OUT0, RT274_DAC_OUT1,
    RT274_ADC_IN1, RT274_ADC_IN2, RT274_DMIC1, RT274_DMIC2, RT274_LINE1,
    RT274_LINE2, RT274_MIXER_IN1, RT274_MIXER_IN2, RT274_INLINE_CMD,
    RT274_SET_AUDIO_POWER, RT274_SET_HPO_POWER, RT274_SET_DMIC1_POWER,
    RT274_LOUT_MUX, RT274_HPO_MUX, RT274_ADC0_MUX, RT274_ADC1_MUX,
    RT274_SET_MIC, RT274_SET_PIN_HPO, RT274_SET_PIN_LOUT3, RT274_SET_PIN_DMIC1,
    RT274_SET_AMP_GAIN_HPO, RT274_SET_DMIC2_DEFAULT, RT274_DAC0L_GAIN,
    RT274_DAC0R_GAIN, RT274_DAC1L_GAIN, RT274_DAC1R_GAIN, RT274_ADCL_GAIN,
    RT274_ADCR_GAIN, RT274_MIC_GAIN, RT274_HPOL_GAIN, RT274_HPOR_GAIN,
    RT274_LOUTL_GAIN, RT274_LOUTR_GAIN, RT274_DAC_FORMAT, RT274_ADC_FORMAT,
    RT274_COEF_INDEX, RT274_SET_AMP_GAIN_ADC_IN1, RT274_SET_AMP_GAIN_ADC_IN2,
    RT274_EAPD_GPIO_IRQ_CTRL, RT274_IRQ_EN, RT274_IRQ_DIS, RT274_MUTE_SFT,
    RT274_ADC_SEL_SFT, SND_SOC_NOPM, RT274_SET_STREAMID_ADC1,
    RT274_SET_STREAMID_ADC2, RT274_SET_STREAMID_DAC0, RT274_SET_STREAMID_DAC1,
    RT274_SET_PIN_SFT, SND_JACK_HEADPHONE, SND_JACK_MICROPHONE,
    SND_SOC_DAIFMT_MASTER_MASK, SND_SOC_DAIFMT_CBP_CFP, SND_SOC_DAIFMT_CBC_CFC,
    SND_SOC_DAIFMT_FORMAT_MASK, SND_SOC_DAIFMT_I2S, SND_SOC_DAIFMT_LEFT_J,
    SND_SOC_DAIFMT_DSP_A, SND_SOC_DAIFMT_DSP_B, RT274_I2S_CTRL1,
    RT274_I2S_MODE_MASK, RT274_I2S_MODE_M, RT274_I2S_MODE_S,
    RT274_I2S_FMT_MASK, RT274_I2S_FMT_I2S, RT274_I2S_FMT_LJ,
    RT274_I2S_FMT_PCMA, RT274_I2S_FMT_PCMB, RT274_PLL2_S_MCLK,
    RT274_PLL2_S_BCLK, RT274_PLL2_CTRL, RT274_PLL2_SRC_MASK,
    RT274_PLL2_SRC_MCLK, RT274_PLL2_SRC_BCLK, RT274_MCLK_CTRL,
    RT274_SCLK_S_MCLK, RT274_SCLK_S_PLL1, RT274_SCLK_S_PLL2,
    RT274_MCLK_MODE_EN, RT274_MCLK_MODE_DIS, RT274_CLK_SRC_MCLK,
    RT274_CLK_SRC_PLL2, RT274_MCLK_MODE_MASK, RT274_CLK_CTRL,
    RT274_CLK_SRC_MASK, RT274_I2S_CTRL2, RT274_TDM_EN, RT274_TDM_DIS,
    RT274_TDM_CH_NUM, RT274_TDM_4CH, RT274_TDM_2CH, SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY, AC_PWRST_D0, AC_PWRST_D3, RT274_IRQ_CLR,
    IRQ_HANDLED, SNDRV_PCM_RATE_44100, SNDRV_PCM_RATE_48000,
    SNDRV_PCM_FMTBIT_S16_LE, SNDRV_PCM_FMTBIT_S20_3LE,
    SNDRV_PCM_FMTBIT_S24_LE, SNDRV_PCM_FMTBIT_S8, RT274_AIF1,
    REGCACHE_RBTREE, GFP_KERNEL, RT274_RESET, RT274_PAD_CTRL12,
    RT274_COEF5b_INDEX, RT274_COEF5b_COEF, RT274_COEF58_INDEX,
    RT274_COEF58_COEF, RT274_GPI2_SEL_MASK, RT274_GPI2_SEL_DMIC_CLK,
    RT274_UNSOLICITED_HP_OUT, RT274_UNSOLICITED_MIC, IRQF_TRIGGER_HIGH,
    IRQF_ONESHOT,
);

extern "C" {
    fn RT274_GET_PARAM(node: c_uint, param: c_uint) -> c_uint;
    fn RT274_SET_POWER(node: c_uint) -> c_uint;
    fn VERB_CMD(verb: c_uint, nid: c_uint, payload: c_uint) -> c_uint;
    static AC_VERB_GET_EAPD_BTLENABLE: c_uint;
    static AC_VERB_GET_STREAM_FORMAT: c_uint;
    static AC_VERB_GET_AMP_GAIN_MUTE: c_uint;
    static AC_VERB_GET_CONNECT_SEL: c_uint;
    static AC_VERB_GET_PIN_WIDGET_CONTROL: c_uint;
    static AC_VERB_GET_UNSOLICITED_RESPONSE: c_uint;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn pm_wakeup_event(dev: *mut device, msec: c_uint);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

const rt274_index_def: [reg_default; 128] = [
    reg_default { reg: 0x00, def: 0x1004 }, reg_default { reg: 0x01, def: 0xaaaa },
    reg_default { reg: 0x02, def: 0x88aa }, reg_default { reg: 0x03, def: 0x0002 },
    reg_default { reg: 0x04, def: 0xaa09 }, reg_default { reg: 0x05, def: 0x0700 },
    reg_default { reg: 0x06, def: 0x6110 }, reg_default { reg: 0x07, def: 0x0200 },
    reg_default { reg: 0x08, def: 0xa807 }, reg_default { reg: 0x09, def: 0x0021 },
    reg_default { reg: 0x0a, def: 0x7770 }, reg_default { reg: 0x0b, def: 0x7770 },
    reg_default { reg: 0x0c, def: 0x002b }, reg_default { reg: 0x0d, def: 0x2420 },
    reg_default { reg: 0x0e, def: 0x65c0 }, reg_default { reg: 0x0f, def: 0x7770 },
    reg_default { reg: 0x10, def: 0x0420 }, reg_default { reg: 0x11, def: 0x7418 },
    reg_default { reg: 0x12, def: 0x6bd0 }, reg_default { reg: 0x13, def: 0x645f },
    reg_default { reg: 0x14, def: 0x0400 }, reg_default { reg: 0x15, def: 0x8ccc },
    reg_default { reg: 0x16, def: 0x4c50 }, reg_default { reg: 0x17, def: 0xff00 },
    reg_default { reg: 0x18, def: 0x0003 }, reg_default { reg: 0x19, def: 0x2c11 },
    reg_default { reg: 0x1a, def: 0x830b }, reg_default { reg: 0x1b, def: 0x4e4b },
    reg_default { reg: 0x1c, def: 0x0000 }, reg_default { reg: 0x1d, def: 0x0000 },
    reg_default { reg: 0x1e, def: 0x0000 }, reg_default { reg: 0x1f, def: 0x0000 },
    reg_default { reg: 0x20, def: 0x51ff }, reg_default { reg: 0x21, def: 0x8000 },
    reg_default { reg: 0x22, def: 0x8f00 }, reg_default { reg: 0x23, def: 0x88f4 },
    reg_default { reg: 0x24, def: 0x0000 }, reg_default { reg: 0x25, def: 0x0000 },
    reg_default { reg: 0x26, def: 0x0000 }, reg_default { reg: 0x27, def: 0x0000 },
    reg_default { reg: 0x28, def: 0x0000 }, reg_default { reg: 0x29, def: 0x3000 },
    reg_default { reg: 0x2a, def: 0x0000 }, reg_default { reg: 0x2b, def: 0x0000 },
    reg_default { reg: 0x2c, def: 0x0f00 }, reg_default { reg: 0x2d, def: 0x100f },
    reg_default { reg: 0x2e, def: 0x2902 }, reg_default { reg: 0x2f, def: 0xe280 },
    reg_default { reg: 0x30, def: 0x1000 }, reg_default { reg: 0x31, def: 0x8400 },
    reg_default { reg: 0x32, def: 0x5aaa }, reg_default { reg: 0x33, def: 0x8420 },
    reg_default { reg: 0x34, def: 0xa20c }, reg_default { reg: 0x35, def: 0x096a },
    reg_default { reg: 0x36, def: 0x5757 }, reg_default { reg: 0x37, def: 0xfe05 },
    reg_default { reg: 0x38, def: 0x4901 }, reg_default { reg: 0x39, def: 0x110a },
    reg_default { reg: 0x3a, def: 0x0010 }, reg_default { reg: 0x3b, def: 0x60d9 },
    reg_default { reg: 0x3c, def: 0xf214 }, reg_default { reg: 0x3d, def: 0xc2ba },
    reg_default { reg: 0x3e, def: 0xa928 }, reg_default { reg: 0x3f, def: 0x0000 },
    reg_default { reg: 0x40, def: 0x9800 }, reg_default { reg: 0x41, def: 0x0000 },
    reg_default { reg: 0x42, def: 0x2000 }, reg_default { reg: 0x43, def: 0x3d90 },
    reg_default { reg: 0x44, def: 0x4900 }, reg_default { reg: 0x45, def: 0x5289 },
    reg_default { reg: 0x46, def: 0x0004 }, reg_default { reg: 0x47, def: 0xa47a },
    reg_default { reg: 0x48, def: 0xd049 }, reg_default { reg: 0x49, def: 0x0049 },
    reg_default { reg: 0x4a, def: 0xa83b }, reg_default { reg: 0x4b, def: 0x0777 },
    reg_default { reg: 0x4c, def: 0x065c }, reg_default { reg: 0x4d, def: 0x7fff },
    reg_default { reg: 0x4e, def: 0x7fff }, reg_default { reg: 0x4f, def: 0x0000 },
    reg_default { reg: 0x50, def: 0x0000 }, reg_default { reg: 0x51, def: 0x0000 },
    reg_default { reg: 0x52, def: 0xbf5f }, reg_default { reg: 0x53, def: 0x3320 },
    reg_default { reg: 0x54, def: 0xcc00 }, reg_default { reg: 0x55, def: 0x0000 },
    reg_default { reg: 0x56, def: 0x3f00 }, reg_default { reg: 0x57, def: 0x0000 },
    reg_default { reg: 0x58, def: 0x0000 }, reg_default { reg: 0x59, def: 0x0000 },
    reg_default { reg: 0x5a, def: 0x1300 }, reg_default { reg: 0x5b, def: 0x005f },
    reg_default { reg: 0x5c, def: 0x0000 }, reg_default { reg: 0x5d, def: 0x1001 },
    reg_default { reg: 0x5e, def: 0x1000 }, reg_default { reg: 0x5f, def: 0x0000 },
    reg_default { reg: 0x60, def: 0x5554 }, reg_default { reg: 0x61, def: 0xffc0 },
    reg_default { reg: 0x62, def: 0xa000 }, reg_default { reg: 0x63, def: 0xd010 },
    reg_default { reg: 0x64, def: 0x0000 }, reg_default { reg: 0x65, def: 0x3fb1 },
    reg_default { reg: 0x66, def: 0x1881 }, reg_default { reg: 0x67, def: 0xc810 },
    reg_default { reg: 0x68, def: 0x2000 }, reg_default { reg: 0x69, def: 0xfff0 },
    reg_default { reg: 0x6a, def: 0x0300 }, reg_default { reg: 0x6b, def: 0x5060 },
    reg_default { reg: 0x6c, def: 0x0000 }, reg_default { reg: 0x6d, def: 0x0000 },
    reg_default { reg: 0x6e, def: 0x0c25 }, reg_default { reg: 0x6f, def: 0x0c0b },
    reg_default { reg: 0x70, def: 0x8000 }, reg_default { reg: 0x71, def: 0x4008 },
    reg_default { reg: 0x72, def: 0x0000 }, reg_default { reg: 0x73, def: 0x0800 },
    reg_default { reg: 0x74, def: 0xa28f }, reg_default { reg: 0x75, def: 0xa050 },
    reg_default { reg: 0x76, def: 0x7fe8 }, reg_default { reg: 0x77, def: 0xdb8c },
    reg_default { reg: 0x78, def: 0x0000 }, reg_default { reg: 0x79, def: 0x0000 },
    reg_default { reg: 0x7a, def: 0x2a96 }, reg_default { reg: 0x7b, def: 0x800f },
    reg_default { reg: 0x7c, def: 0x0200 }, reg_default { reg: 0x7d, def: 0x1600 },
    reg_default { reg: 0x7e, def: 0x0000 }, reg_default { reg: 0x7f, def: 0x0000 },
];
const INDEX_CACHE_SIZE: usize = rt274_index_def.len();

const rt274_reg: [reg_default; 33] = [
    reg_default { reg: 0x00170500, def: 0x00000400 }, reg_default { reg: 0x00220000, def: 0x00000031 },
    reg_default { reg: 0x00239000, def: 0x00000057 }, reg_default { reg: 0x0023a000, def: 0x00000057 },
    reg_default { reg: 0x00270500, def: 0x00000400 }, reg_default { reg: 0x00370500, def: 0x00000400 },
    reg_default { reg: 0x00830000, def: 0x00000097 }, reg_default { reg: 0x00870500, def: 0x00000400 },
    reg_default { reg: 0x00920000, def: 0x00000031 }, reg_default { reg: 0x00930000, def: 0x00000097 },
    reg_default { reg: 0x00935000, def: 0x00000097 }, reg_default { reg: 0x00936000, def: 0x00000097 },
    reg_default { reg: 0x00970500, def: 0x00000400 }, reg_default { reg: 0x00b37000, def: 0x00000400 },
    reg_default { reg: 0x00b37200, def: 0x00000400 }, reg_default { reg: 0x00b37300, def: 0x00000400 },
    reg_default { reg: 0x00c37000, def: 0x00000400 }, reg_default { reg: 0x00c37100, def: 0x00000400 },
    reg_default { reg: 0x01270500, def: 0x00000400 }, reg_default { reg: 0x01270700, def: 0x00000000 },
    reg_default { reg: 0x01370500, def: 0x00000400 }, reg_default { reg: 0x01371f00, def: 0x411111f0 },
    reg_default { reg: 0x01937000, def: 0x00000000 }, reg_default { reg: 0x01970500, def: 0x00000400 },
    reg_default { reg: 0x01970700, def: 0x00000020 }, reg_default { reg: 0x02050000, def: 0x0000001b },
    reg_default { reg: 0x02139000, def: 0x00000080 }, reg_default { reg: 0x0213a000, def: 0x00000080 },
    reg_default { reg: 0x02170100, def: 0x00000001 }, reg_default { reg: 0x02170500, def: 0x00000400 },
    reg_default { reg: 0x02170700, def: 0x00000000 }, reg_default { reg: 0x02270100, def: 0x00000000 },
    reg_default { reg: 0x02370100, def: 0x00000000 },
];

unsafe fn rt274_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg <= 0xff { return true; }
    reg == RT274_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID)
        || reg == RT274_GET_HP_SENSE || reg == RT274_GET_MIC_SENSE || reg == RT274_PROC_COEF
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT274_MIC, 0)
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT274_HP_OUT, 0)
        || reg == VERB_CMD(AC_VERB_GET_STREAM_FORMAT, RT274_DAC_OUT0, 0)
        || reg == VERB_CMD(AC_VERB_GET_STREAM_FORMAT, RT274_DAC_OUT1, 0)
        || reg == VERB_CMD(AC_VERB_GET_STREAM_FORMAT, RT274_ADC_IN1, 0)
        || reg == VERB_CMD(AC_VERB_GET_STREAM_FORMAT, RT274_ADC_IN2, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_DAC_OUT0, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_DAC_OUT1, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_ADC_IN1, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_ADC_IN2, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_DMIC1, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_DMIC2, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_MIC, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_LINE1, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_LINE2, 0)
        || reg == VERB_CMD(AC_VERB_GET_AMP_GAIN_MUTE, RT274_HP_OUT, 0)
        || reg == VERB_CMD(AC_VERB_GET_CONNECT_SEL, RT274_HP_OUT, 0)
        || reg == VERB_CMD(AC_VERB_GET_CONNECT_SEL, RT274_MIXER_IN1, 0)
        || reg == VERB_CMD(AC_VERB_GET_CONNECT_SEL, RT274_MIXER_IN2, 0)
        || reg == VERB_CMD(AC_VERB_GET_PIN_WIDGET_CONTROL, RT274_DMIC1, 0)
        || reg == VERB_CMD(AC_VERB_GET_PIN_WIDGET_CONTROL, RT274_DMIC2, 0)
        || reg == VERB_CMD(AC_VERB_GET_PIN_WIDGET_CONTROL, RT274_MIC, 0)
        || reg == VERB_CMD(AC_VERB_GET_PIN_WIDGET_CONTROL, RT274_LINE1, 0)
        || reg == VERB_CMD(AC_VERB_GET_PIN_WIDGET_CONTROL, RT274_LINE2, 0)
        || reg == VERB_CMD(AC_VERB_GET_PIN_WIDGET_CONTROL, RT274_HP_OUT, 0)
        || reg == VERB_CMD(AC_VERB_GET_UNSOLICITED_RESPONSE, RT274_HP_OUT, 0)
        || reg == VERB_CMD(AC_VERB_GET_UNSOLICITED_RESPONSE, RT274_MIC, 0)
        || reg == VERB_CMD(AC_VERB_GET_UNSOLICITED_RESPONSE, RT274_INLINE_CMD, 0)
}

unsafe fn rt274_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg <= 0xff { return true; }
    rt274_volatile_register(_dev, reg)
        || reg == RT274_SET_AUDIO_POWER || reg == RT274_SET_HPO_POWER
        || reg == RT274_SET_DMIC1_POWER || reg == RT274_LOUT_MUX
        || reg == RT274_HPO_MUX || reg == RT274_ADC0_MUX || reg == RT274_ADC1_MUX
        || reg == RT274_SET_MIC || reg == RT274_SET_PIN_HPO || reg == RT274_SET_PIN_LOUT3
        || reg == RT274_SET_PIN_DMIC1 || reg == RT274_SET_AMP_GAIN_HPO
        || reg == RT274_SET_DMIC2_DEFAULT || reg == RT274_DAC0L_GAIN
        || reg == RT274_DAC0R_GAIN || reg == RT274_DAC1L_GAIN
        || reg == RT274_DAC1R_GAIN || reg == RT274_ADCL_GAIN || reg == RT274_ADCR_GAIN
        || reg == RT274_MIC_GAIN || reg == RT274_HPOL_GAIN || reg == RT274_HPOR_GAIN
        || reg == RT274_LOUTL_GAIN || reg == RT274_LOUTR_GAIN || reg == RT274_DAC_FORMAT
        || reg == RT274_ADC_FORMAT || reg == RT274_COEF_INDEX
        || reg == RT274_SET_AMP_GAIN_ADC_IN1 || reg == RT274_SET_AMP_GAIN_ADC_IN2
        || reg == RT274_SET_POWER(RT274_DAC_OUT0) || reg == RT274_SET_POWER(RT274_DAC_OUT1)
        || reg == RT274_SET_POWER(RT274_ADC_IN1) || reg == RT274_SET_POWER(RT274_ADC_IN2)
        || reg == RT274_SET_POWER(RT274_DMIC2) || reg == RT274_SET_POWER(RT274_MIC)
}

/* CONFIG_PM */
unsafe fn rt274_index_sync(component: *mut snd_soc_component) {
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    let mut i = 0;
    while i < INDEX_CACHE_SIZE {
        let entry = (*rt274).index_cache.add(i);
        snd_soc_component_write(component, (*entry).reg, (*entry).def);
        i += 1;
    }
}

unsafe fn rt274_jack_detect(rt274: *mut rt274_priv, hp: *mut bool, mic: *mut bool) -> c_int {
    let mut buf: c_uint = 0;
    *hp = false;
    *mic = false;
    if (*rt274).component.is_null() { return -EINVAL; }
    let mut ret = regmap_read((*rt274).regmap, RT274_GET_HP_SENSE, &mut buf);
    if ret != 0 { return ret; }
    *hp = (buf & 0x80000000) != 0;
    ret = regmap_read((*rt274).regmap, RT274_GET_MIC_SENSE, &mut buf);
    if ret != 0 { return ret; }
    *mic = (buf & 0x80000000) != 0;
    ret
}

unsafe fn rt274_jack_detect_work(_work: *mut work_struct) {
    /* container_of(work, struct rt274_priv, jack_detect_work.work) */
    let rt274 = _work as *mut rt274_priv;
    let mut status = 0;
    let mut hp = false;
    let mut mic = false;
    if rt274_jack_detect(rt274, &mut hp, &mut mic) < 0 { return; }
    if hp { status |= SND_JACK_HEADPHONE as c_int; }
    if mic { status |= SND_JACK_MICROPHONE as c_int; }
    snd_soc_jack_report((*rt274).jack, status,
        (SND_JACK_MICROPHONE | SND_JACK_HEADPHONE) as c_int);
}

unsafe fn rt274_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let rt274 = data as *mut rt274_priv;
    let mut hp = false;
    let mut mic = false;
    let mut status = 0;
    regmap_update_bits((*rt274).regmap, RT274_EAPD_GPIO_IRQ_CTRL, RT274_IRQ_CLR, RT274_IRQ_CLR);
    let ret = rt274_jack_detect(rt274, &mut hp, &mut mic);
    if ret == 0 {
        if hp { status |= SND_JACK_HEADPHONE as c_int; }
        if mic { status |= SND_JACK_MICROPHONE as c_int; }
        snd_soc_jack_report((*rt274).jack, status,
            (SND_JACK_MICROPHONE | SND_JACK_HEADPHONE) as c_int);
        /* pm_wakeup_event(&rt274->i2c->dev, 300); */
    }
    IRQ_HANDLED as irqreturn_t
}

unsafe fn rt274_mic_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    (*rt274).jack = jack;
    if jack.is_null() {
        /* Disable jack detection */
        regmap_update_bits((*rt274).regmap, RT274_EAPD_GPIO_IRQ_CTRL, RT274_IRQ_EN, RT274_IRQ_DIS);
        return 0;
    }
    regmap_update_bits((*rt274).regmap, RT274_EAPD_GPIO_IRQ_CTRL, RT274_IRQ_EN, RT274_IRQ_EN);
    /* Send an initial report */
    rt274_irq(0, rt274 as *mut c_void);
    0
}

/* TLV controls, kcontrols, DAPM widgets and routes are macro-defined ALSA data.
 * Source-level equivalents:
 * DECLARE_TLV_DB_SCALE(out_vol_tlv, -6350, 50, 0);
 * DECLARE_TLV_DB_SCALE(mic_vol_tlv, 0, 1000, 0);
 * rt274_snd_controls[], hpol/hpor/loutl/loutr controls,
 * rt274_adc_src[], rt274_adc0_enum, rt274_adc0_mux, rt274_adc1_enum,
 * rt274_adc1_mux, rt274_dac_src[], rt274_hpo_enum, rt274_hpo_mux,
 * rt274_lout_enum, rt274_lout_mux, rt274_dapm_widgets[], rt274_dapm_routes[].
 */
const rt274_adc_src: [&[u8]; 4] = [b"Mic\0", b"Line1\0", b"Line2\0", b"Dmic\0"];
const rt274_dac_src: [&[u8]; 2] = [b"DAC OUT0\0", b"DAC OUT1\0"];

unsafe fn rt274_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = dai as *mut snd_soc_component; /* dai->component */
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    let mut val: c_uint = 0;
    let mut d_len_code: c_int = 0;
    let mut c_len_code: c_int = 0;

    match params_rate(params) {
        44100 | 48000 => {}
        _ => return -EINVAL,
    }
    match (*rt274).sys_clk {
        12288000 | 24576000 => {
            if params_rate(params) != 48000 { return -EINVAL; }
        }
        11289600 | 22579200 => {
            if params_rate(params) != 44100 { return -EINVAL; }
        }
        _ => {}
    }
    if params_channels(params) <= 16 {
        val |= (params_channels(params) - 1) as c_uint;
    } else {
        return -EINVAL;
    }
    match params_width(params) {
        16 => { d_len_code = 0; c_len_code = 0; val |= 0x1 << 4; }
        32 => { d_len_code = 2; c_len_code = 3; val |= 0x4 << 4; }
        20 => { d_len_code = 1; c_len_code = 1; val |= 0x2 << 4; }
        24 => { d_len_code = 2; c_len_code = 2; val |= 0x3 << 4; }
        8 => { d_len_code = 3; c_len_code = 0; }
        _ => return -EINVAL,
    }
    if (*rt274).master { c_len_code = 0x3; }
    snd_soc_component_update_bits(component, RT274_I2S_CTRL1, 0xc018,
        ((d_len_code << 3) | (c_len_code << 14)) as c_uint);
    snd_soc_component_update_bits(component, RT274_DAC_FORMAT, 0x407f, val);
    snd_soc_component_update_bits(component, RT274_ADC_FORMAT, 0x407f, val);
    0
}

unsafe fn rt274_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = dai as *mut snd_soc_component; /* dai->component */
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_I2S_MODE_MASK, RT274_I2S_MODE_M);
            (*rt274).master = true;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_I2S_MODE_MASK, RT274_I2S_MODE_S);
            (*rt274).master = false;
        }
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => { snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_I2S_FMT_MASK, RT274_I2S_FMT_I2S); }
        x if x == SND_SOC_DAIFMT_LEFT_J => { snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_I2S_FMT_MASK, RT274_I2S_FMT_LJ); }
        x if x == SND_SOC_DAIFMT_DSP_A => { snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_I2S_FMT_MASK, RT274_I2S_FMT_PCMA); }
        x if x == SND_SOC_DAIFMT_DSP_B => { snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_I2S_FMT_MASK, RT274_I2S_FMT_PCMB); }
        _ => return -EINVAL,
    }
    /* bit 15 Stream Type 0:PCM 1:Non-PCM */
    snd_soc_component_update_bits(component, RT274_DAC_FORMAT, 0x8000, 0);
    snd_soc_component_update_bits(component, RT274_ADC_FORMAT, 0x8000, 0);
    0
}

unsafe fn rt274_set_dai_pll(dai: *mut snd_soc_dai, _pll_id: c_int, source: c_int, _freq_in: c_uint, _freq_out: c_uint) -> c_int {
    let component = dai as *mut snd_soc_component; /* dai->component */
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    match source as c_uint {
        x if x == RT274_PLL2_S_MCLK => {
            snd_soc_component_update_bits(component, RT274_PLL2_CTRL, RT274_PLL2_SRC_MASK, RT274_PLL2_SRC_MCLK);
        }
        _ => {
            snd_soc_component_update_bits(component, RT274_PLL2_CTRL, RT274_PLL2_SRC_MASK, RT274_PLL2_SRC_BCLK);
        }
    }
    if source as c_uint == RT274_PLL2_S_BCLK {
        snd_soc_component_update_bits(component, RT274_MCLK_CTRL, 0x3 << 12, 0x3 << 12);
        match (*rt274).fs {
            50 => { snd_soc_component_write(component, 0x7a, 0xaab6); snd_soc_component_write(component, 0x7b, 0x0301); snd_soc_component_write(component, 0x7c, 0x04fe); }
            64 => { snd_soc_component_write(component, 0x7a, 0xaa96); snd_soc_component_write(component, 0x7b, 0x8003); snd_soc_component_write(component, 0x7c, 0x081e); }
            128 => { snd_soc_component_write(component, 0x7a, 0xaa96); snd_soc_component_write(component, 0x7b, 0x8003); snd_soc_component_write(component, 0x7c, 0x080e); }
            _ | 100 => { snd_soc_component_write(component, 0x7a, 0xaab6); snd_soc_component_write(component, 0x7b, 0x0301); snd_soc_component_write(component, 0x7c, 0x047e); }
        }
    }
    0
}

unsafe fn rt274_set_dai_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = dai as *mut snd_soc_component; /* dai->component */
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    let (mclk_en, clk_src) = match clk_id as c_uint {
        x if x == RT274_SCLK_S_MCLK => (RT274_MCLK_MODE_EN, RT274_CLK_SRC_MCLK),
        x if x == RT274_SCLK_S_PLL1 => (RT274_MCLK_MODE_DIS, RT274_CLK_SRC_MCLK),
        x if x == RT274_SCLK_S_PLL2 => (RT274_MCLK_MODE_EN, RT274_CLK_SRC_PLL2),
        _ => (RT274_MCLK_MODE_DIS, RT274_CLK_SRC_MCLK),
    };
    snd_soc_component_update_bits(component, RT274_MCLK_CTRL, RT274_MCLK_MODE_MASK, mclk_en);
    snd_soc_component_update_bits(component, RT274_CLK_CTRL, RT274_CLK_SRC_MASK, clk_src);
    match freq {
        19200000 => {
            if clk_id as c_uint == RT274_SCLK_S_MCLK { return -EINVAL; }
            snd_soc_component_update_bits(component, RT274_I2S_CTRL2, 0x40, 0x40);
        }
        24000000 => {
            if clk_id as c_uint == RT274_SCLK_S_MCLK { return -EINVAL; }
            snd_soc_component_update_bits(component, RT274_I2S_CTRL2, 0x40, 0x0);
        }
        12288000 | 11289600 => { snd_soc_component_update_bits(component, RT274_MCLK_CTRL, 0x1fcf, 0x0008); }
        24576000 | 22579200 => { snd_soc_component_update_bits(component, RT274_MCLK_CTRL, 0x1fcf, 0x1543); }
        _ => return -EINVAL,
    }
    (*rt274).sys_clk = freq as c_int;
    (*rt274).clk_id = clk_id;
    0
}

unsafe fn rt274_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let component = dai as *mut snd_soc_component;
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    (*rt274).fs = ratio as c_int;
    if (ratio / 50) == 0 {
        snd_soc_component_update_bits(component, RT274_I2S_CTRL1, 0x1000, 0x1000);
    } else {
        snd_soc_component_update_bits(component, RT274_I2S_CTRL1, 0x1000, 0);
    }
    0
}

unsafe fn rt274_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, _slot_width: c_int) -> c_int {
    let component = dai as *mut snd_soc_component;
    if rx_mask != 0 || tx_mask != 0 {
        snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_TDM_EN, RT274_TDM_EN);
    } else {
        snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_TDM_EN, RT274_TDM_DIS);
        return 0;
    }
    match slots {
        4 => { snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_TDM_CH_NUM, RT274_TDM_4CH); }
        2 => { snd_soc_component_update_bits(component, RT274_I2S_CTRL1, RT274_TDM_CH_NUM, RT274_TDM_2CH); }
        _ => return -EINVAL,
    }
    0
}

unsafe fn rt274_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    match level as c_uint {
        x if x == SND_SOC_BIAS_PREPARE => {
            if SND_SOC_BIAS_STANDBY as snd_soc_bias_level == snd_soc_dapm_get_bias_level(dapm) {
                snd_soc_component_write(component, RT274_SET_AUDIO_POWER, AC_PWRST_D0);
            }
        }
        x if x == SND_SOC_BIAS_STANDBY => {
            snd_soc_component_write(component, RT274_SET_AUDIO_POWER, AC_PWRST_D3);
        }
        _ => {}
    }
    0
}

unsafe fn rt274_probe(component: *mut snd_soc_component) -> c_int {
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    (*rt274).component = component;
    /* INIT_DELAYED_WORK(&rt274->jack_detect_work, rt274_jack_detect_work);
     * if (rt274->i2c->irq) schedule_delayed_work(..., msecs_to_jiffies(1250));
     */
    0
}

unsafe fn rt274_remove(component: *mut snd_soc_component) {
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    /* cancel_delayed_work_sync(&rt274->jack_detect_work); */
    (*rt274).component = ptr::null_mut();
}

unsafe fn rt274_suspend(component: *mut snd_soc_component) -> c_int {
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    regcache_cache_only((*rt274).regmap, true);
    regcache_mark_dirty((*rt274).regmap);
    0
}

unsafe fn rt274_resume(component: *mut snd_soc_component) -> c_int {
    let rt274 = snd_soc_component_get_drvdata(component) as *mut rt274_priv;
    regcache_cache_only((*rt274).regmap, false);
    rt274_index_sync(component);
    regcache_sync((*rt274).regmap);
    0
}

const RT274_STEREO_RATES: c_uint = 0; /* SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 */
const RT274_FORMATS: c_uint = 0; /* S16_LE | S20_3LE | S24_LE | S8 */

/* Macro-defined static driver data preserved by name:
 * rt274_aif_dai_ops, rt274_dai[], soc_component_dev_rt274, rt274_regmap,
 * rt274_of_match[], rt274_i2c_id[], rt274_acpi_match[].
 */

unsafe fn rt274_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let rt274: *mut rt274_priv;
    let mut ret: c_int;
    let mut val: c_uint = 0;

    /* rt274 = devm_kzalloc(&i2c->dev, sizeof(*rt274), GFP_KERNEL); */
    rt274 = ptr::null_mut();
    if rt274.is_null() { return -ENOMEM; }

    /* rt274->regmap = devm_regmap_init(&i2c->dev, NULL, i2c, &rt274_regmap); */
    ret = regmap_read((*rt274).regmap, RT274_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID), &mut val);
    if ret != 0 { return ret; }
    if val != RT274_VENDOR_ID { return -ENODEV; }

    /* rt274->index_cache = devm_kmemdup(&i2c->dev, rt274_index_def,
     * sizeof(rt274_index_def), GFP_KERNEL);
     */
    if (*rt274).index_cache.is_null() { return -ENOMEM; }
    (*rt274).index_cache_size = INDEX_CACHE_SIZE as c_int;
    (*rt274).i2c = i2c;
    /* i2c_set_clientdata(i2c, rt274); */

    /* reset codec */
    regmap_write((*rt274).regmap, RT274_RESET, 0);
    regmap_update_bits((*rt274).regmap, 0x1a, 0x4000, 0x4000);

    /* Set Pad PDB is floating */
    regmap_update_bits((*rt274).regmap, RT274_PAD_CTRL12, 0x3, 0x0);
    regmap_write((*rt274).regmap, RT274_COEF5b_INDEX, 0x01);
    regmap_write((*rt274).regmap, RT274_COEF5b_COEF, 0x8540);
    regmap_update_bits((*rt274).regmap, 0x6f, 0x0100, 0x0100);
    /* Combo jack auto detect */
    regmap_write((*rt274).regmap, 0x4a, 0x201b);
    /* Aux mode off */
    regmap_update_bits((*rt274).regmap, 0x6f, 0x3000, 0x2000);
    /* HP DC Calibration */
    regmap_update_bits((*rt274).regmap, 0x6f, 0xf, 0x0);
    /* Set NID=58h.Index 00h [15]= 1b; */
    regmap_write((*rt274).regmap, RT274_COEF58_INDEX, 0x00);
    regmap_write((*rt274).regmap, RT274_COEF58_COEF, 0xb888);
    /* msleep(500); */
    regmap_update_bits((*rt274).regmap, 0x6f, 0xf, 0xb);
    regmap_write((*rt274).regmap, RT274_COEF58_INDEX, 0x00);
    regmap_write((*rt274).regmap, RT274_COEF58_COEF, 0x3888);
    /* Set pin widget */
    regmap_write((*rt274).regmap, RT274_SET_PIN_HPO, 0x40);
    regmap_write((*rt274).regmap, RT274_SET_PIN_LOUT3, 0x40);
    regmap_write((*rt274).regmap, RT274_SET_MIC, 0x20);
    regmap_write((*rt274).regmap, RT274_SET_PIN_DMIC1, 0x20);

    regmap_update_bits((*rt274).regmap, RT274_I2S_CTRL2, 0xc004, 0x4004);
    regmap_update_bits((*rt274).regmap, RT274_EAPD_GPIO_IRQ_CTRL,
        RT274_GPI2_SEL_MASK, RT274_GPI2_SEL_DMIC_CLK);

    /* jack detection */
    regmap_write((*rt274).regmap, RT274_UNSOLICITED_HP_OUT, 0x81);
    regmap_write((*rt274).regmap, RT274_UNSOLICITED_MIC, 0x82);

    /* if (rt274->i2c->irq) devm_request_threaded_irq(..., rt274_irq, ...); */
    /* ret = devm_snd_soc_register_component(&i2c->dev, &soc_component_dev_rt274,
     *                                      rt274_dai, ARRAY_SIZE(rt274_dai));
     */
    ret = 0;
    ret
}

/* module_i2c_driver(rt274_i2c_driver);
 * MODULE_DESCRIPTION("ASoC RT274 driver");
 * MODULE_AUTHOR("Bard Liao <bardliao@realtek.com>");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
