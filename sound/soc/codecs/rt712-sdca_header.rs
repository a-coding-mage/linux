/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt712-sdca.h -- RT712 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2023 Realtek Semiconductor Corp.
 */

/* Dependencies from the original header:
 * linux/pm.h, linux/regmap.h, linux/soundwire/sdw.h,
 * linux/soundwire/sdw_type.h, sound/soc.h, linux/workqueue.h
 */

#[repr(C)]
pub struct rt712_sdca_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub dmic_component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibrate_mutex: mutex, /* for headset calibration */
    pub disable_irq_lock: mutex, /* SDCA irq lock protection */
    pub disable_irq: bool,
    pub jack_type: core::ffi::c_int,
    pub jd_src: core::ffi::c_int,
    pub scp_sdca_stat1: core::ffi::c_uint,
    pub scp_sdca_stat2: core::ffi::c_uint,
    pub hw_id: core::ffi::c_uint,
    pub version_id: core::ffi::c_uint,
    pub dmic_function_found: bool,
    pub fu0f_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
    pub fu05_dapm_mute: bool,
    pub fu05_mixer_l_mute: bool,
    pub fu05_mixer_r_mute: bool,
}

#[repr(C)]
pub struct rt712_dmic_kctrl_priv {
    pub reg_base: core::ffi::c_uint,
    pub count: core::ffi::c_uint,
    pub max: core::ffi::c_uint,
    pub invert: core::ffi::c_uint,
}

/* SDCA (Channel) */
pub const CH_01: core::ffi::c_int = 0x01;
pub const CH_02: core::ffi::c_int = 0x02;
pub const CH_03: core::ffi::c_int = 0x03;
pub const CH_04: core::ffi::c_int = 0x04;

/* NID */
pub const RT712_VENDOR_REG: core::ffi::c_int = 0x20;
pub const RT712_EQ_CTRL: core::ffi::c_int = 0x53;
pub const RT712_CHARGE_PUMP: core::ffi::c_int = 0x57;
pub const RT712_VENDOR_CALI: core::ffi::c_int = 0x58;
pub const RT712_ULTRA_SOUND_DET: core::ffi::c_int = 0x59;
pub const RT712_VENDOR_IMS_DRE: core::ffi::c_int = 0x5b;
pub const RT712_VENDOR_ANALOG_CTL: core::ffi::c_int = 0x5f;
pub const RT712_VENDOR_HDA_CTL: core::ffi::c_int = 0x61;

/* Index (NID:20h) */
pub const RT712_JD_PRODUCT_NUM: core::ffi::c_int = 0x00;
pub const RT712_ANALOG_BIAS_CTL3: core::ffi::c_int = 0x04;
pub const RT712_JD_CTL1: core::ffi::c_int = 0x09;
pub const RT712_JD_CTL3: core::ffi::c_int = 0x0b;
pub const RT712_IO_CTL: core::ffi::c_int = 0x0c;
pub const RT712_LDO2_3_CTL1: core::ffi::c_int = 0x0e;
pub const RT712_PARA_VERB_CTL: core::ffi::c_int = 0x1a;
pub const RT712_CC_DET1: core::ffi::c_int = 0x24;
pub const RT712_CLASSD_AMP_CTL1: core::ffi::c_int = 0x37;
pub const RT712_CLASSD_AMP_CTL6: core::ffi::c_int = 0x3c;
pub const RT712_COMBO_JACK_AUTO_CTL1: core::ffi::c_int = 0x45;
pub const RT712_COMBO_JACK_AUTO_CTL2: core::ffi::c_int = 0x46;
pub const RT712_COMBO_JACK_AUTO_CTL3: core::ffi::c_int = 0x47;
pub const RT712_DIGITAL_MISC_CTRL4: core::ffi::c_int = 0x4a;
pub const RT712_FSM_CTL: core::ffi::c_int = 0x67;
pub const RT712_SW_CONFIG1: core::ffi::c_int = 0x8a;
pub const RT712_SW_CONFIG2: core::ffi::c_int = 0x8b;

/* Index (NID:57h) */
pub const RT712_HP_DET_CTL3: core::ffi::c_int = 0x0c;

/* Index (NID:58h) */
pub const RT712_DAC_DC_CALI_CTL1: core::ffi::c_int = 0x00;
pub const RT712_DAC_DC_CALI_CTL2: core::ffi::c_int = 0x01;

/* Index (NID:59h) */
pub const RT712_ULTRA_SOUND_DETECTOR6: core::ffi::c_int = 0x1e;

/* Index (NID:5bh) */
pub const RT712_IMS_DIGITAL_CTL1: core::ffi::c_int = 0x00;
pub const RT712_IMS_DIGITAL_CTL5: core::ffi::c_int = 0x05;
pub const RT712_SEL_VEE2_HP_CTL1: core::ffi::c_int = 0x23;
pub const RT712_HP_DETECT_RLDET_CTL1: core::ffi::c_int = 0x29;
pub const RT712_HP_DETECT_RLDET_CTL2: core::ffi::c_int = 0x2a;

/* Index (NID:5fh) */
pub const RT712_MISC_POWER_CTL0: core::ffi::c_int = 0x00;
pub const RT712_MISC_POWER_CTL7: core::ffi::c_int = 0x08;

/* Index (NID:61h) */
pub const RT712_HDA_LEGACY_MUX_CTL0: core::ffi::c_int = 0x00;
pub const RT712_HDA_LEGACY_CONFIG_CTL0: core::ffi::c_int = 0x06;
pub const RT712_HDA_LEGACY_RESET_CTL: core::ffi::c_int = 0x08;
pub const RT712_HDA_LEGACY_GPIO_WAKE_EN_CTL: core::ffi::c_int = 0x0e;
pub const RT712_DMIC_ENT_FLOAT_CTL: core::ffi::c_int = 0x10;
pub const RT712_DMIC_GAIN_ENT_FLOAT_CTL0: core::ffi::c_int = 0x11;
pub const RT712_DMIC_GAIN_ENT_FLOAT_CTL2: core::ffi::c_int = 0x13;
pub const RT712_ADC_ENT_FLOAT_CTL: core::ffi::c_int = 0x15;
pub const RT712_ADC_VOL_CH_FLOAT_CTL2: core::ffi::c_int = 0x18;
pub const RT712_DAC03_HP_PDE_FLOAT_CTL: core::ffi::c_int = 0x22;
pub const RT712_MIC2_LINE2_PDE_FLOAT_CTL: core::ffi::c_int = 0x23;
pub const RT712_ADC0A_08_PDE_FLOAT_CTL: core::ffi::c_int = 0x26;
pub const RT712_ADC0B_11_PDE_FLOAT_CTL: core::ffi::c_int = 0x27;
pub const RT712_DMIC1_2_PDE_FLOAT_CTL: core::ffi::c_int = 0x2b;
pub const RT712_AMP_PDE_FLOAT_CTL: core::ffi::c_int = 0x2c;
pub const RT712_I2S_IN_OUT_PDE_FLOAT_CTL: core::ffi::c_int = 0x2f;
pub const RT712_GE_RELATED_CTL1: core::ffi::c_int = 0x45;
pub const RT712_GE_RELATED_CTL2: core::ffi::c_int = 0x46;
pub const RT712_MIXER_CTL0: core::ffi::c_int = 0x52;
pub const RT712_MIXER_CTL1: core::ffi::c_int = 0x53;
pub const RT712_EAPD_CTL: core::ffi::c_int = 0x55;
pub const RT712_UMP_HID_CTL0: core::ffi::c_int = 0x60;
pub const RT712_UMP_HID_CTL1: core::ffi::c_int = 0x61;
pub const RT712_UMP_HID_CTL2: core::ffi::c_int = 0x62;
pub const RT712_UMP_HID_CTL3: core::ffi::c_int = 0x63;
pub const RT712_UMP_HID_CTL4: core::ffi::c_int = 0x64;
pub const RT712_UMP_HID_CTL5: core::ffi::c_int = 0x65;
pub const RT712_UMP_HID_CTL6: core::ffi::c_int = 0x66;
pub const RT712_UMP_HID_CTL7: core::ffi::c_int = 0x67;
pub const RT712_UMP_HID_CTL8: core::ffi::c_int = 0x68;
pub const RT712_MISC_CTL_FOR_UAJ: core::ffi::c_int = 0x72;
pub const RT712_ADC0A_CS_ADC0B_FU_FLOAT_CTL: core::ffi::c_int = 0xa2;
pub const RT712_DMIC2_FU_IT_FLOAT_CTL: core::ffi::c_int = 0xa6;
pub const RT712_ADC0B_FU_CH12_FLOAT_CTL: core::ffi::c_int = 0xb0;
pub const RT712_DMIC2_FU_CH12_FLOAT_CTL: core::ffi::c_int = 0xb1;

/* Parameter & Verb control 01 (0x1a)(NID:20h) */
pub const RT712_HIDDEN_REG_SW_RESET: core::ffi::c_int = 0x1 << 14;

/* combo jack auto switch control 2 (0x46)(NID:20h) */
pub const RT712_COMBOJACK_AUTO_DET_STATUS: core::ffi::c_int = 0x1 << 11;
pub const RT712_COMBOJACK_AUTO_DET_TRS: core::ffi::c_int = 0x1 << 10;
pub const RT712_COMBOJACK_AUTO_DET_CTIA: core::ffi::c_int = 0x1 << 9;
pub const RT712_COMBOJACK_AUTO_DET_OMTP: core::ffi::c_int = 0x1 << 8;

/* DAC DC offset calibration control-1 (0x00)(NID:58h) */
pub const RT712_DAC_DC_CALI_TRIGGER: core::ffi::c_int = 0x1 << 15;

pub const RT712_EAPD_HIGH: core::ffi::c_int = 0x2;
pub const RT712_EAPD_LOW: core::ffi::c_int = 0x0;

/* RC Calibration register */
pub const RT712_RC_CAL: core::ffi::c_int = 0x3201;

/* Buffer address for HID */
pub const RT712_BUF_ADDR_HID1: core::ffi::c_uint = 0x44030000;
pub const RT712_BUF_ADDR_HID2: core::ffi::c_uint = 0x44030020;

/* RT712 SDCA Control - function number */
pub const FUNC_NUM_JACK_CODEC: core::ffi::c_int = 0x01;
pub const FUNC_NUM_MIC_ARRAY: core::ffi::c_int = 0x02;
pub const FUNC_NUM_HID: core::ffi::c_int = 0x03;
pub const FUNC_NUM_AMP: core::ffi::c_int = 0x04;

/* RT712 SDCA entity */
pub const RT712_SDCA_ENT0: core::ffi::c_int = 0x00;
pub const RT712_SDCA_ENT_HID01: core::ffi::c_int = 0x01;
pub const RT712_SDCA_ENT_GE49: core::ffi::c_int = 0x49;
pub const RT712_SDCA_ENT_USER_FU05: core::ffi::c_int = 0x05;
pub const RT712_SDCA_ENT_USER_FU06: core::ffi::c_int = 0x06;
pub const RT712_SDCA_ENT_USER_FU0F: core::ffi::c_int = 0x0f;
pub const RT712_SDCA_ENT_USER_FU10: core::ffi::c_int = 0x19;
pub const RT712_SDCA_ENT_USER_FU1E: core::ffi::c_int = 0x1e;
pub const RT712_SDCA_ENT_FU15: core::ffi::c_int = 0x15;
pub const RT712_SDCA_ENT_PDE23: core::ffi::c_int = 0x23;
pub const RT712_SDCA_ENT_PDE40: core::ffi::c_int = 0x40;
pub const RT712_SDCA_ENT_PDE11: core::ffi::c_int = 0x11;
pub const RT712_SDCA_ENT_PDE12: core::ffi::c_int = 0x12;
pub const RT712_SDCA_ENT_CS01: core::ffi::c_int = 0x01;
pub const RT712_SDCA_ENT_CS11: core::ffi::c_int = 0x11;
pub const RT712_SDCA_ENT_CS1F: core::ffi::c_int = 0x1f;
pub const RT712_SDCA_ENT_CS1C: core::ffi::c_int = 0x1c;
pub const RT712_SDCA_ENT_CS31: core::ffi::c_int = 0x31;
pub const RT712_SDCA_ENT_OT23: core::ffi::c_int = 0x42;
pub const RT712_SDCA_ENT_IT11: core::ffi::c_int = 0x26;
pub const RT712_SDCA_ENT_IT26: core::ffi::c_int = 0x26;
pub const RT712_SDCA_ENT_IT09: core::ffi::c_int = 0x09;
pub const RT712_SDCA_ENT_PLATFORM_FU15: core::ffi::c_int = 0x15;
pub const RT712_SDCA_ENT_PLATFORM_FU44: core::ffi::c_int = 0x44;

/* RT712 SDCA control */
pub const RT712_SDCA_CTL_SAMPLE_FREQ_INDEX: core::ffi::c_int = 0x10;
pub const RT712_SDCA_CTL_FU_MUTE: core::ffi::c_int = 0x01;
pub const RT712_SDCA_CTL_FU_VOLUME: core::ffi::c_int = 0x02;
pub const RT712_SDCA_CTL_HIDTX_CURRENT_OWNER: core::ffi::c_int = 0x10;
pub const RT712_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: core::ffi::c_int = 0x11;
pub const RT712_SDCA_CTL_HIDTX_MESSAGE_OFFSET: core::ffi::c_int = 0x12;
pub const RT712_SDCA_CTL_HIDTX_MESSAGE_LENGTH: core::ffi::c_int = 0x13;
pub const RT712_SDCA_CTL_SELECTED_MODE: core::ffi::c_int = 0x01;
pub const RT712_SDCA_CTL_DETECTED_MODE: core::ffi::c_int = 0x02;
pub const RT712_SDCA_CTL_REQ_POWER_STATE: core::ffi::c_int = 0x01;
pub const RT712_SDCA_CTL_VENDOR_DEF: core::ffi::c_int = 0x30;
pub const RT712_SDCA_CTL_FU_CH_GAIN: core::ffi::c_int = 0x0b;
pub const RT712_SDCA_CTL_FUNC_STATUS: core::ffi::c_int = 0x10;

/* Function_Status */
pub const FUNCTION_NEEDS_INITIALIZATION: core::ffi::c_int = 1 << 5;
pub const FUNCTION_HAS_BEEN_RESET: core::ffi::c_int = 1 << 6;
pub const FUNCTION_BUSY: core::ffi::c_int = 1 << 7;

/* sample frequency index */
pub const RT712_SDCA_RATE_16000HZ: core::ffi::c_int = 0x04;
pub const RT712_SDCA_RATE_32000HZ: core::ffi::c_int = 0x07;
pub const RT712_SDCA_RATE_44100HZ: core::ffi::c_int = 0x08;
pub const RT712_SDCA_RATE_48000HZ: core::ffi::c_int = 0x09;
pub const RT712_SDCA_RATE_96000HZ: core::ffi::c_int = 0x0b;
pub const RT712_SDCA_RATE_192000HZ: core::ffi::c_int = 0x0d;

pub const RT712_AIF1: core::ffi::c_int = 0;
pub const RT712_AIF2: core::ffi::c_int = 1;
pub const RT712_AIF3: core::ffi::c_int = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rt712_sdca_jd_src {
    RT712_JD_NULL = 0,
    RT712_JD1 = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rt712_sdca_hw_id {
    RT712_DEV_ID_712 = 0x7,
    RT712_DEV_ID_713 = 0x6,
    RT712_DEV_ID_716 = 0x5,
    RT712_DEV_ID_717 = 0x4,
}

pub const RT712_PART_ID_713: core::ffi::c_int = 0x713;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rt712_sdca_version {
    RT712_VA = 0,
    RT712_VB = 1,
}

unsafe extern "C" {
    pub fn rt712_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> core::ffi::c_int;
    pub fn rt712_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        mbq_regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> core::ffi::c_int;

    pub fn rt712_sdca_jack_detect(
        rt712: *mut rt712_sdca_priv,
        hp: *mut bool,
        mic: *mut bool,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
