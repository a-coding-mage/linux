// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ak4671.rs  --  audio driver for AK4671
 *
 * Copyright (C) 2009 Samsung Electronics Co.Ltd
 * Author: Joonyoung Shim <jy0922.shim@samsung.com>
 *
 * Rust translation of soc/codecs/ak4671.c.
 */

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

// Constants and construction macros are supplied by Linux/ASoC headers and ak4671.h.

/* ak4671 register cache & default register settings */
static ak4671_reg_defaults: [reg_default; 90] = [
    reg_default { reg: 0x00, def: 0x00 }, /* AK4671_AD_DA_POWER_MANAGEMENT        (0x00) */
    reg_default { reg: 0x01, def: 0xf6 }, /* AK4671_PLL_MODE_SELECT0              (0x01) */
    reg_default { reg: 0x02, def: 0x00 }, /* AK4671_PLL_MODE_SELECT1              (0x02) */
    reg_default { reg: 0x03, def: 0x02 }, /* AK4671_FORMAT_SELECT                 (0x03) */
    reg_default { reg: 0x04, def: 0x00 }, /* AK4671_MIC_SIGNAL_SELECT             (0x04) */
    reg_default { reg: 0x05, def: 0x55 }, /* AK4671_MIC_AMP_GAIN                  (0x05) */
    reg_default { reg: 0x06, def: 0x00 }, /* AK4671_MIXING_POWER_MANAGEMENT0      (0x06) */
    reg_default { reg: 0x07, def: 0x00 }, /* AK4671_MIXING_POWER_MANAGEMENT1      (0x07) */
    reg_default { reg: 0x08, def: 0xb5 }, /* AK4671_OUTPUT_VOLUME_CONTROL         (0x08) */
    reg_default { reg: 0x09, def: 0x00 }, /* AK4671_LOUT1_SIGNAL_SELECT           (0x09) */
    reg_default { reg: 0x0a, def: 0x00 }, /* AK4671_ROUT1_SIGNAL_SELECT           (0x0a) */
    reg_default { reg: 0x0b, def: 0x00 }, /* AK4671_LOUT2_SIGNAL_SELECT           (0x0b) */
    reg_default { reg: 0x0c, def: 0x00 }, /* AK4671_ROUT2_SIGNAL_SELECT           (0x0c) */
    reg_default { reg: 0x0d, def: 0x00 }, /* AK4671_LOUT3_SIGNAL_SELECT           (0x0d) */
    reg_default { reg: 0x0e, def: 0x00 }, /* AK4671_ROUT3_SIGNAL_SELECT           (0x0e) */
    reg_default { reg: 0x0f, def: 0x00 }, /* AK4671_LOUT1_POWER_MANAGERMENT       (0x0f) */
    reg_default { reg: 0x10, def: 0x00 }, /* AK4671_LOUT2_POWER_MANAGERMENT       (0x10) */
    reg_default { reg: 0x11, def: 0x80 }, /* AK4671_LOUT3_POWER_MANAGERMENT       (0x11) */
    reg_default { reg: 0x12, def: 0x91 }, /* AK4671_LCH_INPUT_VOLUME_CONTROL      (0x12) */
    reg_default { reg: 0x13, def: 0x91 }, /* AK4671_RCH_INPUT_VOLUME_CONTROL      (0x13) */
    reg_default { reg: 0x14, def: 0xe1 }, /* AK4671_ALC_REFERENCE_SELECT          (0x14) */
    reg_default { reg: 0x15, def: 0x00 }, /* AK4671_DIGITAL_MIXING_CONTROL        (0x15) */
    reg_default { reg: 0x16, def: 0x00 }, /* AK4671_ALC_TIMER_SELECT              (0x16) */
    reg_default { reg: 0x17, def: 0x00 }, /* AK4671_ALC_MODE_CONTROL              (0x17) */
    reg_default { reg: 0x18, def: 0x02 }, /* AK4671_MODE_CONTROL1                 (0x18) */
    reg_default { reg: 0x19, def: 0x01 }, /* AK4671_MODE_CONTROL2                 (0x19) */
    reg_default { reg: 0x1a, def: 0x18 }, /* AK4671_LCH_OUTPUT_VOLUME_CONTROL     (0x1a) */
    reg_default { reg: 0x1b, def: 0x18 }, /* AK4671_RCH_OUTPUT_VOLUME_CONTROL     (0x1b) */
    reg_default { reg: 0x1c, def: 0x00 }, /* AK4671_SIDETONE_A_CONTROL            (0x1c) */
    reg_default { reg: 0x1d, def: 0x02 }, /* AK4671_DIGITAL_FILTER_SELECT         (0x1d) */
    reg_default { reg: 0x1e, def: 0x00 }, /* AK4671_FIL3_COEFFICIENT0             (0x1e) */
    reg_default { reg: 0x1f, def: 0x00 }, /* AK4671_FIL3_COEFFICIENT1             (0x1f) */
    reg_default { reg: 0x20, def: 0x00 }, /* AK4671_FIL3_COEFFICIENT2             (0x20) */
    reg_default { reg: 0x21, def: 0x00 }, /* AK4671_FIL3_COEFFICIENT3             (0x21) */
    reg_default { reg: 0x22, def: 0x00 }, /* AK4671_EQ_COEFFICIENT0               (0x22) */
    reg_default { reg: 0x23, def: 0x00 }, /* AK4671_EQ_COEFFICIENT1               (0x23) */
    reg_default { reg: 0x24, def: 0x00 }, /* AK4671_EQ_COEFFICIENT2               (0x24) */
    reg_default { reg: 0x25, def: 0x00 }, /* AK4671_EQ_COEFFICIENT3               (0x25) */
    reg_default { reg: 0x26, def: 0x00 }, /* AK4671_EQ_COEFFICIENT4               (0x26) */
    reg_default { reg: 0x27, def: 0x00 }, /* AK4671_EQ_COEFFICIENT5               (0x27) */
    reg_default { reg: 0x28, def: 0xa9 }, /* AK4671_FIL1_COEFFICIENT0             (0x28) */
    reg_default { reg: 0x29, def: 0x1f }, /* AK4671_FIL1_COEFFICIENT1             (0x29) */
    reg_default { reg: 0x2a, def: 0xad }, /* AK4671_FIL1_COEFFICIENT2             (0x2a) */
    reg_default { reg: 0x2b, def: 0x20 }, /* AK4671_FIL1_COEFFICIENT3             (0x2b) */
    reg_default { reg: 0x2c, def: 0x00 }, /* AK4671_FIL2_COEFFICIENT0             (0x2c) */
    reg_default { reg: 0x2d, def: 0x00 }, /* AK4671_FIL2_COEFFICIENT1             (0x2d) */
    reg_default { reg: 0x2e, def: 0x00 }, /* AK4671_FIL2_COEFFICIENT2             (0x2e) */
    reg_default { reg: 0x2f, def: 0x00 }, /* AK4671_FIL2_COEFFICIENT3             (0x2f) */
    reg_default { reg: 0x30, def: 0x00 }, /* AK4671_DIGITAL_FILTER_SELECT2        (0x30) */
    reg_default { reg: 0x32, def: 0x00 }, /* AK4671_E1_COEFFICIENT0               (0x32) */
    reg_default { reg: 0x33, def: 0x00 }, /* AK4671_E1_COEFFICIENT1               (0x33) */
    reg_default { reg: 0x34, def: 0x00 }, /* AK4671_E1_COEFFICIENT2               (0x34) */
    reg_default { reg: 0x35, def: 0x00 }, /* AK4671_E1_COEFFICIENT3               (0x35) */
    reg_default { reg: 0x36, def: 0x00 }, /* AK4671_E1_COEFFICIENT4               (0x36) */
    reg_default { reg: 0x37, def: 0x00 }, /* AK4671_E1_COEFFICIENT5               (0x37) */
    reg_default { reg: 0x38, def: 0x00 }, /* AK4671_E2_COEFFICIENT0               (0x38) */
    reg_default { reg: 0x39, def: 0x00 }, /* AK4671_E2_COEFFICIENT1               (0x39) */
    reg_default { reg: 0x3a, def: 0x00 }, /* AK4671_E2_COEFFICIENT2               (0x3a) */
    reg_default { reg: 0x3b, def: 0x00 }, /* AK4671_E2_COEFFICIENT3               (0x3b) */
    reg_default { reg: 0x3c, def: 0x00 }, /* AK4671_E2_COEFFICIENT4               (0x3c) */
    reg_default { reg: 0x3d, def: 0x00 }, /* AK4671_E2_COEFFICIENT5               (0x3d) */
    reg_default { reg: 0x3e, def: 0x00 }, /* AK4671_E3_COEFFICIENT0               (0x3e) */
    reg_default { reg: 0x3f, def: 0x00 }, /* AK4671_E3_COEFFICIENT1               (0x3f) */
    reg_default { reg: 0x40, def: 0x00 }, /* AK4671_E3_COEFFICIENT2               (0x40) */
    reg_default { reg: 0x41, def: 0x00 }, /* AK4671_E3_COEFFICIENT3               (0x41) */
    reg_default { reg: 0x42, def: 0x00 }, /* AK4671_E3_COEFFICIENT4               (0x42) */
    reg_default { reg: 0x43, def: 0x00 }, /* AK4671_E3_COEFFICIENT5               (0x43) */
    reg_default { reg: 0x44, def: 0x00 }, /* AK4671_E4_COEFFICIENT0               (0x44) */
    reg_default { reg: 0x45, def: 0x00 }, /* AK4671_E4_COEFFICIENT1               (0x45) */
    reg_default { reg: 0x46, def: 0x00 }, /* AK4671_E4_COEFFICIENT2               (0x46) */
    reg_default { reg: 0x47, def: 0x00 }, /* AK4671_E4_COEFFICIENT3               (0x47) */
    reg_default { reg: 0x48, def: 0x00 }, /* AK4671_E4_COEFFICIENT4               (0x48) */
    reg_default { reg: 0x49, def: 0x00 }, /* AK4671_E4_COEFFICIENT5               (0x49) */
    reg_default { reg: 0x4a, def: 0x00 }, /* AK4671_E5_COEFFICIENT0               (0x4a) */
    reg_default { reg: 0x4b, def: 0x00 }, /* AK4671_E5_COEFFICIENT1               (0x4b) */
    reg_default { reg: 0x4c, def: 0x00 }, /* AK4671_E5_COEFFICIENT2               (0x4c) */
    reg_default { reg: 0x4d, def: 0x00 }, /* AK4671_E5_COEFFICIENT3               (0x4d) */
    reg_default { reg: 0x4e, def: 0x00 }, /* AK4671_E5_COEFFICIENT4               (0x4e) */
    reg_default { reg: 0x4f, def: 0x00 }, /* AK4671_E5_COEFFICIENT5               (0x4f) */
    reg_default { reg: 0x50, def: 0x88 }, /* AK4671_EQ_CONTROL_250HZ_100HZ        (0x50) */
    reg_default { reg: 0x51, def: 0x88 }, /* AK4671_EQ_CONTROL_3500HZ_1KHZ        (0x51) */
    reg_default { reg: 0x52, def: 0x08 }, /* AK4671_EQ_CONTRO_10KHZ               (0x52) */
    reg_default { reg: 0x53, def: 0x00 }, /* AK4671_PCM_IF_CONTROL0               (0x53) */
    reg_default { reg: 0x54, def: 0x00 }, /* AK4671_PCM_IF_CONTROL1               (0x54) */
    reg_default { reg: 0x55, def: 0x00 }, /* AK4671_PCM_IF_CONTROL2               (0x55) */
    reg_default { reg: 0x56, def: 0x18 }, /* AK4671_DIGITAL_VOLUME_B_CONTROL      (0x56) */
    reg_default { reg: 0x57, def: 0x18 }, /* AK4671_DIGITAL_VOLUME_C_CONTROL      (0x57) */
    reg_default { reg: 0x58, def: 0x00 }, /* AK4671_SIDETONE_VOLUME_CONTROL       (0x58) */
    reg_default { reg: 0x59, def: 0x00 }, /* AK4671_DIGITAL_MIXING_CONTROL2       (0x59) */
    reg_default { reg: 0x5a, def: 0x00 }, /* AK4671_SAR_ADC_CONTROL               (0x5a) */
];

/*
 * LOUT1/ROUT1 output volume control:
 * from -24 to 6 dB in 6 dB steps (mute instead of -30 dB)
 */
static out1_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-3000, 600, 1);

/*
 * LOUT2/ROUT2 output volume control:
 * from -33 to 6 dB in 3 dB steps (mute instead of -33 dB)
 */
static out2_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-3300, 300, 1);

/*
 * LOUT3/ROUT3 output volume control:
 * from -6 to 3 dB in 3 dB steps
 */
static out3_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-600, 300, 0);

/*
 * Mic amp gain control:
 * from -15 to 30 dB in 3 dB steps
 * REVISIT: The actual min value(0x01) is -12 dB and the reg value 0x00 is not
 * available
 */
static mic_amp_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-1500, 300, 0);

static ak4671_snd_controls: [snd_kcontrol_new; 4] = [
    SOC_SINGLE_TLV!("Line Output1 Playback Volume", AK4671_OUTPUT_VOLUME_CONTROL, 0, 0x6, 0, out1_tlv),
    SOC_SINGLE_TLV!("Headphone Output2 Playback Volume", AK4671_OUTPUT_VOLUME_CONTROL, 4, 0xd, 0, out2_tlv),
    SOC_SINGLE_TLV!("Line Output3 Playback Volume", AK4671_LOUT3_POWER_MANAGERMENT, 6, 0x3, 0, out3_tlv),
    SOC_DOUBLE_TLV!("Mic Amp Capture Volume", AK4671_MIC_AMP_GAIN, 0, 4, 0xf, 0, mic_amp_tlv),
];

/* event handlers */
unsafe extern "C" fn ak4671_out2_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_update_bits(
                component,
                AK4671_LOUT2_POWER_MANAGERMENT,
                AK4671_MUTEN,
                AK4671_MUTEN,
            );
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, AK4671_LOUT2_POWER_MANAGERMENT, AK4671_MUTEN, 0);
        }
        _ => {}
    }

    0
}

/* Output Mixers */
static ak4671_lout1_mixer_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("DACL", AK4671_LOUT1_SIGNAL_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("LINL1", AK4671_LOUT1_SIGNAL_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("LINL2", AK4671_LOUT1_SIGNAL_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("LINL3", AK4671_LOUT1_SIGNAL_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("LINL4", AK4671_LOUT1_SIGNAL_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("LOOPL", AK4671_LOUT1_SIGNAL_SELECT, 5, 1, 0),
];

static ak4671_rout1_mixer_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("DACR", AK4671_ROUT1_SIGNAL_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("RINR1", AK4671_ROUT1_SIGNAL_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("RINR2", AK4671_ROUT1_SIGNAL_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("RINR3", AK4671_ROUT1_SIGNAL_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("RINR4", AK4671_ROUT1_SIGNAL_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("LOOPR", AK4671_ROUT1_SIGNAL_SELECT, 5, 1, 0),
];

static ak4671_lout2_mixer_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("DACHL", AK4671_LOUT2_SIGNAL_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("LINH1", AK4671_LOUT2_SIGNAL_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("LINH2", AK4671_LOUT2_SIGNAL_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("LINH3", AK4671_LOUT2_SIGNAL_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("LINH4", AK4671_LOUT2_SIGNAL_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("LOOPHL", AK4671_LOUT2_SIGNAL_SELECT, 5, 1, 0),
];

static ak4671_rout2_mixer_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("DACHR", AK4671_ROUT2_SIGNAL_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("RINH1", AK4671_ROUT2_SIGNAL_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("RINH2", AK4671_ROUT2_SIGNAL_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("RINH3", AK4671_ROUT2_SIGNAL_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("RINH4", AK4671_ROUT2_SIGNAL_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("LOOPHR", AK4671_ROUT2_SIGNAL_SELECT, 5, 1, 0),
];

static ak4671_lout3_mixer_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("DACSL", AK4671_LOUT3_SIGNAL_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("LINS1", AK4671_LOUT3_SIGNAL_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("LINS2", AK4671_LOUT3_SIGNAL_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("LINS3", AK4671_LOUT3_SIGNAL_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("LINS4", AK4671_LOUT3_SIGNAL_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("LOOPSL", AK4671_LOUT3_SIGNAL_SELECT, 5, 1, 0),
];

static ak4671_rout3_mixer_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("DACSR", AK4671_ROUT3_SIGNAL_SELECT, 0, 1, 0),
    SOC_DAPM_SINGLE!("RINS1", AK4671_ROUT3_SIGNAL_SELECT, 1, 1, 0),
    SOC_DAPM_SINGLE!("RINS2", AK4671_ROUT3_SIGNAL_SELECT, 2, 1, 0),
    SOC_DAPM_SINGLE!("RINS3", AK4671_ROUT3_SIGNAL_SELECT, 3, 1, 0),
    SOC_DAPM_SINGLE!("RINS4", AK4671_ROUT3_SIGNAL_SELECT, 4, 1, 0),
    SOC_DAPM_SINGLE!("LOOPSR", AK4671_ROUT3_SIGNAL_SELECT, 5, 1, 0),
];

/* Input MUXs */
static ak4671_lin_mux_texts: [*const c_char; 4] = [c"LIN1".as_ptr(), c"LIN2".as_ptr(), c"LIN3".as_ptr(), c"LIN4".as_ptr()];
static ak4671_lin_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(AK4671_MIC_SIGNAL_SELECT, 0, ak4671_lin_mux_texts);
static ak4671_lin_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", ak4671_lin_mux_enum);

static ak4671_rin_mux_texts: [*const c_char; 4] = [c"RIN1".as_ptr(), c"RIN2".as_ptr(), c"RIN3".as_ptr(), c"RIN4".as_ptr()];
static ak4671_rin_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(AK4671_MIC_SIGNAL_SELECT, 2, ak4671_rin_mux_texts);
static ak4671_rin_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", ak4671_rin_mux_enum);

static ak4671_dapm_widgets: [snd_soc_dapm_widget; 35] = [
    SND_SOC_DAPM_INPUT!("LIN1"),
    SND_SOC_DAPM_INPUT!("RIN1"),
    SND_SOC_DAPM_INPUT!("LIN2"),
    SND_SOC_DAPM_INPUT!("RIN2"),
    SND_SOC_DAPM_INPUT!("LIN3"),
    SND_SOC_DAPM_INPUT!("RIN3"),
    SND_SOC_DAPM_INPUT!("LIN4"),
    SND_SOC_DAPM_INPUT!("RIN4"),
    SND_SOC_DAPM_OUTPUT!("LOUT1"),
    SND_SOC_DAPM_OUTPUT!("ROUT1"),
    SND_SOC_DAPM_OUTPUT!("LOUT2"),
    SND_SOC_DAPM_OUTPUT!("ROUT2"),
    SND_SOC_DAPM_OUTPUT!("LOUT3"),
    SND_SOC_DAPM_OUTPUT!("ROUT3"),
    SND_SOC_DAPM_DAC!("DAC Left", "Left HiFi Playback", AK4671_AD_DA_POWER_MANAGEMENT, 6, 0),
    SND_SOC_DAPM_DAC!("DAC Right", "Right HiFi Playback", AK4671_AD_DA_POWER_MANAGEMENT, 7, 0),
    SND_SOC_DAPM_ADC!("ADC Left", "Left HiFi Capture", AK4671_AD_DA_POWER_MANAGEMENT, 4, 0),
    SND_SOC_DAPM_ADC!("ADC Right", "Right HiFi Capture", AK4671_AD_DA_POWER_MANAGEMENT, 5, 0),
    SND_SOC_DAPM_PGA!("LOUT2 Mix Amp", AK4671_LOUT2_POWER_MANAGERMENT, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("ROUT2 Mix Amp", AK4671_LOUT2_POWER_MANAGERMENT, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("LIN1 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("RIN1 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 1, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("LIN2 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("RIN2 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("LIN3 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("RIN3 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("LIN4 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("RIN4 Mixing Circuit", AK4671_MIXING_POWER_MANAGEMENT1, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("LOUT1 Mixer", AK4671_LOUT1_POWER_MANAGERMENT, 0, 0, &ak4671_lout1_mixer_controls[0], ak4671_lout1_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("ROUT1 Mixer", AK4671_LOUT1_POWER_MANAGERMENT, 1, 0, &ak4671_rout1_mixer_controls[0], ak4671_rout1_mixer_controls.len()),
    SND_SOC_DAPM_MIXER_E!("LOUT2 Mixer", AK4671_LOUT2_POWER_MANAGERMENT, 0, 0, &ak4671_lout2_mixer_controls[0], ak4671_lout2_mixer_controls.len(), ak4671_out2_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MIXER_E!("ROUT2 Mixer", AK4671_LOUT2_POWER_MANAGERMENT, 1, 0, &ak4671_rout2_mixer_controls[0], ak4671_rout2_mixer_controls.len(), ak4671_out2_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MIXER!("LOUT3 Mixer", AK4671_LOUT3_POWER_MANAGERMENT, 0, 0, &ak4671_lout3_mixer_controls[0], ak4671_lout3_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("ROUT3 Mixer", AK4671_LOUT3_POWER_MANAGERMENT, 1, 0, &ak4671_rout3_mixer_controls[0], ak4671_rout3_mixer_controls.len()),
    SND_SOC_DAPM_MUX!("LIN MUX", AK4671_AD_DA_POWER_MANAGEMENT, 2, 0, &ak4671_lin_mux_control),
    SND_SOC_DAPM_MUX!("RIN MUX", AK4671_AD_DA_POWER_MANAGEMENT, 3, 0, &ak4671_rin_mux_control),
    SND_SOC_DAPM_MICBIAS!("Mic Bias", AK4671_AD_DA_POWER_MANAGEMENT, 1, 0),
    SND_SOC_DAPM_SUPPLY!("PMPLL", AK4671_PLL_MODE_SELECT1, 0, 0, core::ptr::null(), 0),
];

static ak4671_intercon: [snd_soc_dapm_route; 66] = [
    route!("DAC Left", core::ptr::null(), "PMPLL"),
    route!("DAC Right", core::ptr::null(), "PMPLL"),
    route!("ADC Left", core::ptr::null(), "PMPLL"),
    route!("ADC Right", core::ptr::null(), "PMPLL"),
    route!("LOUT1", core::ptr::null(), "LOUT1 Mixer"),
    route!("ROUT1", core::ptr::null(), "ROUT1 Mixer"),
    route!("LOUT2", core::ptr::null(), "LOUT2 Mix Amp"),
    route!("ROUT2", core::ptr::null(), "ROUT2 Mix Amp"),
    route!("LOUT3", core::ptr::null(), "LOUT3 Mixer"),
    route!("ROUT3", core::ptr::null(), "ROUT3 Mixer"),
    route!("LOUT1 Mixer", "DACL", "DAC Left"),
    route!("ROUT1 Mixer", "DACR", "DAC Right"),
    route!("LOUT2 Mixer", "DACHL", "DAC Left"),
    route!("ROUT2 Mixer", "DACHR", "DAC Right"),
    route!("LOUT2 Mix Amp", core::ptr::null(), "LOUT2 Mixer"),
    route!("ROUT2 Mix Amp", core::ptr::null(), "ROUT2 Mixer"),
    route!("LOUT3 Mixer", "DACSL", "DAC Left"),
    route!("ROUT3 Mixer", "DACSR", "DAC Right"),
    route!("LIN MUX", "LIN1", "LIN1"),
    route!("LIN MUX", "LIN2", "LIN2"),
    route!("LIN MUX", "LIN3", "LIN3"),
    route!("LIN MUX", "LIN4", "LIN4"),
    route!("RIN MUX", "RIN1", "RIN1"),
    route!("RIN MUX", "RIN2", "RIN2"),
    route!("RIN MUX", "RIN3", "RIN3"),
    route!("RIN MUX", "RIN4", "RIN4"),
    route!("LIN1", core::ptr::null(), "Mic Bias"),
    route!("RIN1", core::ptr::null(), "Mic Bias"),
    route!("LIN2", core::ptr::null(), "Mic Bias"),
    route!("RIN2", core::ptr::null(), "Mic Bias"),
    route!("ADC Left", core::ptr::null(), "LIN MUX"),
    route!("ADC Right", core::ptr::null(), "RIN MUX"),
    route!("LIN1 Mixing Circuit", core::ptr::null(), "LIN1"),
    route!("RIN1 Mixing Circuit", core::ptr::null(), "RIN1"),
    route!("LIN2 Mixing Circuit", core::ptr::null(), "LIN2"),
    route!("RIN2 Mixing Circuit", core::ptr::null(), "RIN2"),
    route!("LIN3 Mixing Circuit", core::ptr::null(), "LIN3"),
    route!("RIN3 Mixing Circuit", core::ptr::null(), "RIN3"),
    route!("LIN4 Mixing Circuit", core::ptr::null(), "LIN4"),
    route!("RIN4 Mixing Circuit", core::ptr::null(), "RIN4"),
    route!("LOUT1 Mixer", "LINL1", "LIN1 Mixing Circuit"),
    route!("ROUT1 Mixer", "RINR1", "RIN1 Mixing Circuit"),
    route!("LOUT2 Mixer", "LINH1", "LIN1 Mixing Circuit"),
    route!("ROUT2 Mixer", "RINH1", "RIN1 Mixing Circuit"),
    route!("LOUT3 Mixer", "LINS1", "LIN1 Mixing Circuit"),
    route!("ROUT3 Mixer", "RINS1", "RIN1 Mixing Circuit"),
    route!("LOUT1 Mixer", "LINL2", "LIN2 Mixing Circuit"),
    route!("ROUT1 Mixer", "RINR2", "RIN2 Mixing Circuit"),
    route!("LOUT2 Mixer", "LINH2", "LIN2 Mixing Circuit"),
    route!("ROUT2 Mixer", "RINH2", "RIN2 Mixing Circuit"),
    route!("LOUT3 Mixer", "LINS2", "LIN2 Mixing Circuit"),
    route!("ROUT3 Mixer", "RINS2", "RIN2 Mixing Circuit"),
    route!("LOUT1 Mixer", "LINL3", "LIN3 Mixing Circuit"),
    route!("ROUT1 Mixer", "RINR3", "RIN3 Mixing Circuit"),
    route!("LOUT2 Mixer", "LINH3", "LIN3 Mixing Circuit"),
    route!("ROUT2 Mixer", "RINH3", "RIN3 Mixing Circuit"),
    route!("LOUT3 Mixer", "LINS3", "LIN3 Mixing Circuit"),
    route!("ROUT3 Mixer", "RINS3", "RIN3 Mixing Circuit"),
    route!("LOUT1 Mixer", "LINL4", "LIN4 Mixing Circuit"),
    route!("ROUT1 Mixer", "RINR4", "RIN4 Mixing Circuit"),
    route!("LOUT2 Mixer", "LINH4", "LIN4 Mixing Circuit"),
    route!("ROUT2 Mixer", "RINH4", "RIN4 Mixing Circuit"),
    route!("LOUT3 Mixer", "LINS4", "LIN4 Mixing Circuit"),
    route!("ROUT3 Mixer", "RINS4", "RIN4 Mixing Circuit"),
];

unsafe extern "C" fn ak4671_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mut fs: u8;

    fs = snd_soc_component_read(component, AK4671_PLL_MODE_SELECT0) as u8;
    fs &= !(AK4671_FS as u8);

    match params_rate(params) {
        8000 => fs |= AK4671_FS_8KHZ as u8,
        12000 => fs |= AK4671_FS_12KHZ as u8,
        16000 => fs |= AK4671_FS_16KHZ as u8,
        24000 => fs |= AK4671_FS_24KHZ as u8,
        11025 => fs |= AK4671_FS_11_025KHZ as u8,
        22050 => fs |= AK4671_FS_22_05KHZ as u8,
        32000 => fs |= AK4671_FS_32KHZ as u8,
        44100 => fs |= AK4671_FS_44_1KHZ as u8,
        48000 => fs |= AK4671_FS_48KHZ as u8,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, AK4671_PLL_MODE_SELECT0, fs as c_uint);

    0
}

unsafe extern "C" fn ak4671_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let mut pll: u8;

    pll = snd_soc_component_read(component, AK4671_PLL_MODE_SELECT0) as u8;
    pll &= !(AK4671_PLL as u8);

    match freq {
        11289600 => pll |= AK4671_PLL_11_2896MHZ as u8,
        12000000 => pll |= AK4671_PLL_12MHZ as u8,
        12288000 => pll |= AK4671_PLL_12_288MHZ as u8,
        13000000 => pll |= AK4671_PLL_13MHZ as u8,
        13500000 => pll |= AK4671_PLL_13_5MHZ as u8,
        19200000 => pll |= AK4671_PLL_19_2MHZ as u8,
        24000000 => pll |= AK4671_PLL_24MHZ as u8,
        26000000 => pll |= AK4671_PLL_26MHZ as u8,
        27000000 => pll |= AK4671_PLL_27MHZ as u8,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, AK4671_PLL_MODE_SELECT0, pll as c_uint);

    0
}

unsafe extern "C" fn ak4671_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut mode: u8;
    let mut format: u8;

    /* set master/slave audio interface */
    mode = snd_soc_component_read(component, AK4671_PLL_MODE_SELECT1) as u8;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => mode |= AK4671_M_S as u8,
        SND_SOC_DAIFMT_CBP_CFC => mode &= !(AK4671_M_S as u8),
        _ => return -EINVAL,
    }

    /* interface format */
    format = snd_soc_component_read(component, AK4671_FORMAT_SELECT) as u8;
    format &= !(AK4671_DIF as u8);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => format |= AK4671_DIF_I2S_MODE as u8,
        SND_SOC_DAIFMT_LEFT_J => format |= AK4671_DIF_MSB_MODE as u8,
        SND_SOC_DAIFMT_DSP_A => {
            format |= AK4671_DIF_DSP_MODE as u8;
            format |= AK4671_BCKP as u8;
            format |= AK4671_MSBS as u8;
        }
        _ => return -EINVAL,
    }

    /* set mode and format */
    snd_soc_component_write(component, AK4671_PLL_MODE_SELECT1, mode as c_uint);
    snd_soc_component_write(component, AK4671_FORMAT_SELECT, format as c_uint);

    0
}

unsafe extern "C" fn ak4671_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON
        | snd_soc_bias_level::SND_SOC_BIAS_PREPARE
        | snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            snd_soc_component_update_bits(
                component,
                AK4671_AD_DA_POWER_MANAGEMENT,
                AK4671_PMVCM,
                AK4671_PMVCM,
            );
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, AK4671_AD_DA_POWER_MANAGEMENT, 0x00);
        }
    }
    0
}

const AK4671_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;

const AK4671_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE;

static ak4671_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ak4671_hw_params),
    set_sysclk: Some(ak4671_set_dai_sysclk),
    set_fmt: Some(ak4671_set_dai_fmt),
};

static mut ak4671_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak4671-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: AK4671_RATES,
        formats: AK4671_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: AK4671_RATES,
        formats: AK4671_FORMATS,
    },
    ops: &ak4671_dai_ops,
};

static soc_component_dev_ak4671: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(ak4671_set_bias_level),
    controls: ak4671_snd_controls.as_ptr(),
    num_controls: ak4671_snd_controls.len() as c_uint,
    dapm_widgets: ak4671_dapm_widgets.as_ptr(),
    num_dapm_widgets: ak4671_dapm_widgets.len() as c_uint,
    dapm_routes: ak4671_intercon.as_ptr(),
    num_dapm_routes: ak4671_intercon.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static ak4671_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: AK4671_SAR_ADC_CONTROL,
    reg_defaults: ak4671_reg_defaults.as_ptr(),
    num_reg_defaults: ak4671_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn ak4671_i2c_probe(client: *mut i2c_client) -> c_int {
    let regmap: *mut regmap;
    let ret: c_int;

    regmap = devm_regmap_init_i2c(client, &ak4671_regmap);
    if IS_ERR(regmap as *const core::ffi::c_void) {
        let err = PTR_ERR(regmap as *const core::ffi::c_void);
        dev_err(&mut (*client).dev, c"Failed to create regmap: %d\n".as_ptr(), err);
        return err;
    }

    ret = devm_snd_soc_register_component(
        &mut (*client).dev,
        &soc_component_dev_ak4671,
        &mut ak4671_dai,
        1,
    );
    ret
}

static ak4671_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"ak4671\0\0\0\0\0\0\0\0\0\0\0\0\0" as [c_char; 20],
    },
    i2c_device_id { name: [0; 20] },
];
MODULE_DEVICE_TABLE!(i2c, ak4671_i2c_id);

static mut ak4671_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"ak4671-codec".as_ptr(),
    },
    probe: Some(ak4671_i2c_probe),
    id_table: ak4671_i2c_id.as_ptr(),
};

module_i2c_driver!(ak4671_i2c_driver);

MODULE_DESCRIPTION!("ASoC AK4671 codec driver");
MODULE_AUTHOR!("Joonyoung Shim <jy0922.shim@samsung.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
