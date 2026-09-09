// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Dispatcher parse tree walk management routines.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

extern "C" {
    fn acpi_ut_get_object_type_name(object: *mut acpi_operand_object) -> *const u8;
    fn acpi_ut_create_generic_state() -> *mut acpi_generic_state;
    fn acpi_ut_push_generic_state(list: *mut *mut acpi_generic_state, state: *mut acpi_generic_state);
    fn acpi_ut_pop_generic_state(list: *mut *mut acpi_generic_state) -> *mut acpi_generic_state;
    fn acpi_ut_delete_generic_state(state: *mut acpi_generic_state);
    fn acpi_ut_remove_reference(object: *mut acpi_operand_object);
    fn acpi_ps_init_scope(parser_state: *mut acpi_parse_state, op: *mut acpi_parse_object) -> acpi_status;
    fn acpi_ps_cleanup_scope(parser_state: *mut acpi_parse_state);
    fn acpi_ds_method_data_init(walk_state: *mut acpi_walk_state);
    fn acpi_ds_method_data_init_args(params: *mut *mut acpi_operand_object, count: u32, walk_state: *mut acpi_walk_state) -> acpi_status;
    fn acpi_ds_scope_stack_push(node: *mut acpi_namespace_node, ty: u32, walk_state: *mut acpi_walk_state) -> acpi_status;
    fn acpi_ns_get_attached_object(node: *mut acpi_namespace_node) -> *mut acpi_operand_object;
    fn acpi_ds_init_callbacks(walk_state: *mut acpi_walk_state, pass_number: u8) -> acpi_status;
}

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type s32 = core::ffi::c_int;
pub type acpi_owner_id = u16;
pub type acpi_status = u32;
pub type u16 = core::ffi::c_ushort;

const AE_OK: acpi_status = 0;
const AE_AML_INTERNAL: acpi_status = 0x1001;
const AE_AML_NO_RETURN_VALUE: acpi_status = 0x1002;
const AE_BAD_PARAMETER: acpi_status = 0x0005;
const AE_STACK_OVERFLOW: acpi_status = 0x0009;
const AE_AML_NO_OPERAND: acpi_status = 0x1003;
const AE_STACK_UNDERFLOW: acpi_status = 0x000a;
const AE_NO_MEMORY: acpi_status = 0x000c;
const ACPI_DESC_TYPE_STATE_RESULT: u8 = 0x08;
const ACPI_DESC_TYPE_WALK: u8 = 0x0e;
const ACPI_RESULTS_FRAME_OBJ_NUM: u32 = 8;
const ACPI_RESULTS_OBJ_NUM_MAX: u32 = 128;
const ACPI_OBJ_NUM_OPERANDS: u32 = 7;
const ACPI_METHOD_NUM_ARGS: u32 = 7;
const ACPI_WALK_METHOD: u32 = 1;
const ACPI_TYPE_METHOD: u32 = 8;

#[repr(C)] pub struct acpi_operand_object { _private: [u8; 0] }
#[repr(C)] pub struct acpi_parse_object { pub common: acpi_parse_object_common }
#[repr(C)] pub struct acpi_parse_object_common { pub node: *mut acpi_namespace_node, pub parent: *mut acpi_parse_object, pub _rest: [u8; 0] }
#[repr(C)] pub struct acpi_namespace_node { pub type_: u32 }
#[repr(C)] pub struct acpi_evaluate_info { pub parameters: *mut *mut acpi_operand_object, pub return_object: *mut *mut acpi_operand_object }
#[repr(C)] pub struct acpi_parse_state { pub aml: *mut u8, pub aml_start: *mut u8, pub aml_end: *mut u8, pub pkg_end: *mut u8, pub start_op: *mut acpi_parse_object, pub start_node: *mut acpi_namespace_node, pub scope: *mut c_void }
#[repr(C)] pub union acpi_result_union { pub obj_desc: [*mut acpi_operand_object; 8] }
#[repr(C)] pub struct acpi_generic_state_common { pub descriptor_type: u8, pub next: *mut acpi_generic_state }
#[repr(C)] pub struct acpi_generic_state { pub common: acpi_generic_state_common, pub results: acpi_result_union }
#[repr(C)] pub struct acpi_walk_state { pub descriptor_type: u8, pub method_desc: *mut acpi_operand_object, pub owner_id: acpi_owner_id, pub origin: *mut acpi_parse_object, pub thread: *mut acpi_thread_state, pub parser_state: acpi_parse_state, pub next: *mut acpi_walk_state, pub results: *mut acpi_generic_state, pub result_count: u32, pub result_size: u32, pub operands: [*mut c_void; 7], pub operand_index: u32, pub num_operands: u32, pub current_result: u32, pub params: *mut *mut acpi_operand_object, pub caller_return_desc: *mut *mut acpi_operand_object, pub next_op: *mut acpi_parse_object, pub pass_number: u8, pub walk_type: u32, pub method_node: *mut acpi_namespace_node, pub control_state: *mut acpi_generic_state, pub scope_info: *mut acpi_generic_state }
#[repr(C)] pub struct acpi_thread_state { pub walk_state_list: *mut acpi_walk_state }

#[inline] unsafe fn failure(s: acpi_status) -> bool { s != AE_OK }

pub unsafe fn acpi_ds_result_pop(object: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status {
    let state = (*walk_state).results;
    if !state.is_null() && (*walk_state).result_count == 0 { return AE_AML_INTERNAL; }
    if state.is_null() && (*walk_state).result_count != 0 { return AE_AML_INTERNAL; }
    if state.is_null() { return AE_AML_NO_RETURN_VALUE; }
    (*walk_state).result_count -= 1;
    let index = (*walk_state).result_count % ACPI_RESULTS_FRAME_OBJ_NUM;
    *object = (*state).results.obj_desc[index as usize];
    if (*object).is_null() { return AE_AML_NO_RETURN_VALUE; }
    (*state).results.obj_desc[index as usize] = core::ptr::null_mut();
    if index == 0 { let status = acpi_ds_result_stack_pop(walk_state); if failure(status) { return status; } }
    AE_OK
}

pub unsafe fn acpi_ds_result_push(object: *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status {
    if (*walk_state).result_count > (*walk_state).result_size { return AE_AML_INTERNAL; }
    if (*walk_state).result_count == (*walk_state).result_size { let s = acpi_ds_result_stack_push(walk_state); if failure(s) { return s; } }
    if (*walk_state).result_count >= (*walk_state).result_size || object.is_null() { return if object.is_null() { AE_BAD_PARAMETER } else { AE_AML_INTERNAL }; }
    let state = (*walk_state).results; if state.is_null() { return AE_AML_INTERNAL; }
    let index = (*walk_state).result_count % ACPI_RESULTS_FRAME_OBJ_NUM;
    (*state).results.obj_desc[index as usize] = object; (*walk_state).result_count += 1; AE_OK
}

unsafe fn acpi_ds_result_stack_push(walk_state: *mut acpi_walk_state) -> acpi_status {
    if (*walk_state).result_size + ACPI_RESULTS_FRAME_OBJ_NUM > ACPI_RESULTS_OBJ_NUM_MAX { return AE_STACK_OVERFLOW; }
    let state = acpi_ut_create_generic_state(); if state.is_null() { return AE_NO_MEMORY; }
    (*state).common.descriptor_type = ACPI_DESC_TYPE_STATE_RESULT;
    acpi_ut_push_generic_state(&mut (*walk_state).results, state);
    (*walk_state).result_size += ACPI_RESULTS_FRAME_OBJ_NUM; AE_OK
}

unsafe fn acpi_ds_result_stack_pop(walk_state: *mut acpi_walk_state) -> acpi_status {
    if (*walk_state).results.is_null() { return AE_AML_NO_OPERAND; }
    if (*walk_state).result_size < ACPI_RESULTS_FRAME_OBJ_NUM { return AE_AML_INTERNAL; }
    let state = acpi_ut_pop_generic_state(&mut (*walk_state).results); acpi_ut_delete_generic_state(state);
    (*walk_state).result_size -= ACPI_RESULTS_FRAME_OBJ_NUM; AE_OK
}

pub unsafe fn acpi_ds_obj_stack_push(object: *mut c_void, walk_state: *mut acpi_walk_state) -> acpi_status {
    if (*walk_state).num_operands >= ACPI_OBJ_NUM_OPERANDS { return AE_STACK_OVERFLOW; }
    (*walk_state).operands[(*walk_state).operand_index as usize] = object; (*walk_state).num_operands += 1; (*walk_state).operand_index += 1; AE_OK
}

pub unsafe fn acpi_ds_obj_stack_pop(pop_count: u32, walk_state: *mut acpi_walk_state) -> acpi_status {
    for _ in 0..pop_count { if (*walk_state).num_operands == 0 { return AE_STACK_UNDERFLOW; } (*walk_state).num_operands -= 1; (*walk_state).operands[(*walk_state).num_operands as usize] = core::ptr::null_mut(); } AE_OK
}

pub unsafe fn acpi_ds_obj_stack_pop_and_delete(pop_count: u32, walk_state: *mut acpi_walk_state) {
    if pop_count == 0 { return; }
    let mut i = pop_count as s32 - 1;
    while i >= 0 { if (*walk_state).num_operands == 0 { return; } (*walk_state).num_operands -= 1; let obj = (*walk_state).operands[i as usize] as *mut acpi_operand_object; if !obj.is_null() { acpi_ut_remove_reference(obj); (*walk_state).operands[i as usize] = core::ptr::null_mut(); } i -= 1; }
}

pub unsafe fn acpi_ds_get_current_walk_state(thread: *mut acpi_thread_state) -> *mut acpi_walk_state { if thread.is_null() { core::ptr::null_mut() } else { (*thread).walk_state_list } }
pub unsafe fn acpi_ds_push_walk_state(walk_state: *mut acpi_walk_state, thread: *mut acpi_thread_state) { (*walk_state).next = (*thread).walk_state_list; (*thread).walk_state_list = walk_state; }
pub unsafe fn acpi_ds_pop_walk_state(thread: *mut acpi_thread_state) -> *mut acpi_walk_state { let w = (*thread).walk_state_list; if !w.is_null() { (*thread).walk_state_list = (*w).next; } w }

pub unsafe fn acpi_ds_create_walk_state(owner_id: acpi_owner_id, origin: *mut acpi_parse_object, method_desc: *mut acpi_operand_object, thread: *mut acpi_thread_state) -> *mut acpi_walk_state {
    let walk_state = acpi_alloc_zeroed(core::mem::size_of::<acpi_walk_state>()); if walk_state.is_null() { return core::ptr::null_mut(); }
    (*walk_state).descriptor_type = ACPI_DESC_TYPE_WALK; (*walk_state).method_desc = method_desc; (*walk_state).owner_id = owner_id; (*walk_state).origin = origin; (*walk_state).thread = thread; (*walk_state).parser_state.start_op = origin;
    #[cfg(not(feature = "acpi_constant_eval_only"))] acpi_ds_method_data_init(walk_state);
    if !thread.is_null() { acpi_ds_push_walk_state(walk_state, thread); } walk_state
}

extern "C" { fn acpi_alloc_zeroed(size: usize) -> *mut acpi_walk_state; fn acpi_free(ptr: *mut acpi_walk_state); }

pub unsafe fn acpi_ds_init_aml_walk(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, method_node: *mut acpi_namespace_node, aml_start: *mut u8, aml_length: u32, info: *mut acpi_evaluate_info, pass_number: u8) -> acpi_status {
    (*walk_state).parser_state.aml = aml_start; (*walk_state).parser_state.aml_start = aml_start; (*walk_state).parser_state.aml_end = aml_start; (*walk_state).parser_state.pkg_end = aml_start;
    if aml_length != 0 { (*walk_state).parser_state.aml_end = aml_start.add(aml_length as usize); (*walk_state).parser_state.pkg_end = aml_start.add(aml_length as usize); }
    (*walk_state).next_op = core::ptr::null_mut(); (*walk_state).pass_number = pass_number;
    if !info.is_null() { (*walk_state).params = (*info).parameters; (*walk_state).caller_return_desc = &mut (*info).return_object; }
    let mut status = acpi_ps_init_scope(&mut (*walk_state).parser_state, op); if failure(status) { return status; }
    if !method_node.is_null() { (*walk_state).parser_state.start_node = method_node; (*walk_state).walk_type = ACPI_WALK_METHOD; (*walk_state).method_node = method_node; (*walk_state).method_desc = acpi_ns_get_attached_object(method_node); status = acpi_ds_scope_stack_push(method_node, ACPI_TYPE_METHOD, walk_state); if failure(status) { return status; } status = acpi_ds_method_data_init_args((*walk_state).params, ACPI_METHOD_NUM_ARGS, walk_state); if failure(status) { return status; } }
    else { let mut extra_op = (*walk_state).parser_state.start_op; while !extra_op.is_null() && (*extra_op).common.node.is_null() { extra_op = (*extra_op).common.parent; } (*walk_state).parser_state.start_node = if extra_op.is_null() { core::ptr::null_mut() } else { (*extra_op).common.node }; if !(*walk_state).parser_state.start_node.is_null() { let n = (*walk_state).parser_state.start_node; status = acpi_ds_scope_stack_push(n, (*n).type_, walk_state); if failure(status) { return status; } } }
    acpi_ds_init_callbacks(walk_state, pass_number)
}

pub unsafe fn acpi_ds_delete_walk_state(walk_state: *mut acpi_walk_state) {
    if walk_state.is_null() { return; }
    if !(*walk_state).parser_state.scope.is_null() { acpi_ps_cleanup_scope(&mut (*walk_state).parser_state); }
    while !(*walk_state).control_state.is_null() { let state = (*walk_state).control_state; (*walk_state).control_state = (*state).common.next; acpi_ut_delete_generic_state(state); }
    while !(*walk_state).scope_info.is_null() { let state = (*walk_state).scope_info; (*walk_state).scope_info = (*state).common.next; acpi_ut_delete_generic_state(state); }
    while !(*walk_state).results.is_null() { let state = (*walk_state).results; (*walk_state).results = (*state).common.next; acpi_ut_delete_generic_state(state); }
    acpi_free(walk_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
