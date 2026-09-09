/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for Intel extcon hardware
 *
 * Copyright (C) 2019 Intel Corporation. All rights reserved.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum extcon_intel_usb_id {
    INTEL_USB_ID_OTG,
    INTEL_USB_ID_GND,
    INTEL_USB_ID_FLOAT,
    INTEL_USB_RID_A,
    INTEL_USB_RID_B,
    INTEL_USB_RID_C,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
