// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2023-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//
// Rust source-level translation of soc/codecs/wcd937x-sdw.c.
// C include dependencies intentionally remain external to this isolated file:
// linux/component.h, linux/device.h, linux/irq.h, linux/irqdomain.h,
// linux/kernel.h, linux/module.h, linux/of.h, linux/platform_device.h,
// linux/pm_runtime.h, linux/regmap.h, linux/slab.h,
// linux/soundwire/sdw.h, linux/soundwire/sdw_registers.h,
// linux/soundwire/sdw_type.h, sound/soc-dapm.h, sound/soc.h, "wcd937x.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type bool_ = bool;
type u8 = u8;

#[repr(C)]
pub struct device {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct sdw_stream_runtime {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave_intr_status {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct sdw_device_id {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct component_ops {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct wcd_sdw_ch_info {
    pub ch_id: c_uint,
    pub port_num: c_uint,
    pub ch_mask: c_ulong,
    pub master_ch_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_stream_config {
    pub ch_count: c_uint,
    pub bps: c_uint,
    pub frame_rate: c_uint,
    pub direction: c_uint,
    pub type_: c_uint,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub min_ch: c_uint,
    pub max_ch: c_uint,
    pub simple_ch_prep_sm: bool,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub lane_control_support: bool,
    pub simple_clk_stop_capable: bool,
    pub source_ports: c_uint,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub wake_capable: bool,
    pub sink_ports: c_uint,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub m_port_map: [c_uint; WCD937X_MAX_SWR_PORTS + 1],
}

#[repr(C)]
pub struct wcd937x_sdw_priv {
    pub sconfig: sdw_stream_config,
    pub active_ports: c_int,
    pub port_config: [sdw_port_config; WCD937X_MAX_SWR_PORTS],
    pub is_tx: bool,
    pub sdev: *mut sdw_slave,
    pub sruntime: *mut sdw_stream_runtime,
    pub slave_irq: *mut irq_domain,
    pub ch_info: *mut wcd_sdw_ch_info,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub update_status: Option<unsafe extern "C" fn()>,
    pub interrupt_callback:
        Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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

extern "C" {
    static wcd_sdw_component_ops: component_ops;
    static wcd_update_status: unsafe extern "C" fn();

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn sdw_stream_add_slave(
        slave: *mut sdw_slave,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
        num_ports: c_int,
        stream: *mut sdw_stream_runtime,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn wcd_interrupt_callback(
        slave: *mut sdw_slave,
        slave_irq: *mut irq_domain,
        status0: c_uint,
        status1: c_uint,
        status2: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32_array(
        np: *mut device_node,
        propname: *const c_char,
        out_values: *mut c_uint,
        sz: usize,
    ) -> c_int;
    fn of_property_count_u8_elems(np: *mut device_node, propname: *const c_char) -> c_int;
    fn of_property_read_u8_array(
        np: *mut device_node,
        propname: *const c_char,
        out_values: *mut u8,
        sz: usize,
    ) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn component_add(dev: *mut device, ops: *const component_ops) -> c_int;
    fn component_del(dev: *mut device, ops: *const component_ops);
    fn pm_runtime_set_suspended(dev: *mut device);
}

extern "C" {
    static mut WCD937X_HPH_L: c_uint;
    static mut WCD937X_HPH_R: c_uint;
    static mut WCD937X_CLSH: c_uint;
    static mut WCD937X_COMP_L: c_uint;
    static mut WCD937X_COMP_R: c_uint;
    static mut WCD937X_LO: c_uint;
    static mut WCD937X_DSD_L: c_uint;
    static mut WCD937X_DSD_R: c_uint;
    static mut WCD937X_ADC1: c_uint;
    static mut WCD937X_ADC2: c_uint;
    static mut WCD937X_ADC3: c_uint;
    static mut WCD937X_DMIC0: c_uint;
    static mut WCD937X_DMIC1: c_uint;
    static mut WCD937X_MBHC: c_uint;
    static mut WCD937X_DMIC2: c_uint;
    static mut WCD937X_DMIC3: c_uint;
    static mut WCD937X_DMIC4: c_uint;
    static mut WCD937X_DMIC5: c_uint;
    static mut WCD937X_DMIC6: c_uint;
}

const WCD937X_MAX_SWR_PORTS: usize = 5;
const WCD937X_MAX_TX_SWR_PORTS: usize = 5;
const WCD937X_MAX_SWR_CH_IDS: usize = 11;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const REGCACHE_MAPLE: c_uint = 0;
const SDW_DPN_SIMPLE: c_uint = 0;
const SDW_DATA_DIR_TX: c_uint = 1;
const SDW_DATA_DIR_RX: c_uint = 0;
const SDW_STREAM_PCM: c_uint = 0;
const SDW_SCP_INT1_IMPL_DEF: c_uint = 1 << 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 1 << 1;
const SDW_SCP_INT1_PARITY: c_uint = 1 << 2;
const WCD937X_MAX_REGISTER: c_uint = 0xffff;

const WCD937X_HPH_PORT: c_uint = 1;
const WCD937X_CLSH_PORT: c_uint = 2;
const WCD937X_COMP_PORT: c_uint = 3;
const WCD937X_LO_PORT: c_uint = 4;
const WCD937X_DSD_PORT: c_uint = 5;
const WCD937X_ADC_1_PORT: c_uint = 1;
const WCD937X_ADC_2_3_PORT: c_uint = 2;
const WCD937X_DMIC_0_3_MBHC_PORT: c_uint = 3;
const WCD937X_DMIC_4_6_PORT: c_uint = 4;

const fn BIT(n: c_uint) -> c_ulong {
    1usize.wrapping_shl(n) as c_ulong
}

const fn GENMASK(h: usize, l: usize) -> c_uint {
    if h >= 31 {
        (!0u32).wrapping_shl(l as u32)
    } else {
        (((1u32 << (h + 1)) - 1) & (!0u32 << l)) as c_uint
    }
}

const fn WCD937X_SWRM_CH_MASK(ch: u8) -> c_uint {
    1u32.wrapping_shl(ch as u32)
}

const fn WCD_SDW_CH_CONST(ch_id: c_uint, port_num: c_uint, ch_mask: c_ulong) -> wcd_sdw_ch_info {
    wcd_sdw_ch_info {
        ch_id,
        port_num,
        ch_mask,
        master_ch_mask: 0,
    }
}

static mut wcd937x_sdw_rx_ch_info: [wcd_sdw_ch_info; 8] = unsafe {
    [
        WCD_SDW_CH_CONST(WCD937X_HPH_L, WCD937X_HPH_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_HPH_R, WCD937X_HPH_PORT, BIT(1)),
        WCD_SDW_CH_CONST(WCD937X_CLSH, WCD937X_CLSH_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_COMP_L, WCD937X_COMP_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_COMP_R, WCD937X_COMP_PORT, BIT(1)),
        WCD_SDW_CH_CONST(WCD937X_LO, WCD937X_LO_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_DSD_L, WCD937X_DSD_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_DSD_R, WCD937X_DSD_PORT, BIT(1)),
    ]
};

static mut wcd937x_sdw_tx_ch_info: [wcd_sdw_ch_info; 11] = unsafe {
    [
        WCD_SDW_CH_CONST(WCD937X_ADC1, WCD937X_ADC_1_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_ADC2, WCD937X_ADC_2_3_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_ADC3, WCD937X_ADC_2_3_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_DMIC0, WCD937X_DMIC_0_3_MBHC_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_DMIC1, WCD937X_DMIC_0_3_MBHC_PORT, BIT(1)),
        WCD_SDW_CH_CONST(WCD937X_MBHC, WCD937X_DMIC_0_3_MBHC_PORT, BIT(2)),
        WCD_SDW_CH_CONST(WCD937X_DMIC2, WCD937X_DMIC_0_3_MBHC_PORT, BIT(2)),
        WCD_SDW_CH_CONST(WCD937X_DMIC3, WCD937X_DMIC_0_3_MBHC_PORT, BIT(3)),
        WCD_SDW_CH_CONST(WCD937X_DMIC4, WCD937X_DMIC_4_6_PORT, BIT(0)),
        WCD_SDW_CH_CONST(WCD937X_DMIC5, WCD937X_DMIC_4_6_PORT, BIT(1)),
        WCD_SDW_CH_CONST(WCD937X_DMIC6, WCD937X_DMIC_4_6_PORT, BIT(2)),
    ]
};

static mut wcd937x_dpn_prop: [sdw_dpn_prop; WCD937X_MAX_SWR_PORTS] = [
    sdw_dpn_prop { num: 1, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 8, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 2, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 3, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 4, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 5, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
];

#[no_mangle]
pub unsafe extern "C" fn wcd937x_sdw_hw_params(
    wcd: *mut wcd937x_sdw_priv,
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let mut port_config: [sdw_port_config; WCD937X_MAX_SWR_PORTS] =
        MaybeUninit::<[sdw_port_config; WCD937X_MAX_SWR_PORTS]>::zeroed().assume_init();
    let mut ch_mask: c_ulong;
    let mut i: usize;
    let mut j: c_int;

    (*wcd).sconfig.ch_count = 1;
    (*wcd).active_ports = 0;
    i = 0;
    while i < WCD937X_MAX_SWR_PORTS {
        ch_mask = (*wcd).port_config[i].ch_mask;
        if ch_mask != 0 {
            j = 0;
            while j < 4 {
                if (ch_mask & BIT(j as c_uint)) != 0 {
                    (*wcd).sconfig.ch_count += 1;
                }
                j += 1;
            }
            port_config[(*wcd).active_ports as usize] = (*wcd).port_config[i];
            (*wcd).active_ports += 1;
        }
        i += 1;
    }

    (*wcd).sconfig.bps = 1;
    (*wcd).sconfig.frame_rate = params_rate(params);
    (*wcd).sconfig.direction = if (*wcd).is_tx { SDW_DATA_DIR_TX } else { SDW_DATA_DIR_RX };
    (*wcd).sconfig.type_ = SDW_STREAM_PCM;

    sdw_stream_add_slave(
        (*wcd).sdev,
        &mut (*wcd).sconfig,
        &mut port_config[0],
        (*wcd).active_ports,
        (*wcd).sruntime,
    )
}

/*
 * Handle Soundwire out-of-band interrupt event by triggering
 * the first irq of the slave_irq irq domain, which then will
 * be handled by the regmap_irq threaded irq.
 * Looping is to ensure no interrupts were missed in the process.
 */
unsafe extern "C" fn wcd9370_interrupt_callback(
    slave: *mut sdw_slave,
    _status: *mut sdw_slave_intr_status,
) -> c_int {
    let wcd = dev_get_drvdata(&mut (*slave).dev) as *mut wcd937x_sdw_priv;

    wcd_interrupt_callback(
        slave,
        (*wcd).slave_irq,
        WCD937X_DIGITAL_INTR_STATUS_0,
        WCD937X_DIGITAL_INTR_STATUS_1,
        WCD937X_DIGITAL_INTR_STATUS_2,
    )
}

// Register default table translated from wcd937x_defaults.
// Register constants are provided by the translated dependency corresponding to "wcd937x.h".
macro_rules! reg_defaults {
    ($($reg:ident => $val:expr),* $(,)?) => {
        &[$(reg_default { reg: $reg, def: $val },)*]
    };
}

// The C source contains a long static const reg_default initializer. It is represented
// here in the same order through a Rust macro invocation so dependency-provided register
// constants remain symbolic and side-effect free.
static wcd937x_defaults: &[reg_default] = reg_defaults! {
    WCD937X_ANA_BIAS => 0x00, WCD937X_ANA_RX_SUPPLIES => 0x00, WCD937X_ANA_HPH => 0x0c,
    WCD937X_ANA_EAR => 0x00, WCD937X_ANA_EAR_COMPANDER_CTL => 0x02,
    WCD937X_ANA_TX_CH1 => 0x20, WCD937X_ANA_TX_CH2 => 0x00, WCD937X_ANA_TX_CH3 => 0x20,
    WCD937X_ANA_TX_CH3_HPF => 0x00, WCD937X_ANA_MICB1_MICB2_DSP_EN_LOGIC => 0x00,
    WCD937X_ANA_MICB3_DSP_EN_LOGIC => 0x00, WCD937X_ANA_MBHC_MECH => 0x39,
    WCD937X_ANA_MBHC_ELECT => 0x08, WCD937X_ANA_MBHC_ZDET => 0x00,
    WCD937X_ANA_MBHC_BTN0 => 0x00, WCD937X_ANA_MBHC_BTN1 => 0x10,
    WCD937X_ANA_MBHC_BTN2 => 0x20, WCD937X_ANA_MBHC_BTN3 => 0x30,
    WCD937X_ANA_MBHC_BTN4 => 0x40, WCD937X_ANA_MBHC_BTN5 => 0x50,
    WCD937X_ANA_MBHC_BTN6 => 0x60, WCD937X_ANA_MBHC_BTN7 => 0x70,
    WCD937X_ANA_MICB1 => 0x10, WCD937X_ANA_MICB2 => 0x10, WCD937X_ANA_MICB2_RAMP => 0x00,
    WCD937X_ANA_MICB3 => 0x10, WCD937X_BIAS_CTL => 0x2a, WCD937X_BIAS_VBG_FINE_ADJ => 0x55,
    WCD937X_LDOL_VDDCX_ADJUST => 0x01, WCD937X_LDOL_DISABLE_LDOL => 0x00,
    WCD937X_MBHC_CTL_CLK => 0x00, WCD937X_MBHC_CTL_ANA => 0x00,
    WCD937X_MBHC_CTL_SPARE_1 => 0x00, WCD937X_MBHC_CTL_SPARE_2 => 0x00,
    WCD937X_MBHC_CTL_BCS => 0x00, WCD937X_MBHC_TEST_CTL => 0x00,
    WCD937X_LDOH_MODE => 0x2b, WCD937X_LDOH_BIAS => 0x68, WCD937X_LDOH_STB_LOADS => 0x00,
    WCD937X_LDOH_SLOWRAMP => 0x50, WCD937X_MICB1_TEST_CTL_1 => 0x1a,
    WCD937X_MICB1_TEST_CTL_2 => 0x18, WCD937X_MICB1_TEST_CTL_3 => 0xa4,
    WCD937X_MICB2_TEST_CTL_1 => 0x1a, WCD937X_MICB2_TEST_CTL_2 => 0x18,
    WCD937X_MICB2_TEST_CTL_3 => 0xa4, WCD937X_MICB3_TEST_CTL_1 => 0x1a,
    WCD937X_MICB3_TEST_CTL_2 => 0x18, WCD937X_MICB3_TEST_CTL_3 => 0xa4,
    WCD937X_TX_COM_ADC_VCM => 0x39, WCD937X_TX_COM_BIAS_ATEST => 0xc0,
    WCD937X_TX_COM_ADC_INT1_IB => 0x6f, WCD937X_TX_COM_ADC_INT2_IB => 0x4f,
    WCD937X_TX_COM_TXFE_DIV_CTL => 0x2e, WCD937X_TX_COM_TXFE_DIV_START => 0x00,
    WCD937X_TX_COM_TXFE_DIV_STOP_9P6M => 0xc7, WCD937X_TX_COM_TXFE_DIV_STOP_12P288M => 0xff,
    WCD937X_TX_1_2_TEST_EN => 0xcc, WCD937X_TX_1_2_ADC_IB => 0x09,
    WCD937X_TX_1_2_ATEST_REFCTL => 0x0a, WCD937X_TX_1_2_TEST_CTL => 0x38,
    WCD937X_TX_1_2_TEST_BLK_EN => 0xff, WCD937X_TX_1_2_TXFE_CLKDIV => 0x00,
    WCD937X_TX_3_TEST_EN => 0xcc, WCD937X_TX_3_ADC_IB => 0x09,
    WCD937X_TX_3_ATEST_REFCTL => 0x0a, WCD937X_TX_3_TEST_CTL => 0x38,
    WCD937X_TX_3_TEST_BLK_EN => 0xff, WCD937X_TX_3_TXFE_CLKDIV => 0x00,
    WCD937X_TX_3_SPARE_MONO => 0x00,
    WCD937X_CLASSH_MODE_1 => 0x40, WCD937X_CLASSH_MODE_2 => 0x3a, WCD937X_CLASSH_MODE_3 => 0x00,
    WCD937X_CLASSH_CTRL_VCL_1 => 0x70, WCD937X_CLASSH_CTRL_VCL_2 => 0x82,
    WCD937X_CLASSH_CTRL_CCL_1 => 0x31, WCD937X_CLASSH_CTRL_CCL_2 => 0x80,
    WCD937X_CLASSH_CTRL_CCL_3 => 0x80, WCD937X_CLASSH_CTRL_CCL_4 => 0x51,
    WCD937X_CLASSH_CTRL_CCL_5 => 0x00, WCD937X_CLASSH_BUCK_TMUX_A_D => 0x00,
    WCD937X_CLASSH_BUCK_SW_DRV_CNTL => 0x77, WCD937X_CLASSH_SPARE => 0x00,
    WCD937X_FLYBACK_EN => 0x4e, WCD937X_FLYBACK_VNEG_CTRL_1 => 0x0b,
    WCD937X_FLYBACK_VNEG_CTRL_2 => 0x45, WCD937X_FLYBACK_VNEG_CTRL_3 => 0x74,
    WCD937X_FLYBACK_VNEG_CTRL_4 => 0x7f, WCD937X_FLYBACK_VNEG_CTRL_5 => 0x83,
    WCD937X_FLYBACK_VNEG_CTRL_6 => 0x98, WCD937X_FLYBACK_VNEG_CTRL_7 => 0xa9,
    WCD937X_FLYBACK_VNEG_CTRL_8 => 0x68, WCD937X_FLYBACK_VNEG_CTRL_9 => 0x64,
    WCD937X_FLYBACK_VNEGDAC_CTRL_1 => 0xed, WCD937X_FLYBACK_VNEGDAC_CTRL_2 => 0xf0,
    WCD937X_FLYBACK_VNEGDAC_CTRL_3 => 0xa6, WCD937X_FLYBACK_CTRL_1 => 0x65,
    WCD937X_FLYBACK_TEST_CTL => 0x00,
    WCD937X_DIGITAL_PAGE_REGISTER => 0x00, WCD937X_DIGITAL_CDC_RST_CTL => 0x03,
    WCD937X_DIGITAL_TOP_CLK_CFG => 0x00, WCD937X_DIGITAL_CDC_PATH_MODE => 0x55,
    WCD937X_DIGITAL_INTR_MASK_0 => 0xff, WCD937X_DIGITAL_INTR_MASK_1 => 0xff,
    WCD937X_DIGITAL_INTR_MASK_2 => 0x0f, WCD937X_DIGITAL_SPARE_0 => 0x00,
    WCD937X_DIGITAL_SPARE_1 => 0x00, WCD937X_DIGITAL_SPARE_2 => 0x00,
};

macro_rules! reg_is_one_of {
    ($reg:expr, $($name:ident),* $(,)?) => {
        false $(|| $reg == $name)*
    };
}

unsafe extern "C" fn wcd937x_rdwr_register(_dev: *mut device, reg: c_uint) -> bool {
    reg_is_one_of!(
        reg,
        WCD937X_ANA_BIAS, WCD937X_ANA_RX_SUPPLIES, WCD937X_ANA_HPH, WCD937X_ANA_EAR,
        WCD937X_ANA_EAR_COMPANDER_CTL, WCD937X_ANA_TX_CH1, WCD937X_ANA_TX_CH2,
        WCD937X_ANA_TX_CH3, WCD937X_ANA_TX_CH3_HPF, WCD937X_ANA_MICB1_MICB2_DSP_EN_LOGIC,
        WCD937X_ANA_MICB3_DSP_EN_LOGIC, WCD937X_ANA_MBHC_MECH, WCD937X_ANA_MBHC_ELECT,
        WCD937X_ANA_MBHC_ZDET, WCD937X_ANA_MBHC_BTN0, WCD937X_ANA_MBHC_BTN1,
        WCD937X_ANA_MBHC_BTN2, WCD937X_ANA_MBHC_BTN3, WCD937X_ANA_MBHC_BTN4,
        WCD937X_ANA_MBHC_BTN5, WCD937X_ANA_MBHC_BTN6, WCD937X_ANA_MBHC_BTN7,
        WCD937X_ANA_MICB1, WCD937X_ANA_MICB2, WCD937X_ANA_MICB2_RAMP, WCD937X_ANA_MICB3,
        WCD937X_BIAS_CTL, WCD937X_BIAS_VBG_FINE_ADJ, WCD937X_LDOL_VDDCX_ADJUST,
        WCD937X_LDOL_DISABLE_LDOL, WCD937X_MBHC_CTL_CLK, WCD937X_MBHC_CTL_ANA,
        WCD937X_MBHC_CTL_SPARE_1, WCD937X_MBHC_CTL_SPARE_2, WCD937X_MBHC_CTL_BCS,
        WCD937X_MBHC_TEST_CTL, WCD937X_LDOH_MODE, WCD937X_LDOH_BIAS, WCD937X_LDOH_STB_LOADS,
        WCD937X_LDOH_SLOWRAMP, WCD937X_TX_COM_ADC_VCM, WCD937X_TX_1_2_TEST_EN,
        WCD937X_TX_3_TEST_EN, WCD937X_CLASSH_MODE_1, WCD937X_CLASSH_MODE_2,
        WCD937X_FLYBACK_EN, WCD937X_HPH_L_EN, WCD937X_HPH_R_EN, WCD937X_SLEEP_CTL,
        WCD937X_AUX_AUXPA, WCD937X_DIGITAL_CDC_RST_CTL, WCD937X_DIGITAL_INTR_MASK_0,
        WCD937X_DIGITAL_INTR_MASK_1, WCD937X_DIGITAL_INTR_MASK_2, WCD937X_DIGITAL_SPARE_0,
        WCD937X_DIGITAL_SPARE_1, WCD937X_DIGITAL_SPARE_2
    )
}

unsafe extern "C" fn wcd937x_readable_register(dev: *mut device, reg: c_uint) -> bool {
    if reg_is_one_of!(
        reg,
        WCD937X_ANA_MBHC_RESULT_1, WCD937X_ANA_MBHC_RESULT_2, WCD937X_ANA_MBHC_RESULT_3,
        WCD937X_MBHC_MOISTURE_DET_FSM_STATUS, WCD937X_TX_1_2_SAR2_ERR,
        WCD937X_TX_1_2_SAR1_ERR, WCD937X_TX_3_SPARE_MONO, WCD937X_TX_3_SAR1_ERR,
        WCD937X_HPH_L_STATUS, WCD937X_HPH_R_STATUS, WCD937X_HPH_SURGE_HPHLR_SURGE_STATUS,
        WCD937X_EAR_STATUS_REG_1, WCD937X_EAR_STATUS_REG_2, WCD937X_MBHC_NEW_FSM_STATUS,
        WCD937X_MBHC_NEW_ADC_RESULT, WCD937X_DIE_CRACK_DIE_CRK_DET_OUT,
        WCD937X_AUX_INT_STATUS_REG, WCD937X_LDORXTX_INT_STATUS, WCD937X_DIGITAL_CHIP_ID0,
        WCD937X_DIGITAL_CHIP_ID1, WCD937X_DIGITAL_CHIP_ID2, WCD937X_DIGITAL_CHIP_ID3,
        WCD937X_DIGITAL_INTR_STATUS_0, WCD937X_DIGITAL_INTR_STATUS_1,
        WCD937X_DIGITAL_INTR_STATUS_2
    ) {
        return true;
    }
    wcd937x_rdwr_register(dev, reg)
}

unsafe extern "C" fn wcd937x_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg_is_one_of!(
        reg,
        WCD937X_ANA_MBHC_RESULT_1, WCD937X_ANA_MBHC_RESULT_2, WCD937X_ANA_MBHC_RESULT_3,
        WCD937X_MBHC_MOISTURE_DET_FSM_STATUS, WCD937X_TX_1_2_SAR1_ERR,
        WCD937X_TX_1_2_SAR2_ERR, WCD937X_TX_3_SAR1_ERR, WCD937X_HPH_L_STATUS,
        WCD937X_HPH_R_STATUS, WCD937X_HPH_SURGE_HPHLR_SURGE_STATUS, WCD937X_EAR_STATUS_REG_1,
        WCD937X_EAR_STATUS_REG_2, WCD937X_MBHC_NEW_FSM_STATUS, WCD937X_MBHC_NEW_ADC_RESULT,
        WCD937X_DIE_CRACK_DIE_CRK_DET_OUT, WCD937X_DIGITAL_INTR_STATUS_0,
        WCD937X_DIGITAL_INTR_STATUS_1, WCD937X_DIGITAL_INTR_STATUS_2,
        WCD937X_DIGITAL_SWR_HM_TEST, WCD937X_DIGITAL_PIN_STATUS_0,
        WCD937X_DIGITAL_PIN_STATUS_1, WCD937X_DIGITAL_MODE_STATUS_0,
        WCD937X_DIGITAL_MODE_STATUS_1
    )
}

static wcd937x_regmap_config_name: &[u8] = b"wcd937x_csr\0";
static wcd937x_regmap_config: regmap_config = regmap_config {
    name: wcd937x_regmap_config_name.as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 8,
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wcd937x_defaults.as_ptr(),
    num_reg_defaults: wcd937x_defaults.len() as c_uint,
    max_register: WCD937X_MAX_REGISTER,
    readable_reg: Some(wcd937x_readable_register),
    writeable_reg: Some(wcd937x_rdwr_register),
    volatile_reg: Some(wcd937x_volatile_register),
};

static wcd9370_slave_ops: sdw_slave_ops = sdw_slave_ops {
    update_status: Some(unsafe { core::mem::transmute(wcd_update_status) }),
    interrupt_callback: Some(wcd9370_interrupt_callback),
};

unsafe extern "C" fn wcd9370_probe(pdev: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut master_ch_mask: [u8; WCD937X_MAX_SWR_CH_IDS] = [0; WCD937X_MAX_SWR_CH_IDS];
    let mut master_ch_mask_size: c_int = 0;
    let mut ret: c_int;
    let mut i: c_int;

    let wcd = devm_kzalloc(dev, core::mem::size_of::<wcd937x_sdw_priv>(), GFP_KERNEL)
        as *mut wcd937x_sdw_priv;
    if wcd.is_null() {
        return -ENOMEM;
    }

    // Port map index starts at 0, however the data port for this codec start at index 1.
    if of_property_present(ptr::null_mut(), b"qcom,tx-port-mapping\0".as_ptr() as *const c_char) {
        (*wcd).is_tx = true;
        ret = of_property_read_u32_array(
            ptr::null_mut(),
            b"qcom,tx-port-mapping\0".as_ptr() as *const c_char,
            (*pdev).m_port_map.as_mut_ptr().add(1),
            WCD937X_MAX_TX_SWR_PORTS,
        );
    } else {
        ret = of_property_read_u32_array(
            ptr::null_mut(),
            b"qcom,rx-port-mapping\0".as_ptr() as *const c_char,
            (*pdev).m_port_map.as_mut_ptr().add(1),
            WCD937X_MAX_SWR_PORTS,
        );
    }
    if ret < 0 {
        dev_info(
            dev,
            b"Error getting static port mapping for %s (%d)\n\0".as_ptr() as *const c_char,
            if (*wcd).is_tx { b"TX\0".as_ptr() } else { b"RX\0".as_ptr() },
            ret,
        );
    }

    (*wcd).sdev = pdev;
    dev_set_drvdata(dev, wcd as *mut c_void);

    (*pdev).prop.scp_int1_mask = SDW_SCP_INT1_IMPL_DEF | SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*pdev).prop.lane_control_support = true;
    (*pdev).prop.simple_clk_stop_capable = true;

    if (*wcd).is_tx {
        master_ch_mask_size = of_property_count_u8_elems(ptr::null_mut(), b"qcom,tx-channel-mapping\0".as_ptr() as *const c_char);
        if master_ch_mask_size != 0 {
            ret = of_property_read_u8_array(ptr::null_mut(), b"qcom,tx-channel-mapping\0".as_ptr() as *const c_char, master_ch_mask.as_mut_ptr(), master_ch_mask_size as usize);
        }
    } else {
        master_ch_mask_size = of_property_count_u8_elems(ptr::null_mut(), b"qcom,rx-channel-mapping\0".as_ptr() as *const c_char);
        if master_ch_mask_size != 0 {
            ret = of_property_read_u8_array(ptr::null_mut(), b"qcom,rx-channel-mapping\0".as_ptr() as *const c_char, master_ch_mask.as_mut_ptr(), master_ch_mask_size as usize);
        }
    }

    if ret < 0 {
        dev_info(dev, b"Static channel mapping not specified using device channel maps\n\0".as_ptr() as *const c_char);
    }

    if (*wcd).is_tx {
        (*pdev).prop.source_ports = GENMASK(WCD937X_MAX_TX_SWR_PORTS, 0);
        (*pdev).prop.src_dpn_prop = wcd937x_dpn_prop.as_mut_ptr();
        (*wcd).ch_info = wcd937x_sdw_tx_ch_info.as_mut_ptr();

        i = 0;
        while i < master_ch_mask_size {
            (*(*wcd).ch_info.add(i as usize)).master_ch_mask = WCD937X_SWRM_CH_MASK(master_ch_mask[i as usize]);
            i += 1;
        }

        (*pdev).prop.wake_capable = true;

        (*wcd).regmap = devm_regmap_init_sdw(pdev, &wcd937x_regmap_config);
        if ((*wcd).regmap as isize) < 0 {
            return dev_err_probe(dev, (*wcd).regmap as isize, b"Regmap init failed\n\0".as_ptr() as *const c_char);
        }

        // Start in cache-only until device is enumerated.
        regcache_cache_only((*wcd).regmap, true);
    } else {
        (*pdev).prop.sink_ports = GENMASK(WCD937X_MAX_SWR_PORTS - 1, 0);
        (*pdev).prop.sink_dpn_prop = wcd937x_dpn_prop.as_mut_ptr();
        (*wcd).ch_info = wcd937x_sdw_rx_ch_info.as_mut_ptr();

        i = 0;
        while i < master_ch_mask_size {
            (*(*wcd).ch_info.add(i as usize)).master_ch_mask = WCD937X_SWRM_CH_MASK(master_ch_mask[i as usize]);
            i += 1;
        }
    }

    ret = component_add(dev, &wcd_sdw_component_ops);
    if ret != 0 {
        return ret;
    }

    // Set suspended until aggregate device is bind.
    pm_runtime_set_suspended(dev);

    0
}

unsafe extern "C" fn wcd9370_remove(pdev: *mut sdw_slave) {
    let dev = &mut (*pdev).dev as *mut device;
    component_del(dev, &wcd_sdw_component_ops);
}

// static const struct sdw_device_id wcd9370_slave_id[] = {
//     SDW_SLAVE_ENTRY(0x0217, 0x10a, 0), /* WCD9370 RX/TX Device ID */
//     { },
// };
// MODULE_DEVICE_TABLE(sdw, wcd9370_slave_id);
static wcd9370_slave_id: [sdw_device_id; 2] = unsafe { MaybeUninit::zeroed().assume_init() };

unsafe extern "C" fn wcd937x_sdw_runtime_suspend(dev: *mut device) -> c_int {
    let wcd = dev_get_drvdata(dev) as *mut wcd937x_sdw_priv;

    if !(*wcd).regmap.is_null() {
        regcache_cache_only((*wcd).regmap, true);
        regcache_mark_dirty((*wcd).regmap);
    }

    0
}

unsafe extern "C" fn wcd937x_sdw_runtime_resume(dev: *mut device) -> c_int {
    let wcd = dev_get_drvdata(dev) as *mut wcd937x_sdw_priv;
    let ret: c_int;

    if !(*wcd).regmap.is_null() {
        regcache_cache_only((*wcd).regmap, false);
        ret = regcache_sync((*wcd).regmap);
        if ret != 0 {
            regcache_cache_only((*wcd).regmap, true);
            regcache_mark_dirty((*wcd).regmap);
            return ret;
        }
    }

    0
}

static wcd937x_sdw_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(wcd937x_sdw_runtime_suspend),
    runtime_resume: Some(wcd937x_sdw_runtime_resume),
};

static wcd9370_codec_driver_name: &[u8] = b"wcd9370-codec\0";
static mut wcd9370_codec_driver: sdw_driver = sdw_driver {
    probe: Some(wcd9370_probe),
    remove: Some(wcd9370_remove),
    ops: &wcd9370_slave_ops,
    id_table: wcd9370_slave_id.as_ptr(),
    driver: driver_inner {
        name: wcd9370_codec_driver_name.as_ptr() as *const c_char,
        pm: &wcd937x_sdw_pm_ops,
    },
};

// module_sdw_driver(wcd9370_codec_driver);
// MODULE_DESCRIPTION("WCD937X SDW codec driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
