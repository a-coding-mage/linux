// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2017 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

/* SDB : Status Data Block */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum usb_pd_ext_sdb_fields {
    USB_PD_EXT_SDB_INTERNAL_TEMP = 0,
    USB_PD_EXT_SDB_PRESENT_INPUT,
    USB_PD_EXT_SDB_PRESENT_BATT_INPUT,
    USB_PD_EXT_SDB_EVENT_FLAGS,
    USB_PD_EXT_SDB_TEMP_STATUS,
    USB_PD_EXT_SDB_DATA_SIZE,
}

/* Event Flags */
pub const USB_PD_EXT_SDB_EVENT_OCP: u32 = 1u32 << 1;
pub const USB_PD_EXT_SDB_EVENT_OTP: u32 = 1u32 << 2;
pub const USB_PD_EXT_SDB_EVENT_OVP: u32 = 1u32 << 3;
pub const USB_PD_EXT_SDB_EVENT_CF_CV_MODE: u32 = 1u32 << 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
