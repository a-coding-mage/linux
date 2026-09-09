// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: evxfregn - External Interfaces, ACPI Operation Regions and
 *                         Address Spaces.
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

// C dependencies: acpi/acpi.h, accommon.h, acnamesp.h, acevents.h

static unsafe fn acpi_install_address_space_handler_internal(
    device: acpi_handle,
    space_id: acpi_adr_space_type,
    handler: acpi_adr_space_handler,
    setup: acpi_adr_space_setup,
    context: *mut core::ffi::c_void,
    run_reg: u8,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node;
    let mut status: acpi_status;

    if device.is_null() {
        return AE_BAD_PARAMETER;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    node = acpi_ns_validate_handle(device);
    if node.is_null() {
        status = AE_BAD_PARAMETER;
        acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
        return status;
    }

    status = acpi_ev_install_space_handler(node, space_id, handler, setup, context);
    if ACPI_FAILURE(status) {
        acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
        return status;
    }

    if run_reg != 0 {
        acpi_ev_execute_reg_methods(node, ACPI_UINT32_MAX, space_id, ACPI_REG_CONNECT);
    }

    acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

pub unsafe fn acpi_install_address_space_handler(
    device: acpi_handle,
    space_id: acpi_adr_space_type,
    handler: acpi_adr_space_handler,
    setup: acpi_adr_space_setup,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    acpi_install_address_space_handler_internal(device, space_id, handler, setup, context, TRUE)
}

pub unsafe fn acpi_install_address_space_handler_no_reg(
    device: acpi_handle,
    space_id: acpi_adr_space_type,
    handler: acpi_adr_space_handler,
    setup: acpi_adr_space_setup,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    acpi_install_address_space_handler_internal(device, space_id, handler, setup, context, FALSE)
}

pub unsafe fn acpi_remove_address_space_handler(
    device: acpi_handle,
    space_id: acpi_adr_space_type,
    handler: acpi_adr_space_handler,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object;
    let mut handler_obj: *mut acpi_operand_object;
    let mut region_obj: *mut acpi_operand_object;
    let mut last_obj_ptr: *mut *mut acpi_operand_object;
    let node: *mut acpi_namespace_node;
    let mut status: acpi_status;

    if device.is_null() {
        return AE_BAD_PARAMETER;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    node = acpi_ns_validate_handle(device);
    if node.is_null()
        || ((*node).type_ != ACPI_TYPE_DEVICE
            && (*node).type_ != ACPI_TYPE_PROCESSOR
            && (*node).type_ != ACPI_TYPE_THERMAL
            && node != acpi_gbl_root_node)
    {
        status = AE_BAD_PARAMETER;
        acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
        return status;
    }

    obj_desc = acpi_ns_get_attached_object(node);
    if obj_desc.is_null() {
        status = AE_NOT_EXIST;
        acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
        return status;
    }

    handler_obj = (*obj_desc).common_notify.handler;
    last_obj_ptr = &mut (*obj_desc).common_notify.handler;
    while !handler_obj.is_null() {
        if (*handler_obj).address_space.space_id == space_id {
            if (*handler_obj).address_space.handler != handler {
                status = AE_BAD_PARAMETER;
                acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
                return status;
            }

            region_obj = (*handler_obj).address_space.region_list;
            while !region_obj.is_null() {
                acpi_ev_detach_region(region_obj, TRUE);
                region_obj = (*handler_obj).address_space.region_list;
            }

            *last_obj_ptr = (*handler_obj).address_space.next;
            acpi_ut_remove_reference(handler_obj);
            acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
            return status;
        }

        last_obj_ptr = &mut (*handler_obj).address_space.next;
        handler_obj = (*handler_obj).address_space.next;
    }

    status = AE_NOT_EXIST;
    acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

pub unsafe fn acpi_execute_reg_methods(
    device: acpi_handle,
    max_depth: u32,
    space_id: acpi_adr_space_type,
) -> acpi_status {
    let node: *mut acpi_namespace_node;
    let mut status: acpi_status;

    if device.is_null() {
        return AE_BAD_PARAMETER;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    node = acpi_ns_validate_handle(device);
    if !node.is_null() {
        acpi_ev_execute_reg_methods(node, max_depth, space_id, ACPI_REG_CONNECT);
    } else {
        status = AE_BAD_PARAMETER;
    }

    acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
