// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utownerid - Support for Table/Method Owner IDs

// Dependencies supplied by the ACPICA headers and other translation units.

extern "C" {
    static mut acpi_gbl_last_owner_id_index: u8;
    static mut acpi_gbl_next_owner_id_offset: u8;
    static mut acpi_gbl_owner_id_mask: [u32; ACPI_NUM_OWNERID_MASKS as usize];

    fn acpi_ut_acquire_mutex(mutex_id: u32) -> acpi_status;
    fn acpi_ut_release_mutex(mutex_id: u32) -> acpi_status;
}

// The following types, constants, and diagnostic facilities are provided by ACPICA.

pub unsafe fn acpi_ut_allocate_owner_id(owner_id: *mut acpi_owner_id) -> acpi_status {
    let mut j: u32;
    let mut k: u32;
    let mut status: acpi_status;

    // ACPI_FUNCTION_TRACE(ut_allocate_owner_id)

    // Guard against multiple allocations of ID to the same location
    if *owner_id != 0 {
        // ACPI_ERROR((AE_INFO, "Owner ID [0x%3.3X] already exists", *owner_id))
        return AE_ALREADY_EXISTS;
    }

    // Mutex for the global ID mask
    status = acpi_ut_acquire_mutex(ACPI_MTX_CACHES);
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Find a free owner ID, cycling through all possible IDs. */
    let mut i: u32 = 0;
    j = acpi_gbl_last_owner_id_index as u32;
    while i < (ACPI_NUM_OWNERID_MASKS + 1) {
        if j >= ACPI_NUM_OWNERID_MASKS {
            j = 0;
        }

        k = acpi_gbl_next_owner_id_offset as u32;
        while k < 32 {
            if acpi_gbl_owner_id_mask[j as usize] == ACPI_UINT32_MAX {
                break;
            }

            if (acpi_gbl_owner_id_mask[j as usize] & (1u32 << k)) == 0 {
                acpi_gbl_owner_id_mask[j as usize] |= 1u32 << k;
                acpi_gbl_last_owner_id_index = j as u8;
                acpi_gbl_next_owner_id_offset = (k + 1) as u8;
                *owner_id = ((k + 1) + ACPI_MUL_32(j)) as acpi_owner_id;
                // ACPI_DEBUG_PRINT((ACPI_DB_VALUES, "Allocated OwnerId: 0x%3.3X\n", *owner_id))
                break;
            }
            k += 1;
        }

        if k < 32 {
            break;
        }
        acpi_gbl_next_owner_id_offset = 0;
        i += 1;
        j += 1;
    }

    if *owner_id == 0 {
        status = AE_OWNER_ID_LIMIT;
        // ACPI_ERROR((AE_INFO, "Could not allocate new OwnerId (4095 max), AE_OWNER_ID_LIMIT"))
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_CACHES);
    status
}

pub unsafe fn acpi_ut_release_owner_id(owner_id_ptr: *mut acpi_owner_id) {
    let mut owner_id: acpi_owner_id = *owner_id_ptr;
    let status: acpi_status;
    let index: u32;
    let bit: u32;

    // ACPI_FUNCTION_TRACE_U32(ut_release_owner_id, owner_id)
    *owner_id_ptr = 0;

    if owner_id == 0 {
        // ACPI_ERROR((AE_INFO, "Invalid OwnerId: 0x%3.3X", owner_id))
        return;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_CACHES);
    if ACPI_FAILURE(status) {
        return;
    }

    owner_id -= 1;
    index = ACPI_DIV_32(owner_id as u32);
    bit = 1u32 << ACPI_MOD_32(owner_id as u32);

    if (acpi_gbl_owner_id_mask[index as usize] & bit) != 0 {
        acpi_gbl_owner_id_mask[index as usize] ^= bit;
    } else {
        // ACPI_ERROR((AE_INFO, "Attempted release of non-allocated OwnerId: 0x%3.3X", owner_id + 1))
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_CACHES);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
