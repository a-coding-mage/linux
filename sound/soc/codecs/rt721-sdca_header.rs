/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt721-sdca.h -- RT721 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2024 Realtek Semiconductor Corp.
 */

/* Dependencies in the original C header:
 * <linux/pm.h>
 * <linux/regmap.h>
 * <linux/soundwire/sdw.h>
 * <linux/soundwire/sdw_type.h>
 * <sound/soc.h>
 * <linux/workqueue.h>
 */

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus_params {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rt721_sdca_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub disable_irq: bool,
    /* For Headset jack & Headphone */
    pub scp_sdca_stat1: ::core::ffi::c_uint,
    pub scp_sdca_stat2: ::core::ffi::c_uint,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub jack_type: ::core::ffi::c_int,
    pub jd_src: ::core::ffi::c_int,
    pub fu0f_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    /* For DMIC */
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
}

#[repr(C)]
pub struct rt721_sdca_dmic_kctrl_priv {
    pub reg_base: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub invert: ::core::ffi::c_uint,
}

/* NID */
pub const RT721_ANA_POW_PART: u32 = 0x01;
pub const RT721_DAC_CTRL: u32 = 0x04;
pub const RT721_JD_CTRL: u32 = 0x09;
pub const RT721_CBJ_CTRL: u32 = 0x0a;
pub const RT721_CAP_PORT_CTRL: u32 = 0x0c;
pub const RT721_CLASD_AMP_CTRL: u32 = 0x0d;
pub const RT721_BOOST_CTRL: u32 = 0x0f;
pub const RT721_VENDOR_REG: u32 = 0x20;
pub const RT721_RC_CALIB_CTRL: u32 = 0x40;
pub const RT721_VENDOR_EQ_L: u32 = 0x53;
pub const RT721_VENDOR_EQ_R: u32 = 0x54;
pub const RT721_VENDOR_HP_CALI: u32 = 0x56;
pub const RT721_VENDOR_CHARGE_PUMP: u32 = 0x57;
pub const RT721_VENDOR_CLASD_CALI: u32 = 0x58;
pub const RT721_VENDOR_IMS_DRE: u32 = 0x5b;
pub const RT721_VENDOR_SPK_EFUSE: u32 = 0x5c;
pub const RT721_VENDOR_LEVEL_CTRL: u32 = 0x5d;
pub const RT721_VENDOR_ANA_CTL: u32 = 0x5f;
pub const RT721_HDA_SDCA_FLOAT: u32 = 0x61;

/* Index (NID:01h) */
pub const RT721_MBIAS_LV_CTRL2: u32 = 0x07;
pub const RT721_VREF1_HV_CTRL1: u32 = 0x0a;
pub const RT721_VREF2_LV_CTRL1: u32 = 0x0b;

/* Index (NID:04h) */
pub const RT721_DAC_2CH_CTRL3: u32 = 0x02;
pub const RT721_DAC_2CH_CTRL4: u32 = 0x03;

/* Index (NID:09h) */
pub const RT721_JD_1PIN_GAT_CTRL2: u32 = 0x07;

/* Index (NID:0ah) */
pub const RT721_CBJ_A0_GAT_CTRL1: u32 = 0x04;
pub const RT721_CBJ_A0_GAT_CTRL2: u32 = 0x05;

/* Index (NID:0Ch) */
pub const RT721_HP_AMP_2CH_CAL1: u32 = 0x05;
pub const RT721_HP_AMP_2CH_CAL4: u32 = 0x08;
pub const RT721_HP_AMP_2CH_CAL18: u32 = 0x1b;

/* Index (NID:0dh) */
pub const RT721_CLASD_AMP_2CH_CAL: u32 = 0x14;

/* Index (NID:0fh) */
pub const RT721_BST_4CH_TOP_GATING_CTRL1: u32 = 0x05;

/* Index (NID:20h) */
pub const RT721_JD_PRODUCT_NUM: u32 = 0x00;
pub const RT721_ANALOG_BIAS_CTL3: u32 = 0x04;
pub const RT721_JD_CTRL1: u32 = 0x09;
pub const RT721_LDO2_3_CTL1: u32 = 0x0e;
pub const RT721_GPIO_PAD_CTRL5: u32 = 0x13;
pub const RT721_LDO1_CTL: u32 = 0x1a;
pub const RT721_HP_JD_CTRL: u32 = 0x24;
pub const RT721_VD_HIDDEN_CTRL: u32 = 0x26;
pub const RT721_CLSD_CTRL6: u32 = 0x3c;
pub const RT721_COMBO_JACK_AUTO_CTL1: u32 = 0x45;
pub const RT721_COMBO_JACK_AUTO_CTL2: u32 = 0x46;
pub const RT721_COMBO_JACK_AUTO_CTL3: u32 = 0x47;
pub const RT721_DIGITAL_MISC_CTRL4: u32 = 0x4a;
pub const RT721_VREFO_GAT: u32 = 0x63;
pub const RT721_FSM_CTL: u32 = 0x67;
pub const RT721_SDCA_INTR_REC: u32 = 0x82;
pub const RT721_SW_CONFIG1: u32 = 0x8a;
pub const RT721_SW_CONFIG2: u32 = 0x8b;

/* Index (NID:40h) */
pub const RT721_RC_CALIB_CTRL0: u32 = 0x00;

/* Index (NID:58h) */
pub const RT721_DAC_DC_CALI_CTL1: u32 = 0x01;
pub const RT721_DAC_DC_CALI_CTL2: u32 = 0x02;
pub const RT721_DAC_DC_CALI_CTL3: u32 = 0x03;

/* Index (NID:5fh) */
pub const RT721_MISC_POWER_CTL0: u32 = 0x00;
pub const RT721_MISC_POWER_CTL31: u32 = 0x31;
pub const RT721_UAJ_TOP_TCON13: u32 = 0x44;
pub const RT721_UAJ_TOP_TCON14: u32 = 0x45;
pub const RT721_UAJ_TOP_TCON17: u32 = 0x48;

/* Index (NID:61h) */
pub const RT721_HDA_LEGACY_MUX_CTL0: u32 = 0x00;
pub const RT721_HDA_LEGACY_UAJ_CTL: u32 = 0x02;
pub const RT721_HDA_LEGACY_CTL1: u32 = 0x05;
pub const RT721_HDA_LEGACY_RESET_CTL: u32 = 0x06;
pub const RT721_MISC_CTL: u32 = 0x07;
pub const RT721_XU_REL_CTRL: u32 = 0x0c;
pub const RT721_GE_REL_CTRL1: u32 = 0x0d;
pub const RT721_HDA_LEGACY_GPIO_WAKE_EN_CTL: u32 = 0x0e;
pub const RT721_GE_SDCA_RST_CTRL: u32 = 0x10;
pub const RT721_INT_RST_EN_CTRL: u32 = 0x11;
pub const RT721_XU_EVENT_EN: u32 = 0x13;
pub const RT721_INLINE_CTL2: u32 = 0x17;
pub const RT721_UMP_HID_CTRL1: u32 = 0x18;
pub const RT721_UMP_HID_CTRL2: u32 = 0x19;
pub const RT721_UMP_HID_CTRL3: u32 = 0x1a;
pub const RT721_UMP_HID_CTRL4: u32 = 0x1b;
pub const RT721_UMP_HID_CTRL5: u32 = 0x1c;
pub const RT721_FUNC_FLOAT_CTL0: u32 = 0x22;
pub const RT721_FUNC_FLOAT_CTL1: u32 = 0x23;
pub const RT721_FUNC_FLOAT_CTL2: u32 = 0x24;
pub const RT721_FUNC_FLOAT_CTL3: u32 = 0x25;
pub const RT721_ENT_FLOAT_CTL0: u32 = 0x29;
pub const RT721_ENT_FLOAT_CTL1: u32 = 0x2c;
pub const RT721_ENT_FLOAT_CTL2: u32 = 0x2d;
pub const RT721_ENT_FLOAT_CTL3: u32 = 0x2e;
pub const RT721_ENT_FLOAT_CTL4: u32 = 0x2f;
pub const RT721_CH_FLOAT_CTL1: u32 = 0x45;
pub const RT721_CH_FLOAT_CTL2: u32 = 0x46;
pub const RT721_ENT_FLOAT_CTL5: u32 = 0x53;
pub const RT721_ENT_FLOAT_CTL6: u32 = 0x54;
pub const RT721_ENT_FLOAT_CTL7: u32 = 0x55;
pub const RT721_ENT_FLOAT_CTL8: u32 = 0x57;
pub const RT721_ENT_FLOAT_CTL9: u32 = 0x5a;
pub const RT721_ENT_FLOAT_CTL10: u32 = 0x5b;
pub const RT721_CH_FLOAT_CTL3: u32 = 0x6a;
pub const RT721_CH_FLOAT_CTL4: u32 = 0x6d;
pub const RT721_CH_FLOAT_CTL5: u32 = 0x70;
pub const RT721_CH_FLOAT_CTL6: u32 = 0x92;

/* Parameter & Verb control 01 (0x26)(NID:20h) */
pub const RT721_HIDDEN_REG_SW_RESET: u32 = 0x1 << 14;

/* Buffer address for HID */
pub const RT721_BUF_ADDR_HID1: u32 = 0x44030000;
pub const RT721_BUF_ADDR_HID2: u32 = 0x44030020;

/* RT721 SDCA Control - function number */
pub const FUNC_NUM_JACK_CODEC: u32 = 0x01;
pub const FUNC_NUM_MIC_ARRAY: u32 = 0x02;
pub const FUNC_NUM_HID: u32 = 0x03;
pub const FUNC_NUM_AMP: u32 = 0x04;

/* RT721 SDCA entity */
pub const RT721_SDCA_ENT_HID01: u32 = 0x01;
pub const RT721_SDCA_ENT_XUV: u32 = 0x03;
pub const RT721_SDCA_ENT_GE49: u32 = 0x49;
pub const RT721_SDCA_ENT_USER_FU05: u32 = 0x05;
pub const RT721_SDCA_ENT_USER_FU06: u32 = 0x06;
pub const RT721_SDCA_ENT_USER_FU0F: u32 = 0x0f;
pub const RT721_SDCA_ENT_USER_FU10: u32 = 0x19;
pub const RT721_SDCA_ENT_USER_FU1E: u32 = 0x1e;
pub const RT721_SDCA_ENT_FU15: u32 = 0x15;
pub const RT721_SDCA_ENT_PDE23: u32 = 0x23;
pub const RT721_SDCA_ENT_PDE40: u32 = 0x40;
pub const RT721_SDCA_ENT_PDE41: u32 = 0x41;
pub const RT721_SDCA_ENT_PDE11: u32 = 0x11;
pub const RT721_SDCA_ENT_PDE12: u32 = 0x12;
pub const RT721_SDCA_ENT_PDE2A: u32 = 0x2a;
pub const RT721_SDCA_ENT_CS01: u32 = 0x01;
pub const RT721_SDCA_ENT_CS11: u32 = 0x11;
pub const RT721_SDCA_ENT_CS1F: u32 = 0x1f;
pub const RT721_SDCA_ENT_CS1C: u32 = 0x1c;
pub const RT721_SDCA_ENT_CS31: u32 = 0x31;
pub const RT721_SDCA_ENT_OT23: u32 = 0x42;
pub const RT721_SDCA_ENT_IT26: u32 = 0x26;
pub const RT721_SDCA_ENT_IT09: u32 = 0x09;
pub const RT721_SDCA_ENT_PLATFORM_FU15: u32 = 0x15;
pub const RT721_SDCA_ENT_PLATFORM_FU44: u32 = 0x44;
pub const RT721_SDCA_ENT_XU03: u32 = 0x03;
pub const RT721_SDCA_ENT_XU0D: u32 = 0x0d;
pub const RT721_SDCA_ENT_FU55: u32 = 0x55;

/* RT721 SDCA control */
pub const RT721_SDCA_CTL_SAMPLE_FREQ_INDEX: u32 = 0x10;
pub const RT721_SDCA_CTL_FU_MUTE: u32 = 0x01;
pub const RT721_SDCA_CTL_FU_VOLUME: u32 = 0x02;
pub const RT721_SDCA_CTL_HIDTX_CURRENT_OWNER: u32 = 0x10;
pub const RT721_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: u32 = 0x11;
pub const RT721_SDCA_CTL_HIDTX_MESSAGE_OFFSET: u32 = 0x12;
pub const RT721_SDCA_CTL_HIDTX_MESSAGE_LENGTH: u32 = 0x13;
pub const RT721_SDCA_CTL_SELECTED_MODE: u32 = 0x01;
pub const RT721_SDCA_CTL_DETECTED_MODE: u32 = 0x02;
pub const RT721_SDCA_CTL_REQ_POWER_STATE: u32 = 0x01;
pub const RT721_SDCA_CTL_VENDOR_DEF: u32 = 0x30;
pub const RT721_SDCA_CTL_XUV: u32 = 0x34;
pub const RT721_SDCA_CTL_FU_CH_GAIN: u32 = 0x0b;

/* RT721 SDCA channel */
pub const CH_L: u32 = 0x01;
pub const CH_R: u32 = 0x02;
pub const CH_01: u32 = 0x01;
pub const CH_02: u32 = 0x02;
pub const CH_03: u32 = 0x03;
pub const CH_04: u32 = 0x04;
pub const CH_08: u32 = 0x08;
pub const CH_09: u32 = 0x09;
pub const CH_0A: u32 = 0x0a;

/* sample frequency index */
pub const RT721_SDCA_RATE_8000HZ: u32 = 0x01;
pub const RT721_SDCA_RATE_11025HZ: u32 = 0x02;
pub const RT721_SDCA_RATE_12000HZ: u32 = 0x03;
pub const RT721_SDCA_RATE_16000HZ: u32 = 0x04;
pub const RT721_SDCA_RATE_22050HZ: u32 = 0x05;
pub const RT721_SDCA_RATE_24000HZ: u32 = 0x06;
pub const RT721_SDCA_RATE_32000HZ: u32 = 0x07;
pub const RT721_SDCA_RATE_44100HZ: u32 = 0x08;
pub const RT721_SDCA_RATE_48000HZ: u32 = 0x09;
pub const RT721_SDCA_RATE_88200HZ: u32 = 0x0a;
pub const RT721_SDCA_RATE_96000HZ: u32 = 0x0b;
pub const RT721_SDCA_RATE_176400HZ: u32 = 0x0c;
pub const RT721_SDCA_RATE_192000HZ: u32 = 0x0d;
pub const RT721_SDCA_RATE_384000HZ: u32 = 0x0e;
pub const RT721_SDCA_RATE_768000HZ: u32 = 0x0f;

/* RT721 HID ID */
pub const RT721_SDCA_HID_ID: u32 = 0x11;

pub const RT721_AIF1: u32 = 0; /* For headset mic and headphone */
pub const RT721_AIF2: u32 = 1; /* For speaker */
pub const RT721_AIF3: u32 = 2; /* For dmic */
pub const RT721_AIFS: u32 = 3;

unsafe extern "C" {
    pub fn rt721_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> ::core::ffi::c_int;
    pub fn rt721_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        mbq_regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
