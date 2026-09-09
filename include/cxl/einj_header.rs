/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CXL protocol Error INJection support.
 *
 * Copyright (c) 2023 Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Author: Ben Cheatham <benjamin.cheatham@amd.com>
 */

// C dependency: linux/errno.h
// C dependency: linux/types.h

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

// C conditional: IS_ENABLED(CONFIG_ACPI_APEI_EINJ_CXL)
#[cfg(feature = "CONFIG_ACPI_APEI_EINJ_CXL")]
extern "C" {
    pub fn einj_cxl_available_error_type_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
    pub fn einj_cxl_inject_error(dport_dev: *mut pci_dev, type_: u64) -> i32;
    pub fn einj_cxl_inject_rch_error(rcrb: u64, type_: u64) -> i32;
    pub fn einj_cxl_is_initialized() -> bool;
}

// C fallback: !IS_ENABLED(CONFIG_ACPI_APEI_EINJ_CXL)
// linux/errno.h: ENXIO == 6
#[cfg(not(feature = "CONFIG_ACPI_APEI_EINJ_CXL"))]
#[inline]
pub unsafe fn einj_cxl_available_error_type_show(
    _m: *mut seq_file,
    _v: *mut core::ffi::c_void,
) -> i32 {
    -6
}

#[cfg(not(feature = "CONFIG_ACPI_APEI_EINJ_CXL"))]
#[inline]
pub unsafe fn einj_cxl_inject_error(_dport_dev: *mut pci_dev, _type_: u64) -> i32 {
    -6
}

#[cfg(not(feature = "CONFIG_ACPI_APEI_EINJ_CXL"))]
#[inline]
pub unsafe fn einj_cxl_inject_rch_error(_rcrb: u64, _type_: u64) -> i32 {
    -6
}

#[cfg(not(feature = "CONFIG_ACPI_APEI_EINJ_CXL"))]
#[inline]
pub unsafe fn einj_cxl_is_initialized() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
