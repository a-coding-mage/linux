// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utalloc - local memory allocation routines
//
// Copyright (C) 2000 - 2026, Intel Corp.

use core::ffi::c_void;

// Dependencies supplied by the ACPICA implementation.
#[allow(non_camel_case_types)]
pub type acpi_size = usize;
#[allow(non_camel_case_types)]
pub type acpi_status = u32;

#[repr(C)]
pub struct acpi_buffer {
    pub length: acpi_size,
    pub pointer: *mut c_void,
}

#[repr(C)]
pub struct acpi_namespace_node;
#[repr(C)]
pub union acpi_generic_state { _private: u8 }
#[repr(C)]
pub struct acpi_parse_obj_common;
#[repr(C)]
pub struct acpi_parse_obj_named;
#[repr(C)]
pub union acpi_operand_object { _private: u8 }
#[repr(C)]
pub struct acpi_comment_node;
#[repr(C)]
pub struct acpi_comment_addr_node;
#[repr(C)]
pub struct acpi_file_node;

#[cfg(not(feature = "use_native_allocate_zeroed"))]
pub unsafe fn acpi_os_allocate_zeroed(size: acpi_size) -> *mut c_void {
    let allocation = acpi_os_allocate(size);
    if !allocation.is_null() {
        core::ptr::write_bytes(allocation as *mut u8, 0, size);
    }
    allocation
}

extern "C" {
    fn acpi_os_allocate(size: acpi_size) -> *mut c_void;
    fn acpi_os_free(object: *mut c_void);
    fn acpi_os_create_cache(
        name: *const u8,
        object_size: acpi_size,
        max_depth: u16,
        return_cache: *mut *mut c_void,
    ) -> acpi_status;
    fn acpi_os_delete_cache(cache: *mut c_void) -> acpi_status;
    fn acpi_ut_create_list(name: *const u8, object_size: acpi_size, return_list: *mut *mut c_void) -> acpi_status;
    fn acpi_db_display_statistics(buffer: *mut u8) -> acpi_status;
    fn acpi_ut_dump_allocations(flags: u32, component: *mut c_void);

    static mut acpi_gbl_namespace_cache: *mut c_void;
    static mut acpi_gbl_state_cache: *mut c_void;
    static mut acpi_gbl_ps_node_cache: *mut c_void;
    static mut acpi_gbl_ps_node_ext_cache: *mut c_void;
    static mut acpi_gbl_operand_cache: *mut c_void;
    static mut acpi_gbl_reg_comment_cache: *mut c_void;
    static mut acpi_gbl_comment_addr_cache: *mut c_void;
    static mut acpi_gbl_file_cache: *mut c_void;
    static mut acpi_gbl_global_list: *mut c_void;
    static mut acpi_gbl_ns_node_list: *mut c_void;
    static mut acpi_gbl_display_final_mem_stats: bool;
}

const AE_OK: acpi_status = 0;
const AE_BAD_PARAMETER: acpi_status = 1;
const AE_BUFFER_OVERFLOW: acpi_status = 2;
const AE_NO_MEMORY: acpi_status = 3;
const ACPI_NO_BUFFER: acpi_size = usize::MAX;
const ACPI_ALLOCATE_BUFFER: acpi_size = usize::MAX - 1;
const ACPI_ALLOCATE_LOCAL_BUFFER: acpi_size = usize::MAX - 2;

pub unsafe fn acpi_ut_create_caches() -> acpi_status {
    let mut status;
    status = acpi_os_create_cache(b"Acpi-Namespace\0".as_ptr(), core::mem::size_of::<acpi_namespace_node>(), 0, &mut acpi_gbl_namespace_cache);
    if status != AE_OK { return status; }
    status = acpi_os_create_cache(b"Acpi-State\0".as_ptr(), core::mem::size_of::<acpi_generic_state>(), 0, &mut acpi_gbl_state_cache);
    if status != AE_OK { return status; }
    status = acpi_os_create_cache(b"Acpi-Parse\0".as_ptr(), core::mem::size_of::<acpi_parse_obj_common>(), 0, &mut acpi_gbl_ps_node_cache);
    if status != AE_OK { return status; }
    status = acpi_os_create_cache(b"Acpi-ParseExt\0".as_ptr(), core::mem::size_of::<acpi_parse_obj_named>(), 0, &mut acpi_gbl_ps_node_ext_cache);
    if status != AE_OK { return status; }
    status = acpi_os_create_cache(b"Acpi-Operand\0".as_ptr(), core::mem::size_of::<acpi_operand_object>(), 0, &mut acpi_gbl_operand_cache);
    if status != AE_OK { return status; }
    status = AE_OK;
    status
}

pub unsafe fn acpi_ut_delete_caches() -> acpi_status {
    acpi_os_delete_cache(acpi_gbl_namespace_cache); acpi_gbl_namespace_cache = core::ptr::null_mut();
    acpi_os_delete_cache(acpi_gbl_state_cache); acpi_gbl_state_cache = core::ptr::null_mut();
    acpi_os_delete_cache(acpi_gbl_operand_cache); acpi_gbl_operand_cache = core::ptr::null_mut();
    acpi_os_delete_cache(acpi_gbl_ps_node_cache); acpi_gbl_ps_node_cache = core::ptr::null_mut();
    acpi_os_delete_cache(acpi_gbl_ps_node_ext_cache); acpi_gbl_ps_node_ext_cache = core::ptr::null_mut();
    acpi_os_free(acpi_gbl_global_list); acpi_gbl_global_list = core::ptr::null_mut();
    acpi_os_free(acpi_gbl_ns_node_list); acpi_gbl_ns_node_list = core::ptr::null_mut();
    AE_OK
}

pub unsafe fn acpi_ut_validate_buffer(buffer: *mut acpi_buffer) -> acpi_status {
    if buffer.is_null() { return AE_BAD_PARAMETER; }
    let length = (*buffer).length;
    if length == ACPI_NO_BUFFER || length == ACPI_ALLOCATE_BUFFER || length == ACPI_ALLOCATE_LOCAL_BUFFER { return AE_OK; }
    if (*buffer).pointer.is_null() { return AE_BAD_PARAMETER; }
    AE_OK
}

pub unsafe fn acpi_ut_initialize_buffer(buffer: *mut acpi_buffer, required_length: acpi_size) -> acpi_status {
    if buffer.is_null() || required_length == 0 { return AE_BAD_PARAMETER; }
    let input_buffer_length = (*buffer).length;
    (*buffer).length = required_length;
    match input_buffer_length {
        ACPI_NO_BUFFER => AE_BUFFER_OVERFLOW,
        ACPI_ALLOCATE_BUFFER => { (*buffer).pointer = acpi_os_allocate(required_length); if (*buffer).pointer.is_null() { AE_NO_MEMORY } else { core::ptr::write_bytes((*buffer).pointer as *mut u8, 0, required_length); AE_OK } }
        ACPI_ALLOCATE_LOCAL_BUFFER => { (*buffer).pointer = acpi_os_allocate(required_length); if (*buffer).pointer.is_null() { AE_NO_MEMORY } else { core::ptr::write_bytes((*buffer).pointer as *mut u8, 0, required_length); AE_OK } }
        length if length < required_length => AE_BUFFER_OVERFLOW,
        _ => { if (*buffer).pointer.is_null() { AE_NO_MEMORY } else { core::ptr::write_bytes((*buffer).pointer as *mut u8, 0, required_length); AE_OK } },
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
