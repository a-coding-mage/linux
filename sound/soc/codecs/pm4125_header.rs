/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (c) 2023-2024 Qualcomm Innovation Center, Inc. All rights reserved.
 */

use core::ffi::{c_int, c_uint, c_void};

const fn BIT(nr: u32) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: u32, l: u32) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

/* Dependencies from <linux/soundwire/sdw.h>, <linux/soundwire/sdw_type.h>,
 * and "wcd-common.h".
 */
#[repr(C)]
pub struct sdw_slave {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_stream_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_port_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wcd_sdw_ch_info {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub static EOPNOTSUPP: c_int;
}

pub const SDW_MAX_PORTS: usize = 15usize; /* external dependency value from SoundWire headers */

pub const PM4125_ANA_BASE_ADDR: c_uint = 0x3000;
pub const PM4125_DIG_BASE_ADDR: c_uint = 0x3400;

pub const PM4125_ANA_MICBIAS_MICB_1_2_EN: c_uint = PM4125_ANA_BASE_ADDR + 0x040;
pub const PM4125_ANA_MICBIAS_MICB1_PULL_UP_MASK: c_uint = BIT(5);
pub const PM4125_ANA_MICBIAS_MICB2_PULL_UP_MASK: c_uint = BIT(1);
pub const PM4125_ANA_MICBIAS_MICB2_PULL_DN_MASK: c_uint = BIT(0);
pub const PM4125_ANA_MICBIAS_MICB_PULL_ENABLE: c_uint = 1;
pub const PM4125_ANA_MICBIAS_MICB_PULL_DISABLE: c_uint = 0;
pub const PM4125_ANA_MICBIAS_MICB_3_EN: c_uint = PM4125_ANA_BASE_ADDR + 0x041;
pub const PM4125_ANA_MICBIAS_LDO_1_SETTING: c_uint = PM4125_ANA_BASE_ADDR + 0x042;
pub const PM4125_ANA_MICBIAS_MICB_OUT_VAL_MASK: c_uint = GENMASK(7, 3);
pub const PM4125_ANA_MICBIAS_LDO_1_CTRL: c_uint = PM4125_ANA_BASE_ADDR + 0x043;
pub const PM4125_ANA_TX_AMIC1: c_uint = PM4125_ANA_BASE_ADDR + 0x047;
pub const PM4125_ANA_TX_AMIC2: c_uint = PM4125_ANA_BASE_ADDR + 0x048;
pub const PM4125_ANA_MBHC_MECH: c_uint = PM4125_ANA_BASE_ADDR + 0x05A;
pub const PM4125_ANA_MBHC_ELECT: c_uint = PM4125_ANA_BASE_ADDR + 0x05B;
pub const PM4125_ANA_MBHC_ELECT_BIAS_EN_MASK: c_uint = BIT(0);
pub const PM4125_ANA_MBHC_ELECT_BIAS_ENABLE: c_uint = 1;
pub const PM4125_ANA_MBHC_ELECT_BIAS_DISABLE: c_uint = 0;
pub const PM4125_ANA_MBHC_ZDET: c_uint = PM4125_ANA_BASE_ADDR + 0x05C;
pub const PM4125_ANA_MBHC_RESULT_1: c_uint = PM4125_ANA_BASE_ADDR + 0x05D;
pub const PM4125_ANA_MBHC_RESULT_2: c_uint = PM4125_ANA_BASE_ADDR + 0x05E;
pub const PM4125_ANA_MBHC_RESULT_3: c_uint = PM4125_ANA_BASE_ADDR + 0x05F;
pub const PM4125_ANA_MBHC_BTN0_ZDET_VREF1: c_uint = PM4125_ANA_BASE_ADDR + 0x060;
pub const PM4125_ANA_MBHC_BTN0_THRESHOLD_MASK: c_uint = GENMASK(7, 2);
pub const PM4125_ANA_MBHC_BTN1_ZDET_VREF2: c_uint = PM4125_ANA_BASE_ADDR + 0x061;
pub const PM4125_ANA_MBHC_BTN2_ZDET_VREF3: c_uint = PM4125_ANA_BASE_ADDR + 0x062;
pub const PM4125_ANA_MBHC_BTN3_ZDET_DBG_400: c_uint = PM4125_ANA_BASE_ADDR + 0x063;
pub const PM4125_ANA_MBHC_BTN4_ZDET_DBG_1400: c_uint = PM4125_ANA_BASE_ADDR + 0x064;
pub const PM4125_ANA_MBHC_MICB2_RAMP: c_uint = PM4125_ANA_BASE_ADDR + 0x065;
pub const PM4125_ANA_MBHC_CTL_1: c_uint = PM4125_ANA_BASE_ADDR + 0x066;
pub const PM4125_ANA_MBHC_CTL_2: c_uint = PM4125_ANA_BASE_ADDR + 0x067;
pub const PM4125_ANA_MBHC_PLUG_DETECT_CTL: c_uint = PM4125_ANA_BASE_ADDR + 0x068;
pub const PM4125_ANA_MBHC_ZDET_ANA_CTL: c_uint = PM4125_ANA_BASE_ADDR + 0x069;
pub const PM4125_ANA_MBHC_ZDET_RAMP_CTL: c_uint = PM4125_ANA_BASE_ADDR + 0x06A;
pub const PM4125_ANA_MBHC_FSM_STATUS: c_uint = PM4125_ANA_BASE_ADDR + 0x06B;
pub const PM4125_ANA_MBHC_ADC_RESULT: c_uint = PM4125_ANA_BASE_ADDR + 0x06C;
pub const PM4125_ANA_MBHC_CTL_CLK: c_uint = PM4125_ANA_BASE_ADDR + 0x06D;
pub const PM4125_ANA_MBHC_ZDET_CALIB_RESULT: c_uint = PM4125_ANA_BASE_ADDR + 0x072;
pub const PM4125_ANA_NCP_EN: c_uint = PM4125_ANA_BASE_ADDR + 0x077;
pub const PM4125_ANA_NCP_ENABLE_MASK: c_uint = BIT(0);
pub const PM4125_ANA_NCP_ENABLE: c_uint = 1;
pub const PM4125_ANA_NCP_DISABLE: c_uint = 0;
pub const PM4125_ANA_NCP_VCTRL: c_uint = PM4125_ANA_BASE_ADDR + 0x07C;
pub const PM4125_ANA_HPHPA_CNP_CTL_1: c_uint = PM4125_ANA_BASE_ADDR + 0x083;
pub const PM4125_ANA_HPHPA_CNP_CTL_1_EN_MASK: c_uint = BIT(1);
pub const PM4125_ANA_HPHPA_CNP_CTL_1_EN: c_uint = 1;
pub const PM4125_ANA_HPHPA_CNP_CTL_2: c_uint = PM4125_ANA_BASE_ADDR + 0x084;
pub const PM4125_ANA_HPHPA_CNP_OCP_EN_L_MASK: c_uint = BIT(1);
pub const PM4125_ANA_HPHPA_CNP_OCP_EN_R_MASK: c_uint = BIT(0);
pub const PM4125_ANA_HPHPA_CNP_OCP_ENABLE: c_uint = 1;
pub const PM4125_ANA_HPHPA_CNP_OCP_DISABLE: c_uint = 0;
pub const PM4125_ANA_HPHPA_PA_STATUS: c_uint = PM4125_ANA_BASE_ADDR + 0x087;
pub const PM4125_ANA_HPHPA_FSM_CLK: c_uint = PM4125_ANA_BASE_ADDR + 0x088;
pub const PM4125_ANA_HPHPA_FSM_CLK_DIV_EN_MASK: c_uint = BIT(7);
pub const PM4125_ANA_HPHPA_FSM_CLK_DIV_ENABLE: c_uint = 1;
pub const PM4125_ANA_HPHPA_FSM_CLK_DIV_DISABLE: c_uint = 0;
pub const PM4125_ANA_HPHPA_FSM_DIV_RATIO_MASK: c_uint = GENMASK(6, 0);
pub const PM4125_ANA_HPHPA_FSM_DIV_RATIO_68: c_uint = 0x11;
pub const PM4125_ANA_HPHPA_L_GAIN: c_uint = PM4125_ANA_BASE_ADDR + 0x08B;
pub const PM4125_ANA_HPHPA_R_GAIN: c_uint = PM4125_ANA_BASE_ADDR + 0x08C;
pub const PM4125_ANA_HPHPA_SPARE_CTL: c_uint = PM4125_ANA_BASE_ADDR + 0x08E;
pub const PM4125_SWR_HPHPA_HD2: c_uint = PM4125_ANA_BASE_ADDR + 0x090;
pub const PM4125_SWR_HPHPA_HD2_LEFT_MASK: c_uint = GENMASK(5, 3);
pub const PM4125_SWR_HPHPA_HD2_RIGHT_MASK: c_uint = GENMASK(2, 0);
pub const PM4125_SWR_HPHPA_HD2_ENABLE: c_uint = BIT(2) | BIT(1) | BIT(0);
pub const PM4125_ANA_SURGE_EN: c_uint = PM4125_ANA_BASE_ADDR + 0x097;
pub const PM4125_ANA_SURGE_PROTECTION_HPHL_MASK: c_uint = BIT(7);
pub const PM4125_ANA_SURGE_PROTECTION_HPHR_MASK: c_uint = BIT(6);
pub const PM4125_ANA_SURGE_PROTECTION_ENABLE: c_uint = 1;
pub const PM4125_ANA_SURGE_PROTECTION_DISABLE: c_uint = 0;
pub const PM4125_ANA_COMBOPA_CTL: c_uint = PM4125_ANA_BASE_ADDR + 0x09B;
pub const PM4125_ANA_COMBO_PA_SELECT_MASK: c_uint = BIT(6);
pub const PM4125_ANA_COMBO_PA_SELECT_EAR: c_uint = 0;
pub const PM4125_ANA_COMBO_PA_SELECT_LO: c_uint = 1;
pub const PM4125_ANA_COMBOPA_CTL_4: c_uint = PM4125_ANA_BASE_ADDR + 0x09F;
pub const PM4125_ANA_COMBOPA_CTL_5: c_uint = PM4125_ANA_BASE_ADDR + 0x0A0;
pub const PM4125_ANA_RXLDO_CTL: c_uint = PM4125_ANA_BASE_ADDR + 0x0B2;
pub const PM4125_ANA_MBIAS_EN: c_uint = PM4125_ANA_BASE_ADDR + 0x0B4;
pub const PM4125_ANA_MBIAS_EN_GLOBAL_MASK: c_uint = BIT(5);
pub const PM4125_ANA_MBIAS_EN_V2I_MASK: c_uint = BIT(4);
pub const PM4125_ANA_MBIAS_EN_ENABLE: c_uint = 1;
pub const PM4125_ANA_MBIAS_EN_DISABLE: c_uint = 0;

pub const PM4125_DIG_SWR_CHIP_ID0: c_uint = PM4125_DIG_BASE_ADDR + 0x001;
pub const PM4125_DIG_SWR_CHIP_ID1: c_uint = PM4125_DIG_BASE_ADDR + 0x002;
pub const PM4125_DIG_SWR_CHIP_ID2: c_uint = PM4125_DIG_BASE_ADDR + 0x003;
pub const PM4125_DIG_SWR_CHIP_ID3: c_uint = PM4125_DIG_BASE_ADDR + 0x004;
pub const PM4125_DIG_SWR_SWR_TX_CLK_RATE: c_uint = PM4125_DIG_BASE_ADDR + 0x040;
pub const PM4125_DIG_SWR_CDC_RST_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x041;
pub const PM4125_DIG_SWR_TOP_CLK_CFG: c_uint = PM4125_DIG_BASE_ADDR + 0x042;
pub const PM4125_DIG_SWR_CDC_RX_CLK_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x043;
pub const PM4125_DIG_SWR_ANA_RX_DIV2_CLK_EN_MASK: c_uint = BIT(5);
pub const PM4125_DIG_SWR_ANA_RX_CLK_EN_MASK: c_uint = BIT(4);
pub const PM4125_DIG_SWR_RX1_CLK_EN_MASK: c_uint = BIT(1);
pub const PM4125_DIG_SWR_RX0_CLK_EN_MASK: c_uint = BIT(0);
pub const PM4125_DIG_SWR_RX_CLK_ENABLE: c_uint = 1;
pub const PM4125_DIG_SWR_RX_CLK_DISABLE: c_uint = 0;
pub const PM4125_DIG_SWR_CDC_TX_CLK_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x044;
pub const PM4125_DIG_SWR_SWR_RST_EN: c_uint = PM4125_DIG_BASE_ADDR + 0x045;
pub const PM4125_DIG_SWR_CDC_RX_RST: c_uint = PM4125_DIG_BASE_ADDR + 0x047;
pub const PM4125_DIG_SWR_CDC_RX0_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x048;
pub const PM4125_DIG_SWR_DSM_DITHER_EN_MASK: c_uint = BIT(7);
pub const PM4125_DIG_SWR_DSM_DITHER_DISABLE: c_uint = 0;
pub const PM4125_DIG_SWR_DSM_DITHER_ENABLE: c_uint = 1;
pub const PM4125_DIG_SWR_CDC_RX1_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x049;
pub const PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1: c_uint = PM4125_DIG_BASE_ADDR + 0x04B;
pub const PM4125_DIG_SWR_TX_ANA_TXD1_MODE_MASK: c_uint = GENMASK(7, 4);
pub const PM4125_DIG_SWR_TX_ANA_TXD0_MODE_MASK: c_uint = GENMASK(3, 0);
pub const PM4125_DIG_SWR_TXD_MODE_ULPI: c_uint = 0x9;
pub const PM4125_DIG_SWR_TXD_MODE_NORMAL: c_uint = 0x3;
pub const PM4125_DIG_SWR_CDC_COMP_CTL_0: c_uint = PM4125_DIG_BASE_ADDR + 0x04F;
pub const PM4125_DIG_SWR_COMP_HPHL_EN_MASK: c_uint = BIT(1);
pub const PM4125_DIG_SWR_COMP_HPHR_EN_MASK: c_uint = BIT(0);
pub const PM4125_DIG_SWR_COMP_ENABLE: c_uint = 1;
pub const PM4125_DIG_SWR_COMP_DISABLE: c_uint = 0;
pub const PM4125_DIG_SWR_CDC_RX_DELAY_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x052;
pub const PM4125_DIG_SWR_CDC_RX_GAIN_0: c_uint = PM4125_DIG_BASE_ADDR + 0x053;
pub const PM4125_DIG_SWR_CDC_RX_GAIN_1: c_uint = PM4125_DIG_BASE_ADDR + 0x054;
pub const PM4125_DIG_SWR_CDC_RX_GAIN_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x057;
pub const PM4125_DIG_SWR_RX1_EN_MASK: c_uint = BIT(3);
pub const PM4125_DIG_SWR_RX0_EN_MASK: c_uint = BIT(2);
pub const PM4125_DIG_SWR_RX_INPUT_DISABLE: c_uint = 0;
pub const PM4125_DIG_SWR_RX_INPUT_ENABLE: c_uint = 1;
pub const PM4125_DIG_SWR_CDC_TX0_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x060;
pub const PM4125_DIG_SWR_CDC_TX1_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x061;
pub const PM4125_DIG_SWR_CDC_TX_RST: c_uint = PM4125_DIG_BASE_ADDR + 0x063;
pub const PM4125_DIG_SWR_CDC_REQ0_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x064;
pub const PM4125_DIG_SWR_CDC_REQ1_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x065;
pub const PM4125_DIG_SWR_CDC_RST: c_uint = PM4125_DIG_BASE_ADDR + 0x067;
pub const PM4125_DIG_SWR_CDC_AMIC_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x06A;
pub const PM4125_DIG_SWR_AMIC_SELECT_MASK: c_uint = BIT(1);
pub const PM4125_DIG_SWR_AMIC_SELECT_DMIC1: c_uint = 0;
pub const PM4125_DIG_SWR_AMIC_SELECT_AMIC3: c_uint = 1;
pub const PM4125_DIG_SWR_CDC_DMIC_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x06B;
pub const PM4125_DIG_SWR_CDC_DMIC1_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x06C;
pub const PM4125_DIG_SWR_DMIC1_CLK_EN_MASK: c_uint = BIT(3);
pub const PM4125_DIG_SWR_DMIC1_CLK_ENABLE: c_uint = 1;
pub const PM4125_DIG_SWR_DMIC1_CLK_DISABLE: c_uint = 0;
pub const PM4125_DIG_SWR_CDC_DMIC1_RATE: c_uint = PM4125_DIG_BASE_ADDR + 0x06D;
pub const PM4125_DIG_SWR_PDM_WD_CTL0: c_uint = PM4125_DIG_BASE_ADDR + 0x070;
pub const PM4125_WDT_ENABLE_MASK: c_uint = GENMASK(1, 0);
pub const PM4125_WDT_ENABLE_RX0_L: c_uint = BIT(0);
pub const PM4125_WDT_ENABLE_RX0_M: c_uint = BIT(1);
pub const PM4125_DIG_SWR_PDM_WD_CTL1: c_uint = PM4125_DIG_BASE_ADDR + 0x071;
pub const PM4125_WDT_ENABLE_RX1_L: c_uint = BIT(0);
pub const PM4125_WDT_ENABLE_RX1_M: c_uint = BIT(1);
pub const PM4125_DIG_SWR_INTR_MODE: c_uint = PM4125_DIG_BASE_ADDR + 0x080;
pub const PM4125_DIG_SWR_INTR_MASK_0: c_uint = PM4125_DIG_BASE_ADDR + 0x081;
pub const PM4125_DIG_SWR_INTR_MASK_1: c_uint = PM4125_DIG_BASE_ADDR + 0x082;
pub const PM4125_DIG_SWR_INTR_MASK_2: c_uint = PM4125_DIG_BASE_ADDR + 0x083;
pub const PM4125_DIG_SWR_INTR_STATUS_0: c_uint = PM4125_DIG_BASE_ADDR + 0x084;
pub const PM4125_DIG_SWR_INTR_STATUS_1: c_uint = PM4125_DIG_BASE_ADDR + 0x085;
pub const PM4125_DIG_SWR_INTR_STATUS_2: c_uint = PM4125_DIG_BASE_ADDR + 0x086;
pub const PM4125_DIG_SWR_INTR_CLEAR_0: c_uint = PM4125_DIG_BASE_ADDR + 0x087;
pub const PM4125_DIG_SWR_INTR_CLEAR_1: c_uint = PM4125_DIG_BASE_ADDR + 0x088;
pub const PM4125_DIG_SWR_INTR_CLEAR_2: c_uint = PM4125_DIG_BASE_ADDR + 0x089;
pub const PM4125_DIG_SWR_INTR_LEVEL_0: c_uint = PM4125_DIG_BASE_ADDR + 0x08A;
pub const PM4125_DIG_SWR_INTR_LEVEL_1: c_uint = PM4125_DIG_BASE_ADDR + 0x08B;
pub const PM4125_DIG_SWR_INTR_LEVEL_2: c_uint = PM4125_DIG_BASE_ADDR + 0x08C;
pub const PM4125_DIG_SWR_CDC_CONN_RX0_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x093;
pub const PM4125_DIG_SWR_CDC_CONN_RX1_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x094;
pub const PM4125_DIG_SWR_LOOP_BACK_MODE: c_uint = PM4125_DIG_BASE_ADDR + 0x097;
pub const PM4125_DIG_SWR_DRIVE_STRENGTH_0: c_uint = PM4125_DIG_BASE_ADDR + 0x0A0;
pub const PM4125_DIG_SWR_DIG_DEBUG_CTL: c_uint = PM4125_DIG_BASE_ADDR + 0x0AB;
pub const PM4125_DIG_SWR_DIG_DEBUG_EN: c_uint = PM4125_DIG_BASE_ADDR + 0x0AC;
pub const PM4125_DIG_SWR_DEM_BYPASS_DATA0: c_uint = PM4125_DIG_BASE_ADDR + 0x0B0;
pub const PM4125_DIG_SWR_DEM_BYPASS_DATA1: c_uint = PM4125_DIG_BASE_ADDR + 0x0B1;
pub const PM4125_DIG_SWR_DEM_BYPASS_DATA2: c_uint = PM4125_DIG_BASE_ADDR + 0x0B2;
pub const PM4125_DIG_SWR_DEM_BYPASS_DATA3: c_uint = PM4125_DIG_BASE_ADDR + 0x0B3;

pub const PM4125_ANALOG_REGISTERS_MAX_SIZE: c_uint = PM4125_ANA_BASE_ADDR + 0x0B5;
pub const PM4125_DIGITAL_REGISTERS_MAX_SIZE: c_uint = PM4125_DIG_BASE_ADDR + 0x0B4;
pub const PM4125_ANALOG_MAX_REGISTER: c_uint = PM4125_ANALOG_REGISTERS_MAX_SIZE - 1;
pub const PM4125_DIGITAL_MAX_REGISTER: c_uint = PM4125_DIGITAL_REGISTERS_MAX_SIZE - 1;
pub const PM4125_MAX_REGISTER: c_uint = PM4125_DIGITAL_MAX_REGISTER;

pub const PM4125_MAX_MICBIAS: usize = 3;
pub const PM4125_MAX_SWR_CH_IDS: usize = 15;
pub const fn PM4125_SWRM_CH_MASK(ch_idx: c_uint) -> c_uint {
    BIT(ch_idx - 1)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pm4125_tx_sdw_ports {
    PM4125_ADC_1_2_DMIC1L_BCS_PORT = 1,
    PM4125_DMIC_1L_1R_ADC1_BCS_PORT = 2,
    PM4125_MAX_TX_SWR_PORTS = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pm4125_rx_sdw_ports {
    PM4125_HPH_PORT = 1,
    PM4125_COMP_PORT = 2,
    PM4125_MAX_SWR_PORTS = 2,
}

#[repr(C)]
pub struct pm4125_priv {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pm4125_sdw_priv {
    pub sdev: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; PM4125_MAX_SWR_PORTS as usize],
    pub ch_info: *mut wcd_sdw_ch_info,
    pub port_enable: [bool; PM4125_MAX_SWR_CH_IDS],
    pub master_channel_map: [c_uint; SDW_MAX_PORTS],
    pub active_ports: c_int,
    pub num_ports: c_int,
    pub is_tx: bool,
    pub pm4125: *mut pm4125_priv,
    pub slave_irq: *mut irq_domain,
    pub regmap: *mut regmap,
}

/* Original C condition: #if IS_ENABLED(CONFIG_SND_SOC_PM4125_SDW) */
#[cfg(CONFIG_SND_SOC_PM4125_SDW)]
unsafe extern "C" {
    pub fn pm4125_sdw_free(
        pm4125: *mut pm4125_sdw_priv,
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    pub fn pm4125_sdw_set_sdw_stream(
        pm4125: *mut pm4125_sdw_priv,
        dai: *mut snd_soc_dai,
        stream: *mut c_void,
        direction: c_int,
    ) -> c_int;
    pub fn pm4125_sdw_hw_params(
        pm4125: *mut pm4125_sdw_priv,
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
}

/* Original C #else fallback for !IS_ENABLED(CONFIG_SND_SOC_PM4125_SDW). */
#[cfg(not(CONFIG_SND_SOC_PM4125_SDW))]
pub unsafe fn pm4125_sdw_free(
    _pm4125: *mut pm4125_sdw_priv,
    _substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    unsafe { -EOPNOTSUPP }
}

#[cfg(not(CONFIG_SND_SOC_PM4125_SDW))]
pub unsafe fn pm4125_sdw_set_sdw_stream(
    _pm4125: *mut pm4125_sdw_priv,
    _dai: *mut snd_soc_dai,
    _stream: *mut c_void,
    _direction: c_int,
) -> c_int {
    unsafe { -EOPNOTSUPP }
}

#[cfg(not(CONFIG_SND_SOC_PM4125_SDW))]
pub unsafe fn pm4125_sdw_hw_params(
    _pm4125: *mut pm4125_sdw_priv,
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    _dai: *mut snd_soc_dai,
) -> c_int {
    unsafe { -EOPNOTSUPP }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pm4125_irqs {
    /* INTR_CTRL_INT_MASK_0 */
    PM4125_IRQ_MBHC_BUTTON_PRESS_DET = 0,
    PM4125_IRQ_MBHC_BUTTON_RELEASE_DET = 1,
    PM4125_IRQ_MBHC_ELECT_INS_REM_DET = 2,
    PM4125_IRQ_MBHC_ELECT_INS_REM_LEG_DET = 3,
    PM4125_IRQ_MBHC_SW_DET = 4,
    PM4125_IRQ_HPHR_OCP_INT = 5,
    PM4125_IRQ_HPHR_CNP_INT = 6,
    PM4125_IRQ_HPHL_OCP_INT = 7,

    /* INTR_CTRL_INT_MASK_1 */
    PM4125_IRQ_HPHL_CNP_INT = 8,
    PM4125_IRQ_EAR_CNP_INT = 9,
    PM4125_IRQ_EAR_SCD_INT = 10,
    PM4125_IRQ_AUX_CNP_INT = 11,
    PM4125_IRQ_AUX_SCD_INT = 12,
    PM4125_IRQ_HPHL_PDM_WD_INT = 13,
    PM4125_IRQ_HPHR_PDM_WD_INT = 14,
    PM4125_IRQ_AUX_PDM_WD_INT = 15,

    /* INTR_CTRL_INT_MASK_2 */
    PM4125_IRQ_LDORT_SCD_INT = 16,
    PM4125_IRQ_MBHC_MOISTURE_INT = 17,
    PM4125_IRQ_HPHL_SURGE_DET_INT = 18,
    PM4125_IRQ_HPHR_SURGE_DET_INT = 19,
    PM4125_NUM_IRQS = 20,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pm4125_tx_sdw_channels {
    PM4125_ADC1 = 0,
    PM4125_ADC2 = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pm4125_rx_sdw_channels {
    PM4125_HPH_L = 0,
    PM4125_HPH_R = 1,
    PM4125_COMP_L = 2,
    PM4125_COMP_R = 3,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
