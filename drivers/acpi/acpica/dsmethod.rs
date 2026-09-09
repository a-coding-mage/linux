// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Parser/Interpreter interface - control method parsing
//
// This file is a direct low-level translation of dsmethod.c. Types, constants,
// globals, and external routines are supplied by the ACPICA Rust bindings.

// C includes intentionally omitted; their symbols are external dependencies.

extern "C" {
    static mut acpi_gbl_exception_handler: Option<unsafe extern "C" fn(acpi_status, acpi_name, u32, u32, *mut core::ffi::c_void) -> acpi_status>;
    static mut acpi_method_count: u32;
    static mut acpi_ds_do_implicit_return: bool;
}

type acpi_status = u32;
type acpi_name = u32;
type u32_t = u32;

extern "C" {
    fn acpi_ps_alloc_op(opcode: u16, aml: *mut u8) -> *mut acpi_parse_object;
    fn acpi_ps_set_name(op: *mut acpi_parse_object, name: u32);
    fn acpi_ps_free_op(op: *mut acpi_parse_object);
    fn acpi_ps_delete_parse_tree(op: *mut acpi_parse_object);
    fn acpi_ds_create_walk_state(owner: u32, a: *mut core::ffi::c_void, b: *mut acpi_operand_object, c: *mut acpi_thread_state) -> *mut acpi_walk_state;
    fn acpi_ds_delete_walk_state(walk: *mut acpi_walk_state);
    fn acpi_ds_init_aml_walk(walk: *mut acpi_walk_state, op: *mut acpi_parse_object, node: *mut acpi_namespace_node, aml: *mut u8, len: u32, info: *mut acpi_evaluate_info, mode: u32) -> acpi_status;
    fn acpi_ps_parse_aml(walk: *mut acpi_walk_state) -> acpi_status;
    fn acpi_ds_clear_implicit_return(walk: *mut acpi_walk_state);
    fn acpi_ds_dump_method_stack(status: acpi_status, walk: *mut acpi_walk_state, op: *mut acpi_parse_object);
    fn acpi_ex_exit_interpreter(); fn acpi_ex_enter_interpreter();
    fn acpi_ds_method_data_delete_all(walk: *mut acpi_walk_state);
    fn acpi_ns_delete_namespace_subtree(node: *mut acpi_namespace_node);
    fn acpi_ns_delete_namespace_by_owner(owner: u32);
    fn acpi_ut_release_owner_id(owner: *mut u32);
    fn acpi_ut_allocate_owner_id(owner: *mut u32) -> acpi_status;
    fn acpi_ut_remove_reference(obj: *mut acpi_operand_object);
    fn acpi_ds_clear_operands(walk: *mut acpi_walk_state);
    fn acpi_ds_pop_walk_state(thread: *mut acpi_thread_state);
    fn acpi_ds_terminate_control_method(obj: *mut acpi_operand_object, walk: *mut acpi_walk_state);
    fn acpi_ds_result_push(obj: *mut acpi_operand_object, walk: *mut acpi_walk_state) -> acpi_status;
    fn acpi_ds_do_implicit_return(obj: *mut acpi_operand_object, walk: *mut acpi_walk_state, add: bool) -> bool;
    fn acpi_ex_start_trace_method(node: *mut acpi_namespace_node, obj: *mut acpi_operand_object, walk: *mut acpi_walk_state);
    fn acpi_ex_stop_trace_method(node: *mut acpi_namespace_node, obj: *mut acpi_operand_object, walk: *mut acpi_walk_state);
    fn acpi_ns_get_attached_object(node: *mut acpi_namespace_node) -> *mut acpi_operand_object;
    fn acpi_os_create_mutex(mutex: *mut core::ffi::c_void) -> acpi_status;
    fn acpi_os_release_mutex(mutex: *mut core::ffi::c_void);
    fn acpi_os_get_thread_id() -> u64;
    fn acpi_ex_system_wait_mutex(mutex: *mut core::ffi::c_void, timeout: u32) -> acpi_status;
    fn acpi_ut_create_internal_object(ty: u32) -> *mut acpi_operand_object;
    fn acpi_ut_delete_object_desc(obj: *mut acpi_operand_object);
    fn acpi_ns_get_normalized_pathname(node: *mut acpi_namespace_node, no_capture: bool) -> *mut u8;
    fn acpi_ns_get_node_name(node: *mut acpi_namespace_node) -> *const u8;
}

#[repr(C)] pub struct acpi_namespace_node { pub name: u32, pub owner_id: u32 }
#[repr(C)] pub struct acpi_parse_object { pub node: *mut acpi_namespace_node }
#[repr(C)] pub struct acpi_thread_state { pub thread_id: u64, pub current_sync_level: u8 }
#[repr(C)] pub struct acpi_evaluate_info { pub parameters: *mut *mut acpi_operand_object }
#[repr(C)] pub struct acpi_operand_object { pub method: acpi_method, pub common: acpi_common }
#[repr(C)] pub struct acpi_common { pub ty: u8 }
#[repr(C)] pub struct acpi_method { pub aml_start: *mut u8, pub aml_length: u32, pub owner_id: u32, pub thread_count: u8, pub info_flags: u32, pub sync_level: u8, pub mutex: *mut acpi_operand_object, pub param_count: u8, pub node: *mut acpi_namespace_node, pub dispatch: *mut core::ffi::c_void }
#[repr(C)] pub struct acpi_walk_state { pub op: *mut acpi_parse_object, pub prev_op: *mut acpi_parse_object, pub method_node: *mut acpi_namespace_node, pub deferred_node: *mut acpi_namespace_node, pub method_call_node: *mut acpi_namespace_node, pub method_desc: *mut acpi_operand_object, pub thread: *mut acpi_thread_state, pub operands: [*mut acpi_operand_object; 32], pub num_operands: u32, pub method_nesting_depth: u32, pub return_used: bool, pub return_desc: *mut acpi_operand_object, pub implicit_return_obj: *mut acpi_operand_object, pub method_pathname: *mut u8, pub method_is_nested: bool, pub local_variables: [acpi_namespace_node; 8], pub arguments: [acpi_namespace_node; 7] }

const AE_OK: acpi_status = 0; const AE_NO_MEMORY: acpi_status = 1; const AE_NULL_ENTRY: acpi_status = 2; const AE_NULL_OBJECT: acpi_status = 3; const AE_CTRL_TERMINATE: acpi_status = 4;
const ACPI_METHOD_SERIALIZED: u32 = 1; const ACPI_METHOD_IGNORE_SYNC_LEVEL: u32 = 2; const ACPI_METHOD_MODULE_LEVEL: u32 = 4; const ACPI_METHOD_MODIFIED_NAMESPACE: u32 = 8; const ACPI_METHOD_SERIALIZED_PENDING: u32 = 16;
const ACPI_TYPE_MUTEX: u32 = 0x0A; const ACPI_WAIT_FOREVER: u32 = 0xffff_ffff; const AML_METHOD_OP: u16 = 0x14; const ACPI_UINT8_MAX: u8 = 255;

unsafe extern "C" fn acpi_ds_detect_named_opcodes(walk_state: *mut acpi_walk_state, _out_op: *mut *mut acpi_parse_object) -> acpi_status {
    // The parser callback marks a method serialized when it encounters a named,
    // create, or field opcode, then terminates the scan.
    if (*walk_state).method_desc.is_null() { return AE_OK; }
    (*(*walk_state).method_desc).method.sync_level = 0;
    (*(*walk_state).method_desc).method.info_flags |= ACPI_METHOD_SERIALIZED | ACPI_METHOD_IGNORE_SYNC_LEVEL;
    AE_CTRL_TERMINATE
}

pub unsafe extern "C" fn acpi_ds_auto_serialize_method(node: *mut acpi_namespace_node, obj_desc: *mut acpi_operand_object) -> acpi_status {
    let op = acpi_ps_alloc_op(AML_METHOD_OP, (*obj_desc).method.aml_start);
    if op.is_null() { return AE_NO_MEMORY; }
    acpi_ps_set_name(op, (*node).name); (*op).node = node;
    let walk = acpi_ds_create_walk_state((*node).owner_id, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if walk.is_null() { acpi_ps_free_op(op); return AE_NO_MEMORY; }
    let status = acpi_ds_init_aml_walk(walk, op, node, (*obj_desc).method.aml_start, (*obj_desc).method.aml_length, core::ptr::null_mut(), 0);
    if status != AE_OK { acpi_ds_delete_walk_state(walk); acpi_ps_free_op(op); return status; }
    let status = acpi_ps_parse_aml(walk); acpi_ps_delete_parse_tree(op); status
}

pub unsafe extern "C" fn acpi_ds_method_error(status: acpi_status, walk_state: *mut acpi_walk_state) -> acpi_status {
    if status == AE_OK { return status; }
    if acpi_gbl_exception_handler.is_some() {
        acpi_ex_exit_interpreter();
        let name = if !(*walk_state).method_node.is_null() { (*(*walk_state).method_node).name } else { 0 };
        let handler = acpi_gbl_exception_handler.unwrap();
        let new_status = handler(status, name, 0, 0, core::ptr::null_mut()); acpi_ex_enter_interpreter();
        acpi_ds_clear_implicit_return(walk_state); return new_status;
    }
    acpi_ds_clear_implicit_return(walk_state); acpi_ds_dump_method_stack(status, walk_state, (*walk_state).op); status
}

unsafe fn acpi_ds_create_method_mutex(method_desc: *mut acpi_operand_object) -> acpi_status {
    let mutex = acpi_ut_create_internal_object(ACPI_TYPE_MUTEX); if mutex.is_null() { return AE_NO_MEMORY; }
    let status = acpi_os_create_mutex(core::ptr::null_mut()); if status != AE_OK { acpi_ut_delete_object_desc(mutex); return status; }
    (*method_desc).method.mutex = mutex; AE_OK
}

pub unsafe extern "C" fn acpi_ds_begin_method_execution(node: *mut acpi_namespace_node, obj: *mut acpi_operand_object, walk: *mut acpi_walk_state) -> acpi_status {
    if node.is_null() { return AE_NULL_ENTRY; }
    acpi_ex_start_trace_method(node, obj, walk);
    if (*obj).method.thread_count == ACPI_UINT8_MAX { return 0x1001; }
    if (*obj).method.info_flags & ACPI_METHOD_SERIALIZED != 0 && (*obj).method.mutex.is_null() {
        let status = acpi_ds_create_method_mutex(obj); if status != AE_OK { return status; }
    }
    if (*obj).method.owner_id == 0 { let status = acpi_ut_allocate_owner_id(&mut (*obj).method.owner_id); if status != AE_OK { return status; } }
    (*obj).method.thread_count = (*obj).method.thread_count.wrapping_add(1); acpi_method_count = acpi_method_count.wrapping_add(1); AE_OK
}

pub unsafe extern "C" fn acpi_ds_call_control_method(thread: *mut acpi_thread_state, walk: *mut acpi_walk_state, _op: *mut acpi_parse_object) -> acpi_status {
    let node = (*walk).method_call_node; if node.is_null() { return AE_NULL_ENTRY; }
    let obj = acpi_ns_get_attached_object(node); if obj.is_null() { return AE_NULL_OBJECT; }
    let status = acpi_ds_begin_method_execution(node, obj, walk); if status != AE_OK { return status; }
    let next = acpi_ds_create_walk_state((*obj).method.owner_id, core::ptr::null_mut(), obj, thread);
    if next.is_null() { acpi_ds_terminate_control_method(obj, core::ptr::null_mut()); return AE_NO_MEMORY; }
    let status = acpi_ds_init_aml_walk(next, core::ptr::null_mut(), node, (*obj).method.aml_start, (*obj).method.aml_length, core::ptr::null_mut(), 1);
    if status != AE_OK { acpi_ds_pop_walk_state(thread); acpi_ds_terminate_control_method(obj, next); acpi_ds_delete_walk_state(next); }
    status
}

pub unsafe extern "C" fn acpi_ds_restart_control_method(walk: *mut acpi_walk_state, ret: *mut acpi_operand_object) -> acpi_status {
    if !ret.is_null() { if (*walk).return_used { let status = acpi_ds_result_push(ret, walk); if status != AE_OK { acpi_ut_remove_reference(ret); return status; } (*walk).return_desc = ret; } else { acpi_ut_remove_reference(ret); } } AE_OK
}

pub unsafe extern "C" fn acpi_ds_terminate_control_method(obj: *mut acpi_operand_object, walk: *mut acpi_walk_state) {
    if obj.is_null() { return; }
    if !walk.is_null() { acpi_ds_method_data_delete_all(walk); if (*obj).method.mutex != core::ptr::null_mut() { acpi_os_release_mutex(core::ptr::null_mut()); } }
    if (*obj).method.thread_count != 0 { (*obj).method.thread_count -= 1; }
    if (*obj).method.thread_count == 0 && (*obj).method.info_flags & ACPI_METHOD_MODULE_LEVEL == 0 { acpi_ut_release_owner_id(&mut (*obj).method.owner_id); }
    acpi_ex_stop_trace_method((*obj).method.node, obj, walk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
