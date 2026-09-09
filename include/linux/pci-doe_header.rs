/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Data Object Exchange
 *	PCIe r6.0, sec 6.30 DOE
 *
 * Copyright (C) 2021 Huawei
 *     Jonathan Cameron <Jonathan.Cameron@huawei.com>
 *
 * Copyright (C) 2022 Intel Corporation
 *	Ira Weiny <ira.weiny@intel.com>
 */

// The original header guard is omitted from executable Rust syntax.

pub struct pci_doe_mb {
    _private: [u8; 0],
}

pub const PCI_DOE_FEATURE_DISCOVERY: u32 = 0;
pub const PCI_DOE_FEATURE_CMA: u32 = 1;
pub const PCI_DOE_FEATURE_SSESSION: u32 = 2;

extern "C" {
    pub fn pci_find_doe_mailbox(
        pdev: *mut pci_dev,
        vendor: u16,
        type_: u8,
    ) -> *mut pci_doe_mb;

    pub fn pci_doe(
        doe_mb: *mut pci_doe_mb,
        vendor: u16,
        type_: u8,
        request: *const core::ffi::c_void,
        request_sz: usize,
        response: *mut core::ffi::c_void,
        response_sz: usize,
    ) -> core::ffi::c_int;
}

// `struct pci_dev` is supplied by another header/dependency.
pub enum pci_dev {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
