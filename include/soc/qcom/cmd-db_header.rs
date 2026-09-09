/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

use core::ffi::{c_char, c_void};

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cmd_db_hw_type {
    CMD_DB_HW_INVALID = 0,
    CMD_DB_HW_MIN = 3,
    CMD_DB_HW_ARC = 3,
    CMD_DB_HW_VRM = 4,
    CMD_DB_HW_BCM = 5,
    CMD_DB_HW_MAX = 5,
    CMD_DB_HW_ALL = 0xff,
}

// CONFIG_QCOM_COMMAND_DB is a build-time configuration condition from the C header.
#[cfg(feature = "config_qcom_command_db")]
unsafe extern "C" {
    pub fn cmd_db_read_addr(resource_id: *const c_char) -> u32;

    pub fn cmd_db_read_aux_data(resource_id: *const c_char, len: *mut usize) -> *const c_void;

    pub fn cmd_db_match_resource_addr(addr1: u32, addr2: u32) -> bool;

    pub fn cmd_db_read_slave_id(resource_id: *const c_char) -> cmd_db_hw_type;

    pub fn cmd_db_ready() -> i32;
}

#[cfg(not(feature = "config_qcom_command_db"))]
pub unsafe fn cmd_db_read_addr(_resource_id: *const c_char) -> u32 {
    0
}

#[cfg(not(feature = "config_qcom_command_db"))]
pub unsafe fn cmd_db_read_aux_data(_resource_id: *const c_char, _len: *mut usize) -> *const c_void {
    // ERR_PTR(-ENODEV), with ENODEV supplied by the Linux error definitions.
    (-19isize) as *const c_void
}

#[cfg(not(feature = "config_qcom_command_db"))]
pub unsafe fn cmd_db_match_resource_addr(_addr1: u32, _addr2: u32) -> bool {
    false
}

#[cfg(not(feature = "config_qcom_command_db"))]
pub unsafe fn cmd_db_read_slave_id(_resource_id: *const c_char) -> cmd_db_hw_type {
    // -ENODEV, with ENODEV supplied by the Linux error definitions.
    (-19i32) as cmd_db_hw_type
}

#[cfg(not(feature = "config_qcom_command_db"))]
pub unsafe fn cmd_db_ready() -> i32 {
    // -ENODEV, with ENODEV supplied by the Linux error definitions.
    -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
