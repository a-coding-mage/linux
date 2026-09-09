/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acstruct.h - Internal structs
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* acpisrc:struct_defs -- for acpisrc conversion */

/* Tree walking typedefs and structs */

/* Walk state - current state of a parse tree walk. Used for both a leisurely
 * stroll through the tree (for whatever reason), and for control method
 * execution.
 */
pub const ACPI_NEXT_OP_DOWNWARD: u32 = 1;
pub const ACPI_NEXT_OP_UPWARD: u32 = 2;

/* Groups of definitions for walk_type used for different implementations of
 * walkers (never simultaneously) - flags for interpreter:
 */
pub const ACPI_WALK_NON_METHOD: u32 = 0;
pub const ACPI_WALK_METHOD: u32 = 0x01;
pub const ACPI_WALK_METHOD_RESTART: u32 = 0x02;

#[repr(C)]
pub struct acpi_walk_state {
    pub next: *mut acpi_walk_state,
    pub descriptor_type: u8,
    pub walk_type: u8,
    pub opcode: u16,
    pub next_op_info: u8,
    pub num_operands: u8,
    pub operand_index: u8,
    pub owner_id: acpi_owner_id,
    pub last_predicate: u8,
    pub current_result: u8,
    pub return_used: u8,
    pub scope_depth: u8,
    pub pass_number: u8,
    pub namespace_override: u8,
    pub result_size: u8,
    pub result_count: u8,
    pub aml: *mut u8,
    pub arg_types: u32,
    pub method_breakpoint: u32,
    pub user_breakpoint: u32,
    pub parse_flags: u32,
    pub parser_state: acpi_parse_state,
    pub prev_arg_types: u32,
    pub arg_count: u32,
    pub method_nesting_depth: u16,
    pub method_is_nested: u8,
    pub arguments: [acpi_namespace_node; ACPI_METHOD_NUM_ARGS],
    pub local_variables: [acpi_namespace_node; ACPI_METHOD_NUM_LOCALS],
    pub operands: [*mut acpi_operand_object; ACPI_OBJ_NUM_OPERANDS + 1],
    pub params: *mut *mut acpi_operand_object,
    pub aml_last_while: *mut u8,
    pub caller_return_desc: *mut *mut acpi_operand_object,
    pub control_state: *mut acpi_generic_state,
    pub deferred_node: *mut acpi_namespace_node,
    pub implicit_return_obj: *mut acpi_operand_object,
    pub method_call_node: *mut acpi_namespace_node,
    pub method_call_op: *mut acpi_parse_object,
    pub method_desc: *mut acpi_operand_object,
    pub method_node: *mut acpi_namespace_node,
    pub method_pathname: *mut core::ffi::c_char,
    pub op: *mut acpi_parse_object,
    pub op_info: *const acpi_opcode_info,
    pub origin: *mut acpi_parse_object,
    pub result_obj: *mut acpi_operand_object,
    pub results: *mut acpi_generic_state,
    pub return_desc: *mut acpi_operand_object,
    pub scope_info: *mut acpi_generic_state,
    pub prev_op: *mut acpi_parse_object,
    pub next_op: *mut acpi_parse_object,
    pub thread: *mut acpi_thread_state,
    pub descending_callback: acpi_parse_downwards,
    pub ascending_callback: acpi_parse_upwards,
}

/* Info used by acpi_ns_initialize_objects and acpi_ds_initialize_objects */
#[repr(C)]
pub struct acpi_init_walk_info {
    pub table_index: u32, pub object_count: u32, pub method_count: u32,
    pub serial_method_count: u32, pub non_serial_method_count: u32,
    pub serialized_method_count: u32, pub device_count: u32,
    pub op_region_count: u32, pub field_count: u32, pub buffer_count: u32,
    pub package_count: u32, pub op_region_init: u32, pub field_init: u32,
    pub buffer_init: u32, pub package_init: u32, pub owner_id: acpi_owner_id,
}

#[repr(C)]
pub struct acpi_get_devices_info {
    pub user_function: acpi_walk_callback,
    pub context: *mut core::ffi::c_void,
    pub hid: *const core::ffi::c_char,
}

#[repr(C)]
pub struct acpi_aml_operands_fatal {
    pub type_: *mut acpi_object_integer, pub code: *mut acpi_object_integer,
    pub argument: *mut acpi_object_integer,
}
#[repr(C)]
pub struct acpi_aml_operands_index {
    pub source: *mut acpi_operand_object, pub index: *mut acpi_object_integer,
    pub target: *mut acpi_operand_object,
}
#[repr(C)]
pub struct acpi_aml_operands_mid {
    pub source: *mut acpi_operand_object, pub index: *mut acpi_object_integer,
    pub length: *mut acpi_object_integer, pub target: *mut acpi_operand_object,
}
#[repr(C)]
pub union acpi_aml_operands {
    pub operands: [*mut acpi_operand_object; 7],
    pub fatal: acpi_aml_operands_fatal,
    pub index: acpi_aml_operands_index,
    pub mid: acpi_aml_operands_mid,
}

/* Structure used to pass object evaluation information and parameters.
 * Purpose is to reduce CPU stack use.
 */
#[repr(C)]
pub struct acpi_evaluate_info {
    pub prefix_node: *mut acpi_namespace_node,
    pub relative_pathname: *const core::ffi::c_char,
    pub parameters: *mut *mut acpi_operand_object,
    pub node: *mut acpi_namespace_node,
    pub obj_desc: *mut acpi_operand_object,
    pub full_pathname: *mut core::ffi::c_char,
    pub predefined: *const acpi_predefined_info,
    pub return_object: *mut acpi_operand_object,
    pub parent_package: *mut acpi_operand_object,
    pub return_flags: u32,
    pub return_btype: u32,
    pub param_count: u16,
    pub node_flags: u16,
    pub pass_number: u8,
    pub return_object_type: u8,
    pub flags: u8,
}

pub const ACPI_IGNORE_RETURN_VALUE: u32 = 1;
pub const ACPI_OBJECT_REPAIRED: u32 = 1;
pub const ACPI_OBJECT_WRAPPED: u32 = 2;

#[repr(C)]
pub struct acpi_device_walk_info {
    pub table_desc: *mut acpi_table_desc,
    pub evaluate_info: *mut acpi_evaluate_info,
    pub device_count: u32, pub num_STA: u32, pub num_INI: u32,
}

/* Info used by Acpi  acpi_db_display_fields */
#[repr(C)]
pub struct acpi_region_walk_info {
    pub debug_level: u32, pub count: u32, pub owner_id: acpi_owner_id,
    pub display_type: u8, pub address_space_id: u32,
}

/* TBD: [Restructure] Merge with struct above */
#[repr(C)]
pub struct acpi_walk_info {
    pub debug_level: u32, pub count: u32, pub owner_id: acpi_owner_id,
    pub display_type: u8,
}

/* Display Types */
pub const ACPI_DISPLAY_SUMMARY: u8 = 0;
pub const ACPI_DISPLAY_OBJECTS: u8 = 1;
pub const ACPI_DISPLAY_MASK: u8 = 1;
pub const ACPI_DISPLAY_SHORT: u8 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
