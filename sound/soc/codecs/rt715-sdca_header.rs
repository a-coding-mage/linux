/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt715-sdca.h -- RT715 ALSA SoC audio driver header
 *
 * Copyright(c) 2020 Realtek Semiconductor Corp.
 */

/* C header dependencies:
 * linux/regmap.h
 * linux/soundwire/sdw.h
 * linux/soundwire/sdw_type.h
 * sound/soc.h
 * linux/workqueue.h
 * linux/device.h
 */

#[repr(C)]
pub struct rt715_sdca_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub codec: *mut snd_soc_codec,
    pub slave: *mut sdw_slave,
    pub adc_mute_work: delayed_work,
    pub dbg_nid: core::ffi::c_int,
    pub dbg_vid: core::ffi::c_int,
    pub dbg_payload: core::ffi::c_int,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub l_is_unmute: core::ffi::c_int,
    pub r_is_unmute: core::ffi::c_int,
    pub hw_sdw_ver: core::ffi::c_int,
    pub kctl_switch_orig: [core::ffi::c_int; 4],
    pub kctl_2ch_orig: [core::ffi::c_int; 2],
    pub kctl_4ch_orig: [core::ffi::c_int; 4],
    pub kctl_8ch_orig: [core::ffi::c_int; 8],
}

#[repr(C)]
pub struct rt715_sdca_kcontrol_private {
    pub reg_base: core::ffi::c_uint,
    pub count: core::ffi::c_uint,
    pub max: core::ffi::c_uint,
    pub shift: core::ffi::c_uint,
    pub invert: core::ffi::c_uint,
}

/* MIPI Register */
pub const RT715_INT_CTRL: core::ffi::c_uint = 0x005a;
pub const RT715_INT_MASK: core::ffi::c_uint = 0x005e;

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
pub const RT715_VENDOR_REG: core::ffi::c_uint = 0x20;
pub const RT715_MUX_IN1: core::ffi::c_uint = 0x22;
pub const RT715_MUX_IN2: core::ffi::c_uint = 0x23;
pub const RT715_MUX_IN3: core::ffi::c_uint = 0x24;
pub const RT715_MUX_IN4: core::ffi::c_uint = 0x25;
pub const RT715_MIX_ADC2: core::ffi::c_uint = 0x27;
pub const RT715_INLINE_CMD: core::ffi::c_uint = 0x55;
pub const RT715_VENDOR_HDA_CTL: core::ffi::c_uint = 0x61;

/* Index (NID:20h) */
pub const RT715_PRODUCT_NUM: core::ffi::c_uint = 0x0;
pub const RT715_IRQ_CTRL: core::ffi::c_uint = 0x2b;
pub const RT715_AD_FUNC_EN: core::ffi::c_uint = 0x36;
pub const RT715_REV_1: core::ffi::c_uint = 0x37;
pub const RT715_SDW_INPUT_SEL: core::ffi::c_uint = 0x39;
pub const RT715_DFLL_VAD: core::ffi::c_uint = 0x44;
pub const RT715_EXT_DMIC_CLK_CTRL2: core::ffi::c_uint = 0x54;

/* Index (NID:61h) */
pub const RT715_HDA_LEGACY_MUX_CTL1: core::ffi::c_uint = 0x00;

/* SDCA (Function) */
pub const FUN_JACK_CODEC: core::ffi::c_uint = 0x01;
pub const FUN_MIC_ARRAY: core::ffi::c_uint = 0x02;
pub const FUN_HID: core::ffi::c_uint = 0x03;
/* SDCA (Entity) */
pub const RT715_SDCA_ST_EN: core::ffi::c_uint = 0x00;
pub const RT715_SDCA_CS_FREQ_IND_EN: core::ffi::c_uint = 0x01;
pub const RT715_SDCA_FU_ADC8_9_VOL: core::ffi::c_uint = 0x02;
pub const RT715_SDCA_SMPU_TRIG_ST_EN: core::ffi::c_uint = 0x05;
pub const RT715_SDCA_FU_ADC10_11_VOL: core::ffi::c_uint = 0x06;
pub const RT715_SDCA_FU_ADC7_27_VOL: core::ffi::c_uint = 0x0a;
pub const RT715_SDCA_FU_AMIC_GAIN_EN: core::ffi::c_uint = 0x0c;
pub const RT715_SDCA_FU_DMIC_GAIN_EN: core::ffi::c_uint = 0x0e;
pub const RT715_SDCA_CX_CLK_SEL_EN: core::ffi::c_uint = 0x10;
pub const RT715_SDCA_CREQ_POW_EN: core::ffi::c_uint = 0x18;
/* SDCA (Control) */
pub const RT715_SDCA_ST_CTRL: core::ffi::c_uint = 0x00;
pub const RT715_SDCA_CX_CLK_SEL_CTRL: core::ffi::c_uint = 0x01;
pub const RT715_SDCA_REQ_POW_CTRL: core::ffi::c_uint = 0x01;
pub const RT715_SDCA_FU_MUTE_CTRL: core::ffi::c_uint = 0x01;
pub const RT715_SDCA_FU_VOL_CTRL: core::ffi::c_uint = 0x02;
pub const RT715_SDCA_FU_DMIC_GAIN_CTRL: core::ffi::c_uint = 0x0b;
pub const RT715_SDCA_FREQ_IND_CTRL: core::ffi::c_uint = 0x10;
pub const RT715_SDCA_SMPU_TRIG_EN_CTRL: core::ffi::c_uint = 0x10;
pub const RT715_SDCA_SMPU_TRIG_ST_CTRL: core::ffi::c_uint = 0x11;
/* SDCA (Channel) */
pub const CH_00: core::ffi::c_uint = 0x00;
pub const CH_01: core::ffi::c_uint = 0x01;
pub const CH_02: core::ffi::c_uint = 0x02;
pub const CH_03: core::ffi::c_uint = 0x03;
pub const CH_04: core::ffi::c_uint = 0x04;
pub const CH_05: core::ffi::c_uint = 0x05;
pub const CH_06: core::ffi::c_uint = 0x06;
pub const CH_07: core::ffi::c_uint = 0x07;
pub const CH_08: core::ffi::c_uint = 0x08;

pub const RT715_SDCA_DB_STEP: core::ffi::c_uint = 375;

pub const RT715_AIF1: core::ffi::c_int = 0;
pub const RT715_AIF2: core::ffi::c_int = 1;

unsafe extern "C" {
    pub fn rt715_sdca_io_init(
        dev: *mut device,
        slave: *mut sdw_slave,
    ) -> core::ffi::c_int;
    pub fn rt715_sdca_init(
        dev: *mut device,
        mbq_regmap: *mut regmap,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
