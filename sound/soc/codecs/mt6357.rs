// SPDX-License-Identifier: GPL-2.0
/*
 * MT6357 ALSA SoC audio codec driver
 *
 * Copyright (c) 2024 Baylibre
 * Author: Nicolas Belin <nbelin@baylibre.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_ = bool;
type u32 = c_uint;

#[repr(C)]
pub struct regmap(c_void);
#[repr(C)]
pub struct device_node(c_void);
#[repr(C)]
pub struct snd_kcontrol(c_void);
#[repr(C)]
pub struct snd_soc_component(c_void);

#[repr(C)]
pub struct device {
    parent: *mut device,
    of_node: *mut device_node,
    coherent_dma_mask: u64,
    dma_mask: *mut u64,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct mt6397_chip {
    regmap: *mut regmap,
}

#[repr(C)]
pub struct mt6357_priv {
    regmap: *mut regmap,
    dev: *mut device,
    pull_down_needed: bool_,
    hp_channel_number: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_context(c_void);
#[repr(C)]
pub struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
    kcontrols: *mut *mut snd_kcontrol,
}
#[repr(C)]
pub struct snd_kcontrol_new(c_void);
#[repr(C)]
pub struct snd_soc_dapm_widget_desc(c_void);
#[repr(C)]
pub struct snd_soc_dapm_route(c_void);
#[repr(C)]
pub struct snd_soc_dai_driver(c_void);
#[repr(C)]
pub struct snd_soc_component_driver(c_void);
#[repr(C)]
pub struct platform_device_id(c_void);
#[repr(C)]
pub struct platform_driver(c_void);

extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mt6357_priv;
    fn snd_soc_dapm_kcontrol_get_value(kcontrol: *mut snd_kcontrol) -> c_uint;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool_;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_regulator_get_enable(dev: *mut device, id: *const c_char) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        ($array.len() as c_int)
    };
}

fn IS_DCC_BASE(type_: c_uint) -> bool {
    type_ == MIC_TYPE_MUX_DCC as c_uint
        || type_ == MIC_TYPE_MUX_DCC_ECM_DIFF as c_uint
        || type_ == MIC_TYPE_MUX_DCC_ECM_SINGLE as c_uint
}

unsafe fn set_playback_gpio(priv_: *mut mt6357_priv, enable: bool_) {
    regmap_write((*priv_).regmap, MT6357_GPIO_MODE2_CLR, MT6357_GPIO_MODE2_CLEAR_ALL);
    if enable {
        /* set gpio mosi mode */
        regmap_write(
            (*priv_).regmap,
            MT6357_GPIO_MODE2_SET,
            MT6357_GPIO8_MODE_SET_AUD_CLK_MOSI
                | MT6357_GPIO9_MODE_SET_AUD_DAT_MOSI0
                | MT6357_GPIO10_MODE_SET_AUD_DAT_MOSI1
                | MT6357_GPIO11_MODE_SET_AUD_SYNC_MOSI,
        );
    } else {
        /* pad_aud_*_mosi are GPIO mode after clear and set them to dir input
         * reason:
         * pad_aud_dat_mosi*, because the pin is used as boot strap
         */
        regmap_update_bits(
            (*priv_).regmap,
            MT6357_GPIO_DIR0,
            MT6357_GPIO8_DIR_MASK
                | MT6357_GPIO9_DIR_MASK
                | MT6357_GPIO10_DIR_MASK
                | MT6357_GPIO11_DIR_MASK,
            MT6357_GPIO8_DIR_INPUT
                | MT6357_GPIO9_DIR_INPUT
                | MT6357_GPIO10_DIR_INPUT
                | MT6357_GPIO11_DIR_INPUT,
        );
    }
}

unsafe fn set_capture_gpio(priv_: *mut mt6357_priv, enable: bool_) {
    regmap_write((*priv_).regmap, MT6357_GPIO_MODE3_CLR, MT6357_GPIO_MODE3_CLEAR_ALL);
    if enable {
        /* set gpio miso mode */
        regmap_write(
            (*priv_).regmap,
            MT6357_GPIO_MODE3_SET,
            MT6357_GPIO12_MODE_SET_AUD_CLK_MISO
                | MT6357_GPIO13_MODE_SET_AUD_DAT_MISO0
                | MT6357_GPIO14_MODE_SET_AUD_DAT_MISO1
                | MT6357_GPIO15_MODE_SET_AUD_SYNC_MISO,
        );
    } else {
        /* pad_aud_*_mosi are GPIO mode after clear and set them to dir input
         * reason:
         * pad_aud_clk_miso, because when playback only the miso_clk
         * will also have 26m, so will have power leak
         * pad_aud_dat_miso*, because the pin is used as boot strap
         */
        regmap_update_bits(
            (*priv_).regmap,
            MT6357_GPIO_DIR0,
            MT6357_GPIO12_DIR_MASK
                | MT6357_GPIO13_DIR_MASK
                | MT6357_GPIO14_DIR_MASK
                | MT6357_GPIO15_DIR_MASK,
            MT6357_GPIO12_DIR_INPUT
                | MT6357_GPIO13_DIR_INPUT
                | MT6357_GPIO14_DIR_INPUT
                | MT6357_GPIO15_DIR_INPUT,
        );
    }
}

unsafe fn hp_main_output_ramp(priv_: *mut mt6357_priv, up: bool_) {
    let mut i: c_int = 0;
    while i <= MT6357_HPLOUT_STG_CTRL_VAUDP15_MAX as c_int {
        let stage = if up {
            i
        } else {
            MT6357_HPLOUT_STG_CTRL_VAUDP15_MAX as c_int - i
        };
        regmap_update_bits(
            (*priv_).regmap,
            MT6357_AUDDEC_ANA_CON1,
            MT6357_HPLOUT_STG_CTRL_VAUDP15_MASK,
            (stage as c_uint) << MT6357_HPLOUT_STG_CTRL_VAUDP15_SFT,
        );
        regmap_update_bits(
            (*priv_).regmap,
            MT6357_AUDDEC_ANA_CON1,
            MT6357_HPROUT_STG_CTRL_VAUDP15_MASK,
            (stage as c_uint) << MT6357_HPROUT_STG_CTRL_VAUDP15_SFT,
        );
        usleep_range(600, 700);
        i += 1;
    }
}

unsafe fn hp_aux_feedback_loop_gain_ramp(priv_: *mut mt6357_priv, up: bool_) {
    let mut i: c_int = 0;
    while i <= MT6357_HP_AUX_LOOP_GAIN_MAX as c_int {
        let stage = if up {
            i
        } else {
            MT6357_HP_AUX_LOOP_GAIN_MAX as c_int - i
        };
        regmap_update_bits(
            (*priv_).regmap,
            MT6357_AUDDEC_ANA_CON6,
            MT6357_HP_AUX_LOOP_GAIN_MASK,
            (stage as c_uint) << MT6357_HP_AUX_LOOP_GAIN_SFT,
        );
        usleep_range(600, 700);
        i += 1;
    }
}

unsafe fn hp_pull_down(priv_: *mut mt6357_priv, enable: bool_) {
    if enable {
        regmap_update_bits(
            (*priv_).regmap,
            MT6357_AUDDEC_ANA_CON2,
            MT6357_HPP_SHORT_2VCM_VAUDP15_MASK,
            MT6357_HPP_SHORT_2VCM_VAUDP15_ENABLE,
        );
    } else {
        regmap_update_bits(
            (*priv_).regmap,
            MT6357_AUDDEC_ANA_CON2,
            MT6357_HPP_SHORT_2VCM_VAUDP15_MASK,
            MT6357_HPP_SHORT_2VCM_VAUDP15_DISABLE,
        );
    }
}

fn is_valid_hp_pga_idx(reg_idx: c_int) -> bool {
    (reg_idx >= DL_GAIN_8DB as c_int && reg_idx <= DL_GAIN_N_12DB as c_int)
        || reg_idx == DL_GAIN_N_40DB as c_int
}

unsafe fn volume_ramp(
    priv_: *mut mt6357_priv,
    mut lfrom: c_int,
    lto: c_int,
    mut rfrom: c_int,
    rto: c_int,
    reg_addr: c_uint,
) {
    let lcount: c_int;
    let rcount: c_int;
    let mut sleep: c_int = 0;

    if !is_valid_hp_pga_idx(lfrom) || !is_valid_hp_pga_idx(lto) {
        pr_debug!(
            "%s(), invalid left volume index, from %d, to %d\n",
            "__func__",
            lfrom,
            lto
        );
    }

    if !is_valid_hp_pga_idx(rfrom) || !is_valid_hp_pga_idx(rto) {
        pr_debug!(
            "%s(), invalid right volume index, from %d, to %d\n",
            "__func__",
            rfrom,
            rto
        );
    }

    if lto > lfrom {
        lcount = 1;
    } else {
        lcount = -1;
    }

    if rto > rfrom {
        rcount = 1;
    } else {
        rcount = -1;
    }

    while lto != lfrom || rto != rfrom {
        if lto != lfrom {
            lfrom += lcount;
            if is_valid_hp_pga_idx(lfrom) {
                regmap_update_bits(
                    (*priv_).regmap,
                    reg_addr,
                    MT6357_DL_GAIN_REG_LEFT_MASK,
                    (lfrom as c_uint) << MT6357_DL_GAIN_REG_LEFT_SHIFT,
                );
                sleep = 1;
            }
        }
        if rto != rfrom {
            rfrom += rcount;
            if is_valid_hp_pga_idx(rfrom) {
                regmap_update_bits(
                    (*priv_).regmap,
                    reg_addr,
                    MT6357_DL_GAIN_REG_RIGHT_MASK,
                    (rfrom as c_uint) << MT6357_DL_GAIN_REG_RIGHT_SHIFT,
                );
                sleep = 1;
            }
        }
        if sleep != 0 {
            usleep_range(200, 300);
        }
    }
}

unsafe fn lo_volume_ramp(priv_: *mut mt6357_priv, lfrom: c_int, lto: c_int, rfrom: c_int, rto: c_int) {
    volume_ramp(priv_, lfrom, lto, rfrom, rto, MT6357_ZCD_CON1);
}

unsafe fn hp_volume_ramp(priv_: *mut mt6357_priv, lfrom: c_int, lto: c_int, rfrom: c_int, rto: c_int) {
    volume_ramp(priv_, lfrom, lto, rfrom, rto, MT6357_ZCD_CON2);
}

unsafe fn hs_volume_ramp(priv_: *mut mt6357_priv, from: c_int, to: c_int) {
    volume_ramp(priv_, from, to, 0, 0, MT6357_ZCD_CON3);
}

/* Volume and channel swap controls */
static playback_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1000, 100, 0);
static capture_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(0, 600, 0);
static hp_degain_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1200, 1200, 0);

static mt6357_controls: &[snd_kcontrol_new] = &[
    /* dl pga gain */
    SOC_DOUBLE_TLV!("Headphone Volume", MT6357_ZCD_CON2, MT6357_AUD_HPL_GAIN_SFT,
                   MT6357_AUD_HPR_GAIN_SFT, MT6357_AUD_HP_GAIN_MAX, 1, playback_tlv),
    SOC_SINGLE_TLV!("Headphone Vin Volume", MT6357_AUDDEC_ANA_CON7, MT6357_HP_IVBUF_DEGAIN_SFT,
                   MT6357_HP_IVBUF_DEGAIN_MAX, 1, hp_degain_tlv),
    SOC_DOUBLE_TLV!("Lineout Volume", MT6357_ZCD_CON1, MT6357_AUD_LOL_GAIN_SFT,
                   MT6357_AUD_LOR_GAIN_SFT, MT6357_AUD_LO_GAIN_MAX, 1, playback_tlv),
    SOC_SINGLE_TLV!("Handset Volume", MT6357_ZCD_CON3, MT6357_AUD_HS_GAIN_SFT,
                   MT6357_AUD_HS_GAIN_MAX, 1, playback_tlv),
    /* ul pga gain */
    SOC_DOUBLE_R_TLV!("Mic Volume", MT6357_AUDENC_ANA_CON0, MT6357_AUDENC_ANA_CON1,
                     MT6357_AUDPREAMPLGAIN_SFT, MT6357_AUDPREAMPLGAIN_MAX, 0, capture_tlv),
];

/* Uplink controls */
const MIC_TYPE_MUX_IDLE: c_int = 0;
const MIC_TYPE_MUX_ACC: c_int = 1;
const MIC_TYPE_MUX_DMIC: c_int = 2;
const MIC_TYPE_MUX_DCC: c_int = 3;
const MIC_TYPE_MUX_DCC_ECM_DIFF: c_int = 4;
const MIC_TYPE_MUX_DCC_ECM_SINGLE: c_int = 5;
const MIC_TYPE_MUX_LPBK: c_int = 6;
const MIC_TYPE_MUX_SGEN: c_int = 7;

static mic_type_mux_map: &[*const c_char] = &[
    c"Idle".as_ptr(),
    c"ACC".as_ptr(),
    c"DMIC".as_ptr(),
    c"DCC".as_ptr(),
    c"DCC_ECM_DIFF".as_ptr(),
    c"DCC_ECM_SINGLE".as_ptr(),
    c"Loopback".as_ptr(),
    c"Sine Generator".as_ptr(),
];

static mic_type_mux_map_enum: _ = SOC_ENUM_SINGLE_DECL!(SND_SOC_NOPM, 0, mic_type_mux_map);
static mic_type_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Mic Type Select", mic_type_mux_map_enum);

static pga_mux_map: &[*const c_char] = &[c"None".as_ptr(), c"AIN0".as_ptr(), c"AIN1".as_ptr(), c"AIN2".as_ptr()];
static pga_left_mux_map_enum: _ =
    SOC_ENUM_SINGLE_DECL!(MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLINPUTSEL_SFT, pga_mux_map);
static pga_left_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("PGA L Select", pga_left_mux_map_enum);
static pga_right_mux_map_enum: _ =
    SOC_ENUM_SINGLE_DECL!(MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRINPUTSEL_SFT, pga_mux_map);
static pga_right_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("PGA R Select", pga_right_mux_map_enum);

/* Downlink controls */
static hslo_mux_map: &[*const c_char] =
    &[c"Open".as_ptr(), c"DACR".as_ptr(), c"Playback".as_ptr(), c"Test mode".as_ptr()];
static lo_mux_map_enum: _ =
    SOC_ENUM_SINGLE_DECL!(MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_MUX_INPUT_VAUDP15_SFT, hslo_mux_map);
static lo_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Line out source", lo_mux_map_enum);
static hs_mux_map_enum: _ =
    SOC_ENUM_SINGLE_DECL!(MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_MUX_INPUT_VAUDP15_SFT, hslo_mux_map);
static hs_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Handset source", hs_mux_map_enum);

static hplr_mux_map: &[*const c_char] =
    &[c"Open".as_ptr(), c"Line Out".as_ptr(), c"DAC".as_ptr(), c"Handset".as_ptr()];
static hpr_mux_map_enum: _ =
    SOC_ENUM_SINGLE_DECL!(MT6357_AUDDEC_ANA_CON0, MT6357_AUD_HPR_MUX_INPUT_VAUDP15_SFT, hplr_mux_map);
static hpr_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Headphone Right source", hpr_mux_map_enum);
static hpl_mux_map_enum: _ =
    SOC_ENUM_SINGLE_DECL!(MT6357_AUDDEC_ANA_CON0, MT6357_AUD_HPL_MUX_INPUT_VAUDP15_SFT, hplr_mux_map);
static hpl_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Headphone Left source", hpl_mux_map_enum);

static dac_mux_map: &[*const c_char] = &[c"Normal Path".as_ptr(), c"Sine Generator".as_ptr()];
static dac_mux_map_enum: _ =
    SOC_ENUM_SINGLE_DECL!(MT6357_AFE_TOP_CON0, MT6357_DL_SINE_ON_SFT, dac_mux_map);
static dac_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("DAC Select", dac_mux_map_enum);

unsafe fn mt6357_set_dmic(priv_: *mut mt6357_priv, enable: bool_) -> c_int {
    if enable {
        /* DMIC enable */
        regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON7,
            MT6357_AUDDIGMICBIAS_MASK | MT6357_AUDDIGMICEN_MASK,
            MT6357_AUDDIGMICBIAS_DEFAULT_VALUE | MT6357_AUDDIGMICEN_ENABLE);
        /* enable aud_pad TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP,
            MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_ENABLE);
        /* UL dmic setting: dual mode */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_H,
            MT6357_C_TWO_DIGITAL_MIC_CTL_MASK, MT6357_C_TWO_DIGITAL_MIC_ENABLE);
        /* UL turn on SDM 3 level mode */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L,
            MT6357_UL_SDM_3_LEVEL_CTL_MASK, MT6357_UL_SDM_3_LEVEL_SELECT);
        /* UL turn on */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L,
            MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_ENABLE);
        /* Wait to avoid any pop noises */
        msleep(100);
    } else {
        /* UL turn off */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L,
            MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_DISABLE);
        /* UL turn on SDM 3 level mode */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L,
            MT6357_UL_SDM_3_LEVEL_CTL_MASK, MT6357_UL_SDM_3_LEVEL_DESELECT);
        /* disable aud_pad TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP,
            MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_DISABLE);
        /* UL dmic setting: dual mode */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_H,
            MT6357_C_TWO_DIGITAL_MIC_CTL_MASK, MT6357_C_TWO_DIGITAL_MIC_DISABLE);
        /* DMIC disable */
        regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON7,
            MT6357_AUDDIGMICBIAS_MASK | MT6357_AUDDIGMICEN_MASK,
            MT6357_AUDDIGMICBIAS_OFF | MT6357_AUDDIGMICEN_DISABLE);
    }
    0
}

unsafe fn mt6357_set_amic(priv_: *mut mt6357_priv, enable: bool_, mic_type: c_uint) -> c_int {
    if enable {
        if IS_DCC_BASE(mic_type) {
            regmap_update_bits((*priv_).regmap, MT6357_AFE_DCCLK_CFG0, MT6357_DCCLK_DIV_MASK, MT6357_DCCLK_DIV_RUN_VALUE);
            regmap_update_bits((*priv_).regmap, MT6357_AFE_DCCLK_CFG0, MT6357_DCCLK_PDN_MASK, MT6357_DCCLK_OUTPUT);
            regmap_update_bits((*priv_).regmap, MT6357_AFE_DCCLK_CFG0, MT6357_DCCLK_GEN_ON_MASK, MT6357_DCCLK_GEN_ON);
            regmap_update_bits((*priv_).regmap, MT6357_AFE_DCCLK_CFG1, MT6357_DCCLK_RESYNC_BYPASS_MASK, MT6357_DCCLK_RESYNC_BYPASS);

            /* mic bias 0: set the correct DC couple*/
            match mic_type as c_int {
                MIC_TYPE_MUX_DCC_ECM_DIFF => regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON8,
                    MT6357_AUD_MICBIAS0_DC_MASK, MT6357_AUD_MICBIAS0_DC_ENABLE_ALL),
                MIC_TYPE_MUX_DCC_ECM_SINGLE => regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON8,
                    MT6357_AUD_MICBIAS0_DC_MASK, MT6357_AUD_MICBIAS0_DC_ENABLE_P1),
                _ => regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON8,
                    MT6357_AUD_MICBIAS0_DC_MASK, MT6357_AUD_MICBIAS0_DC_DISABLE_ALL),
            };

            /* mic bias 1: set the correct DC couple */
            if mic_type == MIC_TYPE_MUX_DCC_ECM_SINGLE as c_uint {
                regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON9,
                    MT6357_AUD_MICBIAS1_DCSW1P_EN_MASK, MT6357_AUD_MICBIAS1_DCSW1P_ENABLE);
            }

            /* Audio L/R preamplifier DCC precharge */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLDCPRECHARGE_MASK, MT6357_AUDPREAMPLDCPRECHARGE_ENABLE);
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRDCPRECHARGE_MASK, MT6357_AUDPREAMPRDCPRECHARGE_ENABLE);
            /* L preamplifier DCCEN */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLDCCEN_MASK, MT6357_AUDPREAMPLDCCEN_DC);
            /* R preamplifier DCCEN */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRDCCEN_MASK, MT6357_AUDPREAMPRDCCEN_DC);
        } else {
            /* Audio L preamplifier DCC precharge disable */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLDCPRECHARGE_MASK, MT6357_AUDPREAMPLDCPRECHARGE_DISABLE);
            /* L preamplifier ACC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLDCCEN_MASK, MT6357_AUDPREAMPLDCCEN_AC);
            /* Audio R preamplifier DCC precharge disable */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRDCPRECHARGE_MASK, MT6357_AUDPREAMPRDCPRECHARGE_DISABLE);
            /* R preamplifier ACC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRDCCEN_MASK, MT6357_AUDPREAMPRDCCEN_AC);
        }
    } else {
        /* disable any Mic Bias 0 DC couple */
        regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON8, MT6357_AUD_MICBIAS0_DC_MASK, MT6357_AUD_MICBIAS0_DC_DISABLE_ALL);
        /* disable any Mic Bias 1 DC couple */
        regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON9, MT6357_AUD_MICBIAS1_DCSW1P_EN_MASK, MT6357_AUD_MICBIAS1_DCSW1P_DISABLE);
        if IS_DCC_BASE(mic_type) {
            regmap_update_bits((*priv_).regmap, MT6357_AFE_DCCLK_CFG0, MT6357_DCCLK_GEN_ON_MASK, MT6357_DCCLK_GEN_OFF);
            regmap_update_bits((*priv_).regmap, MT6357_AFE_DCCLK_CFG0, MT6357_DCCLK_PDN_MASK, MT6357_DCCLK_PDN);
            regmap_update_bits((*priv_).regmap, MT6357_AFE_DCCLK_CFG0, MT6357_DCCLK_DIV_MASK, MT6357_DCCLK_DIV_STOP_VALUE);
        }
    }
    0
}

unsafe fn mt6357_set_loopback(priv_: *mut mt6357_priv, enable: bool_) -> c_int {
    if enable {
        /* enable aud_pad TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_ENABLE);
        /* enable aud_pad lpk TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_LPBK_MASK, MT6357_AUD_PAD_TX_FIFO_LPBK_ENABLE);
        /* Set UL Part: enable new lpbk 2 */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_ADDA_MTKAIF_CFG0, MT6357_ADDA_MTKAIF_LPBK_CTL_MASK, MT6357_ADDA_MTKAIF_LPBK_ENABLE);
        /* UL turn on */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L, MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_ENABLE);
    } else {
        /* UL turn off */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L, MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_DISABLE);
        /* disable new lpbk 2 */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_ADDA_MTKAIF_CFG0, MT6357_ADDA_MTKAIF_LPBK_CTL_MASK, MT6357_ADDA_MTKAIF_LPBK_DISABLE);
        /* disable aud_pad lpbk TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_LPBK_MASK, MT6357_AUD_PAD_TX_FIFO_LPBK_DISABLE);
        /* disable aud_pad TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_DISABLE);
    }
    0
}

unsafe fn mt6357_set_ul_sine_gen(priv_: *mut mt6357_priv, enable: bool_) -> c_int {
    if enable {
        /* enable aud_pad TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_ENABLE);
        /* UL turn on */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L, MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_ENABLE);
    } else {
        /* UL turn off */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L, MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_DISABLE);
        /* disable aud_pad TX fifos */
        regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_DISABLE);
    }
    0
}

unsafe fn mt_aif_out_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_PRE_PMU => set_capture_gpio(priv_, true),
        SND_SOC_DAPM_POST_PMD => set_capture_gpio(priv_, false),
        _ => {}
    }
    0
}

unsafe fn mt_adc_supply_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Enable audio ADC CLKGEN  */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON11, MT6357_RSTB_ENCODER_VA28_MASK, MT6357_RSTB_ENCODER_VA28_ENABLE);
            /* Enable  LCLDO_ENC 2P8V */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON12, MT6357_LCLDO_ENC_EN_VA28_MASK, MT6357_LCLDO_ENC_EN_VA28_ENABLE);
            /* LCLDO_ENC remote sense */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON12,
                MT6357_VA28REFGEN_EN_VA28_MASK | MT6357_LCLDO_ENC_REMOTE_SENSE_VA28_MASK,
                MT6357_VA28REFGEN_EN_VA28_ENABLE | MT6357_LCLDO_ENC_REMOTE_SENSE_VA28_ENABLE);
        }
        SND_SOC_DAPM_POST_PMD => {
            /* LCLDO_ENC remote sense off */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON12,
                MT6357_VA28REFGEN_EN_VA28_MASK | MT6357_LCLDO_ENC_REMOTE_SENSE_VA28_MASK,
                MT6357_VA28REFGEN_EN_VA28_DISABLE | MT6357_LCLDO_ENC_REMOTE_SENSE_VA28_DISABLE);
            /* disable LCLDO_ENC 2P8V */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON12, MT6357_LCLDO_ENC_EN_VA28_MASK, MT6357_LCLDO_ENC_EN_VA28_DISABLE);
            /* disable audio ADC CLKGEN  */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON11, MT6357_RSTB_ENCODER_VA28_MASK, MT6357_RSTB_ENCODER_VA28_DISABLE);
        }
        _ => {}
    }
    0
}

unsafe fn mt_mic_type_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mic_type = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);

    match event {
        SND_SOC_DAPM_PRE_PMU => match mic_type as c_int {
            MIC_TYPE_MUX_DMIC => { mt6357_set_dmic(priv_, true); }
            MIC_TYPE_MUX_LPBK => { mt6357_set_loopback(priv_, true); }
            MIC_TYPE_MUX_SGEN => { mt6357_set_ul_sine_gen(priv_, true); }
            _ => { mt6357_set_amic(priv_, true, mic_type); }
        },
        SND_SOC_DAPM_POST_PMD => match mic_type as c_int {
            MIC_TYPE_MUX_DMIC => { mt6357_set_dmic(priv_, false); }
            MIC_TYPE_MUX_LPBK => { mt6357_set_loopback(priv_, false); }
            MIC_TYPE_MUX_SGEN => { mt6357_set_ul_sine_gen(priv_, false); }
            _ => { mt6357_set_amic(priv_, false, mic_type); }
        },
        _ => {}
    }
    0
}

unsafe fn mt_pga_left_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* L preamplifier enable */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLON_MASK, MT6357_AUDPREAMPLON_ENABLE);
            /* L ADC input sel : L PGA. Enable audio L ADC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDADCLINPUTSEL_MASK, MT6357_AUDADCLINPUTSEL_PREAMPLIFIER);
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDADCLPWRUP_MASK, MT6357_AUDADCLPWRUP);
            /* Audio L preamplifier DCC precharge off */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLDCPRECHARGE_MASK, MT6357_AUDPREAMPLDCPRECHARGE_DISABLE);
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* Audio L ADC input sel : off, disable audio L ADC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDADCLPWRUP_MASK, MT6357_AUDADCLPWRDOWN);
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDADCLINPUTSEL_MASK, MT6357_AUDADCLINPUTSEL_IDLE);
            /* L preamplifier ACC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLDCCEN_MASK, MT6357_AUDPREAMPLDCCEN_AC);
            /* L preamplifier disable */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLON_MASK, MT6357_AUDPREAMPLON_DISABLE);
            /* disable Audio L preamplifier DCC precharge */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLDCPRECHARGE_MASK, MT6357_AUDPREAMPLDCPRECHARGE_DISABLE);
        }
        _ => {}
    }
    0
}

unsafe fn mt_pga_right_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* R preamplifier enable */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRON_MASK, MT6357_AUDPREAMPRON_ENABLE);
            /* R ADC input sel : R PGA. Enable audio R ADC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDADCRINPUTSEL_MASK, MT6357_AUDADCRINPUTSEL_PREAMPLIFIER);
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDADCRPWRUP_MASK, MT6357_AUDADCRPWRUP);
            /* Audio R preamplifier DCC precharge off */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRDCPRECHARGE_MASK, MT6357_AUDPREAMPRDCPRECHARGE_DISABLE);
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* Audio R ADC input sel : off, disable audio R ADC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDADCRPWRUP_MASK, MT6357_AUDADCRPWRDOWN);
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDADCRINPUTSEL_MASK, MT6357_AUDADCRINPUTSEL_IDLE);
            /* R preamplifier ACC */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRDCCEN_MASK, MT6357_AUDPREAMPRDCCEN_AC);
            /* R preamplifier disable */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRON_MASK, MT6357_AUDPREAMPRON_DISABLE);
            /* disable Audio R preamplifier DCC precharge */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRDCPRECHARGE_MASK, MT6357_AUDPREAMPRDCPRECHARGE_DISABLE);
        }
        _ => {}
    }
    0
}

unsafe fn adc_enable_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mut lgain: c_int = 0;
    let mut rgain: c_int = 0;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_read((*priv_).regmap, MT6357_AUDENC_ANA_CON0, &mut lgain);
            regmap_read((*priv_).regmap, MT6357_AUDENC_ANA_CON1, &mut rgain);
            /* L PGA 0 dB gain */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLGAIN_MASK, (UL_GAIN_0DB as c_uint) << MT6357_AUDPREAMPLGAIN_SFT);
            /* R PGA 0 dB gain */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRGAIN_MASK, (UL_GAIN_0DB as c_uint) << MT6357_AUDPREAMPRGAIN_SFT);
            /* enable aud_pad TX fifos */
            regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_ENABLE);
            /* UL turn on */
            regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L, MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_ENABLE);
            /* Wait to avoid any pop noises */
            msleep(100);
            /* set the mic gains to the stored values */
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON0, MT6357_AUDPREAMPLGAIN_MASK, lgain as c_uint);
            regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON1, MT6357_AUDPREAMPRGAIN_MASK, rgain as c_uint);
        }
        SND_SOC_DAPM_POST_PMD => {
            /* UL turn off */
            regmap_update_bits((*priv_).regmap, MT6357_AFE_UL_SRC_CON0_L, MT6357_UL_SRC_ON_TMP_CTL_MASK, MT6357_UL_SRC_DISABLE);
            /* disable aud_pad TX fifos */
            regmap_update_bits((*priv_).regmap, MT6357_AFE_AUD_PAD_TOP, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_MASK, MT6357_AUD_PAD_TX_FIFO_NORMAL_PATH_DISABLE);
        }
        _ => {}
    }
    0
}

unsafe fn configure_downlinks(priv_: *mut mt6357_priv, enable: bool_) {
    if enable {
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ELR_0, MT6357_AUD_HP_TRIM_EN_VAUDP15_MASK, MT6357_AUD_HP_TRIM_EN_VAUDP15_ENABLE);
        /* Disable headphone short-circuit protection */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
            MT6357_AUD_HPR_SC_VAUDP15_MASK | MT6357_AUD_HPL_SC_VAUDP15_MASK,
            MT6357_AUD_HPR_SC_VAUDP15_DISABLE | MT6357_AUD_HPL_SC_VAUDP15_DISABLE);
        /* Disable handset short-circuit protection */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_SC_VAUDP15_MASK, MT6357_AUD_HS_SC_VAUDP15_DISABLE);
        /* Disable lineout short-circuit protection */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_SC_VAUDP15_MASK, MT6357_AUD_LOL_SC_VAUDP15_DISABLE);
        /* Reduce ESD resistance of AU_REFN */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON2, MT6357_AUD_REFN_DERES_VAUDP15_MASK, MT6357_AUD_REFN_DERES_VAUDP15_ENABLE);
        /* Turn on DA_600K_NCP_VA18 */
        regmap_write((*priv_).regmap, MT6357_AUDNCP_CLKDIV_CON1, MT6357_DIVCKS_ON);
        /* Set NCP clock as 604kHz // 26MHz/43 = 604KHz */
        regmap_write((*priv_).regmap, MT6357_AUDNCP_CLKDIV_CON2, 0x002c);
        /* Toggle DIVCKS_CHG */
        regmap_write((*priv_).regmap, MT6357_AUDNCP_CLKDIV_CON0, MT6357_DIVCKS_CHG);
        /* Set NCP soft start mode as default mode: 150us */
        regmap_write((*priv_).regmap, MT6357_AUDNCP_CLKDIV_CON4, MT6357_DIVCKS_PWD_NCP_ST_150US);
        /* Enable NCP */
        regmap_write((*priv_).regmap, MT6357_AUDNCP_CLKDIV_CON3, MT6357_DIVCKS_PWD_NCP_ENABLE);
        usleep_range(250, 270);
        /* Enable cap-less LDOs (1.5V) */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON12,
            MT6357_VA33REFGEN_EN_VA18_MASK | MT6357_LCLDO_REMOTE_SENSE_VA18_MASK |
            MT6357_LCLDO_EN_VA18_MASK | MT6357_HCLDO_REMOTE_SENSE_VA18_MASK |
            MT6357_HCLDO_EN_VA18_MASK,
            MT6357_VA33REFGEN_EN_VA18_ENABLE | MT6357_LCLDO_REMOTE_SENSE_VA18_ENABLE |
            MT6357_LCLDO_EN_VA18_ENABLE | MT6357_HCLDO_REMOTE_SENSE_VA18_ENABLE |
            MT6357_HCLDO_EN_VA18_ENABLE);
        /* Enable NV regulator (-1.2V) */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON13, MT6357_NVREG_EN_VAUDP15_MASK, MT6357_NVREG_EN_VAUDP15_ENABLE);
        usleep_range(100, 120);
        /* Enable IBIST */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON10, MT6357_AUD_IBIAS_PWRDN_VAUDP15_MASK, MT6357_AUD_IBIAS_PWRDN_VAUDP15_ENABLE);
        /* Enable AUD_CLK */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON11, MT6357_RSTB_DECODER_VA28_MASK, MT6357_RSTB_DECODER_VA28_ENABLE);
        /* Enable low-noise mode of DAC */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6, MT6357_DAC_LOW_NOISE_MODE_MASK, MT6357_DAC_LOW_NOISE_MODE_ENABLE);
        usleep_range(100, 120);
    } else {
        /* Disable low-noise mode of DAC */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6, MT6357_DAC_LOW_NOISE_MODE_MASK, MT6357_DAC_LOW_NOISE_MODE_DISABLE);
        /* Disable AUD_CLK */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON11, MT6357_RSTB_DECODER_VA28_MASK, MT6357_RSTB_DECODER_VA28_DISABLE);
        /* Enable linout short-circuit protection */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_SC_VAUDP15_MASK, MT6357_AUD_LOL_SC_VAUDP15_ENABLE);
        /* Enable handset short-circuit protection */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_SC_VAUDP15_MASK, MT6357_AUD_HS_SC_VAUDP15_ENABLE);
        /* Enable headphone short-circuit protection */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
            MT6357_AUD_HPR_SC_VAUDP15_MASK | MT6357_AUD_HPL_SC_VAUDP15_MASK,
            MT6357_AUD_HPR_SC_VAUDP15_ENABLE | MT6357_AUD_HPL_SC_VAUDP15_ENABLE);
        /* Disable IBIST */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON10, MT6357_AUD_IBIAS_PWRDN_VAUDP15_MASK, MT6357_AUD_IBIAS_PWRDN_VAUDP15_DISABLE);
        /* Disable NV regulator (-1.2V) */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON13, MT6357_NVREG_EN_VAUDP15_MASK, MT6357_NVREG_EN_VAUDP15_DISABLE);
        /* Disable cap-less LDOs (1.5V) */
        regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON12,
            MT6357_VA33REFGEN_EN_VA18_MASK | MT6357_LCLDO_REMOTE_SENSE_VA18_MASK |
            MT6357_LCLDO_EN_VA18_MASK | MT6357_HCLDO_REMOTE_SENSE_VA18_MASK |
            MT6357_HCLDO_EN_VA18_MASK,
            MT6357_VA33REFGEN_EN_VA18_DISABLE | MT6357_LCLDO_REMOTE_SENSE_VA18_DISABLE |
            MT6357_LCLDO_EN_VA18_DISABLE | MT6357_HCLDO_REMOTE_SENSE_VA18_DISABLE |
            MT6357_HCLDO_EN_VA18_DISABLE);
        /* Disable NCP */
        regmap_update_bits((*priv_).regmap, MT6357_AUDNCP_CLKDIV_CON3, MT6357_DIVCKS_PWD_NCP_MASK, MT6357_DIVCKS_PWD_NCP_DISABLE);
    }
}

unsafe fn mt_audio_in_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            set_playback_gpio(priv_, true);
            /* Pull-down HPL/R to AVSS28_AUD */
            if (*priv_).pull_down_needed {
                hp_pull_down(priv_, true);
            }
            /* Disable HP main CMFB Switch */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6, MT6357_HPRL_MAIN_CMFB_LOOP_MASK, MT6357_HPRL_MAIN_CMFB_LOOP_DISABLE);
            /* Audio system digital clock power down release */
            regmap_write((*priv_).regmap, MT6357_AFUNC_AUD_CON2,
                MT6357_CCI_AUDIO_FIFO_DISABLE | MT6357_CCI_ACD_MODE_NORMAL_PATH |
                MT6357_CCI_AFIFO_CLK_PWDB_ON | MT6357_CCI_ACD_FUNC_RSTB_RESET);
            /* sdm audio fifo clock power on */
            regmap_write((*priv_).regmap, MT6357_AFUNC_AUD_CON0,
                MT6357_CCI_AUD_ANACK_INVERT | (4 << MT6357_CCI_AUDIO_FIFO_WPTR_SFT) |
                MT6357_CCI_SCRAMBLER_CG_ENABLE | MT6357_CCI_RAND_ENABLE |
                MT6357_CCI_SPLT_SCRMB_CLK_ON | MT6357_CCI_SPLT_SCRMB_ON |
                MT6357_CCI_ZERO_PADDING_DISABLE | MT6357_CCI_SCRAMBLER_ENABLE);
            /* scrambler clock on enable */
            regmap_write((*priv_).regmap, MT6357_AFUNC_AUD_CON2,
                MT6357_CCI_AUDIO_FIFO_DISABLE | MT6357_CCI_ACD_MODE_TEST_PATH |
                MT6357_CCI_AFIFO_CLK_PWDB_ON | MT6357_CCI_ACD_FUNC_RSTB_RELEASE);
            /* sdm power on */
            regmap_write((*priv_).regmap, MT6357_AFUNC_AUD_CON2,
                MT6357_CCI_AUDIO_FIFO_ENABLE | MT6357_CCI_ACD_MODE_TEST_PATH |
                MT6357_CCI_AFIFO_CLK_PWDB_ON | MT6357_CCI_ACD_FUNC_RSTB_RELEASE);
            configure_downlinks(priv_, true);
        }
        SND_SOC_DAPM_POST_PMD => {
            configure_downlinks(priv_, false);
            /* DL scrambler disabling sequence */
            regmap_write((*priv_).regmap, MT6357_AFUNC_AUD_CON2,
                MT6357_CCI_AUDIO_FIFO_DISABLE | MT6357_CCI_ACD_MODE_TEST_PATH |
                MT6357_CCI_AFIFO_CLK_PWDB_DOWN | MT6357_CCI_ACD_FUNC_RSTB_RESET);
            regmap_write((*priv_).regmap, MT6357_AFUNC_AUD_CON0,
                MT6357_CCI_AUD_ANACK_INVERT | (4 << MT6357_CCI_AUDIO_FIFO_WPTR_SFT) |
                MT6357_CCI_SCRAMBLER_CG_ENABLE | MT6357_CCI_RAND_ENABLE |
                MT6357_CCI_SPLT_SCRMB_CLK_ON | MT6357_CCI_SPLT_SCRMB_ON |
                MT6357_CCI_ZERO_PADDING_DISABLE | MT6357_CCI_SCRAMBLER_DISABLE);
            set_playback_gpio(priv_, false);
            /* disable Pull-down HPL/R to AVSS28_AUD */
            if (*priv_).pull_down_needed {
                hp_pull_down(priv_, false);
            }
        }
        _ => {}
    }
    0
}

unsafe fn mt_delay_250_event(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    match event {
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD => usleep_range(250, 270),
        _ => {}
    }
    0
}

unsafe fn lo_mux_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mut lgain: c_int = 0;
    let rgain: c_int;
    /* Get current gain value */
    regmap_read((*priv_).regmap, MT6357_ZCD_CON1, &mut lgain);
    rgain = ((lgain as c_uint & MT6357_AUD_LOR_GAIN_MASK) >> MT6357_AUD_LOR_GAIN_SFT) as c_int;
    lgain = (lgain as c_uint & MT6357_AUD_LOL_GAIN_MASK) as c_int;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* Set -40dB before enable HS to avoid POP noise */
            regmap_update_bits((*priv_).regmap, MT6357_ZCD_CON1,
                MT6357_AUD_LOL_GAIN_MASK | MT6357_AUD_LOR_GAIN_MASK, MT6357_DL_GAIN_N_40DB_REG);
            /* Set LO STB enhance circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOLOUT_STB_ENH_VAUDP15_MASK, MT6357_AUD_LOLOUT_STB_ENH_VAUDP15_ENABLE);
            /* Enable LO driver bias circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_PWRUP_BIAS_VAUDP15_MASK, MT6357_AUD_LOL_PWRUP_BIAS_VAUDP15_ENABLE);
            /* Enable LO driver core circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_PWRUP_VAUDP15_MASK, MT6357_AUD_LOL_PWRUP_VAUDP15_ENABLE);
            /* Set LOL gain to normal gain step by step */
            lo_volume_ramp(priv_, DL_GAIN_N_40DB as c_int, lgain, DL_GAIN_N_40DB as c_int, rgain);
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* decrease LOL gain to minimum gain step by step */
            lo_volume_ramp(priv_, lgain, DL_GAIN_N_40DB as c_int, rgain, DL_GAIN_N_40DB as c_int);
            /* Disable LO driver core circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_PWRUP_VAUDP15_MASK, MT6357_AUD_LOL_PWRUP_VAUDP15_DISABLE);
            /* Disable LO driver bias circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_PWRUP_BIAS_VAUDP15_MASK, MT6357_AUD_LOL_PWRUP_BIAS_VAUDP15_DISABLE);
            /* Clear LO STB enhance circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOLOUT_STB_ENH_VAUDP15_MASK, MT6357_AUD_LOLOUT_STB_ENH_VAUDP15_DISABLE);
            /* Save the gain value into the register*/
            regmap_update_bits((*priv_).regmap, MT6357_ZCD_CON1,
                MT6357_AUD_LOL_GAIN_MASK | MT6357_AUD_LOR_GAIN_MASK,
                ((lgain as c_uint) << MT6357_AUD_LOL_GAIN_SFT) | ((rgain as c_uint) << MT6357_AUD_LOR_GAIN_SFT));
        }
        _ => {}
    }
    0
}

unsafe fn hs_mux_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mut gain: c_int = 0; /* HS register has only one gain slot */
    /* Get current gain value */
    regmap_read((*priv_).regmap, MT6357_ZCD_CON3, &mut gain);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* Set -40dB before enable HS to avoid POP noise */
            regmap_update_bits((*priv_).regmap, MT6357_ZCD_CON3, MT6357_AUD_HS_GAIN_MASK, DL_GAIN_N_40DB);
            /* Set HS STB enhance circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HSOUT_STB_ENH_VAUDP15_MASK, MT6357_AUD_HSOUT_STB_ENH_VAUDP15_ENABLE);
            /* Enable HS driver bias circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_PWRUP_BIAS_VAUDP15_MASK, MT6357_AUD_HS_PWRUP_BIAS_VAUDP15_ENABLE);
            /* Enable HS driver core circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_PWRUP_VAUDP15_MASK, MT6357_AUD_HS_PWRUP_VAUDP15_ENABLE);
            /* Set HS gain to normal gain step by step */
            hs_volume_ramp(priv_, DL_GAIN_N_40DB as c_int, gain);
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* decrease HS gain to minimum gain step by step */
            hs_volume_ramp(priv_, gain, DL_GAIN_N_40DB as c_int);
            /* Disable HS driver core circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_PWRUP_VAUDP15_MASK, MT6357_AUD_HS_PWRUP_VAUDP15_DISABLE);
            /* Disable HS driver bias circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_PWRUP_BIAS_VAUDP15_MASK, MT6357_AUD_HS_PWRUP_BIAS_VAUDP15_ENABLE);
            /* Clear HS STB enhance circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HSOUT_STB_ENH_VAUDP15_MASK, MT6357_AUD_HSOUT_STB_ENH_VAUDP15_DISABLE);
            /* Save the gain value into the register*/
            regmap_update_bits((*priv_).regmap, MT6357_ZCD_CON3, MT6357_AUD_HS_GAIN_MASK, gain as c_uint);
        }
        _ => {}
    }
    0
}

unsafe fn hp_main_mux_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mut lgain: c_int = 0;
    let rgain: c_int;
    /* Get current gain value */
    regmap_read((*priv_).regmap, MT6357_ZCD_CON2, &mut lgain);
    rgain = ((lgain as c_uint & MT6357_AUD_HPR_GAIN_MASK) >> MT6357_AUD_HPR_GAIN_SFT) as c_int;
    lgain = (lgain as c_uint & MT6357_AUD_HPL_GAIN_MASK) as c_int;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            (*priv_).hp_channel_number += 1;
            if (*priv_).hp_channel_number > 1 {
                return 0;
            }
            /* Set -40dB before enable HS to avoid POP noise */
            regmap_update_bits((*priv_).regmap, MT6357_ZCD_CON2,
                MT6357_AUD_HPL_GAIN_MASK | MT6357_AUD_HPR_GAIN_MASK, MT6357_DL_GAIN_N_40DB_REG);
            /* Set HPP/N STB enhance circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON2,
                MT6357_HPROUT_STB_ENH_VAUDP15_MASK | MT6357_HPLOUT_STB_ENH_VAUDP15_MASK,
                MT6357_HPROUT_STB_ENH_VAUDP15_N470_P250 | MT6357_HPLOUT_STB_ENH_VAUDP15_N470_P250);
            /* Enable HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_MASK | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_MASK,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_ENABLE | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_ENABLE);
            /* Enable HP aux feedback loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_AUX_FBRSW_VAUDP15_MASK | MT6357_HPL_AUX_FBRSW_VAUDP15_MASK,
                MT6357_HPR_AUX_FBRSW_VAUDP15_ENABLE | MT6357_HPL_AUX_FBRSW_VAUDP15_ENABLE);
            /* Enable HP aux CMFB loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6,
                MT6357_HP_CMFB_RST_MASK | MT6357_HPL_AUX_CMFB_LOOP_MASK | MT6357_HPR_AUX_CMFB_LOOP_MASK,
                MT6357_HP_CMFB_RST_NORMAL | MT6357_HPL_AUX_CMFB_LOOP_ENABLE | MT6357_HPR_AUX_CMFB_LOOP_ENABLE);
            /* Enable HP driver bias circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_HPR_BIAS_VAUDP15_MASK | MT6357_AUD_HPL_BIAS_VAUDP15_MASK,
                MT6357_AUD_HPR_BIAS_VAUDP15_ENABLE | MT6357_AUD_HPL_BIAS_VAUDP15_ENABLE);
            /* Enable HP driver core circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_HPR_PWRUP_VAUDP15_MASK | MT6357_AUD_HPL_PWRUP_VAUDP15_MASK,
                MT6357_AUD_HPR_PWRUP_VAUDP15_ENABLE | MT6357_AUD_HPL_PWRUP_VAUDP15_ENABLE);
            /* Short HP main output to HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_MASK | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_MASK,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_ENABLE | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_ENABLE);
            /* Enable HP main CMFB loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6, MT6357_HPRL_MAIN_CMFB_LOOP_MASK, MT6357_HPRL_MAIN_CMFB_LOOP_ENABLE);
            /* Disable HP aux CMFB loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6,
                MT6357_HPR_AUX_CMFB_LOOP_MASK | MT6357_HPL_AUX_CMFB_LOOP_MASK,
                MT6357_HPR_AUX_CMFB_LOOP_DISABLE | MT6357_HPL_AUX_CMFB_LOOP_DISABLE);
            /* Enable HP main output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPROUT_PWRUP_VAUDP15_MASK | MT6357_HPLOUT_PWRUP_VAUDP15_MASK,
                MT6357_HPROUT_PWRUP_VAUDP15_ENABLE | MT6357_HPLOUT_PWRUP_VAUDP15_ENABLE);
            /* Enable HPR/L main output stage step by step */
            hp_main_output_ramp(priv_, true);
            usleep_range(1000, 1200);
            /* Reduce HP aux feedback loop gain */
            hp_aux_feedback_loop_gain_ramp(priv_, true);
            /* Disable HP aux feedback loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_AUX_FBRSW_VAUDP15_MASK | MT6357_HPL_AUX_FBRSW_VAUDP15_MASK,
                MT6357_HPR_AUX_FBRSW_VAUDP15_DISABLE | MT6357_HPL_AUX_FBRSW_VAUDP15_DISABLE);
            /* apply volume setting */
            hp_volume_ramp(priv_, DL_GAIN_N_40DB as c_int, lgain, DL_GAIN_N_40DB as c_int, rgain);
            /* Disable HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_MASK | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_MASK,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_DISABLE | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_DISABLE);
            /* Unshort HP main output to HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_MASK | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_MASK,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_DISABLE | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_DISABLE);
            usleep_range(100, 120);
        }
        SND_SOC_DAPM_PRE_PMD => {
            (*priv_).hp_channel_number -= 1;
            if (*priv_).hp_channel_number > 0 {
                return 0;
            }
            /* Short HP main output to HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_MASK | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_MASK,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_ENABLE | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_ENABLE);
            /* Enable HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_MASK | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_MASK,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_ENABLE | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_ENABLE);
            /* decrease HPL/R gain to normal gain step by step */
            hp_volume_ramp(priv_, lgain, DL_GAIN_N_40DB as c_int, rgain, DL_GAIN_N_40DB as c_int);
            /* Enable HP aux feedback loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_AUX_FBRSW_VAUDP15_MASK | MT6357_HPL_AUX_FBRSW_VAUDP15_MASK,
                MT6357_HPR_AUX_FBRSW_VAUDP15_ENABLE | MT6357_HPL_AUX_FBRSW_VAUDP15_ENABLE);
            /* Reduce HP aux feedback loop gain */
            hp_aux_feedback_loop_gain_ramp(priv_, false);
            /* decrease HPR/L main output stage step by step */
            hp_main_output_ramp(priv_, false);
            /* Disable HP main output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPROUT_PWRUP_VAUDP15_MASK | MT6357_HPLOUT_PWRUP_VAUDP15_MASK,
                MT6357_HPROUT_PWRUP_VAUDP15_DISABLE | MT6357_HPLOUT_PWRUP_VAUDP15_DISABLE);
            /* Enable HP aux CMFB loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6,
                MT6357_HP_CMFB_RST_MASK | MT6357_HPL_AUX_CMFB_LOOP_MASK | MT6357_HPR_AUX_CMFB_LOOP_MASK,
                MT6357_HP_CMFB_RST_RESET | MT6357_HPL_AUX_CMFB_LOOP_ENABLE | MT6357_HPR_AUX_CMFB_LOOP_ENABLE);
            /* Disable HP main CMFB loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6, MT6357_HPRL_MAIN_CMFB_LOOP_MASK, MT6357_HPRL_MAIN_CMFB_LOOP_DISABLE);
            /* Unshort HP main output to HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_MASK | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_MASK,
                MT6357_HPR_SHORT2HPR_AUX_VAUDP15_DISABLE | MT6357_HPL_SHORT2HPR_AUX_VAUDP15_DISABLE);
            /* Disable HP driver core circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_HPR_PWRUP_VAUDP15_MASK | MT6357_AUD_HPL_PWRUP_VAUDP15_MASK,
                MT6357_AUD_HPR_PWRUP_VAUDP15_DISABLE | MT6357_AUD_HPL_PWRUP_VAUDP15_DISABLE);
            /* Disable HP driver bias circuits */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_HPR_BIAS_VAUDP15_MASK | MT6357_AUD_HPL_BIAS_VAUDP15_MASK,
                MT6357_AUD_HPR_BIAS_VAUDP15_DISABLE | MT6357_AUD_HPL_BIAS_VAUDP15_DISABLE);
            /* Disable HP aux CMFB loop,
             * Enable HP main CMFB for HP off state
             */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON6,
                MT6357_HPRL_MAIN_CMFB_LOOP_MASK | MT6357_HPR_AUX_CMFB_LOOP_MASK | MT6357_HPL_AUX_CMFB_LOOP_MASK,
                MT6357_HPRL_MAIN_CMFB_LOOP_ENABLE | MT6357_HPR_AUX_CMFB_LOOP_DISABLE | MT6357_HPL_AUX_CMFB_LOOP_DISABLE);
            /* Disable HP aux feedback loop */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPR_AUX_FBRSW_VAUDP15_MASK | MT6357_HPL_AUX_FBRSW_VAUDP15_MASK,
                MT6357_HPR_AUX_FBRSW_VAUDP15_DISABLE | MT6357_HPL_AUX_FBRSW_VAUDP15_DISABLE);
            /* Disable HP aux output stage */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON1,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_MASK | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_MASK,
                MT6357_HPROUT_AUX_PWRUP_VAUDP15_DISABLE | MT6357_HPLOUT_AUX_PWRUP_VAUDP15_DISABLE);
            /* Save the gain value into the register*/
            regmap_update_bits((*priv_).regmap, MT6357_ZCD_CON2,
                MT6357_AUD_HPL_GAIN_MASK | MT6357_AUD_HPR_GAIN_MASK,
                ((lgain as c_uint) << MT6357_AUD_HPL_GAIN_SFT) | ((rgain as c_uint) << MT6357_AUD_HPR_GAIN_SFT));
        }
        _ => {}
    }
    0
}

unsafe fn right_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Enable Audio DAC and control audio bias gen */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_DACR_PWRUP_VA28_MASK | MT6357_AUD_DACR_PWRUP_VAUDP15_MASK,
                MT6357_AUD_DACR_PWRUP_VA28_ENABLE | MT6357_AUD_DACR_PWRUP_VAUDP15_ENABLE);
        }
        SND_SOC_DAPM_POST_PMU => {
            /* disable Pull-down HPL/R to AVSS28_AUD */
            if (*priv_).pull_down_needed {
                hp_pull_down(priv_, false);
            }
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* Pull-down HPL/R to AVSS28_AUD */
            if (*priv_).pull_down_needed {
                hp_pull_down(priv_, true);
            }
            /* Disable Audio DAC and control audio bias gen  */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_DACR_PWRUP_VA28_MASK | MT6357_AUD_DACR_PWRUP_VAUDP15_MASK,
                MT6357_AUD_DACR_PWRUP_VA28_DISABLE | MT6357_AUD_DACR_PWRUP_VAUDP15_DISABLE);
        }
        _ => {}
    }
    0
}

unsafe fn left_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Enable Audio DAC and control audio bias gen  */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_DACL_PWRUP_VA28_MASK | MT6357_AUD_DACL_PWRUP_VAUDP15_MASK,
                MT6357_AUD_DACL_PWRUP_VA28_ENABLE | MT6357_AUD_DACL_PWRUP_VAUDP15_ENABLE);
        }
        SND_SOC_DAPM_POST_PMU => {
            /* disable Pull-down HPL/R to AVSS28_AUD */
            if (*priv_).pull_down_needed {
                hp_pull_down(priv_, false);
            }
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* Pull-down HPL/R to AVSS28_AUD */
            if (*priv_).pull_down_needed {
                hp_pull_down(priv_, true);
            }
            /* Disable Audio DAC and control audio bias gen  */
            regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
                MT6357_AUD_DACL_PWRUP_VA28_MASK | MT6357_AUD_DACL_PWRUP_VAUDP15_MASK,
                MT6357_AUD_DACL_PWRUP_VA28_DISABLE | MT6357_AUD_DACL_PWRUP_VAUDP15_DISABLE);
        }
        _ => {}
    }
    0
}

/* Supply widgets subsequence */
const SUPPLY_SEQ_CLK_BUF: c_int = 0;
const SUPPLY_SEQ_AUD_GLB: c_int = 1;
const SUPPLY_SEQ_CLKSQ: c_int = 2;
const SUPPLY_SEQ_VOW_AUD_LPW: c_int = 3;
const SUPPLY_SEQ_AUD_VOW: c_int = 4;
const SUPPLY_SEQ_VOW_CLK: c_int = 5;
const SUPPLY_SEQ_VOW_LDO: c_int = 6;
const SUPPLY_SEQ_TOP_CK: c_int = 7;
const SUPPLY_SEQ_TOP_CK_LAST: c_int = 8;
const SUPPLY_SEQ_AUD_TOP: c_int = 9;
const SUPPLY_SEQ_AUD_TOP_LAST: c_int = 10;
const SUPPLY_SEQ_AFE: c_int = 11;
/* capture */
const SUPPLY_SEQ_ADC_SUPPLY: c_int = 12;

/* DAPM Widgets */
static mt6357_dapm_widgets: &[snd_soc_dapm_widget_desc] = &[
    /* Analog Clocks */
    SND_SOC_DAPM_SUPPLY_S!("CLK_BUF", SUPPLY_SEQ_CLK_BUF, MT6357_DCXO_CW14, MT6357_XO_AUDIO_EN_M_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDGLB", SUPPLY_SEQ_AUD_GLB, MT6357_AUDDEC_ANA_CON11, MT6357_AUDGLB_PWRDN_VA28_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("CLKSQ Audio", SUPPLY_SEQ_CLKSQ, MT6357_AUDENC_ANA_CON6, MT6357_CLKSQ_EN_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDNCP_CK", SUPPLY_SEQ_TOP_CK, MT6357_AUD_TOP_CKPDN_CON0, MT6357_AUDNCP_CK_PDN_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ZCD13M_CK", SUPPLY_SEQ_TOP_CK, MT6357_AUD_TOP_CKPDN_CON0, MT6357_ZCD13M_CK_PDN_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUD_CK", SUPPLY_SEQ_TOP_CK_LAST, MT6357_AUD_TOP_CKPDN_CON0, MT6357_AUD_CK_PDN_SFT, 1, mt_delay_250_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY_S!("AUDIF_CK", SUPPLY_SEQ_TOP_CK, MT6357_AUD_TOP_CKPDN_CON0, MT6357_AUDIF_CK_PDN_SFT, 1, None, 0),

    /* Digital Clocks */
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_AFE_CTL", SUPPLY_SEQ_AUD_TOP_LAST, MT6357_AUDIO_TOP_CON0, MT6357_PDN_AFE_CTL_SFT, 1, mt_delay_250_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_DAC_CTL", SUPPLY_SEQ_AUD_TOP, MT6357_AUDIO_TOP_CON0, MT6357_PDN_DAC_CTL_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_ADC_CTL", SUPPLY_SEQ_AUD_TOP, MT6357_AUDIO_TOP_CON0, MT6357_PDN_ADC_CTL_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_I2S_DL", SUPPLY_SEQ_AUD_TOP, MT6357_AUDIO_TOP_CON0, MT6357_PDN_I2S_DL_CTL_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_PWR_CLK", SUPPLY_SEQ_AUD_TOP, MT6357_AUDIO_TOP_CON0, MT6357_PWR_CLK_DIS_CTL_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_PDN_AFE_TESTMODEL", SUPPLY_SEQ_AUD_TOP, MT6357_AUDIO_TOP_CON0, MT6357_PDN_AFE_TESTMODEL_CTL_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_PDN_RESERVED", SUPPLY_SEQ_AUD_TOP, MT6357_AUDIO_TOP_CON0, MT6357_PDN_RESERVED_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_TOP_LPBK", SUPPLY_SEQ_AUD_TOP, MT6357_AUDIO_TOP_CON0, MT6357_PDN_LPBK_CTL_SFT, 1, None, 0),

    /* General */
    SND_SOC_DAPM_SUPPLY_S!("AFE_ON", SUPPLY_SEQ_AFE, MT6357_AFE_UL_DL_CON0, MT6357_AFE_ON_SFT, 0, None, 0),

    /* Uplinks */
    SND_SOC_DAPM_AIF_OUT_E!("AIF1TX", "MT6357 Capture", 0, SND_SOC_NOPM, 0, 0, mt_aif_out_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADC Supply", SUPPLY_SEQ_ADC_SUPPLY, SND_SOC_NOPM, 0, 0, mt_adc_supply_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_ADC_E!("ADC", None, SND_SOC_NOPM, 0, 0, adc_enable_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E!("PGA L Mux", SND_SOC_NOPM, 0, 0, &pga_left_mux_control, mt_pga_left_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX_E!("PGA R Mux", SND_SOC_NOPM, 0, 0, &pga_right_mux_control, mt_pga_right_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA!("PGA L", SND_SOC_NOPM, 0, 0, None, 0),
    SND_SOC_DAPM_PGA!("PGA R", SND_SOC_NOPM, 0, 0, None, 0),
    SND_SOC_DAPM_MUX_E!("Mic Type Mux", SND_SOC_NOPM, 0, 0, &mic_type_mux_control, mt_mic_type_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("MICBIAS0", MT6357_AUDENC_ANA_CON8, MT6357_AUD_MICBIAS0_PWD_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS1", MT6357_AUDENC_ANA_CON9, MT6357_AUD_MICBIAS1_PWD_SFT, 0, None, 0),

    /* UL inputs */
    SND_SOC_DAPM_INPUT!("AIN0"),
    SND_SOC_DAPM_INPUT!("AIN1"),
    SND_SOC_DAPM_INPUT!("AIN2"),
    SND_SOC_DAPM_INPUT!("LPBK"),
    SND_SOC_DAPM_INPUT!("SGEN UL"),

    /* Downlinks */
    SND_SOC_DAPM_AIF_IN_E!("AIF_RX", "MT6357 Playback", 0, SND_SOC_NOPM, 0, 0, mt_audio_in_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_INPUT!("SGEN DL"),
    SND_SOC_DAPM_MUX!("DAC Mux", SND_SOC_NOPM, 0, 0, &dac_mux_control),
    SND_SOC_DAPM_DAC_E!("DACR", None, SND_SOC_NOPM, 0, 0, right_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_DAC_E!("DACL", None, SND_SOC_NOPM, 0, 0, left_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!("DL Digital Supply", SND_SOC_NOPM, 0, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("DL Analog Supply", SND_SOC_NOPM, 0, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("DL SRC", MT6357_AFE_DL_SRC2_CON0_L, MT6357_DL_2_SRC_ON_TMP_CTL_PRE_SFT, 0, None, 0),
    SND_SOC_DAPM_MUX_E!("Line Out Source", SND_SOC_NOPM, 0, 0, &lo_mux_control, lo_mux_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX_E!("Handset Source", SND_SOC_NOPM, 0, 0, &hs_mux_control, hs_mux_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX_E!("Headphone Right Source", SND_SOC_NOPM, 0, 0, &hpr_mux_control, hp_main_mux_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX_E!("Headphone Left Source", SND_SOC_NOPM, 0, 0, &hpl_mux_control, hp_main_mux_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    /* DL outputs */
    SND_SOC_DAPM_OUTPUT!("Headphones"),
    SND_SOC_DAPM_OUTPUT!("Hansdet"),
    SND_SOC_DAPM_OUTPUT!("Line out"),

    /* Sine generator */
    SND_SOC_DAPM_SUPPLY!("SGEN UL Enable", MT6357_AFE_TOP_CON0, MT6357_UL_SINE_ON_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("SGEN Enable", MT6357_AFE_SGEN_CFG0, MT6357_SGEN_DAC_EN_CTL_SFT, 0, mt_audio_in_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("SGEN MUTE", MT6357_AFE_SGEN_CFG0, MT6357_SGEN_MUTE_SW_CTL_SFT, 1, None, 0),
];

static mt6357_dapm_routes: &[snd_soc_dapm_route] = &[
    /* Capture */
    route!("AIF1TX", None, "Mic Type Mux"),
    route!("AIF1TX", None, "CLK_BUF"),
    route!("AIF1TX", None, "AUDGLB"),
    route!("AIF1TX", None, "CLKSQ Audio"),
    route!("AIF1TX", None, "AUD_CK"),
    route!("AIF1TX", None, "AUDIF_CK"),
    route!("AIF1TX", None, "AUDIO_TOP_AFE_CTL"),
    route!("AIF1TX", None, "AUDIO_TOP_ADC_CTL"),
    route!("AIF1TX", None, "AUDIO_TOP_PWR_CLK"),
    route!("AIF1TX", None, "AUDIO_TOP_PDN_RESERVED"),
    route!("AIF1TX", None, "AUDIO_TOP_I2S_DL"),
    route!("AIF1TX", None, "AFE_ON"),
    route!("Mic Type Mux", "ACC", "ADC"),
    route!("Mic Type Mux", "DCC", "ADC"),
    route!("Mic Type Mux", "DCC_ECM_DIFF", "ADC"),
    route!("Mic Type Mux", "DCC_ECM_SINGLE", "ADC"),
    route!("Mic Type Mux", "DMIC", "AIN0"),
    route!("Mic Type Mux", "DMIC", "AIN2"),
    route!("Mic Type Mux", "Loopback", "LPBK"),
    route!("Mic Type Mux", "Sine Generator", "SGEN UL"),
    route!("SGEN UL", None, "AUDIO_TOP_PDN_AFE_TESTMODEL"),
    route!("SGEN UL", None, "SGEN UL Enable"),
    route!("SGEN UL", None, "SGEN MUTE"),
    route!("SGEN UL", None, "SGEN Enable"),
    route!("ADC", None, "PGA L Mux"),
    route!("ADC", None, "PGA R Mux"),
    route!("ADC", None, "ADC Supply"),
    route!("PGA L Mux", "AIN0", "AIN0"),
    route!("PGA L Mux", "AIN1", "AIN1"),
    route!("PGA L Mux", "AIN2", "AIN2"),
    route!("PGA R Mux", "AIN0", "AIN0"),
    route!("PGA R Mux", "AIN1", "AIN1"),
    route!("PGA R Mux", "AIN2", "AIN2"),
    route!("AIN0", None, "MICBIAS0"),
    route!("AIN1", None, "MICBIAS1"),
    route!("AIN2", None, "MICBIAS0"),
    route!("LPBK", None, "AUDIO_TOP_LPBK"),

    /* Playback */
    route!("DAC Mux", "Normal Path", "AIF_RX"),
    route!("DAC Mux", "Sine Generator", "SGEN DL"),
    route!("AIF_RX", None, "DL SRC"),
    route!("SGEN DL", None, "DL SRC"),
    route!("SGEN DL", None, "SGEN MUTE"),
    route!("SGEN DL", None, "SGEN Enable"),
    route!("SGEN DL", None, "DL Digital Supply"),
    route!("SGEN DL", None, "AUDIO_TOP_PDN_AFE_TESTMODEL"),
    route!("DACL", None, "DAC Mux"),
    route!("DACR", None, "DAC Mux"),
    route!("DL Analog Supply", None, "CLK_BUF"),
    route!("DL Analog Supply", None, "AUDGLB"),
    route!("DL Analog Supply", None, "CLKSQ Audio"),
    route!("DL Analog Supply", None, "AUDNCP_CK"),
    route!("DL Analog Supply", None, "ZCD13M_CK"),
    route!("DL Analog Supply", None, "AUD_CK"),
    route!("DL Analog Supply", None, "AUDIF_CK"),
    route!("DL Digital Supply", None, "AUDIO_TOP_AFE_CTL"),
    route!("DL Digital Supply", None, "AUDIO_TOP_DAC_CTL"),
    route!("DL Digital Supply", None, "AUDIO_TOP_PWR_CLK"),
    route!("DL Digital Supply", None, "AFE_ON"),
    route!("DACR", None, "DL Digital Supply"),
    route!("DACR", None, "DL Analog Supply"),
    route!("DACL", None, "DL Digital Supply"),
    route!("DACL", None, "DL Analog Supply"),
    route!("Line Out Source", "DACR", "DACR"),
    route!("Line Out Source", "Playback", "DACL"),
    route!("Line Out Source", "Test mode", "DACL"),
    route!("Handset Source", "DACR", "DACR"),
    route!("Handset Source", "Playback", "DACL"),
    route!("Handset Source", "Test mode", "DACL"),
    route!("Headphone Right Source", "DAC", "DACR"),
    route!("Headphone Right Source", "Line Out", "Line Out Source"),
    route!("Headphone Right Source", "Handset", "Handset Source"),
    route!("Headphone Left Source", "DAC", "DACL"),
    route!("Headphone Left Source", "Line Out", "Line Out Source"),
    route!("Headphone Left Source", "Handset", "Handset Source"),
    route!("Line out", None, "Line Out Source"),
    route!("Hansdet", None, "Handset Source"),
    route!("Headphones", None, "Headphone Right Source"),
    route!("Headphones", None, "Headphone Left Source"),
];

static mut mtk_6357_dai_codecs: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver! {
        name: "mt6357-snd-codec-aif1",
        playback: {
            stream_name: "MT6357 Playback",
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: MT6357_SND_SOC_ADV_MT_FMTS,
        },
        capture: {
            stream_name: "MT6357 Capture",
            channels_min: 1,
            channels_max: 2,
            rates: MT6357_SOC_HIGH_USE_RATE,
            formats: MT6357_SND_SOC_ADV_MT_FMTS,
        },
    },
];

unsafe fn mt6357_codec_probe(codec: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(codec);

    snd_soc_component_init_regmap(codec, (*priv_).regmap);

    /* Enable audio part */
    regmap_update_bits((*priv_).regmap, MT6357_DCXO_CW14, MT6357_XO_AUDIO_EN_M_MASK, MT6357_XO_AUDIO_EN_M_ENABLE);
    /* Disable HeadphoneL/HeadphoneR short circuit protection */
    regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON0,
        MT6357_AUD_HPR_SC_VAUDP15_MASK | MT6357_AUD_HPL_SC_VAUDP15_MASK,
        MT6357_AUD_HPR_SC_VAUDP15_DISABLE | MT6357_AUD_HPL_SC_VAUDP15_DISABLE);
    /* Disable voice short circuit protection */
    regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON3, MT6357_AUD_HS_SC_VAUDP15_MASK, MT6357_AUD_HS_SC_VAUDP15_DISABLE);
    /* disable LO buffer left short circuit protection */
    regmap_update_bits((*priv_).regmap, MT6357_AUDDEC_ANA_CON4, MT6357_AUD_LOL_SC_VAUDP15_MASK, MT6357_AUD_LOL_SC_VAUDP15_DISABLE);
    /* set gpio */
    set_playback_gpio(priv_, false);
    set_capture_gpio(priv_, false);
    /* Disable audio part */
    regmap_update_bits((*priv_).regmap, MT6357_DCXO_CW14, MT6357_XO_AUDIO_EN_M_MASK, MT6357_XO_AUDIO_EN_M_DISABLE);

    0
}

static mt6357_soc_component_driver: snd_soc_component_driver = snd_soc_component_driver! {
    probe: mt6357_codec_probe,
    read: snd_soc_component_read,
    write: snd_soc_component_write,
    controls: mt6357_controls,
    num_controls: ARRAY_SIZE!(mt6357_controls),
    dapm_widgets: mt6357_dapm_widgets,
    num_dapm_widgets: ARRAY_SIZE!(mt6357_dapm_widgets),
    dapm_routes: mt6357_dapm_routes,
    num_dapm_routes: ARRAY_SIZE!(mt6357_dapm_routes),
};

static micbias_values: [u32; 8] = [
    1700000, 1800000, 1900000, 2000000,
    2100000, 2500000, 2600000, 2700000,
];

unsafe fn mt6357_get_micbias_idx(np: *mut device_node, micbias: *const c_char) -> u32 {
    let err: c_int;
    let mut idx: u32 = 0;
    let mut val: u32 = 0;

    err = of_property_read_u32(np, micbias, &mut val);
    if err != 0 {
        return 0;
    }

    while idx < micbias_values.len() as u32 {
        if val == micbias_values[idx as usize] {
            return idx;
        }
        idx += 1;
    }
    0
}

unsafe fn mt6357_parse_dt(priv_: *mut mt6357_priv) -> c_int {
    let mut micbias_voltage_index: u32;
    let np: *mut device_node = (*(*(*priv_).dev).parent).of_node;

    if np.is_null() {
        return -EINVAL;
    }

    (*priv_).pull_down_needed = false;
    if of_property_read_bool(np, c"mediatek,hp-pull-down".as_ptr()) {
        (*priv_).pull_down_needed = true;
    }

    micbias_voltage_index = mt6357_get_micbias_idx(np, c"mediatek,micbias0-microvolt".as_ptr());
    regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON8,
        MT6357_AUD_MICBIAS0_VREF_MASK, micbias_voltage_index << MT6357_AUD_MICBIAS0_VREF_SFT);

    micbias_voltage_index = mt6357_get_micbias_idx(np, c"mediatek,micbias1-microvolt".as_ptr());
    regmap_update_bits((*priv_).regmap, MT6357_AUDENC_ANA_CON9,
        MT6357_AUD_MICBIAS1_VREF_MASK, micbias_voltage_index << MT6357_AUD_MICBIAS1_VREF_SFT);

    0
}

unsafe fn mt6357_platform_driver_probe(pdev: *mut platform_device) -> c_int {
    let mt6397: *mut mt6397_chip = dev_get_drvdata((*pdev).dev.parent) as *mut mt6397_chip;
    let priv_: *mut mt6357_priv;
    let mut ret: c_int;

    ret = devm_regulator_get_enable(&mut (*pdev).dev, c"vaud28".as_ptr());
    if ret != 0 {
        return dev_err_probe(&mut (*pdev).dev, ret, c"Failed to enable vaud28 regulator\n".as_ptr());
    }

    priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mt6357_priv>(), GFP_KERNEL) as *mut mt6357_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);
    (*priv_).dev = &mut (*pdev).dev;

    (*priv_).regmap = (*mt6397).regmap;
    if IS_ERR((*priv_).regmap as *mut c_void) {
        return PTR_ERR((*priv_).regmap as *mut c_void);
    }

    ret = mt6357_parse_dt(priv_);
    if ret != 0 {
        return dev_err_probe(&mut (*pdev).dev, ret, c"Failed to parse dts\n".as_ptr());
    }

    (*pdev).dev.coherent_dma_mask = DMA_BIT_MASK(64);
    if (*pdev).dev.dma_mask.is_null() {
        (*pdev).dev.dma_mask = &mut (*pdev).dev.coherent_dma_mask;
    }

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &mt6357_soc_component_driver,
        mtk_6357_dai_codecs.as_mut_ptr(),
        ARRAY_SIZE!(mtk_6357_dai_codecs),
    )
}

static mt6357_platform_ids: &[platform_device_id] = &[
    platform_device_id! { name: "mt6357-sound" },
    platform_device_id! { /* sentinel */ },
];
MODULE_DEVICE_TABLE!(platform, mt6357_platform_ids);

static mut mt6357_platform_driver: platform_driver = platform_driver! {
    driver: {
        name: "mt6357-sound",
        probe_type: PROBE_PREFER_ASYNCHRONOUS,
    },
    probe: mt6357_platform_driver_probe,
    id_table: mt6357_platform_ids,
};

module_platform_driver!(mt6357_platform_driver);

MODULE_DESCRIPTION!("MT6357 ALSA SoC codec driver");
MODULE_AUTHOR!("Nicolas Belin <nbelin@baylibre.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
