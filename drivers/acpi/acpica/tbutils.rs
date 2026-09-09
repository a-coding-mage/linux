// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: tbutils - ACPI Table utilities

// External ACPI declarations and macros are supplied by the surrounding crate.

static unsafe fn acpi_tb_get_root_table_entry(
    table_entry: *mut u8,
    table_entry_size: u32,
) -> acpi_physical_address {
    let mut address32: u32 = 0;
    let mut address64: u64 = 0;

    if table_entry_size == ACPI_RSDT_ENTRY_SIZE {
        ACPI_MOVE_32_TO_32(&mut address32, table_entry);
        address32 as acpi_physical_address
    } else {
        ACPI_MOVE_64_TO_64(&mut address64, table_entry);
        // On 32-bit targets the C implementation warns before truncating.
        #[cfg(target_pointer_width = "32")]
        if address64 > ACPI_UINT32_MAX as u64 {
            ACPI_BIOS_WARNING((AE_INFO,
                "64-bit Physical Address in XSDT is too large (0x%8.8X%8.8X), truncating",
                ACPI_FORMAT_UINT64(address64)));
        }
        address64 as acpi_physical_address
    }
}

pub unsafe fn acpi_tb_initialize_facs() -> acpi_status {
    let mut facs: *mut acpi_table_facs = core::ptr::null_mut();

    if acpi_gbl_FADT.Xfacs != 0
        && (acpi_gbl_FADT.facs == 0 || !acpi_gbl_use32_bit_facs_addresses)
    {
        let _ = acpi_get_table_by_index(
            acpi_gbl_xfacs_index,
            &mut facs as *mut _ as *mut *mut acpi_table_header,
        );
        acpi_gbl_FACS = facs;
    } else if acpi_gbl_FADT.facs != 0 {
        let _ = acpi_get_table_by_index(
            acpi_gbl_facs_index,
            &mut facs as *mut _ as *mut *mut acpi_table_header,
        );
        acpi_gbl_FACS = facs;
    }

    AE_OK
}

pub unsafe fn acpi_tb_check_dsdt_header() {
    if acpi_gbl_original_dsdt_header.length != (*acpi_gbl_DSDT).length
        || acpi_gbl_original_dsdt_header.checksum != (*acpi_gbl_DSDT).checksum
    {
        ACPI_BIOS_ERROR((AE_INFO,
            "The DSDT has been corrupted or replaced - old, new headers below"));
        acpi_tb_print_table_header(0, &acpi_gbl_original_dsdt_header);
        acpi_tb_print_table_header(0, acpi_gbl_DSDT);
        ACPI_ERROR((AE_INFO,
            "Please send DMI info to linux-acpi@vger.kernel.org\n"
            "If system does not work as expected, please boot with acpi=copy_dsdt"));
        acpi_gbl_original_dsdt_header.length = (*acpi_gbl_DSDT).length;
        acpi_gbl_original_dsdt_header.checksum = (*acpi_gbl_DSDT).checksum;
    }
}

pub unsafe fn acpi_tb_copy_dsdt(table_index: u32) -> *mut acpi_table_header {
    let table_desc = &mut acpi_gbl_root_table_list.tables[table_index as usize];
    let new_table = ACPI_ALLOCATE(table_desc.length) as *mut acpi_table_header;
    if new_table.is_null() {
        ACPI_ERROR((AE_INFO, "Could not copy DSDT of length 0x%X", table_desc.length));
        return core::ptr::null_mut();
    }
    core::ptr::copy_nonoverlapping(
        table_desc.pointer as *const u8,
        new_table as *mut u8,
        table_desc.length as usize,
    );
    acpi_tb_uninstall_table(table_desc);
    acpi_tb_init_table_descriptor(
        &mut acpi_gbl_root_table_list.tables[acpi_gbl_dsdt_index as usize],
        ACPI_PTR_TO_PHYSADDR(new_table),
        ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL,
        new_table,
    );
    ACPI_INFO((
        "Forced DSDT copy: length 0x%05X copied locally, original unmapped",
        (*new_table).length,
    ));
    new_table
}

pub unsafe fn acpi_tb_parse_root_table(rsdp_address: acpi_physical_address) -> acpi_status {
    let rsdp = acpi_os_map_memory(rsdp_address, core::mem::size_of::<acpi_table_rsdp>())
        as *mut acpi_table_rsdp;
    if rsdp.is_null() { return AE_NO_MEMORY; }
    acpi_tb_print_table_header(rsdp_address, rsdp as *mut acpi_table_header);

    let (mut address, table_entry_size) = if (*rsdp).revision > 1
        && (*rsdp).xsdt_physical_address != 0 && !acpi_gbl_do_not_use_xsdt {
        ((*rsdp).xsdt_physical_address as acpi_physical_address, ACPI_XSDT_ENTRY_SIZE)
    } else {
        ((*rsdp).rsdt_physical_address as acpi_physical_address, ACPI_RSDT_ENTRY_SIZE)
    };
    acpi_os_unmap_memory(rsdp as *mut _, core::mem::size_of::<acpi_table_rsdp>());

    let header_size = core::mem::size_of::<acpi_table_header>();
    let mut table = acpi_os_map_memory(address, header_size) as *mut acpi_table_header;
    if table.is_null() { return AE_NO_MEMORY; }
    acpi_tb_print_table_header(address, table);
    let length = (*table).length;
    acpi_os_unmap_memory(table as *mut _, header_size);
    if length < (header_size as u32 + table_entry_size) {
        ACPI_BIOS_ERROR((AE_INFO, "Invalid table length 0x%X in RSDT/XSDT", length));
        return AE_INVALID_TABLE_LENGTH;
    }
    table = acpi_os_map_memory(address, length as usize) as *mut acpi_table_header;
    if table.is_null() { return AE_NO_MEMORY; }
    let status = acpi_ut_verify_checksum(table, length);
    if ACPI_FAILURE(status) {
        acpi_os_unmap_memory(table as *mut _, length as usize);
        return status;
    }
    let table_count = (length - header_size as u32) / table_entry_size;
    let mut table_entry = (table as *mut u8).add(header_size);
    for _ in 0..table_count {
        address = acpi_tb_get_root_table_entry(table_entry, table_entry_size);
        if address != 0 {
            let mut table_index: u32 = 0;
            let status = acpi_tb_install_standard_table(
                address, ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL,
                core::ptr::null_mut(), false, true, &mut table_index);
            if ACPI_SUCCESS(status)
                && ACPI_COMPARE_NAMESEG(&acpi_gbl_root_table_list.tables[table_index as usize].signature,
                                        ACPI_SIG_FADT) {
                acpi_gbl_fadt_index = table_index;
                acpi_tb_parse_fadt();
            }
        }
        table_entry = table_entry.add(table_entry_size as usize);
    }
    acpi_os_unmap_memory(table as *mut _, length as usize);
    AE_OK
}

pub unsafe fn acpi_tb_get_table(
    table_desc: *mut acpi_table_desc,
    out_table: *mut *mut acpi_table_header,
) -> acpi_status {
    if (*table_desc).validation_count == 0 {
        let status = acpi_tb_validate_table(table_desc);
        if ACPI_FAILURE(status) { return status; }
    }
    if (*table_desc).validation_count < ACPI_MAX_TABLE_VALIDATIONS {
        (*table_desc).validation_count += 1;
        if (*table_desc).validation_count >= ACPI_MAX_TABLE_VALIDATIONS {
            ACPI_WARNING((AE_INFO, "Table %p, Validation count overflows\n", table_desc));
        }
    }
    *out_table = (*table_desc).pointer;
    AE_OK
}

pub unsafe fn acpi_tb_put_table(table_desc: *mut acpi_table_desc) {
    if (*table_desc).validation_count < ACPI_MAX_TABLE_VALIDATIONS {
        (*table_desc).validation_count -= 1;
        if (*table_desc).validation_count >= ACPI_MAX_TABLE_VALIDATIONS {
            ACPI_WARNING((AE_INFO, "Table %p, Validation count underflows\n", table_desc));
            return;
        }
    }
    if (*table_desc).validation_count == 0 { acpi_tb_invalidate_table(table_desc); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
