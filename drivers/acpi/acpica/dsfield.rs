// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: dsfield - Dispatcher field routines

// Dependencies supplied by the surrounding ACPICA translation.

#[cfg(feature = "acpi_asl_compiler")]
unsafe fn acpi_ds_create_external_region(
    lookup_status: acpi_status,
    op: *mut acpi_parse_object,
    path: *mut i8,
    walk_state: *mut acpi_walk_state,
    node: *mut *mut acpi_namespace_node,
) -> acpi_status {
    let mut status: acpi_status;
    let obj_desc: *mut acpi_operand_object;

    if lookup_status != AE_NOT_FOUND { return lookup_status; }

    acpi_dm_add_op_to_external_list(op, path, ACPI_TYPE_REGION, 0, 0);
    status = acpi_ns_lookup((*walk_state).scope_info, path, ACPI_TYPE_REGION,
        ACPI_IMODE_LOAD_PASS1, ACPI_NS_SEARCH_PARENT, walk_state, node);
    if ACPI_FAILURE(status) { return status; }

    obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_REGION);
    if obj_desc.is_null() { return AE_NO_MEMORY; }
    (*obj_desc).region.node = *node;
    status = acpi_ns_attach_object(*node, obj_desc, ACPI_TYPE_REGION);
    status
}

unsafe fn acpi_ds_get_field_names(
    info: *mut acpi_create_field_info,
    walk_state: *mut acpi_walk_state,
    mut arg: *mut acpi_parse_object,
) -> acpi_status {
    let mut status: acpi_status;
    let mut position: u64;
    let mut child: *mut acpi_parse_object;

    (*info).field_bit_position = 0;
    while !arg.is_null() {
        match (*arg).common.aml_opcode {
            AML_INT_RESERVEDFIELD_OP => {
                position = (*info).field_bit_position as u64 + (*arg).common.value.size as u64;
                if position > ACPI_UINT32_MAX as u64 { return AE_SUPPORT; }
                (*info).field_bit_position = position as u32;
            }
            AML_INT_ACCESSFIELD_OP | AML_INT_EXTACCESSFIELD_OP => {
                (*info).field_flags = (((*info).field_flags & !(AML_FIELD_ACCESS_TYPE_MASK as u8)) |
                    (((*arg).common.value.integer as u32 & 0x07) as u8));
                (*info).attribute = (((*arg).common.value.integer >> 8) & 0xFF) as u8;
                (*info).access_length = (((*arg).common.value.integer >> 16) & 0xFF) as u8;
            }
            AML_INT_CONNECTION_OP => {
                (*info).resource_buffer = core::ptr::null_mut();
                (*info).connection_node = core::ptr::null_mut();
                (*info).pin_number_index = 0;
                child = (*arg).common.value.arg;
                if (*child).common.aml_opcode == AML_INT_BYTELIST_OP {
                    (*info).resource_buffer = (*child).named.data;
                    (*info).resource_length = (*child).named.value.integer as u16;
                } else {
                    status = acpi_ns_lookup((*walk_state).scope_info, (*child).common.value.name,
                        ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE, ACPI_NS_DONT_OPEN_SCOPE,
                        walk_state, &mut (*info).connection_node);
                    if ACPI_FAILURE(status) { return status; }
                }
            }
            AML_INT_NAMEDFIELD_OP => {
                status = acpi_ns_lookup((*walk_state).scope_info,
                    &mut (*arg).named.name as *mut _ as *mut i8, (*info).field_type,
                    ACPI_IMODE_EXECUTE, ACPI_NS_DONT_OPEN_SCOPE, walk_state,
                    &mut (*info).field_node);
                if ACPI_FAILURE(status) { return status; }
                (*arg).common.node = (*info).field_node;
                (*info).field_bit_length = (*arg).common.value.size;
                if acpi_ns_get_attached_object((*info).field_node).is_null() {
                    status = acpi_ex_prep_field_value(info);
                    if ACPI_FAILURE(status) { return status; }
                }
                position = (*info).field_bit_position as u64 + (*arg).common.value.size as u64;
                if position > ACPI_UINT32_MAX as u64 { return AE_SUPPORT; }
                (*info).field_bit_position = (*info).field_bit_position.wrapping_add((*info).field_bit_length);
                (*info).pin_number_index = (*info).pin_number_index.wrapping_add(1);
            }
            _ => return AE_AML_BAD_OPCODE,
        }
        arg = (*arg).common.next;
    }
    AE_OK
}

pub unsafe fn acpi_ds_create_buffer_field(op: *mut acpi_parse_object, walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut arg: *mut acpi_parse_object;
    let mut node: *mut acpi_namespace_node;
    let mut status: acpi_status;
    let obj_desc: *mut acpi_operand_object;
    let mut second_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut flags: u32;

    arg = if (*op).common.aml_opcode == AML_CREATE_FIELD_OP { acpi_ps_get_arg(op, 3) } else { acpi_ps_get_arg(op, 2) };
    if arg.is_null() { return AE_AML_NO_OPERAND; }
    if !(*walk_state).deferred_node.is_null() { node = (*walk_state).deferred_node; }
    else {
        if ((*walk_state).parse_flags & ACPI_PARSE_EXECUTE) == 0 { return AE_AML_INTERNAL; }
        flags = ACPI_NS_NO_UPSEARCH | ACPI_NS_DONT_OPEN_SCOPE | ACPI_NS_ERROR_IF_FOUND;
        if !(*walk_state).method_node.is_null() && ((*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL) == 0 { flags |= ACPI_NS_TEMPORARY; }
        status = acpi_ns_lookup((*walk_state).scope_info, (*arg).common.value.string, ACPI_TYPE_ANY,
            ACPI_IMODE_LOAD_PASS1, flags, walk_state, &mut node);
        if ACPI_FAILURE(status) && !(((*walk_state).parse_flags & ACPI_PARSE_DISASSEMBLE) != 0 && status == AE_ALREADY_EXISTS) { return status; }
    }
    (*op).common.node = node;
    if !acpi_ns_get_attached_object(node).is_null() { return AE_OK; }
    obj_desc = acpi_ut_create_internal_object(ACPI_TYPE_BUFFER_FIELD);
    if obj_desc.is_null() { return AE_NO_MEMORY; }
    second_desc = (*obj_desc).common.next_object;
    (*second_desc).extra.aml_start = (*op).named.data;
    (*second_desc).extra.aml_length = (*op).named.length;
    (*obj_desc).buffer_field.node = node;
    status = acpi_ns_attach_object(node, obj_desc, ACPI_TYPE_BUFFER_FIELD);
    acpi_ut_remove_reference(obj_desc);
    status
}

pub unsafe fn acpi_ds_create_field(op: *mut acpi_parse_object, region_node: *mut acpi_namespace_node, walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut arg = (*op).common.value.arg;
    let mut info: acpi_create_field_info = core::mem::zeroed();
    let mut status: acpi_status;
    if region_node.is_null() {
        status = acpi_ns_lookup((*walk_state).scope_info, (*arg).common.value.name, ACPI_TYPE_REGION,
            ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut (region_node as *mut _));
        #[cfg(feature = "acpi_asl_compiler")]
        { status = acpi_ds_create_external_region(status, arg, (*arg).common.value.name, walk_state, &mut (region_node as *mut _)); }
        if ACPI_FAILURE(status) { return status; }
    }
    arg = (*arg).common.next; info.field_flags = (*arg).common.value.integer as u8; info.attribute = 0;
    info.field_type = ACPI_TYPE_LOCAL_REGION_FIELD; info.region_node = region_node;
    status = acpi_ds_get_field_names(&mut info, walk_state, (*arg).common.next);
    if ACPI_FAILURE(status) { return status; }
    if (*(*region_node).object).region.space_id == ACPI_ADR_SPACE_PLATFORM_COMM {
        (*(*region_node).object).field.internal_pcc_buffer = ACPI_ALLOCATE_ZEROED((*(*region_node).object).region.length);
        if (*(*region_node).object).field.internal_pcc_buffer.is_null() { return AE_NO_MEMORY; }
    }
    status
}

pub unsafe fn acpi_ds_init_field_objects(op: *mut acpi_parse_object, walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut arg: *mut acpi_parse_object; let mut node: *mut acpi_namespace_node = core::ptr::null_mut(); let mut typ: u8; let mut flags: u32;
    if ((*walk_state).parse_flags & ACPI_PARSE_EXECUTE) == 0 { if ((*walk_state).parse_flags & ACPI_PARSE_DEFERRED_OP) != 0 { return AE_OK; } return AE_AML_INTERNAL; }
    match (*walk_state).opcode { AML_FIELD_OP => { arg = acpi_ps_get_arg(op, 2); typ = ACPI_TYPE_LOCAL_REGION_FIELD; }, AML_BANK_FIELD_OP => { arg = acpi_ps_get_arg(op, 4); typ = ACPI_TYPE_LOCAL_BANK_FIELD; }, AML_INDEX_FIELD_OP => { arg = acpi_ps_get_arg(op, 3); typ = ACPI_TYPE_LOCAL_INDEX_FIELD; }, _ => return AE_BAD_PARAMETER }
    flags = ACPI_NS_NO_UPSEARCH | ACPI_NS_DONT_OPEN_SCOPE | ACPI_NS_ERROR_IF_FOUND;
    if !(*walk_state).method_node.is_null() && ((*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL) == 0 { flags |= ACPI_NS_TEMPORARY; }
    while !arg.is_null() { if (*arg).common.aml_opcode == AML_INT_NAMEDFIELD_OP { let _ = acpi_ns_lookup((*walk_state).scope_info, &mut (*arg).named.name as *mut _ as *mut i8, typ, ACPI_IMODE_LOAD_PASS1, flags, walk_state, &mut node); (*arg).common.node = node; } arg = (*arg).common.next; }
    AE_OK
}

pub unsafe fn acpi_ds_create_bank_field(op: *mut acpi_parse_object, region_node: *mut acpi_namespace_node, walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut arg = (*op).common.value.arg; let mut info: acpi_create_field_info = core::mem::zeroed(); let mut status: acpi_status;
    if region_node.is_null() { status = acpi_ns_lookup((*walk_state).scope_info, (*arg).common.value.name, ACPI_TYPE_REGION, ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut (region_node as *mut _)); #[cfg(feature="acpi_asl_compiler")] { status = acpi_ds_create_external_region(status, arg, (*arg).common.value.name, walk_state, &mut (region_node as *mut _)); } if ACPI_FAILURE(status) { return status; } }
    arg = (*arg).common.next; status = acpi_ns_lookup((*walk_state).scope_info, (*arg).common.value.string, ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut info.register_node); if ACPI_FAILURE(status) { return status; }
    arg = (*arg).common.next; arg = (*arg).common.next; info.field_flags = (*arg).common.value.integer as u8; info.field_type = ACPI_TYPE_LOCAL_BANK_FIELD; info.region_node = region_node; info.data_register_node = op as *mut acpi_namespace_node; acpi_ds_get_field_names(&mut info, walk_state, (*arg).common.next)
}

pub unsafe fn acpi_ds_create_index_field(op: *mut acpi_parse_object, region_node: *mut acpi_namespace_node, walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut arg = (*op).common.value.arg; let mut info: acpi_create_field_info = core::mem::zeroed(); let mut status: acpi_status;
    status = acpi_ns_lookup((*walk_state).scope_info, (*arg).common.value.string, ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut info.register_node); if ACPI_FAILURE(status) { return status; }
    arg = (*arg).common.next; status = acpi_ns_lookup((*walk_state).scope_info, (*arg).common.value.string, ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut info.data_register_node); if ACPI_FAILURE(status) { return status; }
    arg = (*arg).common.next; info.field_flags = (*arg).common.value.integer as u8; info.field_type = ACPI_TYPE_LOCAL_INDEX_FIELD; info.region_node = region_node; acpi_ds_get_field_names(&mut info, walk_state, (*arg).common.next)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
