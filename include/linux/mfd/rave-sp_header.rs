/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * Core definitions for RAVE SP MFD driver.
 *
 * Copyright (C) 2017 Zodiac Inflight Innovations
 */

// Dependency supplied by the surrounding kernel translation: linux/notifier.h

#[repr(u32)]
pub enum rave_sp_command {
    RAVE_SP_CMD_GET_FIRMWARE_VERSION = 0x20,
    RAVE_SP_CMD_GET_BOOTLOADER_VERSION = 0x21,
    RAVE_SP_CMD_BOOT_SOURCE = 0x26,
    RAVE_SP_CMD_GET_BOARD_COPPER_REV = 0x2B,
    RAVE_SP_CMD_GET_GPIO_STATE = 0x2F,

    RAVE_SP_CMD_STATUS = 0xA0,
    RAVE_SP_CMD_SW_WDT = 0xA1,
    RAVE_SP_CMD_PET_WDT = 0xA2,
    RAVE_SP_CMD_RMB_EEPROM = 0xA4,
    RAVE_SP_CMD_SET_BACKLIGHT = 0xA6,
    RAVE_SP_CMD_RESET = 0xA7,
    RAVE_SP_CMD_RESET_REASON = 0xA8,

    RAVE_SP_CMD_REQ_COPPER_REV = 0xB6,
    RAVE_SP_CMD_GET_I2C_DEVICE_STATUS = 0xBA,
    RAVE_SP_CMD_GET_SP_SILICON_REV = 0xB9,
    RAVE_SP_CMD_CONTROL_EVENTS = 0xBB,

    RAVE_SP_EVNT_BASE = 0xE0,
}

#[repr(C)]
pub struct rave_sp {
    _private: [u8; 0],
}

#[inline]
pub fn rave_sp_action_pack(event: u8, value: u8) -> core::ffi::c_ulong {
    ((value as core::ffi::c_ulong) << 8) | event as core::ffi::c_ulong
}

#[inline]
pub fn rave_sp_action_unpack_event(action: core::ffi::c_ulong) -> u8 {
    action as u8
}

#[inline]
pub fn rave_sp_action_unpack_value(action: core::ffi::c_ulong) -> u8 {
    (action >> 8) as u8
}

extern "C" {
    pub fn rave_sp_exec(
        sp: *mut rave_sp,
        data: *mut core::ffi::c_void,
        data_size: usize,
        reply_data: *mut core::ffi::c_void,
        reply_data_size: usize,
    ) -> core::ffi::c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    pub fn devm_rave_sp_register_event_notifier(
        dev: *mut device,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
