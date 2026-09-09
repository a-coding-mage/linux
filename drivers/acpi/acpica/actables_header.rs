/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: actables.h - ACPI table management
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* C header guard: __ACTABLES_H__ */

extern "C" {
    pub fn acpi_allocate_root_table(initial_table_count: u32) -> acpi_status;

    /*
     * tbxfroot - Root pointer utilities
     */
    pub fn acpi_tb_get_rsdp_length(rsdp: *mut acpi_table_rsdp) -> u32;

    pub fn acpi_tb_validate_rsdp(rsdp: *mut acpi_table_rsdp) -> acpi_status;

    pub fn acpi_tb_scan_memory_for_rsdp(start_address: *mut u8, length: u32) -> *mut u8;

    /*
     * tbdata - table data structure management
     */
    pub fn acpi_tb_get_next_table_descriptor(
        table_index: *mut u32,
        table_desc: *mut *mut acpi_table_desc,
    ) -> acpi_status;

    pub fn acpi_tb_init_table_descriptor(
        table_desc: *mut acpi_table_desc,
        address: acpi_physical_address,
        flags: u8,
        table: *mut acpi_table_header,
    );

    pub fn acpi_tb_acquire_temp_table(
        table_desc: *mut acpi_table_desc,
        address: acpi_physical_address,
        flags: u8,
        table: *mut acpi_table_header,
    ) -> acpi_status;

    pub fn acpi_tb_release_temp_table(table_desc: *mut acpi_table_desc);

    pub fn acpi_tb_validate_temp_table(table_desc: *mut acpi_table_desc) -> acpi_status;

    pub fn acpi_tb_verify_temp_table(
        table_desc: *mut acpi_table_desc,
        signature: *mut core::ffi::c_char,
        table_index: *mut u32,
    ) -> acpi_status;

    pub fn acpi_tb_is_table_loaded(table_index: u32) -> u8;

    pub fn acpi_tb_set_table_loaded_flag(table_index: u32, is_loaded: u8);

    /*
     * tbfadt - FADT parse/convert/validate
     */
    pub fn acpi_tb_parse_fadt();

    pub fn acpi_tb_create_local_fadt(table: *mut acpi_table_header, length: u32);

    /*
     * tbfind - find ACPI table
     */
    pub fn acpi_tb_find_table(
        signature: *mut core::ffi::c_char,
        oem_id: *mut core::ffi::c_char,
        oem_table_id: *mut core::ffi::c_char,
        table_index: *mut u32,
    ) -> acpi_status;

    /*
     * tbinstal - Table removal and deletion
     */
    pub fn acpi_tb_resize_root_table_list() -> acpi_status;

    pub fn acpi_tb_validate_table(table_desc: *mut acpi_table_desc) -> acpi_status;

    pub fn acpi_tb_invalidate_table(table_desc: *mut acpi_table_desc);

    pub fn acpi_tb_override_table(old_table_desc: *mut acpi_table_desc);

    pub fn acpi_tb_acquire_table(
        table_desc: *mut acpi_table_desc,
        table_ptr: *mut *mut acpi_table_header,
        table_length: *mut u32,
        table_flags: *mut u8,
    ) -> acpi_status;

    pub fn acpi_tb_release_table(table: *mut acpi_table_header, table_length: u32, table_flags: u8);

    pub fn acpi_tb_install_standard_table(
        address: acpi_physical_address,
        flags: u8,
        table: *mut acpi_table_header,
        reload: u8,
        override_: u8,
        table_index: *mut u32,
    ) -> acpi_status;

    pub fn acpi_tb_uninstall_table(table_desc: *mut acpi_table_desc);

    pub fn acpi_tb_load_table(table_index: u32, parent_node: *mut acpi_namespace_node) -> acpi_status;

    pub fn acpi_tb_install_and_load_table(
        address: acpi_physical_address,
        flags: u8,
        table: *mut acpi_table_header,
        override_: u8,
        table_index: *mut u32,
    ) -> acpi_status;

    pub fn acpi_tb_unload_table(table_index: u32) -> acpi_status;

    pub fn acpi_tb_notify_table(event: u32, table: *mut core::ffi::c_void);

    pub fn acpi_tb_terminate();

    pub fn acpi_tb_delete_namespace_by_owner(table_index: u32) -> acpi_status;

    pub fn acpi_tb_allocate_owner_id(table_index: u32) -> acpi_status;

    pub fn acpi_tb_release_owner_id(table_index: u32) -> acpi_status;

    pub fn acpi_tb_get_owner_id(table_index: u32, owner_id: *mut acpi_owner_id) -> acpi_status;

    /*
     * tbutils - table manager utilities
     */
    pub fn acpi_tb_initialize_facs() -> acpi_status;

    pub fn acpi_tb_print_table_header(address: acpi_physical_address, header: *mut acpi_table_header);

    pub fn acpi_tb_check_dsdt_header();

    pub fn acpi_tb_copy_dsdt(table_index: u32) -> *mut acpi_table_header;

    pub fn acpi_tb_install_table_with_override(
        new_table_desc: *mut acpi_table_desc,
        override_: u8,
        table_index: *mut u32,
    );

    pub fn acpi_tb_parse_root_table(rsdp_address: acpi_physical_address) -> acpi_status;

    pub fn acpi_tb_get_table(
        table_desc: *mut acpi_table_desc,
        out_table: *mut *mut acpi_table_header,
    ) -> acpi_status;

    pub fn acpi_tb_put_table(table_desc: *mut acpi_table_desc);

    /*
     * tbxfload
     */
    pub fn acpi_tb_load_namespace() -> acpi_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
