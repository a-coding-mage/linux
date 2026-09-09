/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Namespace subcomponent prototypes and defines. */

// Dependencies supplied by the surrounding ACPICA translation.

pub const ACPI_NS_ALL: acpi_handle = 0 as acpi_handle;

pub const ACPI_NS_NORMAL: u32 = 0;
pub const ACPI_NS_NEWSCOPE: u32 = 1; // a definition of this type opens a name scope
pub const ACPI_NS_LOCAL: u32 = 2; // suppress search of enclosing scopes

pub const ACPI_NS_NO_UPSEARCH: u32 = 0;
pub const ACPI_NS_SEARCH_PARENT: u32 = 0x0001;
pub const ACPI_NS_DONT_OPEN_SCOPE: u32 = 0x0002;
pub const ACPI_NS_NO_PEER_SEARCH: u32 = 0x0004;
pub const ACPI_NS_ERROR_IF_FOUND: u32 = 0x0008;
pub const ACPI_NS_PREFIX_IS_SCOPE: u32 = 0x0010;
pub const ACPI_NS_EXTERNAL: u32 = 0x0020;
pub const ACPI_NS_TEMPORARY: u32 = 0x0040;
pub const ACPI_NS_OVERRIDE_IF_FOUND: u32 = 0x0080;
pub const ACPI_NS_EARLY_INIT: u32 = 0x0100;
pub const ACPI_NS_PREFIX_MUST_EXIST: u32 = 0x0200;

pub const ACPI_NS_WALK_NO_UNLOCK: u32 = 0;
pub const ACPI_NS_WALK_UNLOCK: u32 = 0x01;
pub const ACPI_NS_WALK_TEMP_NODES: u32 = 0x02;

pub const ACPI_NOT_PACKAGE_ELEMENT: u32 = ACPI_UINT32_MAX;
pub const ACPI_ALL_PACKAGE_ELEMENTS: u32 = ACPI_UINT32_MAX - 1;
pub const ACPI_WARN_ALWAYS: u32 = 0;

extern "C" {
    pub fn acpi_ns_initialize_objects() -> acpi_status;
    pub fn acpi_ns_initialize_devices(flags: u32) -> acpi_status;
    pub fn acpi_ns_init_one_package(obj_handle: acpi_handle, level: u32, context: *mut core::ffi::c_void, return_value: *mut *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_ns_load_namespace() -> acpi_status;
    pub fn acpi_ns_load_table(table_index: u32, node: *mut acpi_namespace_node) -> acpi_status;

    pub fn acpi_ns_walk_namespace(type_: acpi_object_type, start_object: acpi_handle, max_depth: u32, flags: u32, descending_callback: acpi_walk_callback, ascending_callback: acpi_walk_callback, context: *mut core::ffi::c_void, return_value: *mut *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_ns_get_next_node(parent: *mut acpi_namespace_node, child: *mut acpi_namespace_node) -> *mut acpi_namespace_node;
    pub fn acpi_ns_get_next_node_typed(type_: acpi_object_type, parent: *mut acpi_namespace_node, child: *mut acpi_namespace_node) -> *mut acpi_namespace_node;

    pub fn acpi_ns_parse_table(table_index: u32, start_node: *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ns_execute_table(table_index: u32, start_node: *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ns_one_complete_parse(pass_number: u32, table_index: u32, start_node: *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ns_root_initialize() -> acpi_status;
    pub fn acpi_ns_lookup(scope_info: *mut acpi_generic_state, name: *mut i8, type_: acpi_object_type, interpreter_mode: acpi_interpreter_mode, flags: u32, walk_state: *mut acpi_walk_state, ret_node: *mut *mut acpi_namespace_node) -> acpi_status;

    pub fn acpi_ns_create_node(name: u32) -> *mut acpi_namespace_node;
    pub fn acpi_ns_delete_node(node: *mut acpi_namespace_node);
    pub fn acpi_ns_remove_node(node: *mut acpi_namespace_node);
    pub fn acpi_ns_delete_namespace_subtree(parent_handle: *mut acpi_namespace_node);
    pub fn acpi_ns_delete_namespace_by_owner(owner_id: acpi_owner_id);
    pub fn acpi_ns_detach_object(node: *mut acpi_namespace_node);
    pub fn acpi_ns_delete_children(parent: *mut acpi_namespace_node);
    pub fn acpi_ns_compare_names(name1: *mut i8, name2: *mut i8) -> i32;

    pub fn acpi_ns_convert_to_integer(original_object: *mut acpi_operand_object, return_object: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_convert_to_string(original_object: *mut acpi_operand_object, return_object: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_convert_to_buffer(original_object: *mut acpi_operand_object, return_object: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_convert_to_unicode(scope: *mut acpi_namespace_node, original_object: *mut acpi_operand_object, return_object: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_convert_to_resource(scope: *mut acpi_namespace_node, original_object: *mut acpi_operand_object, return_object: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_convert_to_reference(scope: *mut acpi_namespace_node, original_object: *mut acpi_operand_object, return_object: *mut *mut acpi_operand_object) -> acpi_status;

    pub fn acpi_ns_dump_tables(search_base: acpi_handle, max_depth: u32);
    pub fn acpi_ns_dump_entry(handle: acpi_handle, debug_level: u32);
    pub fn acpi_ns_dump_pathname(handle: acpi_handle, msg: *const i8, level: u32, component: u32);
    pub fn acpi_ns_print_pathname(num_segments: u32, pathname: *const i8);
    pub fn acpi_ns_dump_one_object(obj_handle: acpi_handle, level: u32, context: *mut core::ffi::c_void, return_value: *mut *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_ns_dump_objects(type_: acpi_object_type, display_type: u8, max_depth: u32, owner_id: acpi_owner_id, start_handle: acpi_handle);
    pub fn acpi_ns_dump_object_paths(type_: acpi_object_type, display_type: u8, max_depth: u32, owner_id: acpi_owner_id, start_handle: acpi_handle);
    pub fn acpi_ns_evaluate(info: *mut acpi_evaluate_info) -> acpi_status;

    pub fn acpi_ns_check_argument_count(pathname: *mut i8, node: *mut acpi_namespace_node, user_param_count: u32, info: *const acpi_predefined_info);
    pub fn acpi_ns_check_acpi_compliance(pathname: *mut i8, node: *mut acpi_namespace_node, predefined: *const acpi_predefined_info);
    pub fn acpi_ns_check_argument_types(info: *mut acpi_evaluate_info);
    pub fn acpi_ns_check_return_value(node: *mut acpi_namespace_node, info: *mut acpi_evaluate_info, user_param_count: u32, return_status: acpi_status, return_object: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_check_object_type(info: *mut acpi_evaluate_info, return_object_ptr: *mut *mut acpi_operand_object, expected_btypes: u32, package_index: u32) -> acpi_status;
    pub fn acpi_ns_check_package(info: *mut acpi_evaluate_info, return_object_ptr: *mut *mut acpi_operand_object) -> acpi_status;

    pub fn acpi_ns_opens_scope(type_: acpi_object_type) -> u32;
    pub fn acpi_ns_get_external_pathname(node: *mut acpi_namespace_node) -> *mut i8;
    pub fn acpi_ns_build_normalized_path(node: *mut acpi_namespace_node, full_path: *mut i8, path_size: u32, no_trailing: u8) -> u32;
    pub fn acpi_ns_normalize_pathname(original_path: *mut i8);
    pub fn acpi_ns_get_normalized_pathname(node: *mut acpi_namespace_node, no_trailing: u8) -> *mut i8;
    pub fn acpi_ns_build_prefixed_pathname(prefix_scope: *mut acpi_generic_state, internal_path: *const i8) -> *mut i8;
    pub fn acpi_ns_name_of_current_scope(walk_state: *mut acpi_walk_state) -> *mut i8;
    pub fn acpi_ns_handle_to_name(target_handle: acpi_handle, buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_ns_handle_to_pathname(target_handle: acpi_handle, buffer: *mut acpi_buffer, no_trailing: u8) -> acpi_status;
    pub fn acpi_ns_pattern_match(obj_node: *mut acpi_namespace_node, search_for: *mut i8) -> u8;
    pub fn acpi_ns_get_node_unlocked(prefix_node: *mut acpi_namespace_node, external_pathname: *const i8, flags: u32, out_node: *mut *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ns_get_node(prefix_node: *mut acpi_namespace_node, external_pathname: *const i8, flags: u32, out_node: *mut *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ns_get_pathname_length(node: *mut acpi_namespace_node) -> acpi_size;

    pub fn acpi_ns_attach_object(node: *mut acpi_namespace_node, object: *mut acpi_operand_object, type_: acpi_object_type) -> acpi_status;
    pub fn acpi_ns_get_attached_object(node: *mut acpi_namespace_node) -> *mut acpi_operand_object;
    pub fn acpi_ns_get_secondary_object(obj_desc: *mut acpi_operand_object) -> *mut acpi_operand_object;
    pub fn acpi_ns_attach_data(node: *mut acpi_namespace_node, handler: acpi_object_handler, data: *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_ns_detach_data(node: *mut acpi_namespace_node, handler: acpi_object_handler) -> acpi_status;
    pub fn acpi_ns_get_attached_data(node: *mut acpi_namespace_node, handler: acpi_object_handler, data: *mut *mut core::ffi::c_void) -> acpi_status;

    pub fn acpi_ns_simple_repair(info: *mut acpi_evaluate_info, expected_btypes: u32, package_index: u32, return_object_ptr: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_wrap_with_package(info: *mut acpi_evaluate_info, original_object: *mut acpi_operand_object, obj_desc_ptr: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_repair_null_element(info: *mut acpi_evaluate_info, expected_btypes: u32, package_index: u32, return_object_ptr: *mut *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ns_remove_null_elements(info: *mut acpi_evaluate_info, package_type: u8, obj_desc: *mut acpi_operand_object);
    pub fn acpi_ns_complex_repairs(info: *mut acpi_evaluate_info, node: *mut acpi_namespace_node, validate_status: acpi_status, return_object_ptr: *mut *mut acpi_operand_object) -> acpi_status;

    pub fn acpi_ns_search_and_enter(entry_name: u32, walk_state: *mut acpi_walk_state, node: *mut acpi_namespace_node, interpreter_mode: acpi_interpreter_mode, type_: acpi_object_type, flags: u32, ret_node: *mut *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ns_search_one_scope(entry_name: u32, node: *mut acpi_namespace_node, type_: acpi_object_type, ret_node: *mut *mut acpi_namespace_node) -> acpi_status;
    pub fn acpi_ns_install_node(walk_state: *mut acpi_walk_state, parent_node: *mut acpi_namespace_node, node: *mut acpi_namespace_node, type_: acpi_object_type);
    pub fn acpi_ns_get_type(node: *mut acpi_namespace_node) -> acpi_object_type;
    pub fn acpi_ns_local(type_: acpi_object_type) -> u32;
    pub fn acpi_ns_print_node_pathname(node: *mut acpi_namespace_node, msg: *const i8);
    pub fn acpi_ns_build_internal_name(info: *mut acpi_namestring_info) -> acpi_status;
    pub fn acpi_ns_get_internal_name_length(info: *mut acpi_namestring_info);
    pub fn acpi_ns_internalize_name(dotted_name: *const i8, converted_name: *mut *mut i8) -> acpi_status;
    pub fn acpi_ns_externalize_name(internal_name_length: u32, internal_name: *const i8, converted_name_length: *mut u32, converted_name: *mut *mut i8) -> acpi_status;
    pub fn acpi_ns_validate_handle(handle: acpi_handle) -> *mut acpi_namespace_node;
    pub fn acpi_ns_terminate();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
