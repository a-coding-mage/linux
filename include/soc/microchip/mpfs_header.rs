/* SPDX-License-Identifier: GPL-2.0 */
/*
 *
 * Microchip PolarFire SoC (MPFS)
 *
 * Copyright (c) 2020 Microchip Corporation. All rights reserved.
 *
 * Author: Conor Dooley <conor.dooley@microchip.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external to this header.

#[repr(C)]
pub struct mpfs_sys_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mpfs_mss_msg {
    pub cmd_opcode: u8,
    pub cmd_data_size: u16,
    pub response: *mut mpfs_mss_response,
    pub cmd_data: *mut u8,
    pub mbox_offset: u16,
    pub resp_offset: u16,
}

#[repr(C)]
pub struct mpfs_mss_response {
    pub resp_status: u32,
    pub resp_msg: *mut u32,
    pub resp_size: u16,
}

// Equivalent declarations for IS_ENABLED(CONFIG_POLARFIRE_SOC_SYS_CTRL).
extern "C" {
    pub fn mpfs_blocking_transaction(
        mpfs_client: *mut mpfs_sys_controller,
        msg: *mut mpfs_mss_msg,
    ) -> i32;

    pub fn mpfs_sys_controller_get(dev: *mut device) -> *mut mpfs_sys_controller;

    pub fn mpfs_sys_controller_get_flash(
        mpfs_client: *mut mpfs_sys_controller,
    ) -> *mut mtd_info;
}

// Equivalent declaration for IS_ENABLED(CONFIG_MCHP_CLK_MPFS).
// When IS_ENABLED(CONFIG_RESET_POLARFIRE_SOC) is true:
extern "C" {
    pub fn mpfs_reset_controller_register(
        clk_dev: *mut device,
        map: *mut regmap,
    ) -> i32;
}

// When IS_ENABLED(CONFIG_RESET_POLARFIRE_SOC) is false, the C header provides:
// static inline int mpfs_reset_controller_register(struct device *clk_dev,
//                                                 void __iomem *base) { return 0; }
#[inline]
pub unsafe fn mpfs_reset_controller_register_disabled(
    _clk_dev: *mut device,
    _base: *mut core::ffi::c_void,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
