/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acdebug.h - ACPI/AML debugger
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* The debugger is used in conjunction with the disassembler most of time. */
/* The ACPI_DISASSEMBLER conditional include is supplied by the surrounding build. */

use std::os::raw::c_char;

pub const ACPI_DEBUG_BUFFER_SIZE: usize = 0x4000; /* 16K buffer for return objects */
pub const ACPI_DEBUG_LENGTH_FORMAT: &str = " (%.4X bits, %.3X bytes)";

#[repr(C)]
pub struct acpi_db_command_info {
    pub name: *const c_char, /* Command Name */
    pub min_args: u8,        /* Minimum arguments required */
}

#[repr(C)]
pub struct acpi_db_command_help {
    pub line_count: u8,       /* Number of help lines */
    pub invocation: *mut c_char, /* Command Invocation */
    pub description: *mut c_char, /* Command Description */
}

#[repr(C)]
pub struct acpi_db_argument_info {
    pub name: *const c_char, /* Argument Name */
}

#[repr(C)]
pub struct acpi_db_execute_walk {
    pub count: u32,
    pub max_count: u32,
    pub name_seg: [c_char; ACPI_NAMESEG_SIZE + 1],
}

pub const EX_NO_SINGLE_STEP: u32 = 1;
pub const EX_SINGLE_STEP: u32 = 2;
pub const EX_ALL: u32 = 4;

/* PARAM_LIST(pl) pl */
/* ACPI_DBR_DEPENDENT_RETURN_* and ACPI_HW_DEPENDENT_RETURN_* wrappers retain
 * build-dependent declaration attributes in the original header. */

extern "C" {
    pub fn acpi_db_single_step(
        walk_state: *mut acpi_walk_state,
        op: *mut acpi_parse_object,
        op_type: u32,
    ) -> acpi_status;
    pub fn acpi_db_signal_break_point(walk_state: *mut acpi_walk_state);

    pub fn acpi_db_convert_to_node(in_string: *mut c_char) -> *mut acpi_namespace_node;
    pub fn acpi_db_display_table_info(table_arg: *mut c_char);
    pub fn acpi_db_display_template(buffer_arg: *mut c_char);
    pub fn acpi_db_unload_acpi_table(name: *mut c_char);
    pub fn acpi_db_send_notify(name: *mut c_char, value: u32);
    pub fn acpi_db_display_interfaces(action_arg: *mut c_char, interface_name_arg: *mut c_char);
    pub fn acpi_db_sleep(object_arg: *mut c_char) -> acpi_status;
    pub fn acpi_db_trace(enable_arg: *mut c_char, method_arg: *mut c_char, once_arg: *mut c_char);
    pub fn acpi_db_display_locks();
    pub fn acpi_db_display_resources(object_arg: *mut c_char);
    pub fn acpi_db_display_gpes();
    pub fn acpi_db_display_handlers();
    pub fn acpi_db_generate_gpe(gpe_arg: *mut c_char, block_arg: *mut c_char);
    pub fn acpi_db_generate_sci();
    pub fn acpi_db_execute_test(type_arg: *mut c_char);

    pub fn acpi_db_hex_char_to_value(hex_char: i32, return_value: *mut u8) -> acpi_status;
    pub fn acpi_db_convert_to_package(string: *mut c_char, object: *mut acpi_object) -> acpi_status;
    pub fn acpi_db_convert_to_object(type_: acpi_object_type, string: *mut c_char, object: *mut acpi_object) -> acpi_status;
    pub fn acpi_db_encode_pld_buffer(pld_info: *mut acpi_pld_info) -> *mut u8;
    pub fn acpi_db_dump_pld_buffer(obj_desc: *mut acpi_object);

    pub fn acpi_db_set_method_breakpoint(location: *mut c_char, walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object);
    pub fn acpi_db_set_method_call_breakpoint(op: *mut acpi_parse_object);
    pub fn acpi_db_set_method_data(type_arg: *mut c_char, index_arg: *mut c_char, value_arg: *mut c_char);
    pub fn acpi_db_disassemble_method(name: *mut c_char) -> acpi_status;
    pub fn acpi_db_disassemble_aml(statements: *mut c_char, op: *mut acpi_parse_object);
    pub fn acpi_db_evaluate_predefined_names();
    pub fn acpi_db_evaluate_all(name_seg: *mut c_char);

    pub fn acpi_db_set_scope(name: *mut c_char);
    pub fn acpi_db_dump_namespace(start_arg: *mut c_char, depth_arg: *mut c_char);
    pub fn acpi_db_dump_namespace_paths();
    pub fn acpi_db_dump_namespace_by_owner(owner_arg: *mut c_char, depth_arg: *mut c_char);
    pub fn acpi_db_find_name_in_namespace(name_arg: *mut c_char) -> acpi_status;
    pub fn acpi_db_check_predefined_names();
    pub fn acpi_db_display_objects(obj_type_arg: *mut c_char, display_count_arg: *mut c_char) -> acpi_status;
    pub fn acpi_db_check_integrity();
    pub fn acpi_db_find_references(object_arg: *mut c_char);
    pub fn acpi_db_get_bus_info();
    pub fn acpi_db_display_fields(address_space_id: u32) -> acpi_status;

    pub fn acpi_db_display_method_info(op: *mut acpi_parse_object);
    pub fn acpi_db_decode_and_display_object(target: *mut c_char, output_type: *mut c_char);
    pub fn acpi_db_display_result_object(obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_db_display_all_methods(display_count_arg: *mut c_char) -> acpi_status;
    pub fn acpi_db_display_arguments();
    pub fn acpi_db_display_locals();
    pub fn acpi_db_display_results();
    pub fn acpi_db_display_calling_tree();
    pub fn acpi_db_display_object_type(object_arg: *mut c_char);
    pub fn acpi_db_display_argument_object(obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state);

    pub fn acpi_db_execute(name: *mut c_char, args: *mut *mut c_char, types: *mut acpi_object_type, flags: u32);
    pub fn acpi_db_create_execution_thread(method_name_arg: *mut c_char, arguments: *mut *mut c_char, types: *mut acpi_object_type);
    pub fn acpi_db_create_execution_threads(num_threads_arg: *mut c_char, num_loops_arg: *mut c_char, method_name_arg: *mut c_char);
    pub fn acpi_db_delete_objects(count: u32, objects: *mut acpi_object);
    /* Present only when ACPI_DBG_TRACK_ALLOCATIONS is enabled. */
    pub fn acpi_db_get_cache_info(cache: *mut acpi_memory_list) -> u32;

    pub fn acpi_db_match_argument(user_argument: *mut c_char, arguments: *mut acpi_db_argument_info) -> acpi_object_type;
    pub fn acpi_db_close_debug_file();
    pub fn acpi_db_open_debug_file(name: *mut c_char);
    pub fn acpi_db_load_acpi_table(filename: *mut c_char) -> acpi_status;
    pub fn acpi_db_load_tables(list_head: *mut acpi_new_table_desc) -> acpi_status;

    pub fn acpi_db_add_to_history(command_line: *mut c_char);
    pub fn acpi_db_display_history();
    pub fn acpi_db_get_from_history(command_num_arg: *mut c_char) -> *mut c_char;
    pub fn acpi_db_get_history_by_index(commandd_num: u32) -> *mut c_char;

    pub fn acpi_db_command_dispatch(input_buffer: *mut c_char, walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_db_execute_thread(context: *mut std::ffi::c_void);
    pub fn acpi_db_user_commands() -> acpi_status;
    pub fn acpi_db_get_next_token(string: *mut c_char, next: *mut *mut c_char, return_type: *mut acpi_object_type) -> *mut c_char;

    pub fn acpi_db_decode_internal_object(obj_desc: *mut acpi_operand_object);
    pub fn acpi_db_display_internal_object(obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_db_decode_arguments(walk_state: *mut acpi_walk_state);
    pub fn acpi_db_decode_locals(walk_state: *mut acpi_walk_state);
    pub fn acpi_db_dump_method_info(status: acpi_status, walk_state: *mut acpi_walk_state);

    pub fn acpi_db_generate_statistics(root: *mut acpi_parse_object, is_method: u8);
    pub fn acpi_db_display_statistics(type_arg: *mut c_char) -> acpi_status;
    pub fn acpi_db_set_output_destination(where_: u32);
    pub fn acpi_db_dump_external_object(obj_desc: *mut acpi_object, level: u32);
    pub fn acpi_db_prep_namestring(name: *mut c_char);
    pub fn acpi_db_local_ns_lookup(name: *mut c_char) -> *mut acpi_namespace_node;
    pub fn acpi_db_uint32_to_hex_string(value: u32, buffer: *mut c_char);
    pub fn acpi_db_generate_interrupt(gsiv_arg: *mut c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
