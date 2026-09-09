// SPDX-License-Identifier: GPL-2.0-only
/*
 * CXL Error INJection support. Used by CXL core to inject
 * protocol errors into CXL ports.
 *
 * Copyright (C) 2023 Advanced Micro Devices, Inc.
 *
 * Author: Ben Cheatham <benjamin.cheatham@amd.com>
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_bus {
    pub number: u8,
}

#[repr(C)]
pub struct pci_host_bridge {
    pub domain_nr: c_int,
}

#[repr(C)]
pub struct pci_dev {
    pub bus: *mut pci_bus,
    pub devfn: u32,
}

extern "C" {
    pub static mut einj_initialized: bool;

    fn einj_get_available_error_type(available_error_type: *mut u32, type_: u32) -> c_int;
    fn einj_is_cxl_error_type(type_: u64) -> bool;
    fn einj_validate_error_type(type_: u64) -> c_int;
    fn einj_cxl_rch_error_inject(
        type_: u64,
        param1: u64,
        param2: u64,
        param3: u64,
        param4: u64,
        param5: u64,
    ) -> c_int;
    fn einj_error_inject(
        type_: u64,
        param1: u64,
        param2: u64,
        param3: u64,
        param4: u64,
        param5: u64,
    ) -> c_int;
    fn pci_find_host_bridge(bus: *mut pci_bus) -> *mut pci_host_bridge;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
}

const ACPI_EINJ_GET_ERROR_TYPE: u32 = 0;
const ACPI_EINJ_CXL_CACHE_CORRECTABLE: u64 = 1 << 9;
const ACPI_EINJ_CXL_CACHE_UNCORRECTABLE: u64 = 1 << 10;
const ACPI_EINJ_CXL_CACHE_FATAL: u64 = 1 << 11;
const ACPI_EINJ_CXL_MEM_CORRECTABLE: u64 = 1 << 12;
const ACPI_EINJ_CXL_MEM_UNCORRECTABLE: u64 = 1 << 13;
const ACPI_EINJ_CXL_MEM_FATAL: u64 = 1 << 14;
const PCI_DOMAIN_NR_NOT_SET: c_int = -1;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;

struct EinjCxlErrorTypeString {
    mask: u32,
    str_: *const c_char,
}

static EINJ_CXL_ERROR_TYPE_STRING: [EinjCxlErrorTypeString; 6] = [
    EinjCxlErrorTypeString { mask: ACPI_EINJ_CXL_CACHE_CORRECTABLE as u32, str_: c"CXL.cache Protocol Correctable".as_ptr() },
    EinjCxlErrorTypeString { mask: ACPI_EINJ_CXL_CACHE_UNCORRECTABLE as u32, str_: c"CXL.cache Protocol Uncorrectable non-fatal".as_ptr() },
    EinjCxlErrorTypeString { mask: ACPI_EINJ_CXL_CACHE_FATAL as u32, str_: c"CXL.cache Protocol Uncorrectable fatal".as_ptr() },
    EinjCxlErrorTypeString { mask: ACPI_EINJ_CXL_MEM_CORRECTABLE as u32, str_: c"CXL.mem Protocol Correctable".as_ptr() },
    EinjCxlErrorTypeString { mask: ACPI_EINJ_CXL_MEM_UNCORRECTABLE as u32, str_: c"CXL.mem Protocol Uncorrectable non-fatal".as_ptr() },
    EinjCxlErrorTypeString { mask: ACPI_EINJ_CXL_MEM_FATAL as u32, str_: c"CXL.mem Protocol Uncorrectable fatal".as_ptr() },
];

pub unsafe extern "C" fn einj_cxl_available_error_type_show(
    m: *mut seq_file,
    _v: *mut c_void,
) -> c_int {
    let mut available_error_type: u32 = 0;
    let rc = einj_get_available_error_type(
        &mut available_error_type,
        ACPI_EINJ_GET_ERROR_TYPE,
    );
    if rc != 0 {
        return rc;
    }

    for pos in 0..EINJ_CXL_ERROR_TYPE_STRING.len() {
        let cxl_err = ACPI_EINJ_CXL_CACHE_CORRECTABLE << pos;

        if available_error_type as u64 & cxl_err != 0 {
            let fmt = c"0x%08x\t%s\n";
            seq_printf(
                m,
                fmt.as_ptr(),
                EINJ_CXL_ERROR_TYPE_STRING[pos].mask,
                EINJ_CXL_ERROR_TYPE_STRING[pos].str_,
            );
        }
    }

    0
}

unsafe fn cxl_dport_get_sbdf(dport_dev: *mut pci_dev, sbdf: *mut u64) -> c_int {
    let pbus = (*dport_dev).bus;
    let bridge = pci_find_host_bridge(pbus);

    if bridge.is_null() {
        return -ENODEV;
    }

    let mut seg: u64 = 0;
    if (*bridge).domain_nr != PCI_DOMAIN_NR_NOT_SET {
        seg = (*bridge).domain_nr as u64;
    }

    let bus = (*pbus).number as u64;
    *sbdf = (seg << 24) | (bus << 16) | (((*dport_dev).devfn as u64) << 8);

    0
}

pub unsafe extern "C" fn einj_cxl_inject_rch_error(rcrb: u64, type_: u64) -> c_int {
    if !einj_is_cxl_error_type(type_) {
        return -EINVAL;
    }

    let rc = einj_validate_error_type(type_);
    if rc != 0 {
        return rc;
    }

    einj_cxl_rch_error_inject(type_, 0x2, rcrb, u64::MAX, 0, 0)
}

pub unsafe extern "C" fn einj_cxl_inject_error(
    dport: *mut pci_dev,
    type_: u64,
) -> c_int {
    let mut param4: u64 = 0;

    if !einj_is_cxl_error_type(type_) {
        return -EINVAL;
    }

    let rc = einj_validate_error_type(type_);
    if rc != 0 {
        return rc;
    }

    let rc = cxl_dport_get_sbdf(dport, &mut param4);
    if rc != 0 {
        return rc;
    }

    einj_error_inject(type_, 0x4, 0, 0, 0, param4)
}

pub unsafe extern "C" fn einj_cxl_is_initialized() -> bool {
    einj_initialized
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
