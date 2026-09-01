// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2016, The Linux Foundation. All rights reserved.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type irqreturn_t = c_uint;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    if h == 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)
    }
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const CDC_D_REVISION1: u32 = 0xf000;
const CDC_D_PERPH_SUBTYPE: u32 = 0xf005;
const CDC_D_INT_EN_SET: u32 = 0xf015;
const CDC_D_INT_EN_CLR: u32 = 0xf016;
const MBHC_SWITCH_INT: u32 = BIT(7);
const MBHC_MIC_ELECTRICAL_INS_REM_DET: u32 = BIT(6);
const MBHC_BUTTON_PRESS_DET: u32 = BIT(5);
const MBHC_BUTTON_RELEASE_DET: u32 = BIT(4);
const CDC_D_CDC_RST_CTL: u32 = 0xf046;
const RST_CTL_DIG_SW_RST_N_MASK: u32 = BIT(7);
const RST_CTL_DIG_SW_RST_N_RESET: u32 = 0;
const RST_CTL_DIG_SW_RST_N_REMOVE_RESET: u32 = BIT(7);

const CDC_D_CDC_TOP_CLK_CTL: u32 = 0xf048;
const TOP_CLK_CTL_A_MCLK_MCLK2_EN_MASK: u32 = BIT(2) | BIT(3);
const TOP_CLK_CTL_A_MCLK_EN_ENABLE: u32 = BIT(2);
const TOP_CLK_CTL_A_MCLK2_EN_ENABLE: u32 = BIT(3);

const CDC_D_CDC_ANA_CLK_CTL: u32 = 0xf049;
const ANA_CLK_CTL_EAR_HPHR_CLK_EN_MASK: u32 = BIT(0);
const ANA_CLK_CTL_EAR_HPHR_CLK_EN: u32 = BIT(0);
const ANA_CLK_CTL_EAR_HPHL_CLK_EN: u32 = BIT(1);
const ANA_CLK_CTL_SPKR_CLK_EN_MASK: u32 = BIT(4);
const ANA_CLK_CTL_SPKR_CLK_EN: u32 = BIT(4);
const ANA_CLK_CTL_TXA_CLK25_EN: u32 = BIT(5);

const CDC_D_CDC_DIG_CLK_CTL: u32 = 0xf04A;
const DIG_CLK_CTL_RXD1_CLK_EN: u32 = BIT(0);
const DIG_CLK_CTL_RXD2_CLK_EN: u32 = BIT(1);
const DIG_CLK_CTL_RXD3_CLK_EN: u32 = BIT(2);
const DIG_CLK_CTL_D_MBHC_CLK_EN_MASK: u32 = BIT(3);
const DIG_CLK_CTL_D_MBHC_CLK_EN: u32 = BIT(3);
const DIG_CLK_CTL_TXD_CLK_EN: u32 = BIT(4);
const DIG_CLK_CTL_NCP_CLK_EN_MASK: u32 = BIT(6);
const DIG_CLK_CTL_NCP_CLK_EN: u32 = BIT(6);
const DIG_CLK_CTL_RXD_PDM_CLK_EN_MASK: u32 = BIT(7);
const DIG_CLK_CTL_RXD_PDM_CLK_EN: u32 = BIT(7);

const CDC_D_CDC_CONN_TX1_CTL: u32 = 0xf050;
const CONN_TX1_SERIAL_TX1_MUX: u32 = GENMASK(1, 0);
const CONN_TX1_SERIAL_TX1_ADC_1: u32 = 0x0;
const CONN_TX1_SERIAL_TX1_RX_PDM_LB: u32 = 0x1;
const CONN_TX1_SERIAL_TX1_ZERO: u32 = 0x2;

const CDC_D_CDC_CONN_TX2_CTL: u32 = 0xf051;
const CONN_TX2_SERIAL_TX2_MUX: u32 = GENMASK(1, 0);
const CONN_TX2_SERIAL_TX2_ADC_2: u32 = 0x0;
const CONN_TX2_SERIAL_TX2_RX_PDM_LB: u32 = 0x1;
const CONN_TX2_SERIAL_TX2_ZERO: u32 = 0x2;
const CDC_D_CDC_CONN_HPHR_DAC_CTL: u32 = 0xf052;
const CDC_D_CDC_CONN_RX1_CTL: u32 = 0xf053;
const CDC_D_CDC_CONN_RX2_CTL: u32 = 0xf054;
const CDC_D_CDC_CONN_RX3_CTL: u32 = 0xf055;
const CDC_D_CDC_CONN_RX_LB_CTL: u32 = 0xf056;
const CDC_D_SEC_ACCESS: u32 = 0xf0D0;
const CDC_D_PERPH_RESET_CTL3: u32 = 0xf0DA;
const CDC_D_PERPH_RESET_CTL4: u32 = 0xf0DB;
const CDC_A_REVISION1: u32 = 0xf100;
const CDC_A_REVISION2: u32 = 0xf101;
const CDC_A_REVISION3: u32 = 0xf102;
const CDC_A_REVISION4: u32 = 0xf103;
const CDC_A_PERPH_TYPE: u32 = 0xf104;
const CDC_A_PERPH_SUBTYPE: u32 = 0xf105;
const CDC_A_INT_RT_STS: u32 = 0xf110;
const CDC_A_INT_SET_TYPE: u32 = 0xf111;
const CDC_A_INT_POLARITY_HIGH: u32 = 0xf112;
const CDC_A_INT_POLARITY_LOW: u32 = 0xf113;
const CDC_A_INT_LATCHED_CLR: u32 = 0xf114;
const CDC_A_INT_EN_SET: u32 = 0xf115;
const CDC_A_INT_EN_CLR: u32 = 0xf116;
const CDC_A_INT_LATCHED_STS: u32 = 0xf118;
const CDC_A_INT_PENDING_STS: u32 = 0xf119;
const CDC_A_INT_MID_SEL: u32 = 0xf11A;
const CDC_A_INT_PRIORITY: u32 = 0xf11B;
const CDC_A_MICB_1_EN: u32 = 0xf140;
const MICB_1_EN_MICB_ENABLE: u32 = BIT(7);
const MICB_1_EN_BYP_CAP_MASK: u32 = BIT(6);
const MICB_1_EN_NO_EXT_BYP_CAP: u32 = BIT(6);
const MICB_1_EN_EXT_BYP_CAP: u32 = 0;
const MICB_1_EN_PULL_DOWN_EN_MASK: u32 = BIT(5);
const MICB_1_EN_PULL_DOWN_EN_ENABLE: u32 = BIT(5);
const MICB_1_EN_OPA_STG2_TAIL_CURR_MASK: u32 = GENMASK(3, 1);
const MICB_1_EN_OPA_STG2_TAIL_CURR_1_60UA: u32 = 0x4;
const MICB_1_EN_PULL_UP_EN_MASK: u32 = BIT(4);
const MICB_1_EN_TX3_GND_SEL_MASK: u32 = BIT(0);
const MICB_1_EN_TX3_GND_SEL_TX_GND: u32 = 0;

const CDC_A_MICB_1_VAL: u32 = 0xf141;
const MICB_MIN_VAL: u32 = 1600;
const MICB_STEP_SIZE: u32 = 50;
const fn MICB_VOLTAGE_REGVAL(v: u32) -> u32 {
    ((v - MICB_MIN_VAL) / MICB_STEP_SIZE) << 3
}
const MICB_1_VAL_MICB_OUT_VAL_MASK: u32 = GENMASK(7, 3);
const MICB_1_VAL_MICB_OUT_VAL_V2P70V: u32 = 0x16 << 3;
const MICB_1_VAL_MICB_OUT_VAL_V1P80V: u32 = 0x4 << 3;
const CDC_A_MICB_1_CTL: u32 = 0xf142;

const MICB_1_CTL_CFILT_REF_SEL_MASK: u32 = BIT(1);
const MICB_1_CTL_CFILT_REF_SEL_HPF_REF: u32 = BIT(1);
const MICB_1_CTL_EXT_PRECHARG_EN_MASK: u32 = BIT(5);
const MICB_1_CTL_EXT_PRECHARG_EN_ENABLE: u32 = BIT(5);
const MICB_1_CTL_INT_PRECHARG_BYP_MASK: u32 = BIT(6);
const MICB_1_CTL_INT_PRECHARG_BYP_EXT_PRECHRG_SEL: u32 = BIT(6);

const CDC_A_MICB_1_INT_RBIAS: u32 = 0xf143;
const MICB_1_INT_TX1_INT_RBIAS_EN_MASK: u32 = BIT(7);
const MICB_1_INT_TX1_INT_RBIAS_EN_ENABLE: u32 = BIT(7);
const MICB_1_INT_TX1_INT_RBIAS_EN_DISABLE: u32 = 0;
const MICB_1_INT_TX1_INT_PULLUP_EN_MASK: u32 = BIT(6);
const MICB_1_INT_TX1_INT_PULLUP_EN_TX1N_TO_MICBIAS: u32 = BIT(6);
const MICB_1_INT_TX1_INT_PULLUP_EN_TX1N_TO_GND: u32 = 0;
const MICB_1_INT_TX2_INT_RBIAS_EN_MASK: u32 = BIT(4);
const MICB_1_INT_TX2_INT_RBIAS_EN_ENABLE: u32 = BIT(4);
const MICB_1_INT_TX2_INT_RBIAS_EN_DISABLE: u32 = 0;
const MICB_1_INT_TX2_INT_PULLUP_EN_MASK: u32 = BIT(3);
const MICB_1_INT_TX2_INT_PULLUP_EN_TX1N_TO_MICBIAS: u32 = BIT(3);
const MICB_1_INT_TX2_INT_PULLUP_EN_TX1N_TO_GND: u32 = 0;
const MICB_1_INT_TX3_INT_RBIAS_EN_MASK: u32 = BIT(1);
const MICB_1_INT_TX3_INT_RBIAS_EN_ENABLE: u32 = BIT(1);
const MICB_1_INT_TX3_INT_RBIAS_EN_DISABLE: u32 = 0;
const MICB_1_INT_TX3_INT_PULLUP_EN_MASK: u32 = BIT(0);
const MICB_1_INT_TX3_INT_PULLUP_EN_TX1N_TO_MICBIAS: u32 = BIT(0);
const MICB_1_INT_TX3_INT_PULLUP_EN_TX1N_TO_GND: u32 = 0;

const CDC_A_MICB_2_EN: u32 = 0xf144;
const CDC_A_MICB_2_EN_ENABLE: u32 = BIT(7);
const CDC_A_MICB_2_PULL_DOWN_EN_MASK: u32 = BIT(5);
const CDC_A_MICB_2_PULL_DOWN_EN: u32 = BIT(5);
const CDC_A_TX_1_2_ATEST_CTL_2: u32 = 0xf145;
const CDC_A_MASTER_BIAS_CTL: u32 = 0xf146;
const CDC_A_MBHC_DET_CTL_1: u32 = 0xf147;
const CDC_A_MBHC_DET_CTL_L_DET_EN: u32 = BIT(7);
const CDC_A_MBHC_DET_CTL_GND_DET_EN: u32 = BIT(6);
const CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_INSERTION: u32 = BIT(5);
const CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_REMOVAL: u32 = 0;
const CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_MASK: u32 = BIT(5);
const CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_SHIFT: u32 = 5;
const CDC_A_MBHC_DET_CTL_MIC_CLAMP_CTL_AUTO: u32 = BIT(4);
const CDC_A_MBHC_DET_CTL_MIC_CLAMP_CTL_MANUAL: u32 = BIT(3);
const CDC_A_MBHC_DET_CTL_MIC_CLAMP_CTL_MASK: u32 = GENMASK(4, 3);
const CDC_A_MBHC_DET_CTL_MBHC_BIAS_EN: u32 = BIT(2);
const CDC_A_MBHC_DET_CTL_2: u32 = 0xf150;
const CDC_A_MBHC_DET_CTL_HS_L_DET_PULL_UP_CTRL_I_3P0: u32 = BIT(7) | BIT(6);
const CDC_A_MBHC_DET_CTL_HS_L_DET_COMPA_CTRL_V0P9_VDD: u32 = BIT(5);
const CDC_A_PLUG_TYPE_MASK: u32 = GENMASK(4, 3);
const CDC_A_HPHL_PLUG_TYPE_NO: u32 = BIT(4);
const CDC_A_GND_PLUG_TYPE_NO: u32 = BIT(3);
const CDC_A_MBHC_DET_CTL_HPHL_100K_TO_GND_EN_MASK: u32 = BIT(0);
const CDC_A_MBHC_DET_CTL_HPHL_100K_TO_GND_EN: u32 = BIT(0);
const CDC_A_MBHC_FSM_CTL: u32 = 0xf151;
const CDC_A_MBHC_FSM_CTL_MBHC_FSM_EN: u32 = BIT(7);
const CDC_A_MBHC_FSM_CTL_MBHC_FSM_EN_MASK: u32 = BIT(7);
const CDC_A_MBHC_FSM_CTL_BTN_ISRC_CTRL_I_100UA: u32 = 0x3 << 4;
const CDC_A_MBHC_FSM_CTL_BTN_ISRC_CTRL_MASK: u32 = GENMASK(6, 4);
const CDC_A_MBHC_DBNC_TIMER: u32 = 0xf152;
const CDC_A_MBHC_DBNC_TIMER_BTN_DBNC_T_16MS: u32 = BIT(3);
const CDC_A_MBHC_DBNC_TIMER_INSREM_DBNC_T_256_MS: u32 = 0x9 << 4;
const CDC_A_MBHC_BTN0_ZDET_CTL_0: u32 = 0xf153;
const CDC_A_MBHC_BTN1_ZDET_CTL_1: u32 = 0xf154;
const CDC_A_MBHC_BTN2_ZDET_CTL_2: u32 = 0xf155;
const CDC_A_MBHC_BTN3_CTL: u32 = 0xf156;
const CDC_A_MBHC_BTN4_CTL: u32 = 0xf157;
const CDC_A_MBHC_BTN_VREF_FINE_SHIFT: u32 = 2;
const CDC_A_MBHC_BTN_VREF_FINE_MASK: u32 = GENMASK(4, 2);
const CDC_A_MBHC_BTN_VREF_COARSE_MASK: u32 = GENMASK(7, 5);
const CDC_A_MBHC_BTN_VREF_COARSE_SHIFT: u32 = 5;
const CDC_A_MBHC_BTN_VREF_MASK: u32 =
    CDC_A_MBHC_BTN_VREF_COARSE_MASK | CDC_A_MBHC_BTN_VREF_FINE_MASK;
const CDC_A_MBHC_RESULT_1: u32 = 0xf158;
const CDC_A_MBHC_RESULT_1_BTN_RESULT_MASK: u32 = GENMASK(4, 0);
const CDC_A_TX_1_EN: u32 = 0xf160;
const CDC_A_TX_2_EN: u32 = 0xf161;
const CDC_A_TX_1_2_TEST_CTL_1: u32 = 0xf162;
const CDC_A_TX_1_2_TEST_CTL_2: u32 = 0xf163;
const CDC_A_TX_1_2_ATEST_CTL: u32 = 0xf164;
const CDC_A_TX_1_2_OPAMP_BIAS: u32 = 0xf165;
const CDC_A_TX_3_EN: u32 = 0xf167;
const CDC_A_NCP_EN: u32 = 0xf180;
const CDC_A_NCP_CLK: u32 = 0xf181;
const CDC_A_NCP_FBCTRL: u32 = 0xf183;
const CDC_A_NCP_FBCTRL_FB_CLK_INV_MASK: u32 = BIT(5);
const CDC_A_NCP_FBCTRL_FB_CLK_INV: u32 = BIT(5);
const CDC_A_NCP_BIAS: u32 = 0xf184;
const CDC_A_NCP_VCTRL: u32 = 0xf185;
const CDC_A_NCP_TEST: u32 = 0xf186;
const CDC_A_NCP_CLIM_ADDR: u32 = 0xf187;
const CDC_A_RX_CLOCK_DIVIDER: u32 = 0xf190;
const CDC_A_RX_COM_OCP_CTL: u32 = 0xf191;
const CDC_A_RX_COM_OCP_COUNT: u32 = 0xf192;
const CDC_A_RX_COM_BIAS_DAC: u32 = 0xf193;
const RX_COM_BIAS_DAC_RX_BIAS_EN_MASK: u32 = BIT(7);
const RX_COM_BIAS_DAC_RX_BIAS_EN_ENABLE: u32 = BIT(7);
const RX_COM_BIAS_DAC_DAC_REF_EN_MASK: u32 = BIT(0);
const RX_COM_BIAS_DAC_DAC_REF_EN_ENABLE: u32 = BIT(0);
const CDC_A_RX_HPH_BIAS_PA: u32 = 0xf194;
const CDC_A_RX_HPH_BIAS_LDO_OCP: u32 = 0xf195;
const CDC_A_RX_HPH_BIAS_CNP: u32 = 0xf196;
const CDC_A_RX_HPH_CNP_EN: u32 = 0xf197;
const CDC_A_RX_HPH_L_TEST: u32 = 0xf19A;
const CDC_A_RX_HPH_L_PA_DAC_CTL: u32 = 0xf19B;
const RX_HPA_L_PA_DAC_CTL_DATA_RESET_MASK: u32 = BIT(1);
const RX_HPA_L_PA_DAC_CTL_DATA_RESET_RESET: u32 = BIT(1);
const CDC_A_RX_HPH_R_TEST: u32 = 0xf19C;
const CDC_A_RX_HPH_R_PA_DAC_CTL: u32 = 0xf19D;
const RX_HPH_R_PA_DAC_CTL_DATA_RESET: u32 = BIT(1);
const RX_HPH_R_PA_DAC_CTL_DATA_RESET_MASK: u32 = BIT(1);
const CDC_A_RX_EAR_CTL: u32 = 0xf19E;
const RX_EAR_CTL_SPK_VBAT_LDO_EN_MASK: u32 = BIT(0);
const RX_EAR_CTL_SPK_VBAT_LDO_EN_ENABLE: u32 = BIT(0);
const RX_EAR_CTL_PA_EAR_PA_EN_MASK: u32 = BIT(6);
const RX_EAR_CTL_PA_EAR_PA_EN_ENABLE: u32 = BIT(6);
const RX_EAR_CTL_PA_SEL_MASK: u32 = BIT(7);
const RX_EAR_CTL_PA_SEL: u32 = BIT(7);
const CDC_A_RX_EAR_STATUS: u32 = 0xf1A1;
const CDC_A_SPKR_DAC_CTL: u32 = 0xf1B0;
const SPKR_DAC_CTL_DAC_RESET_MASK: u32 = BIT(4);
const SPKR_DAC_CTL_DAC_RESET_NORMAL: u32 = 0;
const CDC_A_SPKR_DRV_CTL: u32 = 0xf1B2;
const SPKR_DRV_CTL_DEF_MASK: u32 = 0xEF;
const SPKR_DRV_CLASSD_PA_EN_MASK: u32 = BIT(7);
const SPKR_DRV_CLASSD_PA_EN_ENABLE: u32 = BIT(7);
const SPKR_DRV_CAL_EN: u32 = BIT(6);
const SPKR_DRV_SETTLE_EN: u32 = BIT(5);
const SPKR_DRV_FW_EN: u32 = BIT(3);
const SPKR_DRV_BOOST_SET: u32 = BIT(2);
const SPKR_DRV_CMFB_SET: u32 = BIT(1);
const SPKR_DRV_GAIN_SET: u32 = BIT(0);
const SPKR_DRV_CTL_DEF_VAL: u32 = SPKR_DRV_CLASSD_PA_EN_ENABLE
    | SPKR_DRV_CAL_EN
    | SPKR_DRV_SETTLE_EN
    | SPKR_DRV_FW_EN
    | SPKR_DRV_BOOST_SET
    | SPKR_DRV_CMFB_SET
    | SPKR_DRV_GAIN_SET;
const CDC_A_SPKR_ANA_BIAS_SET: u32 = 0xf1B3;
const CDC_A_SPKR_OCP_CTL: u32 = 0xf1B4;
const CDC_A_SPKR_PWRSTG_CTL: u32 = 0xf1B5;
const SPKR_PWRSTG_CTL_DAC_EN_MASK: u32 = BIT(0);
const SPKR_PWRSTG_CTL_DAC_EN: u32 = BIT(0);
const SPKR_PWRSTG_CTL_MASK: u32 = 0xE0;
const SPKR_PWRSTG_CTL_BBM_MASK: u32 = BIT(7);
const SPKR_PWRSTG_CTL_BBM_EN: u32 = BIT(7);
const SPKR_PWRSTG_CTL_HBRDGE_EN_MASK: u32 = BIT(6);
const SPKR_PWRSTG_CTL_HBRDGE_EN: u32 = BIT(6);
const SPKR_PWRSTG_CTL_CLAMP_EN_MASK: u32 = BIT(5);
const SPKR_PWRSTG_CTL_CLAMP_EN: u32 = BIT(5);
const CDC_A_SPKR_DRV_DBG: u32 = 0xf1B7;
const CDC_A_CURRENT_LIMIT: u32 = 0xf1C0;
const CDC_A_BYPASS_MODE: u32 = 0xf1C2;
const CDC_A_BOOST_EN_CTL: u32 = 0xf1C3;
const CDC_A_SLOPE_COMP_IP_ZERO: u32 = 0xf1C4;
const CDC_A_SEC_ACCESS: u32 = 0xf1D0;
const CDC_A_PERPH_RESET_CTL3: u32 = 0xf1DA;
const CDC_A_PERPH_RESET_CTL4: u32 = 0xf1DB;

const MSM8916_WCD_ANALOG_RATES: u32 =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;
const MSM8916_WCD_ANALOG_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut btn_mask: c_int =
    SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4;
static mut hs_jack_mask: c_int = SND_JACK_HEADPHONE | SND_JACK_HEADSET;

static supply_names: [*const c_char; 2] = [c"vdd-cdc-io".as_ptr(), c"vdd-cdc-tx-rx-cx".as_ptr()];

const MBHC_MAX_BUTTONS: usize = 5;

#[repr(C)]
struct wcd_reg_seq {
    seq: *const reg_default,
    seq_size: c_int,
}

#[repr(C)]
struct pm8916_wcd_analog_priv {
    pmic_rev: u16_,
    codec_version: u16_,
    mbhc_btn_enabled: bool_,
    /* special event to detect accessory type */
    mbhc_btn0_released: c_int,
    detect_accessory_type: bool_,
    mclk: *mut clk,
    component: *mut snd_soc_component,
    supplies: [regulator_bulk_data; 2],
    jack: *mut snd_soc_jack,
    hphl_jack_type_normally_open: bool_,
    gnd_jack_type_normally_open: bool_,
    /* Voltage threshold when internal current source of 100uA is used */
    vref_btn_cs: [u32_; MBHC_MAX_BUTTONS],
    /* Voltage threshold when microphone bias is ON */
    vref_btn_micb: [u32_; MBHC_MAX_BUTTONS],
    micbias1_cap_mode: c_uint,
    micbias2_cap_mode: c_uint,
    micbias_mv: c_uint,
}

static adc2_mux_text: [*const c_char; 3] = [c"ZERO".as_ptr(), c"INP2".as_ptr(), c"INP3".as_ptr()];
static rdac2_mux_text: [*const c_char; 2] = [c"RX1".as_ptr(), c"RX2".as_ptr()];
static hph_text: [*const c_char; 2] = [c"ZERO".as_ptr(), c"Switch".as_ptr()];

static hph_enum: soc_enum = unsafe { SOC_ENUM_SINGLE_VIRT(ARRAY_SIZE(&hph_text), hph_text.as_ptr()) };

static ear_mux: snd_kcontrol_new = unsafe { SOC_DAPM_ENUM(c"EAR_S".as_ptr(), &hph_enum) };
static hphl_mux: snd_kcontrol_new = unsafe { SOC_DAPM_ENUM(c"HPHL".as_ptr(), &hph_enum) };
static hphr_mux: snd_kcontrol_new = unsafe { SOC_DAPM_ENUM(c"HPHR".as_ptr(), &hph_enum) };

/* ADC2 MUX */
static adc2_enum: soc_enum = unsafe { SOC_ENUM_SINGLE_VIRT(ARRAY_SIZE(&adc2_mux_text), adc2_mux_text.as_ptr()) };

/* RDAC2 MUX */
static rdac2_mux_enum: soc_enum =
    unsafe { SOC_ENUM_SINGLE(CDC_D_CDC_CONN_HPHR_DAC_CTL, 0, 2, rdac2_mux_text.as_ptr()) };

static spkr_switch: [snd_kcontrol_new; 1] =
    [unsafe { SOC_DAPM_SINGLE(c"Switch".as_ptr(), CDC_A_SPKR_DAC_CTL, 7, 1, 0) }];

static rdac2_mux: snd_kcontrol_new =
    unsafe { SOC_DAPM_ENUM(c"RDAC2 MUX Mux".as_ptr(), &rdac2_mux_enum) };
static tx_adc2_mux: snd_kcontrol_new =
    unsafe { SOC_DAPM_ENUM(c"ADC2 MUX Mux".as_ptr(), &adc2_enum) };

/* Analog Gain control 0 dB to +24 dB in 6 dB steps */
static analog_gain: [c_uint; 4] = DECLARE_TLV_DB_SCALE(0, 600, 0);

static pm8916_wcd_analog_snd_controls: [snd_kcontrol_new; 3] = [
    unsafe { SOC_SINGLE_TLV(c"ADC1 Volume".as_ptr(), CDC_A_TX_1_EN, 3, 8, 0, analog_gain.as_ptr()) },
    unsafe { SOC_SINGLE_TLV(c"ADC2 Volume".as_ptr(), CDC_A_TX_2_EN, 3, 8, 0, analog_gain.as_ptr()) },
    unsafe { SOC_SINGLE_TLV(c"ADC3 Volume".as_ptr(), CDC_A_TX_3_EN, 3, 8, 0, analog_gain.as_ptr()) },
];

unsafe extern "C" fn pm8916_wcd_analog_micbias_enable(component: *mut snd_soc_component) {
    let wcd = snd_soc_component_get_drvdata(component) as *mut pm8916_wcd_analog_priv;

    snd_soc_component_update_bits(
        component,
        CDC_A_MICB_1_CTL,
        MICB_1_CTL_EXT_PRECHARG_EN_MASK | MICB_1_CTL_INT_PRECHARG_BYP_MASK,
        MICB_1_CTL_INT_PRECHARG_BYP_EXT_PRECHRG_SEL | MICB_1_CTL_EXT_PRECHARG_EN_ENABLE,
    );

    if (*wcd).micbias_mv != 0 {
        snd_soc_component_update_bits(
            component,
            CDC_A_MICB_1_VAL,
            MICB_1_VAL_MICB_OUT_VAL_MASK,
            MICB_VOLTAGE_REGVAL((*wcd).micbias_mv),
        );
        /*
         * Special headset needs MICBIAS as 2.7V so wait for
         * 50 msec for the MICBIAS to reach 2.7 volts.
         */
        if (*wcd).micbias_mv >= 2700 {
            msleep(50);
        }
    }

    snd_soc_component_update_bits(
        component,
        CDC_A_MICB_1_CTL,
        MICB_1_CTL_EXT_PRECHARG_EN_MASK | MICB_1_CTL_INT_PRECHARG_BYP_MASK,
        0,
    );
}

unsafe extern "C" fn pm8916_wcd_analog_enable_micbias(
    component: *mut snd_soc_component,
    event: c_int,
    cap_mode: c_uint,
) -> c_int {
    match event as u32 {
        SND_SOC_DAPM_POST_PMU => {
            pm8916_wcd_analog_micbias_enable(component);
            snd_soc_component_update_bits(component, CDC_A_MICB_1_EN, MICB_1_EN_BYP_CAP_MASK, cap_mode);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn pm8916_wcd_analog_enable_micbias_int(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event as u32 {
        SND_SOC_DAPM_PRE_PMU => snd_soc_component_update_bits(
            component,
            CDC_A_MICB_1_EN,
            MICB_1_EN_OPA_STG2_TAIL_CURR_MASK,
            MICB_1_EN_OPA_STG2_TAIL_CURR_1_60UA,
        ),
        _ => {}
    }
    0
}

unsafe extern "C" fn pm8916_wcd_analog_enable_micbias1(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wcd = snd_soc_component_get_drvdata(component) as *mut pm8916_wcd_analog_priv;
    pm8916_wcd_analog_enable_micbias(component, event, (*wcd).micbias1_cap_mode)
}

unsafe extern "C" fn pm8916_wcd_analog_enable_micbias2(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wcd = snd_soc_component_get_drvdata(component) as *mut pm8916_wcd_analog_priv;
    pm8916_wcd_analog_enable_micbias(component, event, (*wcd).micbias2_cap_mode)
}

unsafe extern "C" fn pm8916_mbhc_configure_bias(
    priv_: *mut pm8916_wcd_analog_priv,
    micbias2_enabled: bool_,
) -> c_int {
    let component = (*priv_).component;
    let mut coarse: u32_;
    let mut fine: u32_;
    let mut reg_val: u32_;
    let mut reg_addr: u32_;
    let vrefs: *mut u32_;

    if !micbias2_enabled {
        /* use internal 100uA Current source */
        /* Enable internal 2.2k Internal Rbias Resistor */
        snd_soc_component_update_bits(
            component,
            CDC_A_MICB_1_INT_RBIAS,
            MICB_1_INT_TX2_INT_RBIAS_EN_MASK,
            MICB_1_INT_TX2_INT_RBIAS_EN_ENABLE,
        );
        /* Remove pull down on MIC BIAS2 */
        snd_soc_component_update_bits(component, CDC_A_MICB_2_EN, CDC_A_MICB_2_PULL_DOWN_EN_MASK, 0);
        /* enable 100uA internal current source */
        snd_soc_component_update_bits(
            component,
            CDC_A_MBHC_FSM_CTL,
            CDC_A_MBHC_FSM_CTL_BTN_ISRC_CTRL_MASK,
            CDC_A_MBHC_FSM_CTL_BTN_ISRC_CTRL_I_100UA,
        );
    }
    snd_soc_component_update_bits(
        component,
        CDC_A_MBHC_FSM_CTL,
        CDC_A_MBHC_FSM_CTL_MBHC_FSM_EN_MASK,
        CDC_A_MBHC_FSM_CTL_MBHC_FSM_EN,
    );

    if micbias2_enabled {
        vrefs = (*priv_).vref_btn_micb.as_mut_ptr();
    } else {
        vrefs = (*priv_).vref_btn_cs.as_mut_ptr();
    }

    /* program vref ranges for all the buttons */
    reg_addr = CDC_A_MBHC_BTN0_ZDET_CTL_0;
    for i in 0..MBHC_MAX_BUTTONS {
        /* split mv in to coarse parts of 100mv & fine parts of 12mv */
        coarse = *vrefs.add(i) / 100;
        fine = (*vrefs.add(i) % 100) / 12;
        reg_val = (coarse << CDC_A_MBHC_BTN_VREF_COARSE_SHIFT) | (fine << CDC_A_MBHC_BTN_VREF_FINE_SHIFT);
        snd_soc_component_update_bits(component, reg_addr, CDC_A_MBHC_BTN_VREF_MASK, reg_val);
        reg_addr = reg_addr.wrapping_add(1);
    }

    0
}

unsafe extern "C" fn pm8916_wcd_setup_mbhc(wcd: *mut pm8916_wcd_analog_priv) {
    let component = (*wcd).component;
    let mut micbias_enabled = false;
    let mut plug_type: u32_ = 0;
    let mut int_en_mask: u32_;

    snd_soc_component_write(
        component,
        CDC_A_MBHC_DET_CTL_1,
        CDC_A_MBHC_DET_CTL_L_DET_EN
            | CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_INSERTION
            | CDC_A_MBHC_DET_CTL_MIC_CLAMP_CTL_AUTO
            | CDC_A_MBHC_DET_CTL_MBHC_BIAS_EN,
    );

    if (*wcd).hphl_jack_type_normally_open {
        plug_type |= CDC_A_HPHL_PLUG_TYPE_NO;
    }
    if (*wcd).gnd_jack_type_normally_open {
        plug_type |= CDC_A_GND_PLUG_TYPE_NO;
    }

    snd_soc_component_write(
        component,
        CDC_A_MBHC_DET_CTL_2,
        CDC_A_MBHC_DET_CTL_HS_L_DET_PULL_UP_CTRL_I_3P0
            | CDC_A_MBHC_DET_CTL_HS_L_DET_COMPA_CTRL_V0P9_VDD
            | plug_type
            | CDC_A_MBHC_DET_CTL_HPHL_100K_TO_GND_EN,
    );

    snd_soc_component_write(
        component,
        CDC_A_MBHC_DBNC_TIMER,
        CDC_A_MBHC_DBNC_TIMER_INSREM_DBNC_T_256_MS | CDC_A_MBHC_DBNC_TIMER_BTN_DBNC_T_16MS,
    );

    /* enable MBHC clock */
    snd_soc_component_update_bits(
        component,
        CDC_D_CDC_DIG_CLK_CTL,
        DIG_CLK_CTL_D_MBHC_CLK_EN_MASK,
        DIG_CLK_CTL_D_MBHC_CLK_EN,
    );

    if snd_soc_component_read(component, CDC_A_MICB_2_EN) & CDC_A_MICB_2_EN_ENABLE != 0 {
        micbias_enabled = true;
    }

    pm8916_mbhc_configure_bias(wcd, micbias_enabled);

    int_en_mask = MBHC_SWITCH_INT;
    if (*wcd).mbhc_btn_enabled {
        int_en_mask |= MBHC_BUTTON_PRESS_DET | MBHC_BUTTON_RELEASE_DET;
    }

    snd_soc_component_update_bits(component, CDC_D_INT_EN_CLR, int_en_mask, 0);
    snd_soc_component_update_bits(component, CDC_D_INT_EN_SET, int_en_mask, int_en_mask);
    (*wcd).mbhc_btn0_released = 0;
    (*wcd).detect_accessory_type = true;
}

unsafe extern "C" fn pm8916_wcd_analog_enable_micbias_int2(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wcd = snd_soc_component_get_drvdata(component) as *mut pm8916_wcd_analog_priv;

    match event as u32 {
        SND_SOC_DAPM_PRE_PMU => snd_soc_component_update_bits(component, CDC_A_MICB_2_EN, CDC_A_MICB_2_PULL_DOWN_EN_MASK, 0),
        SND_SOC_DAPM_POST_PMU => {
            pm8916_mbhc_configure_bias(wcd, true);
        }
        SND_SOC_DAPM_POST_PMD => {
            pm8916_mbhc_configure_bias(wcd, false);
        }
        _ => {}
    }

    pm8916_wcd_analog_enable_micbias_int(w, kcontrol, event)
}

unsafe extern "C" fn pm8916_wcd_analog_enable_adc(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let adc_reg: u16_ = CDC_A_TX_1_2_TEST_CTL_2 as u16_;
    let init_bit_shift: u8_ = if (*w).reg == CDC_A_TX_1_EN as c_int { 5 } else { 4 };

    match event as u32 {
        SND_SOC_DAPM_PRE_PMU => {
            if (*w).reg == CDC_A_TX_2_EN as c_int {
                snd_soc_component_update_bits(
                    component,
                    CDC_A_MICB_1_CTL,
                    MICB_1_CTL_CFILT_REF_SEL_MASK,
                    MICB_1_CTL_CFILT_REF_SEL_HPF_REF,
                );
            }
            /*
             * Add delay of 10 ms to give sufficient time for the voltage
             * to shoot up and settle so that the txfe init does not
             * happen when the input voltage is changing too much.
             */
            usleep_range(10000, 10010);
            snd_soc_component_update_bits(component, adc_reg as u32, 1 << init_bit_shift, 1 << init_bit_shift);
            match (*w).reg as u32 {
                CDC_A_TX_1_EN => snd_soc_component_update_bits(
                    component,
                    CDC_D_CDC_CONN_TX1_CTL,
                    CONN_TX1_SERIAL_TX1_MUX,
                    CONN_TX1_SERIAL_TX1_ADC_1,
                ),
                CDC_A_TX_2_EN | CDC_A_TX_3_EN => snd_soc_component_update_bits(
                    component,
                    CDC_D_CDC_CONN_TX2_CTL,
                    CONN_TX2_SERIAL_TX2_MUX,
                    CONN_TX2_SERIAL_TX2_ADC_2,
                ),
                _ => {}
            }
        }
        SND_SOC_DAPM_POST_PMU => {
            /*
             * Add delay of 12 ms before deasserting the init
             * to reduce the tx pop
             */
            usleep_range(12000, 12010);
            snd_soc_component_update_bits(component, adc_reg as u32, 1 << init_bit_shift, 0x00);
        }
        SND_SOC_DAPM_POST_PMD => match (*w).reg as u32 {
            CDC_A_TX_1_EN => snd_soc_component_update_bits(
                component,
                CDC_D_CDC_CONN_TX1_CTL,
                CONN_TX1_SERIAL_TX1_MUX,
                CONN_TX1_SERIAL_TX1_ZERO,
            ),
            CDC_A_TX_2_EN => {
                snd_soc_component_update_bits(component, CDC_A_MICB_1_CTL, MICB_1_CTL_CFILT_REF_SEL_MASK, 0);
                snd_soc_component_update_bits(
                    component,
                    CDC_D_CDC_CONN_TX2_CTL,
                    CONN_TX2_SERIAL_TX2_MUX,
                    CONN_TX2_SERIAL_TX2_ZERO,
                );
            }
            CDC_A_TX_3_EN => snd_soc_component_update_bits(
                component,
                CDC_D_CDC_CONN_TX2_CTL,
                CONN_TX2_SERIAL_TX2_MUX,
                CONN_TX2_SERIAL_TX2_ZERO,
            ),
            _ => {}
        },
        _ => {}
    }
    0
}

unsafe extern "C" fn pm8916_wcd_analog_enable_spk_pa(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event as u32 {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(
                component,
                CDC_A_SPKR_PWRSTG_CTL,
                SPKR_PWRSTG_CTL_DAC_EN_MASK
                    | SPKR_PWRSTG_CTL_BBM_MASK
                    | SPKR_PWRSTG_CTL_HBRDGE_EN_MASK
                    | SPKR_PWRSTG_CTL_CLAMP_EN_MASK,
                SPKR_PWRSTG_CTL_DAC_EN
                    | SPKR_PWRSTG_CTL_BBM_EN
                    | SPKR_PWRSTG_CTL_HBRDGE_EN
                    | SPKR_PWRSTG_CTL_CLAMP_EN,
            );
            snd_soc_component_update_bits(
                component,
                CDC_A_RX_EAR_CTL,
                RX_EAR_CTL_SPK_VBAT_LDO_EN_MASK,
                RX_EAR_CTL_SPK_VBAT_LDO_EN_ENABLE,
            );
        }
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_update_bits(component, CDC_A_SPKR_DRV_CTL, SPKR_DRV_CTL_DEF_MASK, SPKR_DRV_CTL_DEF_VAL);
            snd_soc_component_update_bits(
                component,
                (*w).reg as u32,
                SPKR_DRV_CLASSD_PA_EN_MASK,
                SPKR_DRV_CLASSD_PA_EN_ENABLE,
            );
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(
                component,
                CDC_A_SPKR_PWRSTG_CTL,
                SPKR_PWRSTG_CTL_DAC_EN_MASK
                    | SPKR_PWRSTG_CTL_BBM_MASK
                    | SPKR_PWRSTG_CTL_HBRDGE_EN_MASK
                    | SPKR_PWRSTG_CTL_CLAMP_EN_MASK,
                0,
            );
            snd_soc_component_update_bits(
                component,
                CDC_A_SPKR_DAC_CTL,
                SPKR_DAC_CTL_DAC_RESET_MASK,
                SPKR_DAC_CTL_DAC_RESET_NORMAL,
            );
            snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL, RX_EAR_CTL_SPK_VBAT_LDO_EN_MASK, 0);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn pm8916_wcd_analog_enable_ear_pa(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event as u32 {
        SND_SOC_DAPM_PRE_PMU => snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL, RX_EAR_CTL_PA_SEL_MASK, RX_EAR_CTL_PA_SEL),
        SND_SOC_DAPM_POST_PMU => snd_soc_component_update_bits(
            component,
            CDC_A_RX_EAR_CTL,
            RX_EAR_CTL_PA_EAR_PA_EN_MASK,
            RX_EAR_CTL_PA_EAR_PA_EN_ENABLE,
        ),
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL, RX_EAR_CTL_PA_EAR_PA_EN_MASK, 0);
            /* Delay to reduce ear turn off pop */
            usleep_range(7000, 7100);
            snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL, RX_EAR_CTL_PA_SEL_MASK, 0);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn pm8916_wcd_analog_enable_hphl_pa(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = dev_get_drvdata((*component).dev) as *mut pm8916_wcd_analog_priv;

    /* This quirk is not required for revisions prior to CAJON_2_0 */
    if (*priv_).codec_version < 4 {
        return 0;
    }

    match event as u32 {
        SND_SOC_DAPM_POST_PMU => {
            usleep_range(7000, 7100);
            snd_soc_component_update_bits(component, CDC_A_RX_HPH_L_TEST, 0x04, 0x04);
        }
        SND_SOC_DAPM_POST_PMD => {
            /* wait 20 ms after the digital codec has powered down */
            msleep(20);
            snd_soc_component_update_bits(component, CDC_A_RX_HPH_L_TEST, 0x04, 0x00);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn pm8916_wcd_analog_enable_hphr_pa(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = dev_get_drvdata((*component).dev) as *mut pm8916_wcd_analog_priv;

    /* This quirk is not required for revisions prior to CAJON_2_0 */
    if (*priv_).codec_version < 4 {
        return 0;
    }

    match event as u32 {
        SND_SOC_DAPM_POST_PMU => {
            usleep_range(7000, 7100);
            snd_soc_component_update_bits(component, CDC_A_RX_HPH_R_TEST, 0x04, 0x04);
        }
        SND_SOC_DAPM_POST_PMD => {
            msleep(20);
            snd_soc_component_update_bits(component, CDC_A_RX_HPH_R_TEST, 0x04, 0x00);
        }
        _ => {}
    }
    0
}

static wcd_reg_defaults_2_0: [reg_default; 16] = [
    reg_default { reg: CDC_A_RX_COM_OCP_CTL, def: 0xD1 },
    reg_default { reg: CDC_A_RX_COM_OCP_COUNT, def: 0xFF },
    reg_default { reg: CDC_D_SEC_ACCESS, def: 0xA5 },
    reg_default { reg: CDC_D_PERPH_RESET_CTL3, def: 0x0F },
    reg_default { reg: CDC_A_TX_1_2_OPAMP_BIAS, def: 0x4F },
    reg_default { reg: CDC_A_NCP_FBCTRL, def: 0x28 },
    reg_default { reg: CDC_A_SPKR_DRV_CTL, def: 0x69 },
    reg_default { reg: CDC_A_SPKR_DRV_DBG, def: 0x01 },
    reg_default { reg: CDC_A_BOOST_EN_CTL, def: 0x5F },
    reg_default { reg: CDC_A_SLOPE_COMP_IP_ZERO, def: 0x88 },
    reg_default { reg: CDC_A_SEC_ACCESS, def: 0xA5 },
    reg_default { reg: CDC_A_PERPH_RESET_CTL3, def: 0x0F },
    reg_default { reg: CDC_A_CURRENT_LIMIT, def: 0x82 },
    reg_default { reg: CDC_A_SPKR_DAC_CTL, def: 0x03 },
    reg_default { reg: CDC_A_SPKR_OCP_CTL, def: 0xE1 },
    reg_default { reg: CDC_A_MASTER_BIAS_CTL, def: 0x30 },
];

static pm8916_data: wcd_reg_seq =
    wcd_reg_seq { seq: wcd_reg_defaults_2_0.as_ptr(), seq_size: 16 };

static wcd_reg_defaults_pm8950: [reg_default; 18] = [
    reg_default { reg: CDC_A_RX_COM_OCP_CTL, def: 0xd1 },
    reg_default { reg: CDC_A_RX_COM_OCP_COUNT, def: 0xff },
    reg_default { reg: CDC_D_SEC_ACCESS, def: 0xa5 },
    reg_default { reg: CDC_D_PERPH_RESET_CTL3, def: 0x0f },
    reg_default { reg: CDC_A_TX_1_2_OPAMP_BIAS, def: 0x4c },
    reg_default { reg: CDC_A_NCP_FBCTRL, def: 0xa8 },
    reg_default { reg: CDC_A_NCP_VCTRL, def: 0xa4 },
    reg_default { reg: CDC_A_SPKR_DRV_CTL, def: 0x69 },
    reg_default { reg: CDC_A_SPKR_DRV_DBG, def: 0x01 },
    reg_default { reg: CDC_A_SEC_ACCESS, def: 0xa5 },
    reg_default { reg: CDC_A_PERPH_RESET_CTL3, def: 0x0f },
    reg_default { reg: CDC_A_CURRENT_LIMIT, def: 0x82 },
    reg_default { reg: CDC_A_SPKR_ANA_BIAS_SET, def: 0x41 },
    reg_default { reg: CDC_A_SPKR_DAC_CTL, def: 0x03 },
    reg_default { reg: CDC_A_SPKR_OCP_CTL, def: 0xe1 },
    reg_default { reg: CDC_A_RX_HPH_BIAS_PA, def: 0xfa },
    reg_default { reg: CDC_A_MASTER_BIAS_CTL, def: 0x30 },
    reg_default { reg: CDC_A_MICB_1_INT_RBIAS, def: 0x00 },
];

static pm8950_data: wcd_reg_seq =
    wcd_reg_seq { seq: wcd_reg_defaults_pm8950.as_ptr(), seq_size: 18 };

static wcd_reg_defaults_pm8953: [reg_default; 20] = [
    reg_default { reg: CDC_A_RX_COM_OCP_CTL, def: 0xd1 },
    reg_default { reg: CDC_A_RX_COM_OCP_COUNT, def: 0xff },
    reg_default { reg: CDC_D_SEC_ACCESS, def: 0xa5 },
    reg_default { reg: CDC_D_PERPH_RESET_CTL3, def: 0x0f },
    reg_default { reg: CDC_A_TX_1_2_OPAMP_BIAS, def: 0x4c },
    reg_default { reg: CDC_A_NCP_FBCTRL, def: 0xa8 },
    reg_default { reg: CDC_A_NCP_VCTRL, def: 0xa4 },
    reg_default { reg: CDC_A_SPKR_DRV_CTL, def: 0x69 },
    reg_default { reg: CDC_A_SPKR_DRV_DBG, def: 0x01 },
    reg_default { reg: CDC_A_SEC_ACCESS, def: 0xa5 },
    reg_default { reg: CDC_A_PERPH_RESET_CTL3, def: 0x0f },
    reg_default { reg: CDC_A_CURRENT_LIMIT, def: 0xa2 },
    reg_default { reg: CDC_A_BYPASS_MODE, def: 0x18 },
    reg_default { reg: CDC_A_SPKR_ANA_BIAS_SET, def: 0x41 },
    reg_default { reg: CDC_A_SPKR_DAC_CTL, def: 0x03 },
    reg_default { reg: CDC_A_SPKR_OCP_CTL, def: 0xe1 },
    reg_default { reg: CDC_A_RX_HPH_BIAS_PA, def: 0xfa },
    reg_default { reg: CDC_A_RX_EAR_STATUS, def: 0x10 },
    reg_default { reg: CDC_A_MASTER_BIAS_CTL, def: 0x30 },
    reg_default { reg: CDC_A_MICB_1_INT_RBIAS, def: 0x00 },
];

static pm8953_data: wcd_reg_seq =
    wcd_reg_seq { seq: wcd_reg_defaults_pm8953.as_ptr(), seq_size: 20 };

unsafe extern "C" fn pm8916_wcd_analog_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = dev_get_drvdata((*component).dev) as *mut pm8916_wcd_analog_priv;
    let mut err: c_int;

    err = regulator_bulk_enable(ARRAY_SIZE(&(*priv_).supplies) as c_int, (*priv_).supplies.as_mut_ptr());
    if err != 0 {
        dev_err((*component).dev, c"failed to enable regulators (%d)\n".as_ptr(), err);
        return err;
    }

    snd_soc_component_init_regmap(component, dev_get_regmap((*(*component).dev).parent, ptr::null()));
    snd_soc_component_set_drvdata(component, priv_ as *mut c_void);
    (*priv_).pmic_rev = snd_soc_component_read(component, CDC_D_REVISION1) as u16_;
    (*priv_).codec_version = snd_soc_component_read(component, CDC_D_PERPH_SUBTYPE) as u16_;

    dev_info(
        (*component).dev,
        c"PMIC REV: %d\t CODEC Version: %d\n".as_ptr(),
        (*priv_).pmic_rev as c_int,
        (*priv_).codec_version as c_int,
    );

    snd_soc_component_write(component, CDC_D_PERPH_RESET_CTL4, 0x01);
    snd_soc_component_write(component, CDC_A_PERPH_RESET_CTL4, 0x01);

    let wcd_reg_init_data = of_device_get_match_data((*component).dev) as *const wcd_reg_seq;

    for reg in 0..(*wcd_reg_init_data).seq_size {
        let entry = (*wcd_reg_init_data).seq.add(reg as usize);
        snd_soc_component_write(component, (*entry).reg, (*entry).def);
    }

    (*priv_).component = component;

    snd_soc_component_update_bits(
        component,
        CDC_D_CDC_RST_CTL,
        RST_CTL_DIG_SW_RST_N_MASK,
        RST_CTL_DIG_SW_RST_N_REMOVE_RESET,
    );

    pm8916_wcd_setup_mbhc(priv_);
    0
}

unsafe extern "C" fn pm8916_wcd_analog_remove(component: *mut snd_soc_component) {
    let priv_ = dev_get_drvdata((*component).dev) as *mut pm8916_wcd_analog_priv;

    snd_soc_component_update_bits(component, CDC_D_CDC_RST_CTL, RST_CTL_DIG_SW_RST_N_MASK, 0);
    regulator_bulk_disable(ARRAY_SIZE(&(*priv_).supplies) as c_int, (*priv_).supplies.as_mut_ptr());
}

static pm8916_wcd_analog_audio_map: [snd_soc_dapm_route; 65] = [
    route(c"PDM_RX1", ptr::null(), c"PDM Playback"),
    route(c"PDM_RX2", ptr::null(), c"PDM Playback"),
    route(c"PDM_RX3", ptr::null(), c"PDM Playback"),
    route(c"PDM Capture", ptr::null(), c"PDM_TX"),
    /* ADC Connections */
    route(c"PDM_TX", ptr::null(), c"ADC2"),
    route(c"PDM_TX", ptr::null(), c"ADC3"),
    route(c"ADC2", ptr::null(), c"ADC2 MUX"),
    route(c"ADC3", ptr::null(), c"ADC2 MUX"),
    route(c"ADC2 MUX", c"INP2", c"ADC2_INP2"),
    route(c"ADC2 MUX", c"INP3", c"ADC2_INP3"),
    route(c"PDM_TX", ptr::null(), c"ADC1"),
    route(c"ADC1", ptr::null(), c"AMIC1"),
    route(c"ADC2_INP2", ptr::null(), c"AMIC2"),
    route(c"ADC2_INP3", ptr::null(), c"AMIC3"),
    /* RDAC Connections */
    route(c"HPHR DAC", ptr::null(), c"RDAC2 MUX"),
    route(c"RDAC2 MUX", c"RX1", c"PDM_RX1"),
    route(c"RDAC2 MUX", c"RX2", c"PDM_RX2"),
    route(c"HPHL DAC", ptr::null(), c"PDM_RX1"),
    route(c"PDM_RX1", ptr::null(), c"RXD1_CLK"),
    route(c"PDM_RX2", ptr::null(), c"RXD2_CLK"),
    route(c"PDM_RX3", ptr::null(), c"RXD3_CLK"),
    route(c"PDM_RX1", ptr::null(), c"RXD_PDM_CLK"),
    route(c"PDM_RX2", ptr::null(), c"RXD_PDM_CLK"),
    route(c"PDM_RX3", ptr::null(), c"RXD_PDM_CLK"),
    route(c"ADC1", ptr::null(), c"TXD_CLK"),
    route(c"ADC2", ptr::null(), c"TXD_CLK"),
    route(c"ADC3", ptr::null(), c"TXD_CLK"),
    route(c"ADC1", ptr::null(), c"TXA_CLK25"),
    route(c"ADC2", ptr::null(), c"TXA_CLK25"),
    route(c"ADC3", ptr::null(), c"TXA_CLK25"),
    route(c"PDM_RX1", ptr::null(), c"A_MCLK2"),
    route(c"PDM_RX2", ptr::null(), c"A_MCLK2"),
    route(c"PDM_RX3", ptr::null(), c"A_MCLK2"),
    route(c"PDM_TX", ptr::null(), c"A_MCLK2"),
    route(c"A_MCLK2", ptr::null(), c"A_MCLK"),
    /* Earpiece (RX MIX1) */
    route(c"EAR", ptr::null(), c"EAR_S"),
    route(c"EAR_S", c"Switch", c"EAR PA"),
    route(c"EAR PA", ptr::null(), c"RX_BIAS"),
    route(c"EAR PA", ptr::null(), c"HPHL DAC"),
    route(c"EAR PA", ptr::null(), c"HPHR DAC"),
    route(c"EAR PA", ptr::null(), c"EAR CP"),
    /* Headset (RX MIX1 and RX MIX2) */
    route(c"HPH_L", ptr::null(), c"HPHL PA"),
    route(c"HPH_R", ptr::null(), c"HPHR PA"),
    route(c"HPHL DAC", ptr::null(), c"EAR_HPHL_CLK"),
    route(c"HPHR DAC", ptr::null(), c"EAR_HPHR_CLK"),
    route(c"CP", ptr::null(), c"NCP_CLK"),
    route(c"HPHL PA", ptr::null(), c"HPHL"),
    route(c"HPHR PA", ptr::null(), c"HPHR"),
    route(c"HPHL PA", ptr::null(), c"CP"),
    route(c"HPHL PA", ptr::null(), c"RX_BIAS"),
    route(c"HPHR PA", ptr::null(), c"CP"),
    route(c"HPHR PA", ptr::null(), c"RX_BIAS"),
    route(c"HPHL", c"Switch", c"HPHL DAC"),
    route(c"HPHR", c"Switch", c"HPHR DAC"),
    route(c"RX_BIAS", ptr::null(), c"DAC_REF"),
    route(c"SPK_OUT", ptr::null(), c"SPK PA"),
    route(c"SPK PA", ptr::null(), c"RX_BIAS"),
    route(c"SPK PA", ptr::null(), c"SPKR_CLK"),
    route(c"SPK PA", ptr::null(), c"SPK DAC"),
    route(c"SPK DAC", c"Switch", c"PDM_RX3"),
    route(c"MIC_BIAS1", ptr::null(), c"INT_LDO_H"),
    route(c"MIC_BIAS2", ptr::null(), c"INT_LDO_H"),
    route(c"MIC_BIAS1", ptr::null(), c"vdd-micbias"),
    route(c"MIC_BIAS2", ptr::null(), c"vdd-micbias"),
    route(c"MIC BIAS External1", ptr::null(), c"MIC_BIAS1"),
    route(c"MIC BIAS Internal1", ptr::null(), c"MIC_BIAS1"),
    route(c"MIC BIAS External2", ptr::null(), c"MIC_BIAS2"),
    route(c"MIC BIAS Internal2", ptr::null(), c"MIC_BIAS2"),
    route(c"MIC BIAS Internal3", ptr::null(), c"MIC_BIAS1"),
];

// The C source initializes these with ASoC DAPM macros. Keep the macro-expanded
// construction delegated to external Rust equivalents.
static pm8916_wcd_analog_dapm_widgets: [snd_soc_dapm_widget; 51] = unsafe {
    [
        SND_SOC_DAPM_AIF_IN(c"PDM_RX1".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN(c"PDM_RX2".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN(c"PDM_RX3".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_OUT(c"PDM_TX".as_ptr(), ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_INPUT(c"AMIC1".as_ptr()),
        SND_SOC_DAPM_INPUT(c"AMIC3".as_ptr()),
        SND_SOC_DAPM_INPUT(c"AMIC2".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"EAR".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"HPH_L".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"HPH_R".as_ptr()),
        /* RX stuff */
        SND_SOC_DAPM_SUPPLY(c"INT_LDO_H".as_ptr(), SND_SOC_NOPM, 1, 0, None, 0),
        SND_SOC_DAPM_PGA_E(c"EAR PA".as_ptr(), SND_SOC_NOPM, 0, 0, ptr::null(), 0, Some(pm8916_wcd_analog_enable_ear_pa), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_MUX(c"EAR_S".as_ptr(), SND_SOC_NOPM, 0, 0, &ear_mux),
        SND_SOC_DAPM_SUPPLY(c"EAR CP".as_ptr(), CDC_A_NCP_EN as c_int, 4, 0, None, 0),
        SND_SOC_DAPM_PGA_E(c"HPHL PA".as_ptr(), CDC_A_RX_HPH_CNP_EN as c_int, 5, 0, ptr::null(), 0, Some(pm8916_wcd_analog_enable_hphl_pa), SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_MUX(c"HPHL".as_ptr(), SND_SOC_NOPM, 0, 0, &hphl_mux),
        SND_SOC_DAPM_MIXER(c"HPHL DAC".as_ptr(), CDC_A_RX_HPH_L_PA_DAC_CTL as c_int, 3, 0, ptr::null(), 0),
        SND_SOC_DAPM_PGA_E(c"HPHR PA".as_ptr(), CDC_A_RX_HPH_CNP_EN as c_int, 4, 0, ptr::null(), 0, Some(pm8916_wcd_analog_enable_hphr_pa), SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_MUX(c"HPHR".as_ptr(), SND_SOC_NOPM, 0, 0, &hphr_mux),
        SND_SOC_DAPM_MIXER(c"HPHR DAC".as_ptr(), CDC_A_RX_HPH_R_PA_DAC_CTL as c_int, 3, 0, ptr::null(), 0),
        SND_SOC_DAPM_MIXER(c"SPK DAC".as_ptr(), SND_SOC_NOPM, 0, 0, spkr_switch.as_ptr(), ARRAY_SIZE(&spkr_switch) as c_int),
        /* Speaker */
        SND_SOC_DAPM_OUTPUT(c"SPK_OUT".as_ptr()),
        SND_SOC_DAPM_PGA_E(c"SPK PA".as_ptr(), CDC_A_SPKR_DRV_CTL as c_int, 6, 0, ptr::null(), 0, Some(pm8916_wcd_analog_enable_spk_pa), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_REGULATOR_SUPPLY(c"vdd-micbias".as_ptr(), 0, 0),
        SND_SOC_DAPM_SUPPLY(c"CP".as_ptr(), CDC_A_NCP_EN as c_int, 0, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"DAC_REF".as_ptr(), CDC_A_RX_COM_BIAS_DAC as c_int, 0, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"RX_BIAS".as_ptr(), CDC_A_RX_COM_BIAS_DAC as c_int, 7, 0, None, 0),
        /* TX */
        SND_SOC_DAPM_SUPPLY(c"MIC_BIAS1".as_ptr(), CDC_A_MICB_1_EN as c_int, 7, 0, Some(pm8916_wcd_analog_enable_micbias1), SND_SOC_DAPM_POST_PMU),
        SND_SOC_DAPM_SUPPLY(c"MIC_BIAS2".as_ptr(), CDC_A_MICB_2_EN as c_int, 7, 0, Some(pm8916_wcd_analog_enable_micbias2), SND_SOC_DAPM_POST_PMU),
        SND_SOC_DAPM_SUPPLY(c"MIC BIAS External1".as_ptr(), SND_SOC_NOPM, 0, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"MIC BIAS External2".as_ptr(), SND_SOC_NOPM, 0, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"MIC BIAS Internal1".as_ptr(), CDC_A_MICB_1_INT_RBIAS as c_int, 7, 0, Some(pm8916_wcd_analog_enable_micbias_int), SND_SOC_DAPM_PRE_PMU),
        SND_SOC_DAPM_SUPPLY(c"MIC BIAS Internal2".as_ptr(), CDC_A_MICB_1_INT_RBIAS as c_int, 4, 0, Some(pm8916_wcd_analog_enable_micbias_int2), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_SUPPLY(c"MIC BIAS Internal3".as_ptr(), CDC_A_MICB_1_INT_RBIAS as c_int, 1, 0, Some(pm8916_wcd_analog_enable_micbias_int), SND_SOC_DAPM_PRE_PMU),
        SND_SOC_DAPM_ADC_E(c"ADC1".as_ptr(), ptr::null(), CDC_A_TX_1_EN as c_int, 7, 0, Some(pm8916_wcd_analog_enable_adc), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_ADC_E(c"ADC2_INP2".as_ptr(), ptr::null(), CDC_A_TX_2_EN as c_int, 7, 0, Some(pm8916_wcd_analog_enable_adc), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_ADC_E(c"ADC2_INP3".as_ptr(), ptr::null(), CDC_A_TX_3_EN as c_int, 7, 0, Some(pm8916_wcd_analog_enable_adc), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_MIXER(c"ADC2".as_ptr(), SND_SOC_NOPM, 0, 0, ptr::null(), 0),
        SND_SOC_DAPM_MIXER(c"ADC3".as_ptr(), SND_SOC_NOPM, 0, 0, ptr::null(), 0),
        SND_SOC_DAPM_MUX(c"ADC2 MUX".as_ptr(), SND_SOC_NOPM, 0, 0, &tx_adc2_mux),
        SND_SOC_DAPM_MUX(c"RDAC2 MUX".as_ptr(), SND_SOC_NOPM, 0, 0, &rdac2_mux),
        /* Analog path clocks */
        SND_SOC_DAPM_SUPPLY(c"EAR_HPHR_CLK".as_ptr(), CDC_D_CDC_ANA_CLK_CTL as c_int, 0, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"EAR_HPHL_CLK".as_ptr(), CDC_D_CDC_ANA_CLK_CTL as c_int, 1, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"SPKR_CLK".as_ptr(), CDC_D_CDC_ANA_CLK_CTL as c_int, 4, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"TXA_CLK25".as_ptr(), CDC_D_CDC_ANA_CLK_CTL as c_int, 5, 0, None, 0),
        /* Digital path clocks */
        SND_SOC_DAPM_SUPPLY(c"RXD1_CLK".as_ptr(), CDC_D_CDC_DIG_CLK_CTL as c_int, 0, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"RXD2_CLK".as_ptr(), CDC_D_CDC_DIG_CLK_CTL as c_int, 1, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"RXD3_CLK".as_ptr(), CDC_D_CDC_DIG_CLK_CTL as c_int, 2, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"TXD_CLK".as_ptr(), CDC_D_CDC_DIG_CLK_CTL as c_int, 4, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"NCP_CLK".as_ptr(), CDC_D_CDC_DIG_CLK_CTL as c_int, 6, 0, None, 0),
        SND_SOC_DAPM_SUPPLY(c"RXD_PDM_CLK".as_ptr(), CDC_D_CDC_DIG_CLK_CTL as c_int, 7, 0, None, 0),
        /* System Clock source */
        SND_SOC_DAPM_SUPPLY(c"A_MCLK".as_ptr(), CDC_D_CDC_TOP_CLK_CTL as c_int, 2, 0, None, 0),
        /* TX ADC and RX DAC Clock source. */
        SND_SOC_DAPM_SUPPLY(c"A_MCLK2".as_ptr(), CDC_D_CDC_TOP_CLK_CTL as c_int, 3, 0, None, 0),
    ]
};

unsafe extern "C" fn pm8916_wcd_analog_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    _data: *mut c_void,
) -> c_int {
    let wcd = snd_soc_component_get_drvdata(component) as *mut pm8916_wcd_analog_priv;
    (*wcd).jack = jack;
    0
}

unsafe extern "C" fn mbhc_btn_release_irq_handler(_irq: c_int, arg: *mut c_void) -> irqreturn_t {
    let priv_ = arg as *mut pm8916_wcd_analog_priv;

    if (*priv_).detect_accessory_type {
        let component = (*priv_).component;
        let val: u32_ = snd_soc_component_read(component, CDC_A_MBHC_RESULT_1);

        /* check if its BTN0 thats released */
        if val != u32::MAX && (val & CDC_A_MBHC_RESULT_1_BTN_RESULT_MASK) == 0 {
            (*priv_).mbhc_btn0_released = 1;
        }
    } else {
        snd_soc_jack_report((*priv_).jack, 0, btn_mask);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn mbhc_btn_press_irq_handler(_irq: c_int, arg: *mut c_void) -> irqreturn_t {
    let priv_ = arg as *mut pm8916_wcd_analog_priv;
    let component = (*priv_).component;
    let btn_result: u32_ = snd_soc_component_read(component, CDC_A_MBHC_RESULT_1) & CDC_A_MBHC_RESULT_1_BTN_RESULT_MASK;

    match btn_result {
        0xf => snd_soc_jack_report((*priv_).jack, SND_JACK_BTN_4, btn_mask),
        0x7 => snd_soc_jack_report((*priv_).jack, SND_JACK_BTN_3, btn_mask),
        0x3 => snd_soc_jack_report((*priv_).jack, SND_JACK_BTN_2, btn_mask),
        0x1 => snd_soc_jack_report((*priv_).jack, SND_JACK_BTN_1, btn_mask),
        0x0 => {
            /* handle BTN_0 specially for type detection */
            if !(*priv_).detect_accessory_type {
                snd_soc_jack_report((*priv_).jack, SND_JACK_BTN_0, btn_mask);
            }
        }
        _ => dev_err((*component).dev, c"Unexpected button press result (%x)".as_ptr(), btn_result),
    }

    IRQ_HANDLED
}

unsafe extern "C" fn pm8916_mbhc_switch_irq_handler(_irq: c_int, arg: *mut c_void) -> irqreturn_t {
    let priv_ = arg as *mut pm8916_wcd_analog_priv;
    let component = (*priv_).component;
    let mut ins = false;

    if snd_soc_component_read(component, CDC_A_MBHC_DET_CTL_1) & CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_MASK != 0 {
        ins = true;
    }

    /* Set the detection type appropriately */
    snd_soc_component_update_bits(
        component,
        CDC_A_MBHC_DET_CTL_1,
        CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_MASK,
        ((!ins) as u32) << CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_SHIFT,
    );

    if ins {
        /* hs insertion */
        let mut micbias_enabled = false;

        if snd_soc_component_read(component, CDC_A_MICB_2_EN) & CDC_A_MICB_2_EN_ENABLE != 0 {
            micbias_enabled = true;
        }

        pm8916_mbhc_configure_bias(priv_, micbias_enabled);

        /*
         * if only a btn0 press event is receive just before
         * insert event then its a 3 pole headphone else if
         * both press and release event received then its
         * a headset.
         */
        if (*priv_).mbhc_btn0_released != 0 {
            snd_soc_jack_report((*priv_).jack, SND_JACK_HEADSET, hs_jack_mask);
        } else {
            snd_soc_jack_report((*priv_).jack, SND_JACK_HEADPHONE, hs_jack_mask);
        }

        (*priv_).detect_accessory_type = false;
    } else {
        /* removal */
        snd_soc_jack_report((*priv_).jack, 0, hs_jack_mask);
        (*priv_).detect_accessory_type = true;
        (*priv_).mbhc_btn0_released = 0;
    }

    IRQ_HANDLED
}

static mut pm8916_wcd_analog_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"pm8916_wcd_analog_pdm_rx".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: c"PDM Playback".as_ptr(),
            rates: MSM8916_WCD_ANALOG_RATES,
            formats: MSM8916_WCD_ANALOG_FORMATS,
            channels_min: 1,
            channels_max: 3,
        },
        capture: snd_soc_pcm_stream::zeroed(),
    },
    snd_soc_dai_driver {
        name: c"pm8916_wcd_analog_pdm_tx".as_ptr(),
        id: 1,
        playback: snd_soc_pcm_stream::zeroed(),
        capture: snd_soc_pcm_stream {
            stream_name: c"PDM Capture".as_ptr(),
            rates: MSM8916_WCD_ANALOG_RATES,
            formats: MSM8916_WCD_ANALOG_FORMATS,
            channels_min: 1,
            channels_max: 4,
        },
    },
];

static pm8916_wcd_analog: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(pm8916_wcd_analog_probe),
    remove: Some(pm8916_wcd_analog_remove),
    set_jack: Some(pm8916_wcd_analog_set_jack),
    controls: pm8916_wcd_analog_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&pm8916_wcd_analog_snd_controls) as c_uint,
    dapm_widgets: pm8916_wcd_analog_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&pm8916_wcd_analog_dapm_widgets) as c_uint,
    dapm_routes: pm8916_wcd_analog_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&pm8916_wcd_analog_audio_map) as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn pm8916_wcd_analog_parse_dt(
    dev: *mut device,
    priv_: *mut pm8916_wcd_analog_priv,
) -> c_int {
    let mut rval: c_int;

    if of_property_read_bool((*dev).of_node, c"qcom,micbias1-ext-cap".as_ptr()) {
        (*priv_).micbias1_cap_mode = MICB_1_EN_EXT_BYP_CAP;
    } else {
        (*priv_).micbias1_cap_mode = MICB_1_EN_NO_EXT_BYP_CAP;
    }

    if of_property_read_bool((*dev).of_node, c"qcom,micbias2-ext-cap".as_ptr()) {
        (*priv_).micbias2_cap_mode = MICB_1_EN_EXT_BYP_CAP;
    } else {
        (*priv_).micbias2_cap_mode = MICB_1_EN_NO_EXT_BYP_CAP;
    }

    of_property_read_u32((*dev).of_node, c"qcom,micbias-lvl".as_ptr(), &mut (*priv_).micbias_mv);

    if of_property_read_bool((*dev).of_node, c"qcom,hphl-jack-type-normally-open".as_ptr()) {
        (*priv_).hphl_jack_type_normally_open = true;
    } else {
        (*priv_).hphl_jack_type_normally_open = false;
    }

    if of_property_read_bool((*dev).of_node, c"qcom,gnd-jack-type-normally-open".as_ptr()) {
        (*priv_).gnd_jack_type_normally_open = true;
    } else {
        (*priv_).gnd_jack_type_normally_open = false;
    }

    (*priv_).mbhc_btn_enabled = true;
    rval = of_property_read_u32_array(
        (*dev).of_node,
        c"qcom,mbhc-vthreshold-low".as_ptr(),
        (*priv_).vref_btn_cs.as_mut_ptr(),
        MBHC_MAX_BUTTONS,
    );
    if rval < 0 {
        (*priv_).mbhc_btn_enabled = false;
    } else {
        rval = of_property_read_u32_array(
            (*dev).of_node,
            c"qcom,mbhc-vthreshold-high".as_ptr(),
            (*priv_).vref_btn_micb.as_mut_ptr(),
            MBHC_MAX_BUTTONS,
        );
        if rval < 0 {
            (*priv_).mbhc_btn_enabled = false;
        }
    }

    if !(*priv_).mbhc_btn_enabled {
        dev_err(dev, c"DT property missing, MBHC btn detection disabled\n".as_ptr());
    }

    0
}

unsafe extern "C" fn pm8916_wcd_analog_spmi_probe(pdev: *mut platform_device) -> c_int {
    let mut priv_: *mut pm8916_wcd_analog_priv;
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: c_int;
    let mut irq: c_int;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<pm8916_wcd_analog_priv>(), GFP_KERNEL) as *mut pm8916_wcd_analog_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    ret = pm8916_wcd_analog_parse_dt(dev, priv_);
    if ret < 0 {
        return ret;
    }

    for i in 0..ARRAY_SIZE(&supply_names) {
        (*priv_).supplies[i].supply = supply_names[i];
    }

    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(&(*priv_).supplies) as c_int, (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, c"Failed to get regulator supplies %d\n".as_ptr(), ret);
        return ret;
    }

    irq = platform_get_irq_byname(pdev, c"mbhc_switch_int".as_ptr());
    if irq < 0 {
        return irq;
    }

    ret = devm_request_threaded_irq(
        dev,
        irq,
        None,
        Some(pm8916_mbhc_switch_irq_handler),
        IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
        c"mbhc switch irq".as_ptr(),
        priv_ as *mut c_void,
    );
    if ret != 0 {
        dev_err(dev, c"cannot request mbhc switch irq\n".as_ptr());
        return ret;
    }

    if (*priv_).mbhc_btn_enabled {
        irq = platform_get_irq_byname(pdev, c"mbhc_but_press_det".as_ptr());
        if irq < 0 {
            return irq;
        }

        ret = devm_request_threaded_irq(
            dev,
            irq,
            None,
            Some(mbhc_btn_press_irq_handler),
            IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
            c"mbhc btn press irq".as_ptr(),
            priv_ as *mut c_void,
        );
        if ret != 0 {
            dev_err(dev, c"cannot request mbhc button press irq\n".as_ptr());
            return ret;
        }

        irq = platform_get_irq_byname(pdev, c"mbhc_but_rel_det".as_ptr());
        if irq < 0 {
            return irq;
        }

        ret = devm_request_threaded_irq(
            dev,
            irq,
            None,
            Some(mbhc_btn_release_irq_handler),
            IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
            c"mbhc btn release irq".as_ptr(),
            priv_ as *mut c_void,
        );
        if ret != 0 {
            dev_err(dev, c"cannot request mbhc button release irq\n".as_ptr());
            return ret;
        }
    }

    dev_set_drvdata(dev, priv_ as *mut c_void);

    devm_snd_soc_register_component(
        dev,
        &pm8916_wcd_analog,
        pm8916_wcd_analog_dai.as_mut_ptr(),
        ARRAY_SIZE(&pm8916_wcd_analog_dai) as c_int,
    )
}

static pm8916_wcd_analog_spmi_match_table: [of_device_id; 4] = [
    of_device_id { compatible: c"qcom,pm8916-wcd-analog-codec".as_ptr(), data: &pm8916_data as *const _ as *const c_void },
    of_device_id { compatible: c"qcom,pm8950-wcd-analog-codec".as_ptr(), data: &pm8950_data as *const _ as *const c_void },
    of_device_id { compatible: c"qcom,pm8953-wcd-analog-codec".as_ptr(), data: &pm8953_data as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];

MODULE_DEVICE_TABLE(c"of".as_ptr(), pm8916_wcd_analog_spmi_match_table.as_ptr());

static mut pm8916_wcd_analog_spmi_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"qcom,pm8916-wcd-spmi-codec".as_ptr(),
        of_match_table: pm8916_wcd_analog_spmi_match_table.as_ptr(),
    },
    probe: Some(pm8916_wcd_analog_spmi_probe),
};

module_platform_driver(&mut pm8916_wcd_analog_spmi_driver);

MODULE_AUTHOR(c"Srinivas Kandagatla <srinivas.kandagatla@linaro.org>".as_ptr());
MODULE_DESCRIPTION(c"PMIC PM8916 WCD Analog Codec driver".as_ptr());
MODULE_LICENSE(c"GPL v2".as_ptr());

#[repr(C)]
struct clk {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}
#[repr(C)]
struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_widget {
    reg: c_int,
    dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
struct device {
    parent: *mut device,
    of_node: *mut device_node,
}
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct regulator_bulk_data {
    supply: *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct reg_default {
    reg: u32,
    def: u32,
}
#[repr(C)]
struct soc_enum {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
const fn route(sink: &core::ffi::CStr, control: *const c_char, source: &core::ffi::CStr) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink: sink.as_ptr(), control, source: source.as_ptr() }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    rates: u32,
    formats: u64,
    channels_min: c_uint,
    channels_max: c_uint,
}
impl snd_soc_pcm_stream {
    const fn zeroed() -> Self {
        Self { stream_name: ptr::null(), rates: 0, formats: 0, channels_min: 0, channels_max: 0 }
    }
}
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
}
#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}
#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}
#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    static SNDRV_PCM_RATE_8000: u32;
    static SNDRV_PCM_RATE_16000: u32;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_BTN_4: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_SOC_DAPM_POST_PMU: u32;
    static SND_SOC_DAPM_PRE_PMU: u32;
    static SND_SOC_DAPM_POST_PMD: u32;
    static SND_SOC_DAPM_PRE_PMD: u32;
    static SND_SOC_NOPM: c_int;
    static IRQ_HANDLED: irqreturn_t;
    static IRQF_TRIGGER_RISING: c_uint;
    static IRQF_TRIGGER_FALLING: c_uint;
    static IRQF_ONESHOT: c_uint;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: u32, mask: u32, val: u32) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: u32) -> u32;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: u32, val: u32) -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut u32, sz: usize) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_irq_byname(pdev: *mut platform_device, name: *const c_char) -> c_int;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_uint,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);

    fn SOC_ENUM_SINGLE_VIRT(items: usize, texts: *const *const c_char) -> soc_enum;
    fn SOC_ENUM_SINGLE(reg: u32, shift_l: c_uint, items: c_uint, texts: *const *const c_char) -> soc_enum;
    fn SOC_DAPM_ENUM(name: *const c_char, e: *const soc_enum) -> snd_kcontrol_new;
    fn SOC_DAPM_SINGLE(name: *const c_char, reg: u32, shift: c_uint, max: c_uint, invert: c_uint) -> snd_kcontrol_new;
    fn SOC_SINGLE_TLV(name: *const c_char, reg: u32, shift: c_uint, max: c_uint, invert: c_uint, tlv: *const c_uint) -> snd_kcontrol_new;
    fn DECLARE_TLV_DB_SCALE(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4];
    fn SND_SOC_DAPM_AIF_IN(name: *const c_char, stream: *const c_char, slot: c_int, reg: c_int, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_AIF_OUT(name: *const c_char, stream: *const c_char, slot: c_int, reg: c_int, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SUPPLY(name: *const c_char, reg: c_int, shift: c_uint, invert: c_uint, event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>, event_flags: u32) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_PGA_E(name: *const c_char, reg: c_int, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_int, event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>, event_flags: u32) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MUX(name: *const c_char, reg: c_int, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIXER(name: *const c_char, reg: c_int, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_int) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_REGULATOR_SUPPLY(name: *const c_char, delay: c_int, flags: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_ADC_E(name: *const c_char, stream: *const c_char, reg: c_int, shift: c_uint, invert: c_uint, event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>, event_flags: u32) -> snd_soc_dapm_widget;

    fn MODULE_DEVICE_TABLE(kind: *const c_char, table: *const of_device_id);
    fn module_platform_driver(driver: *mut platform_driver);
    fn MODULE_AUTHOR(author: *const c_char);
    fn MODULE_DESCRIPTION(description: *const c_char);
    fn MODULE_LICENSE(license: *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
