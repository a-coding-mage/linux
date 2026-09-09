/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acdispat.h - dispatcher (parser to interpreter interface)
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

pub const NAMEOF_LOCAL_NTE: &[u8] = b"__L0\0";
pub const NAMEOF_ARG_NTE: &[u8] = b"__A0\0";

/* dsargs - execution of dynamic arguments for static objects */
/* dscontrol - support for execution control opcodes */
/* dsopcode - support for late operand evaluation */
/* dsexec - Parser/Interpreter interface, method execution callbacks */
/* dsfield - Parser/Interpreter interface for AML fields */
/* dsload - Parser/Interpreter interface */
/* dsmthdat - method data (locals/args) */
/* dsmethod - Parser/Interpreter interface - control method parsing */
/* dsinit */
/* dsobject - Parser/Interpreter interface - object initialization and conversion */
/* dspkginit - Package object initialization */
/* dsutils - Parser/Interpreter interface utility routines */
/* dswscope - Scope Stack manipulation */
/* dswstate - parser WALK_STATE management routines */
/* dsdebug - parser debugging routines */

extern "C" {
    pub fn acpi_ds_get_buffer_field_arguments(obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_get_bank_field_arguments(obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_get_region_arguments(rgn_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_get_buffer_arguments(obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_get_package_arguments(obj_desc: *mut acpi_operand_object) -> acpi_status;

    pub fn acpi_ds_exec_begin_control_op(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_exec_end_control_op(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;

    pub fn acpi_ds_eval_buffer_field_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_eval_region_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_eval_table_region_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_eval_data_object_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_eval_bank_field_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_initialize_region(obj_handle: acpi_handle) -> acpi_status;

    pub fn acpi_ds_get_predicate_value(walk_state: *mut acpi_walk_state, result_obj: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_exec_begin_op(walk_state: *mut acpi_walk_state, out_op: *mut *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_exec_end_op(state: *mut acpi_walk_state) -> acpi_status;

    pub fn acpi_ds_create_field(op: *mut acpi_parse_object, region_node: *mut acpi_namespace_node, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_create_bank_field(op: *mut acpi_parse_object, region_node: *mut acpi_namespace_node, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_create_index_field(op: *mut acpi_parse_object, region_node: *mut acpi_namespace_node, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_create_buffer_field(op: *mut acpi_parse_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_init_field_objects(op: *mut acpi_parse_object, walk_state: *mut acpi_walk_state) -> acpi_status;

    pub fn acpi_ds_init_callbacks(walk_state: *mut acpi_walk_state, pass_number: u32) -> acpi_status;
    pub fn acpi_ds_load1_begin_op(walk_state: *mut acpi_walk_state, out_op: *mut *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_load1_end_op(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_load2_begin_op(walk_state: *mut acpi_walk_state, out_op: *mut *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_load2_end_op(walk_state: *mut acpi_walk_state) -> acpi_status;

    pub fn acpi_ds_store_object_to_local(type_: u8, index: u32, src_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_method_data_get_entry(opcode: u16, index: u32, walk_state: *mut acpi_walk_state, node: *mut *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_method_data_delete_all(walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_is_method_value(obj_desc: *mut acpi_operand_object) -> u8;
    pub fn acpi_ds_method_data_get_value(type_: u8, index: u32, walk_state: *mut acpi_walk_state, dest_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_method_data_init_args(params: *mut *mut acpi_operand_object, max_param_count: u32, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_method_data_get_node(type_: u8, index: u32, walk_state: *mut acpi_walk_state, node: *mut *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ds_method_data_init(walk_state: *mut acpi_walk_state);

    pub fn acpi_ds_auto_serialize_method(node: *mut acpi_namespace_node, obj_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_call_control_method(thread: *mut acpi_thread_state, walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_restart_control_method(walk_state: *mut acpi_walk_state, return_desc: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_terminate_control_method(method_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_begin_method_execution(method_node: *mut acpi_namespace_node, obj_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_method_error(status: acpi_status, walk_state: *mut acpi_walk_state) -> acpi_status;

    pub fn acpi_ds_initialize_objects(table_index: u32, start_node: *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ds_build_internal_object(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, obj_desc_ptr: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_build_internal_buffer_obj(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, buffer_length: u32, obj_desc_ptr: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_build_internal_package_obj(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, package_length: u32, obj_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_init_object_from_op(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, opcode: u16, obj_desc: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ds_create_node(walk_state: *mut acpi_walk_state, node: *mut acpi_namespace_node, op: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_init_package_element(object_type: u8, source_object: *mut acpi_operand_object, state: *mut acpi_generic_state, context: *mut core::ffi::c_void) -> acpi_status;

    pub fn acpi_ds_clear_implicit_return(walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_do_implicit_return(return_desc: *mut acpi_operand_object, walk_state: *mut acpi_walk_state, add_reference: u8) -> u8;
    pub fn acpi_ds_is_result_used(op: *mut acpi_parse_object, walk_state: *mut acpi_walk_state) -> u8;
    pub fn acpi_ds_delete_result_if_not_used(op: *mut acpi_parse_object, result_obj: *mut acpi_operand_object, walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_create_operand(walk_state: *mut acpi_walk_state, arg: *mut acpi_parse_object, args_remaining: u32) -> acpi_status;
    pub fn acpi_ds_create_operands(walk_state: *mut acpi_walk_state, first_arg: *mut acpi_parse_object) -> acpi_status;
    pub fn acpi_ds_resolve_operands(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_clear_operands(walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_evaluate_name_path(walk_state: *mut acpi_walk_state) -> acpi_status;

    pub fn acpi_ds_scope_stack_push(node: *mut acpi_namespace_node, type_: acpi_object_type, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_scope_stack_pop(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_scope_stack_clear(walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_obj_stack_push(object: *mut core::ffi::c_void, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_obj_stack_pop(pop_count: u32, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_create_walk_state(owner_id: acpi_owner_id, origin: *mut acpi_parse_object, mth_desc: *mut acpi_operand_object, thread: *mut acpi_thread_state) -> *mut acpi_walk_state;
    pub fn acpi_ds_init_aml_walk(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, method_node: *mut acpi_namespace_node, aml_start: *mut u8, aml_length: u32, info: *mut acpi_evaluate_info, pass_number: u8) -> acpi_status;
    pub fn acpi_ds_obj_stack_pop_and_delete(pop_count: u32, walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_delete_walk_state(walk_state: *mut acpi_walk_state);
    pub fn acpi_ds_pop_walk_state(thread: *mut acpi_thread_state) -> *mut acpi_walk_state;
    pub fn acpi_ds_push_walk_state(walk_state: *mut acpi_walk_state, thread: *mut acpi_thread_state);
    pub fn acpi_ds_result_stack_clear(walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_get_current_walk_state(thread: *mut acpi_thread_state) -> *mut acpi_walk_state;
    pub fn acpi_ds_result_pop(object: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_result_push(object: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    pub fn acpi_ds_dump_method_stack(status: acpi_status, walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
