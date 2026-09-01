/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rt715.h -- RT715 ALSA SoC audio driver header
 *
 * Copyright(c) 2019 Realtek Semiconductor Corp.
 */

/* C dependency intent: #include <linux/regulator/consumer.h> */

#[repr(C)]
pub struct rt715_priv {
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub codec: *mut snd_soc_codec,
    pub slave: *mut sdw_slave,
    pub dbg_nid: core::ffi::c_int,
    pub dbg_vid: core::ffi::c_int,
    pub dbg_payload: core::ffi::c_int,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub kctl_2ch_vol_ori: [core::ffi::c_uint; 2],
    pub kctl_8ch_switch_ori: [core::ffi::c_uint; 8],
    pub kctl_8ch_vol_ori: [core::ffi::c_uint; 8],
}

/* NID */
pub const RT715_AUDIO_FUNCTION_GROUP: core::ffi::c_uint = 0x01;
pub const RT715_MIC_ADC: core::ffi::c_uint = 0x07;
pub const RT715_LINE_ADC: core::ffi::c_uint = 0x08;
pub const RT715_MIX_ADC: core::ffi::c_uint = 0x09;
pub const RT715_DMIC1: core::ffi::c_uint = 0x12;
pub const RT715_DMIC2: core::ffi::c_uint = 0x13;
pub const RT715_MIC1: core::ffi::c_uint = 0x18;
pub const RT715_MIC2: core::ffi::c_uint = 0x19;
pub const RT715_LINE1: core::ffi::c_uint = 0x1a;
pub const RT715_LINE2: core::ffi::c_uint = 0x1b;
pub const RT715_DMIC3: core::ffi::c_uint = 0x1d;
pub const RT715_DMIC4: core::ffi::c_uint = 0x29;
pub const RT715_VENDOR_REGISTERS: core::ffi::c_uint = 0x20;
pub const RT715_MUX_IN1: core::ffi::c_uint = 0x22;
pub const RT715_MUX_IN2: core::ffi::c_uint = 0x23;
pub const RT715_MUX_IN3: core::ffi::c_uint = 0x24;
pub const RT715_MUX_IN4: core::ffi::c_uint = 0x25;
pub const RT715_MIX_ADC2: core::ffi::c_uint = 0x27;
pub const RT715_INLINE_CMD: core::ffi::c_uint = 0x55;

/* Index (NID:20h) */
pub const RT715_VD_CLEAR_CTRL: core::ffi::c_uint = 0x01;
pub const RT715_SDW_INPUT_SEL: core::ffi::c_uint = 0x39;
pub const RT715_EXT_DMIC_CLK_CTRL2: core::ffi::c_uint = 0x54;

/* Verb */
pub const RT715_VERB_SET_CONNECT_SEL: core::ffi::c_uint = 0x3100;
pub const RT715_VERB_GET_CONNECT_SEL: core::ffi::c_uint = 0xb100;
pub const RT715_VERB_SET_EAPD_BTLENABLE: core::ffi::c_uint = 0x3c00;
pub const RT715_VERB_SET_POWER_STATE: core::ffi::c_uint = 0x3500;
pub const RT715_VERB_SET_CHANNEL_STREAMID: core::ffi::c_uint = 0x3600;
pub const RT715_VERB_SET_PIN_WIDGET_CONTROL: core::ffi::c_uint = 0x3700;
pub const RT715_VERB_SET_CONFIG_DEFAULT1: core::ffi::c_uint = 0x4c00;
pub const RT715_VERB_SET_CONFIG_DEFAULT2: core::ffi::c_uint = 0x4d00;
pub const RT715_VERB_SET_CONFIG_DEFAULT3: core::ffi::c_uint = 0x4e00;
pub const RT715_VERB_SET_CONFIG_DEFAULT4: core::ffi::c_uint = 0x4f00;
pub const RT715_VERB_SET_UNSOLICITED_ENABLE: core::ffi::c_uint = 0x3800;
pub const RT715_SET_AMP_GAIN_MUTE_H: core::ffi::c_uint = 0x7300;
pub const RT715_SET_AMP_GAIN_MUTE_L: core::ffi::c_uint = 0x8380;
pub const RT715_READ_HDA_3: core::ffi::c_uint = 0x2012;
pub const RT715_READ_HDA_2: core::ffi::c_uint = 0x2013;
pub const RT715_READ_HDA_1: core::ffi::c_uint = 0x2014;
pub const RT715_READ_HDA_0: core::ffi::c_uint = 0x2015;
pub const RT715_PRIV_INDEX_W_H: core::ffi::c_uint = 0x7520;
pub const RT715_PRIV_INDEX_W_L: core::ffi::c_uint = 0x85a0;
pub const RT715_PRIV_INDEX_W_H_2: core::ffi::c_uint = 0x7500;
pub const RT715_PRIV_INDEX_W_L_2: core::ffi::c_uint = 0x8580;
pub const RT715_PRIV_DATA_W_H: core::ffi::c_uint = 0x7420;
pub const RT715_PRIV_DATA_W_L: core::ffi::c_uint = 0x84a0;
pub const RT715_PRIV_INDEX_R_H: core::ffi::c_uint = 0x9d20;
pub const RT715_PRIV_INDEX_R_L: core::ffi::c_uint = 0xada0;
pub const RT715_PRIV_DATA_R_H: core::ffi::c_uint = 0x9c20;
pub const RT715_PRIV_DATA_R_L: core::ffi::c_uint = 0xaca0;
pub const RT715_MIC_ADC_FORMAT_H: core::ffi::c_uint = 0x7207;
pub const RT715_MIC_ADC_FORMAT_L: core::ffi::c_uint = 0x8287;
pub const RT715_MIC_LINE_FORMAT_H: core::ffi::c_uint = 0x7208;
pub const RT715_MIC_LINE_FORMAT_L: core::ffi::c_uint = 0x8288;
pub const RT715_MIX_ADC_FORMAT_H: core::ffi::c_uint = 0x7209;
pub const RT715_MIX_ADC_FORMAT_L: core::ffi::c_uint = 0x8289;
pub const RT715_MIX_ADC2_FORMAT_H: core::ffi::c_uint = 0x7227;
pub const RT715_MIX_ADC2_FORMAT_L: core::ffi::c_uint = 0x82a7;
pub const RT715_FUNC_RESET: core::ffi::c_uint = 0xff01;

pub const RT715_SET_AUDIO_POWER_STATE: core::ffi::c_uint =
    RT715_VERB_SET_POWER_STATE | RT715_AUDIO_FUNCTION_GROUP;
pub const RT715_SET_PIN_DMIC1: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_DMIC1;
pub const RT715_SET_PIN_DMIC2: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_DMIC2;
pub const RT715_SET_PIN_DMIC3: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_DMIC3;
pub const RT715_SET_PIN_DMIC4: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_DMIC4;
pub const RT715_SET_PIN_MIC1: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_MIC1;
pub const RT715_SET_PIN_MIC2: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_MIC2;
pub const RT715_SET_PIN_LINE1: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_LINE1;
pub const RT715_SET_PIN_LINE2: core::ffi::c_uint = RT715_VERB_SET_PIN_WIDGET_CONTROL | RT715_LINE2;
pub const RT715_SET_MIC1_UNSOLICITED_ENABLE: core::ffi::c_uint =
    RT715_VERB_SET_UNSOLICITED_ENABLE | RT715_MIC1;
pub const RT715_SET_MIC2_UNSOLICITED_ENABLE: core::ffi::c_uint =
    RT715_VERB_SET_UNSOLICITED_ENABLE | RT715_MIC2;
pub const RT715_SET_STREAMID_MIC_ADC: core::ffi::c_uint =
    RT715_VERB_SET_CHANNEL_STREAMID | RT715_MIC_ADC;
pub const RT715_SET_STREAMID_LINE_ADC: core::ffi::c_uint =
    RT715_VERB_SET_CHANNEL_STREAMID | RT715_LINE_ADC;
pub const RT715_SET_STREAMID_MIX_ADC: core::ffi::c_uint =
    RT715_VERB_SET_CHANNEL_STREAMID | RT715_MIX_ADC;
pub const RT715_SET_STREAMID_MIX_ADC2: core::ffi::c_uint =
    RT715_VERB_SET_CHANNEL_STREAMID | RT715_MIX_ADC2;
pub const RT715_SET_GAIN_MIC_ADC_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_MIC_ADC;
pub const RT715_SET_GAIN_MIC_ADC_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_MIC_ADC;
pub const RT715_SET_GAIN_LINE_ADC_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_LINE_ADC;
pub const RT715_SET_GAIN_LINE_ADC_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_LINE_ADC;
pub const RT715_SET_GAIN_MIX_ADC_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_MIX_ADC;
pub const RT715_SET_GAIN_MIX_ADC_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_MIX_ADC;
pub const RT715_SET_GAIN_MIX_ADC2_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_MIX_ADC2;
pub const RT715_SET_GAIN_MIX_ADC2_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_MIX_ADC2;
pub const RT715_SET_GAIN_DMIC1_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_DMIC1;
pub const RT715_SET_GAIN_DMIC1_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_DMIC1;
pub const RT715_SET_GAIN_DMIC2_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_DMIC2;
pub const RT715_SET_GAIN_DMIC2_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_DMIC2;
pub const RT715_SET_GAIN_DMIC3_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_DMIC3;
pub const RT715_SET_GAIN_DMIC3_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_DMIC3;
pub const RT715_SET_GAIN_DMIC4_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_DMIC4;
pub const RT715_SET_GAIN_DMIC4_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_DMIC4;
pub const RT715_SET_GAIN_MIC1_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_MIC1;
pub const RT715_SET_GAIN_MIC1_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_MIC1;
pub const RT715_SET_GAIN_MIC2_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_MIC2;
pub const RT715_SET_GAIN_MIC2_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_MIC2;
pub const RT715_SET_GAIN_LINE1_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_LINE1;
pub const RT715_SET_GAIN_LINE1_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_LINE1;
pub const RT715_SET_GAIN_LINE2_L: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_L | RT715_LINE2;
pub const RT715_SET_GAIN_LINE2_H: core::ffi::c_uint = RT715_SET_AMP_GAIN_MUTE_H | RT715_LINE2;
pub const RT715_SET_DMIC1_CONFIG_DEFAULT1: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT1 | RT715_DMIC1;
pub const RT715_SET_DMIC2_CONFIG_DEFAULT1: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT1 | RT715_DMIC2;
pub const RT715_SET_DMIC1_CONFIG_DEFAULT2: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT2 | RT715_DMIC1;
pub const RT715_SET_DMIC2_CONFIG_DEFAULT2: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT2 | RT715_DMIC2;
pub const RT715_SET_DMIC1_CONFIG_DEFAULT3: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT3 | RT715_DMIC1;
pub const RT715_SET_DMIC2_CONFIG_DEFAULT3: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT3 | RT715_DMIC2;
pub const RT715_SET_DMIC1_CONFIG_DEFAULT4: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT4 | RT715_DMIC1;
pub const RT715_SET_DMIC2_CONFIG_DEFAULT4: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT4 | RT715_DMIC2;
pub const RT715_SET_DMIC3_CONFIG_DEFAULT1: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT1 | RT715_DMIC3;
pub const RT715_SET_DMIC4_CONFIG_DEFAULT1: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT1 | RT715_DMIC4;
pub const RT715_SET_DMIC3_CONFIG_DEFAULT2: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT2 | RT715_DMIC3;
pub const RT715_SET_DMIC4_CONFIG_DEFAULT2: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT2 | RT715_DMIC4;
pub const RT715_SET_DMIC3_CONFIG_DEFAULT3: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT3 | RT715_DMIC3;
pub const RT715_SET_DMIC4_CONFIG_DEFAULT3: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT3 | RT715_DMIC4;
pub const RT715_SET_DMIC3_CONFIG_DEFAULT4: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT4 | RT715_DMIC3;
pub const RT715_SET_DMIC4_CONFIG_DEFAULT4: core::ffi::c_uint =
    RT715_VERB_SET_CONFIG_DEFAULT4 | RT715_DMIC4;

/* vendor register clear ctrl-1    (0x01)(NID:20h) */
pub const RT715_CLEAR_HIDDEN_REG: core::ffi::c_uint = 0x1 << 15;

pub const RT715_MUTE_SFT: core::ffi::c_uint = 7;
pub const RT715_DIR_IN_SFT: core::ffi::c_uint = 6;
pub const RT715_DIR_OUT_SFT: core::ffi::c_uint = 7;

pub const RT715_AIF1: core::ffi::c_uint = 0;
pub const RT715_AIF2: core::ffi::c_uint = 1;

pub const RT715_POWER_UP_DELAY_MS: core::ffi::c_uint = 400;

unsafe extern "C" {
    pub fn rt715_io_init(dev: *mut device, slave: *mut sdw_slave) -> core::ffi::c_int;
    pub fn rt715_init(
        dev: *mut device,
        sdw_regmap: *mut regmap,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> core::ffi::c_int;

    pub fn rt715_clock_config(dev: *mut device) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
