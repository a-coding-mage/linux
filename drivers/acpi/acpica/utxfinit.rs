// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utxfinit - External interfaces for ACPICA initialization
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies: acpi/acpi.h, accommon.h, acevents.h, acnamesp.h,
// acdebug.h, and actables.h.
// The C preprocessor component and export/init tracing macros are represented
// by their corresponding external Rust dependencies/macros where available.

/* For acpi_exec only */
extern "C" {
    pub fn ae_do_object_overrides();
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_initialize_subsystem
 *
 * PARAMETERS:  None
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Initializes all global variables. This is the first function
 *              called, so any early initialization belongs here.
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_initialize_subsystem() -> acpi_status {
    let mut status: acpi_status;

    // ACPI_FUNCTION_TRACE(acpi_initialize_subsystem);

    acpi_gbl_startup_flags = ACPI_SUBSYSTEM_INITIALIZE;
    // ACPI_DEBUG_EXEC(acpi_ut_init_stack_ptr_trace());

    /* Initialize the OS-Dependent layer */
    status = acpi_os_initialize();
    if (ACPI_FAILURE(status)) {
        // ACPI_EXCEPTION((AE_INFO, status, "During OSL initialization"));
        return status;
    }

    /* Initialize all globals used by the subsystem */
    status = acpi_ut_init_globals();
    if (ACPI_FAILURE(status)) {
        // ACPI_EXCEPTION((AE_INFO, status, "During initialization of globals"));
        return status;
    }

    /* Create the default mutex objects */
    status = acpi_ut_mutex_initialize();
    if (ACPI_FAILURE(status)) {
        // ACPI_EXCEPTION((AE_INFO, status, "During Global Mutex creation"));
        return status;
    }

    /* Initialize the namespace manager and the root of the namespace tree */
    status = acpi_ns_root_initialize();
    if (ACPI_FAILURE(status)) {
        // ACPI_EXCEPTION((AE_INFO, status, "During Namespace initialization"));
        return status;
    }

    /* Initialize the global OSI interfaces list with the static names */
    status = acpi_ut_initialize_interfaces();
    if (ACPI_FAILURE(status)) {
        // ACPI_EXCEPTION((AE_INFO, status, "During OSI interfaces initialization"));
        return status;
    }

    return AE_OK;
}

// ACPI_EXPORT_SYMBOL_INIT(acpi_initialize_subsystem)

/*******************************************************************************
 *
 * FUNCTION:    acpi_enable_subsystem
 *
 * PARAMETERS:  flags               - Init/enable Options
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Completes the subsystem initialization including hardware.
 *              Puts system into ACPI mode if it isn't already.
 *
 ******************************************************************************/
pub unsafe extern "C" fn acpi_enable_subsystem(mut flags: u32) -> acpi_status {
    let mut status: acpi_status = AE_OK;

    // ACPI_FUNCTION_TRACE(acpi_enable_subsystem);
    acpi_gbl_early_initialization = FALSE;

    /* Obtain a permanent mapping for the FACS. */
    if (flags & ACPI_NO_FACS_INIT) == 0 {
        status = acpi_tb_initialize_facs();
        if (ACPI_FAILURE(status)) {
            // ACPI_WARNING((AE_INFO, "Could not map the FACS table"));
            return status;
        }
    }

    // #if (!ACPI_REDUCED_HARDWARE)
    if (flags & ACPI_NO_ACPI_ENABLE) == 0 {
        // ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "[Init] Going into ACPI mode\n"));
        acpi_gbl_original_mode = acpi_hw_get_mode();
        status = acpi_enable();
        if (ACPI_FAILURE(status)) {
            // ACPI_WARNING((AE_INFO, "AcpiEnable failed"));
            return status;
        }
    }

    if (flags & ACPI_NO_EVENT_INIT) == 0 {
        // ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "[Init] Initializing ACPI events\n"));
        status = acpi_ev_initialize_events();
        if (ACPI_FAILURE(status)) {
            return status;
        }
    }

    if (flags & ACPI_NO_HANDLER_INIT) == 0 {
        // ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "[Init] Installing SCI/GL handlers\n"));
        status = acpi_ev_install_xrupt_handlers();
        if (ACPI_FAILURE(status)) {
            return status;
        }
    }
    // #endif /* !ACPI_REDUCED_HARDWARE */

    return status;
}

// ACPI_EXPORT_SYMBOL_INIT(acpi_enable_subsystem)

/*******************************************************************************
 *
 * FUNCTION:    acpi_initialize_objects
 *
 * PARAMETERS:  flags               - Init/enable Options
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Completes namespace initialization by initializing device
 *              objects and executing AML code for Regions, buffers, etc.
 *
 ******************************************************************************/
pub unsafe extern "C" fn acpi_initialize_objects(flags: u32) -> acpi_status {
    let mut status: acpi_status = AE_OK;

    // ACPI_FUNCTION_TRACE(acpi_initialize_objects);

    // #ifdef ACPI_OBSOLETE_BEHAVIOR
    // 05/2019: Removed, initialization now happens at both object creation
    // and table load time.
    // If the build enables this legacy conditional, the original behavior is:
    // if (flags & ACPI_NO_OBJECT_INIT) == 0 {
    //     status = acpi_ns_initialize_objects();
    //     if (ACPI_FAILURE(status)) {
    //         return status;
    //     }
    // }
    // #endif

    if (flags & (ACPI_NO_DEVICE_INIT | ACPI_NO_ADDRESS_SPACE_INIT)) == 0 {
        status = acpi_ns_initialize_devices(flags);
        if (ACPI_FAILURE(status)) {
            return status;
        }
    }

    /*
     * Empty the caches (delete the cached objects) on the assumption that
     * the table load filled them up more than they will be at runtime --
     * thus wasting non-paged memory.
     */
    status = acpi_purge_cached_objects();

    acpi_gbl_startup_flags |= ACPI_INITIALIZED_OK;
    return status;
}

// ACPI_EXPORT_SYMBOL_INIT(acpi_initialize_objects)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
