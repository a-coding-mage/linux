// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dsobject - Dispatcher object management routines
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation.

pub unsafe fn acpi_ds_build_internal_object(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
    obj_desc_ptr: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object;
    let mut status: acpi_status;

    (*obj_desc_ptr) = core::ptr::null_mut();
    if (*op).common.aml_opcode == AML_INT_NAMEPATH_OP {
        /* This is a named object reference. */
        if (*op).common.node.is_null() {
            if (*(*op).common.parent).common.aml_opcode == AML_PACKAGE_OP
                || (*(*op).common.parent).common.aml_opcode == AML_VARIABLE_PACKAGE_OP
            {
                goto_create_new_object();
            } else {
                status = acpi_ns_lookup(
                    (*walk_state).scope_info,
                    (*op).common.value.string,
                    ACPI_TYPE_ANY,
                    ACPI_IMODE_EXECUTE,
                    ACPI_NS_SEARCH_PARENT | ACPI_NS_DONT_OPEN_SCOPE,
                    core::ptr::null_mut(),
                    &mut (*op).common.node,
                );
                if ACPI_FAILURE(status) {
                    ACPI_ERROR_NAMESPACE((*walk_state).scope_info, (*op).common.value.string, status);
                    return status;
                }
            }
        }
    }

    goto_create_new_object();
    #[allow(unreachable_code)]
    fn goto_create_new_object() {}

    obj_desc = acpi_ut_create_internal_object(
        acpi_ps_get_opcode_info((*op).common.aml_opcode).object_type,
    );
    if obj_desc.is_null() { return AE_NO_MEMORY; }
    status = acpi_ds_init_object_from_op(walk_state, op, (*op).common.aml_opcode, &mut obj_desc);
    if ACPI_FAILURE(status) {
        acpi_ut_remove_reference(obj_desc);
        return status;
    }
    if (*(*op).common.parent).common.aml_opcode == AML_PACKAGE_OP
        || (*(*op).common.parent).common.aml_opcode == AML_VARIABLE_PACKAGE_OP
    {
        (*obj_desc).reference.resolved = TRUE;
        if (*op).common.aml_opcode == AML_INT_NAMEPATH_OP
            && (*obj_desc).reference.node.is_null()
        {
            (*obj_desc).reference.node = (*(*walk_state).scope_info).scope.node;
            (*obj_desc).reference.aml = (*op).common.aml;
            (*obj_desc).reference.resolved = FALSE;
        }
    }
    *obj_desc_ptr = obj_desc;
    status
}

pub unsafe fn acpi_ds_build_internal_buffer_obj(
    _walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
    buffer_length: u32,
    obj_desc_ptr: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut obj_desc = *obj_desc_ptr;
    let arg: *mut acpi_parse_object;
    let byte_list: *mut acpi_parse_object;
    let mut byte_list_length: u32 = 0;
    if obj_desc.is_null() {
        obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_BUFFER);
        *obj_desc_ptr = obj_desc;
        if obj_desc.is_null() { return AE_NO_MEMORY; }
    }
    arg = (*op).common.value.arg;
    byte_list = (*arg).named.next;
    if !byte_list.is_null() {
        if (*byte_list).common.aml_opcode != AML_INT_BYTELIST_OP {
            acpi_ut_remove_reference(obj_desc);
            return AE_TYPE;
        }
        byte_list_length = (*byte_list).common.value.integer as u32;
    }
    (*obj_desc).buffer.length = if byte_list_length > buffer_length { byte_list_length } else { buffer_length };
    if (*obj_desc).buffer.length == 0 {
        (*obj_desc).buffer.pointer = core::ptr::null_mut();
    } else {
        (*obj_desc).buffer.pointer = ACPI_ALLOCATE_ZEROED((*obj_desc).buffer.length);
        if (*obj_desc).buffer.pointer.is_null() {
            acpi_ut_delete_object_desc(obj_desc);
            return AE_NO_MEMORY;
        }
        if !byte_list.is_null() {
            core::ptr::copy_nonoverlapping(
                (*byte_list).named.data,
                (*obj_desc).buffer.pointer,
                byte_list_length as usize,
            );
        }
    }
    (*obj_desc).buffer.flags |= AOPOBJ_DATA_VALID;
    (*op).common.node = obj_desc as *mut acpi_namespace_node;
    AE_OK
}

pub unsafe fn acpi_ds_create_node(
    walk_state: *mut acpi_walk_state,
    node: *mut acpi_namespace_node,
    op: *mut acpi_parse_object,
) -> acpi_status {
    let status: acpi_status;
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    if !acpi_ns_get_attached_object(node).is_null() { return AE_OK; }
    if (*op).common.value.arg.is_null() { return AE_OK; }
    status = acpi_ds_build_internal_object(walk_state, (*op).common.value.arg, &mut obj_desc);
    if ACPI_FAILURE(status) { return status; }
    (*node).type_ = (*obj_desc).common.type_;
    let status = acpi_ns_attach_object(node, obj_desc, (*node).type_);
    acpi_ut_remove_reference(obj_desc);
    status
}

pub unsafe fn acpi_ds_init_object_from_op(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
    opcode: u16,
    ret_obj_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let obj_desc = *ret_obj_desc;
    let op_info = acpi_ps_get_opcode_info(opcode);
    let mut status = AE_OK;
    if (*op_info).class == AML_CLASS_UNKNOWN { return AE_TYPE; }
    match (*obj_desc).common.type_ {
        ACPI_TYPE_BUFFER => {
            (*obj_desc).buffer.node = (*walk_state).operands[0] as *mut acpi_namespace_node;
            (*obj_desc).buffer.aml_start = (*op).named.data;
            (*obj_desc).buffer.aml_length = (*op).named.length;
        }
        ACPI_TYPE_PACKAGE => {
            (*obj_desc).package.node = (*walk_state).operands[0] as *mut acpi_namespace_node;
            if (*op).named.data.is_null() { return AE_OK; }
            (*obj_desc).package.aml_start = (*op).named.data;
            (*obj_desc).package.aml_length = (*op).named.length;
        }
        ACPI_TYPE_INTEGER => match (*op_info).type_ {
            AML_TYPE_CONSTANT => {
                (*obj_desc).common.flags = AOPOBJ_AML_CONSTANT;
                match opcode {
                    AML_ZERO_OP => (*obj_desc).integer.value = 0,
                    AML_ONE_OP => (*obj_desc).integer.value = 1,
                    AML_ONES_OP => { (*obj_desc).integer.value = ACPI_UINT64_MAX; acpi_ex_truncate_for32bit_table(obj_desc); }
                    AML_REVISION_OP => (*obj_desc).integer.value = ACPI_CA_VERSION,
                    _ => status = AE_AML_OPERAND_TYPE,
                }
            }
            AML_TYPE_LITERAL => {
                (*obj_desc).integer.value = (*op).common.value.integer;
                acpi_ex_truncate_for32bit_table(obj_desc);
            }
            _ => status = AE_AML_OPERAND_TYPE,
        },
        ACPI_TYPE_STRING => {
            (*obj_desc).string.pointer = (*op).common.value.string;
            (*obj_desc).string.length = libc::strlen((*op).common.value.string) as u32;
            (*obj_desc).common.flags |= AOPOBJ_STATIC_POINTER;
        }
        ACPI_TYPE_METHOD => {}
        ACPI_TYPE_LOCAL_REFERENCE => match (*op_info).type_ {
            AML_TYPE_LOCAL_VARIABLE => {
                (*obj_desc).reference.value = opcode as u32 - AML_FIRST_LOCAL_OP;
                (*obj_desc).reference.class = ACPI_REFCLASS_LOCAL;
                status = acpi_ds_method_data_get_node(ACPI_REFCLASS_LOCAL, (*obj_desc).reference.value, walk_state, &mut (*obj_desc).reference.object);
            }
            AML_TYPE_METHOD_ARGUMENT => {
                (*obj_desc).reference.value = opcode as u32 - AML_FIRST_ARG_OP;
                (*obj_desc).reference.class = ACPI_REFCLASS_ARG;
                status = acpi_ds_method_data_get_node(ACPI_REFCLASS_ARG, (*obj_desc).reference.value, walk_state, &mut (*obj_desc).reference.object);
            }
            _ => match (*op).common.aml_opcode {
                AML_INT_NAMEPATH_OP => { (*obj_desc).reference.node = (*op).common.node; (*obj_desc).reference.class = ACPI_REFCLASS_NAME; if !(*op).common.node.is_null() { (*obj_desc).reference.object = (*(*op).common.node).object; } }
                AML_DEBUG_OP => (*obj_desc).reference.class = ACPI_REFCLASS_DEBUG,
                _ => return AE_AML_OPERAND_TYPE,
            },
        },
        _ => status = AE_AML_OPERAND_TYPE,
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
