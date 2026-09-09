/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2020 Intel Corporation. All rights reserved. */

/* Register Block Identifier (RBI) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cxl_regloc_type {
    CXL_REGLOC_RBI_EMPTY = 0,
    CXL_REGLOC_RBI_COMPONENT,
    CXL_REGLOC_RBI_VIRT,
    CXL_REGLOC_RBI_MEMDEV,
    CXL_REGLOC_RBI_PMU,
    CXL_REGLOC_RBI_TYPES,
}

#[repr(C)]
pub struct cxl_register_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn cxl_pci_setup_regs(
        pdev: *mut pci_dev,
        type_: cxl_regloc_type,
        map: *mut cxl_register_map,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
