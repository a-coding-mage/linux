// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021, Linaro Limited

// Rust translation of ./wcd938x-sdw.c. External kernel headers and local codec
// headers remain dependencies of the final repository integration.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;

const fn BIT(n: c_uint) -> c_ulong { 1c_ulong << n }
const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    if h >= 31 { c_uint::MAX << l } else { ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1) }
}

#[repr(C)] pub struct wcd_sdw_ch_info { _private: [u8; 0] }
#[repr(C)] pub struct sdw_dpn_prop { pub num: c_uint, pub type_: c_uint, pub min_ch: c_uint, pub max_ch: c_uint, pub simple_ch_prep_sm: bool }
#[repr(C)] #[derive(Copy, Clone)] pub struct sdw_port_config { pub num: c_uint, pub ch_mask: c_ulong }
#[repr(C)] pub struct sdw_stream_config { pub ch_count: c_uint, pub bps: c_uint, pub frame_rate: c_uint, pub direction: c_uint, pub type_: c_uint }
#[repr(C)] pub struct wcd938x_sdw_priv { pub sconfig: sdw_stream_config, pub active_ports: c_uint, pub port_config: [sdw_port_config; WCD938X_MAX_SWR_PORTS], pub is_tx: bool, pub sdev: *mut sdw_slave, pub sruntime: *mut c_void, pub slave_irq: c_int, pub ch_info: *const wcd_sdw_ch_info, pub regmap: *mut regmap }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] pub struct sdw_slave_intr_status { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct sdw_slave_prop { pub scp_int1_mask: c_uint, pub lane_control_support: bool, pub simple_clk_stop_capable: bool, pub source_ports: c_uint, pub src_dpn_prop: *mut sdw_dpn_prop, pub wake_capable: bool, pub sink_ports: c_uint, pub sink_dpn_prop: *mut sdw_dpn_prop }
#[repr(C)] pub struct sdw_slave { pub dev: device, pub m_port_map: [u32; WCD938X_MAX_SWR_PORTS + 1], pub prop: sdw_slave_prop }
#[repr(C)] pub struct sdw_device_id { _private: [u8; 0] }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct sdw_slave_ops { pub update_status: Option<unsafe extern "C" fn() -> c_int>, pub interrupt_callback: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>, pub bus_config: Option<unsafe extern "C" fn() -> c_int> }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub pm: *const dev_pm_ops }
#[repr(C)] pub struct sdw_driver { pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>, pub ops: *const sdw_slave_ops, pub id_table: *const sdw_device_id, pub driver: device_driver }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct regmap_config { pub name: *const c_char, pub reg_bits: c_uint, pub val_bits: c_uint, pub cache_type: c_uint, pub reg_defaults: *const reg_default, pub num_reg_defaults: c_uint, pub max_register: c_uint, pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>, pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>, pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool> }

unsafe extern "C" {
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn sdw_stream_add_slave(sdev: *mut sdw_slave, sconfig: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_uint, sruntime: *mut c_void) -> c_int;
    fn sdw_stream_remove_slave(sdev: *mut sdw_slave, sruntime: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn wcd_interrupt_callback(slave: *mut sdw_slave, irq: c_int, s0: c_uint, s1: c_uint, s2: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut u32, sz: usize) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn devm_regmap_init_sdw(sdw: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn component_add(dev: *mut device, ops: *const c_void) -> c_int;
    fn component_del(dev: *mut device, ops: *const c_void);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    static wcd_sdw_component_ops: c_void;
    static wcd_update_status: unsafe extern "C" fn() -> c_int;
    static wcd_bus_config: unsafe extern "C" fn() -> c_int;
    fn WCD_SDW_CH(ch: c_uint, port: c_uint, mask: c_ulong) -> wcd_sdw_ch_info;
}

pub static wcd938x_sdw_rx_ch_info: &[wcd_sdw_ch_info] = &[
    WCD_SDW_CH(WCD938X_HPH_L, WCD938X_HPH_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_HPH_R, WCD938X_HPH_PORT, BIT(1)),
    WCD_SDW_CH(WCD938X_CLSH, WCD938X_CLSH_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_COMP_L, WCD938X_COMP_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_COMP_R, WCD938X_COMP_PORT, BIT(1)),
    WCD_SDW_CH(WCD938X_LO, WCD938X_LO_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_DSD_L, WCD938X_DSD_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_DSD_R, WCD938X_DSD_PORT, BIT(1)),
];

pub static wcd938x_sdw_tx_ch_info: &[wcd_sdw_ch_info] = &[
    WCD_SDW_CH(WCD938X_ADC1, WCD938X_ADC_1_2_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_ADC2, WCD938X_ADC_1_2_PORT, BIT(1)),
    WCD_SDW_CH(WCD938X_ADC3, WCD938X_ADC_3_4_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_ADC4, WCD938X_ADC_3_4_PORT, BIT(1)),
    WCD_SDW_CH(WCD938X_DMIC0, WCD938X_DMIC_0_3_MBHC_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_DMIC1, WCD938X_DMIC_0_3_MBHC_PORT, BIT(1)),
    WCD_SDW_CH(WCD938X_MBHC, WCD938X_DMIC_0_3_MBHC_PORT, BIT(2)),
    WCD_SDW_CH(WCD938X_DMIC2, WCD938X_DMIC_0_3_MBHC_PORT, BIT(2)),
    WCD_SDW_CH(WCD938X_DMIC3, WCD938X_DMIC_0_3_MBHC_PORT, BIT(3)),
    WCD_SDW_CH(WCD938X_DMIC4, WCD938X_DMIC_4_7_PORT, BIT(0)),
    WCD_SDW_CH(WCD938X_DMIC5, WCD938X_DMIC_4_7_PORT, BIT(1)),
    WCD_SDW_CH(WCD938X_DMIC6, WCD938X_DMIC_4_7_PORT, BIT(2)),
    WCD_SDW_CH(WCD938X_DMIC7, WCD938X_DMIC_4_7_PORT, BIT(3)),
];

pub static mut wcd938x_dpn_prop: [sdw_dpn_prop; WCD938X_MAX_SWR_PORTS] = [
    sdw_dpn_prop { num: 1, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 8, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 2, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 3, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 4, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
    sdw_dpn_prop { num: 5, type_: SDW_DPN_SIMPLE, min_ch: 1, max_ch: 4, simple_ch_prep_sm: true },
];

pub unsafe extern "C" fn wcd938x_sdw_hw_params(wcd: *mut wcd938x_sdw_priv, _substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, _dai: *mut snd_soc_dai) -> c_int {
    let mut port_config: [sdw_port_config; WCD938X_MAX_SWR_PORTS] = MaybeUninit::zeroed().assume_init();
    (*wcd).sconfig.ch_count = 1;
    (*wcd).active_ports = 0;
    let mut i = 0usize;
    while i < WCD938X_MAX_SWR_PORTS {
        let ch_mask = (*wcd).port_config[i].ch_mask;
        if ch_mask != 0 {
            let mut j = 0usize;
            while j < 4 {
                if (ch_mask & BIT(j as c_uint)) != 0 { (*wcd).sconfig.ch_count += 1; }
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
    sdw_stream_add_slave((*wcd).sdev, &mut (*wcd).sconfig, port_config.as_mut_ptr(), (*wcd).active_ports, (*wcd).sruntime)
}
// EXPORT_SYMBOL_GPL(wcd938x_sdw_hw_params);

pub unsafe extern "C" fn wcd938x_sdw_free(wcd: *mut wcd938x_sdw_priv, _substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) -> c_int { sdw_stream_remove_slave((*wcd).sdev, (*wcd).sruntime); 0 }
// EXPORT_SYMBOL_GPL(wcd938x_sdw_free);

pub unsafe extern "C" fn wcd938x_sdw_set_sdw_stream(wcd: *mut wcd938x_sdw_priv, _dai: *mut snd_soc_dai, stream: *mut c_void, _direction: c_int) -> c_int { (*wcd).sruntime = stream; 0 }
// EXPORT_SYMBOL_GPL(wcd938x_sdw_set_sdw_stream);

unsafe extern "C" fn wcd9380_interrupt_callback(slave: *mut sdw_slave, _status: *mut sdw_slave_intr_status) -> c_int {
    let wcd = dev_get_drvdata(&mut (*slave).dev) as *mut wcd938x_sdw_priv;
    wcd_interrupt_callback(slave, (*wcd).slave_irq, WCD938X_DIGITAL_INTR_STATUS_0, WCD938X_DIGITAL_INTR_STATUS_1, WCD938X_DIGITAL_INTR_STATUS_2)
}

static wcd938x_defaults: &[reg_default] = &[

    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },
    reg_default { reg: $1, def: $2 },];

unsafe extern "C" fn wcd938x_rdwr_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WCD938X_ANA_PAGE_REGISTER |
        WCD938X_ANA_BIAS |
        WCD938X_ANA_RX_SUPPLIES |
        WCD938X_ANA_HPH |
        WCD938X_ANA_EAR |
        WCD938X_ANA_EAR_COMPANDER_CTL |
        WCD938X_ANA_TX_CH1 |
        WCD938X_ANA_TX_CH2 |
        WCD938X_ANA_TX_CH3 |
        WCD938X_ANA_TX_CH4 |
        WCD938X_ANA_MICB1_MICB2_DSP_EN_LOGIC |
        WCD938X_ANA_MICB3_DSP_EN_LOGIC |
        WCD938X_ANA_MBHC_MECH |
        WCD938X_ANA_MBHC_ELECT |
        WCD938X_ANA_MBHC_ZDET |
        WCD938X_ANA_MBHC_BTN0 |
        WCD938X_ANA_MBHC_BTN1 |
        WCD938X_ANA_MBHC_BTN2 |
        WCD938X_ANA_MBHC_BTN3 |
        WCD938X_ANA_MBHC_BTN4 |
        WCD938X_ANA_MBHC_BTN5 |
        WCD938X_ANA_MBHC_BTN6 |
        WCD938X_ANA_MBHC_BTN7 |
        WCD938X_ANA_MICB1 |
        WCD938X_ANA_MICB2 |
        WCD938X_ANA_MICB2_RAMP |
        WCD938X_ANA_MICB3 |
        WCD938X_ANA_MICB4 |
        WCD938X_BIAS_CTL |
        WCD938X_BIAS_VBG_FINE_ADJ |
        WCD938X_LDOL_VDDCX_ADJUST |
        WCD938X_LDOL_DISABLE_LDOL |
        WCD938X_MBHC_CTL_CLK |
        WCD938X_MBHC_CTL_ANA |
        WCD938X_MBHC_CTL_SPARE_1 |
        WCD938X_MBHC_CTL_SPARE_2 |
        WCD938X_MBHC_CTL_BCS |
        WCD938X_MBHC_TEST_CTL |
        WCD938X_LDOH_MODE |
        WCD938X_LDOH_BIAS |
        WCD938X_LDOH_STB_LOADS |
        WCD938X_LDOH_SLOWRAMP |
        WCD938X_MICB1_TEST_CTL_1 |
        WCD938X_MICB1_TEST_CTL_2 |
        WCD938X_MICB1_TEST_CTL_3 |
        WCD938X_MICB2_TEST_CTL_1 |
        WCD938X_MICB2_TEST_CTL_2 |
        WCD938X_MICB2_TEST_CTL_3 |
        WCD938X_MICB3_TEST_CTL_1 |
        WCD938X_MICB3_TEST_CTL_2 |
        WCD938X_MICB3_TEST_CTL_3 |
        WCD938X_MICB4_TEST_CTL_1 |
        WCD938X_MICB4_TEST_CTL_2 |
        WCD938X_MICB4_TEST_CTL_3 |
        WCD938X_TX_COM_ADC_VCM |
        WCD938X_TX_COM_BIAS_ATEST |
        WCD938X_TX_COM_SPARE1 |
        WCD938X_TX_COM_SPARE2 |
        WCD938X_TX_COM_TXFE_DIV_CTL |
        WCD938X_TX_COM_TXFE_DIV_START |
        WCD938X_TX_COM_SPARE3 |
        WCD938X_TX_COM_SPARE4 |
        WCD938X_TX_1_2_TEST_EN |
        WCD938X_TX_1_2_ADC_IB |
        WCD938X_TX_1_2_ATEST_REFCTL |
        WCD938X_TX_1_2_TEST_CTL |
        WCD938X_TX_1_2_TEST_BLK_EN1 |
        WCD938X_TX_1_2_TXFE1_CLKDIV |
        WCD938X_TX_3_4_TEST_EN |
        WCD938X_TX_3_4_ADC_IB |
        WCD938X_TX_3_4_ATEST_REFCTL |
        WCD938X_TX_3_4_TEST_CTL |
        WCD938X_TX_3_4_TEST_BLK_EN3 |
        WCD938X_TX_3_4_TXFE3_CLKDIV |
        WCD938X_TX_3_4_TEST_BLK_EN2 |
        WCD938X_TX_3_4_TXFE2_CLKDIV |
        WCD938X_TX_3_4_SPARE1 |
        WCD938X_TX_3_4_TEST_BLK_EN4 |
        WCD938X_TX_3_4_TXFE4_CLKDIV |
        WCD938X_TX_3_4_SPARE2 |
        WCD938X_CLASSH_MODE_1 |
        WCD938X_CLASSH_MODE_2 |
        WCD938X_CLASSH_MODE_3 |
        WCD938X_CLASSH_CTRL_VCL_1 |
        WCD938X_CLASSH_CTRL_VCL_2 |
        WCD938X_CLASSH_CTRL_CCL_1 |
        WCD938X_CLASSH_CTRL_CCL_2 |
        WCD938X_CLASSH_CTRL_CCL_3 |
        WCD938X_CLASSH_CTRL_CCL_4 |
        WCD938X_CLASSH_CTRL_CCL_5 |
        WCD938X_CLASSH_BUCK_TMUX_A_D |
        WCD938X_CLASSH_BUCK_SW_DRV_CNTL |
        WCD938X_CLASSH_SPARE |
        WCD938X_FLYBACK_EN |
        WCD938X_FLYBACK_VNEG_CTRL_1 |
        WCD938X_FLYBACK_VNEG_CTRL_2 |
        WCD938X_FLYBACK_VNEG_CTRL_3 |
        WCD938X_FLYBACK_VNEG_CTRL_4 |
        WCD938X_FLYBACK_VNEG_CTRL_5 |
        WCD938X_FLYBACK_VNEG_CTRL_6 |
        WCD938X_FLYBACK_VNEG_CTRL_7 |
        WCD938X_FLYBACK_VNEG_CTRL_8 |
        WCD938X_FLYBACK_VNEG_CTRL_9 |
        WCD938X_FLYBACK_VNEGDAC_CTRL_1 |
        WCD938X_FLYBACK_VNEGDAC_CTRL_2 |
        WCD938X_FLYBACK_VNEGDAC_CTRL_3 |
        WCD938X_FLYBACK_CTRL_1 |
        WCD938X_FLYBACK_TEST_CTL |
        WCD938X_RX_AUX_SW_CTL |
        WCD938X_RX_PA_AUX_IN_CONN |
        WCD938X_RX_TIMER_DIV |
        WCD938X_RX_OCP_CTL |
        WCD938X_RX_OCP_COUNT |
        WCD938X_RX_BIAS_EAR_DAC |
        WCD938X_RX_BIAS_EAR_AMP |
        WCD938X_RX_BIAS_HPH_LDO |
        WCD938X_RX_BIAS_HPH_PA |
        WCD938X_RX_BIAS_HPH_RDACBUFF_CNP2 |
        WCD938X_RX_BIAS_HPH_RDAC_LDO |
        WCD938X_RX_BIAS_HPH_CNP1 |
        WCD938X_RX_BIAS_HPH_LOWPOWER |
        WCD938X_RX_BIAS_AUX_DAC |
        WCD938X_RX_BIAS_AUX_AMP |
        WCD938X_RX_BIAS_VNEGDAC_BLEEDER |
        WCD938X_RX_BIAS_MISC |
        WCD938X_RX_BIAS_BUCK_RST |
        WCD938X_RX_BIAS_BUCK_VREF_ERRAMP |
        WCD938X_RX_BIAS_FLYB_ERRAMP |
        WCD938X_RX_BIAS_FLYB_BUFF |
        WCD938X_RX_BIAS_FLYB_MID_RST |
        WCD938X_HPH_CNP_EN |
        WCD938X_HPH_CNP_WG_CTL |
        WCD938X_HPH_CNP_WG_TIME |
        WCD938X_HPH_OCP_CTL |
        WCD938X_HPH_AUTO_CHOP |
        WCD938X_HPH_CHOP_CTL |
        WCD938X_HPH_PA_CTL1 |
        WCD938X_HPH_PA_CTL2 |
        WCD938X_HPH_L_EN |
        WCD938X_HPH_L_TEST |
        WCD938X_HPH_L_ATEST |
        WCD938X_HPH_R_EN |
        WCD938X_HPH_R_TEST |
        WCD938X_HPH_R_ATEST |
        WCD938X_HPH_RDAC_CLK_CTL1 |
        WCD938X_HPH_RDAC_CLK_CTL2 |
        WCD938X_HPH_RDAC_LDO_CTL |
        WCD938X_HPH_RDAC_CHOP_CLK_LP_CTL |
        WCD938X_HPH_REFBUFF_UHQA_CTL |
        WCD938X_HPH_REFBUFF_LP_CTL |
        WCD938X_HPH_L_DAC_CTL |
        WCD938X_HPH_R_DAC_CTL |
        WCD938X_HPH_SURGE_HPHLR_SURGE_COMP_SEL |
        WCD938X_HPH_SURGE_HPHLR_SURGE_EN |
        WCD938X_HPH_SURGE_HPHLR_SURGE_MISC1 |
        WCD938X_EAR_EAR_EN_REG |
        WCD938X_EAR_EAR_PA_CON |
        WCD938X_EAR_EAR_SP_CON |
        WCD938X_EAR_EAR_DAC_CON |
        WCD938X_EAR_EAR_CNP_FSM_CON |
        WCD938X_EAR_TEST_CTL |
        WCD938X_ANA_NEW_PAGE_REGISTER |
        WCD938X_HPH_NEW_ANA_HPH2 |
        WCD938X_HPH_NEW_ANA_HPH3 |
        WCD938X_SLEEP_CTL |
        WCD938X_SLEEP_WATCHDOG_CTL |
        WCD938X_MBHC_NEW_ELECT_REM_CLAMP_CTL |
        WCD938X_MBHC_NEW_CTL_1 |
        WCD938X_MBHC_NEW_CTL_2 |
        WCD938X_MBHC_NEW_PLUG_DETECT_CTL |
        WCD938X_MBHC_NEW_ZDET_ANA_CTL |
        WCD938X_MBHC_NEW_ZDET_RAMP_CTL |
        WCD938X_TX_NEW_AMIC_MUX_CFG |
        WCD938X_AUX_AUXPA |
        WCD938X_LDORXTX_MODE |
        WCD938X_LDORXTX_CONFIG |
        WCD938X_DIE_CRACK_DIE_CRK_DET_EN |
        WCD938X_HPH_NEW_INT_RDAC_GAIN_CTL |
        WCD938X_HPH_NEW_INT_RDAC_HD2_CTL_L |
        WCD938X_HPH_NEW_INT_RDAC_VREF_CTL |
        WCD938X_HPH_NEW_INT_RDAC_OVERRIDE_CTL |
        WCD938X_HPH_NEW_INT_RDAC_HD2_CTL_R |
        WCD938X_HPH_NEW_INT_PA_MISC1 |
        WCD938X_HPH_NEW_INT_PA_MISC2 |
        WCD938X_HPH_NEW_INT_PA_RDAC_MISC |
        WCD938X_HPH_NEW_INT_HPH_TIMER1 |
        WCD938X_HPH_NEW_INT_HPH_TIMER2 |
        WCD938X_HPH_NEW_INT_HPH_TIMER3 |
        WCD938X_HPH_NEW_INT_HPH_TIMER4 |
        WCD938X_HPH_NEW_INT_PA_RDAC_MISC2 |
        WCD938X_HPH_NEW_INT_PA_RDAC_MISC3 |
        WCD938X_HPH_NEW_INT_RDAC_HD2_CTL_L_NEW |
        WCD938X_HPH_NEW_INT_RDAC_HD2_CTL_R_NEW |
        WCD938X_RX_NEW_INT_HPH_RDAC_BIAS_LOHIFI |
        WCD938X_RX_NEW_INT_HPH_RDAC_BIAS_ULP |
        WCD938X_RX_NEW_INT_HPH_RDAC_LDO_LP |
        WCD938X_MBHC_NEW_INT_MOISTURE_DET_DC_CTRL |
        WCD938X_MBHC_NEW_INT_MOISTURE_DET_POLLING_CTRL |
        WCD938X_MBHC_NEW_INT_MECH_DET_CURRENT |
        WCD938X_MBHC_NEW_INT_SPARE_2 |
        WCD938X_EAR_INT_NEW_EAR_CHOPPER_CON |
        WCD938X_EAR_INT_NEW_CNP_VCM_CON1 |
        WCD938X_EAR_INT_NEW_CNP_VCM_CON2 |
        WCD938X_EAR_INT_NEW_EAR_DYNAMIC_BIAS |
        WCD938X_AUX_INT_EN_REG |
        WCD938X_AUX_INT_PA_CTRL |
        WCD938X_AUX_INT_SP_CTRL |
        WCD938X_AUX_INT_DAC_CTRL |
        WCD938X_AUX_INT_CLK_CTRL |
        WCD938X_AUX_INT_TEST_CTRL |
        WCD938X_AUX_INT_MISC |
        WCD938X_LDORXTX_INT_BIAS |
        WCD938X_LDORXTX_INT_STB_LOADS_DTEST |
        WCD938X_LDORXTX_INT_TEST0 |
        WCD938X_LDORXTX_INT_STARTUP_TIMER |
        WCD938X_LDORXTX_INT_TEST1 |
        WCD938X_SLEEP_INT_WATCHDOG_CTL_1 |
        WCD938X_SLEEP_INT_WATCHDOG_CTL_2 |
        WCD938X_DIE_CRACK_INT_DIE_CRK_DET_INT1 |
        WCD938X_DIE_CRACK_INT_DIE_CRK_DET_INT2 |
        WCD938X_TX_COM_NEW_INT_TXFE_DIVSTOP_L2 |
        WCD938X_TX_COM_NEW_INT_TXFE_DIVSTOP_L1 |
        WCD938X_TX_COM_NEW_INT_TXFE_DIVSTOP_L0 |
        WCD938X_TX_COM_NEW_INT_TXFE_DIVSTOP_ULP1P2M |
        WCD938X_TX_COM_NEW_INT_TXFE_DIVSTOP_ULP0P6M |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG1_L2L1 |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG1_L0 |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG1_ULP |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG2MAIN_L2L1 |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG2MAIN_L0 |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG2MAIN_ULP |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG2CASC_L2L1L0 |
        WCD938X_TX_COM_NEW_INT_TXFE_ICTRL_STG2CASC_ULP |
        WCD938X_TX_COM_NEW_INT_TXADC_SCBIAS_L2L1 |
        WCD938X_TX_COM_NEW_INT_TXADC_SCBIAS_L0ULP |
        WCD938X_TX_COM_NEW_INT_TXADC_INT_L2 |
        WCD938X_TX_COM_NEW_INT_TXADC_INT_L1 |
        WCD938X_TX_COM_NEW_INT_TXADC_INT_L0 |
        WCD938X_TX_COM_NEW_INT_TXADC_INT_ULP |
        WCD938X_DIGITAL_PAGE_REGISTER |
        WCD938X_DIGITAL_SWR_TX_CLK_RATE |
        WCD938X_DIGITAL_CDC_RST_CTL |
        WCD938X_DIGITAL_TOP_CLK_CFG |
        WCD938X_DIGITAL_CDC_ANA_CLK_CTL |
        WCD938X_DIGITAL_CDC_DIG_CLK_CTL |
        WCD938X_DIGITAL_SWR_RST_EN |
        WCD938X_DIGITAL_CDC_PATH_MODE |
        WCD938X_DIGITAL_CDC_RX_RST |
        WCD938X_DIGITAL_CDC_RX0_CTL |
        WCD938X_DIGITAL_CDC_RX1_CTL |
        WCD938X_DIGITAL_CDC_RX2_CTL |
        WCD938X_DIGITAL_CDC_TX_ANA_MODE_0_1 |
        WCD938X_DIGITAL_CDC_TX_ANA_MODE_2_3 |
        WCD938X_DIGITAL_CDC_COMP_CTL_0 |
        WCD938X_DIGITAL_CDC_ANA_TX_CLK_CTL |
        WCD938X_DIGITAL_CDC_HPH_DSM_A1_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A1_1 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A2_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A2_1 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A3_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A3_1 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A4_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A4_1 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A5_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A5_1 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A6_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_A7_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_C_0 |
        WCD938X_DIGITAL_CDC_HPH_DSM_C_1 |
        WCD938X_DIGITAL_CDC_HPH_DSM_C_2 |
        WCD938X_DIGITAL_CDC_HPH_DSM_C_3 |
        WCD938X_DIGITAL_CDC_HPH_DSM_R1 |
        WCD938X_DIGITAL_CDC_HPH_DSM_R2 |
        WCD938X_DIGITAL_CDC_HPH_DSM_R3 |
        WCD938X_DIGITAL_CDC_HPH_DSM_R4 |
        WCD938X_DIGITAL_CDC_HPH_DSM_R5 |
        WCD938X_DIGITAL_CDC_HPH_DSM_R6 |
        WCD938X_DIGITAL_CDC_HPH_DSM_R7 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A1_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A1_1 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A2_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A2_1 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A3_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A3_1 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A4_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A4_1 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A5_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A5_1 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A6_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_A7_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_C_0 |
        WCD938X_DIGITAL_CDC_AUX_DSM_C_1 |
        WCD938X_DIGITAL_CDC_AUX_DSM_C_2 |
        WCD938X_DIGITAL_CDC_AUX_DSM_C_3 |
        WCD938X_DIGITAL_CDC_AUX_DSM_R1 |
        WCD938X_DIGITAL_CDC_AUX_DSM_R2 |
        WCD938X_DIGITAL_CDC_AUX_DSM_R3 |
        WCD938X_DIGITAL_CDC_AUX_DSM_R4 |
        WCD938X_DIGITAL_CDC_AUX_DSM_R5 |
        WCD938X_DIGITAL_CDC_AUX_DSM_R6 |
        WCD938X_DIGITAL_CDC_AUX_DSM_R7 |
        WCD938X_DIGITAL_CDC_HPH_GAIN_RX_0 |
        WCD938X_DIGITAL_CDC_HPH_GAIN_RX_1 |
        WCD938X_DIGITAL_CDC_HPH_GAIN_DSD_0 |
        WCD938X_DIGITAL_CDC_HPH_GAIN_DSD_1 |
        WCD938X_DIGITAL_CDC_HPH_GAIN_DSD_2 |
        WCD938X_DIGITAL_CDC_AUX_GAIN_DSD_0 |
        WCD938X_DIGITAL_CDC_AUX_GAIN_DSD_1 |
        WCD938X_DIGITAL_CDC_AUX_GAIN_DSD_2 |
        WCD938X_DIGITAL_CDC_HPH_GAIN_CTL |
        WCD938X_DIGITAL_CDC_AUX_GAIN_CTL |
        WCD938X_DIGITAL_CDC_EAR_PATH_CTL |
        WCD938X_DIGITAL_CDC_SWR_CLH |
        WCD938X_DIGITAL_SWR_CLH_BYP |
        WCD938X_DIGITAL_CDC_TX0_CTL |
        WCD938X_DIGITAL_CDC_TX1_CTL |
        WCD938X_DIGITAL_CDC_TX2_CTL |
        WCD938X_DIGITAL_CDC_TX_RST |
        WCD938X_DIGITAL_CDC_REQ_CTL |
        WCD938X_DIGITAL_CDC_RST |
        WCD938X_DIGITAL_CDC_AMIC_CTL |
        WCD938X_DIGITAL_CDC_DMIC_CTL |
        WCD938X_DIGITAL_CDC_DMIC1_CTL |
        WCD938X_DIGITAL_CDC_DMIC2_CTL |
        WCD938X_DIGITAL_CDC_DMIC3_CTL |
        WCD938X_DIGITAL_CDC_DMIC4_CTL |
        WCD938X_DIGITAL_EFUSE_PRG_CTL |
        WCD938X_DIGITAL_EFUSE_CTL |
        WCD938X_DIGITAL_CDC_DMIC_RATE_1_2 |
        WCD938X_DIGITAL_CDC_DMIC_RATE_3_4 |
        WCD938X_DIGITAL_PDM_WD_CTL0 |
        WCD938X_DIGITAL_PDM_WD_CTL1 |
        WCD938X_DIGITAL_PDM_WD_CTL2 |
        WCD938X_DIGITAL_INTR_MODE |
        WCD938X_DIGITAL_INTR_MASK_0 |
        WCD938X_DIGITAL_INTR_MASK_1 |
        WCD938X_DIGITAL_INTR_MASK_2 |
        WCD938X_DIGITAL_INTR_CLEAR_0 |
        WCD938X_DIGITAL_INTR_CLEAR_1 |
        WCD938X_DIGITAL_INTR_CLEAR_2 |
        WCD938X_DIGITAL_INTR_LEVEL_0 |
        WCD938X_DIGITAL_INTR_LEVEL_1 |
        WCD938X_DIGITAL_INTR_LEVEL_2 |
        WCD938X_DIGITAL_INTR_SET_0 |
        WCD938X_DIGITAL_INTR_SET_1 |
        WCD938X_DIGITAL_INTR_SET_2 |
        WCD938X_DIGITAL_INTR_TEST_0 |
        WCD938X_DIGITAL_INTR_TEST_1 |
        WCD938X_DIGITAL_INTR_TEST_2 |
        WCD938X_DIGITAL_TX_MODE_DBG_EN |
        WCD938X_DIGITAL_TX_MODE_DBG_0_1 |
        WCD938X_DIGITAL_TX_MODE_DBG_2_3 |
        WCD938X_DIGITAL_LB_IN_SEL_CTL |
        WCD938X_DIGITAL_LOOP_BACK_MODE |
        WCD938X_DIGITAL_SWR_DAC_TEST |
        WCD938X_DIGITAL_SWR_HM_TEST_RX_0 |
        WCD938X_DIGITAL_SWR_HM_TEST_TX_0 |
        WCD938X_DIGITAL_SWR_HM_TEST_RX_1 |
        WCD938X_DIGITAL_SWR_HM_TEST_TX_1 |
        WCD938X_DIGITAL_SWR_HM_TEST_TX_2 |
        WCD938X_DIGITAL_PAD_CTL_SWR_0 |
        WCD938X_DIGITAL_PAD_CTL_SWR_1 |
        WCD938X_DIGITAL_I2C_CTL |
        WCD938X_DIGITAL_CDC_TX_TANGGU_SW_MODE |
        WCD938X_DIGITAL_EFUSE_TEST_CTL_0 |
        WCD938X_DIGITAL_EFUSE_TEST_CTL_1 |
        WCD938X_DIGITAL_PAD_CTL_PDM_RX0 |
        WCD938X_DIGITAL_PAD_CTL_PDM_RX1 |
        WCD938X_DIGITAL_PAD_CTL_PDM_TX0 |
        WCD938X_DIGITAL_PAD_CTL_PDM_TX1 |
        WCD938X_DIGITAL_PAD_CTL_PDM_TX2 |
        WCD938X_DIGITAL_PAD_INP_DIS_0 |
        WCD938X_DIGITAL_PAD_INP_DIS_1 |
        WCD938X_DIGITAL_DRIVE_STRENGTH_0 |
        WCD938X_DIGITAL_DRIVE_STRENGTH_1 |
        WCD938X_DIGITAL_DRIVE_STRENGTH_2 |
        WCD938X_DIGITAL_RX_DATA_EDGE_CTL |
        WCD938X_DIGITAL_TX_DATA_EDGE_CTL |
        WCD938X_DIGITAL_GPIO_MODE |
        WCD938X_DIGITAL_PIN_CTL_OE |
        WCD938X_DIGITAL_PIN_CTL_DATA_0 |
        WCD938X_DIGITAL_PIN_CTL_DATA_1 |
        WCD938X_DIGITAL_DIG_DEBUG_CTL |
        WCD938X_DIGITAL_DIG_DEBUG_EN |
        WCD938X_DIGITAL_ANA_CSR_DBG_ADD |
        WCD938X_DIGITAL_ANA_CSR_DBG_CTL |
        WCD938X_DIGITAL_SSP_DBG |
        WCD938X_DIGITAL_SPARE_0 |
        WCD938X_DIGITAL_SPARE_1 |
        WCD938X_DIGITAL_SPARE_2 |
        WCD938X_DIGITAL_TX_REQ_FB_CTL_0 |
        WCD938X_DIGITAL_TX_REQ_FB_CTL_1 |
        WCD938X_DIGITAL_TX_REQ_FB_CTL_2 |
        WCD938X_DIGITAL_TX_REQ_FB_CTL_3 |
        WCD938X_DIGITAL_TX_REQ_FB_CTL_4 |
        WCD938X_DIGITAL_DEM_BYPASS_DATA0 |
        WCD938X_DIGITAL_DEM_BYPASS_DATA1 |
        WCD938X_DIGITAL_DEM_BYPASS_DATA2 |
        WCD938X_DIGITAL_DEM_BYPASS_DATA3 => true,
        _ => false,
    }
}

unsafe extern "C" fn wcd938x_readonly_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WCD938X_ANA_MBHC_RESULT_1 |
        WCD938X_ANA_MBHC_RESULT_2 |
        WCD938X_ANA_MBHC_RESULT_3 |
        WCD938X_MBHC_MOISTURE_DET_FSM_STATUS |
        WCD938X_TX_1_2_SAR2_ERR |
        WCD938X_TX_1_2_SAR1_ERR |
        WCD938X_TX_3_4_SAR4_ERR |
        WCD938X_TX_3_4_SAR3_ERR |
        WCD938X_HPH_L_STATUS |
        WCD938X_HPH_R_STATUS |
        WCD938X_HPH_SURGE_HPHLR_SURGE_STATUS |
        WCD938X_EAR_STATUS_REG_1 |
        WCD938X_EAR_STATUS_REG_2 |
        WCD938X_MBHC_NEW_FSM_STATUS |
        WCD938X_MBHC_NEW_ADC_RESULT |
        WCD938X_DIE_CRACK_DIE_CRK_DET_OUT |
        WCD938X_AUX_INT_STATUS_REG |
        WCD938X_LDORXTX_INT_STATUS |
        WCD938X_DIGITAL_CHIP_ID0 |
        WCD938X_DIGITAL_CHIP_ID1 |
        WCD938X_DIGITAL_CHIP_ID2 |
        WCD938X_DIGITAL_CHIP_ID3 |
        WCD938X_DIGITAL_INTR_STATUS_0 |
        WCD938X_DIGITAL_INTR_STATUS_1 |
        WCD938X_DIGITAL_INTR_STATUS_2 |
        WCD938X_DIGITAL_INTR_CLEAR_0 |
        WCD938X_DIGITAL_INTR_CLEAR_1 |
        WCD938X_DIGITAL_INTR_CLEAR_2 |
        WCD938X_DIGITAL_SWR_HM_TEST_0 |
        WCD938X_DIGITAL_SWR_HM_TEST_1 |
        WCD938X_DIGITAL_EFUSE_T_DATA_0 |
        WCD938X_DIGITAL_EFUSE_T_DATA_1 |
        WCD938X_DIGITAL_PIN_STATUS_0 |
        WCD938X_DIGITAL_PIN_STATUS_1 |
        WCD938X_DIGITAL_MODE_STATUS_0 |
        WCD938X_DIGITAL_MODE_STATUS_1 |
        WCD938X_DIGITAL_EFUSE_REG_0 |
        WCD938X_DIGITAL_EFUSE_REG_1 |
        WCD938X_DIGITAL_EFUSE_REG_2 |
        WCD938X_DIGITAL_EFUSE_REG_3 |
        WCD938X_DIGITAL_EFUSE_REG_4 |
        WCD938X_DIGITAL_EFUSE_REG_5 |
        WCD938X_DIGITAL_EFUSE_REG_6 |
        WCD938X_DIGITAL_EFUSE_REG_7 |
        WCD938X_DIGITAL_EFUSE_REG_8 |
        WCD938X_DIGITAL_EFUSE_REG_9 |
        WCD938X_DIGITAL_EFUSE_REG_10 |
        WCD938X_DIGITAL_EFUSE_REG_11 |
        WCD938X_DIGITAL_EFUSE_REG_12 |
        WCD938X_DIGITAL_EFUSE_REG_13 |
        WCD938X_DIGITAL_EFUSE_REG_14 |
        WCD938X_DIGITAL_EFUSE_REG_15 |
        WCD938X_DIGITAL_EFUSE_REG_16 |
        WCD938X_DIGITAL_EFUSE_REG_17 |
        WCD938X_DIGITAL_EFUSE_REG_18 |
        WCD938X_DIGITAL_EFUSE_REG_19 |
        WCD938X_DIGITAL_EFUSE_REG_20 |
        WCD938X_DIGITAL_EFUSE_REG_21 |
        WCD938X_DIGITAL_EFUSE_REG_22 |
        WCD938X_DIGITAL_EFUSE_REG_23 |
        WCD938X_DIGITAL_EFUSE_REG_24 |
        WCD938X_DIGITAL_EFUSE_REG_25 |
        WCD938X_DIGITAL_EFUSE_REG_26 |
        WCD938X_DIGITAL_EFUSE_REG_27 |
        WCD938X_DIGITAL_EFUSE_REG_28 |
        WCD938X_DIGITAL_EFUSE_REG_29 |
        WCD938X_DIGITAL_EFUSE_REG_30 |
        WCD938X_DIGITAL_EFUSE_REG_31 => true,
        _ => false,
    }
}

unsafe extern "C" fn wcd938x_readable_register(dev: *mut device, reg: c_uint) -> bool {
    let ret = wcd938x_readonly_register(dev, reg);
    if !ret { return wcd938x_rdwr_register(dev, reg); }
    ret
}

unsafe extern "C" fn wcd938x_writeable_register(dev: *mut device, reg: c_uint) -> bool { wcd938x_rdwr_register(dev, reg) }

unsafe extern "C" fn wcd938x_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    if reg <= WCD938X_BASE_ADDRESS { return false; }
    if reg == WCD938X_DIGITAL_SWR_TX_CLK_RATE { return true; }
    if wcd938x_readonly_register(dev, reg) { return true; }
    false
}

static wcd938x_regmap_config: regmap_config = regmap_config {
    name: b"wcd938x_csr\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 8,
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wcd938x_defaults.as_ptr(),
    num_reg_defaults: wcd938x_defaults.len() as c_uint,
    max_register: WCD938X_MAX_REGISTER,
    readable_reg: Some(wcd938x_readable_register),
    writeable_reg: Some(wcd938x_writeable_register),
    volatile_reg: Some(wcd938x_volatile_register),
};

static wcd9380_slave_ops: sdw_slave_ops = sdw_slave_ops { update_status: Some(wcd_update_status), interrupt_callback: Some(wcd9380_interrupt_callback), bus_config: Some(wcd_bus_config) };

unsafe extern "C" fn wcd9380_probe(pdev: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let wcd = devm_kzalloc(dev, core::mem::size_of::<wcd938x_sdw_priv>(), GFP_KERNEL) as *mut wcd938x_sdw_priv;
    if wcd.is_null() { return -ENOMEM; }
    let ret: c_int;
    /*
     * Port map index starts with 0, however the data port for this codec
     * are from index 1
     */
    if of_property_present((*dev).of_node, b"qcom,tx-port-mapping\0".as_ptr() as *const c_char) {
        (*wcd).is_tx = true;
        ret = of_property_read_u32_array((*dev).of_node, b"qcom,tx-port-mapping\0".as_ptr() as *const c_char, (*pdev).m_port_map.as_mut_ptr().add(1), WCD938X_MAX_TX_SWR_PORTS);
    } else {
        ret = of_property_read_u32_array((*dev).of_node, b"qcom,rx-port-mapping\0".as_ptr() as *const c_char, (*pdev).m_port_map.as_mut_ptr().add(1), WCD938X_MAX_SWR_PORTS);
    }
    if ret < 0 { dev_info(dev, b"Static Port mapping not specified\n\0".as_ptr() as *const c_char); }
    (*wcd).sdev = pdev;
    dev_set_drvdata(dev, wcd as *mut c_void);
    (*pdev).prop.scp_int1_mask = SDW_SCP_INT1_IMPL_DEF | SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*pdev).prop.lane_control_support = true;
    (*pdev).prop.simple_clk_stop_capable = true;
    if (*wcd).is_tx {
        (*pdev).prop.source_ports = GENMASK((WCD938X_MAX_SWR_PORTS - 1) as c_uint, 0);
        (*pdev).prop.src_dpn_prop = wcd938x_dpn_prop.as_mut_ptr();
        (*wcd).ch_info = wcd938x_sdw_tx_ch_info.as_ptr();
        (*pdev).prop.wake_capable = true;
    } else {
        (*pdev).prop.sink_ports = GENMASK((WCD938X_MAX_SWR_PORTS - 1) as c_uint, 0);
        (*pdev).prop.sink_dpn_prop = wcd938x_dpn_prop.as_mut_ptr();
        (*wcd).ch_info = wcd938x_sdw_rx_ch_info.as_ptr();
    }
    if (*wcd).is_tx {
        (*wcd).regmap = devm_regmap_init_sdw(pdev, &wcd938x_regmap_config);
        if IS_ERR((*wcd).regmap as *const c_void) { return dev_err_probe(dev, PTR_ERR((*wcd).regmap as *const c_void), b"Regmap init failed\n\0".as_ptr() as *const c_char); }
        /* Start in cache-only until device is enumerated */
        regcache_cache_only((*wcd).regmap, true);
    }
    let ret = component_add(dev, &wcd_sdw_component_ops as *const c_void);
    if ret != 0 { return ret; }
    /* Set suspended until aggregate device is bind */
    pm_runtime_set_suspended(dev);
    0
}

unsafe extern "C" fn wcd9380_remove(pdev: *mut sdw_slave) { let dev = &mut (*pdev).dev as *mut device; component_del(dev, &wcd_sdw_component_ops as *const c_void); }

static wcd9380_slave_id: &[sdw_device_id] = &[
    /* SDW_SLAVE_ENTRY(0x0217, 0x10d, 0), */
    sdw_device_id { _private: [] },
];
// MODULE_DEVICE_TABLE(sdw, wcd9380_slave_id);

unsafe extern "C" fn wcd938x_sdw_runtime_suspend(dev: *mut device) -> c_int {
    let wcd = dev_get_drvdata(dev) as *mut wcd938x_sdw_priv;
    if !(*wcd).regmap.is_null() { regcache_cache_only((*wcd).regmap, true); regcache_mark_dirty((*wcd).regmap); }
    0
}

unsafe extern "C" fn wcd938x_sdw_runtime_resume(dev: *mut device) -> c_int {
    let wcd = dev_get_drvdata(dev) as *mut wcd938x_sdw_priv;
    if !(*wcd).regmap.is_null() {
        regcache_cache_only((*wcd).regmap, false);
        let ret = regcache_sync((*wcd).regmap);
        if ret != 0 { regcache_cache_only((*wcd).regmap, true); regcache_mark_dirty((*wcd).regmap); return ret; }
    }
    pm_runtime_mark_last_busy(dev);
    0
}

static wcd938x_sdw_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };
// RUNTIME_PM_OPS(wcd938x_sdw_runtime_suspend, wcd938x_sdw_runtime_resume, NULL)

static mut wcd9380_codec_driver: sdw_driver = sdw_driver {
    probe: Some(wcd9380_probe),
    remove: Some(wcd9380_remove),
    ops: &wcd9380_slave_ops,
    id_table: wcd9380_slave_id.as_ptr(),
    driver: device_driver { name: b"wcd9380-codec\0".as_ptr() as *const c_char, pm: &wcd938x_sdw_pm_ops },
};
// module_sdw_driver(wcd9380_codec_driver);
// MODULE_DESCRIPTION("WCD938X SDW codec driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
