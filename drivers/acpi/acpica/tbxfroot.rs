// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: tbxfroot - Find the root ACPI table (RSDT)
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies supplied by the surrounding ACPI translation unit.

/*
 * FUNCTION:    acpi_tb_get_rsdp_length
 *
 * PARAMETERS:  rsdp                - Pointer to RSDP
 *
 * RETURN:      Table length
 *
 * DESCRIPTION: Get the length of the RSDP
 */
pub unsafe fn acpi_tb_get_rsdp_length(rsdp: *mut acpi_table_rsdp) -> u32 {
    if !ACPI_VALIDATE_RSDP_SIG((*rsdp).signature) {
        /* BAD Signature */
        return 0;
    }

    /* "Length" field is available if table version >= 2 */
    if (*rsdp).revision >= 2 {
        (*rsdp).length
    } else {
        ACPI_RSDP_CHECKSUM_LENGTH
    }
}

/*
 * FUNCTION:    acpi_tb_validate_rsdp
 *
 * PARAMETERS:  rsdp                - Pointer to unvalidated RSDP
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Validate the RSDP (ptr)
 */
pub unsafe fn acpi_tb_validate_rsdp(rsdp: *mut acpi_table_rsdp) -> acpi_status {
    /*
     * The signature and checksum must both be correct
     *
     * Note: Sometimes there exists more than one RSDP in memory; the valid
     * RSDP has a valid checksum, all others have an invalid checksum.
     */
    if !ACPI_VALIDATE_RSDP_SIG((*rsdp).signature) {
        /* Nope, BAD Signature */
        return AE_BAD_SIGNATURE;
    }

    /* Check the standard checksum */
    if acpi_ut_checksum(rsdp as *mut u8, ACPI_RSDP_CHECKSUM_LENGTH) != 0 {
        return AE_BAD_CHECKSUM;
    }

    /* Check extended checksum if table version >= 2 */
    if (*rsdp).revision >= 2
        && acpi_ut_checksum(rsdp as *mut u8, ACPI_RSDP_XCHECKSUM_LENGTH) != 0
    {
        return AE_BAD_CHECKSUM;
    }

    AE_OK
}

/*
 * FUNCTION:    acpi_find_root_pointer
 *
 * PARAMETERS:  table_address           - Where the table pointer is returned
 *
 * RETURN:      Status, RSDP physical address
 *
 * DESCRIPTION: Search lower 1Mbyte of memory for the root system descriptor
 *              pointer structure. If it is found, set *RSDP to point to it.
 *
 * NOTE1:       The RSDP must be either in the first 1K of the Extended
 *              BIOS Data Area or between E0000 and FFFFF (From ACPI Spec.)
 *              Only a 32-bit physical address is necessary.
 *
 * NOTE2:       This function is always available, regardless of the
 *              initialization state of the rest of ACPI.
 */
pub unsafe fn acpi_find_root_pointer(table_address: *mut acpi_physical_address) -> acpi_status {
    let mut table_ptr: *mut u8;
    let mut mem_rover: *mut u8;
    let mut physical_address: u32;
    let mut ebda_window_size: u32;

    /* 1a) Get the location of the Extended BIOS Data Area (EBDA) */
    table_ptr = acpi_os_map_memory(
        ACPI_EBDA_PTR_LOCATION as acpi_physical_address,
        ACPI_EBDA_PTR_LENGTH,
    );
    if table_ptr.is_null() {
        ACPI_ERROR((AE_INFO, "Could not map memory at 0x%8.8X for length %u", ACPI_EBDA_PTR_LOCATION, ACPI_EBDA_PTR_LENGTH));
        return AE_NO_MEMORY;
    }

    ACPI_MOVE_16_TO_32(&mut physical_address, table_ptr);
    physical_address <<= 4;
    acpi_os_unmap_memory(table_ptr, ACPI_EBDA_PTR_LENGTH);

    /* EBDA present? */
    /* Check that the EBDA pointer from memory is sane and does not point
     * above valid low memory */
    if physical_address > 0x400 && physical_address < 0xA0000 {
        /* Calculate the scan window size */
        ebda_window_size = ACPI_MIN(ACPI_EBDA_WINDOW_SIZE, 0xA0000 - physical_address);

        /* 1b) Search EBDA paragraphs */
        table_ptr = acpi_os_map_memory(physical_address as acpi_physical_address, ebda_window_size);
        if table_ptr.is_null() {
            ACPI_ERROR((AE_INFO, "Could not map memory at 0x%8.8X for length %u", physical_address, ebda_window_size));
            return AE_NO_MEMORY;
        }

        mem_rover = acpi_tb_scan_memory_for_rsdp(table_ptr, ebda_window_size);
        acpi_os_unmap_memory(table_ptr, ebda_window_size);

        if !mem_rover.is_null() {
            physical_address = physical_address.wrapping_add(mem_rover.offset_from(table_ptr) as u32);
            *table_address = physical_address as acpi_physical_address;
            return AE_OK;
        }
    }

    /* 2) Search upper memory: 16-byte boundaries in E0000h-FFFFFh */
    table_ptr = acpi_os_map_memory(
        ACPI_HI_RSDP_WINDOW_BASE as acpi_physical_address,
        ACPI_HI_RSDP_WINDOW_SIZE,
    );
    if table_ptr.is_null() {
        ACPI_ERROR((AE_INFO, "Could not map memory at 0x%8.8X for length %u", ACPI_HI_RSDP_WINDOW_BASE, ACPI_HI_RSDP_WINDOW_SIZE));
        return AE_NO_MEMORY;
    }

    mem_rover = acpi_tb_scan_memory_for_rsdp(table_ptr, ACPI_HI_RSDP_WINDOW_SIZE);
    acpi_os_unmap_memory(table_ptr, ACPI_HI_RSDP_WINDOW_SIZE);

    if !mem_rover.is_null() {
        physical_address = (ACPI_HI_RSDP_WINDOW_BASE as usize
            + mem_rover.offset_from(table_ptr) as usize) as u32;
        *table_address = physical_address as acpi_physical_address;
        return AE_OK;
    }

    /* A valid RSDP was not found */
    ACPI_BIOS_ERROR((AE_INFO, "A valid RSDP was not found"));
    AE_NOT_FOUND
}

/*
 * FUNCTION:    acpi_tb_scan_memory_for_rsdp
 *
 * PARAMETERS:  start_address       - Starting pointer for search
 *              length              - Maximum length to search
 *
 * RETURN:      Pointer to the RSDP if found, otherwise NULL.
 *
 * DESCRIPTION: Search a block of memory for the RSDP signature
 */
pub unsafe fn acpi_tb_scan_memory_for_rsdp(start_address: *mut u8, length: u32) -> *mut u8 {
    let end_address = start_address.add(length as usize);
    let mut mem_rover = start_address;

    /* Search from given start address for the requested length */
    while mem_rover < end_address {
        /* The RSDP signature and checksum must both be correct */
        let status = acpi_tb_validate_rsdp(mem_rover as *mut acpi_table_rsdp);
        if ACPI_SUCCESS(status) {
            /* Sig and checksum valid, we have found a real RSDP */
            ACPI_DEBUG_PRINT((ACPI_DB_INFO, "RSDP located at physical address %p\n", mem_rover));
            return mem_rover;
        }
        /* No sig match or bad checksum, keep searching */
        mem_rover = mem_rover.add(ACPI_RSDP_SCAN_STEP as usize);
    }

    /* Searched entire block, no RSDP was found */
    ACPI_DEBUG_PRINT((ACPI_DB_INFO, "Searched entire block from %p, valid RSDP was not found\n", start_address));
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
