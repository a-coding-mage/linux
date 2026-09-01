/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2025, Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

/* enum sdw_slave_status is provided by an external dependency. */
pub type sdw_slave_status = c_uint;

/* struct component_ops is provided by an external dependency. */
#[repr(C)]
pub struct component_ops {
    _private: [u8; 0],
}

pub const WCD_MAX_MICBIAS: usize = 4;

#[repr(C)]
pub struct wcd_sdw_ch_info {
    pub port_num: c_int,
    pub ch_mask: c_uint,
    pub master_ch_mask: c_uint,
}

macro_rules! WCD_SDW_CH {
    ($id:expr, $pn:expr, $cmask:expr) => {
        (
            $id,
            wcd_sdw_ch_info {
                port_num: $pn,
                ch_mask: $cmask,
                master_ch_mask: $cmask,
            },
        )
    };
}

#[repr(C)]
pub struct wcd_common {
    pub dev: *mut device,
    pub max_bias: c_int,
    pub micb_mv: [u32; WCD_MAX_MICBIAS],
    pub micb_vout: [u32; WCD_MAX_MICBIAS],
}

unsafe extern "C" {
    pub static wcd_sdw_component_ops: component_ops;

    pub fn wcd_get_micb_vout_ctl_val(dev: *mut device, micb_mv: u32) -> c_int;
    pub fn wcd_dt_parse_micbias_info(common: *mut wcd_common) -> c_int;
    pub fn wcd_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int;
    pub fn wcd_bus_config(slave: *mut sdw_slave, params: *mut sdw_bus_params) -> c_int;
    pub fn wcd_interrupt_callback(
        slave: *mut sdw_slave,
        slave_irq: *mut irq_domain,
        wcd_intr_status0: c_uint,
        wcd_intr_status1: c_uint,
        wcd_intr_status2: c_uint,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
