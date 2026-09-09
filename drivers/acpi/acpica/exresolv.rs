// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exresolv - AML Interpreter object resolution
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// ACPICA dependencies are supplied by the surrounding translation unit.

/* Local prototypes */
unsafe fn acpi_ex_resolve_object_to_value(
    stack_ptr: *mut *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status;

pub unsafe fn acpi_ex_resolve_to_value(
    stack_ptr: *mut *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut status: acpi_status;

    if stack_ptr.is_null() || (*stack_ptr).is_null() {
        acpi_error!("Internal - null pointer");
        return AE_AML_NO_OPERAND;
    }

    if ACPI_GET_DESCRIPTOR_TYPE(*stack_ptr) == ACPI_DESC_TYPE_OPERAND {
        status = acpi_ex_resolve_object_to_value(stack_ptr, walk_state);
        if ACPI_FAILURE(status) {
            return status;
        }
        if (*stack_ptr).is_null() {
            acpi_error!("Internal - null pointer");
            return AE_AML_NO_OPERAND;
        }
    }

    if ACPI_GET_DESCRIPTOR_TYPE(*stack_ptr) == ACPI_DESC_TYPE_NAMED {
        status = acpi_ex_resolve_node_to_value(
            stack_ptr as *mut *mut acpi_namespace_node,
            walk_state,
        );
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    acpi_debug_print!(ACPI_DB_EXEC, "Resolved object {:p}\n", *stack_ptr);
    AE_OK
}

unsafe fn acpi_ex_resolve_object_to_value(
    stack_ptr: *mut *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut status = AE_OK;
    let stack_desc = *stack_ptr;
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let ref_type: u8;

    match (*stack_desc).common.type_ {
        ACPI_TYPE_LOCAL_REFERENCE => {
            ref_type = (*stack_desc).reference.class_;
            match ref_type {
                ACPI_REFCLASS_LOCAL | ACPI_REFCLASS_ARG => {
                    status = acpi_ds_method_data_get_value(
                        ref_type,
                        (*stack_desc).reference.value,
                        walk_state,
                        &mut obj_desc,
                    );
                    if ACPI_FAILURE(status) { return status; }
                    acpi_debug_print!(ACPI_DB_EXEC, "[Arg/Local {:X}] ValueObj is {:p}\n", (*stack_desc).reference.value, obj_desc);
                    acpi_ut_remove_reference(stack_desc);
                    *stack_ptr = obj_desc;
                }
                ACPI_REFCLASS_INDEX => match (*stack_desc).reference.target_type {
                    ACPI_TYPE_BUFFER_FIELD => {}
                    ACPI_TYPE_PACKAGE => {
                        if (*walk_state).opcode == AML_INT_METHODCALL_OP || (*walk_state).opcode == AML_COPY_OBJECT_OP {
                        } else {
                            obj_desc = *(*stack_desc).reference.where_;
                            if !obj_desc.is_null() {
                                acpi_ut_add_reference(obj_desc);
                                *stack_ptr = obj_desc;
                            } else {
                                acpi_error!("Attempt to dereference an Index to NULL package element Idx={:p}", stack_desc);
                                status = AE_AML_UNINITIALIZED_ELEMENT;
                            }
                        }
                    }
                    _ => {
                        acpi_error!("Unknown TargetType 0x{:X} in Index/Reference object {:p}", (*stack_desc).reference.target_type, stack_desc);
                        status = AE_AML_INTERNAL;
                    }
                },
                ACPI_REFCLASS_REFOF | ACPI_REFCLASS_DEBUG | ACPI_REFCLASS_TABLE => {}
                ACPI_REFCLASS_NAME => {
                    let node = (*stack_desc).reference.node;
                    if (*node).type_ == ACPI_TYPE_DEVICE || (*node).type_ == ACPI_TYPE_THERMAL {
                        *stack_ptr = node as *mut acpi_operand_object;
                    } else {
                        *stack_ptr = (*node).object;
                        acpi_ut_add_reference(*stack_ptr);
                    }
                    acpi_ut_remove_reference(stack_desc);
                }
                _ => {
                    acpi_error!("Unknown Reference type 0x{:X} in {:p}", ref_type, stack_desc);
                    status = AE_AML_INTERNAL;
                }
            }
        }
        ACPI_TYPE_BUFFER => status = acpi_ds_get_buffer_arguments(stack_desc),
        ACPI_TYPE_PACKAGE => status = acpi_ds_get_package_arguments(stack_desc),
        ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_REGION_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD | ACPI_TYPE_LOCAL_INDEX_FIELD => {
            acpi_debug_print!(ACPI_DB_EXEC, "FieldRead SourceDesc={:p} Type={:X}\n", stack_desc, (*stack_desc).common.type_);
            status = acpi_ex_read_data_from_field(walk_state, stack_desc, &mut obj_desc);
            acpi_ut_remove_reference(*stack_ptr);
            *stack_ptr = obj_desc;
        }
        _ => {}
    }
    status
}

pub unsafe fn acpi_ex_resolve_multiple(
    walk_state: *mut acpi_walk_state,
    operand: *mut acpi_operand_object,
    return_type: *mut acpi_object_type,
    return_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut obj_desc = operand;
    let mut node = operand as *mut acpi_namespace_node;
    let mut type_: acpi_object_type;
    let mut status: acpi_status;

    match ACPI_GET_DESCRIPTOR_TYPE(obj_desc) {
        ACPI_DESC_TYPE_OPERAND => type_ = (*obj_desc).common.type_,
        ACPI_DESC_TYPE_NAMED => {
            type_ = (*node).type_;
            obj_desc = acpi_ns_get_attached_object(node);
            if type_ == ACPI_TYPE_LOCAL_ALIAS {
                type_ = (*(obj_desc as *mut acpi_namespace_node)).type_;
                obj_desc = acpi_ns_get_attached_object(obj_desc as *mut acpi_namespace_node);
            }
            match type_ {
                ACPI_TYPE_DEVICE | ACPI_TYPE_THERMAL => {}
                _ if obj_desc.is_null() => {
                    acpi_error!("Node is unresolved or uninitialized");
                    return AE_AML_UNINITIALIZED_NODE;
                }
                _ => {}
            }
        }
        _ => return AE_AML_OPERAND_TYPE,
    }

    if type_ == ACPI_TYPE_LOCAL_REFERENCE {
        while (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REFERENCE {
            match (*obj_desc).reference.class_ {
                ACPI_REFCLASS_REFOF | ACPI_REFCLASS_NAME => {
                    node = if (*obj_desc).reference.class_ == ACPI_REFCLASS_REFOF { (*obj_desc).reference.object } else { (*obj_desc).reference.node };
                    if ACPI_GET_DESCRIPTOR_TYPE(node) != ACPI_DESC_TYPE_NAMED { return AE_AML_INTERNAL; }
                    obj_desc = acpi_ns_get_attached_object(node);
                    if obj_desc.is_null() { type_ = acpi_ns_get_type(node); break; }
                    if obj_desc == operand { return AE_AML_CIRCULAR_REFERENCE; }
                }
                ACPI_REFCLASS_INDEX => {
                    type_ = (*obj_desc).reference.target_type;
                    if type_ != ACPI_TYPE_PACKAGE { break; }
                    obj_desc = *(*obj_desc).reference.where_;
                    if obj_desc.is_null() { type_ = 0; break; }
                }
                ACPI_REFCLASS_TABLE => { type_ = ACPI_TYPE_DDB_HANDLE; break; }
                ACPI_REFCLASS_LOCAL | ACPI_REFCLASS_ARG => {
                    if !return_desc.is_null() {
                        status = acpi_ds_method_data_get_value((*obj_desc).reference.class_, (*obj_desc).reference.value, walk_state, &mut obj_desc);
                        if ACPI_FAILURE(status) { return status; }
                        acpi_ut_remove_reference(obj_desc);
                    } else {
                        let mut n = core::ptr::null_mut();
                        status = acpi_ds_method_data_get_node((*obj_desc).reference.class_, (*obj_desc).reference.value, walk_state, &mut n);
                        if ACPI_FAILURE(status) { return status; }
                        obj_desc = acpi_ns_get_attached_object(n);
                        if obj_desc.is_null() { type_ = ACPI_TYPE_ANY; break; }
                    }
                }
                ACPI_REFCLASS_DEBUG => { type_ = ACPI_TYPE_DEBUG_OBJECT; break; }
                _ => return AE_AML_INTERNAL,
            }
        }
        if !obj_desc.is_null() { type_ = (*obj_desc).common.type_; }
    }

    match type_ {
        ACPI_TYPE_LOCAL_REGION_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD | ACPI_TYPE_LOCAL_INDEX_FIELD => type_ = ACPI_TYPE_FIELD_UNIT,
        ACPI_TYPE_LOCAL_SCOPE => type_ = ACPI_TYPE_ANY,
        _ => {}
    }
    *return_type = type_;
    if !return_desc.is_null() { *return_desc = obj_desc; }
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
