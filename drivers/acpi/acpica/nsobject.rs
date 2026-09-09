// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: nsobject - Utilities for objects attached to namespace
 *                         table entries
 *
 ******************************************************************************/

// Dependencies supplied by ACPICA headers and other translation units.

pub unsafe fn acpi_ns_attach_object(
    node: *mut acpi_namespace_node,
    object: *mut acpi_operand_object,
    type_: acpi_object_type,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object;
    let mut last_obj_desc: *mut acpi_operand_object;
    let mut object_type: acpi_object_type = ACPI_TYPE_ANY;

    if node.is_null() {
        return AE_BAD_PARAMETER;
    }
    if object.is_null() && ACPI_TYPE_ANY != type_ {
        return AE_BAD_PARAMETER;
    }
    if ACPI_GET_DESCRIPTOR_TYPE(node) != ACPI_DESC_TYPE_NAMED {
        return AE_BAD_PARAMETER;
    }
    if (*node).object == object {
        return AE_OK;
    }

    if object.is_null() {
        obj_desc = core::ptr::null_mut();
        object_type = ACPI_TYPE_ANY;
    } else if ACPI_GET_DESCRIPTOR_TYPE(object) == ACPI_DESC_TYPE_NAMED
        && (*(object as *mut acpi_namespace_node)).object != core::ptr::null_mut()
    {
        obj_desc = (*(object as *mut acpi_namespace_node)).object;
        object_type = (*(object as *mut acpi_namespace_node)).type_;
    } else {
        obj_desc = object;
        object_type = type_;
    }

    if !(*node).object.is_null() {
        acpi_ns_detach_object(node);
    }

    if !obj_desc.is_null() {
        acpi_ut_add_reference(obj_desc);
        last_obj_desc = obj_desc;
        while !(*last_obj_desc).common.next_object.is_null() {
            last_obj_desc = (*last_obj_desc).common.next_object;
        }
        (*last_obj_desc).common.next_object = (*node).object;
    }

    (*node).type_ = object_type as u8;
    (*node).object = obj_desc;
    AE_OK
}

pub unsafe fn acpi_ns_detach_object(node: *mut acpi_namespace_node) {
    let obj_desc = (*node).object;

    if (*node).flags & ANOBJ_IS_ALIAS != 0 {
        (*node).object = core::ptr::null_mut();
        return;
    }
    if obj_desc.is_null() || (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_DATA {
        return;
    }
    if (*node).flags & ANOBJ_ALLOCATED_BUFFER != 0 {
        if (*obj_desc).common.type_ == ACPI_TYPE_METHOD {
            ACPI_FREE((*obj_desc).method.aml_start);
        }
    }
    if (*obj_desc).common.type_ == ACPI_TYPE_REGION {
        acpi_ut_remove_address_range((*obj_desc).region.space_id, node);
    }

    (*node).object = core::ptr::null_mut();
    if ACPI_GET_DESCRIPTOR_TYPE(obj_desc) == ACPI_DESC_TYPE_OPERAND {
        (*node).object = (*obj_desc).common.next_object;
        if !(*node).object.is_null()
            && (*(*node).object).common.type_ != ACPI_TYPE_LOCAL_DATA
        {
            (*node).object = (*(*node).object).common.next_object;
        }
        if !(*obj_desc).common.next_object.is_null()
            && (*(*obj_desc).common.next_object).common.type_ == ACPI_TYPE_LOCAL_DATA
        {
            (*obj_desc).common.next_object = core::ptr::null_mut();
        }
    }
    (*node).type_ = ACPI_TYPE_ANY;
    acpi_ut_remove_reference(obj_desc);
}

pub unsafe fn acpi_ns_get_attached_object(
    node: *mut acpi_namespace_node,
) -> *mut acpi_operand_object {
    if node.is_null() {
        return core::ptr::null_mut();
    }
    if (*node).object.is_null()
        || (ACPI_GET_DESCRIPTOR_TYPE((*node).object) != ACPI_DESC_TYPE_OPERAND
            && ACPI_GET_DESCRIPTOR_TYPE((*node).object) != ACPI_DESC_TYPE_NAMED)
        || (*(*node).object).common.type_ == ACPI_TYPE_LOCAL_DATA
    {
        return core::ptr::null_mut();
    }
    (*node).object
}

pub unsafe fn acpi_ns_get_secondary_object(
    obj_desc: *mut acpi_operand_object,
) -> *mut acpi_operand_object {
    if obj_desc.is_null()
        || (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_DATA
        || (*obj_desc).common.next_object.is_null()
        || (*(*obj_desc).common.next_object).common.type_ == ACPI_TYPE_LOCAL_DATA
    {
        return core::ptr::null_mut();
    }
    (*obj_desc).common.next_object
}

pub unsafe fn acpi_ns_attach_data(
    node: *mut acpi_namespace_node,
    handler: acpi_object_handler,
    data: *mut core::ffi::c_void,
) -> acpi_status {
    let mut prev_obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut obj_desc = (*node).object;
    while !obj_desc.is_null() {
        if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_DATA
            && (*obj_desc).data.handler == handler
        {
            return AE_ALREADY_EXISTS;
        }
        prev_obj_desc = obj_desc;
        obj_desc = (*obj_desc).common.next_object;
    }
    let data_desc = acpi_ut_create_internal_object(ACPI_TYPE_LOCAL_DATA);
    if data_desc.is_null() {
        return AE_NO_MEMORY;
    }
    (*data_desc).data.handler = handler;
    (*data_desc).data.pointer = data;
    if !prev_obj_desc.is_null() {
        (*prev_obj_desc).common.next_object = data_desc;
    } else {
        (*node).object = data_desc;
    }
    AE_OK
}

pub unsafe fn acpi_ns_detach_data(
    node: *mut acpi_namespace_node,
    handler: acpi_object_handler,
) -> acpi_status {
    let mut prev_obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut obj_desc = (*node).object;
    while !obj_desc.is_null() {
        if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_DATA
            && (*obj_desc).data.handler == handler
        {
            if !prev_obj_desc.is_null() {
                (*prev_obj_desc).common.next_object = (*obj_desc).common.next_object;
            } else {
                (*node).object = (*obj_desc).common.next_object;
            }
            acpi_ut_remove_reference(obj_desc);
            return AE_OK;
        }
        prev_obj_desc = obj_desc;
        obj_desc = (*obj_desc).common.next_object;
    }
    AE_NOT_FOUND
}

pub unsafe fn acpi_ns_get_attached_data(
    node: *mut acpi_namespace_node,
    handler: acpi_object_handler,
    data: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let mut obj_desc = (*node).object;
    while !obj_desc.is_null() {
        if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_DATA
            && (*obj_desc).data.handler == handler
        {
            *data = (*obj_desc).data.pointer;
            return AE_OK;
        }
        obj_desc = (*obj_desc).common.next_object;
    }
    AE_NOT_FOUND
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
