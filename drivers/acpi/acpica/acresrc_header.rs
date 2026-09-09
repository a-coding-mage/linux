/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/*
 * Rust translation of acresrc.h - Resource Manager function prototypes.
 *
 * The C header includes amlresrc.h; its declarations are supplied by the
 * surrounding translation unit.
 */

use core::ffi::c_char;

/* The C header conditionally packs these structures to byte alignment. */
#[repr(C, packed)]
pub struct acpi_rsconvert_info {
    pub opcode: u8,
    pub resource_offset: u8,
    pub aml_offset: u8,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ACPI_RSCONVERT_OPCODES {
    ACPI_RSC_INITGET = 0,
    ACPI_RSC_INITSET,
    ACPI_RSC_FLAGINIT,
    ACPI_RSC_1BITFLAG,
    ACPI_RSC_2BITFLAG,
    ACPI_RSC_3BITFLAG,
    ACPI_RSC_6BITFLAG,
    ACPI_RSC_ADDRESS,
    ACPI_RSC_BITMASK,
    ACPI_RSC_BITMASK16,
    ACPI_RSC_COUNT,
    ACPI_RSC_COUNT16,
    ACPI_RSC_COUNT_GPIO_PIN,
    ACPI_RSC_COUNT_GPIO_RES,
    ACPI_RSC_COUNT_GPIO_VEN,
    ACPI_RSC_COUNT_SERIAL_RES,
    ACPI_RSC_COUNT_SERIAL_VEN,
    ACPI_RSC_DATA8,
    ACPI_RSC_EXIT_EQ,
    ACPI_RSC_EXIT_LE,
    ACPI_RSC_EXIT_NE,
    ACPI_RSC_LENGTH,
    ACPI_RSC_MOVE_GPIO_PIN,
    ACPI_RSC_MOVE_GPIO_RES,
    ACPI_RSC_MOVE_SERIAL_RES,
    ACPI_RSC_MOVE_SERIAL_VEN,
    ACPI_RSC_MOVE8,
    ACPI_RSC_MOVE16,
    ACPI_RSC_MOVE32,
    ACPI_RSC_MOVE64,
    ACPI_RSC_SET8,
    ACPI_RSC_SOURCE,
    ACPI_RSC_SOURCEX,
}

pub const ACPI_RSC_COMPARE_AML_LENGTH: u32 = 0;
pub const ACPI_RSC_COMPARE_VALUE: u32 = 1;

macro_rules! ACPI_RSC_TABLE_SIZE {
    ($d:expr) => { core::mem::size_of_val(&$d) / core::mem::size_of::<acpi_rsconvert_info>() };
}

macro_rules! ACPI_RS_OFFSET {
    ($f:tt) => { core::mem::offset_of!(acpi_resource, $f) as u8 };
}

macro_rules! AML_OFFSET {
    ($f:tt) => { core::mem::offset_of!(aml_resource, $f) as u8 };
}

#[repr(C)]
pub struct acpi_rsdump_info {
    pub opcode: u8,
    pub offset: u8,
    pub name: *const c_char,
    pub pointer: *mut *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ACPI_RSDUMP_OPCODES {
    ACPI_RSD_TITLE = 0,
    ACPI_RSD_1BITFLAG,
    ACPI_RSD_2BITFLAG,
    ACPI_RSD_3BITFLAG,
    ACPI_RSD_6BITFLAG,
    ACPI_RSD_ADDRESS,
    ACPI_RSD_DWORDLIST,
    ACPI_RSD_LITERAL,
    ACPI_RSD_LONGLIST,
    ACPI_RSD_SHORTLIST,
    ACPI_RSD_SHORTLISTX,
    ACPI_RSD_SOURCE,
    ACPI_RSD_STRING,
    ACPI_RSD_UINT8,
    ACPI_RSD_UINT16,
    ACPI_RSD_UINT32,
    ACPI_RSD_UINT64,
    ACPI_RSD_WORDLIST,
    ACPI_RSD_LABEL,
    ACPI_RSD_SOURCE_LABEL,
}

extern "C" {
    pub static acpi_gbl_aml_resource_sizes: [u8; 0];
    pub static acpi_gbl_aml_resource_serial_bus_sizes: [u8; 0];
    pub static mut acpi_gbl_set_resource_dispatch: [*mut acpi_rsconvert_info; 0];
    pub static acpi_gbl_resource_struct_sizes: [u8; 0];
    pub static acpi_gbl_resource_struct_serial_bus_sizes: [u8; 0];
    pub static mut acpi_gbl_get_resource_dispatch: [*mut acpi_rsconvert_info; 0];
    pub static mut acpi_gbl_convert_resource_serial_bus_dispatch: [*mut acpi_rsconvert_info; 0];
}

#[repr(C)]
pub struct acpi_vendor_walk_info {
    pub uuid: *mut acpi_vendor_uuid,
    pub buffer: *mut acpi_buffer,
    pub status: acpi_status,
}

extern "C" {
    pub fn acpi_rs_create_resource_list(aml_buffer: *mut acpi_operand_object, output_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_rs_create_aml_resources(resource_list: *mut acpi_buffer, output_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_rs_create_pci_routing_table(package_object: *mut acpi_operand_object, output_buffer: *mut acpi_buffer) -> acpi_status;

    pub fn acpi_rs_get_prt_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_rs_get_crs_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_rs_get_prs_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_rs_get_method_data(handle: acpi_handle, path: *const c_char, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_rs_set_srs_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status;
    pub fn acpi_rs_get_aei_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status;

    pub fn acpi_rs_get_list_length(aml_buffer: *mut u8, aml_buffer_length: u32, size_needed: *mut acpi_size) -> acpi_status;
    pub fn acpi_rs_get_aml_length(resource_list: *mut acpi_resource, resource_list_size: acpi_size, size_needed: *mut acpi_size) -> acpi_status;
    pub fn acpi_rs_get_pci_routing_table_length(package_object: *mut acpi_operand_object, buffer_size_needed: *mut acpi_size) -> acpi_status;
    pub fn acpi_rs_convert_aml_to_resources(aml: *mut u8, length: u32, offset: u32, resource_index: u8, context: *mut *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_rs_convert_resources_to_aml(resource: *mut acpi_resource, aml_size_needed: acpi_size, output_buffer: *mut u8) -> acpi_status;

    pub fn acpi_rs_set_address_common(aml: *mut aml_resource, resource: *mut acpi_resource);
    pub fn acpi_rs_get_address_common(resource: *mut acpi_resource, aml: *mut aml_resource) -> u8;
    pub fn acpi_rs_convert_aml_to_resource(resource: *mut acpi_resource, aml: *mut aml_resource, info: *mut acpi_rsconvert_info) -> acpi_status;
    pub fn acpi_rs_convert_resource_to_aml(resource: *mut acpi_resource, aml: *mut aml_resource, info: *mut acpi_rsconvert_info) -> acpi_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
