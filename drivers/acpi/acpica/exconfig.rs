// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: exconfig - Namespace reconfiguration (Load/Unload opcodes)
 */

/* C dependencies are supplied by the surrounding ACPICA translation. */

unsafe fn acpi_ex_add_table(
    table_index: u32,
    ddb_handle: *mut *mut acpi_operand_object,
) -> acpi_status {
    let obj_desc: *mut acpi_operand_object;

    /* Create an object to be the table handle */
    obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_LOCAL_REFERENCE);
    if obj_desc.is_null() {
        return AE_NO_MEMORY;
    }

    /* Init the table handle */
    (*obj_desc).common.flags |= AOPOBJ_DATA_VALID;
    (*obj_desc).reference.class = ACPI_REFCLASS_TABLE;
    (*obj_desc).reference.value = table_index as u64;
    *ddb_handle = obj_desc;
    AE_OK
}

pub unsafe fn acpi_ex_load_table_op(
    walk_state: *mut acpi_walk_state,
    return_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut status: acpi_status;
    let operand = (*walk_state).operands.as_mut_ptr();
    let mut parent_node: *mut acpi_namespace_node;
    let mut start_node: *mut acpi_namespace_node;
    let mut parameter_node: *mut acpi_namespace_node = core::ptr::null_mut();
    let return_obj: *mut acpi_operand_object;
    let mut ddb_handle: *mut acpi_operand_object = core::ptr::null_mut();
    let mut table_index: u32 = 0;
    let mut oem_id = [0i8; ACPI_OEM_ID_SIZE + 1];
    let mut oem_table_id = [0i8; ACPI_OEM_TABLE_ID_SIZE + 1];

    /* Create the return object */
    return_obj = acpi_ut_create_integer_object(0);
    if return_obj.is_null() {
        return AE_NO_MEMORY;
    }
    *return_desc = return_obj;

    /* Validate OEM ID and OEM Table ID string lengths. */
    if (*(*operand.add(1)).string.length > ACPI_OEM_ID_SIZE as u32
        || (*(*operand.add(2)).string.length > ACPI_OEM_TABLE_ID_SIZE as u32)
    {
        return AE_AML_STRING_LIMIT;
    }

    /* Copy OEM strings to local buffers with guaranteed null-termination. */
    core::ptr::copy_nonoverlapping(
        (*(*operand.add(1)).string.pointer).cast::<i8>(),
        oem_id.as_mut_ptr(),
        (*(*operand.add(1)).string.length) as usize,
    );
    oem_id[(*(*operand.add(1)).string.length) as usize] = 0;
    core::ptr::copy_nonoverlapping(
        (*(*operand.add(2)).string.pointer).cast::<i8>(),
        oem_table_id.as_mut_ptr(),
        (*(*operand.add(2)).string.length) as usize,
    );
    oem_table_id[(*(*operand.add(2)).string.length) as usize] = 0;

    acpi_ex_exit_interpreter();
    status = acpi_tb_find_table(
        (*(*operand.add(0)).string.pointer).cast(),
        oem_id.as_ptr(),
        oem_table_id.as_ptr(),
        &mut table_index,
    );
    acpi_ex_enter_interpreter();
    if ACPI_FAILURE(status) {
        if status != AE_NOT_FOUND {
            return status;
        }
        return AE_OK;
    }

    start_node = (*(*(*walk_state).scope_info).scope).node;
    parent_node = acpi_gbl_root_node;

    if (*(*operand.add(3)).string.length > 0) {
        status = acpi_ns_get_node_unlocked(
            start_node,
            (*(*operand.add(3)).string.pointer).cast(),
            ACPI_NS_SEARCH_PARENT,
            &mut parent_node,
        );
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    if (*(*operand.add(4)).string.length > 0) {
        let first = *(*(*operand.add(4)).string.pointer);
        if first != AML_ROOT_PREFIX && first != AML_PARENT_PREFIX {
            start_node = parent_node;
        }
        status = acpi_ns_get_node_unlocked(
            start_node,
            (*(*operand.add(4)).string.pointer).cast(),
            ACPI_NS_SEARCH_PARENT,
            &mut parameter_node,
        );
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    acpi_ex_exit_interpreter();
    status = acpi_tb_load_table(table_index, parent_node);
    acpi_ex_enter_interpreter();
    if ACPI_FAILURE(status) {
        return status;
    }
    status = acpi_ex_add_table(table_index, &mut ddb_handle);
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_ex_exit_interpreter();
    acpi_ns_initialize_objects();
    acpi_ex_enter_interpreter();

    if !parameter_node.is_null() {
        status = acpi_ex_store(
            *operand.add(5),
            parameter_node.cast::<acpi_operand_object>(),
            walk_state,
        );
        if ACPI_FAILURE(status) {
            acpi_ex_unload_table(ddb_handle);
            acpi_ut_remove_reference(ddb_handle);
            return status;
        }
    }
    acpi_ut_remove_reference(ddb_handle);
    (*return_obj).integer.value = 0xFFFF_FFFF_FFFF_FFFF;
    status
}

unsafe fn acpi_ex_region_read(
    obj_desc: *mut acpi_operand_object,
    length: u32,
    mut buffer: *mut u8,
) -> acpi_status {
    let mut region_offset: u32 = 0;
    for _ in 0..length {
        let mut value: u64 = 0;
        let status = acpi_ev_address_space_dispatch(
            obj_desc,
            core::ptr::null_mut(),
            ACPI_READ,
            region_offset,
            8,
            &mut value,
        );
        if ACPI_FAILURE(status) {
            return status;
        }
        *buffer = value as u8;
        buffer = buffer.add(1);
        region_offset += 1;
    }
    AE_OK
}

pub unsafe fn acpi_ex_load_op(
    obj_desc: *mut acpi_operand_object,
    target: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut ddb_handle: *mut acpi_operand_object = core::ptr::null_mut();
    let mut table_header: *mut acpi_table_header;
    let mut table: *mut acpi_table_header;
    let mut table_index: u32 = 0;
    let mut status: acpi_status;
    let mut length: u32;

    if (*target).common.descriptor_type == ACPI_DESC_TYPE_NAMED {
        target = acpi_ns_get_attached_object(target.cast::<acpi_namespace_node>());
    }
    if (*target).common.type_ != ACPI_TYPE_INTEGER {
        return AE_AML_OPERAND_TYPE;
    }
    (*target).integer.value = 0;

    match (*obj_desc).common.type_ {
        ACPI_TYPE_REGION => {
            if (*obj_desc).region.space_id != ACPI_ADR_SPACE_SYSTEM_MEMORY {
                return AE_AML_OPERAND_TYPE;
            }
            if (*obj_desc).common.flags & AOPOBJ_DATA_VALID == 0 {
                status = acpi_ds_get_region_arguments(obj_desc);
                if ACPI_FAILURE(status) { return status; }
            }
            table_header = acpi_allocate(core::mem::size_of::<acpi_table_header>() as u32);
            if table_header.is_null() { return AE_NO_MEMORY; }
            status = acpi_ex_region_read(obj_desc, core::mem::size_of::<acpi_table_header>() as u32, table_header.cast());
            length = (*table_header).length;
            acpi_free(table_header.cast());
            if ACPI_FAILURE(status) { return status; }
            if length < core::mem::size_of::<acpi_table_header>() as u32 { return AE_INVALID_TABLE_LENGTH; }
            table = acpi_allocate(length);
            if table.is_null() { return AE_NO_MEMORY; }
            status = acpi_ex_region_read(obj_desc, length, table.cast());
            if ACPI_FAILURE(status) { acpi_free(table.cast()); return status; }
        }
        ACPI_TYPE_BUFFER => {
            if (*obj_desc).buffer.length < core::mem::size_of::<acpi_table_header>() as u32 { return AE_INVALID_TABLE_LENGTH; }
            table_header = (*obj_desc).buffer.pointer.cast();
            length = (*table_header).length;
            if length > (*obj_desc).buffer.length { return AE_AML_BUFFER_LIMIT; }
            if length < core::mem::size_of::<acpi_table_header>() as u32 { return AE_INVALID_TABLE_LENGTH; }
            table = acpi_allocate(length);
            if table.is_null() { return AE_NO_MEMORY; }
            core::ptr::copy_nonoverlapping(table_header.cast::<u8>(), table.cast::<u8>(), length as usize);
        }
        _ => return AE_AML_OPERAND_TYPE,
    }

    acpi_ex_exit_interpreter();
    status = acpi_tb_install_and_load_table(acpi_ptr_to_physaddr(table), ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL, table, true, &mut table_index);
    acpi_ex_enter_interpreter();
    if ACPI_FAILURE(status) { acpi_free(table.cast()); return status; }
    status = acpi_ex_add_table(table_index, &mut ddb_handle);
    if ACPI_FAILURE(status) { return status; }
    acpi_ex_exit_interpreter();
    acpi_ns_initialize_objects();
    acpi_ex_enter_interpreter();
    acpi_ut_remove_reference(ddb_handle);
    (*target).integer.value = 0xFFFF_FFFF_FFFF_FFFF;
    status
}

pub unsafe fn acpi_ex_unload_table(ddb_handle: *mut acpi_operand_object) -> acpi_status {
    let mut status = AE_OK;
    let table_desc = ddb_handle;
    if ddb_handle.is_null()
        || acpi_get_descriptor_type(ddb_handle) != ACPI_DESC_TYPE_OPERAND
        || (*ddb_handle).common.type_ != ACPI_TYPE_LOCAL_REFERENCE
        || (*ddb_handle).common.flags & AOPOBJ_DATA_VALID == 0
    {
        return AE_AML_OPERAND_TYPE;
    }
    let table_index = (*table_desc).reference.value as u32;
    acpi_ex_exit_interpreter();
    status = acpi_tb_unload_table(table_index);
    acpi_ex_enter_interpreter();
    if ACPI_SUCCESS(status) {
        (*ddb_handle).common.flags &= !AOPOBJ_DATA_VALID;
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
