// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: tbinstal - ACPI table installation and removal
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the surrounding ACPI translation unit.

pub unsafe fn acpi_tb_install_table_with_override(
    new_table_desc: *mut acpi_table_desc,
    override_: u8,
    table_index: *mut u32,
) {
    let mut i: u32 = 0;
    let status: acpi_status = acpi_tb_get_next_table_descriptor(&mut i, core::ptr::null_mut());
    if ACPI_FAILURE(status) {
        return;
    }

    /* ACPI Table Override: allow the host OS to replace the table. */
    if override_ != 0 {
        acpi_tb_override_table(new_table_desc);
    }

    acpi_tb_init_table_descriptor(
        &mut (*acpi_gbl_root_table_list).tables[i as usize],
        (*new_table_desc).address,
        (*new_table_desc).flags,
        (*new_table_desc).pointer,
    );
    acpi_tb_print_table_header((*new_table_desc).address, (*new_table_desc).pointer);

    /* This synchronizes acpi_gbl_dsdt_index. */
    *table_index = i;

    /* Set the global integer width based upon the revision of the DSDT. */
    if i == acpi_gbl_dsdt_index {
        acpi_ut_set_integer_width((*(*new_table_desc).pointer).revision);
    }
}

pub unsafe fn acpi_tb_install_standard_table(
    address: acpi_physical_address,
    flags: u8,
    table: *mut acpi_table_header,
    reload: u8,
    override_: u8,
    table_index: *mut u32,
) -> acpi_status {
    let mut i: u32 = 0;
    let mut status: acpi_status = AE_OK;
    let mut new_table_desc: acpi_table_desc = core::mem::zeroed();

    status = acpi_tb_acquire_temp_table(&mut new_table_desc, address, flags, table);
    if ACPI_FAILURE(status) {
        return status;
    }

    if reload == 0
        && acpi_gbl_disable_ssdt_table_install
        && ACPI_COMPARE_NAMESEG(&new_table_desc.signature, ACPI_SIG_SSDT)
    {
        goto_release(&mut new_table_desc);
        return status;
    }

    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    status = acpi_tb_verify_temp_table(&mut new_table_desc, core::ptr::null_mut(), &mut i);
    if ACPI_FAILURE(status) {
        if status == AE_CTRL_TERMINATE {
            acpi_tb_uninstall_table(&mut new_table_desc);
            let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
            *table_index = i;
            return AE_OK;
        }
        let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
        acpi_tb_release_temp_table(&mut new_table_desc);
        return status;
    }

    acpi_tb_install_table_with_override(&mut new_table_desc, override_, table_index);
    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);
    acpi_tb_notify_table(ACPI_TABLE_EVENT_INSTALL, new_table_desc.pointer);
    let _ = acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    let _ = acpi_ut_release_mutex(ACPI_MTX_TABLES);

goto_release(&mut new_table_desc);
    status
}

unsafe fn goto_release(table_desc: *mut acpi_table_desc) {
    acpi_tb_release_temp_table(table_desc);
}

pub unsafe fn acpi_tb_override_table(old_table_desc: *mut acpi_table_desc) {
    let mut status: acpi_status;
    let mut new_table_desc: acpi_table_desc = core::mem::zeroed();
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut address: acpi_physical_address = 0;
    let mut length: u32 = 0;
    let mut override_type: *const core::ffi::c_char = core::ptr::null();

    status = acpi_os_table_override((*old_table_desc).pointer, &mut table);
    if ACPI_SUCCESS(status) && !table.is_null() {
        acpi_tb_acquire_temp_table(&mut new_table_desc, ACPI_PTR_TO_PHYSADDR(table), ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL, table);
        override_type = b"Logical\0".as_ptr() as *const _;
    } else {
        status = acpi_os_physical_table_override((*old_table_desc).pointer, &mut address, &mut length);
        if !(ACPI_SUCCESS(status) && address != 0 && length != 0) {
            return;
        }
        acpi_tb_acquire_temp_table(&mut new_table_desc, address, ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL, core::ptr::null_mut());
        override_type = b"Physical\0".as_ptr() as *const _;
    }

    status = acpi_tb_verify_temp_table(&mut new_table_desc, core::ptr::null_mut(), core::ptr::null_mut());
    if ACPI_FAILURE(status) {
        return;
    }
    let _ = override_type;
    acpi_tb_uninstall_table(old_table_desc);
    acpi_tb_init_table_descriptor(old_table_desc, new_table_desc.address, new_table_desc.flags, new_table_desc.pointer);
    acpi_tb_validate_temp_table(old_table_desc);
    acpi_tb_release_temp_table(&mut new_table_desc);
}

pub unsafe fn acpi_tb_uninstall_table(table_desc: *mut acpi_table_desc) {
    if (*table_desc).address == 0 {
        return;
    }
    acpi_tb_invalidate_table(table_desc);
    if ((*table_desc).flags & ACPI_TABLE_ORIGIN_MASK) == ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL {
        ACPI_FREE((*table_desc).pointer);
        (*table_desc).pointer = core::ptr::null_mut();
    }
    (*table_desc).address = ACPI_PTR_TO_PHYSADDR(core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
