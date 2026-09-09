/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* AML Parser subcomponent prototypes and defines. */

use core::ffi::c_char;

pub const OP_HAS_RETURN_VALUE: u32 = 1;

/* Variable number of arguments. This field must be 32 bits. */
pub const ACPI_VAR_ARGS: u32 = u32::MAX;

pub const ACPI_PARSE_DELETE_TREE: u32 = 0x0001;
pub const ACPI_PARSE_NO_TREE_DELETE: u32 = 0x0000;
pub const ACPI_PARSE_TREE_MASK: u32 = 0x0001;

pub const ACPI_PARSE_LOAD_PASS1: u32 = 0x0010;
pub const ACPI_PARSE_LOAD_PASS2: u32 = 0x0020;
pub const ACPI_PARSE_EXECUTE: u32 = 0x0030;
pub const ACPI_PARSE_MODE_MASK: u32 = 0x0030;

pub const ACPI_PARSE_DEFERRED_OP: u32 = 0x0100;
pub const ACPI_PARSE_DISASSEMBLE: u32 = 0x0200;
pub const ACPI_PARSE_MODULE_LEVEL: u32 = 0x0400;

unsafe extern "C" {
    pub static acpi_gbl_short_op_index: [u8; 0];
    pub static acpi_gbl_long_op_index: [u8; 0];

    pub fn acpi_ps_execute_method(info: *mut acpi_evaluate_info) -> acpi_status;
    pub fn acpi_ps_execute_table(info: *mut acpi_evaluate_info) -> acpi_status;

    pub fn acpi_ps_get_next_package_end(parser_state: *mut acpi_parse_state) -> *mut u8;
    pub fn acpi_ps_get_next_namestring(parser_state: *mut acpi_parse_state) -> *mut c_char;
    pub fn acpi_ps_get_next_simple_arg(
        parser_state: *mut acpi_parse_state,
        arg_type: u32,
        arg: *mut acpi_parse_object,
    );
    pub fn acpi_ps_get_next_namepath(
        walk_state: *mut acpi_walk_state,
        parser_state: *mut acpi_parse_state,
        arg: *mut acpi_parse_object,
        possible_method_call: u8,
    ) -> acpi_status;
    pub fn acpi_ps_get_next_arg(
        walk_state: *mut acpi_walk_state,
        parser_state: *mut acpi_parse_state,
        arg_type: u32,
        return_arg: *mut *mut acpi_parse_object,
    ) -> acpi_status;

    pub fn acpi_ps_find_name(
        scope: *mut acpi_parse_object,
        name: u32,
        opcode: u32,
    ) -> *mut acpi_parse_object;
    pub fn acpi_ps_get_parent(op: *mut acpi_parse_object) -> *mut acpi_parse_object;

    pub fn acpi_ps_build_named_op(
        walk_state: *mut acpi_walk_state,
        aml_op_start: *mut u8,
        unnamed_op: *mut acpi_parse_object,
        op: *mut *mut acpi_parse_object,
    ) -> acpi_status;
    pub fn acpi_ps_create_op(
        walk_state: *mut acpi_walk_state,
        aml_op_start: *mut u8,
        new_op: *mut *mut acpi_parse_object,
    ) -> acpi_status;
    pub fn acpi_ps_complete_op(
        walk_state: *mut acpi_walk_state,
        op: *mut *mut acpi_parse_object,
        status: acpi_status,
    ) -> acpi_status;
    pub fn acpi_ps_complete_final_op(
        walk_state: *mut acpi_walk_state,
        op: *mut acpi_parse_object,
        status: acpi_status,
    ) -> acpi_status;

    pub fn acpi_ps_get_opcode_info(opcode: u16) -> *const acpi_opcode_info;
    pub fn acpi_ps_get_opcode_name(opcode: u16) -> *const c_char;
    pub fn acpi_ps_get_argument_count(op_type: u32) -> u8;

    pub fn acpi_ps_parse_aml(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ps_get_opcode_size(opcode: u32) -> u32;
    pub fn acpi_ps_peek_opcode(state: *mut acpi_parse_state) -> u16;
    pub fn acpi_ps_complete_this_op(
        walk_state: *mut acpi_walk_state,
        op: *mut acpi_parse_object,
    ) -> acpi_status;
    pub fn acpi_ps_next_parse_state(
        walk_state: *mut acpi_walk_state,
        op: *mut acpi_parse_object,
        callback_status: acpi_status,
    ) -> acpi_status;
    pub fn acpi_ps_parse_loop(walk_state: *mut acpi_walk_state) -> acpi_status;

    pub fn acpi_ps_init_scope(
        parser_state: *mut acpi_parse_state,
        root: *mut acpi_parse_object,
    ) -> acpi_status;
    pub fn acpi_ps_get_parent_scope(state: *mut acpi_parse_state) -> *mut acpi_parse_object;
    pub fn acpi_ps_has_completed_scope(parser_state: *mut acpi_parse_state) -> u8;
    pub fn acpi_ps_pop_scope(
        parser_state: *mut acpi_parse_state,
        op: *mut *mut acpi_parse_object,
        arg_list: *mut u32,
        arg_count: *mut u32,
    );
    pub fn acpi_ps_push_scope(
        parser_state: *mut acpi_parse_state,
        op: *mut acpi_parse_object,
        remaining_args: u32,
        arg_count: u32,
    ) -> acpi_status;
    pub fn acpi_ps_cleanup_scope(state: *mut acpi_parse_state);

    pub fn acpi_ps_append_arg(op: *mut acpi_parse_object, arg: *mut acpi_parse_object);
    pub fn acpi_ps_find(
        scope: *mut acpi_parse_object,
        path: *mut c_char,
        opcode: u16,
        create: u32,
    ) -> *mut acpi_parse_object;
    pub fn acpi_ps_get_arg(op: *mut acpi_parse_object, argn: u32) -> *mut acpi_parse_object;
    pub fn acpi_ps_get_depth_next(
        origin: *mut acpi_parse_object,
        op: *mut acpi_parse_object,
    ) -> *mut acpi_parse_object;

    pub fn acpi_ps_walk_parsed_aml(
        start_op: *mut acpi_parse_object,
        end_op: *mut acpi_parse_object,
        mth_desc: *mut acpi_operand_object,
        start_node: *mut acpi_namespace_node,
        params: *mut *mut acpi_operand_object,
        caller_return_desc: *mut *mut acpi_operand_object,
        owner_id: acpi_owner_id,
        descending_callback: acpi_parse_downwards,
        ascending_callback: acpi_parse_upwards,
    ) -> acpi_status;
    pub fn acpi_ps_get_next_walk_op(
        walk_state: *mut acpi_walk_state,
        op: *mut acpi_parse_object,
        ascending_callback: acpi_parse_upwards,
    ) -> acpi_status;
    pub fn acpi_ps_delete_completed_op(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ps_delete_parse_tree(root: *mut acpi_parse_object);

    pub fn acpi_ps_create_scope_op(aml: *mut u8) -> *mut acpi_parse_object;
    pub fn acpi_ps_init_op(op: *mut acpi_parse_object, opcode: u16);
    pub fn acpi_ps_alloc_op(opcode: u16, aml: *mut u8) -> *mut acpi_parse_object;
    pub fn acpi_ps_free_op(op: *mut acpi_parse_object);
    pub fn acpi_ps_is_leading_char(c: u32) -> u8;
    pub fn acpi_ps_get_name(op: *mut acpi_parse_object) -> u32;
    pub fn acpi_ps_set_name(op: *mut acpi_parse_object, name: u32);

    pub fn acpi_ps_sprint_path(
        buffer_start: *mut c_char,
        buffer_size: u32,
        op: *mut acpi_parse_object,
    ) -> u32;
    pub fn acpi_ps_sprint_op(
        buffer_start: *mut c_char,
        buffer_size: u32,
        op: *mut acpi_parse_object,
    ) -> u32;
    pub fn acpi_ps_show(op: *mut acpi_parse_object);
}

pub const ACPI_NOT_METHOD_CALL: u8 = 0;
pub const ACPI_POSSIBLE_METHOD_CALL: u8 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
