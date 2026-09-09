// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: dbutils - AML debugger utilities

// Dependencies supplied by the ACPI implementation are intentionally external.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn acpi_db_match_argument(
    user_argument: *mut i8,
    arguments: *mut acpi_db_argument_info,
) -> acpi_object_type {
    let mut i: u32 = 0;
    if user_argument.is_null() || *user_argument == 0 {
        return ACPI_TYPE_NOT_FOUND;
    }
    while (*arguments.add(i as usize)).name != core::ptr::null() {
        let name = (*arguments.add(i as usize)).name as *const i8;
        if c_strstr(name, user_argument) == (*arguments.add(i as usize)).name {
            return i as acpi_object_type;
        }
        i += 1;
    }
    ACPI_TYPE_NOT_FOUND
}

pub unsafe fn acpi_db_set_output_destination(output_flags: u32) {
    acpi_gbl_db_output_flags = output_flags as u8;
    if (output_flags & ACPI_DB_REDIRECTABLE_OUTPUT) != 0 && acpi_gbl_db_output_to_file {
        acpi_dbg_level = acpi_gbl_db_debug_level;
    } else {
        acpi_dbg_level = acpi_gbl_db_console_debug_level;
    }
}

pub unsafe fn acpi_db_dump_external_object(obj_desc: *mut acpi_object, level: u32) {
    if obj_desc.is_null() {
        acpi_os_printf(b"[Null Object]\n\0".as_ptr() as *const i8);
        return;
    }
    for _ in 0..level {
        acpi_os_printf(b" \0".as_ptr() as *const i8);
    }
    match (*obj_desc).type_ {
        ACPI_TYPE_ANY => { acpi_os_printf(b"[Null Object] (Type=0)\n\0".as_ptr() as *const i8); }
        ACPI_TYPE_INTEGER => {
            acpi_os_printf(b"[Integer] = %8.8X%8.8X\n\0".as_ptr() as *const i8,
                ((*obj_desc).integer.value >> 32) as u32, (*obj_desc).integer.value as u32);
        }
        ACPI_TYPE_STRING => {
            acpi_os_printf(b"[String] Length %.2X = \0".as_ptr() as *const i8, (*obj_desc).string.length);
            acpi_ut_print_string((*obj_desc).string.pointer, ACPI_UINT8_MAX);
            acpi_os_printf(b"\n\0".as_ptr() as *const i8);
        }
        ACPI_TYPE_BUFFER => {
            acpi_os_printf(b"[Buffer] Length %.2X = \0".as_ptr() as *const i8, (*obj_desc).buffer.length);
            if (*obj_desc).buffer.length != 0 {
                if (*obj_desc).buffer.length > 16 { acpi_os_printf(b"\n\0".as_ptr() as *const i8); }
                acpi_ut_debug_dump_buffer((*obj_desc).buffer.pointer, (*obj_desc).buffer.length, DB_BYTE_DISPLAY, _COMPONENT);
            } else { acpi_os_printf(b"\n\0".as_ptr() as *const i8); }
        }
        ACPI_TYPE_PACKAGE => {
            acpi_os_printf(b"[Package] Contains %u Elements:\n\0".as_ptr() as *const i8, (*obj_desc).package.count);
            for i in 0..(*obj_desc).package.count { acpi_db_dump_external_object((*obj_desc).package.elements.add(i as usize), level + 1); }
        }
        ACPI_TYPE_LOCAL_REFERENCE => {
            acpi_os_printf(b"[Object Reference] = \0".as_ptr() as *const i8);
            acpi_db_display_internal_object((*obj_desc).reference.handle, core::ptr::null_mut());
        }
        ACPI_TYPE_PROCESSOR => acpi_os_printf(b"[Processor]\n\0".as_ptr() as *const i8),
        ACPI_TYPE_POWER => acpi_os_printf(b"[Power Resource]\n\0".as_ptr() as *const i8),
        _ => acpi_os_printf(b"[Unknown Type] %X\n\0".as_ptr() as *const i8, (*obj_desc).type_),
    }
}

pub unsafe fn acpi_db_prep_namestring(mut name: *mut i8) {
    if name.is_null() { return; }
    acpi_ut_strupr(name);
    if *name == b'/' as i8 { *name = b'\\' as i8; }
    if acpi_is_root_prefix(*name) { name = name.add(1); }
    while *name != 0 {
        if *name == b'/' as i8 || *name == b'\\' as i8 { *name = b'.' as i8; }
        name = name.add(1);
    }
}

pub unsafe fn acpi_db_local_ns_lookup(name: *mut i8) -> *mut acpi_namespace_node {
    let mut internal_path: *mut i8 = core::ptr::null_mut();
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    acpi_db_prep_namestring(name);
    let status = acpi_ns_internalize_name(name, &mut internal_path);
    if acpi_failure(status) { acpi_os_printf(b"Invalid namestring: %s\n\0".as_ptr() as *const i8, name); return core::ptr::null_mut(); }
    let status = acpi_ns_lookup(core::ptr::null_mut(), internal_path, ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE, ACPI_NS_NO_UPSEARCH | ACPI_NS_DONT_OPEN_SCOPE, core::ptr::null_mut(), &mut node);
    if acpi_failure(status) { acpi_os_printf(b"Could not locate name: %s, %s\n\0".as_ptr() as *const i8, name, acpi_format_exception(status)); }
    acpi_free(internal_path as *mut core::ffi::c_void);
    node
}

pub unsafe fn acpi_db_uint32_to_hex_string(mut value: u32, buffer: *mut i8) {
    if value == 0 { c_strcpy(buffer, b"0\0".as_ptr() as *const i8); return; }
    *buffer.add(8) = 0;
    for i in (0..8).rev() { *buffer.add(i) = acpi_gbl_upper_hex_digits[(value & 0x0f) as usize] as i8; value >>= 4; }
}

#[cfg(feature = "ACPI_OBSOLETE_FUNCTIONS")]
pub unsafe fn acpi_db_second_pass_parse(mut op: *mut acpi_parse_object) -> acpi_status {
    let root = op;
    let mut status = AE_OK;
    acpi_function_entry();
    acpi_os_printf(b"Pass two parse ....\n\0".as_ptr() as *const i8);
    while !op.is_null() {
        if (*op).common.aml_opcode == AML_METHOD_OP {
            let method = op;
            let walk_state = acpi_ds_create_walk_state(0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
            if walk_state.is_null() { return AE_NO_MEMORY; }
            (*walk_state).parser_state.aml = (*method).named.data;
            (*walk_state).parser_state.aml_start = (*method).named.data;
            (*walk_state).parser_state.aml_end = (*method).named.data.add((*method).named.length as usize);
            (*walk_state).parser_state.pkg_end = (*method).named.data.add((*method).named.length as usize);
            (*walk_state).parser_state.start_scope = op;
            (*walk_state).descending_callback = Some(acpi_ds_load1_begin_op);
            (*walk_state).ascending_callback = Some(acpi_ds_load1_end_op);
            status = acpi_ps_parse_aml(walk_state);
            let base_aml_offset = (*(*method).common.value.arg).common.aml_offset + 1;
            let start_op = (*(*method).common.value.arg).common.next;
            let mut search_op = start_op;
            while !search_op.is_null() {
                (*search_op).common.aml_offset += base_aml_offset;
                search_op = acpi_ps_get_depth_next(start_op, search_op);
            }
        }
        if (*op).common.aml_opcode == AML_REGION_OP {
            // TBD: this is not quite the right thing to do, as in the original C source.
        }
        if acpi_failure(status) { break; }
        op = acpi_ps_get_depth_next(root, op);
    }
    status
}

#[cfg(feature = "ACPI_OBSOLETE_FUNCTIONS")]
pub unsafe fn acpi_db_dump_buffer(address: u32) {
    acpi_os_printf(b"\nLocation %X:\n\0".as_ptr() as *const i8, address);
    acpi_dbg_level |= ACPI_LV_TABLES;
    acpi_ut_debug_dump_buffer(address as *mut u8, 64, DB_BYTE_DISPLAY, ACPI_UINT32_MAX);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
