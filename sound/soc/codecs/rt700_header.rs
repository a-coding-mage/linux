/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rt700.h -- RT700 ALSA SoC audio driver header
 *
 * Copyright(c) 2019 Realtek Semiconductor Corp.
 */

// C header guard removed: __RT700_H__

unsafe extern "C" {
    pub static rt700_runtime_pm: dev_pm_ops;
}

#[repr(C)]
pub struct rt700_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub jack_type: core::ffi::c_int,
    pub disable_irq_lock: mutex, /* imp-def irq lock protection */
    pub disable_irq: bool,
}

/* NID */
pub const RT700_AUDIO_FUNCTION_GROUP: u32 = 0x01;
pub const RT700_DAC_OUT1: u32 = 0x02;
pub const RT700_DAC_OUT2: u32 = 0x03;
pub const RT700_ADC_IN1: u32 = 0x09;
pub const RT700_ADC_IN2: u32 = 0x08;
pub const RT700_DMIC1: u32 = 0x12;
pub const RT700_DMIC2: u32 = 0x13;
pub const RT700_SPK_OUT: u32 = 0x14;
pub const RT700_MIC2: u32 = 0x19;
pub const RT700_LINE1: u32 = 0x1a;
pub const RT700_LINE2: u32 = 0x1b;
pub const RT700_BEEP: u32 = 0x1d;
pub const RT700_SPDIF: u32 = 0x1e;
pub const RT700_VENDOR_REGISTERS: u32 = 0x20;
pub const RT700_HP_OUT: u32 = 0x21;
pub const RT700_MIXER_IN1: u32 = 0x22;
pub const RT700_MIXER_IN2: u32 = 0x23;
pub const RT700_INLINE_CMD: u32 = 0x55;

/* Index (NID:20h) */
pub const RT700_DAC_DC_CALI_CTL1: u32 = 0x00;
pub const RT700_PARA_VERB_CTL: u32 = 0x1a;
pub const RT700_COMBO_JACK_AUTO_CTL1: u32 = 0x45;
pub const RT700_COMBO_JACK_AUTO_CTL2: u32 = 0x46;
pub const RT700_INLINE_CMD_CTL: u32 = 0x48;
pub const RT700_DIGITAL_MISC_CTRL4: u32 = 0x4a;
pub const RT700_VREFOUT_CTL: u32 = 0x6b;
pub const RT700_FSM_CTL: u32 = 0x6f;
pub const RT700_IRQ_FLAG_TABLE1: u32 = 0x80;
pub const RT700_IRQ_FLAG_TABLE2: u32 = 0x81;
pub const RT700_IRQ_FLAG_TABLE3: u32 = 0x82;

/* Verb */
pub const RT700_VERB_SET_CONNECT_SEL: u32 = 0x3100;
pub const RT700_VERB_SET_EAPD_BTLENABLE: u32 = 0x3c00;
pub const RT700_VERB_GET_CONNECT_SEL: u32 = 0xb100;
pub const RT700_VERB_SET_POWER_STATE: u32 = 0x3500;
pub const RT700_VERB_SET_CHANNEL_STREAMID: u32 = 0x3600;
pub const RT700_VERB_SET_PIN_WIDGET_CONTROL: u32 = 0x3700;
pub const RT700_VERB_SET_UNSOLICITED_ENABLE: u32 = 0x3800;
pub const RT700_SET_AMP_GAIN_MUTE_H: u32 = 0x7300;
pub const RT700_SET_AMP_GAIN_MUTE_L: u32 = 0x8380;
pub const RT700_VERB_GET_PIN_SENSE: u32 = 0xb900;

pub const RT700_READ_HDA_3: u32 = 0x2012;
pub const RT700_READ_HDA_2: u32 = 0x2013;
pub const RT700_READ_HDA_1: u32 = 0x2014;
pub const RT700_READ_HDA_0: u32 = 0x2015;
pub const RT700_PRIV_INDEX_W_H: u32 = 0x7520;
pub const RT700_PRIV_INDEX_W_L: u32 = 0x85a0;
pub const RT700_PRIV_DATA_W_H: u32 = 0x7420;
pub const RT700_PRIV_DATA_W_L: u32 = 0x84a0;
pub const RT700_PRIV_INDEX_R_H: u32 = 0x9d20;
pub const RT700_PRIV_INDEX_R_L: u32 = 0xada0;
pub const RT700_PRIV_DATA_R_H: u32 = 0x9c20;
pub const RT700_PRIV_DATA_R_L: u32 = 0xaca0;
pub const RT700_DAC_FORMAT_H: u32 = 0x7203;
pub const RT700_DAC_FORMAT_L: u32 = 0x8283;
pub const RT700_ADC_FORMAT_H: u32 = 0x7209;
pub const RT700_ADC_FORMAT_L: u32 = 0x8289;
pub const RT700_SET_AUDIO_POWER_STATE: u32 =
    RT700_VERB_SET_POWER_STATE | RT700_AUDIO_FUNCTION_GROUP;
pub const RT700_SET_PIN_DMIC1: u32 = RT700_VERB_SET_PIN_WIDGET_CONTROL | RT700_DMIC1;
pub const RT700_SET_PIN_DMIC2: u32 = RT700_VERB_SET_PIN_WIDGET_CONTROL | RT700_DMIC2;
pub const RT700_SET_PIN_SPK: u32 = RT700_VERB_SET_PIN_WIDGET_CONTROL | RT700_SPK_OUT;
pub const RT700_SET_PIN_HP: u32 = RT700_VERB_SET_PIN_WIDGET_CONTROL | RT700_HP_OUT;
pub const RT700_SET_PIN_MIC2: u32 = RT700_VERB_SET_PIN_WIDGET_CONTROL | RT700_MIC2;
pub const RT700_SET_PIN_LINE1: u32 = RT700_VERB_SET_PIN_WIDGET_CONTROL | RT700_LINE1;
pub const RT700_SET_PIN_LINE2: u32 = RT700_VERB_SET_PIN_WIDGET_CONTROL | RT700_LINE2;
pub const RT700_SET_MIC2_UNSOLICITED_ENABLE: u32 =
    RT700_VERB_SET_UNSOLICITED_ENABLE | RT700_MIC2;
pub const RT700_SET_HP_UNSOLICITED_ENABLE: u32 =
    RT700_VERB_SET_UNSOLICITED_ENABLE | RT700_HP_OUT;
pub const RT700_SET_INLINE_UNSOLICITED_ENABLE: u32 =
    RT700_VERB_SET_UNSOLICITED_ENABLE | RT700_INLINE_CMD;
pub const RT700_SET_STREAMID_DAC1: u32 = RT700_VERB_SET_CHANNEL_STREAMID | RT700_DAC_OUT1;
pub const RT700_SET_STREAMID_DAC2: u32 = RT700_VERB_SET_CHANNEL_STREAMID | RT700_DAC_OUT2;
pub const RT700_SET_STREAMID_ADC1: u32 = RT700_VERB_SET_CHANNEL_STREAMID | RT700_ADC_IN1;
pub const RT700_SET_STREAMID_ADC2: u32 = RT700_VERB_SET_CHANNEL_STREAMID | RT700_ADC_IN2;
pub const RT700_SET_GAIN_DAC1_L: u32 = RT700_SET_AMP_GAIN_MUTE_L | RT700_DAC_OUT1;
pub const RT700_SET_GAIN_DAC1_H: u32 = RT700_SET_AMP_GAIN_MUTE_H | RT700_DAC_OUT1;
pub const RT700_SET_GAIN_ADC1_L: u32 = RT700_SET_AMP_GAIN_MUTE_L | RT700_ADC_IN1;
pub const RT700_SET_GAIN_ADC1_H: u32 = RT700_SET_AMP_GAIN_MUTE_H | RT700_ADC_IN1;
pub const RT700_SET_GAIN_ADC2_L: u32 = RT700_SET_AMP_GAIN_MUTE_L | RT700_ADC_IN2;
pub const RT700_SET_GAIN_ADC2_H: u32 = RT700_SET_AMP_GAIN_MUTE_H | RT700_ADC_IN2;
pub const RT700_SET_GAIN_AMIC_L: u32 = RT700_SET_AMP_GAIN_MUTE_L | RT700_MIC2;
pub const RT700_SET_GAIN_AMIC_H: u32 = RT700_SET_AMP_GAIN_MUTE_H | RT700_MIC2;
pub const RT700_SET_GAIN_HP_L: u32 = RT700_SET_AMP_GAIN_MUTE_L | RT700_HP_OUT;
pub const RT700_SET_GAIN_HP_H: u32 = RT700_SET_AMP_GAIN_MUTE_H | RT700_HP_OUT;
pub const RT700_SET_GAIN_SPK_L: u32 = RT700_SET_AMP_GAIN_MUTE_L | RT700_SPK_OUT;
pub const RT700_SET_GAIN_SPK_H: u32 = RT700_SET_AMP_GAIN_MUTE_H | RT700_SPK_OUT;
pub const RT700_SET_EAPD_SPK: u32 = RT700_VERB_SET_EAPD_BTLENABLE | RT700_SPK_OUT;

/* combo jack auto switch control 2 (0x46)(NID:20h) */
pub const RT700_COMBOJACK_AUTO_DET_STATUS: u32 = 0x1 << 11;
pub const RT700_COMBOJACK_AUTO_DET_TRS: u32 = 0x1 << 10;
pub const RT700_COMBOJACK_AUTO_DET_CTIA: u32 = 0x1 << 9;
pub const RT700_COMBOJACK_AUTO_DET_OMTP: u32 = 0x1 << 8;

pub const RT700_EAPD_HIGH: u32 = 0x2;
pub const RT700_EAPD_LOW: u32 = 0x0;
pub const RT700_MUTE_SFT: u32 = 7;
pub const RT700_DIR_IN_SFT: u32 = 6;
pub const RT700_DIR_OUT_SFT: u32 = 7;

pub const RT700_AIF1: u32 = 0;
pub const RT700_AIF2: u32 = 1;
pub const RT700_AIFS: u32 = 2;

unsafe extern "C" {
    pub fn rt700_io_init(dev: *mut device, slave: *mut sdw_slave) -> core::ffi::c_int;
    pub fn rt700_init(
        dev: *mut device,
        sdw_regmap: *mut regmap,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> core::ffi::c_int;

    pub fn rt700_jack_detect(
        rt700: *mut rt700_priv,
        hp: *mut bool,
        mic: *mut bool,
    ) -> core::ffi::c_int;
    pub fn rt700_clock_config(dev: *mut device) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
