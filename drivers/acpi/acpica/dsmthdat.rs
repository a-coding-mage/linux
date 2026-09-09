// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: dsmthdat - control method arguments and local variables
//
// C dependencies are supplied by the surrounding ACPI translation unit.

/* Local prototypes */
unsafe fn acpi_ds_method_data_delete_value(
    type_: u8,
    index: u32,
    walk_state: *mut acpi_walk_state,
);

unsafe fn acpi_ds_method_data_set_value(
    type_: u8,
    index: u32,
    object: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status;

// ACPI_OBSOLETE_FUNCTIONS
// unsafe fn acpi_ds_method_data_get_type(
//     opcode: u16, index: u32, walk_state: *mut acpi_walk_state,
// ) -> acpi_object_type;

pub unsafe fn acpi_ds_method_data_init(walk_state: *mut acpi_walk_state) {
    for i in 0..ACPI_METHOD_NUM_ARGS {
        (*walk_state).arguments[i].name = NAMEOF_ARG_NTE;
        (*walk_state).arguments[i].name |= (i as u32) << 24;
        (*walk_state).arguments[i].descriptor_type = ACPI_DESC_TYPE_NAMED;
        (*walk_state).arguments[i].type_ = ACPI_TYPE_ANY;
        (*walk_state).arguments[i].flags = ANOBJ_METHOD_ARG;
    }

    for i in 0..ACPI_METHOD_NUM_LOCALS {
        (*walk_state).local_variables[i].name = NAMEOF_LOCAL_NTE;
        (*walk_state).local_variables[i].name |= (i as u32) << 24;
        (*walk_state).local_variables[i].descriptor_type = ACPI_DESC_TYPE_NAMED;
        (*walk_state).local_variables[i].type_ = ACPI_TYPE_ANY;
        (*walk_state).local_variables[i].flags = ANOBJ_METHOD_LOCAL;
    }
}

pub unsafe fn acpi_ds_method_data_delete_all(walk_state: *mut acpi_walk_state) {
    for index in 0..ACPI_METHOD_NUM_LOCALS {
        if !(*walk_state).local_variables[index].object.is_null() {
            acpi_ns_detach_object(&mut (*walk_state).local_variables[index]);
        }
    }

    for index in 0..ACPI_METHOD_NUM_ARGS {
        if !(*walk_state).arguments[index].object.is_null() {
            acpi_ns_detach_object(&mut (*walk_state).arguments[index]);
        }
    }
}

pub unsafe fn acpi_ds_method_data_init_args(
    params: *mut *mut acpi_operand_object,
    max_param_count: u32,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    if params.is_null() {
        return AE_OK;
    }

    let mut index: u32 = 0;
    while index < ACPI_METHOD_NUM_ARGS
        && index < max_param_count
        && !(*params.add(index as usize)).is_null()
    {
        let status = acpi_ds_method_data_set_value(
            ACPI_REFCLASS_ARG,
            index,
            *params.add(index as usize),
            walk_state,
        );
        if ACPI_FAILURE(status) {
            return status;
        }
        index += 1;
    }
    acpi_ex_trace_args(params, index);
    AE_OK
}

pub unsafe fn acpi_ds_method_data_get_node(
    type_: u8,
    index: u32,
    walk_state: *mut acpi_walk_state,
    node: *mut *mut acpi_namespace_node,
) -> acpi_status {
    match type_ {
        ACPI_REFCLASS_LOCAL => {
            if index > ACPI_METHOD_MAX_LOCAL {
                return AE_AML_INVALID_INDEX;
            }
            *node = &mut (*walk_state).local_variables[index as usize];
        }
        ACPI_REFCLASS_ARG => {
            if index > ACPI_METHOD_MAX_ARG {
                return AE_AML_INVALID_INDEX;
            }
            *node = &mut (*walk_state).arguments[index as usize];
        }
        _ => return AE_TYPE,
    }
    AE_OK
}

unsafe fn acpi_ds_method_data_set_value(
    type_: u8,
    index: u32,
    object: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_ds_method_data_get_node(type_, index, walk_state, &mut node);
    if ACPI_FAILURE(status) {
        return status;
    }
    acpi_ut_add_reference(object);
    (*node).object = object;
    status
}

pub unsafe fn acpi_ds_method_data_get_value(
    type_: u8,
    index: u32,
    walk_state: *mut acpi_walk_state,
    dest_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    if dest_desc.is_null() {
        return AE_BAD_PARAMETER;
    }

    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_ds_method_data_get_node(type_, index, walk_state, &mut node);
    if ACPI_FAILURE(status) {
        return status;
    }

    let mut object = (*node).object;
    if object.is_null() {
        if acpi_gbl_enable_interpreter_slack {
            object = acpi_ut_create_integer_object(0);
            if object.is_null() {
                return AE_NO_MEMORY;
            }
            (*node).object = object;
        } else {
            return match type_ {
                ACPI_REFCLASS_ARG => AE_AML_UNINITIALIZED_ARG,
                ACPI_REFCLASS_LOCAL => AE_AML_UNINITIALIZED_LOCAL,
                _ => AE_AML_INTERNAL,
            };
        }
    }

    *dest_desc = object;
    acpi_ut_add_reference(object);
    AE_OK
}

unsafe fn acpi_ds_method_data_delete_value(
    type_: u8,
    index: u32,
    walk_state: *mut acpi_walk_state,
) {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    if ACPI_FAILURE(acpi_ds_method_data_get_node(type_, index, walk_state, &mut node)) {
        return;
    }

    let object = acpi_ns_get_attached_object(node);
    (*node).object = core::ptr::null_mut();
    if !object.is_null() && ACPI_GET_DESCRIPTOR_TYPE(object) == ACPI_DESC_TYPE_OPERAND {
        acpi_ut_remove_reference(object);
    }
}

pub unsafe fn acpi_ds_store_object_to_local(
    type_: u8,
    index: u32,
    obj_desc: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    if obj_desc.is_null() {
        return AE_BAD_PARAMETER;
    }

    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_ds_method_data_get_node(type_, index, walk_state, &mut node);
    if ACPI_FAILURE(status) {
        return status;
    }

    let current_obj_desc = acpi_ns_get_attached_object(node);
    if current_obj_desc == obj_desc {
        return status;
    }

    let mut new_obj_desc = obj_desc;
    if (*obj_desc).common.reference_count > 1 {
        let copied = acpi_ut_copy_iobject_to_iobject(obj_desc, &mut new_obj_desc, walk_state);
        if ACPI_FAILURE(copied) {
            return copied;
        }
    }

    if !current_obj_desc.is_null()
        && type_ == ACPI_REFCLASS_ARG
        && ACPI_GET_DESCRIPTOR_TYPE(current_obj_desc) == ACPI_DESC_TYPE_OPERAND
        && (*current_obj_desc).common.type_ == ACPI_TYPE_LOCAL_REFERENCE
        && (*current_obj_desc).reference.class_ == ACPI_REFCLASS_REFOF
    {
        let result = acpi_ex_store_object_to_node(
            new_obj_desc,
            (*current_obj_desc).reference.object,
            walk_state,
            ACPI_NO_IMPLICIT_CONVERSION,
        );
        if new_obj_desc != obj_desc {
            acpi_ut_remove_reference(new_obj_desc);
        }
        return result;
    }

    if !current_obj_desc.is_null() {
        acpi_ds_method_data_delete_value(type_, index, walk_state);
    }

    let result = acpi_ds_method_data_set_value(type_, index, new_obj_desc, walk_state);
    if new_obj_desc != obj_desc {
        acpi_ut_remove_reference(new_obj_desc);
    }
    result
}

// ACPI_OBSOLETE_FUNCTIONS
// pub unsafe fn acpi_ds_method_data_get_type(
//     opcode: u16, index: u32, walk_state: *mut acpi_walk_state,
// ) -> acpi_object_type {
//     let mut node = core::ptr::null_mut();
//     if ACPI_FAILURE(acpi_ds_method_data_get_node(opcode as u8, index, walk_state, &mut node)) {
//         return ACPI_TYPE_NOT_FOUND;
//     }
//     let object = acpi_ns_get_attached_object(node);
//     if object.is_null() { ACPI_TYPE_ANY } else { (*object).type_ }
// }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
