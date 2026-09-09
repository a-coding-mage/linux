/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Translation of acpixf.h. Included C headers provide the referenced ACPICA types. */

pub const ACPI_CA_VERSION: u32 = 0x20260408;

/* Build-time C preprocessor conditions are retained as Rust configuration intent. */
#[macro_export]
macro_rules! ACPI_GLOBAL { ($ty:ty, $name:ident) => { extern "C" { pub static mut $name: $ty; } }; }
#[macro_export]
macro_rules! ACPI_INIT_GLOBAL { ($ty:ty, $name:ident, $value:expr) => { pub static mut $name: $ty = $value; }; }

/* Public globals and runtime configuration options. */
ACPI_INIT_GLOBAL!(u8, acpi_gbl_enable_interpreter_slack, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_auto_serialize_methods, TRUE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_create_osi_method, TRUE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_use_default_register_widths, TRUE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_enable_table_validation, TRUE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_enable_aml_debug_object, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_copy_dsdt_locally, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_do_not_use_xsdt, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_use32_bit_fadt_addresses, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_use32_bit_facs_addresses, TRUE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_truncate_io_addresses, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_disable_auto_repair, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_disable_ssdt_table_install, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_runtime_namespace_override, TRUE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_osi_data, 0);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_reduced_hardware, FALSE);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_use_global_lock, TRUE);
ACPI_INIT_GLOBAL!(u32, acpi_gbl_max_loop_iterations, ACPI_MAX_LOOP_TIMEOUT);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_ignore_package_resolution_errors, FALSE);
ACPI_INIT_GLOBAL!(u32, acpi_gbl_trace_flags, 0);
ACPI_INIT_GLOBAL!(*const c_char, acpi_gbl_trace_method_name, core::ptr::null());
ACPI_INIT_GLOBAL!(u32, acpi_gbl_trace_dbg_level, ACPI_TRACE_LEVEL_DEFAULT);
ACPI_INIT_GLOBAL!(u32, acpi_gbl_trace_dbg_layer, ACPI_TRACE_LAYER_DEFAULT);
ACPI_INIT_GLOBAL!(u32, acpi_dbg_level, ACPI_DEBUG_DEFAULT);
ACPI_INIT_GLOBAL!(u32, acpi_dbg_layer, 0);
ACPI_INIT_GLOBAL!(u8, acpi_gbl_display_debug_timer, FALSE);

extern "C" {
    pub static mut acpi_gbl_FADT: acpi_table_fadt;
    pub static mut acpi_current_gpe_count: u32;
    pub static mut acpi_gbl_system_awake_and_running: u8;
}

pub type acpi_walk_resource_callback = unsafe extern "C" fn(*mut acpi_resource, *mut c_void) -> acpi_status;

/* The following declarations are external C interfaces; return-stub macros in the
 * original header are represented by the corresponding declarations here. */
extern "C" {
    pub fn acpi_initialize_tables(initial_storage: *mut acpi_table_desc, initial_table_count: u32, allow_resize: u8) -> acpi_status;
    pub fn acpi_initialize_subsystem() -> acpi_status;
    pub fn acpi_enable_subsystem(flags: u32) -> acpi_status;
    pub fn acpi_initialize_objects(flags: u32) -> acpi_status;
    pub fn acpi_terminate() -> acpi_status;
    pub fn acpi_enable() -> acpi_status;
    pub fn acpi_disable() -> acpi_status;
    pub fn acpi_subsystem_status() -> acpi_status;
    pub fn acpi_get_system_info(ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_get_statistics(stats: *mut acpi_statistics) -> acpi_status;
    pub fn acpi_format_exception(exception: acpi_status) -> *const c_char;
    pub fn acpi_purge_cached_objects() -> acpi_status;
    pub fn acpi_install_interface(interface_name: acpi_string) -> acpi_status;
    pub fn acpi_remove_interface(interface_name: acpi_string) -> acpi_status;
    pub fn acpi_update_interfaces(action: u8) -> acpi_status;
    pub fn acpi_check_address_range(space_id: acpi_adr_space_type, address: acpi_physical_address, length: acpi_size, warn: u8) -> u32;
    pub fn acpi_decode_pld_buffer(in_buffer: *mut u8, length: acpi_size, return_buffer: *mut *mut acpi_pld_info) -> acpi_status;
    pub fn acpi_install_table(table: *mut acpi_table_header) -> acpi_status;
    pub fn acpi_install_physical_table(address: acpi_physical_address) -> acpi_status;
    pub fn acpi_load_table(table: *mut acpi_table_header, table_idx: *mut u32) -> acpi_status;
    pub fn acpi_unload_table(table_index: u32) -> acpi_status;
    pub fn acpi_unload_parent_table(object: acpi_handle) -> acpi_status;
    pub fn acpi_load_tables() -> acpi_status;
    pub fn acpi_reallocate_root_table() -> acpi_status;
    pub fn acpi_find_root_pointer(rsdp_address: *mut acpi_physical_address) -> acpi_status;
    pub fn acpi_get_table_header(signature: acpi_string, instance: u32, out_table_header: *mut acpi_table_header) -> acpi_status;
    pub fn acpi_get_table(signature: acpi_string, instance: u32, out_table: *mut *mut acpi_table_header) -> acpi_status;
    pub fn acpi_put_table(table: *mut acpi_table_header);
    pub fn acpi_get_table_by_index(table_index: u32, out_table: *mut *mut acpi_table_header) -> acpi_status;
    pub fn acpi_install_table_handler(handler: acpi_table_handler, context: *mut c_void) -> acpi_status;
    pub fn acpi_remove_table_handler(handler: acpi_table_handler) -> acpi_status;
    pub fn acpi_walk_namespace(ty: acpi_object_type, start_object: acpi_handle, max_depth: u32, descending_callback: acpi_walk_callback, ascending_callback: acpi_walk_callback, context: *mut c_void, return_value: *mut *mut c_void) -> acpi_status;
    pub fn acpi_get_devices(hid: *const c_char, user_function: acpi_walk_callback, context: *mut c_void, return_value: *mut *mut c_void) -> acpi_status;
    pub fn acpi_get_name(object: acpi_handle, name_type: u32, ret_path_ptr: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_get_handle(parent: acpi_handle, pathname: *const c_char, ret_handle: *mut acpi_handle) -> acpi_status;
    pub fn acpi_attach_data(object: acpi_handle, handler: acpi_object_handler, data: *mut c_void) -> acpi_status;
    pub fn acpi_detach_data(object: acpi_handle, handler: acpi_object_handler) -> acpi_status;
    pub fn acpi_get_data(object: acpi_handle, handler: acpi_object_handler, data: *mut *mut c_void) -> acpi_status;
    pub fn acpi_debug_trace(name: *const c_char, debug_level: u32, debug_layer: u32, flags: u32) -> acpi_status;
    pub fn acpi_evaluate_object(object: acpi_handle, pathname: acpi_string, parameter_objects: *mut acpi_object_list, return_object_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_evaluate_object_typed(object: acpi_handle, pathname: acpi_string, external_params: *mut acpi_object_list, return_buffer: *mut acpi_buffer, return_type: acpi_object_type) -> acpi_status;
    pub fn acpi_get_object_info(object: acpi_handle, return_buffer: *mut *mut acpi_device_info) -> acpi_status;
    pub fn acpi_install_method(buffer: *mut u8) -> acpi_status;
    pub fn acpi_get_next_object(ty: acpi_object_type, parent: acpi_handle, child: acpi_handle, out_handle: *mut acpi_handle) -> acpi_status;
    pub fn acpi_get_type(object: acpi_handle, out_type: *mut acpi_object_type) -> acpi_status;
    pub fn acpi_get_parent(object: acpi_handle, out_handle: *mut acpi_handle) -> acpi_status;
    pub fn acpi_install_initialization_handler(handler: acpi_init_handler, function: u32) -> acpi_status;
    pub fn acpi_install_sci_handler(address: acpi_sci_handler, context: *mut c_void) -> acpi_status;
    pub fn acpi_remove_sci_handler(address: acpi_sci_handler) -> acpi_status;
    pub fn acpi_install_global_event_handler(handler: acpi_gbl_event_handler, context: *mut c_void) -> acpi_status;
    pub fn acpi_install_fixed_event_handler(acpi_event: u32, handler: acpi_event_handler, context: *mut c_void) -> acpi_status;
    pub fn acpi_remove_fixed_event_handler(acpi_event: u32, handler: acpi_event_handler) -> acpi_status;
    pub fn acpi_install_gpe_handler(gpe_device: acpi_handle, gpe_number: u32, ty: u32, address: acpi_gpe_handler, context: *mut c_void) -> acpi_status;
    pub fn acpi_install_gpe_raw_handler(gpe_device: acpi_handle, gpe_number: u32, ty: u32, address: acpi_gpe_handler, context: *mut c_void) -> acpi_status;
    pub fn acpi_remove_gpe_handler(gpe_device: acpi_handle, gpe_number: u32, address: acpi_gpe_handler) -> acpi_status;
    pub fn acpi_install_notify_handler(device: acpi_handle, handler_type: u32, handler: acpi_notify_handler, context: *mut c_void) -> acpi_status;
    pub fn acpi_remove_notify_handler(device: acpi_handle, handler_type: u32, handler: acpi_notify_handler) -> acpi_status;
    pub fn acpi_install_address_space_handler(device: acpi_handle, space_id: acpi_adr_space_type, handler: acpi_adr_space_handler, setup: acpi_adr_space_setup, context: *mut c_void) -> acpi_status;
    pub fn acpi_install_address_space_handler_no_reg(device: acpi_handle, space_id: acpi_adr_space_type, handler: acpi_adr_space_handler, setup: acpi_adr_space_setup, context: *mut c_void) -> acpi_status;
    pub fn acpi_execute_reg_methods(device: acpi_handle, nax_depth: u32, space_id: acpi_adr_space_type) -> acpi_status;
    pub fn acpi_remove_address_space_handler(device: acpi_handle, space_id: acpi_adr_space_type, handler: acpi_adr_space_handler) -> acpi_status;
    pub fn acpi_install_exception_handler(handler: acpi_exception_handler) -> acpi_status;
    pub fn acpi_install_interface_handler(handler: acpi_interface_handler) -> acpi_status;
    pub fn acpi_acquire_global_lock(timeout: u16, handle: *mut u32) -> acpi_status;
    pub fn acpi_release_global_lock(handle: u32) -> acpi_status;
    pub fn acpi_acquire_mutex(handle: acpi_handle, pathname: acpi_string, timeout: u16) -> acpi_status;
    pub fn acpi_release_mutex(handle: acpi_handle, pathname: acpi_string) -> acpi_status;
    pub fn acpi_enable_event(event: u32, flags: u32) -> acpi_status;
    pub fn acpi_disable_event(event: u32, flags: u32) -> acpi_status;
    pub fn acpi_clear_event(event: u32) -> acpi_status;
    pub fn acpi_get_event_status(event: u32, event_status: *mut acpi_event_status) -> acpi_status;
    pub fn acpi_update_all_gpes() -> acpi_status;
    pub fn acpi_enable_gpe_cond(gpe_device: acpi_handle, gpe_number: u32, dispatch_type: u8) -> acpi_status;
    pub fn acpi_enable_gpe(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status;
    pub fn acpi_disable_gpe(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status;
    pub fn acpi_clear_gpe(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status;
    pub fn acpi_set_gpe(gpe_device: acpi_handle, gpe_number: u32, action: u8) -> acpi_status;
    pub fn acpi_finish_gpe(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status;
    pub fn acpi_mask_gpe(gpe_device: acpi_handle, gpe_number: u32, is_masked: u8) -> acpi_status;
    pub fn acpi_mark_gpe_for_wake(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status;
    pub fn acpi_setup_gpe_for_wake(parent_device: acpi_handle, gpe_device: acpi_handle, gpe_number: u32) -> acpi_status;
    pub fn acpi_set_gpe_wake_mask(gpe_device: acpi_handle, gpe_number: u32, action: u8) -> acpi_status;
    pub fn acpi_get_gpe_status(gpe_device: acpi_handle, gpe_number: u32, event_status: *mut acpi_event_status) -> acpi_status;
    pub fn acpi_dispatch_gpe(gpe_device: acpi_handle, gpe_number: u32) -> u32;
    pub fn acpi_hw_disable_all_gpes() -> acpi_status;
    pub fn acpi_hw_enable_all_wakeup_gpes() -> acpi_status;
    pub fn acpi_disable_all_gpes() -> acpi_status;
    pub fn acpi_enable_all_runtime_gpes() -> acpi_status;
    pub fn acpi_enable_all_wakeup_gpes() -> acpi_status;
    pub fn acpi_any_gpe_status_set(gpe_skip_number: u32) -> u32;
    pub fn acpi_any_fixed_event_status_set() -> u32;
    pub fn acpi_get_gpe_device(gpe_index: u32, gpe_device: *mut acpi_handle) -> acpi_status;
    pub fn acpi_install_gpe_block(gpe_device: acpi_handle, gpe_block_address: *mut acpi_generic_address, register_count: u32, interrupt_number: u32) -> acpi_status;
    pub fn acpi_remove_gpe_block(gpe_device: acpi_handle) -> acpi_status;
    pub fn acpi_get_vendor_resource(device: acpi_handle, name: *mut c_char, uuid: *mut acpi_vendor_uuid, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_get_current_resources(device: acpi_handle, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_get_possible_resources(device: acpi_handle, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_get_event_resources(device_handle: acpi_handle, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_walk_resource_buffer(buffer: *mut acpi_buffer, user_function: acpi_walk_resource_callback, context: *mut c_void) -> acpi_status;
    pub fn acpi_walk_resources(device: acpi_handle, name: *mut c_char, user_function: acpi_walk_resource_callback, context: *mut c_void) -> acpi_status;
    pub fn acpi_set_current_resources(device: acpi_handle, in_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_get_irq_routing_table(device: acpi_handle, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_resource_to_address64(resource: *mut acpi_resource, out: *mut acpi_resource_address64) -> acpi_status;
    pub fn acpi_buffer_to_resource(aml_buffer: *mut u8, aml_buffer_length: u16, resource_ptr: *mut *mut acpi_resource) -> acpi_status;
    pub fn acpi_reset() -> acpi_status;
    pub fn acpi_read(value: *mut u64, reg: *mut acpi_generic_address) -> acpi_status;
    pub fn acpi_write(value: u64, reg: *mut acpi_generic_address) -> acpi_status;
    pub fn acpi_read_bit_register(register_id: u32, return_value: *mut u32) -> acpi_status;
    pub fn acpi_write_bit_register(register_id: u32, value: u32) -> acpi_status;
    pub fn acpi_get_sleep_type_data(sleep_state: u8, slp_typ_a: *mut u8, slp_typ_b: *mut u8) -> acpi_status;
    pub fn acpi_enter_sleep_state_prep(sleep_state: u8) -> acpi_status;
    pub fn acpi_enter_sleep_state(sleep_state: u8) -> acpi_status;
    pub fn acpi_enter_sleep_state_s4bios() -> acpi_status;
    pub fn acpi_leave_sleep_state_prep(sleep_state: u8) -> acpi_status;
    pub fn acpi_leave_sleep_state(sleep_state: u8) -> acpi_status;
    pub fn acpi_set_firmware_waking_vector(physical_address: acpi_physical_address, physical_address64: acpi_physical_address) -> acpi_status;
    pub fn acpi_get_timer_resolution(resolution: *mut u32) -> acpi_status;
    pub fn acpi_get_timer(ticks: *mut u32) -> acpi_status;
    pub fn acpi_get_timer_duration(start_ticks: u32, end_ticks: u32, time_elapsed: *mut u32) -> acpi_status;
    pub fn acpi_error(module_name: *const c_char, line_number: u32, format: *const c_char, ...);
    pub fn acpi_exception(module_name: *const c_char, line_number: u32, status: acpi_status, format: *const c_char, ...);
    pub fn acpi_warning(module_name: *const c_char, line_number: u32, format: *const c_char, ...);
    pub fn acpi_info(format: *const c_char, ...);
    pub fn acpi_bios_error(module_name: *const c_char, line_number: u32, format: *const c_char, ...);
    pub fn acpi_bios_exception(module_name: *const c_char, line_number: u32, status: acpi_status, format: *const c_char, ...);
    pub fn acpi_bios_warning(module_name: *const c_char, line_number: u32, format: *const c_char, ...);
    pub fn acpi_debug_print(requested_debug_level: u32, line_number: u32, function_name: *const c_char, module_name: *const c_char, component_id: u32, format: *const c_char, ...);
    pub fn acpi_debug_print_raw(requested_debug_level: u32, line_number: u32, function_name: *const c_char, module_name: *const c_char, component_id: u32, format: *const c_char, ...);
    pub fn acpi_trace_point(ty: acpi_trace_event_type, begin: u8, aml: *mut u8, pathname: *mut c_char);
    pub fn acpi_initialize_debugger() -> acpi_status;
    pub fn acpi_terminate_debugger();
    pub fn acpi_get_data_full(object: acpi_handle, handler: acpi_object_handler, data: *mut *mut c_void, callback: Option<unsafe extern "C" fn(*mut c_void)>);
    pub fn acpi_set_debugger_thread_id(thread_id: acpi_thread_id);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
