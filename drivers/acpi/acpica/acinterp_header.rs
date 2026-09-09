/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/*
 * Name: acinterp.h - Interpreter subcomponent prototypes and defines
 *
 * Rust translation of the C header. External types and functions are supplied
 * by other translation units.
 */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
use core::ffi::{c_char, c_void};

// C macro: (&(walk_state->operands [walk_state->num_operands -1]))
macro_rules! ACPI_WALK_OPERANDS { ($walk_state:expr) => {
    unsafe { (&mut (*$walk_state).operands[(*$walk_state).num_operands.wrapping_sub(1)]) }
}; }

/* Macros for tables used for debug output. */
// ACPI_EXD_OFFSET and ACPI_EXD_NSOFFSET use C offsetof.
macro_rules! ACPI_EXD_OFFSET { ($f:tt) => { core::mem::offset_of!(acpi_operand_object, $f) as u8 }; }
macro_rules! ACPI_EXD_NSOFFSET { ($f:tt) => { core::mem::offset_of!(acpi_namespace_node, $f) as u8 }; }
macro_rules! ACPI_EXD_TABLE_SIZE { ($name:expr) => { core::mem::size_of_val(&$name) / core::mem::size_of::<acpi_exdump_info>() }; }

#[repr(C, packed)]
pub struct acpi_exdump_info {
    pub opcode: u8,
    pub offset: u8,
    pub name: *const c_char,
}

pub const ACPI_EXD_INIT: u8 = 0;
pub const ACPI_EXD_TYPE: u8 = 1;
pub const ACPI_EXD_UINT8: u8 = 2;
pub const ACPI_EXD_UINT16: u8 = 3;
pub const ACPI_EXD_UINT32: u8 = 4;
pub const ACPI_EXD_UINT64: u8 = 5;
pub const ACPI_EXD_LITERAL: u8 = 6;
pub const ACPI_EXD_POINTER: u8 = 7;
pub const ACPI_EXD_ADDRESS: u8 = 8;
pub const ACPI_EXD_STRING: u8 = 9;
pub const ACPI_EXD_BUFFER: u8 = 10;
pub const ACPI_EXD_PACKAGE: u8 = 11;
pub const ACPI_EXD_FIELD: u8 = 12;
pub const ACPI_EXD_REFERENCE: u8 = 13;
pub const ACPI_EXD_LIST: u8 = 14;
pub const ACPI_EXD_HDLR_LIST: u8 = 15;
pub const ACPI_EXD_RGN_LIST: u8 = 16;
pub const ACPI_EXD_NODE: u8 = 17;

extern "C" {
    pub fn acpi_ex_convert_to_integer(obj_desc: *mut acpi_operand_object, result_desc: *mut *mut acpi_operand_object, implicit_conversion: u32) -> acpi_status;
    pub fn acpi_ex_convert_to_buffer(obj_desc: *mut acpi_operand_object, result_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_convert_to_string(obj_desc: *mut acpi_operand_object, result_desc: *mut *mut acpi_operand_object, ty: u32) -> acpi_status;
    pub fn acpi_ex_convert_to_target_type(destination_type: acpi_object_type, source_desc: *mut acpi_operand_object, result_desc: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_do_debug_object(source_desc: *mut acpi_operand_object, level: u32, index: u32);
    pub fn acpi_ex_start_trace_method(method_node: *mut acpi_namespace_node, obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_ex_stop_trace_method(method_node: *mut acpi_namespace_node, obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_ex_start_trace_opcode(op: *mut acpi_parse_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_ex_stop_trace_opcode(op: *mut acpi_parse_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_ex_trace_point(ty: acpi_trace_event_type, begin: u8, aml: *mut u8, pathname: *mut c_char);
    pub fn acpi_ex_trace_args(params: *mut *mut acpi_operand_object, count: u32);
    pub fn acpi_ex_get_protocol_buffer_length(protocol_id: u32, return_length: *mut u32) -> acpi_status;
    pub fn acpi_ex_common_buffer_setup(obj_desc: *mut acpi_operand_object, buffer_length: u32, datum_count: *mut u32) -> acpi_status;
    pub fn acpi_ex_write_with_update_rule(obj_desc: *mut acpi_operand_object, mask: u64, field_value: u64, field_datum_byte_offset: u32) -> acpi_status;
    pub fn acpi_ex_get_buffer_datum(datum: *mut u64, buffer: *mut c_void, buffer_length: u32, byte_granularity: u32, buffer_offset: u32);
    pub fn acpi_ex_set_buffer_datum(merged_datum: u64, buffer: *mut c_void, buffer_length: u32, byte_granularity: u32, buffer_offset: u32);
    pub fn acpi_ex_read_data_from_field(walk_state: *mut acpi_walk_state, obj_desc: *mut acpi_operand_object, ret_buffer_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_write_data_to_field(source_desc: *mut acpi_operand_object, obj_desc: *mut acpi_operand_object, result_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_extract_from_field(obj_desc: *mut acpi_operand_object, buffer: *mut c_void, buffer_length: u32) -> acpi_status;
    pub fn acpi_ex_insert_into_field(obj_desc: *mut acpi_operand_object, buffer: *mut c_void, buffer_length: u32) -> acpi_status;
    pub fn acpi_ex_access_region(obj_desc: *mut acpi_operand_object, field_datum_byte_offset: u32, value: *mut u64, read_write: u32) -> acpi_status;
    pub fn acpi_ex_get_object_reference(obj_desc: *mut acpi_operand_object, return_desc: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_concat_template(obj_desc: *mut acpi_operand_object, obj_desc2: *mut acpi_operand_object, actual_return_desc: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_do_concatenate(obj_desc: *mut acpi_operand_object, obj_desc2: *mut acpi_operand_object, actual_return_desc: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_do_logical_numeric_op(opcode: u16, integer0: u64, integer1: u64, logical_result: *mut u8) -> acpi_status;
    pub fn acpi_ex_do_logical_op(opcode: u16, operand0: *mut acpi_operand_object, operand1: *mut acpi_operand_object, logical_result: *mut u8) -> acpi_status;
    pub fn acpi_ex_do_math_op(opcode: u16, operand0: u64, operand1: u64) -> u64;
    pub fn acpi_ex_create_mutex(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_create_processor(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_create_power_resource(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_create_region(aml_start: *mut u8, aml_length: u32, region_space: u8, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_create_event(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_create_alias(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_create_method(aml_start: *mut u8, aml_length: u32, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_load_op(obj_desc: *mut acpi_operand_object, target: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_load_table_op(walk_state: *mut acpi_walk_state, return_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_unload_table(ddb_handle: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_acquire_mutex(time_desc: *mut acpi_operand_object, obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_acquire_mutex_object(timeout: u16, obj_desc: *mut acpi_operand_object, thread_id: acpi_thread_id) -> acpi_status;
    pub fn acpi_ex_release_mutex(obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_release_mutex_object(obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_release_all_mutexes(thread: *mut acpi_thread_state);
    pub fn acpi_ex_unlink_mutex(obj_desc: *mut acpi_operand_object);
    pub fn acpi_ex_prep_common_field_object(obj_desc: *mut acpi_operand_object, field_flags: u8, field_attribute: u8, field_bit_position: u32, field_bit_length: u32) -> acpi_status;
    pub fn acpi_ex_prep_field_value(info: *mut acpi_create_field_info) -> acpi_status;
    pub fn acpi_ex_read_serial_bus(obj_desc: *mut acpi_operand_object, return_buffer: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_write_serial_bus(source_desc: *mut acpi_operand_object, obj_desc: *mut acpi_operand_object, return_buffer: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_read_gpio(obj_desc: *mut acpi_operand_object, buffer: *mut c_void) -> acpi_status;
    pub fn acpi_ex_write_gpio(source_desc: *mut acpi_operand_object, obj_desc: *mut acpi_operand_object, return_buffer: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_system_do_notify_op(value: *mut acpi_operand_object, obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_system_do_sleep(time: u64) -> acpi_status;
    pub fn acpi_ex_system_do_stall(time: u32) -> acpi_status;
    pub fn acpi_ex_system_signal_event(obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_system_wait_event(time: *mut acpi_operand_object, obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_system_reset_event(obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_system_wait_semaphore(semaphore: acpi_semaphore, timeout: u16) -> acpi_status;
    pub fn acpi_ex_system_wait_mutex(mutex: acpi_mutex, timeout: u16) -> acpi_status;
}

pub const ACPI_EXPLICIT_BYTE_COPY: u32 = 0;
pub const ACPI_EXPLICIT_CONVERT_HEX: u32 = 1;
pub const ACPI_IMPLICIT_CONVERT_HEX: u32 = 2;
pub const ACPI_EXPLICIT_CONVERT_DECIMAL: u32 = 3;

extern "C" {
    pub fn acpi_ex_opcode_0A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_1A_0T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_1A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_1A_1T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_1A_1T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_2A_0T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_2A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_2A_1T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_2A_2T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_3A_0T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_3A_1T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_opcode_6A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_resolve_to_value(stack_ptr: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_resolve_multiple(walk_state: *mut acpi_walk_state, operand: *mut acpi_operand_object, return_type: *mut acpi_object_type, return_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_resolve_node_to_value(stack_ptr: *mut *mut acpi_namespace_node, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_resolve_operands(opcode: u16, stack_ptr: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_dump_operand(obj_desc: *mut acpi_operand_object, depth: u32);
    pub fn acpi_ex_dump_operands(operands: *mut *mut acpi_operand_object, opcode_name: *const c_char, num_opcodes: u32);
    pub fn acpi_ex_dump_object_descriptor(object: *mut acpi_operand_object, flags: u32);
    pub fn acpi_ex_dump_namespace_node(node: *mut acpi_namespace_node, flags: u32);
    pub fn acpi_ex_get_name_string(data_type: acpi_object_type, in_aml_address: *mut u8, out_name_string: *mut *mut c_char, out_name_length: *mut u32) -> acpi_status;
    pub fn acpi_ex_store(val_desc: *mut acpi_operand_object, dest_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_store_object_to_node(source_desc: *mut acpi_operand_object, node: *mut acpi_namespace_node, walk_state: *mut acpi_walk_state, implicit_conversion: u8) -> acpi_status;
    pub fn acpi_ex_resolve_object(source_desc_ptr: *mut *mut acpi_operand_object, target_type: acpi_object_type, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_store_object_to_object(source_desc: *mut acpi_operand_object, dest_desc: *mut acpi_operand_object, new_desc: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ex_store_buffer_to_buffer(source_desc: *mut acpi_operand_object, target_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_store_string_to_string(source_desc: *mut acpi_operand_object, target_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_copy_integer_to_index_field(source_desc: *mut acpi_operand_object, target_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_copy_integer_to_bank_field(source_desc: *mut acpi_operand_object, target_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_copy_data_to_named_field(source_desc: *mut acpi_operand_object, node: *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ex_copy_integer_to_buffer_field(source_desc: *mut acpi_operand_object, target_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ex_enter_interpreter();
    pub fn acpi_ex_exit_interpreter();
    pub fn acpi_ex_truncate_for32bit_table(obj_desc: *mut acpi_operand_object) -> u8;
    pub fn acpi_ex_acquire_global_lock(rule: u32);
    pub fn acpi_ex_release_global_lock(rule: u32);
    pub fn acpi_ex_eisa_id_to_string(dest: *mut c_char, compressed_id: u64);
    pub fn acpi_ex_integer_to_string(dest: *mut c_char, value: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
