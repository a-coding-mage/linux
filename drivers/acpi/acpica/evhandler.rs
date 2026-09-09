// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: evhandler - Support for Address Space handlers
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies supplied by the ACPICA translation unit.

/* Local prototypes */
unsafe fn acpi_ev_install_handler(
    obj_handle: acpi_handle,
    level: u32,
    context: *mut core::ffi::c_void,
    return_value: *mut *mut core::ffi::c_void,
) -> acpi_status;

/* These are the address spaces that will get default handlers */
pub static mut acpi_gbl_default_address_spaces: [u8; ACPI_NUM_DEFAULT_SPACES as usize] = [
    ACPI_ADR_SPACE_SYSTEM_MEMORY,
    ACPI_ADR_SPACE_SYSTEM_IO,
    ACPI_ADR_SPACE_PCI_CONFIG,
    ACPI_ADR_SPACE_DATA_TABLE,
];

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_install_region_handlers
 *
 * PARAMETERS:  None
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Installs the core subsystem default address space handlers.
 *
 ******************************************************************************/

pub unsafe fn acpi_ev_install_region_handlers() -> acpi_status {
    let mut status: acpi_status;

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if (ACPI_FAILURE(status)) {
        return status;
    }

    /*
     * All address spaces (PCI Config, EC, SMBus) are scope dependent and
     * registration must occur for a specific device.
     *
     * In the case of the system memory and IO address spaces there is
     * currently no device associated with the address space. For these we
     * use the root.
     *
     * We install the default PCI config space handler at the root so that
     * this space is immediately available even though the we have not
     * enumerated all the PCI Root Buses yet. This is to conform to the ACPI
     * specification which states that the PCI config space must be always
     * available -- even though we are nowhere near ready to find the PCI root
     * buses at this point.
     *
     * NOTE: We ignore AE_ALREADY_EXISTS because this means that a handler
     * has already been installed (via acpi_install_address_space_handler).
     * Similar for AE_SAME_HANDLER.
     */
    let mut i: u32 = 0;
    while i < ACPI_NUM_DEFAULT_SPACES {
        status = acpi_ev_install_space_handler(
            acpi_gbl_root_node,
            acpi_gbl_default_address_spaces[i as usize],
            ACPI_DEFAULT_HANDLER,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        match status {
            AE_OK | AE_SAME_HANDLER | AE_ALREADY_EXISTS => status = AE_OK,
            _ => break,
        }
        i += 1;
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_has_default_handler
 *
 * PARAMETERS:  node                - Namespace node for the device
 *              space_id            - The address space ID
 *
 * RETURN:      TRUE if default handler is installed, FALSE otherwise
 *
 * DESCRIPTION: Check if the default handler is installed for the requested
 *              space ID.
 *
 ******************************************************************************/

pub unsafe fn acpi_ev_has_default_handler(
    node: *mut acpi_namespace_node,
    space_id: acpi_adr_space_type,
) -> u8 {
    let obj_desc = acpi_ns_get_attached_object(node);
    if !obj_desc.is_null() {
        let mut handler_obj = (*obj_desc).common_notify.handler;
        while !handler_obj.is_null() {
            if (*handler_obj).common.r#type != ACPI_TYPE_LOCAL_ADDRESS_HANDLER {
                break;
            }
            if (*handler_obj).address_space.space_id == space_id
                && ((*handler_obj).address_space.handler_flags
                    & ACPI_ADDR_HANDLER_DEFAULT_INSTALLED) != 0
            {
                return TRUE;
            }
            handler_obj = (*handler_obj).address_space.next;
        }
    }
    FALSE
}

unsafe fn acpi_ev_install_handler(
    obj_handle: acpi_handle,
    _level: u32,
    context: *mut core::ffi::c_void,
    _return_value: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let handler_obj = context as *mut acpi_operand_object;
    if handler_obj.is_null() {
        return AE_OK;
    }

    let node = acpi_ns_validate_handle(obj_handle);
    if node.is_null() {
        return AE_BAD_PARAMETER;
    }

    if (*node).r#type != ACPI_TYPE_DEVICE
        && (*node).r#type != ACPI_TYPE_REGION
        && node != acpi_gbl_root_node
    {
        return AE_OK;
    }

    let obj_desc = acpi_ns_get_attached_object(node);
    if obj_desc.is_null() {
        return AE_OK;
    }

    if (*obj_desc).common.r#type == ACPI_TYPE_DEVICE {
        let next_handler_obj = acpi_ev_find_region_handler(
            (*handler_obj).address_space.space_id,
            (*obj_desc).common_notify.handler,
        );
        if !next_handler_obj.is_null() {
            return AE_CTRL_DEPTH;
        }
        return AE_OK;
    }

    if (*obj_desc).region.space_id != (*handler_obj).address_space.space_id {
        return AE_OK;
    }

    acpi_ev_detach_region(obj_desc, FALSE);
    acpi_ev_attach_region(handler_obj, obj_desc, FALSE)
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_find_region_handler
 *
 ******************************************************************************/

pub unsafe fn acpi_ev_find_region_handler(
    space_id: acpi_adr_space_type,
    mut handler_obj: *mut acpi_operand_object,
) -> *mut acpi_operand_object {
    while !handler_obj.is_null() {
        if (*handler_obj).common.r#type != ACPI_TYPE_LOCAL_ADDRESS_HANDLER {
            break;
        }
        if (*handler_obj).address_space.space_id == space_id {
            return handler_obj;
        }
        handler_obj = (*handler_obj).address_space.next;
    }
    core::ptr::null_mut()
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_install_space_handler
 *
 ******************************************************************************/

pub unsafe fn acpi_ev_install_space_handler(
    node: *mut acpi_namespace_node,
    space_id: acpi_adr_space_type,
    mut handler: acpi_adr_space_handler,
    mut setup: acpi_adr_space_setup,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    let mut status = AE_OK;
    let mut flags: u8 = 0;

    if (*node).r#type != ACPI_TYPE_DEVICE
        && (*node).r#type != ACPI_TYPE_PROCESSOR
        && (*node).r#type != ACPI_TYPE_THERMAL
        && node != acpi_gbl_root_node
    {
        return AE_BAD_PARAMETER;
    }

    if handler == ACPI_DEFAULT_HANDLER {
        flags = ACPI_ADDR_HANDLER_DEFAULT_INSTALLED;
        match space_id {
            ACPI_ADR_SPACE_SYSTEM_MEMORY => {
                handler = acpi_ex_system_memory_space_handler;
                setup = acpi_ev_system_memory_region_setup;
            }
            ACPI_ADR_SPACE_SYSTEM_IO => {
                handler = acpi_ex_system_io_space_handler;
                setup = acpi_ev_io_space_region_setup;
            }
            ACPI_ADR_SPACE_PCI_CONFIG => {
                handler = acpi_ex_pci_config_space_handler;
                setup = acpi_ev_pci_config_region_setup;
            }
            ACPI_ADR_SPACE_CMOS => {
                handler = acpi_ex_cmos_space_handler;
                setup = acpi_ev_cmos_region_setup;
            }
            ACPI_ADR_SPACE_PCI_BAR_TARGET => {
                handler = acpi_ex_pci_bar_space_handler;
                setup = acpi_ev_pci_bar_region_setup;
            }
            ACPI_ADR_SPACE_DATA_TABLE => {
                handler = acpi_ex_data_table_space_handler;
                setup = acpi_ev_data_table_region_setup;
            }
            _ => return AE_BAD_PARAMETER,
        }
    }

    if setup.is_none() {
        setup = acpi_ev_default_region_setup;
    }

    let mut obj_desc = acpi_ns_get_attached_object(node);
    if !obj_desc.is_null() {
        let handler_obj = acpi_ev_find_region_handler(
            space_id,
            (*obj_desc).common_notify.handler,
        );
        if !handler_obj.is_null() {
            if (*handler_obj).address_space.handler == handler {
                return AE_SAME_HANDLER;
            }
            return AE_ALREADY_EXISTS;
        }
    } else {
        let object_type = if (*node).r#type == ACPI_TYPE_ANY {
            ACPI_TYPE_DEVICE
        } else {
            (*node).r#type
        };
        obj_desc = acpi_ut_create_internal_object(object_type);
        if obj_desc.is_null() {
            return AE_NO_MEMORY;
        }
        (*obj_desc).common.r#type = object_type as u8;
        status = acpi_ns_attach_object(node, obj_desc, object_type);
        acpi_ut_remove_reference(obj_desc);
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    let handler_obj = acpi_ut_create_internal_object(ACPI_TYPE_LOCAL_ADDRESS_HANDLER);
    if handler_obj.is_null() {
        return AE_NO_MEMORY;
    }
    status = acpi_os_create_mutex(&mut (*handler_obj).address_space.context_mutex);
    if ACPI_FAILURE(status) {
        acpi_ut_remove_reference(handler_obj);
        return status;
    }

    (*handler_obj).address_space.space_id = space_id as u8;
    (*handler_obj).address_space.handler_flags = flags;
    (*handler_obj).address_space.region_list = core::ptr::null_mut();
    (*handler_obj).address_space.node = node;
    (*handler_obj).address_space.handler = handler;
    (*handler_obj).address_space.context = context;
    (*handler_obj).address_space.setup = setup;
    (*handler_obj).address_space.next = (*obj_desc).common_notify.handler;
    (*obj_desc).common_notify.handler = handler_obj;

    acpi_ns_walk_namespace(
        ACPI_TYPE_ANY,
        node,
        ACPI_UINT32_MAX,
        ACPI_NS_WALK_UNLOCK,
        acpi_ev_install_handler,
        core::ptr::null_mut(),
        handler_obj as *mut core::ffi::c_void,
        core::ptr::null_mut(),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
