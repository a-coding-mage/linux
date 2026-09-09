// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: extrace - Support for interpreter execution tracing
//
// Copyright (C) 2000 - 2026, Intel Corp.

// C dependencies supplied by the surrounding ACPICA translation.

static mut acpi_gbl_trace_method_object: *mut acpi_operand_object = core::ptr::null_mut();

#[cfg(feature = "acpi_debug_output")]
unsafe extern "C" {
    fn acpi_ex_get_trace_event_name(type_: acpi_trace_event_type) -> *const core::ffi::c_char;
}

unsafe fn acpi_ex_interpreter_trace_enabled(name: *mut core::ffi::c_char) -> u8 {
    if (acpi_gbl_trace_flags & ACPI_TRACE_ENABLED) == 0 {
        return FALSE;
    }

    if !acpi_gbl_trace_method_object.is_null() {
        return TRUE;
    }

    if !name.is_null()
        && !acpi_gbl_trace_method_name.is_null()
        && acpi_strcmp(acpi_gbl_trace_method_name, name) != 0
    {
        return FALSE;
    }

    if (acpi_gbl_trace_flags & ACPI_TRACE_ONESHOT) != 0
        && acpi_gbl_trace_method_name.is_null()
    {
        return FALSE;
    }

    TRUE
}

#[cfg(feature = "acpi_debug_output")]
unsafe fn acpi_ex_get_trace_event_name_local(type_: acpi_trace_event_type) -> &'static core::ffi::CStr {
    match type_ {
        ACPI_TRACE_AML_METHOD => c"Method",
        ACPI_TRACE_AML_OPCODE => c"Opcode",
        ACPI_TRACE_AML_REGION => c"Region",
        _ => c"",
    }
}

pub unsafe extern "C" fn acpi_ex_trace_point(
    type_: acpi_trace_event_type,
    begin: u8,
    aml: *mut u8,
    pathname: *mut core::ffi::c_char,
) {
    if !pathname.is_null() {
        acpi_debug_print_trace_point(
            type_,
            if begin != 0 { c"Begin" } else { c"End" },
            pathname,
            aml,
        );
    } else {
        acpi_debug_print_trace_point(
            type_,
            if begin != 0 { c"Begin" } else { c"End" },
            core::ptr::null_mut(),
            aml,
        );
    }
}

pub unsafe extern "C" fn acpi_ex_trace_args(
    params: *mut *mut acpi_operand_object,
    count: u32,
) {
    let mut i = 0;
    while i < count {
        let obj_desc = *params.add(i as usize);
        if i == 0 {
            acpi_debug_print_raw_trace_point(c" ");
        }

        match (*obj_desc).common.type_ {
            ACPI_TYPE_INTEGER => {
                acpi_debug_print_integer_trace_point((*obj_desc).integer.value);
            }
            ACPI_TYPE_STRING => {
                if (*obj_desc).string.length == 0 {
                    acpi_debug_print_raw_trace_point(c"NULL");
                    i += 1;
                    continue;
                }
                if acpi_is_debug_enabled(ACPI_LV_TRACE_POINT, ACPI_EXECUTER) != 0 {
                    acpi_ut_print_string((*obj_desc).string.pointer, ACPI_UINT8_MAX);
                }
            }
            _ => acpi_debug_print_raw_trace_point(c"Unknown"),
        }

        if i + 1 == count {
            acpi_debug_print_raw_trace_point(c"\n");
        } else {
            acpi_debug_print_raw_trace_point(c", ");
        }
        i += 1;
    }
}

pub unsafe extern "C" fn acpi_ex_start_trace_method(
    method_node: *mut acpi_namespace_node,
    obj_desc: *mut acpi_operand_object,
    _walk_state: *mut acpi_walk_state,
) {
    let mut pathname = core::ptr::null_mut();
    if !method_node.is_null() {
        pathname = acpi_ns_get_normalized_pathname(method_node, TRUE);
    }

    let enabled = acpi_ex_interpreter_trace_enabled(pathname);
    if enabled != 0 && acpi_gbl_trace_method_object.is_null() {
        acpi_gbl_trace_method_object = obj_desc;
        acpi_gbl_original_dbg_level = acpi_dbg_level;
        acpi_gbl_original_dbg_layer = acpi_dbg_layer;
        acpi_dbg_level = ACPI_TRACE_LEVEL_ALL;
        acpi_dbg_layer = ACPI_TRACE_LAYER_ALL;
        if acpi_gbl_trace_dbg_level != 0 { acpi_dbg_level = acpi_gbl_trace_dbg_level; }
        if acpi_gbl_trace_dbg_layer != 0 { acpi_dbg_layer = acpi_gbl_trace_dbg_layer; }
    }

    if enabled != 0 {
        acpi_ex_trace_point(ACPI_TRACE_AML_METHOD, TRUE,
            if !obj_desc.is_null() { (*obj_desc).method.aml_start } else { core::ptr::null_mut() }, pathname);
    }
    if !pathname.is_null() { acpi_free(pathname); }
}

pub unsafe extern "C" fn acpi_ex_stop_trace_method(
    method_node: *mut acpi_namespace_node,
    obj_desc: *mut acpi_operand_object,
    _walk_state: *mut acpi_walk_state,
) {
    let mut pathname = core::ptr::null_mut();
    if !method_node.is_null() { pathname = acpi_ns_get_normalized_pathname(method_node, TRUE); }
    let enabled = acpi_ex_interpreter_trace_enabled(core::ptr::null_mut());
    if enabled != 0 {
        acpi_ex_trace_point(ACPI_TRACE_AML_METHOD, FALSE,
            if !obj_desc.is_null() { (*obj_desc).method.aml_start } else { core::ptr::null_mut() }, pathname);
    }
    if acpi_gbl_trace_method_object == obj_desc {
        if (acpi_gbl_trace_flags & ACPI_TRACE_ONESHOT) != 0 { acpi_gbl_trace_method_name = core::ptr::null_mut(); }
        acpi_dbg_level = acpi_gbl_original_dbg_level;
        acpi_dbg_layer = acpi_gbl_original_dbg_layer;
        acpi_gbl_trace_method_object = core::ptr::null_mut();
    }
    if !pathname.is_null() { acpi_free(pathname); }
}

pub unsafe extern "C" fn acpi_ex_start_trace_opcode(op: *mut acpi_parse_object, _walk_state: *mut acpi_walk_state) {
    if acpi_ex_interpreter_trace_enabled(core::ptr::null_mut()) != 0 && (acpi_gbl_trace_flags & ACPI_TRACE_OPCODE) != 0 {
        acpi_ex_trace_point(ACPI_TRACE_AML_OPCODE, TRUE, (*op).common.aml, (*op).common.aml_op_name);
    }
}

pub unsafe extern "C" fn acpi_ex_stop_trace_opcode(op: *mut acpi_parse_object, _walk_state: *mut acpi_walk_state) {
    if acpi_ex_interpreter_trace_enabled(core::ptr::null_mut()) != 0 && (acpi_gbl_trace_flags & ACPI_TRACE_OPCODE) != 0 {
        acpi_ex_trace_point(ACPI_TRACE_AML_OPCODE, FALSE, (*op).common.aml, (*op).common.aml_op_name);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
