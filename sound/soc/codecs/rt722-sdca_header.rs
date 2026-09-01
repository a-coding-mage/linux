/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt722-sdca.h -- RT722 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2023 Realtek Semiconductor Corp.
 */

/* C header guard and include directives removed.
 * External Linux/ALSA/SoundWire types are expected to be supplied by bindings
 * for linux/pm.h, linux/regmap.h, linux/soundwire/sdw.h,
 * linux/soundwire/sdw_type.h, sound/soc.h, and linux/workqueue.h.
 */

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct rt722_sdca_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub disable_irq: bool,
    /* For Headset jack & Headphone */
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub jack_type: c_int,
    pub jd_src: c_int,
    pub fu0f_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    /* For AMP */
    pub fu06_dapm_mute: bool,
    pub fu06_mixer_l_mute: bool,
    pub fu06_mixer_r_mute: bool,
    /* For DMIC */
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
    pub hw_vid: c_int,
    pub cae_update_done: c_int,
}

#[repr(C)]
pub struct rt722_sdca_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
}

/* NID */
pub const RT722_VENDOR_REG: c_uint = 0x20;
pub const RT722_VENDOR_EQ_CAE: c_uint = 0x53;
pub const RT722_VENDOR_CALI: c_uint = 0x58;
pub const RT722_VENDOR_SPK_EFUSE: c_uint = 0x5c;
pub const RT722_VENDOR_IMS_DRE: c_uint = 0x5b;
pub const RT722_VENDOR_ANALOG_CTL: c_uint = 0x5f;
pub const RT722_VENDOR_HDA_CTL: c_uint = 0x61;

/* Index (NID:20h) */
pub const RT722_JD_PRODUCT_NUM: c_uint = 0x00;
pub const RT722_ANALOG_BIAS_CTL3: c_uint = 0x04;
pub const RT722_MISC_CTRL1: c_uint = 0x07;
pub const RT722_JD_CTRL1: c_uint = 0x09;
pub const RT722_LDO2_3_CTL1: c_uint = 0x0e;
pub const RT722_LDO1_CTL: c_uint = 0x1a;
pub const RT722_HP_JD_CTRL: c_uint = 0x24;
pub const RT722_CLSD_CTRL6: c_uint = 0x3c;
pub const RT722_COMBO_JACK_AUTO_CTL1: c_uint = 0x45;
pub const RT722_COMBO_JACK_AUTO_CTL2: c_uint = 0x46;
pub const RT722_COMBO_JACK_AUTO_CTL3: c_uint = 0x47;
pub const RT722_DIGITAL_MISC_CTRL4: c_uint = 0x4a;
pub const RT722_VREFO_GAT: c_uint = 0x63;
pub const RT722_FSM_CTL: c_uint = 0x67;
pub const RT722_SDCA_INTR_REC: c_uint = 0x82;
pub const RT722_SW_CONFIG1: c_uint = 0x8a;
pub const RT722_SW_CONFIG2: c_uint = 0x8b;

/* Index (NID:53h) */
pub const RT722_EQ_CTRL_SPK: c_uint = 0x00;
pub const RT722_EQ_CTRL_HP: c_uint = 0x100;
pub const RT722_EQ_CTRL_DMIC: c_uint = 0x200;
pub const RT722_EQ_CTRL_AMIC: c_uint = 0x300;

/* Index (NID:58h) */
pub const RT722_DAC_DC_CALI_CTL0: c_uint = 0x00;
pub const RT722_DAC_DC_CALI_CTL1: c_uint = 0x01;
pub const RT722_DAC_DC_CALI_CTL2: c_uint = 0x02;
pub const RT722_DAC_DC_CALI_CTL3: c_uint = 0x03;

/* Index (NID:59h) */
pub const RT722_ULTRA_SOUND_DETECTOR6: c_uint = 0x1e;

/* Index (NID:5bh) */
pub const RT722_IMS_DIGITAL_CTL1: c_uint = 0x00;
pub const RT722_IMS_DIGITAL_CTL5: c_uint = 0x05;
pub const RT722_HP_DETECT_RLDET_CTL1: c_uint = 0x29;
pub const RT722_HP_DETECT_RLDET_CTL2: c_uint = 0x2a;

/* Index (NID:5fh) */
pub const RT722_MISC_POWER_CTL0: c_uint = 0x00;
pub const RT722_MISC_POWER_CTL7: c_uint = 0x08;

/* Index (NID:61h) */
pub const RT722_HDA_LEGACY_MUX_CTL0: c_uint = 0x00;
pub const RT722_HDA_LEGACY_UNSOL_CTL: c_uint = 0x03;
pub const RT722_HDA_LEGACY_CONFIG_CTL0: c_uint = 0x06;
pub const RT722_HDA_LEGACY_RESET_CTL: c_uint = 0x08;
pub const RT722_HDA_LEGACY_GPIO_WAKE_EN_CTL: c_uint = 0x0e;
pub const RT722_DMIC_ENT_FLOAT_CTL: c_uint = 0x10;
pub const RT722_DMIC_GAIN_ENT_FLOAT_CTL0: c_uint = 0x11;
pub const RT722_DMIC_GAIN_ENT_FLOAT_CTL2: c_uint = 0x13;
pub const RT722_ADC_ENT_FLOAT_CTL: c_uint = 0x15;
pub const RT722_ADC_VOL_CH_FLOAT_CTL: c_uint = 0x17;
pub const RT722_ADC_SAMPLE_RATE_FLOAT: c_uint = 0x18;
pub const RT722_DAC03_HP_PDE_FLOAT_CTL: c_uint = 0x22;
pub const RT722_MIC2_LINE2_PDE_FLOAT_CTL: c_uint = 0x23;
pub const RT722_ET41_LINE2_PDE_FLOAT_CTL: c_uint = 0x24;
pub const RT722_ADC0A_08_PDE_FLOAT_CTL: c_uint = 0x25;
pub const RT722_ADC10_PDE_FLOAT_CTL: c_uint = 0x26;
pub const RT722_DMIC1_2_PDE_FLOAT_CTL: c_uint = 0x28;
pub const RT722_AMP_PDE_FLOAT_CTL: c_uint = 0x29;
pub const RT722_I2S_IN_OUT_PDE_FLOAT_CTL: c_uint = 0x2f;
pub const RT722_GE_RELATED_CTL1: c_uint = 0x45;
pub const RT722_GE_RELATED_CTL2: c_uint = 0x46;
pub const RT722_MIXER_CTL0: c_uint = 0x52;
pub const RT722_MIXER_CTL1: c_uint = 0x53;
pub const RT722_EAPD_CTL: c_uint = 0x55;
pub const RT722_UMP_HID_CTL0: c_uint = 0x60;
pub const RT722_UMP_HID_CTL1: c_uint = 0x61;
pub const RT722_UMP_HID_CTL2: c_uint = 0x62;
pub const RT722_UMP_HID_CTL3: c_uint = 0x63;
pub const RT722_UMP_HID_CTL4: c_uint = 0x64;
pub const RT722_UMP_HID_CTL5: c_uint = 0x65;
pub const RT722_UMP_HID_CTL6: c_uint = 0x66;
pub const RT722_UMP_HID_CTL7: c_uint = 0x67;
pub const RT722_UMP_HID_CTL8: c_uint = 0x68;
pub const RT722_FLOAT_CTRL_1: c_uint = 0x70;
pub const RT722_ENT_FLOAT_CTRL_1: c_uint = 0x76;

/* Parameter & Verb control 01 (0x1a)(NID:20h) */
pub const RT722_HIDDEN_REG_SW_RESET: c_uint = 0x1 << 14;

/* combo jack auto switch control 2 (0x46)(NID:20h) */
pub const RT722_COMBOJACK_AUTO_DET_STATUS: c_uint = 0x1 << 11;
pub const RT722_COMBOJACK_AUTO_DET_TRS: c_uint = 0x1 << 10;
pub const RT722_COMBOJACK_AUTO_DET_CTIA: c_uint = 0x1 << 9;
pub const RT722_COMBOJACK_AUTO_DET_OMTP: c_uint = 0x1 << 8;

/* DAC calibration control (0x00)(NID:58h) */
pub const RT722_DC_CALIB_CTRL: c_uint = 0x1 << 16;
/* DAC DC offset calibration control-1 (0x01)(NID:58h) */
pub const RT722_PDM_DC_CALIB_STATUS: c_uint = 0x1 << 15;

pub const RT722_EAPD_HIGH: c_uint = 0x2;
pub const RT722_EAPD_LOW: c_uint = 0x0;

/* Buffer address for HID */
pub const RT722_BUF_ADDR_HID1: c_uint = 0x44030000;
pub const RT722_BUF_ADDR_HID2: c_uint = 0x44030020;

/* RT722 CAE parameter settings */
pub const RT722_SPK_CAE_PARAM1: c_uint = 0x44012000;
pub const RT722_SPK_CAE_PARAM34: c_uint = 0x44012021;
pub const RT722_SPK_CAE_PARAM35: c_uint = 0x44012022;
pub const RT722_SPK_CAE_PARAM38: c_uint = 0x44012025;
pub const RT722_HP_CAE_PARAM39: c_uint = 0x44022000;
pub const RT722_HP_CAE_PARAM64: c_uint = 0x44022019;
pub const RT722_HP_CAE_PARAM65: c_uint = 0x4402201a;
pub const RT722_HP_CAE_PARAM68: c_uint = 0x4402201d;
pub const RT722_MIC_CAE_PARAM39: c_uint = 0x44042000;
pub const RT722_MIC_CAE_PARAM95: c_uint = 0x44042019;
pub const RT722_MIC_CAE_PARAM96: c_uint = 0x4404201a;
pub const RT722_MIC_CAE_PARAM99: c_uint = 0x4404201d;

/* RT722 SDCA Control - function number */
pub const FUNC_NUM_JACK_CODEC: c_uint = 0x01;
pub const FUNC_NUM_MIC_ARRAY: c_uint = 0x02;
pub const FUNC_NUM_HID: c_uint = 0x03;
pub const FUNC_NUM_AMP: c_uint = 0x04;

/* RT722 SDCA entity */
pub const RT722_SDCA_ENT_HID01: c_uint = 0x01;
pub const RT722_SDCA_ENT_GE49: c_uint = 0x49;
pub const RT722_SDCA_ENT_USER_FU05: c_uint = 0x05;
pub const RT722_SDCA_ENT_USER_FU06: c_uint = 0x06;
pub const RT722_SDCA_ENT_USER_FU0F: c_uint = 0x0f;
pub const RT722_SDCA_ENT_USER_FU10: c_uint = 0x19;
pub const RT722_SDCA_ENT_USER_FU1E: c_uint = 0x1e;
pub const RT722_SDCA_ENT_FU15: c_uint = 0x15;
pub const RT722_SDCA_ENT_PDE23: c_uint = 0x23;
pub const RT722_SDCA_ENT_PDE40: c_uint = 0x40;
pub const RT722_SDCA_ENT_PDE11: c_uint = 0x11;
pub const RT722_SDCA_ENT_PDE12: c_uint = 0x12;
pub const RT722_SDCA_ENT_PDE2A: c_uint = 0x2a;
pub const RT722_SDCA_ENT_CS01: c_uint = 0x01;
pub const RT722_SDCA_ENT_CS11: c_uint = 0x11;
pub const RT722_SDCA_ENT_CS1F: c_uint = 0x1f;
pub const RT722_SDCA_ENT_CS1C: c_uint = 0x1c;
pub const RT722_SDCA_ENT_CS31: c_uint = 0x31;
pub const RT722_SDCA_ENT_OT23: c_uint = 0x42;
pub const RT722_SDCA_ENT_IT26: c_uint = 0x26;
pub const RT722_SDCA_ENT_IT09: c_uint = 0x09;
pub const RT722_SDCA_ENT_PLATFORM_FU15: c_uint = 0x15;
pub const RT722_SDCA_ENT_PLATFORM_FU44: c_uint = 0x44;
pub const RT722_SDCA_ENT_XU03: c_uint = 0x03;
pub const RT722_SDCA_ENT_XU0D: c_uint = 0x0d;
pub const RT722_SDCA_ENT0: c_uint = 0x00;

/* RT722 SDCA control */
pub const RT722_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const RT722_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT722_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT722_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0x10;
pub const RT722_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: c_uint = 0x11;
pub const RT722_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint = 0x12;
pub const RT722_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint = 0x13;
pub const RT722_SDCA_CTL_SELECTED_MODE: c_uint = 0x01;
pub const RT722_SDCA_CTL_DETECTED_MODE: c_uint = 0x02;
pub const RT722_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT722_SDCA_CTL_VENDOR_DEF: c_uint = 0x30;
pub const RT722_SDCA_CTL_FU_CH_GAIN: c_uint = 0x0b;
pub const RT722_SDCA_CTL_FUNC_STATUS: c_uint = 0x10;
pub const RT722_SDCA_CTL_ACTUAL_POWER_STATE: c_uint = 0x10;

/* RT722 SDCA channel */
pub const CH_L: c_uint = 0x01;
pub const CH_R: c_uint = 0x02;
pub const CH_01: c_uint = 0x01;
pub const CH_02: c_uint = 0x02;
pub const CH_03: c_uint = 0x03;
pub const CH_04: c_uint = 0x04;
pub const CH_08: c_uint = 0x08;

/* sample frequency index */
pub const RT722_SDCA_RATE_16000HZ: c_uint = 0x04;
pub const RT722_SDCA_RATE_32000HZ: c_uint = 0x07;
pub const RT722_SDCA_RATE_44100HZ: c_uint = 0x08;
pub const RT722_SDCA_RATE_48000HZ: c_uint = 0x09;
pub const RT722_SDCA_RATE_96000HZ: c_uint = 0x0b;
pub const RT722_SDCA_RATE_192000HZ: c_uint = 0x0d;

/* Function_Status */
pub const FUNCTION_NEEDS_INITIALIZATION: c_uint = 1 << 5;

pub const RT722_AIF1: c_uint = 0; /* For headset mic and headphone */
pub const RT722_AIF2: c_uint = 1; /* For speaker */
pub const RT722_AIF3: c_uint = 2; /* For dmic */
pub const RT722_AIFS: c_uint = 3;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rt722_sdca_jd_src {
    RT722_JD_NULL = 0,
    RT722_JD1 = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rt722_sdca_version {
    RT722_VA = 0,
    RT722_VB = 1,
}

unsafe extern "C" {
    pub fn rt722_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
    pub fn rt722_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> c_int;
    pub fn rt722_sdca_index_write(
        rt722: *mut rt722_sdca_priv,
        nid: c_uint,
        reg: c_uint,
        value: c_uint,
    ) -> c_int;
    pub fn rt722_sdca_index_read(
        rt722: *mut rt722_sdca_priv,
        nid: c_uint,
        reg: c_uint,
        value: *mut c_uint,
    ) -> c_int;

    pub fn rt722_sdca_jack_detect(
        rt722: *mut rt722_sdca_priv,
        hp: *mut bool,
        mic: *mut bool,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
