/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Rust translation of acglobal.h. C build-condition macros are retained as cfg intent. */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

/* The ACPICA types and constants below are supplied by the surrounding translation. */
extern "C" {
    pub static mut acpi_gbl_root_table_list: crate::acpi_table_list;
    pub static mut acpi_gbl_DSDT: *mut crate::acpi_table_header;
    pub static mut acpi_gbl_original_dsdt_header: crate::acpi_table_header;
    pub static mut acpi_gbl_CDAT: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_dsdt_index: u32;
    pub static mut acpi_gbl_facs_index: u32;
    pub static mut acpi_gbl_xfacs_index: u32;
    pub static mut acpi_gbl_fadt_index: u32;
    pub static mut acpi_gbl_FACS: *mut crate::acpi_table_facs;
    pub static mut acpi_gbl_xpm1a_status: crate::acpi_generic_address;
    pub static mut acpi_gbl_xpm1a_enable: crate::acpi_generic_address;
    pub static mut acpi_gbl_xpm1b_status: crate::acpi_generic_address;
    pub static mut acpi_gbl_xpm1b_enable: crate::acpi_generic_address;
    pub static mut acpi_gbl_integer_bit_width: u8;
    pub static mut acpi_gbl_integer_byte_width: u8;
    pub static mut acpi_gbl_integer_nybble_width: u8;
    pub static mut acpi_gbl_mutex_info: [crate::acpi_mutex_info; crate::ACPI_NUM_MUTEX];
    pub static mut acpi_gbl_global_lock_mutex: *mut crate::acpi_operand_object;
    pub static mut acpi_gbl_global_lock_semaphore: crate::acpi_semaphore;
    pub static mut acpi_gbl_global_lock_pending_lock: crate::acpi_spinlock;
    pub static mut acpi_gbl_global_lock_handle: u16;
    pub static mut acpi_gbl_global_lock_acquired: u8;
    pub static mut acpi_gbl_global_lock_present: u8;
    pub static mut acpi_gbl_global_lock_pending: u8;
    pub static mut acpi_gbl_gpe_lock: crate::acpi_spinlock;
    pub static mut acpi_gbl_hardware_lock: crate::acpi_raw_spinlock;
    pub static mut acpi_gbl_reference_count_lock: crate::acpi_spinlock;
    pub static mut acpi_gbl_osi_mutex: crate::acpi_mutex;
    pub static mut acpi_gbl_namespace_rw_lock: crate::acpi_rw_lock;
    pub static mut acpi_gbl_namespace_cache: *mut crate::acpi_cache_t;
    pub static mut acpi_gbl_state_cache: *mut crate::acpi_cache_t;
    pub static mut acpi_gbl_ps_node_cache: *mut crate::acpi_cache_t;
    pub static mut acpi_gbl_ps_node_ext_cache: *mut crate::acpi_cache_t;
    pub static mut acpi_gbl_operand_cache: *mut crate::acpi_cache_t;
    pub static mut acpi_gbl_startup_flags: u32;
    pub static mut acpi_gbl_shutdown: u8;
    pub static mut acpi_gbl_early_initialization: u8;
    pub static mut acpi_gbl_global_notify: [crate::acpi_global_notify_handler; 2];
    pub static mut acpi_gbl_exception_handler: crate::acpi_exception_handler;
    pub static mut acpi_gbl_init_handler: crate::acpi_init_handler;
    pub static mut acpi_gbl_table_handler: crate::acpi_table_handler;
    pub static mut acpi_gbl_table_handler_context: *mut ::core::ffi::c_void;
    pub static mut acpi_gbl_interface_handler: crate::acpi_interface_handler;
    pub static mut acpi_gbl_sci_handler_list: *mut crate::acpi_sci_handler_info;
    pub static mut acpi_gbl_ged_handler_list: *mut crate::acpi_ged_handler_info;
    pub static mut acpi_gbl_owner_id_mask: [u32; crate::ACPI_NUM_OWNERID_MASKS];
    pub static mut acpi_gbl_last_owner_id_index: u8;
    pub static mut acpi_gbl_next_owner_id_offset: u8;
    pub static mut acpi_gbl_namespace_initialized: u8;
    pub static mut acpi_gbl_original_mode: u32;
    pub static mut acpi_gbl_ns_lookup_count: u32;
    pub static mut acpi_gbl_ps_find_count: u32;
    pub static mut acpi_gbl_pm1_enable_register_save: u16;
    pub static mut acpi_gbl_debugger_configuration: u8;
    pub static mut acpi_gbl_step_to_next_call: u8;
    pub static mut acpi_gbl_acpi_hardware_present: u8;
    pub static mut acpi_gbl_events_initialized: u8;
    pub static mut acpi_gbl_supported_interfaces: *mut crate::acpi_interface_info;
    pub static mut acpi_gbl_address_range_list: [*mut crate::acpi_address_range; crate::ACPI_ADDRESS_RANGE_MAX];
    pub static mut acpi_gbl_root_node_struct: crate::acpi_namespace_node;
    pub static mut acpi_gbl_root_node: *mut crate::acpi_namespace_node;
    pub static mut acpi_gbl_fadt_gpe_device: *mut crate::acpi_namespace_node;
    pub static mut acpi_gbl_cm_single_step: u8;
    pub static mut acpi_gbl_current_walk_list: *mut crate::acpi_thread_state;
    pub static mut acpi_gbl_current_scope: *mut crate::acpi_parse_object;
    pub static mut acpi_gbl_capture_comments: u8;
    pub static mut acpi_gbl_last_list_head: *mut crate::acpi_comment_node;
    pub static mut acpi_gbl_bit_register_info: [crate::acpi_bit_register_info; crate::ACPI_NUM_BITREG];
    pub static mut acpi_gbl_sleep_type_a: u8;
    pub static mut acpi_gbl_sleep_type_b: u8;
    pub static mut acpi_gbl_sleep_type_a_s0: u8;
    pub static mut acpi_gbl_sleep_type_b_s0: u8;
    pub static mut acpi_method_count: u32;
    pub static mut acpi_gpe_count: u32;
    pub static mut acpi_sci_count: u32;
    pub static mut acpi_fixed_event_count: [u32; crate::ACPI_NUM_FIXED_EVENTS];
    pub static mut acpi_gbl_original_dbg_level: u32;
    pub static mut acpi_gbl_original_dbg_layer: u32;
    pub static mut acpi_gbl_db_output_flags: u8;
    pub static mut acpi_gbl_all_gpes_initialized: u8;
    pub static mut acpi_gbl_gpe_xrupt_list_head: *mut crate::acpi_gpe_xrupt_info;
    pub static mut acpi_gbl_gpe_fadt_blocks: [*mut crate::acpi_gpe_block_info; crate::ACPI_MAX_GPE_BLOCKS];
    pub static mut acpi_gbl_global_event_handler: crate::acpi_gbl_event_handler;
    pub static mut acpi_gbl_global_event_handler_context: *mut ::core::ffi::c_void;
    pub static mut acpi_gbl_fixed_event_handlers: [crate::acpi_fixed_event_handler; crate::ACPI_NUM_FIXED_EVENTS];
    pub static mut acpi_gbl_no_resource_disassembly: u8;
    pub static mut acpi_gbl_ignore_noop_operator: u8;
    pub static mut acpi_gbl_cstyle_disassembly: u8;
    pub static mut acpi_gbl_force_aml_disassembly: u8;
    pub static mut acpi_gbl_dm_opt_verbose: u8;
    pub static mut acpi_gbl_dm_emit_external_opcodes: u8;
    pub static mut acpi_gbl_do_disassembler_optimizations: u8;
    pub static mut acpi_gbl_dm_opt_disasm: u8;
    pub static mut acpi_gbl_dm_opt_listing: u8;
    pub static mut acpi_gbl_num_external_methods: u8;
    pub static mut acpi_gbl_resolved_external_methods: u32;
    pub static mut acpi_gbl_external_list: *mut crate::acpi_external_list;
    pub static mut acpi_gbl_external_file_list: *mut crate::acpi_external_file;
    pub static mut acpi_gbl_abort_method: u8;
    pub static mut acpi_gbl_db_thread_id: crate::acpi_thread_id;
    pub static mut acpi_gbl_next_cmd_num: u32;
    pub static mut acpi_gbl_db_opt_no_ini_methods: u8;
    pub static mut acpi_gbl_db_opt_no_region_support: u8;
    pub static mut acpi_gbl_db_output_to_file: u8;
    pub static mut acpi_gbl_db_buffer: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_db_filename: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_db_debug_level: u32;
    pub static mut acpi_gbl_db_console_debug_level: u32;
    pub static mut acpi_gbl_db_scope_node: *mut crate::acpi_namespace_node;
    pub static mut acpi_gbl_db_terminate_loop: u8;
    pub static mut acpi_gbl_db_threads_terminated: u8;
    pub static mut acpi_gbl_db_args: [*mut ::core::ffi::c_char; crate::ACPI_DEBUGGER_MAX_ARGS];
    pub static mut acpi_gbl_db_arg_types: [crate::acpi_object_type; crate::ACPI_DEBUGGER_MAX_ARGS];
    pub static mut acpi_gbl_db_parsed_buf: [::core::ffi::c_char; crate::ACPI_DB_LINE_BUFFER_SIZE];
    pub static mut acpi_gbl_db_scope_buf: [::core::ffi::c_char; crate::ACPI_DB_LINE_BUFFER_SIZE];
    pub static mut acpi_gbl_db_debug_filename: [::core::ffi::c_char; crate::ACPI_DB_LINE_BUFFER_SIZE];
    pub static mut acpi_gbl_obj_type_count: [u16; crate::ACPI_TOTAL_TYPES];
    pub static mut acpi_gbl_node_type_count: [u16; crate::ACPI_TOTAL_TYPES];
    pub static mut acpi_gbl_obj_type_count_misc: u16;
    pub static mut acpi_gbl_node_type_count_misc: u16;
    pub static mut acpi_gbl_num_nodes: u32;
    pub static mut acpi_gbl_num_objects: u32;
    pub static mut acpi_gbl_disasm_flag: u8;
    pub static mut acpi_gbl_current_inline_comment: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_current_end_node_comment: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_current_open_brace_comment: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_current_close_brace_comment: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_root_filename: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_current_filename: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_current_parent_filename: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_current_include_filename: *mut ::core::ffi::c_char;
    pub static mut acpi_gbl_debug_asl_conversion: u8;
    pub static mut acpi_gbl_table_sig: [::core::ffi::c_char; 4];
    pub static mut acpi_gbl_debug_timeout: u8;
    pub static mut acpi_gbl_print_lock: crate::acpi_spinlock;
    pub static mut acpi_gbl_print_buffer: [::core::ffi::c_char; 1024];
}

pub const NUM_PREDEFINED_NAMES: usize = 10;

/* Declaration-only globals initialized in utglobal. */
extern "C" {
    pub static acpi_gbl_sleep_state_names: *const *const ::core::ffi::c_char;
    pub static acpi_gbl_lowest_dstate_names: *const *const ::core::ffi::c_char;
    pub static acpi_gbl_highest_dstate_names: *const *const ::core::ffi::c_char;
    pub static acpi_gbl_region_types: *const *const ::core::ffi::c_char;
    pub static acpi_gbl_lower_hex_digits: *const ::core::ffi::c_char;
    pub static acpi_gbl_upper_hex_digits: *const ::core::ffi::c_char;
    pub static acpi_gbl_ns_properties: *const u8;
    pub static acpi_gbl_pre_defined_names: *const crate::acpi_predefined_names;
}

/* Optional declarations guarded in C by ACPI_GPE_USE_LOGICAL_ADDRESSES. */
#[cfg(feature = "ACPI_GPE_USE_LOGICAL_ADDRESSES")]
extern "C" {
    pub static mut acpi_gbl_xgpe0_block_logical_address: usize;
    pub static mut acpi_gbl_xgpe1_block_logical_address: usize;
}

/* Remaining feature-specific ACPICA globals retain their C conditional intent. */
#[cfg(feature = "ACPI_REDUCED_HARDWARE")]
pub const ACPI_REDUCED_HARDWARE_CONDITION: bool = true;
#[cfg(feature = "ACPI_DISASSEMBLER")]
pub const ACPI_DISASSEMBLER_CONDITION: bool = true;
#[cfg(feature = "ACPI_DEBUGGER")]
pub const ACPI_DEBUGGER_CONDITION: bool = true;
#[cfg(feature = "ACPI_ASL_COMPILER")]
pub const ACPI_ASL_COMPILER_CONDITION: bool = true;
#[cfg(feature = "ACPI_APPLICATION")]
pub const ACPI_APPLICATION_CONDITION: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
