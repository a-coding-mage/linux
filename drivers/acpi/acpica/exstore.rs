// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// AML Interpreter object store support
//
// C headers and build-time component/module macros are supplied by the ACPI
// translation environment.

unsafe fn acpi_ex_store_object_to_index(
    source_desc: *mut acpi_operand_object,
    index_desc: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut status = AE_OK;
    let mut obj_desc: *mut acpi_operand_object;
    let mut new_desc: *mut acpi_operand_object;
    let mut value: u8 = 0;
    let mut i: u32;

    match (*index_desc).reference.target_type {
        ACPI_TYPE_PACKAGE => {
            obj_desc = *(*index_desc).reference.where_;
            if (*source_desc).common.type_ == ACPI_TYPE_LOCAL_REFERENCE
                && (*source_desc).reference.class_ == ACPI_REFCLASS_TABLE
            {
                acpi_ut_add_reference(source_desc);
                new_desc = source_desc;
            } else {
                status = acpi_ut_copy_iobject_to_iobject(source_desc, &mut new_desc, walk_state);
                if ACPI_FAILURE(status) {
                    return status;
                }
            }

            if !obj_desc.is_null() {
                for i in 0..(*( (*index_desc).reference.object as *mut acpi_operand_object)).common.reference_count {
                    acpi_ut_remove_reference(obj_desc);
                }
            }
            *(*index_desc).reference.where_ = new_desc;
            for i in 1..(*( (*index_desc).reference.object as *mut acpi_operand_object)).common.reference_count {
                acpi_ut_add_reference(new_desc);
            }
        }
        ACPI_TYPE_BUFFER_FIELD => {
            obj_desc = (*index_desc).reference.object;
            if (*obj_desc).common.type_ != ACPI_TYPE_BUFFER
                && (*obj_desc).common.type_ != ACPI_TYPE_STRING
            {
                return AE_AML_OPERAND_TYPE;
            }
            match (*source_desc).common.type_ {
                ACPI_TYPE_INTEGER => value = (*source_desc).integer.value as u8,
                ACPI_TYPE_BUFFER | ACPI_TYPE_STRING => value = *(*source_desc).buffer.pointer,
                _ => return AE_AML_OPERAND_TYPE,
            }
            *(*obj_desc).buffer.pointer.add((*index_desc).reference.value as usize) = value;
        }
        _ => status = AE_AML_TARGET_TYPE,
    }
    status
}

pub unsafe fn acpi_ex_store(
    source_desc: *mut acpi_operand_object,
    dest_desc: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    if source_desc.is_null() || dest_desc.is_null() {
        return AE_AML_NO_OPERAND;
    }
    if ACPI_GET_DESCRIPTOR_TYPE(dest_desc) == ACPI_DESC_TYPE_NAMED {
        return acpi_ex_store_object_to_node(source_desc, dest_desc as *mut acpi_namespace_node, walk_state, ACPI_IMPLICIT_CONVERSION);
    }
    match (*dest_desc).common.type_ {
        ACPI_TYPE_LOCAL_REFERENCE => {}
        ACPI_TYPE_INTEGER if (*dest_desc).common.flags & AOPOBJ_AML_CONSTANT != 0 => return AE_OK,
        _ => return AE_AML_OPERAND_TYPE,
    }
    match (*dest_desc).reference.class_ {
        ACPI_REFCLASS_REFOF => acpi_ex_store_object_to_node(source_desc, (*dest_desc).reference.object as *mut acpi_namespace_node, walk_state, ACPI_IMPLICIT_CONVERSION),
        ACPI_REFCLASS_INDEX => acpi_ex_store_object_to_index(source_desc, dest_desc, walk_state),
        ACPI_REFCLASS_LOCAL | ACPI_REFCLASS_ARG => acpi_ds_store_object_to_local((*dest_desc).reference.class_, (*dest_desc).reference.value, source_desc, walk_state),
        ACPI_REFCLASS_DEBUG => AE_OK,
        _ => AE_AML_INTERNAL,
    }
}

pub unsafe fn acpi_ex_store_object_to_node(
    source_desc: *mut acpi_operand_object,
    node: *mut acpi_namespace_node,
    walk_state: *mut acpi_walk_state,
    implicit_conversion: u8,
) -> acpi_status {
    let mut status = AE_OK;
    let target_desc: *mut acpi_operand_object;
    let mut new_desc: *mut acpi_operand_object;
    let target_type: acpi_object_type;

    target_type = acpi_ns_get_type(node);
    target_desc = acpi_ns_get_attached_object(node);

    if (*walk_state).opcode != AML_COPY_OBJECT_OP {
        match target_type {
            ACPI_TYPE_PACKAGE if (*walk_state).opcode == AML_STORE_OP => {
                if (*source_desc).common.type_ != ACPI_TYPE_PACKAGE { return AE_AML_TARGET_TYPE; }
            }
            ACPI_TYPE_PACKAGE | ACPI_TYPE_DEVICE | ACPI_TYPE_EVENT | ACPI_TYPE_MUTEX
            | ACPI_TYPE_REGION | ACPI_TYPE_POWER | ACPI_TYPE_PROCESSOR | ACPI_TYPE_THERMAL => return AE_AML_TARGET_TYPE,
            _ => {}
        }
    }

    status = acpi_ex_resolve_object(&mut (source_desc as *mut acpi_operand_object), target_type, walk_state);
    if ACPI_FAILURE(status) { return status; }
    match target_type {
        ACPI_TYPE_INTEGER | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => {
            if (*walk_state).opcode == AML_COPY_OBJECT_OP || implicit_conversion == 0 {
                status = acpi_ex_store_direct_to_node(source_desc, node, walk_state);
            } else {
                status = acpi_ex_store_object_to_object(source_desc, target_desc, &mut new_desc, walk_state);
                if ACPI_FAILURE(status) { return status; }
                if new_desc != target_desc { status = acpi_ns_attach_object(node, new_desc, (*new_desc).common.type_); }
            }
        }
        ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_REGION_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD | ACPI_TYPE_LOCAL_INDEX_FIELD => {
            status = acpi_ex_write_data_to_field(source_desc, target_desc, &mut (*walk_state).result_obj);
        }
        _ => status = acpi_ex_store_direct_to_node(source_desc, node, walk_state),
    }
    status
}

unsafe fn acpi_ex_store_direct_to_node(
    source_desc: *mut acpi_operand_object,
    node: *mut acpi_namespace_node,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut new_desc: *mut acpi_operand_object;
    let status = acpi_ut_copy_iobject_to_iobject(source_desc, &mut new_desc, walk_state);
    if ACPI_FAILURE(status) { return status; }
    let status = acpi_ns_attach_object(node, new_desc, (*new_desc).common.type_);
    acpi_ut_remove_reference(new_desc);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
