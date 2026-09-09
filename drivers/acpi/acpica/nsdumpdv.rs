// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: nsdump - table dumping routines for debug
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependency equivalent of: #include <acpi/acpi.h>
// TBD: This entire module is apparently obsolete and should be removed

// _COMPONENT ACPI_NAMESPACE
// ACPI_MODULE_NAME("nsdumpdv")

// The following code is present only when ACPI_OBSOLETE_FUNCTIONS and either
// ACPI_DEBUG_OUTPUT or ACPI_DEBUGGER are enabled in the C build.
#[cfg(all(feature = "ACPI_OBSOLETE_FUNCTIONS", any(feature = "ACPI_DEBUG_OUTPUT", feature = "ACPI_DEBUGGER")))]
mod obsolete_debug {
    use super::*;

    /******************************************************************************
     *
     * FUNCTION:    acpi_ns_dump_one_device
     *
     * PARAMETERS:  handle              - Node to be dumped
     *              level               - Nesting level of the handle
     *              context             - Passed into walk_namespace
     *              return_value        - Not used
     *
     * RETURN:      Status
     *
     * DESCRIPTION: Dump a single Node that represents a device
     *              This procedure is a user_function called by acpi_ns_walk_namespace.
     *
     ******************************************************************************/
    unsafe fn acpi_ns_dump_one_device(
        obj_handle: acpi_handle,
        level: u32,
        context: *mut core::ffi::c_void,
        return_value: *mut *mut core::ffi::c_void,
    ) -> acpi_status {
        let mut buffer: acpi_buffer = core::mem::zeroed();
        let mut info: *mut acpi_device_info;
        let mut status: acpi_status;
        let mut i: u32;

        // ACPI_FUNCTION_NAME(ns_dump_one_device)

        status = acpi_ns_dump_one_object(obj_handle, level, context, return_value);

        buffer.length = ACPI_ALLOCATE_LOCAL_BUFFER;
        status = acpi_get_object_info(obj_handle, &mut buffer);
        if ACPI_SUCCESS(status) {
            info = buffer.pointer as *mut acpi_device_info;
            i = 0;
            while i < level {
                // ACPI_DEBUG_PRINT_RAW((ACPI_DB_TABLES, " "));
                i += 1;
            }

            // ACPI_DEBUG_PRINT_RAW((ACPI_DB_TABLES,
            //     "    HID: %s, ADR: %8.8X%8.8X\n",
            //     (*info).hardware_id.value,
            //     ACPI_FORMAT_UINT64((*info).address)));
            ACPI_FREE(info);
        }

        status
    }

    /******************************************************************************
     *
     * FUNCTION:    acpi_ns_dump_root_devices
     *
     * PARAMETERS:  None
     *
     * RETURN:      None
     *
     * DESCRIPTION: Dump all objects of type "device"
     *
     ******************************************************************************/
    pub unsafe fn acpi_ns_dump_root_devices() {
        let mut sys_bus_handle: acpi_handle = core::ptr::null_mut();
        let mut status: acpi_status;

        // ACPI_FUNCTION_NAME(ns_dump_root_devices)

        /* Only dump the table if tracing is enabled */
        if (ACPI_LV_TABLES & acpi_dbg_level) == 0 {
            return;
        }

        status = acpi_get_handle(core::ptr::null_mut(), METHOD_NAME__SB_, &mut sys_bus_handle);
        if ACPI_FAILURE(status) {
            return;
        }

        // ACPI_DEBUG_PRINT((ACPI_DB_TABLES,
        //     "Display of all devices in the namespace:\n"));

        status = acpi_ns_walk_namespace(
            ACPI_TYPE_DEVICE,
            sys_bus_handle,
            ACPI_UINT32_MAX,
            ACPI_NS_WALK_NO_UNLOCK,
            Some(acpi_ns_dump_one_device),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        let _ = status;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
