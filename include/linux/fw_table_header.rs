/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  fw_tables.h - Parsing support for ACPI and ACPI-like tables provided by
 *                platform or device firmware
 *
 *  Copyright (C) 2001 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2023 Intel Corp.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::mem::ManuallyDrop;

pub type AcpiTblEntryHandler = unsafe extern "C" fn(
    header: *mut AcpiSubtableHeaders,
    end: c_ulong,
) -> c_int;

pub type AcpiTblEntryHandlerArg = unsafe extern "C" fn(
    header: *mut AcpiSubtableHeaders,
    arg: *mut core::ffi::c_void,
    end: c_ulong,
) -> c_int;

#[repr(C)]
pub struct AcpiSubtableProc {
    pub id: c_int,
    pub handler: Option<AcpiTblEntryHandler>,
    pub handler_arg: Option<AcpiTblEntryHandlerArg>,
    pub arg: *mut core::ffi::c_void,
    pub count: c_int,
}

#[repr(C)]
pub union FwTableHeader {
    pub acpi: ManuallyDrop<AcpiTableHeader>,
    pub cdat: ManuallyDrop<AcpiTableCdat>,
}

#[repr(C)]
pub union AcpiSubtableHeaders {
    pub common: ManuallyDrop<AcpiSubtableHeader>,
    pub hmat: ManuallyDrop<AcpiHmatStructure>,
    pub prmt: ManuallyDrop<AcpiPrmtModuleHeader>,
    pub cedt: ManuallyDrop<AcpiCedtHeader>,
    pub cdat: ManuallyDrop<AcpiCdatHeader>,
}

extern "C" {
    pub fn acpi_parse_entries_array(
        id: *mut c_char,
        table_size: c_ulong,
        table_header: *mut FwTableHeader,
        max_length: c_ulong,
        proc: *mut AcpiSubtableProc,
        proc_num: c_int,
        max_entries: c_uint,
    ) -> c_int;

    pub fn cdat_table_parse(
        type_: AcpiCdatType,
        handler_arg: Option<AcpiTblEntryHandlerArg>,
        arg: *mut core::ffi::c_void,
        table_header: *mut AcpiTableCdat,
        length: c_ulong,
    ) -> c_int;
}

/* CXL is the only non-ACPI consumer of the FIRMWARE_TABLE library. */
/*
 * When CONFIG_ACPI is enabled without CONFIG_CXL_BUS:
 *   EXPORT_SYMBOL_FWTBL_LIB(x) expands to EXPORT_SYMBOL_ACPI_LIB(x)
 *   __init_or_fwtbl_lib expands to __init_or_acpilib
 * Otherwise:
 *   EXPORT_SYMBOL_FWTBL_LIB(x) expands to EXPORT_SYMBOL_NS_GPL(x, "CXL")
 *   __init_or_fwtbl_lib expands to nothing.
 * These are kernel build/linkage annotations and have no standalone Rust
 * executable equivalent.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
