/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Core interface for Renesas Synchronization Management Unit (SMU) devices.
 *
 * Copyright (C) 2021 Integrated Device Technology, Inc., a Renesas Company.
 */

// C header guard: __LINUX_MFD_RSMU_H
// C dependencies (device, regmap, mutex, and u32) are supplied by other files.

pub const RSMU_MAX_WRITE_COUNT: u32 = 255;
pub const RSMU_MAX_READ_COUNT: u32 = 255;

/* The supported devices are ClockMatrix, Sabre and SnowLotus */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rsmu_type {
    RSMU_CM = 0x34000,
    RSMU_SABRE = 0x33810,
    RSMU_SL = 0x19850,
}

/**
 * struct rsmu_ddata - device data structure for sub devices.
 *
 * @dev:    i2c/spi device.
 * @regmap: i2c/spi bus access.
 * @lock:   mutex used by sub devices to make sure a series of
 *          bus access requests are not interrupted.
 * @type:   RSMU device type.
 * @page:   i2c/spi bus driver internal use only.
 */
#[repr(C)]
pub struct rsmu_ddata {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub type_: rsmu_type,
    pub page: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
