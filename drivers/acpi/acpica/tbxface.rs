// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: tbxface - ACPI table-oriented external interfaces

// C dependencies and build-time ACPI tracing/export macros are supplied by the
// surrounding translation unit.

pub unsafe fn acpi_allocate_root_table(initial_table_count: u32) -> acpi_status {
    acpi_gbl_root_table_list.max_table_count = initial_table_count;
    acpi_gbl_root_table_list.flags = ACPI_ROOT_ALLOW_RESIZE;
    acpi_tb_resize_root_table_list()
}

pub unsafe fn acpi_initialize_tables(
    initial_table_array: *mut acpi_table_desc,
    initial_table_count: u32,
    allow_resize: u8,
) -> acpi_status {
    let rsdp_address: acpi_physical_address;
    let status: acpi_status;

    if initial_table_array.is_null() {
        status = acpi_allocate_root_table(initial_table_count);
        if ACPI_FAILURE(status) {
            return status;
        }
    } else {
        core::ptr::write_bytes(
            initial_table_array as *mut u8,
            0,
            (initial_table_count as usize) * core::mem::size_of::<acpi_table_desc>(),
        );
        acpi_gbl_root_table_list.tables = initial_table_array;
        acpi_gbl_root_table_list.max_table_count = initial_table_count;
        acpi_gbl_root_table_list.flags = ACPI_ROOT_ORIGIN_UNKNOWN;
        if allow_resize != 0 {
            acpi_gbl_root_table_list.flags |= ACPI_ROOT_ALLOW_RESIZE;
        }
    }

    rsdp_address = acpi_os_get_root_pointer();
    if rsdp_address == 0 {
        return AE_NOT_FOUND;
    }
    acpi_tb_parse_root_table(rsdp_address)
}

pub unsafe fn acpi_reallocate_root_table() -> acpi_status {
    let mut status: acpi_status;
    let mut table_desc: *mut acpi_table_desc;
    let mut j: u32 = 0;

    if (acpi_gbl_root_table_list.flags & ACPI_ROOT_ORIGIN_ALLOCATED) != 0
        && acpi_gbl_enable_table_validation
    {
        return AE_SUPPORT;
    }

    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    for i in 0..acpi_gbl_root_table_list.current_table_count {
        table_desc = acpi_gbl_root_table_list.tables.add(i as usize);
        if !(*table_desc).pointer.is_null() {
            ACPI_ERROR((AE_INFO, "Table [%4.4s] is not invalidated during early boot stage", (*table_desc).signature.ascii));
        }
    }

    if !acpi_gbl_enable_table_validation {
        acpi_gbl_enable_table_validation = TRUE;
        for i in 0..acpi_gbl_root_table_list.current_table_count {
            table_desc = acpi_gbl_root_table_list.tables.add(i as usize);
            if ((*table_desc).flags & ACPI_TABLE_IS_VERIFIED) == 0 {
                status = acpi_tb_verify_temp_table(table_desc, core::ptr::null_mut(), &mut j);
                if ACPI_FAILURE(status) {
                    acpi_tb_uninstall_table(table_desc);
                }
            }
        }
    }
    acpi_gbl_root_table_list.flags |= ACPI_ROOT_ALLOW_RESIZE;
    status = acpi_tb_resize_root_table_list();
    acpi_gbl_root_table_list.flags |= ACPI_ROOT_ORIGIN_ALLOCATED;
    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
    status
}

pub unsafe fn acpi_get_table_header(
    signature: *mut i8,
    instance: u32,
    out_table_header: *mut acpi_table_header,
) -> acpi_status {
    if signature.is_null() || out_table_header.is_null() { return AE_BAD_PARAMETER; }
    let mut j = 0;
    for i in 0..acpi_gbl_root_table_list.current_table_count {
        let desc = &mut *acpi_gbl_root_table_list.tables.add(i as usize);
        if !ACPI_COMPARE_NAMESEG(&desc.signature, signature) { continue; }
        j += 1;
        if j < instance { continue; }
        if desc.pointer.is_null() {
            if (desc.flags & ACPI_TABLE_ORIGIN_MASK) == ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL {
                let header = acpi_os_map_memory(desc.address, core::mem::size_of::<acpi_table_header>());
                if header.is_null() { return AE_NO_MEMORY; }
                core::ptr::copy_nonoverlapping(header as *const u8, out_table_header as *mut u8, core::mem::size_of::<acpi_table_header>());
                acpi_os_unmap_memory(header, core::mem::size_of::<acpi_table_header>());
            } else { return AE_NOT_FOUND; }
        } else {
            core::ptr::copy_nonoverlapping(desc.pointer as *const u8, out_table_header as *mut u8, core::mem::size_of::<acpi_table_header>());
        }
        return AE_OK;
    }
    AE_NOT_FOUND
}

pub unsafe fn acpi_get_table(signature: *mut i8, instance: u32, out_table: *mut *mut acpi_table_header) -> acpi_status {
    if signature.is_null() || out_table.is_null() { return AE_BAD_PARAMETER; }
    *out_table = core::ptr::null_mut();
    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    let mut status = AE_NOT_FOUND;
    let mut j = 0;
    for i in 0..acpi_gbl_root_table_list.current_table_count {
        let desc = acpi_gbl_root_table_list.tables.add(i as usize);
        if !ACPI_COMPARE_NAMESEG(&(*desc).signature, signature) { continue; }
        j += 1;
        if j < instance { continue; }
        status = acpi_tb_get_table(desc, out_table);
        break;
    }
    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
    status
}

pub unsafe fn acpi_put_table(table: *mut acpi_table_header) {
    if table.is_null() { return; }
    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    for i in 0..acpi_gbl_root_table_list.current_table_count {
        let desc = acpi_gbl_root_table_list.tables.add(i as usize);
        if (*desc).pointer != table { continue; }
        acpi_tb_put_table(desc);
        break;
    }
    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
}

pub unsafe fn acpi_get_table_by_index(table_index: u32, out_table: *mut *mut acpi_table_header) -> acpi_status {
    if out_table.is_null() { return AE_BAD_PARAMETER; }
    *out_table = core::ptr::null_mut();
    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    let status = if table_index >= acpi_gbl_root_table_list.current_table_count { AE_BAD_PARAMETER } else { acpi_tb_get_table(acpi_gbl_root_table_list.tables.add(table_index as usize), out_table) };
    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
    status
}

pub unsafe fn acpi_install_table_handler(handler: acpi_table_handler, context: *mut core::ffi::c_void) -> acpi_status {
    if handler.is_none() { return AE_BAD_PARAMETER; }
    let status = acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);
    if ACPI_FAILURE(status) { return status; }
    let result;
    if acpi_gbl_table_handler.is_some() { result = AE_ALREADY_EXISTS; } else {
        acpi_gbl_table_handler = handler;
        acpi_gbl_table_handler_context = context;
        result = AE_OK;
    }
    let _ = acpi_ut_release_mutex(ACPI_MTX_EVENTS);
    result
}

pub unsafe fn acpi_remove_table_handler(handler: acpi_table_handler) -> acpi_status {
    let status = acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);
    if ACPI_FAILURE(status) { return status; }
    let result = if handler.is_none() || handler != acpi_gbl_table_handler { AE_BAD_PARAMETER } else { acpi_gbl_table_handler = None; AE_OK };
    let _ = acpi_ut_release_mutex(ACPI_MTX_EVENTS);
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
