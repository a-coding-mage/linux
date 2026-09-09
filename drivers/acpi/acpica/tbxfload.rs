// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: tbxfload - Table load/unload external interfaces
//
// C includes and ACPI tracing/export macros are supplied by the surrounding
// translation unit and are intentionally represented by their Rust symbols.

pub unsafe fn acpi_load_tables() -> acpi_status {
    let mut status: acpi_status;

    status = acpi_ev_install_region_handlers();
    if ACPI_FAILURE(status) {
        ACPI_EXCEPTION((AE_INFO, status, "During Region initialization"));
        return status;
    }

    status = acpi_tb_load_namespace();

    if status == AE_CTRL_TERMINATE {
        status = AE_OK;
    }

    if ACPI_FAILURE(status) {
        ACPI_EXCEPTION((AE_INFO, status, "While loading namespace from ACPI tables"));
    }

    status = acpi_ns_initialize_objects();
    if ACPI_SUCCESS(status) {
        acpi_gbl_namespace_initialized = TRUE;
    }

    status
}

pub unsafe fn acpi_tb_load_namespace() -> acpi_status {
    let mut status: acpi_status;
    let mut i: u32;
    let mut new_dsdt: *mut acpi_table_header;
    let mut table: *mut acpi_table_desc;
    let mut tables_loaded: u32 = 0;
    let mut tables_failed: u32 = 0;

    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);

    table = &mut acpi_gbl_root_table_list.tables[acpi_gbl_dsdt_index as usize];
    if acpi_gbl_root_table_list.current_table_count == 0
        || !ACPI_COMPARE_NAMESEG((*table).signature.ascii, ACPI_SIG_DSDT)
        || ACPI_FAILURE(acpi_tb_validate_table(table))
    {
        status = AE_NO_ACPI_TABLES;
        let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
        return status;
    }

    acpi_gbl_DSDT = (*table).pointer;

    if acpi_gbl_copy_dsdt_locally {
        new_dsdt = acpi_tb_copy_dsdt(acpi_gbl_dsdt_index);
        if !new_dsdt.is_null() {
            acpi_gbl_DSDT = new_dsdt;
        }
    }

    core::ptr::copy_nonoverlapping(
        acpi_gbl_DSDT,
        &mut acpi_gbl_original_dsdt_header,
        1,
    );

    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
    status = acpi_ns_load_table(acpi_gbl_dsdt_index, acpi_gbl_root_node);
    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    if ACPI_FAILURE(status) {
        ACPI_EXCEPTION((AE_INFO, status, "[DSDT] table load failed"));
        tables_failed += 1;
    } else {
        tables_loaded += 1;
    }

    i = 0;
    while i < acpi_gbl_root_table_list.current_table_count {
        table = &mut acpi_gbl_root_table_list.tables[i as usize];
        if (*table).address == 0
            || (!ACPI_COMPARE_NAMESEG((*table).signature.ascii, ACPI_SIG_SSDT)
                && !ACPI_COMPARE_NAMESEG((*table).signature.ascii, ACPI_SIG_PSDT)
                && !ACPI_COMPARE_NAMESEG((*table).signature.ascii, ACPI_SIG_OSDT))
            || ACPI_FAILURE(acpi_tb_validate_table(table))
        {
            i += 1;
            continue;
        }

        let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
        status = acpi_ns_load_table(i, acpi_gbl_root_node);
        let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
        if ACPI_FAILURE(status) {
            ACPI_EXCEPTION((
                AE_INFO,
                status,
                "(%4.4s:%8.8s) while loading table",
                (*table).signature.ascii,
                (*table).pointer->oem_table_id,
            ));
            tables_failed += 1;
            ACPI_DEBUG_PRINT_RAW((
                ACPI_DB_INIT,
                "Table [%4.4s:%8.8s] (id FF) - Table namespace load failed\n\n",
                (*table).signature.ascii,
                (*table).pointer->oem_table_id,
            ));
        } else {
            tables_loaded += 1;
        }
        i += 1;
    }

    if tables_failed == 0 {
        ACPI_INFO(("%u ACPI AML tables successfully acquired and loaded", tables_loaded));
    } else {
        ACPI_ERROR((AE_INFO, "%u table load failures, %u successful", tables_failed, tables_loaded));
        status = AE_CTRL_TERMINATE;
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
    status
}

pub unsafe fn acpi_install_table(table: *mut acpi_table_header) -> acpi_status {
    let mut table_index: u32 = 0;
    acpi_tb_install_standard_table(
        ACPI_PTR_TO_PHYSADDR(table),
        ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL,
        table,
        FALSE,
        FALSE,
        &mut table_index,
    )
}

pub unsafe fn acpi_install_physical_table(address: acpi_physical_address) -> acpi_status {
    let mut table_index: u32 = 0;
    acpi_tb_install_standard_table(
        address,
        ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL,
        core::ptr::null_mut(),
        FALSE,
        FALSE,
        &mut table_index,
    )
}

pub unsafe fn acpi_load_table(table: *mut acpi_table_header, table_idx: *mut u32) -> acpi_status {
    if table.is_null() {
        return AE_BAD_PARAMETER;
    }

    ACPI_INFO(("Host-directed Dynamic ACPI Table Load:"));
    let mut table_index: u32 = 0;
    let status = acpi_tb_install_and_load_table(
        ACPI_PTR_TO_PHYSADDR(table),
        ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL,
        table,
        FALSE,
        &mut table_index,
    );
    if !table_idx.is_null() {
        *table_idx = table_index;
    }
    if ACPI_SUCCESS(status) {
        acpi_ns_initialize_objects();
    }
    status
}

pub unsafe fn acpi_unload_parent_table(object: acpi_handle) -> acpi_status {
    let node = object as *mut acpi_namespace_node;
    let mut status: acpi_status = AE_NOT_EXIST;
    let owner_id: acpi_owner_id;

    if object.is_null() {
        return AE_BAD_PARAMETER;
    }
    owner_id = (*node).owner_id;
    if owner_id == 0 {
        return AE_TYPE;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    if ACPI_FAILURE(status) {
        return status;
    }
    let mut i: u32 = 0;
    while i < acpi_gbl_root_table_list.current_table_count {
        if owner_id == acpi_gbl_root_table_list.tables[i as usize].owner_id {
            if ACPI_COMPARE_NAMESEG(
                acpi_gbl_root_table_list.tables[i as usize].signature.ascii,
                ACPI_SIG_DSDT,
            ) {
                status = AE_TYPE;
                break;
            }
            let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
            status = acpi_tb_unload_table(i);
            let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
            break;
        }
        i += 1;
    }
    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
    status
}

pub unsafe fn acpi_unload_table(table_index: u32) -> acpi_status {
    if table_index == 1 {
        return AE_TYPE;
    }
    acpi_tb_unload_table(table_index)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
