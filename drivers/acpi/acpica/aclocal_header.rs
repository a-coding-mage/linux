/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Rust translation of aclocal.h. External ACPI types are supplied by other headers. */

pub const ACPI_SERIALIZED: u8 = 0xFF;
pub type acpi_mutex_handle = u32;
pub const ACPI_GLOBAL_LOCK: acpi_semaphore = -1 as acpi_semaphore;
pub const AML_NUM_OPCODES: u32 = 0x83;

pub const ACPI_MTX_INTERPRETER: u32 = 0;
pub const ACPI_MTX_NAMESPACE: u32 = 1;
pub const ACPI_MTX_TABLES: u32 = 2;
pub const ACPI_MTX_EVENTS: u32 = 3;
pub const ACPI_MTX_CACHES: u32 = 4;
pub const ACPI_MTX_MEMORY: u32 = 5;
pub const ACPI_MAX_MUTEX: u32 = 5;
pub const ACPI_NUM_MUTEX: u32 = ACPI_MAX_MUTEX + 1;
#[repr(C)] pub struct acpi_rw_lock { pub writer_mutex: acpi_mutex, pub reader_mutex: acpi_mutex, pub num_readers: u32 }
pub const ACPI_LOCK_GPES: u32 = 0;
pub const ACPI_LOCK_HARDWARE: u32 = 1;
pub const ACPI_MAX_LOCK: u32 = 1;
pub const ACPI_NUM_LOCK: u32 = ACPI_MAX_LOCK + 1;
pub const ACPI_MUTEX_NOT_ACQUIRED: acpi_thread_id = 0;
pub const ACPI_INVALID_THREAD_ID: acpi_thread_id = 0xFFFF_FFFF;
#[repr(C)] pub struct acpi_mutex_info { pub mutex: acpi_mutex, pub use_count: u32, pub thread_id: acpi_thread_id }
pub const ACPI_MTX_DO_NOT_LOCK: u32 = 0;
pub const ACPI_MTX_LOCK: u32 = 1;
pub const ACPI_FIELD_BYTE_GRANULARITY: u32 = 1;
pub const ACPI_FIELD_WORD_GRANULARITY: u32 = 2;
pub const ACPI_FIELD_DWORD_GRANULARITY: u32 = 4;
pub const ACPI_FIELD_QWORD_GRANULARITY: u32 = 8;
pub const ACPI_ENTRY_NOT_FOUND: *mut core::ffi::c_void = core::ptr::null_mut();

#[repr(C)] #[derive(Copy, Clone)] pub enum acpi_interpreter_mode { ACPI_IMODE_LOAD_PASS1 = 1, ACPI_IMODE_LOAD_PASS2 = 2, ACPI_IMODE_EXECUTE = 3 }
#[repr(C)] pub struct acpi_namespace_node {
    pub object: *mut acpi_operand_object, pub descriptor_type: u8, pub type_: u8, pub flags: u16,
    pub name: acpi_name_union, pub parent: *mut acpi_namespace_node, pub child: *mut acpi_namespace_node,
    pub peer: *mut acpi_namespace_node, pub owner_id: acpi_owner_id,
}
pub const ANOBJ_RESERVED:u8=1; pub const ANOBJ_TEMPORARY:u8=2; pub const ANOBJ_METHOD_ARG:u8=4; pub const ANOBJ_METHOD_LOCAL:u8=8;
pub const ANOBJ_SUBTREE_HAS_INI:u8=0x10; pub const ANOBJ_EVALUATED:u8=0x20; pub const ANOBJ_ALLOCATED_BUFFER:u8=0x40; pub const ANOBJ_NODE_EARLY_INIT:u8=0x80;
pub const ANOBJ_IS_EXTERNAL:u8=8; pub const ANOBJ_METHOD_NO_RETVAL:u8=0x10; pub const ANOBJ_METHOD_SOME_NO_RETVAL:u8=0x20; pub const ANOBJ_IS_ALIAS:u8=0x40; pub const ANOBJ_IS_REFERENCED:u8=0x80;
#[repr(C)] pub struct acpi_table_list { pub tables:*mut acpi_table_desc, pub current_table_count:u32, pub max_table_count:u32, pub flags:u8 }
pub const ACPI_ROOT_ORIGIN_UNKNOWN:u32=0; pub const ACPI_ROOT_ORIGIN_ALLOCATED:u32=1; pub const ACPI_ROOT_ALLOW_RESIZE:u32=2;
#[repr(C)] pub struct acpi_new_table_desc { pub table:*mut acpi_table_header, pub next:*mut acpi_new_table_desc }
pub const ACPI_INVALID_TABLE_INDEX:u32=0xFFFF_FFFF;
#[repr(C)] pub struct acpi_find_context { pub search_for:*mut i8, pub list:*mut acpi_handle, pub count:*mut u32 }
#[repr(C)] pub struct acpi_ns_search_data { pub node:*mut acpi_namespace_node }
pub const ACPI_COPY_TYPE_SIMPLE:u32=0; pub const ACPI_COPY_TYPE_PACKAGE:u32=1;
#[repr(C)] pub struct acpi_namestring_info { pub external_name:*const i8, pub next_external_char:*const i8, pub internal_name:*mut i8, pub length:u32, pub num_segments:u32, pub num_carats:u32, pub fully_qualified:u8 }
#[repr(C)] pub struct acpi_create_field_info { pub region_node:*mut acpi_namespace_node, pub field_node:*mut acpi_namespace_node, pub register_node:*mut acpi_namespace_node, pub data_register_node:*mut acpi_namespace_node, pub connection_node:*mut acpi_namespace_node, pub resource_buffer:*mut u8, pub bank_value:u32, pub field_bit_position:u32, pub field_bit_length:u32, pub resource_length:u16, pub pin_number_index:u16, pub field_flags:u8, pub attribute:u8, pub field_type:u8, pub access_length:u8 }
pub type acpi_internal_method = Option<unsafe extern "C" fn(*mut acpi_walk_state)->acpi_status>;

pub const ACPI_BTYPE_ANY:u32=0; pub const ACPI_BTYPE_INTEGER:u32=1; pub const ACPI_BTYPE_STRING:u32=2; pub const ACPI_BTYPE_BUFFER:u32=4; pub const ACPI_BTYPE_PACKAGE:u32=8; pub const ACPI_BTYPE_FIELD_UNIT:u32=0x10; pub const ACPI_BTYPE_DEVICE:u32=0x20; pub const ACPI_BTYPE_EVENT:u32=0x40; pub const ACPI_BTYPE_METHOD:u32=0x80; pub const ACPI_BTYPE_MUTEX:u32=0x100; pub const ACPI_BTYPE_REGION:u32=0x200; pub const ACPI_BTYPE_POWER:u32=0x400; pub const ACPI_BTYPE_PROCESSOR:u32=0x800; pub const ACPI_BTYPE_THERMAL:u32=0x1000; pub const ACPI_BTYPE_BUFFER_FIELD:u32=0x2000; pub const ACPI_BTYPE_DDB_HANDLE:u32=0x4000; pub const ACPI_BTYPE_DEBUG_OBJECT:u32=0x8000; pub const ACPI_BTYPE_REFERENCE_OBJECT:u32=0x10000; pub const ACPI_BTYPE_RESOURCE:u32=0x20000; pub const ACPI_BTYPE_NAMED_REFERENCE:u32=0x40000;
pub const ACPI_BTYPE_COMPUTE_DATA:u32=ACPI_BTYPE_INTEGER|ACPI_BTYPE_STRING|ACPI_BTYPE_BUFFER; pub const ACPI_BTYPE_DATA:u32=ACPI_BTYPE_COMPUTE_DATA|ACPI_BTYPE_PACKAGE; pub const ACPI_BTYPE_DATA_REFERENCE:u32=ACPI_BTYPE_DATA|ACPI_BTYPE_REFERENCE_OBJECT|ACPI_BTYPE_DDB_HANDLE; pub const ACPI_BTYPE_DEVICE_OBJECTS:u32=ACPI_BTYPE_DEVICE|ACPI_BTYPE_THERMAL|ACPI_BTYPE_PROCESSOR; pub const ACPI_BTYPE_OBJECTS_AND_REFS:u32=0x1FFFF; pub const ACPI_BTYPE_ALL_OBJECTS:u32=0xFFFF;

#[repr(C, packed)] pub struct acpi_name_info { pub name:[i8; ACPI_NAMESEG_SIZE as usize], pub argument_list:u16, pub expected_btypes:u8 }
#[repr(C, packed)] pub struct acpi_package_info { pub type_:u8, pub object_type1:u8, pub count1:u8, pub object_type2:u8, pub count2:u8, pub reserved:u16 }
#[repr(C, packed)] pub struct acpi_package_info2 { pub type_:u8, pub count:u8, pub object_type:[u8;4], pub reserved:u8 }
#[repr(C, packed)] pub struct acpi_package_info3 { pub type_:u8, pub count:u8, pub object_type:[u8;2], pub tail_object_type:u8, pub reserved:u16 }
#[repr(C, packed)] pub struct acpi_package_info4 { pub type_:u8, pub object_type1:u8, pub count1:u8, pub sub_object_types:u8, pub pkg_count:u8, pub reserved:u16 }
#[repr(C)] pub union acpi_predefined_info { pub info:acpi_name_info, pub ret_info:acpi_package_info, pub ret_info2:acpi_package_info2, pub ret_info3:acpi_package_info3, pub ret_info4:acpi_package_info4 }
pub type acpi_object_converter=Option<unsafe extern "C" fn(*mut acpi_namespace_node,*mut acpi_operand_object,*mut *mut acpi_operand_object)->acpi_status>;
#[repr(C)] pub struct acpi_simple_repair_info { pub name:[i8;ACPI_NAMESEG_SIZE as usize], pub unexpected_btypes:u32, pub package_index:u32, pub object_converter:acpi_object_converter }
pub const ACPI_RTYPE_ANY:u32=0; pub const ACPI_RTYPE_NONE:u32=1; pub const ACPI_RTYPE_INTEGER:u32=2; pub const ACPI_RTYPE_STRING:u32=4; pub const ACPI_RTYPE_BUFFER:u32=8; pub const ACPI_RTYPE_PACKAGE:u32=0x10; pub const ACPI_RTYPE_REFERENCE:u32=0x20; pub const ACPI_RTYPE_ALL:u32=0x3F; pub const ACPI_NUM_RTYPES:u32=5;
#[repr(C)] pub struct acpi_reg_walk_info { pub function:u32, pub reg_run_count:u32, pub space_id:acpi_adr_space_type }

#[repr(C)] pub struct acpi_sci_handler_info { pub next:*mut acpi_sci_handler_info, pub address:acpi_sci_handler, pub context:*mut core::ffi::c_void }
#[repr(C)] pub struct acpi_gpe_handler_info { pub address:acpi_gpe_handler, pub context:*mut core::ffi::c_void, pub method_node:*mut acpi_namespace_node, pub original_flags:u8, pub originally_enabled:u8 }
#[repr(C)] pub struct acpi_gpe_notify_info { pub device_node:*mut acpi_namespace_node, pub next:*mut acpi_gpe_notify_info }
#[repr(C)] pub union acpi_gpe_dispatch_info { pub method_node:*mut acpi_namespace_node, pub handler:*mut acpi_gpe_handler_info, pub notify_list:*mut acpi_gpe_notify_info }
#[repr(C)] pub struct acpi_gpe_event_info { pub dispatch:acpi_gpe_dispatch_info, pub register_info:*mut acpi_gpe_register_info, pub flags:u8, pub gpe_number:u8, pub runtime_count:u8, pub disable_for_dispatch:u8 }
#[repr(C)] pub struct acpi_gpe_address { pub space_id:u8, pub address:u64 }
#[repr(C)] pub struct acpi_gpe_register_info { pub status_address:acpi_gpe_address, pub enable_address:acpi_gpe_address, pub base_gpe_number:u16, pub enable_for_wake:u8, pub enable_for_run:u8, pub mask_for_run:u8, pub enable_mask:u8 }
#[repr(C)] pub struct acpi_gpe_block_info { pub node:*mut acpi_namespace_node, pub previous:*mut acpi_gpe_block_info, pub next:*mut acpi_gpe_block_info, pub xrupt_block:*mut acpi_gpe_xrupt_info, pub register_info:*mut acpi_gpe_register_info, pub event_info:*mut acpi_gpe_event_info, pub address:u64, pub register_count:u32, pub gpe_count:u16, pub block_base_number:u16, pub space_id:u8, pub initialized:u8 }
#[repr(C)] pub struct acpi_gpe_xrupt_info { pub previous:*mut acpi_gpe_xrupt_info, pub next:*mut acpi_gpe_xrupt_info, pub gpe_block_list_head:*mut acpi_gpe_block_info, pub interrupt_number:u32 }
#[repr(C)] pub struct acpi_gpe_walk_info { pub gpe_device:*mut acpi_namespace_node, pub gpe_block:*mut acpi_gpe_block_info, pub count:u16, pub owner_id:acpi_owner_id, pub execute_by_owner_id:u8 }
#[repr(C)] pub struct acpi_gpe_device_info { pub index:u32, pub next_block_base_index:u32, pub status:acpi_status, pub gpe_device:*mut acpi_namespace_node }
pub type acpi_gpe_callback=Option<unsafe extern "C" fn(*mut acpi_gpe_xrupt_info,*mut acpi_gpe_block_info,*mut core::ffi::c_void)->acpi_status>;
#[repr(C)] pub struct acpi_fixed_event_handler { pub handler:acpi_event_handler, pub context:*mut core::ffi::c_void }
#[repr(C)] pub struct acpi_fixed_event_info { pub status_register_id:u8, pub enable_register_id:u8, pub status_bit_mask:u16, pub enable_bit_mask:u16 }
#[repr(C)] pub struct acpi_field_info { pub skip_field:u8, pub field_flag:u8, pub pkg_length:u32 }
#[repr(C)] pub struct acpi_ged_handler_info { pub next:*mut acpi_ged_handler_info, pub int_id:u32, pub evt_method:*mut acpi_namespace_node }

#[repr(C)] pub struct acpi_common_state { pub next:*mut core::ffi::c_void, pub descriptor_type:u8, pub flags:u8, pub value:u16, pub state:u16 }
#[repr(C)] pub struct acpi_update_state { pub common:acpi_common_state, pub object:*mut acpi_operand_object }
#[repr(C)] pub struct acpi_pkg_state { pub common:acpi_common_state, pub index:u32, pub source_object:*mut acpi_operand_object, pub dest_object:*mut acpi_operand_object, pub walk_state:*mut acpi_walk_state, pub this_target_obj:*mut core::ffi::c_void, pub num_packages:u32 }
#[repr(C)] pub struct acpi_control_state { pub common:acpi_common_state, pub opcode:u16, pub predicate_op:*mut acpi_parse_object, pub aml_predicate_start:*mut u8, pub package_end:*mut u8, pub loop_timeout:u64 }
#[repr(C)] pub struct acpi_scope_state { pub common:acpi_common_state, pub node:*mut acpi_namespace_node }
#[repr(C)] pub struct acpi_pscope_state { pub common:acpi_common_state, pub arg_count:u32, pub op:*mut acpi_parse_object, pub arg_end:*mut u8, pub pkg_end:*mut u8, pub arg_list:u32 }
#[repr(C)] pub struct acpi_thread_state { pub common:acpi_common_state, pub current_sync_level:u8, pub walk_state_list:*mut acpi_walk_state, pub acquired_mutex_list:*mut acpi_operand_object, pub thread_id:acpi_thread_id }
#[repr(C)] pub struct acpi_result_values { pub common:acpi_common_state, pub obj_desc:[*mut acpi_operand_object; ACPI_RESULTS_FRAME_OBJ_NUM as usize] }
#[repr(C)] pub struct acpi_notify_info { pub common:acpi_common_state, pub handler_list_id:u8, pub node:*mut acpi_namespace_node, pub handler_list_head:*mut acpi_operand_object, pub global:*mut acpi_global_notify_handler }
#[repr(C)] pub union acpi_generic_state { pub common:acpi_common_state, pub control:acpi_control_state, pub update:acpi_update_state, pub scope:acpi_scope_state, pub parse_scope:acpi_pscope_state, pub pkg:acpi_pkg_state, pub thread:acpi_thread_state, pub results:acpi_result_values, pub notify:acpi_notify_info }
pub type acpi_execute_op=Option<unsafe extern "C" fn(*mut acpi_walk_state)->acpi_status>;
#[repr(C)] pub struct acpi_address_range { pub next:*mut acpi_address_range, pub region_node:*mut acpi_namespace_node, pub start_address:acpi_physical_address, pub end_address:acpi_physical_address }

#[repr(C)] pub struct acpi_opcode_info { pub name:*mut i8, pub parse_args:u32, pub runtime_args:u32, pub flags:u16, pub object_type:u8, pub class:u8, pub type_:u8 }
#[repr(C)] pub union acpi_parse_value { pub integer:u64, pub size:u32, pub string:*mut i8, pub buffer:*mut u8, pub name:*mut i8, pub arg:*mut acpi_parse_object }
#[repr(C)] pub struct acpi_parse_obj_common { pub parent:*mut acpi_parse_object, pub descriptor_type:u8, pub flags:u8, pub aml_opcode:u16, pub aml:*mut u8, pub next:*mut acpi_parse_object, pub node:*mut acpi_namespace_node, pub value:acpi_parse_value, pub arg_list_length:u8 }
#[repr(C)] pub struct acpi_parse_obj_named { pub common:acpi_parse_obj_common, pub path:*mut i8, pub data:*mut u8, pub length:u32, pub name:u32 }
#[repr(C)] pub struct acpi_parse_obj_asl { pub common:acpi_parse_obj_common, pub child:*mut acpi_parse_object, pub parent_method:*mut acpi_parse_object, pub filename:*mut i8, pub file_changed:u8, pub parent_filename:*mut i8, pub external_name:*mut i8, pub namepath:*mut i8, pub name_seg:[i8;4], pub extra_value:u32, pub column:u32, pub line_number:u32, pub logical_line_number:u32, pub logical_byte_offset:u32, pub end_line:u32, pub end_logical_line:u32, pub acpi_btype:u32, pub aml_length:u32, pub aml_subtree_length:u32, pub final_aml_length:u32, pub final_aml_offset:u32, pub compile_flags:u32, pub parse_opcode:u16, pub aml_opcode_length:u8, pub aml_pkg_len_bytes:u8, pub extra:u8, pub parse_op_name:[i8;20] }
#[repr(C)] pub union acpi_parse_object { pub common:acpi_parse_obj_common, pub named:acpi_parse_obj_named, pub asl:acpi_parse_obj_asl }
#[repr(C)] pub struct acpi_parse_state { pub aml_start:*mut u8, pub aml:*mut u8, pub aml_end:*mut u8, pub pkg_start:*mut u8, pub pkg_end:*mut u8, pub start_op:*mut acpi_parse_object, pub start_node:*mut acpi_namespace_node, pub scope:*mut acpi_generic_state, pub start_scope:*mut acpi_parse_object, pub aml_size:u32 }
pub const ACPI_PARSEOP_GENERIC:u8=1; pub const ACPI_PARSEOP_NAMED_OBJECT:u8=2; pub const ACPI_PARSEOP_DEFERRED:u8=4; pub const ACPI_PARSEOP_BYTELIST:u8=8; pub const ACPI_PARSEOP_IN_STACK:u8=0x10; pub const ACPI_PARSEOP_TARGET:u8=0x20; pub const ACPI_PARSEOP_IN_CACHE:u8=0x80;
pub const ACPI_CONTROL_NORMAL:u8=0xC0; pub const ACPI_CONTROL_CONDITIONAL_EXECUTING:u8=0xC1; pub const ACPI_CONTROL_PREDICATE_EXECUTING:u8=0xC2; pub const ACPI_CONTROL_PREDICATE_FALSE:u8=0xC3; pub const ACPI_CONTROL_PREDICATE_TRUE:u8=0xC4;
pub const ACPI_RESOURCE_NAME_LARGE:u8=0x80; pub const ACPI_RESOURCE_NAME_SMALL:u8=0; pub const ACPI_RESOURCE_NAME_SMALL_MASK:u8=0x78; pub const ACPI_RESOURCE_NAME_SMALL_LENGTH_MASK:u8=7; pub const ACPI_RESOURCE_NAME_LARGE_MASK:u8=0x7F;
pub const ACPI_RESOURCE_NAME_IRQ:u8=0x20; pub const ACPI_RESOURCE_NAME_DMA:u8=0x28; pub const ACPI_RESOURCE_NAME_START_DEPENDENT:u8=0x30; pub const ACPI_RESOURCE_NAME_END_DEPENDENT:u8=0x38; pub const ACPI_RESOURCE_NAME_IO:u8=0x40; pub const ACPI_RESOURCE_NAME_FIXED_IO:u8=0x48; pub const ACPI_RESOURCE_NAME_FIXED_DMA:u8=0x50; pub const ACPI_RESOURCE_NAME_VENDOR_SMALL:u8=0x70; pub const ACPI_RESOURCE_NAME_END_TAG:u8=0x78;
pub const ACPI_RESOURCE_NAME_MEMORY24:u8=0x81; pub const ACPI_RESOURCE_NAME_GENERIC_REGISTER:u8=0x82; pub const ACPI_RESOURCE_NAME_VENDOR_LARGE:u8=0x84; pub const ACPI_RESOURCE_NAME_MEMORY32:u8=0x85; pub const ACPI_RESOURCE_NAME_FIXED_MEMORY32:u8=0x86; pub const ACPI_RESOURCE_NAME_ADDRESS32:u8=0x87; pub const ACPI_RESOURCE_NAME_ADDRESS16:u8=0x88; pub const ACPI_RESOURCE_NAME_EXTENDED_IRQ:u8=0x89; pub const ACPI_RESOURCE_NAME_ADDRESS64:u8=0x8A; pub const ACPI_RESOURCE_NAME_EXTENDED_ADDRESS64:u8=0x8B; pub const ACPI_RESOURCE_NAME_GPIO:u8=0x8C; pub const ACPI_RESOURCE_NAME_PIN_FUNCTION:u8=0x8D; pub const ACPI_RESOURCE_NAME_SERIAL_BUS:u8=0x8E; pub const ACPI_RESOURCE_NAME_PIN_CONFIG:u8=0x8F; pub const ACPI_RESOURCE_NAME_PIN_GROUP:u8=0x90; pub const ACPI_RESOURCE_NAME_PIN_GROUP_FUNCTION:u8=0x91; pub const ACPI_RESOURCE_NAME_PIN_GROUP_CONFIG:u8=0x92; pub const ACPI_RESOURCE_NAME_CLOCK_INPUT:u8=0x93; pub const ACPI_RESOURCE_NAME_LARGE_MAX:u8=0x93;

#[repr(C)] pub struct acpi_bit_register_info { pub parent_register:u8, pub bit_position:u8, pub access_bit_mask:u16 }
pub const ACPI_PM1_STATUS_PRESERVED_BITS:u32=0x0800; pub const ACPI_PM1_CONTROL_WRITEONLY_BITS:u32=0x2004; pub const ACPI_PM1_CONTROL_IGNORED_BITS:u32=0x0200; pub const ACPI_PM1_CONTROL_RESERVED_BITS:u32=0xC1F8; pub const ACPI_PM1_CONTROL_PRESERVED_BITS:u32=ACPI_PM1_CONTROL_IGNORED_BITS|ACPI_PM1_CONTROL_RESERVED_BITS; pub const ACPI_PM2_CONTROL_PRESERVED_BITS:u32=0xFFFF_FFFE;
pub const ACPI_REGISTER_PM1_STATUS:u32=1; pub const ACPI_REGISTER_PM1_ENABLE:u32=2; pub const ACPI_REGISTER_PM1_CONTROL:u32=3; pub const ACPI_REGISTER_PM2_CONTROL:u32=4; pub const ACPI_REGISTER_PM_TIMER:u32=5; pub const ACPI_REGISTER_PROCESSOR_BLOCK:u32=6; pub const ACPI_REGISTER_SMI_COMMAND_BLOCK:u32=7;

#[repr(C)] pub struct acpi_interface_info { pub name:*mut i8, pub next:*mut acpi_interface_info, pub flags:u8, pub value:u8 }
pub const ACPI_ALWAYS_ILLEGAL:u32=0; pub const ACPI_OSI_INVALID:u8=1; pub const ACPI_OSI_DYNAMIC:u8=2; pub const ACPI_OSI_FEATURE:u8=4; pub const ACPI_OSI_DEFAULT_INVALID:u8=8; pub const ACPI_OSI_OPTIONAL_FEATURE:u8=0x0D;
#[repr(C)] pub struct acpi_port_info { pub name:*mut i8, pub start:u16, pub end:u16, pub osi_dependency:u8 }
pub const ACPI_ASCII_ZERO:u8=0x30;
#[repr(C)] pub struct acpi_external_list { pub path:*mut i8, pub internal_path:*mut i8, pub next:*mut acpi_external_list, pub value:u32, pub length:u16, pub flags:u16, pub type_:u8 }
pub const ACPI_EXT_RESOLVED_REFERENCE:u16=1; pub const ACPI_EXT_ORIGIN_FROM_FILE:u16=2; pub const ACPI_EXT_INTERNAL_PATH_ALLOCATED:u16=4; pub const ACPI_EXT_EXTERNAL_EMITTED:u16=8; pub const ACPI_EXT_ORIGIN_FROM_OPCODE:u16=0x10; pub const ACPI_EXT_CONFLICTING_DECLARATION:u16=0x20;
#[repr(C)] pub struct acpi_external_file { pub path:*mut i8, pub next:*mut acpi_external_file }
#[repr(C)] pub struct acpi_parse_object_list { pub op:*mut acpi_parse_object, pub next:*mut acpi_parse_object_list }
#[repr(C)] pub struct acpi_integrity_info { pub nodes:u32, pub objects:u32 }
#[repr(C)] pub struct acpi_object_info { pub types:[u32; ACPI_TOTAL_TYPES as usize] }
#[repr(C)] pub struct ah_predefined_name { pub name:*mut i8, pub description:*mut i8, pub action:*mut i8 }
#[repr(C)] pub struct ah_device_id { pub name:*mut i8, pub description:*mut i8 }
#[repr(C)] pub struct ah_uuid { pub description:*mut i8, pub string:*mut i8 }
#[repr(C)] pub struct ah_table { pub signature:*mut i8, pub description:*mut i8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
