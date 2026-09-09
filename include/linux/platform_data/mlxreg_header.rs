/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/*
 * Copyright (C) 2017-2020 Mellanox Technologies Ltd.
 */

use core::ffi::c_void;

pub const MLXREG_CORE_LABEL_MAX_SIZE: usize = 32;
pub const MLXREG_CORE_WD_FEATURE_NOWAYOUT: u32 = 1 << 0;
pub const MLXREG_CORE_WD_FEATURE_START_AT_BOOT: u32 = 1 << 1;

/**
 * enum mlxreg_wdt_type - type of HW watchdog
 *
 * @MLX_WDT_TYPE1: HW watchdog implementation in old systems.
 * @MLX_WDT_TYPE2: All new systems have TYPE2 HW watchdog.
 * @MLX_WDT_TYPE3: HW watchdog that can exist on all systems with new CPLD.
 *   TYPE3 is selected by WD capability bit.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mlxreg_wdt_type {
    MLX_WDT_TYPE1,
    MLX_WDT_TYPE2,
    MLX_WDT_TYPE3,
}

/** Hotplug entry kind. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mlxreg_hotplug_kind {
    MLXREG_HOTPLUG_DEVICE_NA = 0,
    MLXREG_HOTPLUG_LC_PRESENT = 1,
    MLXREG_HOTPLUG_LC_VERIFIED = 2,
    MLXREG_HOTPLUG_LC_POWERED = 3,
    MLXREG_HOTPLUG_LC_SYNCED = 4,
    MLXREG_HOTPLUG_LC_READY = 5,
    MLXREG_HOTPLUG_LC_ACTIVE = 6,
    MLXREG_HOTPLUG_LC_THERMAL = 7,
}

/** Hotplug device action required for driver's connectivity. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mlxreg_hotplug_device_action {
    MLXREG_HOTPLUG_DEVICE_DEFAULT_ACTION = 0,
    MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION = 1,
    MLXREG_HOTPLUG_DEVICE_NO_ACTION = 2,
}

#[repr(C)]
pub struct mlxreg_core_hotplug_notifier {
    pub identity: [i8; MLXREG_CORE_LABEL_MAX_SIZE],
    pub handle: *mut c_void,
    pub user_handler: Option<unsafe extern "C" fn(*mut c_void, mlxreg_hotplug_kind, u8) -> i32>,
}

#[repr(C)]
pub struct mlxreg_hotplug_device {
    pub adapter: *mut i2c_adapter,
    pub client: *mut i2c_client,
    pub brdinfo: *mut i2c_board_info,
    pub nr: i32,
    pub pdev: *mut platform_device,
    pub action: mlxreg_hotplug_device_action,
    pub handle: *mut c_void,
    pub user_handler: Option<unsafe extern "C" fn(*mut c_void, mlxreg_hotplug_kind, u8) -> i32>,
    pub notifier: *mut mlxreg_core_hotplug_notifier,
}

#[repr(C)]
pub struct mlxreg_core_data {
    pub label: [i8; MLXREG_CORE_LABEL_MAX_SIZE],
    pub reg: u32,
    pub mask: u32,
    pub bit: u32,
    pub capability: u32,
    pub reg_prsnt: u32,
    pub reg_sync: u32,
    pub reg_pwr: u32,
    pub reg_ena: u32,
    pub mode: umode_t,
    pub np: *mut device_node,
    pub hpdev: mlxreg_hotplug_device,
    pub notifier: *mut mlxreg_core_hotplug_notifier,
    pub health_cntr: u32,
    pub attached: bool,
    pub regnum: u8,
    pub slot: u8,
    pub secured: u8,
}

#[repr(C)]
pub struct mlxreg_core_item {
    pub data: *mut mlxreg_core_data,
    pub kind: mlxreg_hotplug_kind,
    pub aggr_mask: u32,
    pub reg: u32,
    pub mask: u32,
    pub capability: u32,
    pub cache: u32,
    pub count: u8,
    pub ind: u8,
    pub inversed: u8,
    pub health: u8,
}

#[repr(C)]
pub struct mlxreg_core_platform_data {
    pub data: *mut mlxreg_core_data,
    pub regmap: *mut c_void,
    pub counter: i32,
    pub features: u32,
    pub version: u32,
    pub identity: [i8; MLXREG_CORE_LABEL_MAX_SIZE],
    pub capability: u32,
}

#[repr(C)]
pub struct mlxreg_core_hotplug_platform_data {
    pub items: *mut mlxreg_core_item,
    pub irq: i32,
    pub regmap: *mut c_void,
    pub count: i32,
    pub cell: u32,
    pub mask: u32,
    pub cell_low: u32,
    pub mask_low: u32,
    pub deferred_nr: i32,
    pub shift_nr: i32,
    pub addr: *mut c_void,
    pub handle: *mut c_void,
    pub completion_notify: Option<unsafe extern "C" fn(*mut c_void, i32) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
