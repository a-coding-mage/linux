/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt711-sdca.h -- RT711 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2021 Realtek Semiconductor Corp.
 */

// C header dependencies removed from executable Rust:
// <linux/pm.h>, <linux/regmap.h>, <linux/soundwire/sdw.h>,
// <linux/soundwire/sdw_type.h>, <sound/soc.h>, <linux/workqueue.h>.

use core::ffi::{c_int, c_uint};

// External dependency types. Their concrete definitions/layouts are supplied by
// translated headers outside this isolated file.
pub enum device {}
pub enum regmap {}
pub enum snd_soc_component {}
pub enum sdw_slave {}
pub enum snd_soc_jack {}
pub enum sdw_bus_params {}
pub enum delayed_work {}
pub enum mutex {}

#[repr(C)]
pub struct rt711_sdca_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    // TODO: by-value external kernel struct; use the real translated layout.
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    // TODO: by-value external kernel struct; use the real translated layout.
    pub jack_detect_work: delayed_work,
    // TODO: by-value external kernel struct; use the real translated layout.
    pub jack_btn_check_work: delayed_work,
    pub calibrate_mutex: mutex, /* for headset calibration */
    pub disable_irq_lock: mutex, /* SDCA irq lock protection */
    pub disable_irq: bool,
    pub jack_type: c_int,
    pub jd_src: c_int,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub hw_ver: c_int,
    pub fu0f_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_l_mute: bool,
    pub fu1e_mixer_r_mute: bool,
    pub ge_mode_override: c_uint,
}

/* NID */
pub const RT711_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT711_DAC_OUT2: c_uint = 0x03;
pub const RT711_ADC_IN1: c_uint = 0x09;
pub const RT711_ADC_IN2: c_uint = 0x08;
pub const RT711_DMIC1: c_uint = 0x12;
pub const RT711_DMIC2: c_uint = 0x13;
pub const RT711_MIC2: c_uint = 0x19;
pub const RT711_LINE1: c_uint = 0x1a;
pub const RT711_LINE2: c_uint = 0x1b;
pub const RT711_BEEP: c_uint = 0x1d;
pub const RT711_VENDOR_REG: c_uint = 0x20;
pub const RT711_HP_OUT: c_uint = 0x21;
pub const RT711_MIXER_IN1: c_uint = 0x22;
pub const RT711_MIXER_IN2: c_uint = 0x23;
pub const RT711_INLINE_CMD: c_uint = 0x55;
pub const RT711_VENDOR_CALI: c_uint = 0x58;
pub const RT711_VENDOR_IMS_DRE: c_uint = 0x5b;
pub const RT711_VENDOR_VAD: c_uint = 0x5e;
pub const RT711_VENDOR_ANALOG_CTL: c_uint = 0x5f;
pub const RT711_VENDOR_HDA_CTL: c_uint = 0x61;

/* Index (NID:20h) */
pub const RT711_JD_PRODUCT_NUM: c_uint = 0x00;
pub const RT711_DMIC_CTL1: c_uint = 0x06;
pub const RT711_JD_CTL1: c_uint = 0x08;
pub const RT711_JD_CTL2: c_uint = 0x09;
pub const RT711_CC_DET1: c_uint = 0x11;
pub const RT711_PARA_VERB_CTL: c_uint = 0x1a;
pub const RT711_COMBO_JACK_AUTO_CTL1: c_uint = 0x45;
pub const RT711_COMBO_JACK_AUTO_CTL2: c_uint = 0x46;
pub const RT711_COMBO_JACK_AUTO_CTL3: c_uint = 0x47;
pub const RT711_INLINE_CMD_CTL: c_uint = 0x48;
pub const RT711_DIGITAL_MISC_CTRL4: c_uint = 0x4a;
pub const RT711_JD_CTRL6: c_uint = 0x6a;
pub const RT711_VREFOUT_CTL: c_uint = 0x6b;
pub const RT711_GPIO_TEST_MODE_CTL2: c_uint = 0x6d;
pub const RT711_FSM_CTL: c_uint = 0x6f;
pub const RT711_IRQ_FLAG_TABLE1: c_uint = 0x80;
pub const RT711_IRQ_FLAG_TABLE2: c_uint = 0x81;
pub const RT711_IRQ_FLAG_TABLE3: c_uint = 0x82;
pub const RT711_HP_FSM_CTL: c_uint = 0x83;
pub const RT711_TX_RX_MUX_CTL: c_uint = 0x91;
pub const RT711_FILTER_SRC_SEL: c_uint = 0xb0;
pub const RT711_ADC27_VOL_SET: c_uint = 0xb7;

/* Index (NID:58h) */
pub const RT711_DAC_DC_CALI_CTL1: c_uint = 0x00;
pub const RT711_DAC_DC_CALI_CTL2: c_uint = 0x01;

/* Index (NID:5bh) */
pub const RT711_IMS_DIGITAL_CTL1: c_uint = 0x00;
pub const RT711_HP_IMS_RESULT_L: c_uint = 0x20;
pub const RT711_HP_IMS_RESULT_R: c_uint = 0x21;

/* Index (NID:5eh) */
pub const RT711_VAD_SRAM_CTL1: c_uint = 0x10;

/* Index (NID:5fh) */
pub const RT711_MISC_POWER_CTL0: c_uint = 0x01;
pub const RT711_MISC_POWER_CTL4: c_uint = 0x05;

/* Index (NID:61h) */
pub const RT711_HDA_LEGACY_MUX_CTL1: c_uint = 0x00;
pub const RT711_HDA_LEGACY_UNSOLICITED_CTL: c_uint = 0x03;
pub const RT711_HDA_LEGACY_CONFIG_CTL: c_uint = 0x06;
pub const RT711_HDA_LEGACY_RESET_CTL: c_uint = 0x08;
pub const RT711_HDA_LEGACY_GPIO_CTL: c_uint = 0x0a;
pub const RT711_ADC08_09_PDE_CTL: c_uint = 0x24;
pub const RT711_GE_MODE_RELATED_CTL: c_uint = 0x35;
pub const RT711_PUSH_BTN_INT_CTL0: c_uint = 0x36;
pub const RT711_PUSH_BTN_INT_CTL1: c_uint = 0x37;
pub const RT711_PUSH_BTN_INT_CTL2: c_uint = 0x38;
pub const RT711_PUSH_BTN_INT_CTL6: c_uint = 0x3c;
pub const RT711_PUSH_BTN_INT_CTL7: c_uint = 0x3d;
pub const RT711_PUSH_BTN_INT_CTL9: c_uint = 0x3f;

/* DAC DC offset calibration control-1 (0x00)(NID:20h) */
pub const RT711_DAC_DC_CALI_TRIGGER: c_uint = 0x1 << 15;
pub const RT711_DAC_DC_CALI_CLK_EN: c_uint = 0x1 << 14;
pub const RT711_DAC_DC_FORCE_CALI_RST: c_uint = 0x1 << 3;

/* jack detect control 1 (0x08)(NID:20h) */
pub const RT711_JD2_DIGITAL_MODE_SEL: c_uint = 0x1 << 1;

/* jack detect control 2 (0x09)(NID:20h) */
pub const RT711_JD2_2PORT_200K_DECODE_HP: c_uint = 0x1 << 13;
pub const RT711_JD2_2PORT_100K_DECODE_MASK: c_uint = 0x1 << 12;
pub const RT711_JD2_2PORT_100K_DECODE_HP: c_uint = 0x0 << 12;
pub const RT711_HP_JD_SEL_JD1: c_uint = 0x0 << 1;
pub const RT711_HP_JD_SEL_JD2: c_uint = 0x1 << 1;

/* CC DET1 (0x11)(NID:20h) */
pub const RT711_HP_JD_FINAL_RESULT_CTL_JD12: c_uint = 0x1 << 10;
pub const RT711_HP_JD_FINAL_RESULT_CTL_CCDET: c_uint = 0x0 << 10;
pub const RT711_POW_CC1_AGPI: c_uint = 0x1 << 5;
pub const RT711_POW_CC1_AGPI_ON: c_uint = 0x1 << 5;
pub const RT711_POW_CC1_AGPI_OFF: c_uint = 0x0 << 5;

/* Parameter & Verb control (0x1a)(NID:20h) */
pub const RT711_HIDDEN_REG_SW_RESET: c_uint = 0x1 << 14;

/* combo jack auto switch control 2 (0x46)(NID:20h) */
pub const RT711_COMBOJACK_AUTO_DET_STATUS: c_uint = 0x1 << 11;
pub const RT711_COMBOJACK_AUTO_DET_TRS: c_uint = 0x1 << 10;
pub const RT711_COMBOJACK_AUTO_DET_CTIA: c_uint = 0x1 << 9;
pub const RT711_COMBOJACK_AUTO_DET_OMTP: c_uint = 0x1 << 8;

/* FSM control (0x6f)(NID:20h) */
pub const RT711_CALI_CTL: c_uint = 0x0 << 0;
pub const RT711_COMBOJACK_CTL: c_uint = 0x1 << 0;
pub const RT711_IMS_CTL: c_uint = 0x2 << 0;
pub const RT711_DEPOP_CTL: c_uint = 0x3 << 0;
pub const RT711_FSM_IMP_EN: c_uint = 0x1 << 6;

/* Impedance Sense Digital Control 1 (0x00)(NID:5bh) */
pub const RT711_TRIGGER_IMS: c_uint = 0x1 << 15;
pub const RT711_IMS_EN: c_uint = 0x1 << 6;

pub const RT711_EAPD_HIGH: c_uint = 0x2;
pub const RT711_EAPD_LOW: c_uint = 0x0;
pub const RT711_MUTE_SFT: c_uint = 7;
/* set input/output mapping to payload[14][15] separately */
pub const RT711_DIR_IN_SFT: c_uint = 6;
pub const RT711_DIR_OUT_SFT: c_uint = 7;

/* RC Calibration register */
pub const RT711_RC_CAL_STATUS: c_uint = 0x320c;

/* Buffer address for HID */
pub const RT711_BUF_ADDR_HID1: c_uint = 0x44030000;
pub const RT711_BUF_ADDR_HID2: c_uint = 0x44030020;

/* RT711 SDCA Control - function number */
pub const FUNC_NUM_JACK_CODEC: c_uint = 0x01;
pub const FUNC_NUM_MIC_ARRAY: c_uint = 0x02;
pub const FUNC_NUM_HID: c_uint = 0x03;

/* RT711 SDCA entity */
pub const RT711_SDCA_ENT_HID01: c_uint = 0x01;
pub const RT711_SDCA_ENT_GE49: c_uint = 0x49;
pub const RT711_SDCA_ENT_USER_FU05: c_uint = 0x05;
pub const RT711_SDCA_ENT_USER_FU0F: c_uint = 0x0f;
pub const RT711_SDCA_ENT_USER_FU1E: c_uint = 0x1e;
pub const RT711_SDCA_ENT_PLATFORM_FU15: c_uint = 0x15;
pub const RT711_SDCA_ENT_PLATFORM_FU44: c_uint = 0x44;
pub const RT711_SDCA_ENT_PDE28: c_uint = 0x28;
pub const RT711_SDCA_ENT_PDE29: c_uint = 0x29;
pub const RT711_SDCA_ENT_PDE2A: c_uint = 0x2a;
pub const RT711_SDCA_ENT_CS01: c_uint = 0x01;
pub const RT711_SDCA_ENT_CS11: c_uint = 0x11;
pub const RT711_SDCA_ENT_CS1F: c_uint = 0x1f;
pub const RT711_SDCA_ENT_OT1: c_uint = 0x06;
pub const RT711_SDCA_ENT_LINE1: c_uint = 0x09;
pub const RT711_SDCA_ENT_LINE2: c_uint = 0x31;
pub const RT711_SDCA_ENT_PDELINE2: c_uint = 0x36;
pub const RT711_SDCA_ENT_USER_FU9: c_uint = 0x41;

/* RT711 SDCA control */
pub const RT711_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const RT711_SDCA_CTL_FU_CH_GAIN: c_uint = 0x0b;
pub const RT711_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT711_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT711_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0x10;
pub const RT711_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: c_uint = 0x11;
pub const RT711_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint = 0x12;
pub const RT711_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint = 0x13;
pub const RT711_SDCA_CTL_SELECTED_MODE: c_uint = 0x01;
pub const RT711_SDCA_CTL_DETECTED_MODE: c_uint = 0x02;
pub const RT711_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT711_SDCA_CTL_VENDOR_DEF: c_uint = 0x30;

/* RT711 SDCA channel */
pub const CH_L: c_uint = 0x01;
pub const CH_R: c_uint = 0x02;

/* sample frequency index */
pub const RT711_SDCA_RATE_44100HZ: c_uint = 0x08;
pub const RT711_SDCA_RATE_48000HZ: c_uint = 0x09;
pub const RT711_SDCA_RATE_96000HZ: c_uint = 0x0b;
pub const RT711_SDCA_RATE_192000HZ: c_uint = 0x0d;

pub const RT711_AIF1: c_uint = 0;
pub const RT711_AIF2: c_uint = 1;
pub const RT711_AIFS: c_uint = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rt711_sdca_jd_src {
    RT711_JD_NULL = 0,
    RT711_JD1 = 1,
    RT711_JD2 = 2,
    RT711_JD2_100K = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rt711_sdca_ver {
    RT711_VER_VD0 = 0,
    RT711_VER_VD1 = 1,
}

unsafe extern "C" {
    pub fn rt711_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    pub fn rt711_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        mbq_regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    pub fn rt711_sdca_jack_detect(
        rt711: *mut rt711_sdca_priv,
        hp: *mut bool,
        mic: *mut bool,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
