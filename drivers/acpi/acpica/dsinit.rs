// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dsinit - Object initialization namespace walk
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI headers and the other translation units.

const _COMPONENT: u32 = ACPI_DISPATCHER;

/* Local prototypes */
unsafe fn acpi_ds_init_one_object(
    obj_handle: acpi_handle,
    level: u32,
    context: *mut core::ffi::c_void,
    return_value: *mut *mut core::ffi::c_void,
) -> acpi_status;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ds_init_one_object
 *
 * PARAMETERS:  obj_handle      - Node for the object
 *              level           - Current nesting level
 *              context         - Points to a init info struct
 *              return_value    - Not used
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Callback from acpi_walk_namespace. Invoked for every object
 *              within the namespace.
 *
 *              Currently, the only objects that require initialization are:
 *              1) Methods
 *              2) Operation Regions
 *
 ******************************************************************************/

unsafe fn acpi_ds_init_one_object(
    obj_handle: acpi_handle,
    _level: u32,
    context: *mut core::ffi::c_void,
    _return_value: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let info = &mut *(context as *mut acpi_init_walk_info);
    let node = obj_handle as *mut acpi_namespace_node;
    let mut status: acpi_status;
    let mut obj_desc: *mut acpi_operand_object;

    ACPI_FUNCTION_ENTRY!();

    /* We are only interested in NS nodes owned by the table that was just loaded */
    if (*node).owner_id != info.owner_id {
        return AE_OK;
    }

    info.object_count += 1;

    /* And even then, we are only interested in a few object types */
    match acpi_ns_get_type(obj_handle) {
        ACPI_TYPE_REGION => {
            status = acpi_ds_initialize_region(obj_handle);
            if ACPI_FAILURE!(status) {
                ACPI_EXCEPTION!(AE_INFO, status, "During Region initialization %p [%4.4s]",
                    obj_handle, acpi_ut_get_node_name(obj_handle));
            }
            info.op_region_count += 1;
        }
        ACPI_TYPE_METHOD => {
            /* Auto-serialization support. */
            info.method_count += 1;
            obj_desc = acpi_ns_get_attached_object(node);
            if obj_desc.is_null() {
                return AE_OK;
            }

            /* Ignore if already serialized */
            if (*obj_desc).method.info_flags & ACPI_METHOD_SERIALIZED != 0 {
                info.serial_method_count += 1;
                return AE_OK;
            }

            if acpi_gbl_auto_serialize_methods {
                /* Parse/scan method and serialize it if necessary */
                acpi_ds_auto_serialize_method(node, obj_desc);
                if (*obj_desc).method.info_flags & ACPI_METHOD_SERIALIZED != 0 {
                    /* Method was just converted to Serialized */
                    info.serial_method_count += 1;
                    info.serialized_method_count += 1;
                    return AE_OK;
                }
            }
            info.non_serial_method_count += 1;
        }
        ACPI_TYPE_DEVICE => {
            info.device_count += 1;
        }
        _ => {}
    }

    /* Ignore errors from above and always return OK. */
    AE_OK
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ds_initialize_objects
 *
 * PARAMETERS:  table_desc      - Descriptor for parent ACPI table
 *              start_node      - Root of subtree to be initialized.
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Walk the namespace starting at "StartNode" and perform any
 *              necessary initialization on the objects found therein
 *
 ******************************************************************************/

pub unsafe fn acpi_ds_initialize_objects(
    table_index: u32,
    start_node: *mut acpi_namespace_node,
) -> acpi_status {
    let mut status: acpi_status;
    let mut info: acpi_init_walk_info = core::mem::zeroed();
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut owner_id: acpi_owner_id = 0;

    ACPI_FUNCTION_TRACE!(ds_initialize_objects);

    status = acpi_tb_get_owner_id(table_index, &mut owner_id);
    if ACPI_FAILURE!(status) {
        return_ACPI_STATUS!(status);
    }

    ACPI_DEBUG_PRINT!(ACPI_DB_DISPATCH,
        "**** Starting initialization of namespace objects ****\n");

    info.owner_id = owner_id;
    info.table_index = table_index;

    /* We don't use acpi_walk_namespace since we do not want to acquire the namespace reader lock. */
    status = acpi_ns_walk_namespace(
        ACPI_TYPE_ANY,
        start_node,
        ACPI_UINT32_MAX,
        ACPI_NS_WALK_NO_UNLOCK,
        acpi_ds_init_one_object,
        core::ptr::null_mut(),
        &mut info,
        core::ptr::null_mut(),
    );
    if ACPI_FAILURE!(status) {
        ACPI_EXCEPTION!(AE_INFO, status, "During WalkNamespace");
    }

    status = acpi_get_table_by_index(table_index, &mut table);
    if ACPI_FAILURE!(status) {
        return_ACPI_STATUS!(status);
    }

    /* DSDT is always the first AML table */
    if ACPI_COMPARE_NAMESEG!((*table).signature, ACPI_SIG_DSDT) {
        ACPI_DEBUG_PRINT_RAW!(ACPI_DB_INIT, "\nACPI table initialization:\n");
    }

    ACPI_DEBUG_PRINT_RAW!(ACPI_DB_INIT,
        "Table [%4.4s: %-8.8s] (id %.2X) - %4u Objects with %3u Devices, %3u Regions, %4u Methods (%u/%u/%u Serial/Non/Cvt)\n",
        (*table).signature, (*table).oem_table_id, owner_id,
        info.object_count, info.device_count, info.op_region_count,
        info.method_count, info.serial_method_count,
        info.non_serial_method_count, info.serialized_method_count);

    ACPI_DEBUG_PRINT!(ACPI_DB_DISPATCH, "%u Methods, %u Regions\n",
        info.method_count, info.op_region_count);

    return_ACPI_STATUS!(AE_OK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
