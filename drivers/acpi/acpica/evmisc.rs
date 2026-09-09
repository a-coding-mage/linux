// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: evmisc - Miscellaneous event manager support functions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI implementation are intentionally external.

/* Local prototypes */
unsafe extern "C" fn acpi_ev_notify_dispatch(context: *mut core::ffi::c_void);

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_is_notify_object
 *
 ******************************************************************************/
pub unsafe fn acpi_ev_is_notify_object(node: *mut acpi_namespace_node) -> u8 {
    match (*node).type_ {
        ACPI_TYPE_DEVICE | ACPI_TYPE_PROCESSOR | ACPI_TYPE_THERMAL => TRUE,
        _ => FALSE,
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_queue_notify_request
 *
 ******************************************************************************/
pub unsafe fn acpi_ev_queue_notify_request(
    node: *mut acpi_namespace_node,
    notify_value: u32,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut handler_list_head: *mut acpi_operand_object = core::ptr::null_mut();
    let mut handler_list_id: u8 = 0;
    let mut status: acpi_status = AE_OK;

    if acpi_ev_is_notify_object(node) == FALSE {
        return AE_TYPE;
    }

    if notify_value <= ACPI_MAX_SYS_NOTIFY {
        handler_list_id = ACPI_SYSTEM_HANDLER_LIST;
    } else {
        handler_list_id = ACPI_DEVICE_HANDLER_LIST;
    }

    obj_desc = acpi_ns_get_attached_object(node);
    if !obj_desc.is_null() {
        handler_list_head = (*obj_desc).common_notify.notify_list[handler_list_id as usize];
    }

    if acpi_gbl_global_notify[handler_list_id as usize].handler.is_none()
        && handler_list_head.is_null()
    {
        return AE_OK;
    }

    let info = acpi_ut_create_generic_state();
    if info.is_null() {
        return AE_NO_MEMORY;
    }

    (*info).common.descriptor_type = ACPI_DESC_TYPE_STATE_NOTIFY;
    (*info).notify.node = node;
    (*info).notify.value = notify_value as u16;
    (*info).notify.handler_list_id = handler_list_id;
    (*info).notify.handler_list_head = handler_list_head;
    (*info).notify.global = &mut acpi_gbl_global_notify[handler_list_id as usize];

    status = acpi_os_execute(OSL_NOTIFY_HANDLER, Some(acpi_ev_notify_dispatch), info as *mut _ as *mut core::ffi::c_void);
    if ACPI_FAILURE(status) {
        acpi_ut_delete_generic_state(info);
    }
    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_notify_dispatch
 *
 ******************************************************************************/
unsafe extern "C" fn acpi_ev_notify_dispatch(context: *mut core::ffi::c_void) {
    let info = context as *mut acpi_generic_state;
    let mut handler_obj: *mut acpi_operand_object;

    if (*info).notify.global.handler.is_some() {
        ((*info).notify.global.handler.unwrap())(
            (*info).notify.node,
            (*info).notify.value,
            (*info).notify.global.context,
        );
    }

    handler_obj = (*info).notify.handler_list_head;
    while !handler_obj.is_null() {
        ((*handler_obj).notify.handler.unwrap())(
            (*info).notify.node,
            (*info).notify.value,
            (*handler_obj).notify.context,
        );
        handler_obj = (*handler_obj).notify.next[(*info).notify.handler_list_id as usize];
    }

    acpi_ut_delete_generic_state(info);
}

#[cfg(not(feature = "ACPI_REDUCED_HARDWARE"))]
pub unsafe fn acpi_ev_terminate() {
    let mut status: acpi_status;

    if acpi_gbl_events_initialized {
        for i in 0..ACPI_NUM_FIXED_EVENTS {
            status = acpi_disable_event(i, 0);
            if ACPI_FAILURE(status) {
                ACPI_ERROR!((AE_INFO, "Could not disable fixed event %u", i as u32));
            }
        }

        status = acpi_ev_walk_gpe_list(Some(acpi_hw_disable_gpe_block), core::ptr::null_mut());
        if ACPI_FAILURE(status) {
            ACPI_EXCEPTION!((AE_INFO, status, "Could not disable GPEs in GPE block"));
        }

        status = acpi_ev_remove_global_lock_handler();
        if ACPI_FAILURE(status) {
            ACPI_EXCEPTION!((AE_INFO, status, "Could not remove Global Lock handler"));
        }

        acpi_gbl_events_initialized = FALSE;
    }

    status = acpi_ev_remove_all_sci_handlers();
    if ACPI_FAILURE(status) {
        ACPI_ERROR!((AE_INFO, "Could not remove SCI handler"));
    }

    status = acpi_ev_walk_gpe_list(Some(acpi_ev_delete_gpe_handlers), core::ptr::null_mut());
    if ACPI_FAILURE(status) {
        ACPI_EXCEPTION!((AE_INFO, status, "Could not delete GPE handlers"));
    }

    if acpi_gbl_original_mode == ACPI_SYS_MODE_LEGACY {
        status = acpi_disable();
        if ACPI_FAILURE(status) {
            ACPI_WARNING!((AE_INFO, "AcpiDisable failed"));
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
