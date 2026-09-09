// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: excreate - Named object creation
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies are supplied by the ACPICA Rust translation.

const _COMPONENT: u32 = ACPI_EXECUTER;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_create_alias
 *
 * DESCRIPTION: Create a new named alias
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_create_alias(walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut target_node: *mut acpi_namespace_node;
    let alias_node: *mut acpi_namespace_node;
    let status: acpi_status = AE_OK;

    alias_node = (*walk_state).operands[0] as *mut acpi_namespace_node;
    target_node = (*walk_state).operands[1] as *mut acpi_namespace_node;

    if (*target_node).type_ == ACPI_TYPE_LOCAL_ALIAS
        || (*target_node).type_ == ACPI_TYPE_LOCAL_METHOD_ALIAS
    {
        target_node = (*target_node).object as *mut acpi_namespace_node;
    }

    if target_node.is_null() {
        return AE_NULL_OBJECT;
    }

    match (*target_node).type_ {
        ACPI_TYPE_METHOD => {
            (*alias_node).type_ = ACPI_TYPE_LOCAL_METHOD_ALIAS;
        }
        _ => {
            (*alias_node).type_ = ACPI_TYPE_LOCAL_ALIAS;
            (*alias_node).object = target_node as *mut acpi_operand_object;
        }
    }

    (*alias_node).object = target_node as *mut acpi_operand_object;
    status
}

/*******************************************************************************
 * FUNCTION:    acpi_ex_create_event
 * DESCRIPTION: Create a new event object
 ******************************************************************************/
pub unsafe fn acpi_ex_create_event(walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut status: acpi_status;
    let obj_desc: *mut acpi_operand_object;

    obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_EVENT);
    if obj_desc.is_null() {
        status = AE_NO_MEMORY;
    } else {
        status = acpi_os_create_semaphore(
            ACPI_NO_UNIT_LIMIT,
            0,
            &mut (*obj_desc).event.os_semaphore,
        );
        if !ACPI_FAILURE(status) {
            status = acpi_ns_attach_object(
                (*walk_state).operands[0] as *mut acpi_namespace_node,
                obj_desc,
                ACPI_TYPE_EVENT,
            );
        }
    }

    acpi_ut_remove_reference(obj_desc);
    status
}

/*******************************************************************************
 * FUNCTION:    acpi_ex_create_mutex
 * DESCRIPTION: Create a new mutex object
 ******************************************************************************/
pub unsafe fn acpi_ex_create_mutex(walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let obj_desc: *mut acpi_operand_object;

    obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_MUTEX);
    if obj_desc.is_null() {
        status = AE_NO_MEMORY;
    } else {
        status = acpi_os_create_mutex(&mut (*obj_desc).mutex.os_mutex);
        if !ACPI_FAILURE(status) {
            (*obj_desc).mutex.sync_level = (*walk_state).operands[1].integer.value as u8;
            (*obj_desc).mutex.node = (*walk_state).operands[0] as *mut acpi_namespace_node;
            status = acpi_ns_attach_object(
                (*obj_desc).mutex.node,
                obj_desc,
                ACPI_TYPE_MUTEX,
            );
        }
    }

    acpi_ut_remove_reference(obj_desc);
    status
}

/*******************************************************************************
 * FUNCTION:    acpi_ex_create_region
 * DESCRIPTION: Create a new operation region object
 ******************************************************************************/
pub unsafe fn acpi_ex_create_region(
    aml_start: *mut u8,
    aml_length: u32,
    space_id: u8,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut status: acpi_status;
    let obj_desc: *mut acpi_operand_object;
    let node: *mut acpi_namespace_node = (*walk_state).op->common.node;
    let region_obj2: *mut acpi_operand_object;

    if !acpi_ns_get_attached_object(node).is_null() {
        return AE_OK;
    }

    if !acpi_is_valid_space_id(space_id) {
        ACPI_ERROR((AE_INFO, "Invalid/unknown Address Space ID: 0x%2.2X", space_id));
    }

    ACPI_DEBUG_PRINT((
        ACPI_DB_LOAD,
        "Region Type - %s (0x%X)\n",
        acpi_ut_get_region_name(space_id),
        space_id,
    ));

    obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_REGION);
    if obj_desc.is_null() {
        status = AE_NO_MEMORY;
    } else {
        region_obj2 = acpi_ns_get_secondary_object(obj_desc);
        (*region_obj2).extra.aml_start = aml_start;
        (*region_obj2).extra.aml_length = aml_length;
        (*region_obj2).extra.method_REG = core::ptr::null_mut();
        (*region_obj2).extra.scope_node = if !(*walk_state).scope_info.is_null() {
            (*walk_state).scope_info->scope.node
        } else {
            node
        };

        (*obj_desc).region.space_id = space_id;
        (*obj_desc).region.address = 0;
        (*obj_desc).region.length = 0;
        (*obj_desc).region.pointer = core::ptr::null_mut();
        (*obj_desc).region.node = node;
        (*obj_desc).region.handler = core::ptr::null_mut();
        (*obj_desc).common.flags &=
            !(AOPOBJ_SETUP_COMPLETE | AOPOBJ_REG_CONNECTED | AOPOBJ_OBJECT_INITIALIZED);

        status = acpi_ns_attach_object(node, obj_desc, ACPI_TYPE_REGION);
    }

    acpi_ut_remove_reference(obj_desc);
    status
}

/*******************************************************************************
 * FUNCTION:    acpi_ex_create_processor
 * DESCRIPTION: Create a new processor object and populate the fields
 ******************************************************************************/
pub unsafe fn acpi_ex_create_processor(walk_state: *mut acpi_walk_state) -> acpi_status {
    let operand = &mut (*walk_state).operands[0] as *mut *mut acpi_operand_object;
    let obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_PROCESSOR);
    let status: acpi_status;

    if obj_desc.is_null() {
        return AE_NO_MEMORY;
    }

    (*obj_desc).processor.proc_id = (*operand.add(1)).integer.value as u8;
    (*obj_desc).processor.length = (*operand.add(3)).integer.value as u8;
    (*obj_desc).processor.address = (*operand.add(2)).integer.value as acpi_io_address;
    status = acpi_ns_attach_object(
        *operand as *mut acpi_namespace_node,
        obj_desc,
        ACPI_TYPE_PROCESSOR,
    );
    acpi_ut_remove_reference(obj_desc);
    status
}

/*******************************************************************************
 * FUNCTION:    acpi_ex_create_power_resource
 * DESCRIPTION: Create a new power_resource object and populate the fields
 ******************************************************************************/
pub unsafe fn acpi_ex_create_power_resource(walk_state: *mut acpi_walk_state) -> acpi_status {
    let operand = &mut (*walk_state).operands[0] as *mut *mut acpi_operand_object;
    let obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_POWER);
    let status: acpi_status;

    if obj_desc.is_null() {
        return AE_NO_MEMORY;
    }

    (*obj_desc).power_resource.system_level = (*operand.add(1)).integer.value as u8;
    (*obj_desc).power_resource.resource_order = (*operand.add(2)).integer.value as u16;
    status = acpi_ns_attach_object(
        *operand as *mut acpi_namespace_node,
        obj_desc,
        ACPI_TYPE_POWER,
    );
    acpi_ut_remove_reference(obj_desc);
    status
}

/*******************************************************************************
 * FUNCTION:    acpi_ex_create_method
 * DESCRIPTION: Create a new method object
 ******************************************************************************/
pub unsafe fn acpi_ex_create_method(
    aml_start: *mut u8,
    aml_length: u32,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let operand = &mut (*walk_state).operands[0] as *mut *mut acpi_operand_object;
    let obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_METHOD);
    let status: acpi_status;
    let method_flags: u8;

    if obj_desc.is_null() {
        status = AE_NO_MEMORY;
    } else {
        (*obj_desc).method.aml_start = aml_start;
        (*obj_desc).method.aml_length = aml_length;
        (*obj_desc).method.node = *operand;
        method_flags = (*operand.add(1)).integer.value as u8;
        (*obj_desc).method.param_count = method_flags & AML_METHOD_ARG_COUNT;
        if method_flags & AML_METHOD_SERIALIZED != 0 {
            (*obj_desc).method.info_flags = ACPI_METHOD_SERIALIZED;
            (*obj_desc).method.sync_level = (method_flags & AML_METHOD_SYNC_LEVEL) >> 4;
        }
        status = acpi_ns_attach_object(
            *operand as *mut acpi_namespace_node,
            obj_desc,
            ACPI_TYPE_METHOD,
        );
        acpi_ut_remove_reference(obj_desc);
    }

    acpi_ut_remove_reference(*operand.add(1));
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
