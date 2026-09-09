/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Surface DTX (clipboard detachment system driver) user-space interface.
 *
 * Definitions, structs, and IOCTLs for the /dev/surface/dtx misc device. This
 * device allows user-space to control the clipboard detachment process on
 * Surface Book series devices.
 *
 * Copyright (C) 2020-2021 Maximilian Luz <luzmaximilian@gmail.com>
 */

/* Status/error categories */
pub const SDTX_CATEGORY_STATUS: u16 = 0x0000;
pub const SDTX_CATEGORY_RUNTIME_ERROR: u16 = 0x1000;
pub const SDTX_CATEGORY_HARDWARE_ERROR: u16 = 0x2000;
pub const SDTX_CATEGORY_UNKNOWN: u16 = 0xf000;

pub const SDTX_CATEGORY_MASK: u16 = 0xf000;
pub const fn sdtx_category(value: u16) -> u16 { value & SDTX_CATEGORY_MASK }
pub const fn sdtx_status(code: u16) -> u16 { code | SDTX_CATEGORY_STATUS }
pub const fn sdtx_err_rt(code: u16) -> u16 { code | SDTX_CATEGORY_RUNTIME_ERROR }
pub const fn sdtx_err_hw(code: u16) -> u16 { code | SDTX_CATEGORY_HARDWARE_ERROR }
pub const fn sdtx_unknown(code: u16) -> u16 { code | SDTX_CATEGORY_UNKNOWN }
pub const fn sdtx_success(value: u16) -> bool { sdtx_category(value) == SDTX_CATEGORY_STATUS }

/* Latch status values */
pub const SDTX_LATCH_CLOSED: u16 = sdtx_status(0x00);
pub const SDTX_LATCH_OPENED: u16 = sdtx_status(0x01);

/* Base state values */
pub const SDTX_BASE_DETACHED: u16 = sdtx_status(0x00);
pub const SDTX_BASE_ATTACHED: u16 = sdtx_status(0x01);

/* Runtime errors (non-critical) */
pub const SDTX_DETACH_NOT_FEASIBLE: u16 = sdtx_err_rt(0x01);
pub const SDTX_DETACH_TIMEDOUT: u16 = sdtx_err_rt(0x02);

/* Hardware errors (critical) */
pub const SDTX_ERR_FAILED_TO_OPEN: u16 = sdtx_err_hw(0x01);
pub const SDTX_ERR_FAILED_TO_REMAIN_OPEN: u16 = sdtx_err_hw(0x02);
pub const SDTX_ERR_FAILED_TO_CLOSE: u16 = sdtx_err_hw(0x03);

/* Base types */
pub const SDTX_DEVICE_TYPE_HID: u16 = 0x0100;
pub const SDTX_DEVICE_TYPE_SSH: u16 = 0x0200;
pub const SDTX_DEVICE_TYPE_MASK: u16 = 0x0f00;
pub const fn sdtx_device_type(value: u16) -> u16 { value & SDTX_DEVICE_TYPE_MASK }
pub const fn sdtx_base_type_hid(id: u16) -> u16 { id | SDTX_DEVICE_TYPE_HID }
pub const fn sdtx_base_type_ssh(id: u16) -> u16 { id | SDTX_DEVICE_TYPE_SSH }

#[repr(u16)]
pub enum SdtxDeviceMode {
    SDTX_DEVICE_MODE_TABLET = 0x00,
    SDTX_DEVICE_MODE_LAPTOP = 0x01,
    SDTX_DEVICE_MODE_STUDIO = 0x02,
}

#[repr(C, packed)]
pub struct SdtxEvent {
    pub length: u16,
    pub code: u16,
    pub data: [u8; 0],
}

#[repr(u16)]
pub enum SdtxEventCode {
    SDTX_EVENT_REQUEST = 1,
    SDTX_EVENT_CANCEL = 2,
    SDTX_EVENT_BASE_CONNECTION = 3,
    SDTX_EVENT_LATCH_STATUS = 4,
    SDTX_EVENT_DEVICE_MODE = 5,
}

#[repr(C, packed)]
pub struct SdtxBaseInfo {
    pub state: u16,
    pub base_id: u16,
}

/* IOCTLs. Values use Linux _IO/_IOR encoding (type 0xa5). */
pub const SDTX_IOCTL_EVENTS_ENABLE: u32 = 0x0000_a521;
pub const SDTX_IOCTL_EVENTS_DISABLE: u32 = 0x0000_a522;
pub const SDTX_IOCTL_LATCH_LOCK: u32 = 0x0000_a523;
pub const SDTX_IOCTL_LATCH_UNLOCK: u32 = 0x0000_a524;
pub const SDTX_IOCTL_LATCH_REQUEST: u32 = 0x0000_a525;
pub const SDTX_IOCTL_LATCH_CONFIRM: u32 = 0x0000_a526;
pub const SDTX_IOCTL_LATCH_HEARTBEAT: u32 = 0x0000_a527;
pub const SDTX_IOCTL_LATCH_CANCEL: u32 = 0x0000_a528;
pub const SDTX_IOCTL_GET_BASE_INFO: u32 = 0x8004_a529;
pub const SDTX_IOCTL_GET_DEVICE_MODE: u32 = 0x8002_a52a;
pub const SDTX_IOCTL_GET_LATCH_STATUS: u32 = 0x8002_a52b;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
