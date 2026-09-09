// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utaddress - op_region address range check
//
// Copyright (C) 2000 - 2026, Intel Corp.

use core::ffi::c_char;
use core::ptr;

// The declarations below are supplied by the ACPICA headers and other
// translation units.
extern "C" {
    static mut acpi_gbl_address_range_list: [*mut acpi_address_range; ACPI_ADDRESS_RANGE_MAX];
    fn acpi_ut_get_node_name(node: *mut acpi_namespace_node) -> *const c_char;
    fn acpi_ns_get_normalized_pathname(
        node: *mut acpi_namespace_node,
        validate: u8,
    ) -> *mut c_char;
    fn acpi_ut_get_region_name(space_id: acpi_adr_space_type) -> *const c_char;
    fn acpi_allocate(size: usize) -> *mut core::ffi::c_void;
    fn acpi_free(pointer: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct acpi_address_range {
    pub start_address: acpi_physical_address,
    pub end_address: acpi_physical_address,
    pub region_node: *mut acpi_namespace_node,
    pub next: *mut acpi_address_range,
}

// Types and constants are provided by acpi/acpi.h.
extern "C" {
    fn acpi_debug_print_names(
        component: u32,
        format: *const c_char,
        ...
    );
    fn acpi_warning(
        info: u32,
        format: *const c_char,
        ...
    );
}

pub unsafe fn acpi_ut_add_address_range(
    space_id: acpi_adr_space_type,
    address: acpi_physical_address,
    length: u32,
    region_node: *mut acpi_namespace_node,
) -> acpi_status {
    if space_id != ACPI_ADR_SPACE_SYSTEM_MEMORY && space_id != ACPI_ADR_SPACE_SYSTEM_IO {
        return AE_OK;
    }

    let range_info = acpi_allocate(core::mem::size_of::<acpi_address_range>())
        as *mut acpi_address_range;
    if range_info.is_null() {
        return AE_NO_MEMORY;
    }

    (*range_info).start_address = address;
    (*range_info).end_address = address.wrapping_add(length as acpi_physical_address).wrapping_sub(1);
    (*range_info).region_node = region_node;

    (*range_info).next = acpi_gbl_address_range_list[space_id as usize];
    acpi_gbl_address_range_list[space_id as usize] = range_info;

    // ACPI_DEBUG_PRINT((ACPI_DB_NAMES, ...))
    acpi_debug_print_names(
        ACPI_DB_NAMES,
        b"\nAdded [%4.4s] address range: 0x%8.8X%8.8X-0x%8.8X%8.8X\n\0".as_ptr() as *const c_char,
        acpi_ut_get_node_name((*range_info).region_node),
        address,
        (*range_info).end_address,
    );

    AE_OK
}

pub unsafe fn acpi_ut_remove_address_range(
    space_id: acpi_adr_space_type,
    region_node: *mut acpi_namespace_node,
) {
    if space_id != ACPI_ADR_SPACE_SYSTEM_MEMORY && space_id != ACPI_ADR_SPACE_SYSTEM_IO {
        return;
    }

    let mut range_info = acpi_gbl_address_range_list[space_id as usize];
    let mut prev = range_info;
    while !range_info.is_null() {
        if (*range_info).region_node == region_node {
            if range_info == prev {
                acpi_gbl_address_range_list[space_id as usize] = (*range_info).next;
            } else {
                (*prev).next = (*range_info).next;
            }

            acpi_debug_print_names(
                ACPI_DB_NAMES,
                b"\nRemoved [%4.4s] address range: 0x%8.8X%8.8X-0x%8.8X%8.8X\n\0".as_ptr()
                    as *const c_char,
                acpi_ut_get_node_name((*range_info).region_node),
                (*range_info).start_address,
                (*range_info).end_address,
            );
            acpi_free(range_info as *mut core::ffi::c_void);
            return;
        }
        prev = range_info;
        range_info = (*range_info).next;
    }
}

pub unsafe fn acpi_ut_check_address_range(
    space_id: acpi_adr_space_type,
    address: acpi_physical_address,
    length: u32,
    warn: u8,
) -> u32 {
    if space_id != ACPI_ADR_SPACE_SYSTEM_MEMORY && space_id != ACPI_ADR_SPACE_SYSTEM_IO {
        return 0;
    }

    let mut range_info = acpi_gbl_address_range_list[space_id as usize];
    let end_address = address.wrapping_add(length as acpi_physical_address).wrapping_sub(1);
    let mut overlap_count = 0;

    while !range_info.is_null() {
        if address <= (*range_info).end_address && end_address >= (*range_info).start_address {
            overlap_count += 1;
            if warn != 0 {
                let pathname = acpi_ns_get_normalized_pathname((*range_info).region_node, 1);
                acpi_warning(
                    AE_INFO,
                    b"%s range 0x%8.8X%8.8X-0x%8.8X%8.8X conflicts with OpRegion 0x%8.8X%8.8X-0x%8.8X%8.8X (%s)\0".as_ptr()
                        as *const c_char,
                    acpi_ut_get_region_name(space_id),
                    address,
                    end_address,
                    (*range_info).start_address,
                    (*range_info).end_address,
                    pathname,
                );
                acpi_free(pathname as *mut core::ffi::c_void);
            }
        }
        range_info = (*range_info).next;
    }
    overlap_count
}

pub unsafe fn acpi_ut_delete_address_lists() {
    let mut i = 0;
    while i < ACPI_ADDRESS_RANGE_MAX {
        let mut next = acpi_gbl_address_range_list[i];
        while !next.is_null() {
            let range_info = next;
            next = (*range_info).next;
            acpi_free(range_info as *mut core::ffi::c_void);
        }
        acpi_gbl_address_range_list[i] = ptr::null_mut();
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
