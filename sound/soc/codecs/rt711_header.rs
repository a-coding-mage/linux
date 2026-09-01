/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rt711.h -- RT711 ALSA SoC audio driver header
 *
 * Copyright(c) 2019 Realtek Semiconductor Corp.
 */

// C header guard and include syntax omitted in Rust.

unsafe extern "C" {
    pub static rt711_runtime_pm: dev_pm_ops;
}

#[repr(C)]
pub struct rt711_priv {
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibration_work: work_struct,
    pub calibrate_mutex: mutex, /* for headset calibration */
    pub jack_type: ::core::ffi::c_int,
    pub jd_src: ::core::ffi::c_int,
    pub disable_irq_lock: mutex, /* imp-def irq lock protection */
    pub disable_irq: bool,
}

/* NID */
pub const RT711_AUDIO_FUNCTION_GROUP: u32 = 0x01;
pub const RT711_DAC_OUT2: u32 = 0x03;
pub const RT711_ADC_IN1: u32 = 0x09;
pub const RT711_ADC_IN2: u32 = 0x08;
pub const RT711_DMIC1: u32 = 0x12;
pub const RT711_DMIC2: u32 = 0x13;
pub const RT711_MIC2: u32 = 0x19;
pub const RT711_LINE1: u32 = 0x1a;
pub const RT711_LINE2: u32 = 0x1b;
pub const RT711_BEEP: u32 = 0x1d;
pub const RT711_VENDOR_REG: u32 = 0x20;
pub const RT711_HP_OUT: u32 = 0x21;
pub const RT711_MIXER_IN1: u32 = 0x22;
pub const RT711_MIXER_IN2: u32 = 0x23;
pub const RT711_INLINE_CMD: u32 = 0x55;
pub const RT711_VENDOR_CALI: u32 = 0x58;
pub const RT711_VENDOR_IMS_DRE: u32 = 0x5b;

/* Index (NID:20h) */
pub const RT711_DAC_DC_CALI_CTL1: u32 = 0x00;
pub const RT711_JD_CTL1: u32 = 0x08;
pub const RT711_JD_CTL2: u32 = 0x09;
pub const RT711_JD_CTL4: u32 = 0x0b;
pub const RT711_CC_DET1: u32 = 0x11;
pub const RT711_PARA_VERB_CTL: u32 = 0x1a;
pub const RT711_COMBO_JACK_AUTO_CTL1: u32 = 0x45;
pub const RT711_COMBO_JACK_AUTO_CTL2: u32 = 0x46;
pub const RT711_INLINE_CMD_CTL: u32 = 0x48;
pub const RT711_DIGITAL_MISC_CTRL4: u32 = 0x4a;
pub const RT711_VREFOUT_CTL: u32 = 0x6b;
pub const RT711_FSM_CTL: u32 = 0x6f;
pub const RT711_IRQ_FLAG_TABLE1: u32 = 0x80;
pub const RT711_IRQ_FLAG_TABLE2: u32 = 0x81;
pub const RT711_IRQ_FLAG_TABLE3: u32 = 0x82;
pub const RT711_TX_RX_MUX_CTL: u32 = 0x91;

/* Index (NID:5bh) */
pub const RT711_IMS_DIGITAL_CTL1: u32 = 0x00;
pub const RT711_HP_IMS_RESULT_L: u32 = 0x20;
pub const RT711_HP_IMS_RESULT_R: u32 = 0x21;

/* Verb */
pub const RT711_VERB_SET_CONNECT_SEL: u32 = 0x3100;
pub const RT711_VERB_SET_EAPD_BTLENABLE: u32 = 0x3c00;
pub const RT711_VERB_GET_CONNECT_SEL: u32 = 0xb100;
pub const RT711_VERB_SET_POWER_STATE: u32 = 0x3500;
pub const RT711_VERB_SET_CHANNEL_STREAMID: u32 = 0x3600;
pub const RT711_VERB_SET_PIN_WIDGET_CONTROL: u32 = 0x3700;
pub const RT711_VERB_SET_UNSOLICITED_ENABLE: u32 = 0x3800;
pub const RT711_SET_AMP_GAIN_MUTE_H: u32 = 0x7300;
pub const RT711_SET_AMP_GAIN_MUTE_L: u32 = 0x8380;
pub const RT711_VERB_GET_POWER_STATE: u32 = 0xb500;
pub const RT711_VERB_GET_CHANNEL_STREAMID: u32 = 0xb600;
pub const RT711_VERB_GET_PIN_SENSE: u32 = 0xb900;
pub const RT711_FUNC_RESET: u32 = 0xff01;

pub const RT711_READ_HDA_3: u32 = 0x2012;
pub const RT711_READ_HDA_2: u32 = 0x2013;
pub const RT711_READ_HDA_1: u32 = 0x2014;
pub const RT711_READ_HDA_0: u32 = 0x2015;
pub const RT711_PRIV_INDEX_W_H: u32 = 0x7500;
pub const RT711_PRIV_INDEX_W_L: u32 = 0x8580;
pub const RT711_PRIV_DATA_W_H: u32 = 0x7400;
pub const RT711_PRIV_DATA_W_L: u32 = 0x8480;
pub const RT711_PRIV_INDEX_R_H: u32 = 0x9d00;
pub const RT711_PRIV_INDEX_R_L: u32 = 0xad80;
pub const RT711_PRIV_DATA_R_H: u32 = 0x9c00;
pub const RT711_PRIV_DATA_R_L: u32 = 0xac80;
pub const RT711_DAC_FORMAT_H: u32 = 0x7203;
pub const RT711_DAC_FORMAT_L: u32 = 0x8283;
pub const RT711_ADC1_FORMAT_H: u32 = 0x7209;
pub const RT711_ADC1_FORMAT_L: u32 = 0x8289;
pub const RT711_ADC2_FORMAT_H: u32 = 0x7208;
pub const RT711_ADC2_FORMAT_L: u32 = 0x8288;

pub const RT711_SET_AUDIO_POWER_STATE: u32 =
    RT711_VERB_SET_POWER_STATE | RT711_AUDIO_FUNCTION_GROUP;
pub const RT711_GET_AUDIO_POWER_STATE: u32 =
    RT711_VERB_GET_POWER_STATE | RT711_AUDIO_FUNCTION_GROUP;
pub const RT711_SET_PIN_DMIC1: u32 = RT711_VERB_SET_PIN_WIDGET_CONTROL | RT711_DMIC1;
pub const RT711_SET_PIN_DMIC2: u32 = RT711_VERB_SET_PIN_WIDGET_CONTROL | RT711_DMIC2;
pub const RT711_SET_PIN_HP: u32 = RT711_VERB_SET_PIN_WIDGET_CONTROL | RT711_HP_OUT;
pub const RT711_SET_PIN_MIC2: u32 = RT711_VERB_SET_PIN_WIDGET_CONTROL | RT711_MIC2;
pub const RT711_SET_PIN_LINE1: u32 = RT711_VERB_SET_PIN_WIDGET_CONTROL | RT711_LINE1;
pub const RT711_SET_PIN_LINE2: u32 = RT711_VERB_SET_PIN_WIDGET_CONTROL | RT711_LINE2;
pub const RT711_SET_MIC2_UNSOLICITED_ENABLE: u32 =
    RT711_VERB_SET_UNSOLICITED_ENABLE | RT711_MIC2;
pub const RT711_SET_HP_UNSOLICITED_ENABLE: u32 =
    RT711_VERB_SET_UNSOLICITED_ENABLE | RT711_HP_OUT;
pub const RT711_SET_INLINE_UNSOLICITED_ENABLE: u32 =
    RT711_VERB_SET_UNSOLICITED_ENABLE | RT711_INLINE_CMD;
pub const RT711_SET_STREAMID_DAC2: u32 = RT711_VERB_SET_CHANNEL_STREAMID | RT711_DAC_OUT2;
pub const RT711_SET_STREAMID_ADC1: u32 = RT711_VERB_SET_CHANNEL_STREAMID | RT711_ADC_IN1;
pub const RT711_SET_STREAMID_ADC2: u32 = RT711_VERB_SET_CHANNEL_STREAMID | RT711_ADC_IN2;
pub const RT711_GET_STREAMID_DAC2: u32 = RT711_VERB_GET_CHANNEL_STREAMID | RT711_DAC_OUT2;
pub const RT711_GET_STREAMID_ADC1: u32 = RT711_VERB_GET_CHANNEL_STREAMID | RT711_ADC_IN1;
pub const RT711_GET_STREAMID_ADC2: u32 = RT711_VERB_GET_CHANNEL_STREAMID | RT711_ADC_IN2;
pub const RT711_SET_GAIN_DAC2_L: u32 = RT711_SET_AMP_GAIN_MUTE_L | RT711_DAC_OUT2;
pub const RT711_SET_GAIN_DAC2_H: u32 = RT711_SET_AMP_GAIN_MUTE_H | RT711_DAC_OUT2;
pub const RT711_SET_GAIN_ADC1_L: u32 = RT711_SET_AMP_GAIN_MUTE_L | RT711_ADC_IN1;
pub const RT711_SET_GAIN_ADC1_H: u32 = RT711_SET_AMP_GAIN_MUTE_H | RT711_ADC_IN1;
pub const RT711_SET_GAIN_ADC2_L: u32 = RT711_SET_AMP_GAIN_MUTE_L | RT711_ADC_IN2;
pub const RT711_SET_GAIN_ADC2_H: u32 = RT711_SET_AMP_GAIN_MUTE_H | RT711_ADC_IN2;
pub const RT711_SET_GAIN_AMIC_L: u32 = RT711_SET_AMP_GAIN_MUTE_L | RT711_MIC2;
pub const RT711_SET_GAIN_AMIC_H: u32 = RT711_SET_AMP_GAIN_MUTE_H | RT711_MIC2;
pub const RT711_SET_GAIN_DMIC1_L: u32 = RT711_SET_AMP_GAIN_MUTE_L | RT711_DMIC1;
pub const RT711_SET_GAIN_DMIC1_H: u32 = RT711_SET_AMP_GAIN_MUTE_H | RT711_DMIC1;
pub const RT711_SET_GAIN_DMIC2_L: u32 = RT711_SET_AMP_GAIN_MUTE_L | RT711_DMIC2;
pub const RT711_SET_GAIN_DMIC2_H: u32 = RT711_SET_AMP_GAIN_MUTE_H | RT711_DMIC2;
pub const RT711_SET_GAIN_HP_L: u32 = RT711_SET_AMP_GAIN_MUTE_L | RT711_HP_OUT;
pub const RT711_SET_GAIN_HP_H: u32 = RT711_SET_AMP_GAIN_MUTE_H | RT711_HP_OUT;

/* DAC DC offset calibration control-1 (0x00)(NID:20h) */
pub const RT711_DAC_DC_CALI_TRIGGER: u32 = 0x1 << 15;

/* jack detect control 1 (0x08)(NID:20h) */
pub const RT711_JD2_DIGITAL_JD_MODE_SEL: u32 = 0x1 << 1;
pub const RT711_JD2_1_JD_MODE: u32 = 0x0 << 1;
pub const RT711_JD2_2_JD_MODE: u32 = 0x1 << 1;

/* jack detect control 2 (0x09)(NID:20h) */
pub const RT711_JD2_2PORT_200K_DECODE_HP: u32 = 0x1 << 13;
pub const RT711_JD2_2PORT_100K_DECODE: u32 = 0x1 << 12;
pub const RT711_JD2_2PORT_100K_DECODE_HP: u32 = 0x0 << 12;
pub const RT711_HP_JD_SEL_JD1: u32 = 0x0 << 1;
pub const RT711_HP_JD_SEL_JD2: u32 = 0x1 << 1;
pub const RT711_JD2_1PORT_TYPE_DECODE: u32 = 0x3 << 10;
pub const RT711_JD2_1PORT_JD_LINE2: u32 = 0x0 << 10;
pub const RT711_JD2_1PORT_JD_HP: u32 = 0x1 << 10;
pub const RT711_JD2_1PORT_JD_LINE1: u32 = 0x2 << 10;
pub const RT711_JD1_2PORT_TYPE_100K_DECODE: u32 = 0x1 << 0;
pub const RT711_JD1_2PORT_JD_RESERVED: u32 = 0x0 << 0;
pub const RT711_JD1_2PORT_JD_LINE1: u32 = 0x1 << 0;

/* jack detect control 4 (0x0b)(NID:20h) */
pub const RT711_JD2_PAD_PULL_UP_MASK: u32 = 0x1 << 3;
pub const RT711_JD2_PAD_NOT_PULL_UP: u32 = 0x0 << 3;
pub const RT711_JD2_PAD_PULL_UP: u32 = 0x1 << 3;
pub const RT711_JD2_MODE_SEL_MASK: u32 = 0x3 << 0;
pub const RT711_JD2_MODE0_2PORT: u32 = 0x0 << 0;
pub const RT711_JD2_MODE1_3P3V_1PORT: u32 = 0x1 << 0;
pub const RT711_JD2_MODE2_1P8V_1PORT: u32 = 0x2 << 0;

/* CC DET1 (0x11)(NID:20h) */
pub const RT711_HP_JD_FINAL_RESULT_CTL_JD12: u32 = 0x1 << 10;
pub const RT711_HP_JD_FINAL_RESULT_CTL_CCDET: u32 = 0x0 << 10;

/* Parameter & Verb control (0x1a)(NID:20h) */
pub const RT711_HIDDEN_REG_SW_RESET: u32 = 0x1 << 14;

/* combo jack auto switch control 2 (0x46)(NID:20h) */
pub const RT711_COMBOJACK_AUTO_DET_STATUS: u32 = 0x1 << 11;
pub const RT711_COMBOJACK_AUTO_DET_TRS: u32 = 0x1 << 10;
pub const RT711_COMBOJACK_AUTO_DET_CTIA: u32 = 0x1 << 9;
pub const RT711_COMBOJACK_AUTO_DET_OMTP: u32 = 0x1 << 8;

/* FSM control (0x6f)(NID:20h) */
pub const RT711_CALI_CTL: u32 = 0x0 << 0;
pub const RT711_COMBOJACK_CTL: u32 = 0x1 << 0;
pub const RT711_IMS_CTL: u32 = 0x2 << 0;
pub const RT711_DEPOP_CTL: u32 = 0x3 << 0;

/* Impedance Sense Digital Control 1 (0x00)(NID:5bh) */
pub const RT711_TRIGGER_IMS: u32 = 0x1 << 15;
pub const RT711_IMS_EN: u32 = 0x1 << 6;

pub const RT711_EAPD_HIGH: u32 = 0x2;
pub const RT711_EAPD_LOW: u32 = 0x0;
pub const RT711_MUTE_SFT: u32 = 7;
/* set input/output mapping to payload[14][15] separately */
pub const RT711_DIR_IN_SFT: u32 = 6;
pub const RT711_DIR_OUT_SFT: u32 = 7;

pub const RT711_AIF1: ::core::ffi::c_int = 0;
pub const RT711_AIF2: ::core::ffi::c_int = 1;
pub const RT711_AIFS: ::core::ffi::c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt711_jd_src {
    RT711_JD_NULL = 0,
    RT711_JD1 = 1,
    RT711_JD2 = 2,
    RT711_JD2_100K = 3,
    RT711_JD2_1P8V_1PORT = 4,
}

unsafe extern "C" {
    pub fn rt711_io_init(dev: *mut device, slave: *mut sdw_slave) -> ::core::ffi::c_int;
    pub fn rt711_init(
        dev: *mut device,
        sdw_regmap: *mut regmap,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> ::core::ffi::c_int;

    pub fn rt711_jack_detect(
        rt711: *mut rt711_priv,
        hp: *mut bool,
        mic: *mut bool,
    ) -> ::core::ffi::c_int;
    pub fn rt711_clock_config(dev: *mut device) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
