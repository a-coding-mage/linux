/* SPDX-License-Identifier: GPL-2.0
 *
 * sound/q6usboffload.h -- QDSP6 USB offload
 *
 * Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

/**
 * struct q6usb_offload - USB backend DAI link offload parameters
 * @dev: dev handle to usb be
 * @domain: allocated iommu domain
 * @intr_num: usb interrupter number
 * @sid: streamID for iommu
 **/
#[repr(C)]
pub struct q6usb_offload {
    pub dev: *mut device,
    pub domain: *mut iommu_domain,
    pub intr_num: u16,
    pub sid: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
