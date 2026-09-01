// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2023-2024 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2025, Linaro Ltd

// Translated from soc/codecs/pm4125.c. C include dependencies are expected to be
// supplied by the surrounding kernel/Rust binding environment.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type s32 = i32;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type irqreturn_t = c_uint;
type irq_hw_number_t = c_ulong;

const WCD_MBHC_HS_V_MAX: c_int = 1600;
const PM4125_MBHC_MAX_BUTTONS: c_int = 8;

const PM4125_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_192000
    | SNDRV_PCM_RATE_384000;

/* Fractional Rates */
const PM4125_FRAC_RATES: c_uint = SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_352800;

const PM4125_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S32_LE;

/* Registers in SPMI addr space */
const PM4125_CODEC_RESET_REG: c_uint = 0xF3DB;
const PM4125_CODEC_OFF: c_uint = 0x1;
const PM4125_CODEC_ON: c_uint = 0x0;
const PM4125_CODEC_FOUNDRY_ID_REG: c_uint = 0x7;

const HPH_COMP_DELAY: c_int = 0;
const HPH_PA_DELAY: c_int = 1;
const AMIC2_BCS_ENABLE: c_int = 2;

const AIF1_PB: usize = 0;
const AIF1_CAP: usize = 1;
const NUM_CODEC_DAIS: usize = 2;

#[repr(C)]
pub struct pm4125_priv {
    tx_sdw_dev: *mut sdw_slave,
    sdw_priv: [*mut pm4125_sdw_priv; NUM_CODEC_DAIS],
    txdev: *mut device,
    rxdev: *mut device,
    rxnode: *mut device_node,
    txnode: *mut device_node,
    regmap: *mut regmap,
    spmi_regmap: *mut regmap,
    /* mbhc module */
    wcd_mbhc: *mut wcd_mbhc,
    mbhc_cfg: wcd_mbhc_config,
    intr_ids: wcd_mbhc_intr,
    common: wcd_common,
    virq: *mut irq_domain,
    chip_desc: *const regmap_irq_chip,
    irq_chip: *mut regmap_irq_chip_data,
    jack: *mut snd_soc_jack,
    status_mask: c_ulong,
    micb_ref: [s32; PM4125_MAX_MICBIAS],
    pullup_ref: [s32; PM4125_MAX_MICBIAS],
    hphr_pdm_wd_int: c_int,
    hphl_pdm_wd_int: c_int,
    comp1_enable: bool,
    comp2_enable: bool,
    gloal_mbias_cnt: atomic_t,
}

static pm4125_power_supplies: [*const c_char; 4] = [
    c_str!("vdd-io"),
    c_str!("vdd-cp"),
    c_str!("vdd-mic-bias"),
    c_str!("vdd-pa-vpos"),
];

static line_gain: tlv_db_scale = DECLARE_TLV_DB_SCALE!(0, 7, 1);
static analog_gain: tlv_db_scale = DECLARE_TLV_DB_SCALE!(0, 25, 1);

static pm4125_mbhc_fields: [wcd_mbhc_field; WCD_MBHC_REG_FUNC_MAX] = [
    WCD_MBHC_FIELD!(WCD_MBHC_L_DET_EN, PM4125_ANA_MBHC_MECH, 0x80),
    WCD_MBHC_FIELD!(WCD_MBHC_GND_DET_EN, PM4125_ANA_MBHC_MECH, 0x40),
    WCD_MBHC_FIELD!(WCD_MBHC_MECH_DETECTION_TYPE, PM4125_ANA_MBHC_MECH, 0x20),
    WCD_MBHC_FIELD!(WCD_MBHC_MIC_CLAMP_CTL, PM4125_ANA_MBHC_PLUG_DETECT_CTL, 0x30),
    WCD_MBHC_FIELD!(WCD_MBHC_ELECT_DETECTION_TYPE, PM4125_ANA_MBHC_ELECT, 0x08),
    WCD_MBHC_FIELD!(WCD_MBHC_HS_L_DET_PULL_UP_CTRL, PM4125_ANA_MBHC_PLUG_DETECT_CTL, 0x1F),
    WCD_MBHC_FIELD!(WCD_MBHC_HS_L_DET_PULL_UP_COMP_CTRL, PM4125_ANA_MBHC_MECH, 0x04),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHL_PLUG_TYPE, PM4125_ANA_MBHC_MECH, 0x10),
    WCD_MBHC_FIELD!(WCD_MBHC_GND_PLUG_TYPE, PM4125_ANA_MBHC_MECH, 0x08),
    WCD_MBHC_FIELD!(WCD_MBHC_SW_HPH_LP_100K_TO_GND, PM4125_ANA_MBHC_MECH, 0x01),
    WCD_MBHC_FIELD!(WCD_MBHC_ELECT_SCHMT_ISRC, PM4125_ANA_MBHC_ELECT, 0x06),
    WCD_MBHC_FIELD!(WCD_MBHC_FSM_EN, PM4125_ANA_MBHC_ELECT, 0x80),
    WCD_MBHC_FIELD!(WCD_MBHC_INSREM_DBNC, PM4125_ANA_MBHC_PLUG_DETECT_CTL, 0x0F),
    WCD_MBHC_FIELD!(WCD_MBHC_BTN_DBNC, PM4125_ANA_MBHC_CTL_1, 0x03),
    WCD_MBHC_FIELD!(WCD_MBHC_HS_VREF, PM4125_ANA_MBHC_CTL_2, 0x03),
    WCD_MBHC_FIELD!(WCD_MBHC_HS_COMP_RESULT, PM4125_ANA_MBHC_RESULT_3, 0x08),
    WCD_MBHC_FIELD!(WCD_MBHC_IN2P_CLAMP_STATE, PM4125_ANA_MBHC_RESULT_3, 0x10),
    WCD_MBHC_FIELD!(WCD_MBHC_MIC_SCHMT_RESULT, PM4125_ANA_MBHC_RESULT_3, 0x20),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHL_SCHMT_RESULT, PM4125_ANA_MBHC_RESULT_3, 0x80),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHR_SCHMT_RESULT, PM4125_ANA_MBHC_RESULT_3, 0x40),
    WCD_MBHC_FIELD!(WCD_MBHC_BTN_RESULT, PM4125_ANA_MBHC_RESULT_3, 0x07),
    WCD_MBHC_FIELD!(WCD_MBHC_BTN_ISRC_CTL, PM4125_ANA_MBHC_ELECT, 0x70),
    WCD_MBHC_FIELD!(WCD_MBHC_ELECT_RESULT, PM4125_ANA_MBHC_RESULT_3, 0xFF),
    WCD_MBHC_FIELD!(WCD_MBHC_MICB_CTRL, PM4125_ANA_MICBIAS_MICB_1_2_EN, 0xC0),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHR_PA_EN, PM4125_ANA_HPHPA_CNP_CTL_2, 0x40),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHL_PA_EN, PM4125_ANA_HPHPA_CNP_CTL_2, 0x80),
    WCD_MBHC_FIELD!(WCD_MBHC_HPH_PA_EN, PM4125_ANA_HPHPA_CNP_CTL_2, 0xC0),
    WCD_MBHC_FIELD!(WCD_MBHC_SWCH_LEVEL_REMOVE, PM4125_ANA_MBHC_RESULT_3, 0x10),
    WCD_MBHC_FIELD!(WCD_MBHC_FSM_STATUS, PM4125_ANA_MBHC_FSM_STATUS, 0x01),
    WCD_MBHC_FIELD!(WCD_MBHC_MUX_CTL, PM4125_ANA_MBHC_CTL_2, 0x70),
    WCD_MBHC_FIELD!(WCD_MBHC_MOISTURE_STATUS, PM4125_ANA_MBHC_FSM_STATUS, 0x20),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHL_OCP_DET_EN, PM4125_ANA_HPHPA_CNP_CTL_2, 0x01),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHR_OCP_DET_EN, PM4125_ANA_HPHPA_CNP_CTL_2, 0x01),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHL_OCP_STATUS, PM4125_DIG_SWR_INTR_STATUS_0, 0x80),
    WCD_MBHC_FIELD!(WCD_MBHC_HPHR_OCP_STATUS, PM4125_DIG_SWR_INTR_STATUS_0, 0x20),
    WCD_MBHC_FIELD!(WCD_MBHC_ADC_EN, PM4125_ANA_MBHC_CTL_1, 0x08),
    WCD_MBHC_FIELD!(WCD_MBHC_ADC_COMPLETE, PM4125_ANA_MBHC_FSM_STATUS, 0x40),
    WCD_MBHC_FIELD!(WCD_MBHC_ADC_TIMEOUT, PM4125_ANA_MBHC_FSM_STATUS, 0x80),
    WCD_MBHC_FIELD!(WCD_MBHC_ADC_RESULT, PM4125_ANA_MBHC_ADC_RESULT, 0xFF),
    WCD_MBHC_FIELD!(WCD_MBHC_MICB2_VOUT, PM4125_ANA_MICBIAS_LDO_1_SETTING, 0x3F),
    WCD_MBHC_FIELD!(WCD_MBHC_ADC_MODE, PM4125_ANA_MBHC_CTL_1, 0x10),
    WCD_MBHC_FIELD!(WCD_MBHC_DETECTION_DONE, PM4125_ANA_MBHC_CTL_1, 0x04),
    WCD_MBHC_FIELD!(WCD_MBHC_ELECT_ISRC_EN, PM4125_ANA_MBHC_ZDET, 0x02),
];

static pm4125_irqs: [regmap_irq; PM4125_NUM_IRQS] = [
    REGMAP_IRQ_REG!(PM4125_IRQ_MBHC_BUTTON_PRESS_DET, 0, BIT!(0)),
    REGMAP_IRQ_REG!(PM4125_IRQ_MBHC_BUTTON_RELEASE_DET, 0, BIT!(1)),
    REGMAP_IRQ_REG!(PM4125_IRQ_MBHC_ELECT_INS_REM_DET, 0, BIT!(2)),
    REGMAP_IRQ_REG!(PM4125_IRQ_MBHC_ELECT_INS_REM_LEG_DET, 0, BIT!(3)),
    REGMAP_IRQ_REG!(PM4125_IRQ_MBHC_SW_DET, 0, BIT!(4)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHR_OCP_INT, 0, BIT!(5)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHR_CNP_INT, 0, BIT!(6)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHL_OCP_INT, 0, BIT!(7)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHL_CNP_INT, 1, BIT!(0)),
    REGMAP_IRQ_REG!(PM4125_IRQ_EAR_CNP_INT, 1, BIT!(1)),
    REGMAP_IRQ_REG!(PM4125_IRQ_EAR_SCD_INT, 1, BIT!(2)),
    REGMAP_IRQ_REG!(PM4125_IRQ_AUX_CNP_INT, 1, BIT!(3)),
    REGMAP_IRQ_REG!(PM4125_IRQ_AUX_SCD_INT, 1, BIT!(4)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHL_PDM_WD_INT, 1, BIT!(5)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHR_PDM_WD_INT, 1, BIT!(6)),
    REGMAP_IRQ_REG!(PM4125_IRQ_AUX_PDM_WD_INT, 1, BIT!(7)),
    REGMAP_IRQ_REG!(PM4125_IRQ_LDORT_SCD_INT, 2, BIT!(0)),
    REGMAP_IRQ_REG!(PM4125_IRQ_MBHC_MOISTURE_INT, 2, BIT!(1)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHL_SURGE_DET_INT, 2, BIT!(2)),
    REGMAP_IRQ_REG!(PM4125_IRQ_HPHR_SURGE_DET_INT, 2, BIT!(3)),
];

unsafe extern "C" fn pm4125_handle_post_irq(data: *mut c_void) -> c_int {
    let pm4125 = data as *mut pm4125_priv;
    regmap_write((*pm4125).regmap, PM4125_DIG_SWR_INTR_CLEAR_0, 0);
    regmap_write((*pm4125).regmap, PM4125_DIG_SWR_INTR_CLEAR_1, 0);
    regmap_write((*pm4125).regmap, PM4125_DIG_SWR_INTR_CLEAR_2, 0);
    IRQ_HANDLED as c_int
}

static pm4125_config_regs: [u32; 1] = [PM4125_DIG_SWR_INTR_LEVEL_0];

static pm4125_regmap_irq_chip: regmap_irq_chip = regmap_irq_chip {
    name: c_str!("pm4125"),
    irqs: pm4125_irqs.as_ptr(),
    num_irqs: ARRAY_SIZE!(pm4125_irqs),
    num_regs: 3,
    status_base: PM4125_DIG_SWR_INTR_STATUS_0,
    mask_base: PM4125_DIG_SWR_INTR_MASK_0,
    ack_base: PM4125_DIG_SWR_INTR_CLEAR_0,
    use_ack: 1,
    clear_ack: 1,
    config_base: pm4125_config_regs.as_ptr(),
    num_config_bases: ARRAY_SIZE!(pm4125_config_regs),
    num_config_regs: 1,
    runtime_pm: true,
    handle_post_irq: Some(pm4125_handle_post_irq),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pm4125_reset(pm4125: *mut pm4125_priv) {
    regmap_write((*pm4125).spmi_regmap, PM4125_CODEC_RESET_REG, PM4125_CODEC_OFF);
    usleep_range(20, 30);
    regmap_write((*pm4125).spmi_regmap, PM4125_CODEC_RESET_REG, PM4125_CODEC_ON);
    usleep_range(5000, 5010);
}

unsafe fn pm4125_io_init(regmap: *mut regmap) {
    /* Disable HPH OCP */
    regmap_update_bits(
        regmap,
        PM4125_ANA_HPHPA_CNP_CTL_2,
        PM4125_ANA_HPHPA_CNP_OCP_EN_L_MASK | PM4125_ANA_HPHPA_CNP_OCP_EN_R_MASK,
        PM4125_ANA_HPHPA_CNP_OCP_DISABLE,
    );
    /* Enable surge protection */
    regmap_update_bits(
        regmap,
        PM4125_ANA_SURGE_EN,
        PM4125_ANA_SURGE_PROTECTION_HPHL_MASK,
        FIELD_PREP!(PM4125_ANA_SURGE_PROTECTION_HPHL_MASK, PM4125_ANA_SURGE_PROTECTION_ENABLE),
    );
    regmap_update_bits(
        regmap,
        PM4125_ANA_SURGE_EN,
        PM4125_ANA_SURGE_PROTECTION_HPHR_MASK,
        FIELD_PREP!(PM4125_ANA_SURGE_PROTECTION_HPHR_MASK, PM4125_ANA_SURGE_PROTECTION_ENABLE),
    );
    /* Disable mic bias 2 pull down */
    regmap_update_bits(
        regmap,
        PM4125_ANA_MICBIAS_MICB_1_2_EN,
        PM4125_ANA_MICBIAS_MICB2_PULL_DN_MASK,
        FIELD_PREP!(PM4125_ANA_MICBIAS_MICB2_PULL_DN_MASK, PM4125_ANA_MICBIAS_MICB_PULL_DISABLE),
    );
}

unsafe fn pm4125_global_mbias_disable(component: *mut snd_soc_component) -> c_int {
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    if atomic_dec_and_test(&mut (*pm4125).gloal_mbias_cnt) {
        snd_soc_component_write_field(component, PM4125_ANA_MBIAS_EN, PM4125_ANA_MBIAS_EN_V2I_MASK, PM4125_ANA_MBIAS_EN_DISABLE);
        snd_soc_component_write_field(component, PM4125_ANA_MBIAS_EN, PM4125_ANA_MBIAS_EN_GLOBAL_MASK, PM4125_ANA_MBIAS_EN_DISABLE);
    }
    0
}

unsafe fn pm4125_global_mbias_enable(component: *mut snd_soc_component) -> c_int {
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    if atomic_inc_return(&mut (*pm4125).gloal_mbias_cnt) == 1 {
        snd_soc_component_write_field(component, PM4125_ANA_MBIAS_EN, PM4125_ANA_MBIAS_EN_GLOBAL_MASK, PM4125_ANA_MBIAS_EN_ENABLE);
        snd_soc_component_write_field(component, PM4125_ANA_MBIAS_EN, PM4125_ANA_MBIAS_EN_V2I_MASK, PM4125_ANA_MBIAS_EN_ENABLE);
        usleep_range(1000, 1100);
    }
    0
}

unsafe fn pm4125_rx_clk_enable(component: *mut snd_soc_component) -> c_int {
    pm4125_global_mbias_enable(component);
    snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_ANA_RX_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_ENABLE);
    snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_ANA_RX_DIV2_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_ENABLE);
    usleep_range(5000, 5100);
    snd_soc_component_write_field(component, PM4125_ANA_HPHPA_FSM_CLK, PM4125_ANA_HPHPA_FSM_DIV_RATIO_MASK, PM4125_ANA_HPHPA_FSM_DIV_RATIO_68);
    snd_soc_component_write_field(component, PM4125_ANA_HPHPA_FSM_CLK, PM4125_ANA_HPHPA_FSM_CLK_DIV_EN_MASK, PM4125_ANA_HPHPA_FSM_CLK_DIV_ENABLE);
    snd_soc_component_update_bits(component, PM4125_ANA_NCP_VCTRL, 0x07, 0x06);
    snd_soc_component_write_field(component, PM4125_ANA_NCP_EN, PM4125_ANA_NCP_ENABLE_MASK, PM4125_ANA_NCP_ENABLE);
    usleep_range(500, 510);
    0
}

unsafe fn pm4125_rx_clk_disable(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write_field(component, PM4125_ANA_HPHPA_FSM_CLK, PM4125_ANA_HPHPA_FSM_CLK_DIV_EN_MASK, PM4125_ANA_HPHPA_FSM_CLK_DIV_DISABLE);
    snd_soc_component_write_field(component, PM4125_ANA_HPHPA_FSM_CLK, PM4125_ANA_HPHPA_FSM_DIV_RATIO_MASK, 0x00);
    snd_soc_component_write_field(component, PM4125_ANA_NCP_EN, PM4125_ANA_NCP_ENABLE_MASK, PM4125_ANA_NCP_DISABLE);
    snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_ANA_RX_DIV2_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_DISABLE);
    snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_ANA_RX_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_DISABLE);
    pm4125_global_mbias_disable(component);
    0
}

unsafe fn pm4125_codec_enable_rxclk(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    match event {
        SND_SOC_DAPM_PRE_PMU => { pm4125_rx_clk_enable(component); }
        SND_SOC_DAPM_POST_PMD => { pm4125_rx_clk_disable(component); }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_hphl_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_write_field(component, PM4125_ANA_HPHPA_CNP_CTL_1, PM4125_ANA_HPHPA_CNP_CTL_1_EN_MASK, PM4125_ANA_HPHPA_CNP_CTL_1_EN);
            snd_soc_component_write_field(component, PM4125_SWR_HPHPA_HD2, PM4125_SWR_HPHPA_HD2_LEFT_MASK, PM4125_SWR_HPHPA_HD2_ENABLE);
        }
        SND_SOC_DAPM_POST_PMU => {
            if (*pm4125).comp1_enable {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHL_EN_MASK, PM4125_DIG_SWR_COMP_ENABLE);
                if (*pm4125).comp2_enable {
                    snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHR_EN_MASK, PM4125_DIG_SWR_COMP_ENABLE);
                }
                /*
                 * 5ms sleep is required after COMP is enabled as per
                 * HW requirement
                 */
                usleep_range(5000, 5100);
            } else {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHL_EN_MASK, PM4125_DIG_SWR_COMP_DISABLE);
            }
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX0_CTL, PM4125_DIG_SWR_DSM_DITHER_EN_MASK, PM4125_DIG_SWR_DSM_DITHER_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_GAIN_CTL, PM4125_DIG_SWR_RX0_EN_MASK, PM4125_DIG_SWR_RX_INPUT_ENABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_RX0_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_ENABLE);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_RX0_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_GAIN_CTL, PM4125_DIG_SWR_RX0_EN_MASK, PM4125_DIG_SWR_RX_INPUT_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX0_CTL, PM4125_DIG_SWR_DSM_DITHER_EN_MASK, PM4125_DIG_SWR_DSM_DITHER_ENABLE);
            if (*pm4125).comp1_enable {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHL_EN_MASK, PM4125_DIG_SWR_COMP_DISABLE);
            }
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_hphr_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_write_field(component, PM4125_ANA_HPHPA_CNP_CTL_1, PM4125_ANA_HPHPA_CNP_CTL_1_EN_MASK, PM4125_ANA_HPHPA_CNP_CTL_1_EN);
            snd_soc_component_write_field(component, PM4125_SWR_HPHPA_HD2, PM4125_SWR_HPHPA_HD2_RIGHT_MASK, PM4125_SWR_HPHPA_HD2_ENABLE);
        }
        SND_SOC_DAPM_POST_PMU => {
            if (*pm4125).comp2_enable {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHR_EN_MASK, PM4125_DIG_SWR_COMP_ENABLE);
                if (*pm4125).comp1_enable {
                    snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHL_EN_MASK, PM4125_DIG_SWR_COMP_ENABLE);
                }
                /*
                 * 5ms sleep is required after COMP is enabled
                 * as per HW requirement
                 */
                usleep_range(5000, 5100);
            } else {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHR_EN_MASK, PM4125_DIG_SWR_COMP_DISABLE);
            }
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX1_CTL, PM4125_DIG_SWR_DSM_DITHER_EN_MASK, PM4125_DIG_SWR_DSM_DITHER_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_GAIN_CTL, PM4125_DIG_SWR_RX1_EN_MASK, PM4125_DIG_SWR_RX_INPUT_ENABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_RX1_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_ENABLE);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_RX1_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_GAIN_CTL, PM4125_DIG_SWR_RX1_EN_MASK, PM4125_DIG_SWR_RX_INPUT_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX1_CTL, PM4125_DIG_SWR_DSM_DITHER_EN_MASK, PM4125_DIG_SWR_DSM_DITHER_ENABLE);
            if (*pm4125).comp2_enable {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_COMP_CTL_0, PM4125_DIG_SWR_COMP_HPHR_EN_MASK, PM4125_DIG_SWR_COMP_DISABLE);
            }
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_ear_lo_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX0_CTL, PM4125_DIG_SWR_DSM_DITHER_EN_MASK, PM4125_DIG_SWR_DSM_DITHER_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_RX0_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_ENABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_GAIN_CTL, PM4125_DIG_SWR_RX0_EN_MASK, PM4125_DIG_SWR_RX_INPUT_ENABLE);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_CLK_CTL, PM4125_DIG_SWR_RX0_CLK_EN_MASK, PM4125_DIG_SWR_RX_CLK_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX_GAIN_CTL, PM4125_DIG_SWR_RX0_EN_MASK, PM4125_DIG_SWR_RX_INPUT_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_RX0_CTL, PM4125_DIG_SWR_DSM_DITHER_EN_MASK, PM4125_DIG_SWR_DSM_DITHER_ENABLE);
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_hphl_wdt_irq(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    match event {
        SND_SOC_DAPM_POST_PMU => { usleep_range(5000, 5100); enable_irq((*pm4125).hphl_pdm_wd_int); }
        SND_SOC_DAPM_PRE_PMD => { disable_irq_nosync((*pm4125).hphl_pdm_wd_int); }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_hphr_wdt_irq(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    match event {
        SND_SOC_DAPM_POST_PMU => { usleep_range(5000, 5100); enable_irq((*pm4125).hphr_pdm_wd_int); }
        SND_SOC_DAPM_PRE_PMD => { disable_irq_nosync((*pm4125).hphr_pdm_wd_int); }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_hphr_pa(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            usleep_range(200, 210);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL1, PM4125_WDT_ENABLE_MASK, PM4125_WDT_ENABLE_RX1_M | PM4125_WDT_ENABLE_RX1_L);
        }
        SND_SOC_DAPM_POST_PMD => {
            usleep_range(5000, 5100);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL1, PM4125_WDT_ENABLE_MASK, 0x00);
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_hphl_pa(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            usleep_range(200, 210);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL0, PM4125_WDT_ENABLE_MASK, PM4125_WDT_ENABLE_RX0_M | PM4125_WDT_ENABLE_RX0_L);
        }
        SND_SOC_DAPM_POST_PMD => {
            usleep_range(5000, 5100);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL0, PM4125_WDT_ENABLE_MASK, 0x00);
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_lo_pa(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, PM4125_ANA_COMBOPA_CTL_5, 0x04, 0x00);
            usleep_range(1000, 1010);
            snd_soc_component_update_bits(component, PM4125_ANA_COMBOPA_CTL_4, 0x0F, 0x0F);
            usleep_range(1000, 1010);
            snd_soc_component_write_field(component, PM4125_ANA_COMBOPA_CTL, PM4125_ANA_COMBO_PA_SELECT_MASK, PM4125_ANA_COMBO_PA_SELECT_LO);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL0, PM4125_WDT_ENABLE_MASK, PM4125_WDT_ENABLE_RX0_M | PM4125_WDT_ENABLE_RX0_L);
        }
        SND_SOC_DAPM_POST_PMU => {
            usleep_range(5000, 5010);
            snd_soc_component_update_bits(component, PM4125_ANA_COMBOPA_CTL_4, 0x0F, 0x04);
        }
        SND_SOC_DAPM_POST_PMD => {
            usleep_range(2000, 2010);
            snd_soc_component_write_field(component, PM4125_ANA_COMBOPA_CTL, PM4125_ANA_COMBO_PA_SELECT_MASK, PM4125_ANA_COMBO_PA_SELECT_EAR);
            usleep_range(5000, 5100);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL0, PM4125_WDT_ENABLE_MASK, 0x00);
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_ear_pa(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, PM4125_ANA_COMBOPA_CTL_5, 0x04, 0x00);
            usleep_range(1000, 1010);
            snd_soc_component_update_bits(component, PM4125_ANA_COMBOPA_CTL_4, 0x0F, 0x0F);
            usleep_range(1000, 1010);
            snd_soc_component_update_bits(component, PM4125_ANA_COMBOPA_CTL, PM4125_ANA_COMBO_PA_SELECT_MASK, PM4125_ANA_COMBO_PA_SELECT_EAR);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL0, PM4125_WDT_ENABLE_MASK, PM4125_WDT_ENABLE_RX0_M | PM4125_WDT_ENABLE_RX0_L);
        }
        SND_SOC_DAPM_POST_PMU => {
            usleep_range(5000, 5010);
            snd_soc_component_update_bits(component, PM4125_ANA_COMBOPA_CTL_4, 0x0F, 0x04);
        }
        SND_SOC_DAPM_POST_PMD => {
            usleep_range(5000, 5010);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_PDM_WD_CTL0, PM4125_WDT_ENABLE_MASK, 0x00);
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_adc(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Enable BCS for Headset mic */
            if (*w).shift == 1 && (snd_soc_component_read(component, PM4125_ANA_TX_AMIC2) & 0x10) == 0 {
                set_bit(AMIC2_BCS_ENABLE, &mut (*pm4125).status_mask);
            }
            pm4125_global_mbias_enable(component);
            if (*w).shift != 0 {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1, PM4125_DIG_SWR_TX_ANA_TXD1_MODE_MASK, PM4125_DIG_SWR_TXD_MODE_NORMAL);
            } else {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1, PM4125_DIG_SWR_TX_ANA_TXD0_MODE_MASK, PM4125_DIG_SWR_TXD_MODE_NORMAL);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            if (*w).shift == 1 && test_bit(AMIC2_BCS_ENABLE, &(*pm4125).status_mask) {
                clear_bit(AMIC2_BCS_ENABLE, &mut (*pm4125).status_mask);
            }
            if (*w).shift != 0 {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1, PM4125_DIG_SWR_TX_ANA_TXD1_MODE_MASK, 0x00);
            } else {
                snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1, PM4125_DIG_SWR_TX_ANA_TXD0_MODE_MASK, 0x00);
            }
            pm4125_global_mbias_disable(component);
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_dmic(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let dmic_clk_reg: u16 = (*w).reg as u16;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_AMIC_CTL, PM4125_DIG_SWR_AMIC_SELECT_MASK, PM4125_DIG_SWR_AMIC_SELECT_DMIC1);
            snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, PM4125_DIG_SWR_DMIC1_CLK_EN_MASK, PM4125_DIG_SWR_DMIC1_CLK_ENABLE);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, dmic_clk_reg as c_uint, PM4125_DIG_SWR_DMIC1_CLK_EN_MASK, PM4125_DIG_SWR_DMIC1_CLK_DISABLE);
            snd_soc_component_write_field(component, PM4125_DIG_SWR_CDC_AMIC_CTL, PM4125_DIG_SWR_AMIC_SELECT_MASK, PM4125_DIG_SWR_AMIC_SELECT_AMIC3);
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_micbias_control(component: *mut snd_soc_component, micb_num: c_int, req: c_int, _is_dapm: bool) -> c_int {
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    let micb_index = micb_num - 1;
    let mut micb_reg: u16 = 0;
    let mut pullup_mask: u8 = 0;
    let mut enable_mask: u8 = 0;

    if micb_index < 0 || micb_index > PM4125_MAX_MICBIAS as c_int - 1 {
        dev_err((*component).dev, c_str!("%s: Invalid micbias index, micb_ind:%d\n"), c_str!("pm4125_micbias_control"), micb_index);
        return -EINVAL;
    }
    match micb_num {
        MIC_BIAS_1 => {
            micb_reg = PM4125_ANA_MICBIAS_MICB_1_2_EN as u16;
            pullup_mask = PM4125_ANA_MICBIAS_MICB1_PULL_UP_MASK as u8;
            enable_mask = 0x40;
        }
        MIC_BIAS_2 => {
            micb_reg = PM4125_ANA_MICBIAS_MICB_1_2_EN as u16;
            pullup_mask = PM4125_ANA_MICBIAS_MICB2_PULL_UP_MASK as u8;
            enable_mask = 0x04;
        }
        MIC_BIAS_3 => {
            micb_reg = PM4125_ANA_MICBIAS_MICB_3_EN as u16;
            pullup_mask = 0x02;
        }
        _ => {
            dev_err((*component).dev, c_str!("%s: Invalid micbias number: %d\n"), c_str!("pm4125_micbias_control"), micb_num);
            return -EINVAL;
        }
    }

    match req {
        MICB_PULLUP_ENABLE => {
            (*pm4125).pullup_ref[micb_index as usize] += 1;
            if (*pm4125).pullup_ref[micb_index as usize] == 1 && (*pm4125).micb_ref[micb_index as usize] == 0 {
                snd_soc_component_update_bits(component, micb_reg as c_uint, pullup_mask as c_uint, pullup_mask as c_uint);
            }
        }
        MICB_PULLUP_DISABLE => {
            if (*pm4125).pullup_ref[micb_index as usize] > 0 {
                (*pm4125).pullup_ref[micb_index as usize] -= 1;
            }
            if (*pm4125).pullup_ref[micb_index as usize] == 0 && (*pm4125).micb_ref[micb_index as usize] == 0 {
                snd_soc_component_update_bits(component, micb_reg as c_uint, pullup_mask as c_uint, 0x00);
            }
        }
        MICB_ENABLE => {
            (*pm4125).micb_ref[micb_index as usize] += 1;
            if (*pm4125).micb_ref[micb_index as usize] == 1 {
                pm4125_global_mbias_enable(component);
                snd_soc_component_update_bits(component, micb_reg as c_uint, enable_mask as c_uint, enable_mask as c_uint);
            }
        }
        MICB_DISABLE => {
            if (*pm4125).micb_ref[micb_index as usize] > 0 {
                (*pm4125).micb_ref[micb_index as usize] -= 1;
            }
            if (*pm4125).micb_ref[micb_index as usize] == 0 && (*pm4125).pullup_ref[micb_index as usize] > 0 {
                snd_soc_component_update_bits(component, micb_reg as c_uint, pullup_mask as c_uint, pullup_mask as c_uint);
                snd_soc_component_update_bits(component, micb_reg as c_uint, enable_mask as c_uint, 0x00);
                pm4125_global_mbias_disable(component);
            } else if (*pm4125).micb_ref[micb_index as usize] == 0 && (*pm4125).pullup_ref[micb_index as usize] == 0 {
                snd_soc_component_update_bits(component, micb_reg as c_uint, enable_mask as c_uint, 0x00);
                pm4125_global_mbias_disable(component);
            }
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_micbias(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let micb_num = (*w).shift;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if micb_num == MIC_BIAS_3 { pm4125_micbias_control(component, micb_num, MICB_PULLUP_ENABLE, true); }
            else { pm4125_micbias_control(component, micb_num, MICB_ENABLE, true); }
        }
        SND_SOC_DAPM_POST_PMU => { usleep_range(1000, 1100); }
        SND_SOC_DAPM_POST_PMD => {
            if micb_num == MIC_BIAS_3 { pm4125_micbias_control(component, micb_num, MICB_PULLUP_DISABLE, true); }
            else { pm4125_micbias_control(component, micb_num, MICB_DISABLE, true); }
        }
        _ => {}
    }
    0
}

unsafe fn pm4125_codec_enable_micbias_pullup(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let micb_num = (*w).shift;
    match event {
        SND_SOC_DAPM_PRE_PMU => { pm4125_micbias_control(component, micb_num, MICB_PULLUP_ENABLE, true); }
        SND_SOC_DAPM_POST_PMU => { usleep_range(1000, 1100); }
        SND_SOC_DAPM_POST_PMD => { pm4125_micbias_control(component, micb_num, MICB_PULLUP_DISABLE, true); }
        _ => {}
    }
    0
}

unsafe fn pm4125_connect_port(sdw_priv: *mut pm4125_sdw_priv, port_idx: u8, ch_id: u8, enable: bool) -> c_int {
    let port_config = &mut (*sdw_priv).port_config[(port_idx - 1) as usize] as *mut sdw_port_config;
    let ch_info = &(*sdw_priv).ch_info[ch_id as usize] as *const wcd_sdw_ch_info;
    let sdev = (*sdw_priv).sdev;
    let port_num = (*ch_info).port_num;
    let ch_mask = (*ch_info).ch_mask;
    let mstr_port_num: u8;
    let mstr_ch_mask: u8;

    (*port_config).num = port_num;
    mstr_port_num = (*sdev).m_port_map[port_num as usize];
    mstr_ch_mask = (*ch_info).master_ch_mask;

    if enable {
        (*port_config).ch_mask |= ch_mask;
        (*sdw_priv).master_channel_map[mstr_port_num as usize] |= mstr_ch_mask;
    } else {
        (*port_config).ch_mask &= !ch_mask;
        (*sdw_priv).master_channel_map[mstr_port_num as usize] &= !mstr_ch_mask;
    }
    0
}

unsafe fn pm4125_get_compander(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let hphr = (*mc).shift != 0;
    (*ucontrol).value.integer.value[0] = if hphr { (*pm4125).comp2_enable } else { (*pm4125).comp1_enable } as c_long;
    0
}

unsafe fn pm4125_set_compander(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    let sdw_priv = (*pm4125).sdw_priv[AIF1_PB];
    let value = (*ucontrol).value.integer.value[0] as c_int;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let hphr = (*mc).shift != 0;

    if hphr {
        if (value != 0) == (*pm4125).comp2_enable { return 0; }
        (*pm4125).comp2_enable = value != 0;
    } else {
        if (value != 0) == (*pm4125).comp1_enable { return 0; }
        (*pm4125).comp1_enable = value != 0;
    }

    let portidx = (*sdw_priv).ch_info[(*mc).reg as usize].port_num;
    pm4125_connect_port(sdw_priv, portidx, (*mc).reg as u8, value != 0);
    1
}

unsafe fn pm4125_get_swr_port(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mixer = (*kcontrol).private_value as *mut soc_mixer_control;
    let comp = snd_kcontrol_chip(kcontrol);
    let pm4125 = snd_soc_component_get_drvdata(comp) as *mut pm4125_priv;
    let dai_id = (*mixer).shift as usize;
    let ch_idx = (*mixer).reg as usize;
    let sdw_priv = (*pm4125).sdw_priv[dai_id];
    let portidx = (*sdw_priv).ch_info[ch_idx].port_num;
    (*ucontrol).value.integer.value[0] = (*sdw_priv).port_enable[portidx as usize] as c_long;
    0
}

unsafe fn pm4125_set_swr_port(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mixer = (*kcontrol).private_value as *mut soc_mixer_control;
    let comp = snd_kcontrol_chip(kcontrol);
    let pm4125 = snd_soc_component_get_drvdata(comp) as *mut pm4125_priv;
    let dai_id = (*mixer).shift as usize;
    let ch_idx = (*mixer).reg as usize;
    let sdw_priv = (*pm4125).sdw_priv[dai_id];
    let portidx = (*sdw_priv).ch_info[ch_idx].port_num;
    let enable = (*ucontrol).value.integer.value[0] != 0;

    if enable == (*sdw_priv).port_enable[portidx as usize] {
        pm4125_connect_port(sdw_priv, portidx, ch_idx as u8, enable);
        return 0;
    }
    (*sdw_priv).port_enable[portidx as usize] = enable;
    pm4125_connect_port(sdw_priv, portidx, ch_idx as u8, enable);
    1
}

unsafe fn pm4125_mbhc_bias_control(component: *mut snd_soc_component, enable: bool) {
    snd_soc_component_write_field(
        component,
        PM4125_ANA_MBHC_ELECT,
        PM4125_ANA_MBHC_ELECT_BIAS_EN_MASK,
        if enable { PM4125_ANA_MBHC_ELECT_BIAS_ENABLE } else { PM4125_ANA_MBHC_ELECT_BIAS_DISABLE },
    );
}

unsafe fn pm4125_mbhc_program_btn_thr(component: *mut snd_soc_component, btn_low: *mut c_int, btn_high: *mut c_int, num_btn: c_int, _is_micbias: bool) {
    if num_btn > WCD_MBHC_DEF_BUTTONS {
        dev_err((*component).dev, c_str!("%s: invalid number of buttons: %d\n"), c_str!("pm4125_mbhc_program_btn_thr"), num_btn);
        return;
    }
    let mut i = 0;
    while i < num_btn {
        let vth = ((*btn_high.add(i as usize) * 2) / 25) & 0x3F;
        snd_soc_component_write_field(component, PM4125_ANA_MBHC_BTN0_ZDET_VREF1 + i as c_uint, PM4125_ANA_MBHC_BTN0_THRESHOLD_MASK, (vth << 2) as c_uint);
        i += 1;
    }
}

static mbhc_cb: wcd_mbhc_cb = wcd_mbhc_cb {
    mbhc_bias: Some(pm4125_mbhc_bias_control),
    set_btn_thr: Some(pm4125_mbhc_program_btn_thr),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pm4125_mbhc_init(component: *mut snd_soc_component) -> c_int {
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    let intr_ids = &mut (*pm4125).intr_ids as *mut wcd_mbhc_intr;
    (*intr_ids).mbhc_sw_intr = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_MBHC_SW_DET);
    (*intr_ids).mbhc_btn_press_intr = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_MBHC_BUTTON_PRESS_DET);
    (*intr_ids).mbhc_btn_release_intr = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_MBHC_BUTTON_RELEASE_DET);
    (*intr_ids).mbhc_hs_ins_intr = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_MBHC_ELECT_INS_REM_LEG_DET);
    (*intr_ids).mbhc_hs_rem_intr = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_MBHC_ELECT_INS_REM_DET);
    (*intr_ids).hph_left_ocp = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_HPHL_OCP_INT);
    (*intr_ids).hph_right_ocp = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_HPHR_OCP_INT);
    (*pm4125).wcd_mbhc = wcd_mbhc_init(component, &mbhc_cb, intr_ids, pm4125_mbhc_fields.as_ptr(), false);
    if IS_ERR((*pm4125).wcd_mbhc as *const c_void) {
        return PTR_ERR((*pm4125).wcd_mbhc as *const c_void);
    }
    0
}

unsafe fn pm4125_mbhc_deinit(component: *mut snd_soc_component) {
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    wcd_mbhc_deinit((*pm4125).wcd_mbhc);
}

static pm4125_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_EXT!("HPHL_COMP Switch", PM4125_COMP_L, 0, 1, 0, pm4125_get_compander, pm4125_set_compander),
    SOC_SINGLE_EXT!("HPHR_COMP Switch", PM4125_COMP_R, 1, 1, 0, pm4125_get_compander, pm4125_set_compander),
    SOC_SINGLE_TLV!("HPHL Volume", PM4125_ANA_HPHPA_L_GAIN, 0, 20, 1, line_gain),
    SOC_SINGLE_TLV!("HPHR Volume", PM4125_ANA_HPHPA_R_GAIN, 0, 20, 1, line_gain),
    SOC_SINGLE_TLV!("ADC1 Volume", PM4125_ANA_TX_AMIC1, 0, 8, 0, analog_gain),
    SOC_SINGLE_TLV!("ADC2 Volume", PM4125_ANA_TX_AMIC2, 0, 8, 0, analog_gain),
    SOC_SINGLE_EXT!("HPHL Switch", PM4125_HPH_L, 0, 1, 0, pm4125_get_swr_port, pm4125_set_swr_port),
    SOC_SINGLE_EXT!("HPHR Switch", PM4125_HPH_R, 0, 1, 0, pm4125_get_swr_port, pm4125_set_swr_port),
    SOC_SINGLE_EXT!("ADC1 Switch", PM4125_ADC1, 1, 1, 0, pm4125_get_swr_port, pm4125_set_swr_port),
    SOC_SINGLE_EXT!("ADC2 Switch", PM4125_ADC2, 1, 1, 0, pm4125_get_swr_port, pm4125_set_swr_port),
];

static adc1_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];
static adc2_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];
static dmic1_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];
static dmic2_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];
static ear_rdac_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];
static lo_rdac_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];
static hphl_rdac_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];
static hphr_rdac_switch: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0)];

static adc2_mux_text: [*const c_char; 2] = [c_str!("INP2"), c_str!("INP3")];
static adc2_enum: soc_enum = SOC_ENUM_SINGLE!(PM4125_ANA_TX_AMIC2, 4, ARRAY_SIZE!(adc2_mux_text), adc2_mux_text);
static tx_adc2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("ADC2 MUX Mux", adc2_enum);

static pm4125_dapm_widgets: &[snd_soc_dapm_widget] = &[
    /* Input widgets */
    SND_SOC_DAPM_INPUT!("AMIC1"), SND_SOC_DAPM_INPUT!("AMIC2"), SND_SOC_DAPM_INPUT!("AMIC3"),
    SND_SOC_DAPM_INPUT!("IN1_HPHL"), SND_SOC_DAPM_INPUT!("IN2_HPHR"),
    /* TX widgets */
    SND_SOC_DAPM_ADC_E!("ADC1", core::ptr::null(), SND_SOC_NOPM, 0, 0, pm4125_codec_enable_adc, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_ADC_E!("ADC2", core::ptr::null(), SND_SOC_NOPM, 1, 0, pm4125_codec_enable_adc, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX!("ADC2 MUX", SND_SOC_NOPM, 0, 0, &tx_adc2_mux),
    /* TX mixers */
    SND_SOC_DAPM_MIXER!("ADC1_MIXER", SND_SOC_NOPM, 0, 0, adc1_switch, ARRAY_SIZE!(adc1_switch)),
    SND_SOC_DAPM_MIXER!("ADC2_MIXER", SND_SOC_NOPM, 1, 0, adc2_switch, ARRAY_SIZE!(adc2_switch)),
    /* MIC_BIAS widgets */
    SND_SOC_DAPM_SUPPLY!("MIC BIAS1", SND_SOC_NOPM, MIC_BIAS_1, 0, pm4125_codec_enable_micbias, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("MIC BIAS2", SND_SOC_NOPM, MIC_BIAS_2, 0, pm4125_codec_enable_micbias, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("MIC BIAS3", SND_SOC_NOPM, MIC_BIAS_3, 0, pm4125_codec_enable_micbias, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("PA_VPOS", SND_SOC_NOPM, 0, 0, None, 0),
    /* RX widgets */
    SND_SOC_DAPM_PGA_E!("EAR PGA", PM4125_ANA_COMBOPA_CTL, 7, 0, None, 0, pm4125_codec_enable_ear_pa, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA_E!("LO PGA", PM4125_ANA_COMBOPA_CTL, 7, 0, None, 0, pm4125_codec_enable_lo_pa, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA_E!("HPHL PGA", PM4125_ANA_HPHPA_CNP_CTL_2, 7, 0, None, 0, pm4125_codec_enable_hphl_pa, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA_E!("HPHR PGA", PM4125_ANA_HPHPA_CNP_CTL_2, 6, 0, None, 0, pm4125_codec_enable_hphr_pa, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("RDAC1", core::ptr::null(), SND_SOC_NOPM, 0, 0, pm4125_codec_hphl_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("RDAC2", core::ptr::null(), SND_SOC_NOPM, 0, 0, pm4125_codec_hphr_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("RDAC3", core::ptr::null(), SND_SOC_NOPM, 0, 0, pm4125_codec_ear_lo_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("HPHL_WDT_IRQ", SND_SOC_NOPM, 0, 0, pm4125_codec_enable_hphl_wdt_irq, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!("HPHR_WDT_IRQ", SND_SOC_NOPM, 0, 0, pm4125_codec_enable_hphr_wdt_irq, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!("RXCLK", SND_SOC_NOPM, 0, 0, pm4125_codec_enable_rxclk, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER_E!("RX1", SND_SOC_NOPM, 0, 0, None, 0, None, 0),
    SND_SOC_DAPM_MIXER_E!("RX2", SND_SOC_NOPM, 0, 0, None, 0, None, 0),
    /* RX mixer widgets */
    SND_SOC_DAPM_MIXER!("EAR_RDAC", SND_SOC_NOPM, 0, 0, ear_rdac_switch, ARRAY_SIZE!(ear_rdac_switch)),
    SND_SOC_DAPM_MIXER!("LO_RDAC", SND_SOC_NOPM, 0, 0, lo_rdac_switch, ARRAY_SIZE!(lo_rdac_switch)),
    SND_SOC_DAPM_MIXER!("HPHL_RDAC", SND_SOC_NOPM, 0, 0, hphl_rdac_switch, ARRAY_SIZE!(hphl_rdac_switch)),
    SND_SOC_DAPM_MIXER!("HPHR_RDAC", SND_SOC_NOPM, 0, 0, hphr_rdac_switch, ARRAY_SIZE!(hphr_rdac_switch)),
    /* TX output widgets */
    SND_SOC_DAPM_OUTPUT!("ADC1_OUTPUT"), SND_SOC_DAPM_OUTPUT!("ADC2_OUTPUT"),
    /* RX output widgets */
    SND_SOC_DAPM_OUTPUT!("EAR"), SND_SOC_DAPM_OUTPUT!("LO"), SND_SOC_DAPM_OUTPUT!("HPHL"), SND_SOC_DAPM_OUTPUT!("HPHR"),
    /* MIC_BIAS pull up widgets */
    SND_SOC_DAPM_SUPPLY!("VA MIC BIAS1", SND_SOC_NOPM, MIC_BIAS_1, 0, pm4125_codec_enable_micbias_pullup, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("VA MIC BIAS2", SND_SOC_NOPM, MIC_BIAS_2, 0, pm4125_codec_enable_micbias_pullup, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY!("VA MIC BIAS3", SND_SOC_NOPM, MIC_BIAS_3, 0, pm4125_codec_enable_micbias_pullup, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    /* TX widgets */
    SND_SOC_DAPM_ADC_E!("DMIC1", core::ptr::null(), PM4125_DIG_SWR_CDC_DMIC1_CTL, 0, 0, pm4125_codec_enable_dmic, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_ADC_E!("DMIC2", core::ptr::null(), PM4125_DIG_SWR_CDC_DMIC1_CTL, 1, 0, pm4125_codec_enable_dmic, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    /* TX mixer widgets */
    SND_SOC_DAPM_MIXER!("DMIC1_MIXER", SND_SOC_NOPM, 0, 0, dmic1_switch, ARRAY_SIZE!(dmic1_switch)),
    SND_SOC_DAPM_MIXER!("DMIC2_MIXER", SND_SOC_NOPM, 1, 0, dmic2_switch, ARRAY_SIZE!(dmic2_switch)),
    /* Output widgets */
    SND_SOC_DAPM_OUTPUT!("DMIC1_OUTPUT"), SND_SOC_DAPM_OUTPUT!("DMIC2_OUTPUT"),
];

static pm4125_audio_map: &[snd_soc_dapm_route] = &[
    route!("ADC1_OUTPUT", None, "ADC1_MIXER"), route!("ADC1_MIXER", Some("Switch"), "ADC1"), route!("ADC1", None, "AMIC1"),
    route!("ADC2_OUTPUT", None, "ADC2_MIXER"), route!("ADC2_MIXER", Some("Switch"), "ADC2"), route!("ADC2", None, "ADC2 MUX"),
    route!("ADC2 MUX", Some("INP3"), "AMIC3"), route!("ADC2 MUX", Some("INP2"), "AMIC2"),
    route!("IN1_HPHL", None, "PA_VPOS"), route!("RX1", None, "IN1_HPHL"), route!("RX1", None, "RXCLK"), route!("RX1", None, "HPHL_WDT_IRQ"),
    route!("RDAC1", None, "RX1"), route!("HPHL_RDAC", Some("Switch"), "RDAC1"), route!("HPHL PGA", None, "HPHL_RDAC"), route!("HPHL", None, "HPHL PGA"),
    route!("IN2_HPHR", None, "PA_VPOS"), route!("RX2", None, "IN2_HPHR"), route!("RX2", None, "RXCLK"), route!("RX2", None, "HPHR_WDT_IRQ"),
    route!("RDAC2", None, "RX2"), route!("HPHR_RDAC", Some("Switch"), "RDAC2"), route!("HPHR PGA", None, "HPHR_RDAC"), route!("HPHR", None, "HPHR PGA"),
    route!("RDAC3", None, "RX1"), route!("EAR_RDAC", Some("Switch"), "RDAC3"), route!("EAR PGA", None, "EAR_RDAC"), route!("EAR", None, "EAR PGA"),
    route!("LO_RDAC", Some("Switch"), "RDAC3"), route!("LO PGA", None, "LO_RDAC"), route!("LO", None, "LO PGA"),
    route!("DMIC1_OUTPUT", None, "DMIC1_MIXER"), route!("DMIC1_MIXER", Some("Switch"), "DMIC1"),
    route!("DMIC2_OUTPUT", None, "DMIC2_MIXER"), route!("DMIC2_MIXER", Some("Switch"), "DMIC2"),
];

unsafe fn pm4125_set_micbias_data(_dev: *mut device, pm4125: *mut pm4125_priv) -> c_int {
    regmap_update_bits((*pm4125).regmap, PM4125_ANA_MICBIAS_LDO_1_SETTING, PM4125_ANA_MICBIAS_MICB_OUT_VAL_MASK, (*pm4125).common.micb_vout[0]);
    0
}

unsafe extern "C" fn pm4125_wd_handle_irq(_irq: c_int, _data: *mut c_void) -> irqreturn_t {
    /*
     * HPHR/HPHL Watchdog interrupt threaded handler
     * Watchdog interrupts are expected to be enabled when switching on the HPHL/R
     * in order to make sure the interrupts are acked by the regmap_irq handler
     * io allow PDM sync. We could leave those interrupts masked but we would
     * not haveany valid way to enable/disable them without violating irq layers.
     *
     * The HPHR/HPHL Watchdog interrupts are handled by regmap_irq, so requesting
     * a threaded handler is the safest way to be able to ack those interrupts
     * without colliding with the regmap_irq setup.
     */
    IRQ_HANDLED
}

static pm4125_codec_irq_chip: irq_chip = irq_chip { name: c_str!("pm4125_codec"), ..unsafe { core::mem::zeroed() } };

unsafe extern "C" fn pm4125_codec_irq_chip_map(irqd: *mut irq_domain, virq: c_uint, hw: irq_hw_number_t) -> c_int {
    irq_set_chip_and_handler(virq, &pm4125_codec_irq_chip, handle_simple_irq);
    irq_set_nested_thread(virq, 1);
    irq_set_noprobe(virq);
    0
}

static pm4125_domain_ops: irq_domain_ops = irq_domain_ops { map: Some(pm4125_codec_irq_chip_map), ..unsafe { core::mem::zeroed() } };

unsafe fn pm4125_irq_init(pm4125: *mut pm4125_priv, dev: *mut device) -> c_int {
    (*pm4125).virq = irq_domain_create_linear(core::ptr::null_mut(), 1, &pm4125_domain_ops, core::ptr::null_mut());
    if (*pm4125).virq.is_null() {
        dev_err(dev, c_str!("%s: Failed to add IRQ domain\n"), c_str!("pm4125_irq_init"));
        return -EINVAL;
    }
    devm_regmap_add_irq_chip(dev, (*pm4125).regmap, irq_create_mapping((*pm4125).virq, 0), IRQF_ONESHOT, 0, (*pm4125).chip_desc, &mut (*pm4125).irq_chip)
}

unsafe fn pm4125_soc_codec_probe(component: *mut snd_soc_component) -> c_int {
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    let dev = (*component).dev;
    let mut ret: c_int;
    ret = sdw_slave_wait_for_init((*pm4125).tx_sdw_dev, 5000);
    if ret != 0 { return ret; }
    snd_soc_component_init_regmap(component, (*pm4125).regmap);
    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 { return ret; }
    pm4125_io_init((*pm4125).regmap);
    /* Set all interrupts as edge triggered */
    let mut i = 0;
    while i < pm4125_regmap_irq_chip.num_regs {
        regmap_write((*pm4125).regmap, PM4125_DIG_SWR_INTR_LEVEL_0 + i, 0);
        i += 1;
    }
    pm_runtime_put(dev);
    (*pm4125).hphr_pdm_wd_int = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_HPHR_PDM_WD_INT);
    (*pm4125).hphl_pdm_wd_int = regmap_irq_get_virq((*pm4125).irq_chip, PM4125_IRQ_HPHL_PDM_WD_INT);
    /* Request for watchdog interrupts */
    ret = devm_request_threaded_irq(dev, (*pm4125).hphr_pdm_wd_int, None, Some(pm4125_wd_handle_irq), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c_str!("HPHR PDM WDOG INT"), pm4125 as *mut c_void);
    if ret != 0 { dev_err(dev, c_str!("Failed to request HPHR wdt interrupt: %d\n"), ret); }
    ret = devm_request_threaded_irq(dev, (*pm4125).hphl_pdm_wd_int, None, Some(pm4125_wd_handle_irq), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c_str!("HPHL PDM WDOG INT"), pm4125 as *mut c_void);
    if ret != 0 { dev_err(dev, c_str!("Failed to request HPHL wdt interrupt: %d\n"), ret); }
    disable_irq_nosync((*pm4125).hphr_pdm_wd_int);
    disable_irq_nosync((*pm4125).hphl_pdm_wd_int);
    ret = pm4125_mbhc_init(component);
    if ret != 0 { dev_err((*component).dev, c_str!("mbhc initialization failed\n")); }
    ret
}

unsafe fn pm4125_soc_codec_remove(component: *mut snd_soc_component) {
    let pm4125 = snd_soc_component_get_drvdata(component) as *mut pm4125_priv;
    pm4125_mbhc_deinit(component);
    free_irq((*pm4125).hphl_pdm_wd_int, pm4125 as *mut c_void);
    free_irq((*pm4125).hphr_pdm_wd_int, pm4125 as *mut c_void);
}

unsafe fn pm4125_codec_set_jack(comp: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let pm4125 = dev_get_drvdata((*comp).dev) as *mut pm4125_priv;
    let mut ret = 0;
    if !jack.is_null() { ret = wcd_mbhc_start((*pm4125).wcd_mbhc, &mut (*pm4125).mbhc_cfg, jack); }
    else { wcd_mbhc_stop((*pm4125).wcd_mbhc); }
    ret
}

static soc_codec_dev_pm4125: snd_soc_component_driver = snd_soc_component_driver {
    name: c_str!("pm4125_codec"),
    probe: Some(pm4125_soc_codec_probe),
    remove: Some(pm4125_soc_codec_remove),
    controls: pm4125_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(pm4125_snd_controls),
    dapm_widgets: pm4125_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(pm4125_dapm_widgets),
    dapm_routes: pm4125_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(pm4125_audio_map),
    set_jack: Some(pm4125_codec_set_jack),
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pm4125_codec_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let pm4125 = dev_get_drvdata((*dai).dev) as *mut pm4125_priv;
    let sdw_priv = (*pm4125).sdw_priv[(*dai).id as usize];
    pm4125_sdw_hw_params(sdw_priv, substream, params, dai)
}

unsafe fn pm4125_codec_free(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let pm4125 = dev_get_drvdata((*dai).dev) as *mut pm4125_priv;
    let sdw_priv = (*pm4125).sdw_priv[(*dai).id as usize];
    sdw_stream_remove_slave((*sdw_priv).sdev, (*sdw_priv).sruntime)
}

unsafe fn pm4125_codec_set_sdw_stream(dai: *mut snd_soc_dai, stream: *mut c_void, _direction: c_int) -> c_int {
    let pm4125 = dev_get_drvdata((*dai).dev) as *mut pm4125_priv;
    let sdw_priv = (*pm4125).sdw_priv[(*dai).id as usize];
    (*sdw_priv).sruntime = stream;
    0
}

unsafe fn pm4125_get_channel_map(dai: *const snd_soc_dai, tx_num: *mut c_uint, tx_slot: *mut c_uint, rx_num: *mut c_uint, rx_slot: *mut c_uint) -> c_int {
    let pm4125 = dev_get_drvdata((*dai).dev) as *mut pm4125_priv;
    let sdw_priv = (*pm4125).sdw_priv[(*dai).id as usize];
    let mut i: c_int;
    match (*dai).id as usize {
        AIF1_PB => {
            if rx_slot.is_null() || rx_num.is_null() {
                dev_err((*dai).dev, c_str!("Invalid rx_slot %p or rx_num %p\n"), rx_slot, rx_num);
                return -EINVAL;
            }
            i = 0;
            while i < SDW_MAX_PORTS {
                *rx_slot.add(i as usize) = (*sdw_priv).master_channel_map[i as usize] as c_uint;
                i += 1;
            }
            *rx_num = i as c_uint;
        }
        AIF1_CAP => {
            if tx_slot.is_null() || tx_num.is_null() {
                dev_err((*dai).dev, c_str!("Invalid tx_slot %p or tx_num %p\n"), tx_slot, tx_num);
                return -EINVAL;
            }
            i = 0;
            while i < SDW_MAX_PORTS {
                *tx_slot.add(i as usize) = (*sdw_priv).master_channel_map[i as usize] as c_uint;
                i += 1;
            }
            *tx_num = i as c_uint;
        }
        _ => {}
    }
    0
}

static pm4125_sdw_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(pm4125_codec_hw_params),
    hw_free: Some(pm4125_codec_free),
    set_stream: Some(pm4125_codec_set_sdw_stream),
    get_channel_map: Some(pm4125_get_channel_map),
    ..unsafe { core::mem::zeroed() }
};

static mut pm4125_dais: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c_str!("pm4125-sdw-rx"),
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("PM4125 AIF Playback"),
            rates: PM4125_RATES | PM4125_FRAC_RATES,
            formats: PM4125_FORMATS,
            rate_min: 8000,
            rate_max: 384000,
            channels_min: 1,
            channels_max: 4,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &pm4125_sdw_dai_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("pm4125-sdw-tx"),
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("PM4125 AIF Capture"),
            rates: PM4125_RATES,
            formats: PM4125_FORMATS,
            rate_min: 8000,
            rate_max: 192000,
            channels_min: 1,
            channels_max: 4,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &pm4125_sdw_dai_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn pm4125_bind(dev: *mut device) -> c_int {
    let pm4125 = dev_get_drvdata(dev) as *mut pm4125_priv;
    let mut devlink: *mut device_link;
    let mut ret: c_int;
    /* Give the soundwire subdevices some more time to settle */
    usleep_range(15000, 15010);
    ret = component_bind_all(dev, pm4125 as *mut c_void);
    if ret != 0 {
        dev_err(dev, c_str!("Slave bind failed, ret = %d\n"), ret);
        return ret;
    }
    (*pm4125).rxdev = of_sdw_find_device_by_node((*pm4125).rxnode);
    if (*pm4125).rxdev.is_null() {
        dev_err(dev, c_str!("could not find rxslave with matching of node\n"));
        ret = -EINVAL;
        component_unbind_all(dev, pm4125 as *mut c_void);
        return ret;
    }
    (*pm4125).sdw_priv[AIF1_PB] = dev_get_drvdata((*pm4125).rxdev) as *mut pm4125_sdw_priv;
    (*(*pm4125).sdw_priv[AIF1_PB]).pm4125 = pm4125;
    (*pm4125).txdev = of_sdw_find_device_by_node((*pm4125).txnode);
    if (*pm4125).txdev.is_null() {
        dev_err(dev, c_str!("could not find txslave with matching of node\n"));
        put_device((*pm4125).rxdev);
        component_unbind_all(dev, pm4125 as *mut c_void);
        return -EINVAL;
    }
    (*pm4125).sdw_priv[AIF1_CAP] = dev_get_drvdata((*pm4125).txdev) as *mut pm4125_sdw_priv;
    (*(*pm4125).sdw_priv[AIF1_CAP]).pm4125 = pm4125;
    (*pm4125).tx_sdw_dev = dev_to_sdw_dev((*pm4125).txdev);
    /*
     * As TX is the main CSR reg interface, which should not be suspended first.
     * expicilty add the dependency link
     */
    devlink = device_link_add((*pm4125).rxdev, (*pm4125).txdev, DL_FLAG_STATELESS | DL_FLAG_PM_RUNTIME);
    if devlink.is_null() {
        dev_err(dev, c_str!("Could not devlink TX and RX\n"));
        put_device((*pm4125).txdev); put_device((*pm4125).rxdev); component_unbind_all(dev, pm4125 as *mut c_void);
        return -EINVAL;
    }
    devlink = device_link_add(dev, (*pm4125).txdev, DL_FLAG_STATELESS | DL_FLAG_PM_RUNTIME);
    if devlink.is_null() {
        dev_err(dev, c_str!("Could not devlink PM4125 and TX\n"));
        device_link_remove((*pm4125).rxdev, (*pm4125).txdev); put_device((*pm4125).txdev); put_device((*pm4125).rxdev); component_unbind_all(dev, pm4125 as *mut c_void);
        return -EINVAL;
    }
    devlink = device_link_add(dev, (*pm4125).rxdev, DL_FLAG_STATELESS | DL_FLAG_PM_RUNTIME);
    if devlink.is_null() {
        dev_err(dev, c_str!("Could not devlink PM4125 and RX\n"));
        device_link_remove(dev, (*pm4125).txdev); device_link_remove((*pm4125).rxdev, (*pm4125).txdev); put_device((*pm4125).txdev); put_device((*pm4125).rxdev); component_unbind_all(dev, pm4125 as *mut c_void);
        return -EINVAL;
    }
    (*pm4125).regmap = (*(*pm4125).sdw_priv[AIF1_CAP]).regmap;
    if (*pm4125).regmap.is_null() {
        dev_err(dev, c_str!("could not get TX device regmap\n"));
        device_link_remove(dev, (*pm4125).rxdev); device_link_remove(dev, (*pm4125).txdev); device_link_remove((*pm4125).rxdev, (*pm4125).txdev); put_device((*pm4125).txdev); put_device((*pm4125).rxdev); component_unbind_all(dev, pm4125 as *mut c_void);
        return -EINVAL;
    }
    ret = pm4125_irq_init(pm4125, dev);
    if ret != 0 {
        dev_err(dev, c_str!("IRQ init failed: %d\n"), ret);
        device_link_remove(dev, (*pm4125).rxdev); device_link_remove(dev, (*pm4125).txdev); device_link_remove((*pm4125).rxdev, (*pm4125).txdev); put_device((*pm4125).txdev); put_device((*pm4125).rxdev); component_unbind_all(dev, pm4125 as *mut c_void);
        return ret;
    }
    (*(*pm4125).sdw_priv[AIF1_PB]).slave_irq = (*pm4125).virq;
    (*(*pm4125).sdw_priv[AIF1_CAP]).slave_irq = (*pm4125).virq;
    pm4125_set_micbias_data(dev, pm4125);
    ret = snd_soc_register_component(dev, &soc_codec_dev_pm4125, pm4125_dais.as_mut_ptr(), ARRAY_SIZE!(pm4125_dais));
    if ret == 0 { return ret; }
    dev_err(dev, c_str!("Codec registration failed\n"));
    device_link_remove(dev, (*pm4125).rxdev); device_link_remove(dev, (*pm4125).txdev); device_link_remove((*pm4125).rxdev, (*pm4125).txdev); put_device((*pm4125).txdev); put_device((*pm4125).rxdev); component_unbind_all(dev, pm4125 as *mut c_void);
    ret
}

unsafe fn pm4125_unbind(dev: *mut device) {
    let pm4125 = dev_get_drvdata(dev) as *mut pm4125_priv;
    snd_soc_unregister_component(dev);
    devm_regmap_del_irq_chip(dev, irq_find_mapping((*pm4125).virq, 0), (*pm4125).irq_chip);
    device_link_remove(dev, (*pm4125).txdev);
    device_link_remove(dev, (*pm4125).rxdev);
    device_link_remove((*pm4125).rxdev, (*pm4125).txdev);
    put_device((*pm4125).txdev);
    put_device((*pm4125).rxdev);
    component_unbind_all(dev, pm4125 as *mut c_void);
}

static pm4125_comp_ops: component_master_ops = component_master_ops {
    bind: Some(pm4125_bind),
    unbind: Some(pm4125_unbind),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pm4125_add_slave_components(pm4125: *mut pm4125_priv, dev: *mut device, matchptr: *mut *mut component_match) -> c_int {
    let np = (*dev).of_node;
    (*pm4125).rxnode = of_parse_phandle(np, c_str!("qcom,rx-device"), 0);
    if (*pm4125).rxnode.is_null() {
        return dev_err_probe(dev, -ENODEV, c_str!("Couldn't parse phandle to qcom,rx-device\n"));
    }
    component_match_add_release(dev, matchptr, component_release_of, component_compare_of, (*pm4125).rxnode as *mut c_void);
    (*pm4125).txnode = of_parse_phandle(np, c_str!("qcom,tx-device"), 0);
    if (*pm4125).txnode.is_null() {
        return dev_err_probe(dev, -ENODEV, c_str!("Couldn't parse phandle to qcom,tx-device\n"));
    }
    component_match_add_release(dev, matchptr, component_release_of, component_compare_of, (*pm4125).txnode as *mut c_void);
    0
}

unsafe fn pm4125_probe(pdev: *mut platform_device) -> c_int {
    let mut match_: *mut component_match = core::ptr::null_mut();
    let dev = &mut (*pdev).dev as *mut device;
    let mut chip_desc: *mut regmap_irq_chip;
    let mut ret: c_int;
    let pm4125 = devm_kzalloc(dev, core::mem::size_of::<pm4125_priv>(), GFP_KERNEL) as *mut pm4125_priv;
    if pm4125.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, pm4125 as *mut c_void);
    chip_desc = devm_kmemdup(dev, &pm4125_regmap_irq_chip as *const _ as *const c_void, core::mem::size_of_val(&pm4125_regmap_irq_chip), GFP_KERNEL) as *mut regmap_irq_chip;
    if chip_desc.is_null() { return -ENOMEM; }
    (*chip_desc).irq_drv_data = pm4125 as *mut c_void;
    (*pm4125).chip_desc = chip_desc;
    ret = devm_regulator_bulk_get_enable(dev, ARRAY_SIZE!(pm4125_power_supplies), pm4125_power_supplies.as_ptr());
    if ret != 0 { return dev_err_probe(dev, ret, c_str!("Failed to get and enable supplies\n")); }
    (*pm4125).spmi_regmap = dev_get_regmap((*(*pdev).dev.parent), core::ptr::null());
    if (*pm4125).spmi_regmap.is_null() { return -ENXIO; }
    pm4125_reset(pm4125);
    (*pm4125).common.dev = dev;
    (*pm4125).common.max_bias = 3;
    ret = wcd_dt_parse_micbias_info(&mut (*pm4125).common);
    if ret != 0 { return dev_err_probe(dev, ret, c_str!("Failed to get micbias\n")); }
    atomic_set(&mut (*pm4125).gloal_mbias_cnt, 0);
    let cfg = &mut (*pm4125).mbhc_cfg as *mut wcd_mbhc_config;
    (*cfg).mbhc_micbias = MIC_BIAS_2;
    (*cfg).anc_micbias = MIC_BIAS_2;
    (*cfg).v_hs_max = WCD_MBHC_HS_V_MAX;
    (*cfg).num_btn = PM4125_MBHC_MAX_BUTTONS;
    (*cfg).micb_mv = (*pm4125).common.micb_mv[1];
    (*cfg).linein_th = 5000;
    (*cfg).hs_thr = 1700;
    (*cfg).hph_thr = 50;
    wcd_dt_parse_mbhc_data(dev, &mut (*pm4125).mbhc_cfg);
    ret = pm4125_add_slave_components(pm4125, dev, &mut match_);
    if ret != 0 { return ret; }
    ret = component_master_add_with_match(dev, &pm4125_comp_ops, match_);
    if ret != 0 { return ret; }
    pm_runtime_set_autosuspend_delay(dev, 1000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_idle(dev);
    0
}

unsafe fn pm4125_remove(pdev: *mut platform_device) {
    let dev = &mut (*pdev).dev as *mut device;
    component_master_del(&mut (*pdev).dev, &pm4125_comp_ops);
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    pm_runtime_dont_use_autosuspend(dev);
}

static pm4125_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c_str!("qcom,pm4125-codec"), ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, pm4125_of_match);

static mut pm4125_codec_driver: platform_driver = platform_driver {
    probe: Some(pm4125_probe),
    remove: Some(pm4125_remove),
    driver: device_driver {
        name: c_str!("pm4125_codec"),
        of_match_table: pm4125_of_match.as_ptr(),
        suppress_bind_attrs: true,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(pm4125_codec_driver);
MODULE_DESCRIPTION!("PM4125 audio codec driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
