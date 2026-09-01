// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2023-2024 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright, 2025 Linaro Ltd

// C dependencies: linux/component.h, linux/device.h, linux/irq.h,
// linux/irqdomain.h, linux/kernel.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/pm_runtime.h, linux/regmap.h, linux/slab.h,
// linux/soundwire/sdw.h, linux/soundwire/sdw_registers.h,
// linux/soundwire/sdw_type.h, sound/soc-dapm.h, sound/soc.h, "pm4125.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8_ = u8;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SDW_DPN_SIMPLE: c_uint = 0;
const SDW_DATA_DIR_TX: c_uint = 0;
const SDW_DATA_DIR_RX: c_uint = 1;
const SDW_STREAM_PCM: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SDW_SCP_INT1_IMPL_DEF: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;

const PM4125_HPH_L: c_uint = 0;
const PM4125_HPH_R: c_uint = 1;
const PM4125_HPH_PORT: c_uint = 1;
const PM4125_ADC1: c_uint = 0;
const PM4125_ADC2: c_uint = 1;
const PM4125_ADC_1_2_DMIC1L_BCS_PORT: c_uint = 1;
const PM4125_MAX_SWR_PORTS: usize = 2;
const PM4125_MAX_TX_SWR_PORTS: usize = 2;
const PM4125_MAX_SWR_CH_IDS: usize = 8;
const PM4125_MAX_REGISTER: c_uint = 0;

const PM4125_ANA_MICBIAS_MICB_1_2_EN: c_uint = 0;
const PM4125_ANA_MICBIAS_MICB_3_EN: c_uint = 1;
const PM4125_ANA_MICBIAS_LDO_1_SETTING: c_uint = 2;
const PM4125_ANA_MICBIAS_LDO_1_CTRL: c_uint = 3;
const PM4125_ANA_TX_AMIC1: c_uint = 4;
const PM4125_ANA_TX_AMIC2: c_uint = 5;
const PM4125_ANA_MBHC_MECH: c_uint = 6;
const PM4125_ANA_MBHC_ELECT: c_uint = 7;
const PM4125_ANA_MBHC_ZDET: c_uint = 8;
const PM4125_ANA_MBHC_RESULT_1: c_uint = 9;
const PM4125_ANA_MBHC_RESULT_2: c_uint = 10;
const PM4125_ANA_MBHC_RESULT_3: c_uint = 11;
const PM4125_ANA_MBHC_BTN0_ZDET_VREF1: c_uint = 12;
const PM4125_ANA_MBHC_BTN1_ZDET_VREF2: c_uint = 13;
const PM4125_ANA_MBHC_BTN2_ZDET_VREF3: c_uint = 14;
const PM4125_ANA_MBHC_BTN3_ZDET_DBG_400: c_uint = 15;
const PM4125_ANA_MBHC_BTN4_ZDET_DBG_1400: c_uint = 16;
const PM4125_ANA_MBHC_MICB2_RAMP: c_uint = 17;
const PM4125_ANA_MBHC_CTL_1: c_uint = 18;
const PM4125_ANA_MBHC_CTL_2: c_uint = 19;
const PM4125_ANA_MBHC_PLUG_DETECT_CTL: c_uint = 20;
const PM4125_ANA_MBHC_ZDET_ANA_CTL: c_uint = 21;
const PM4125_ANA_MBHC_ZDET_RAMP_CTL: c_uint = 22;
const PM4125_ANA_MBHC_FSM_STATUS: c_uint = 23;
const PM4125_ANA_MBHC_ADC_RESULT: c_uint = 24;
const PM4125_ANA_MBHC_CTL_CLK: c_uint = 25;
const PM4125_ANA_MBHC_ZDET_CALIB_RESULT: c_uint = 26;
const PM4125_ANA_NCP_EN: c_uint = 27;
const PM4125_ANA_NCP_VCTRL: c_uint = 28;
const PM4125_ANA_HPHPA_CNP_CTL_1: c_uint = 29;
const PM4125_ANA_HPHPA_CNP_CTL_2: c_uint = 30;
const PM4125_ANA_HPHPA_PA_STATUS: c_uint = 31;
const PM4125_ANA_HPHPA_FSM_CLK: c_uint = 32;
const PM4125_ANA_HPHPA_L_GAIN: c_uint = 33;
const PM4125_ANA_HPHPA_R_GAIN: c_uint = 34;
const PM4125_ANA_HPHPA_SPARE_CTL: c_uint = 35;
const PM4125_SWR_HPHPA_HD2: c_uint = 36;
const PM4125_ANA_SURGE_EN: c_uint = 37;
const PM4125_ANA_COMBOPA_CTL: c_uint = 38;
const PM4125_ANA_COMBOPA_CTL_4: c_uint = 39;
const PM4125_ANA_COMBOPA_CTL_5: c_uint = 40;
const PM4125_ANA_RXLDO_CTL: c_uint = 41;
const PM4125_ANA_MBIAS_EN: c_uint = 42;
const PM4125_DIG_SWR_CHIP_ID0: c_uint = 43;
const PM4125_DIG_SWR_CHIP_ID1: c_uint = 44;
const PM4125_DIG_SWR_CHIP_ID2: c_uint = 45;
const PM4125_DIG_SWR_CHIP_ID3: c_uint = 46;
const PM4125_DIG_SWR_SWR_TX_CLK_RATE: c_uint = 47;
const PM4125_DIG_SWR_CDC_RST_CTL: c_uint = 48;
const PM4125_DIG_SWR_TOP_CLK_CFG: c_uint = 49;
const PM4125_DIG_SWR_CDC_RX_CLK_CTL: c_uint = 50;
const PM4125_DIG_SWR_CDC_TX_CLK_CTL: c_uint = 51;
const PM4125_DIG_SWR_SWR_RST_EN: c_uint = 52;
const PM4125_DIG_SWR_CDC_RX_RST: c_uint = 53;
const PM4125_DIG_SWR_CDC_RX0_CTL: c_uint = 54;
const PM4125_DIG_SWR_CDC_RX1_CTL: c_uint = 55;
const PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1: c_uint = 56;
const PM4125_DIG_SWR_CDC_COMP_CTL_0: c_uint = 57;
const PM4125_DIG_SWR_CDC_RX_DELAY_CTL: c_uint = 58;
const PM4125_DIG_SWR_CDC_RX_GAIN_0: c_uint = 59;
const PM4125_DIG_SWR_CDC_RX_GAIN_1: c_uint = 60;
const PM4125_DIG_SWR_CDC_RX_GAIN_CTL: c_uint = 61;
const PM4125_DIG_SWR_CDC_TX0_CTL: c_uint = 62;
const PM4125_DIG_SWR_CDC_TX1_CTL: c_uint = 63;
const PM4125_DIG_SWR_CDC_TX_RST: c_uint = 64;
const PM4125_DIG_SWR_CDC_REQ0_CTL: c_uint = 65;
const PM4125_DIG_SWR_CDC_REQ1_CTL: c_uint = 66;
const PM4125_DIG_SWR_CDC_RST: c_uint = 67;
const PM4125_DIG_SWR_CDC_AMIC_CTL: c_uint = 68;
const PM4125_DIG_SWR_CDC_DMIC_CTL: c_uint = 69;
const PM4125_DIG_SWR_CDC_DMIC1_CTL: c_uint = 70;
const PM4125_DIG_SWR_CDC_DMIC1_RATE: c_uint = 71;
const PM4125_DIG_SWR_PDM_WD_CTL0: c_uint = 72;
const PM4125_DIG_SWR_PDM_WD_CTL1: c_uint = 73;
const PM4125_DIG_SWR_INTR_MODE: c_uint = 74;
const PM4125_DIG_SWR_INTR_MASK_0: c_uint = 75;
const PM4125_DIG_SWR_INTR_MASK_1: c_uint = 76;
const PM4125_DIG_SWR_INTR_MASK_2: c_uint = 77;
const PM4125_DIG_SWR_INTR_STATUS_0: c_uint = 78;
const PM4125_DIG_SWR_INTR_STATUS_1: c_uint = 79;
const PM4125_DIG_SWR_INTR_STATUS_2: c_uint = 80;
const PM4125_DIG_SWR_INTR_CLEAR_0: c_uint = 81;
const PM4125_DIG_SWR_INTR_CLEAR_1: c_uint = 82;
const PM4125_DIG_SWR_INTR_CLEAR_2: c_uint = 83;
const PM4125_DIG_SWR_INTR_LEVEL_0: c_uint = 84;
const PM4125_DIG_SWR_INTR_LEVEL_1: c_uint = 85;
const PM4125_DIG_SWR_INTR_LEVEL_2: c_uint = 86;
const PM4125_DIG_SWR_CDC_CONN_RX0_CTL: c_uint = 87;
const PM4125_DIG_SWR_CDC_CONN_RX1_CTL: c_uint = 88;
const PM4125_DIG_SWR_LOOP_BACK_MODE: c_uint = 89;
const PM4125_DIG_SWR_DRIVE_STRENGTH_0: c_uint = 90;
const PM4125_DIG_SWR_DIG_DEBUG_CTL: c_uint = 91;
const PM4125_DIG_SWR_DIG_DEBUG_EN: c_uint = 92;
const PM4125_DIG_SWR_DEM_BYPASS_DATA0: c_uint = 93;
const PM4125_DIG_SWR_DEM_BYPASS_DATA1: c_uint = 94;
const PM4125_DIG_SWR_DEM_BYPASS_DATA2: c_uint = 95;
const PM4125_DIG_SWR_DEM_BYPASS_DATA3: c_uint = 96;

const fn BIT(nr: c_uint) -> c_ulong {
    1_u64.wrapping_shl(nr) as c_ulong
}

const fn GENMASK(h: usize, l: usize) -> c_uint {
    ((!0_u32).wrapping_shl(l as u32) & (!0_u32).wrapping_shr((31 - h) as u32)) as c_uint
}

const fn PM4125_SWRM_CH_MASK(ch: u8_) -> c_ulong {
    BIT(ch as c_uint)
}

#[repr(C)]
pub struct device {
    pub of_node: *mut c_void,
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_sdw_ch_info {
    pub port_num: c_uint,
    pub ch_num: c_uint,
    pub ch_mask: c_ulong,
    pub master_ch_mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_ulong,
}

#[repr(C)]
pub struct sdw_stream_config {
    pub ch_count: c_uint,
    pub bps: c_uint,
    pub frame_rate: c_uint,
    pub direction: c_uint,
    pub type_: c_uint,
}

#[repr(C)]
pub struct pm4125_sdw_priv {
    pub sconfig: sdw_stream_config,
    pub active_ports: c_int,
    pub port_config: [sdw_port_config; PM4125_MAX_SWR_PORTS],
    pub is_tx: bool_,
    pub sdev: *mut sdw_slave,
    pub sruntime: *mut c_void,
    pub slave_irq: *mut c_void,
    pub ch_info: *mut wcd_sdw_ch_info,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub min_ch: c_uint,
    pub max_ch: c_uint,
    pub simple_ch_prep_sm: bool_,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub lane_control_support: bool_,
    pub simple_clk_stop_capable: bool_,
    pub source_ports: c_uint,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_ports: c_uint,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub wake_capable: bool_,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub m_port_map: [c_uint; PM4125_MAX_SWR_PORTS + 1],
    pub prop: sdw_slave_prop,
}

#[repr(C)]
pub struct sdw_slave_intr_status {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub update_status: Option<unsafe extern "C" fn()>,
    pub interrupt_callback:
        Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
}

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub class_id: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
    pub driver: driver_inner,
}

unsafe extern "C" {
    static wcd_sdw_component_ops: c_void;
    static wcd_update_status: unsafe extern "C" fn();

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn sdw_stream_add_slave(
        sdev: *mut sdw_slave,
        sconfig: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
        num_ports: c_int,
        sruntime: *mut c_void,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn wcd_interrupt_callback(
        slave: *mut sdw_slave,
        slave_irq: *mut c_void,
        status0: c_uint,
        status1: c_uint,
        status2: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_present(np: *mut c_void, propname: *const c_char) -> bool_;
    fn of_property_read_u32_array(
        np: *mut c_void,
        propname: *const c_char,
        out_values: *mut c_uint,
        sz: usize,
    ) -> c_int;
    fn of_property_count_u8_elems(np: *mut c_void, propname: *const c_char) -> c_int;
    fn of_property_read_u8_array(
        np: *mut c_void,
        propname: *const c_char,
        out_values: *mut u8_,
        sz: usize,
    ) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_regmap_init_sdw(pdev: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn component_add(dev: *mut device, ops: *const c_void) -> c_int;
    fn component_del(dev: *mut device, ops: *const c_void);
    fn pm_runtime_set_suspended(dev: *mut device);
}

const fn WCD_SDW_CH(ch_num: c_uint, port_num: c_uint, ch_mask: c_ulong) -> wcd_sdw_ch_info {
    wcd_sdw_ch_info {
        port_num,
        ch_num,
        ch_mask,
        master_ch_mask: 0,
    }
}

static mut pm4125_sdw_rx_ch_info: [wcd_sdw_ch_info; 2] = [
    WCD_SDW_CH(PM4125_HPH_L, PM4125_HPH_PORT, BIT(0)),
    WCD_SDW_CH(PM4125_HPH_R, PM4125_HPH_PORT, BIT(1)),
];

static mut pm4125_sdw_tx_ch_info: [wcd_sdw_ch_info; 2] = [
    WCD_SDW_CH(PM4125_ADC1, PM4125_ADC_1_2_DMIC1L_BCS_PORT, BIT(0)),
    WCD_SDW_CH(PM4125_ADC2, PM4125_ADC_1_2_DMIC1L_BCS_PORT, BIT(1)),
];

static mut pm4125_dpn_prop: [sdw_dpn_prop; PM4125_MAX_SWR_PORTS] = [
    sdw_dpn_prop {
        num: 1,
        type_: SDW_DPN_SIMPLE,
        min_ch: 1,
        max_ch: 8,
        simple_ch_prep_sm: true,
    },
    sdw_dpn_prop {
        num: 2,
        type_: SDW_DPN_SIMPLE,
        min_ch: 1,
        max_ch: 4,
        simple_ch_prep_sm: true,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pm4125_sdw_hw_params(
    priv_: *mut pm4125_sdw_priv,
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let mut port_config: [sdw_port_config; PM4125_MAX_SWR_PORTS] =
        core::mem::zeroed();
    let mut ch_mask: c_ulong;
    let mut i: c_int;
    let mut j: c_int;

    (*priv_).sconfig.ch_count = 1;
    (*priv_).active_ports = 0;
    i = 0;
    while i < PM4125_MAX_SWR_PORTS as c_int {
        ch_mask = (*priv_).port_config[i as usize].ch_mask;
        if ch_mask == 0 {
            i += 1;
            continue;
        }

        j = 0;
        while j < 4 {
            if (ch_mask & BIT(j as c_uint)) != 0 {
                (*priv_).sconfig.ch_count = (*priv_).sconfig.ch_count.wrapping_add(1);
            }
            j += 1;
        }

        port_config[(*priv_).active_ports as usize] = (*priv_).port_config[i as usize];
        (*priv_).active_ports += 1;
        i += 1;
    }

    (*priv_).sconfig.bps = 1;
    (*priv_).sconfig.frame_rate = params_rate(params);
    (*priv_).sconfig.direction = if (*priv_).is_tx {
        SDW_DATA_DIR_TX
    } else {
        SDW_DATA_DIR_RX
    };
    (*priv_).sconfig.type_ = SDW_STREAM_PCM;

    sdw_stream_add_slave(
        (*priv_).sdev,
        &mut (*priv_).sconfig,
        &mut port_config[0],
        (*priv_).active_ports,
        (*priv_).sruntime,
    )
}

/*
 * Handle Soundwire out-of-band interrupt event by triggering the first irq of the slave_irq
 * irq domain, which then will be handled by the regmap_irq threaded irq.
 * Looping is to ensure no interrupts were missed in the process.
 */
unsafe extern "C" fn pm4125_interrupt_callback(
    slave: *mut sdw_slave,
    _status: *mut sdw_slave_intr_status,
) -> c_int {
    let priv_ = dev_get_drvdata(&mut (*slave).dev) as *mut pm4125_sdw_priv;

    wcd_interrupt_callback(
        slave,
        (*priv_).slave_irq,
        PM4125_DIG_SWR_INTR_STATUS_0,
        PM4125_DIG_SWR_INTR_STATUS_1,
        PM4125_DIG_SWR_INTR_STATUS_2,
    )
}

static pm4125_defaults: [reg_default; 98] = [
    reg_default { reg: PM4125_ANA_MICBIAS_MICB_1_2_EN, def: 0x01 },
    reg_default { reg: PM4125_ANA_MICBIAS_MICB_3_EN, def: 0x00 },
    reg_default { reg: PM4125_ANA_MICBIAS_LDO_1_SETTING, def: 0x21 },
    reg_default { reg: PM4125_ANA_MICBIAS_LDO_1_CTRL, def: 0x01 },
    reg_default { reg: PM4125_ANA_TX_AMIC1, def: 0x00 },
    reg_default { reg: PM4125_ANA_TX_AMIC2, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_MECH, def: 0x39 },
    reg_default { reg: PM4125_ANA_MBHC_ELECT, def: 0x08 },
    reg_default { reg: PM4125_ANA_MBHC_ZDET, def: 0x10 },
    reg_default { reg: PM4125_ANA_MBHC_RESULT_1, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_RESULT_2, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_RESULT_3, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_BTN0_ZDET_VREF1, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_BTN1_ZDET_VREF2, def: 0x10 },
    reg_default { reg: PM4125_ANA_MBHC_BTN2_ZDET_VREF3, def: 0x20 },
    reg_default { reg: PM4125_ANA_MBHC_BTN3_ZDET_DBG_400, def: 0x30 },
    reg_default { reg: PM4125_ANA_MBHC_BTN4_ZDET_DBG_1400, def: 0x40 },
    reg_default { reg: PM4125_ANA_MBHC_MICB2_RAMP, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_CTL_1, def: 0x02 },
    reg_default { reg: PM4125_ANA_MBHC_CTL_2, def: 0x05 },
    reg_default { reg: PM4125_ANA_MBHC_PLUG_DETECT_CTL, def: 0xE9 },
    reg_default { reg: PM4125_ANA_MBHC_ZDET_ANA_CTL, def: 0x0F },
    reg_default { reg: PM4125_ANA_MBHC_ZDET_RAMP_CTL, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_FSM_STATUS, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_ADC_RESULT, def: 0x00 },
    reg_default { reg: PM4125_ANA_MBHC_CTL_CLK, def: 0x30 },
    reg_default { reg: PM4125_ANA_MBHC_ZDET_CALIB_RESULT, def: 0x00 },
    reg_default { reg: PM4125_ANA_NCP_EN, def: 0x00 },
    reg_default { reg: PM4125_ANA_NCP_VCTRL, def: 0xA7 },
    reg_default { reg: PM4125_ANA_HPHPA_CNP_CTL_1, def: 0x54 },
    reg_default { reg: PM4125_ANA_HPHPA_CNP_CTL_2, def: 0x2B },
    reg_default { reg: PM4125_ANA_HPHPA_PA_STATUS, def: 0x00 },
    reg_default { reg: PM4125_ANA_HPHPA_FSM_CLK, def: 0x12 },
    reg_default { reg: PM4125_ANA_HPHPA_L_GAIN, def: 0x00 },
    reg_default { reg: PM4125_ANA_HPHPA_R_GAIN, def: 0x00 },
    reg_default { reg: PM4125_ANA_HPHPA_SPARE_CTL, def: 0x02 },
    reg_default { reg: PM4125_SWR_HPHPA_HD2, def: 0x1B },
    reg_default { reg: PM4125_ANA_SURGE_EN, def: 0x38 },
    reg_default { reg: PM4125_ANA_COMBOPA_CTL, def: 0x35 },
    reg_default { reg: PM4125_ANA_COMBOPA_CTL_4, def: 0x84 },
    reg_default { reg: PM4125_ANA_COMBOPA_CTL_5, def: 0x05 },
    reg_default { reg: PM4125_ANA_RXLDO_CTL, def: 0x86 },
    reg_default { reg: PM4125_ANA_MBIAS_EN, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CHIP_ID0, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CHIP_ID1, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CHIP_ID2, def: 0x0C },
    reg_default { reg: PM4125_DIG_SWR_CHIP_ID3, def: 0x01 },
    reg_default { reg: PM4125_DIG_SWR_SWR_TX_CLK_RATE, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RST_CTL, def: 0x03 },
    reg_default { reg: PM4125_DIG_SWR_TOP_CLK_CFG, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX_CLK_CTL, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_TX_CLK_CTL, def: 0x33 },
    reg_default { reg: PM4125_DIG_SWR_SWR_RST_EN, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX_RST, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX0_CTL, def: 0xFC },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX1_CTL, def: 0xFC },
    reg_default { reg: PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_COMP_CTL_0, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX_DELAY_CTL, def: 0x66 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX_GAIN_0, def: 0x55 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX_GAIN_1, def: 0xA9 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RX_GAIN_CTL, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_TX0_CTL, def: 0x68 },
    reg_default { reg: PM4125_DIG_SWR_CDC_TX1_CTL, def: 0x68 },
    reg_default { reg: PM4125_DIG_SWR_CDC_TX_RST, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_REQ0_CTL, def: 0x01 },
    reg_default { reg: PM4125_DIG_SWR_CDC_REQ1_CTL, def: 0x01 },
    reg_default { reg: PM4125_DIG_SWR_CDC_RST, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_AMIC_CTL, def: 0x02 },
    reg_default { reg: PM4125_DIG_SWR_CDC_DMIC_CTL, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_DMIC1_CTL, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_DMIC1_RATE, def: 0x01 },
    reg_default { reg: PM4125_DIG_SWR_PDM_WD_CTL0, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_PDM_WD_CTL1, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_MODE, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_MASK_0, def: 0xFF },
    reg_default { reg: PM4125_DIG_SWR_INTR_MASK_1, def: 0x7F },
    reg_default { reg: PM4125_DIG_SWR_INTR_MASK_2, def: 0x0C },
    reg_default { reg: PM4125_DIG_SWR_INTR_STATUS_0, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_STATUS_1, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_STATUS_2, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_CLEAR_0, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_CLEAR_1, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_CLEAR_2, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_LEVEL_0, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_INTR_LEVEL_1, def: 0x2A },
    reg_default { reg: PM4125_DIG_SWR_INTR_LEVEL_2, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_CONN_RX0_CTL, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_CDC_CONN_RX1_CTL, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_LOOP_BACK_MODE, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_DRIVE_STRENGTH_0, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_DIG_DEBUG_CTL, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_DIG_DEBUG_EN, def: 0x00 },
    reg_default { reg: PM4125_DIG_SWR_DEM_BYPASS_DATA0, def: 0x55 },
    reg_default { reg: PM4125_DIG_SWR_DEM_BYPASS_DATA1, def: 0x55 },
    reg_default { reg: PM4125_DIG_SWR_DEM_BYPASS_DATA2, def: 0x55 },
    reg_default { reg: PM4125_DIG_SWR_DEM_BYPASS_DATA3, def: 0x01 },
];

unsafe extern "C" fn pm4125_rdwr_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        PM4125_ANA_MICBIAS_MICB_1_2_EN
        | PM4125_ANA_MICBIAS_MICB_3_EN
        | PM4125_ANA_MICBIAS_LDO_1_SETTING
        | PM4125_ANA_MICBIAS_LDO_1_CTRL
        | PM4125_ANA_TX_AMIC1
        | PM4125_ANA_TX_AMIC2
        | PM4125_ANA_MBHC_MECH
        | PM4125_ANA_MBHC_ELECT
        | PM4125_ANA_MBHC_ZDET
        | PM4125_ANA_MBHC_BTN0_ZDET_VREF1
        | PM4125_ANA_MBHC_BTN1_ZDET_VREF2
        | PM4125_ANA_MBHC_BTN2_ZDET_VREF3
        | PM4125_ANA_MBHC_BTN3_ZDET_DBG_400
        | PM4125_ANA_MBHC_BTN4_ZDET_DBG_1400
        | PM4125_ANA_MBHC_MICB2_RAMP
        | PM4125_ANA_MBHC_CTL_1
        | PM4125_ANA_MBHC_CTL_2
        | PM4125_ANA_MBHC_PLUG_DETECT_CTL
        | PM4125_ANA_MBHC_ZDET_ANA_CTL
        | PM4125_ANA_MBHC_ZDET_RAMP_CTL
        | PM4125_ANA_MBHC_CTL_CLK
        | PM4125_ANA_NCP_EN
        | PM4125_ANA_NCP_VCTRL
        | PM4125_ANA_HPHPA_CNP_CTL_1
        | PM4125_ANA_HPHPA_CNP_CTL_2
        | PM4125_ANA_HPHPA_FSM_CLK
        | PM4125_ANA_HPHPA_L_GAIN
        | PM4125_ANA_HPHPA_R_GAIN
        | PM4125_ANA_HPHPA_SPARE_CTL
        | PM4125_SWR_HPHPA_HD2
        | PM4125_ANA_SURGE_EN
        | PM4125_ANA_COMBOPA_CTL
        | PM4125_ANA_COMBOPA_CTL_4
        | PM4125_ANA_COMBOPA_CTL_5
        | PM4125_ANA_RXLDO_CTL
        | PM4125_ANA_MBIAS_EN
        | PM4125_DIG_SWR_SWR_TX_CLK_RATE
        | PM4125_DIG_SWR_CDC_RST_CTL
        | PM4125_DIG_SWR_TOP_CLK_CFG
        | PM4125_DIG_SWR_CDC_RX_CLK_CTL
        | PM4125_DIG_SWR_CDC_TX_CLK_CTL
        | PM4125_DIG_SWR_SWR_RST_EN
        | PM4125_DIG_SWR_CDC_RX_RST
        | PM4125_DIG_SWR_CDC_RX0_CTL
        | PM4125_DIG_SWR_CDC_RX1_CTL
        | PM4125_DIG_SWR_CDC_TX_ANA_MODE_0_1
        | PM4125_DIG_SWR_CDC_COMP_CTL_0
        | PM4125_DIG_SWR_CDC_RX_DELAY_CTL
        | PM4125_DIG_SWR_CDC_RX_GAIN_0
        | PM4125_DIG_SWR_CDC_RX_GAIN_1
        | PM4125_DIG_SWR_CDC_RX_GAIN_CTL
        | PM4125_DIG_SWR_CDC_TX0_CTL
        | PM4125_DIG_SWR_CDC_TX1_CTL
        | PM4125_DIG_SWR_CDC_TX_RST
        | PM4125_DIG_SWR_CDC_REQ0_CTL
        | PM4125_DIG_SWR_CDC_REQ1_CTL
        | PM4125_DIG_SWR_CDC_RST
        | PM4125_DIG_SWR_CDC_AMIC_CTL
        | PM4125_DIG_SWR_CDC_DMIC_CTL
        | PM4125_DIG_SWR_CDC_DMIC1_CTL
        | PM4125_DIG_SWR_CDC_DMIC1_RATE
        | PM4125_DIG_SWR_PDM_WD_CTL0
        | PM4125_DIG_SWR_PDM_WD_CTL1
        | PM4125_DIG_SWR_INTR_MODE
        | PM4125_DIG_SWR_INTR_MASK_0
        | PM4125_DIG_SWR_INTR_MASK_1
        | PM4125_DIG_SWR_INTR_MASK_2
        | PM4125_DIG_SWR_INTR_CLEAR_0
        | PM4125_DIG_SWR_INTR_CLEAR_1
        | PM4125_DIG_SWR_INTR_CLEAR_2
        | PM4125_DIG_SWR_INTR_LEVEL_0
        | PM4125_DIG_SWR_INTR_LEVEL_1
        | PM4125_DIG_SWR_INTR_LEVEL_2
        | PM4125_DIG_SWR_CDC_CONN_RX0_CTL
        | PM4125_DIG_SWR_CDC_CONN_RX1_CTL
        | PM4125_DIG_SWR_LOOP_BACK_MODE
        | PM4125_DIG_SWR_DRIVE_STRENGTH_0
        | PM4125_DIG_SWR_DIG_DEBUG_CTL
        | PM4125_DIG_SWR_DIG_DEBUG_EN
        | PM4125_DIG_SWR_DEM_BYPASS_DATA0
        | PM4125_DIG_SWR_DEM_BYPASS_DATA1
        | PM4125_DIG_SWR_DEM_BYPASS_DATA2
        | PM4125_DIG_SWR_DEM_BYPASS_DATA3 => true,
        _ => false,
    }
}

unsafe extern "C" fn pm4125_readable_register(dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        PM4125_ANA_MBHC_RESULT_1
        | PM4125_ANA_MBHC_RESULT_2
        | PM4125_ANA_MBHC_RESULT_3
        | PM4125_ANA_MBHC_FSM_STATUS
        | PM4125_ANA_MBHC_ADC_RESULT
        | PM4125_ANA_MBHC_ZDET_CALIB_RESULT
        | PM4125_ANA_HPHPA_PA_STATUS
        | PM4125_DIG_SWR_CHIP_ID0
        | PM4125_DIG_SWR_CHIP_ID1
        | PM4125_DIG_SWR_CHIP_ID2
        | PM4125_DIG_SWR_CHIP_ID3
        | PM4125_DIG_SWR_INTR_STATUS_0
        | PM4125_DIG_SWR_INTR_STATUS_1
        | PM4125_DIG_SWR_INTR_STATUS_2 => true,
        _ => pm4125_rdwr_register(dev, reg),
    }
}

unsafe extern "C" fn pm4125_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        PM4125_ANA_MBHC_RESULT_1
        | PM4125_ANA_MBHC_RESULT_2
        | PM4125_ANA_MBHC_RESULT_3
        | PM4125_ANA_MBHC_FSM_STATUS
        | PM4125_ANA_MBHC_ADC_RESULT
        | PM4125_ANA_MBHC_ZDET_CALIB_RESULT
        | PM4125_ANA_HPHPA_PA_STATUS
        | PM4125_DIG_SWR_CHIP_ID0
        | PM4125_DIG_SWR_CHIP_ID1
        | PM4125_DIG_SWR_CHIP_ID2
        | PM4125_DIG_SWR_CHIP_ID3
        | PM4125_DIG_SWR_INTR_STATUS_0
        | PM4125_DIG_SWR_INTR_STATUS_1
        | PM4125_DIG_SWR_INTR_STATUS_2 => true,
        _ => false,
    }
}

static pm4125_regmap_config: regmap_config = regmap_config {
    name: b"pm4125_csr\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 8,
    cache_type: REGCACHE_MAPLE,
    reg_defaults: pm4125_defaults.as_ptr(),
    num_reg_defaults: pm4125_defaults.len() as c_uint,
    max_register: PM4125_MAX_REGISTER,
    readable_reg: Some(pm4125_readable_register),
    writeable_reg: Some(pm4125_rdwr_register),
    volatile_reg: Some(pm4125_volatile_register),
};

static pm4125_slave_ops: sdw_slave_ops = sdw_slave_ops {
    update_status: Some(unsafe { wcd_update_status }),
    interrupt_callback: Some(pm4125_interrupt_callback),
};

unsafe extern "C" fn pm4125_probe(
    pdev: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut priv_: *mut pm4125_sdw_priv;
    let mut master_ch_mask: [u8_; PM4125_MAX_SWR_CH_IDS] = [0; PM4125_MAX_SWR_CH_IDS];
    let mut master_ch_mask_size: c_int = 0;
    let mut ret: c_int;
    let mut i: c_int;

    priv_ = devm_kzalloc(dev, size_of::<pm4125_sdw_priv>(), GFP_KERNEL) as *mut pm4125_sdw_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    /* Port map index starts at 0, however the data port for this codec starts at index 1 */
    if of_property_present((*dev).of_node, b"qcom,tx-port-mapping\0".as_ptr() as *const c_char) {
        (*priv_).is_tx = true;
        ret = of_property_read_u32_array(
            (*dev).of_node,
            b"qcom,tx-port-mapping\0".as_ptr() as *const c_char,
            &mut (*pdev).m_port_map[1],
            PM4125_MAX_TX_SWR_PORTS,
        );
    } else {
        ret = of_property_read_u32_array(
            (*dev).of_node,
            b"qcom,rx-port-mapping\0".as_ptr() as *const c_char,
            &mut (*pdev).m_port_map[1],
            PM4125_MAX_SWR_PORTS,
        );
    }

    if ret < 0 {
        dev_info(
            dev,
            b"Error getting static port mapping for %s (%d)\n\0".as_ptr() as *const c_char,
            if (*priv_).is_tx {
                b"TX\0".as_ptr()
            } else {
                b"RX\0".as_ptr()
            },
            ret,
        );
    }

    (*priv_).sdev = pdev;
    dev_set_drvdata(dev, priv_ as *mut c_void);

    (*pdev).prop.scp_int1_mask =
        SDW_SCP_INT1_IMPL_DEF | SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*pdev).prop.lane_control_support = true;
    (*pdev).prop.simple_clk_stop_capable = true;

    master_ch_mask = [0; PM4125_MAX_SWR_CH_IDS];

    if (*priv_).is_tx {
        master_ch_mask_size = of_property_count_u8_elems(
            (*dev).of_node,
            b"qcom,tx-channel-mapping\0".as_ptr() as *const c_char,
        );

        if master_ch_mask_size != 0 {
            ret = of_property_read_u8_array(
                (*dev).of_node,
                b"qcom,tx-channel-mapping\0".as_ptr() as *const c_char,
                master_ch_mask.as_mut_ptr(),
                master_ch_mask_size as usize,
            );
        }
    } else {
        master_ch_mask_size = of_property_count_u8_elems(
            (*dev).of_node,
            b"qcom,rx-channel-mapping\0".as_ptr() as *const c_char,
        );

        if master_ch_mask_size != 0 {
            ret = of_property_read_u8_array(
                (*dev).of_node,
                b"qcom,rx-channel-mapping\0".as_ptr() as *const c_char,
                master_ch_mask.as_mut_ptr(),
                master_ch_mask_size as usize,
            );
        }
    }

    if ret < 0 {
        dev_info(
            dev,
            b"Static channel mapping not specified using device channel maps\n\0".as_ptr()
                as *const c_char,
        );
    }

    if (*priv_).is_tx {
        (*pdev).prop.source_ports = GENMASK(PM4125_MAX_TX_SWR_PORTS, 0);
        (*pdev).prop.src_dpn_prop = pm4125_dpn_prop.as_mut_ptr();
        (*priv_).ch_info = pm4125_sdw_tx_ch_info.as_mut_ptr();

        i = 0;
        while i < master_ch_mask_size {
            (*(*priv_).ch_info.add(i as usize)).master_ch_mask =
                PM4125_SWRM_CH_MASK(master_ch_mask[i as usize]);
            i += 1;
        }

        (*pdev).prop.wake_capable = true;

        (*priv_).regmap = devm_regmap_init_sdw(pdev, &pm4125_regmap_config);
        if IS_ERR((*priv_).regmap as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*priv_).regmap as *const c_void),
                b"regmap init failed\n\0".as_ptr() as *const c_char,
            );
        }

        /* Start in cache-only until device is enumerated */
        regcache_cache_only((*priv_).regmap, true);
    } else {
        (*pdev).prop.sink_ports = GENMASK(PM4125_MAX_SWR_PORTS - 1, 0);
        (*pdev).prop.sink_dpn_prop = pm4125_dpn_prop.as_mut_ptr();
        (*priv_).ch_info = pm4125_sdw_rx_ch_info.as_mut_ptr();

        i = 0;
        while i < master_ch_mask_size {
            (*(*priv_).ch_info.add(i as usize)).master_ch_mask =
                PM4125_SWRM_CH_MASK(master_ch_mask[i as usize]);
            i += 1;
        }
    }

    ret = component_add(dev, &wcd_sdw_component_ops);
    if ret != 0 {
        return ret;
    }

    /* Set suspended until aggregate device is bind */
    pm_runtime_set_suspended(dev);

    0
}

unsafe extern "C" fn pm4125_remove(pdev: *mut sdw_slave) {
    let dev: *mut device = &mut (*pdev).dev;

    component_del(dev, &wcd_sdw_component_ops);
}

static pm4125_slave_id: [sdw_device_id; 2] = [
    sdw_device_id {
        mfg_id: 0x0217,
        part_id: 0x10c,
        class_id: 0,
    }, /* Soundwire pm4125 RX/TX Device ID */
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        class_id: 0,
    },
];
// MODULE_DEVICE_TABLE(sdw, pm4125_slave_id);

unsafe extern "C" fn pm4125_sdw_runtime_suspend(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut pm4125_sdw_priv;

    if !(*priv_).regmap.is_null() {
        regcache_cache_only((*priv_).regmap, true);
        regcache_mark_dirty((*priv_).regmap);
    }

    0
}

unsafe extern "C" fn pm4125_sdw_runtime_resume(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut pm4125_sdw_priv;
    let ret: c_int;

    if !(*priv_).regmap.is_null() {
        regcache_cache_only((*priv_).regmap, false);
        ret = regcache_sync((*priv_).regmap);
        if ret != 0 {
            regcache_cache_only((*priv_).regmap, true);
            regcache_mark_dirty((*priv_).regmap);
            return ret;
        }
    }

    0
}

static pm4125_sdw_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(pm4125_sdw_runtime_suspend),
    runtime_resume: Some(pm4125_sdw_runtime_resume),
    runtime_idle: None,
};

static mut pm4125_codec_driver: sdw_driver = sdw_driver {
    probe: Some(pm4125_probe),
    remove: Some(pm4125_remove),
    ops: &pm4125_slave_ops,
    id_table: pm4125_slave_id.as_ptr(),
    driver: driver_inner {
        name: b"pm4125-codec\0".as_ptr() as *const c_char,
        pm: &pm4125_sdw_pm_ops,
    },
};
// module_sdw_driver(pm4125_codec_driver);

// MODULE_DESCRIPTION("PM4125 SDW codec driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
