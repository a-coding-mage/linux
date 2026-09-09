// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: utdebug - Debug print/trace routines
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

// C dependencies supplied by the surrounding ACPICA translation.

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
static mut ACPI_GBL_PREVIOUS_THREAD_ID: acpi_thread_id = 0xFFFF_FFFF as acpi_thread_id;
#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
static ACPI_GBL_FUNCTION_ENTRY_PREFIX: &[u8] = b"----Entry\0";
#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
static ACPI_GBL_FUNCTION_EXIT_PREFIX: &[u8] = b"----Exit-\0";

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_init_stack_ptr_trace() {
    let current_sp: acpi_size = 0;
    acpi_gbl_entry_stack_pointer = &current_sp as *const acpi_size;
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_track_stack_ptr() {
    let current_sp: acpi_size = 0;
    let current_ptr = &current_sp as *const acpi_size;
    if current_ptr < acpi_gbl_lowest_stack_pointer {
        acpi_gbl_lowest_stack_pointer = current_ptr;
    }
    if acpi_gbl_nesting_level > acpi_gbl_deepest_nesting {
        acpi_gbl_deepest_nesting = acpi_gbl_nesting_level;
    }
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
unsafe fn acpi_ut_trim_function_name(function_name: *const u8) -> *const u8 {
    let value = *(function_name as *const u32);
    if value == ACPI_PREFIX_MIXED {
        return function_name.add(4);
    }
    if value == ACPI_PREFIX_LOWER {
        return function_name.add(5);
    }
    function_name
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_debug_print(
    requested_debug_level: u32, line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, format: *const u8, ...
) {
    if !ACPI_IS_DEBUG_ENABLED(requested_debug_level, component_id) { return; }

    let thread_id = acpi_os_get_thread_id();
    if thread_id != ACPI_GBL_PREVIOUS_THREAD_ID {
        if ACPI_LV_THREADS & acpi_dbg_level != 0 {
            acpi_os_printf(b"\n**** Context Switch from TID %u to TID %u ****\n\n\0".as_ptr(),
                ACPI_GBL_PREVIOUS_THREAD_ID as u32, thread_id as u32);
        }
        ACPI_GBL_PREVIOUS_THREAD_ID = thread_id;
        acpi_gbl_nesting_level = 0;
    }

    acpi_os_printf(b"%9s-%04d \0".as_ptr(), module_name, line_number);
    #[cfg(feature = "ACPI_APPLICATION")]
    {
        if ACPI_LV_THREADS & acpi_dbg_level != 0 {
            acpi_os_printf(b"[%u] \0".as_ptr(), thread_id as u32);
        }
        let mut fill_count = 48 - acpi_gbl_nesting_level -
            acpi_ut_strlen(acpi_ut_trim_function_name(function_name)) as i32;
        if fill_count < 0 { fill_count = 0; }
        acpi_os_printf(b"[%02d] %*s\0".as_ptr(), acpi_gbl_nesting_level,
            acpi_gbl_nesting_level + 1, b" \0".as_ptr());
        acpi_os_printf(b"%s%*s: \0".as_ptr(), acpi_ut_trim_function_name(function_name),
            fill_count, b" \0".as_ptr());
    }
    #[cfg(not(feature = "ACPI_APPLICATION"))]
    acpi_os_printf(b"%-22.22s: \0".as_ptr(), acpi_ut_trim_function_name(function_name));

    // The C varargs list is forwarded unchanged to the ACPICA printf backend.
    acpi_os_vprintf(format, core::ptr::null_mut());
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_debug_print_raw(
    requested_debug_level: u32, _line_number: u32, _function_name: *const u8,
    _module_name: *const u8, component_id: u32, format: *const u8, ...
) {
    if !ACPI_IS_DEBUG_ENABLED(requested_debug_level, component_id) { return; }
    acpi_os_vprintf(format, core::ptr::null_mut());
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_trace(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32) {
    acpi_gbl_nesting_level += 1;
    acpi_ut_track_stack_ptr();
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s\n\0".as_ptr());
    }
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_trace_ptr(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, pointer: *const core::ffi::c_void) {
    acpi_gbl_nesting_level += 1; acpi_ut_track_stack_ptr();
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s %p\n\0".as_ptr());
    }
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_trace_str(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, string: *const u8) {
    acpi_gbl_nesting_level += 1; acpi_ut_track_stack_ptr();
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s %s\n\0".as_ptr());
    }
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_trace_u32(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, integer: u32) {
    acpi_gbl_nesting_level += 1; acpi_ut_track_stack_ptr();
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s %08X\n\0".as_ptr());
    }
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
unsafe fn acpi_ut_dec_nesting() { if acpi_gbl_nesting_level != 0 { acpi_gbl_nesting_level -= 1; } }

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_exit(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32) {
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s\n\0".as_ptr());
    }
    acpi_ut_dec_nesting();
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_status_exit(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, status: acpi_status) {
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        if ACPI_SUCCESS(status) {
            acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
                component_id, b"%s %s\n\0".as_ptr());
        } else {
            acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
                component_id, b"%s ****Exception****: %s\n\0".as_ptr());
        }
    }
    acpi_ut_dec_nesting();
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_value_exit(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, value: u64) {
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s %8.8X%8.8X\n\0".as_ptr());
    }
    acpi_ut_dec_nesting();
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_ptr_exit(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, ptr: *mut u8) {
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s %p\n\0".as_ptr());
    }
    acpi_ut_dec_nesting();
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_ut_str_exit(line_number: u32, function_name: *const u8,
    module_name: *const u8, component_id: u32, string: *const u8) {
    if ACPI_IS_DEBUG_ENABLED(ACPI_LV_FUNCTIONS, component_id) {
        acpi_debug_print(ACPI_LV_FUNCTIONS, line_number, function_name, module_name,
            component_id, b"%s %s\n\0".as_ptr());
    }
    acpi_ut_dec_nesting();
}

#[cfg(feature = "ACPI_DEBUG_OUTPUT")]
pub unsafe extern "C" fn acpi_trace_point(type_: acpi_trace_event_type, begin: u8,
    aml: *mut u8, pathname: *mut u8) {
    acpi_ex_trace_point(type_, begin, aml, pathname);
    #[cfg(feature = "ACPI_USE_SYSTEM_TRACER")]
    acpi_os_trace_point(type_, begin, aml, pathname);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
