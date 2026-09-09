/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Rust translation of acpi/actypes.h. Build-time configuration symbols are
 * intentionally left as dependency-provided cfgs/types. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type s16 = ::core::primitive::i16;
pub type s32 = ::core::primitive::i32;
pub type s64 = ::core::primitive::i64;

pub const ACPI_UINT8_MAX: u8 = u8::MAX;
pub const ACPI_UINT16_MAX: u16 = u16::MAX;
pub const ACPI_UINT32_MAX: u32 = u32::MAX;
pub const ACPI_UINT64_MAX: u64 = u64::MAX;
pub const ACPI_ASCII_MAX: u32 = 0x7f;

pub type acpi_thread_id = u64;
pub type acpi_native_int = isize;
pub type acpi_size = usize;
pub type acpi_io_address = u64;
pub type acpi_physical_address = u64;
pub type acpi_cpu_flags = acpi_size;
pub type acpi_spinlock = *mut ::core::ffi::c_void;
pub type acpi_raw_spinlock = acpi_spinlock;
pub type acpi_semaphore = *mut ::core::ffi::c_void;
pub type acpi_mutex = *mut ::core::ffi::c_void;
pub type acpi_uintptr_t = *mut ::core::ffi::c_void;

pub const ACPI_MAX_PTR: u64 = ACPI_UINT64_MAX;
pub const ACPI_SIZE_MAX: u64 = ACPI_UINT64_MAX;
pub const ACPI_MAX_GPE_BLOCKS: u32 = 2;
pub const ACPI_GPE_REGISTER_WIDTH: u32 = 8;
pub const ACPI_PM1_REGISTER_WIDTH: u32 = 16;
pub const ACPI_PM2_REGISTER_WIDTH: u32 = 8;
pub const ACPI_PM_TIMER_WIDTH: u32 = 32;
pub const ACPI_RESET_REGISTER_WIDTH: u32 = 8;
pub const ACPI_NAMESEG_SIZE: usize = 4;
pub const ACPI_PATH_SEGMENT_LENGTH: usize = 5;
pub const ACPI_PATH_SEPARATOR: u8 = b'.';
pub const ACPI_OEM_ID_SIZE: usize = 6;
pub const ACPI_OEM_TABLE_ID_SIZE: usize = 8;
pub const PCI_ROOT_HID_STRING: &str = "PNP0A03";
pub const PCI_EXPRESS_ROOT_HID_STRING: &str = "PNP0A08";
pub const ACPI_PM_TIMER_FREQUENCY: u32 = 3579545;

pub type acpi_status = u32;
pub type acpi_name = u32;
pub type acpi_string = *mut ::core::ffi::c_char;
pub type acpi_handle = *mut ::core::ffi::c_void;
pub type acpi_owner_id = u16;
pub type acpi_integer = u64;
pub const ACPI_OWNER_ID_MAX: u32 = 0xfff;
pub const ACPI_INTEGER_BIT_SIZE: u32 = 64;
pub const ACPI_WAIT_FOREVER: u16 = 0xffff;
pub const ACPI_DO_NOT_WAIT: u16 = 0;

#[inline] pub const fn ACPI_LOBYTE(x: u64) -> u8 { x as u8 }
#[inline] pub const fn ACPI_HIBYTE(x: u64) -> u8 { (x as u16 >> 8) as u8 }
#[inline] pub const fn ACPI_LOWORD(x: u64) -> u16 { x as u16 }
#[inline] pub const fn ACPI_HIWORD(x: u64) -> u16 { (x as u32 >> 16) as u16 }
#[inline] pub const fn ACPI_LODWORD(x: u64) -> u32 { x as u32 }
#[inline] pub const fn ACPI_HIDWORD(x: u64) -> u32 { (x >> 32) as u32 }
#[inline] pub const fn ACPI_ACCESS_BIT_WIDTH(size: i32) -> u32 { 1u32 << (size + 2) }
#[inline] pub const fn ACPI_ACCESS_BYTE_WIDTH(size: i32) -> u32 { 1u32 << (size - 1) }

pub type acpi_object_type = u32;
pub const ACPI_TYPE_ANY: u32 = 0x00;
pub const ACPI_TYPE_INTEGER: u32 = 0x01;
pub const ACPI_TYPE_STRING: u32 = 0x02;
pub const ACPI_TYPE_BUFFER: u32 = 0x03;
pub const ACPI_TYPE_PACKAGE: u32 = 0x04;
pub const ACPI_TYPE_FIELD_UNIT: u32 = 0x05;
pub const ACPI_TYPE_DEVICE: u32 = 0x06;
pub const ACPI_TYPE_EVENT: u32 = 0x07;
pub const ACPI_TYPE_METHOD: u32 = 0x08;
pub const ACPI_TYPE_MUTEX: u32 = 0x09;
pub const ACPI_TYPE_REGION: u32 = 0x0a;
pub const ACPI_TYPE_POWER: u32 = 0x0b;
pub const ACPI_TYPE_PROCESSOR: u32 = 0x0c;
pub const ACPI_TYPE_THERMAL: u32 = 0x0d;
pub const ACPI_TYPE_BUFFER_FIELD: u32 = 0x0e;
pub const ACPI_TYPE_DDB_HANDLE: u32 = 0x0f;
pub const ACPI_TYPE_DEBUG_OBJECT: u32 = 0x10;
pub const ACPI_TYPE_LOCAL_REGION_FIELD: u32 = 0x11;
pub const ACPI_TYPE_LOCAL_BANK_FIELD: u32 = 0x12;
pub const ACPI_TYPE_LOCAL_INDEX_FIELD: u32 = 0x13;
pub const ACPI_TYPE_LOCAL_REFERENCE: u32 = 0x14;
pub const ACPI_TYPE_LOCAL_ALIAS: u32 = 0x15;
pub const ACPI_TYPE_LOCAL_METHOD_ALIAS: u32 = 0x16;
pub const ACPI_TYPE_LOCAL_NOTIFY: u32 = 0x17;
pub const ACPI_TYPE_LOCAL_ADDRESS_HANDLER: u32 = 0x18;
pub const ACPI_TYPE_LOCAL_RESOURCE: u32 = 0x19;
pub const ACPI_TYPE_LOCAL_RESOURCE_FIELD: u32 = 0x1a;
pub const ACPI_TYPE_LOCAL_SCOPE: u32 = 0x1b;
pub const ACPI_TYPE_LOCAL_EXTRA: u32 = 0x1c;
pub const ACPI_TYPE_LOCAL_DATA: u32 = 0x1d;
pub const ACPI_TYPE_INVALID: u32 = 0x1e;
pub const ACPI_TYPE_NOT_FOUND: u32 = 0xff;

#[repr(C)] pub struct acpi_object_list { pub count: u32, pub pointer: *mut acpi_object }
#[repr(C)] pub struct acpi_buffer { pub length: acpi_size, pub pointer: *mut ::core::ffi::c_void }
#[repr(C)] pub struct acpi_pnp_device_id { pub length: u32, pub string: *mut ::core::ffi::c_char }
#[repr(C)] pub struct acpi_pnp_device_id_list { pub count: u32, pub list_size: u32, pub ids: [acpi_pnp_device_id; 0] }
#[repr(C)] pub struct acpi_pci_id { pub segment: u16, pub bus: u16, pub device: u16, pub function: u16 }
#[repr(C)] pub struct acpi_connection_info { pub connection: *mut u8, pub length: u16, pub access_length: u8 }
#[repr(C)] pub struct acpi_pcc_info { pub subspace_id: u8, pub length: u16, pub internal_buffer: *mut u8 }
#[repr(C)] pub struct acpi_ffh_info { pub offset: u64, pub length: u64 }
#[repr(C)] pub struct acpi_system_info { pub acpi_ca_version:u32, pub flags:u32, pub timer_resolution:u32, pub reserved1:u32, pub reserved2:u32, pub debug_level:u32, pub debug_layer:u32 }
#[repr(C)] pub struct acpi_statistics { pub sci_count:u32, pub gpe_count:u32, pub fixed_event_count:[u32; 5], pub method_count:u32 }

#[repr(C)] pub union acpi_object { pub type_: acpi_object_type, pub integer: acpi_object_integer, pub string: acpi_object_string, pub buffer: acpi_object_buffer, pub package: acpi_object_package, pub reference: acpi_object_reference, pub processor: acpi_object_processor, pub power_resource: acpi_object_power }
#[repr(C)] pub struct acpi_object_integer { pub type_: acpi_object_type, pub value:u64 }
#[repr(C)] pub struct acpi_object_string { pub type_: acpi_object_type, pub length:u32, pub pointer:*mut ::core::ffi::c_char }
#[repr(C)] pub struct acpi_object_buffer { pub type_: acpi_object_type, pub length:u32, pub pointer:*mut u8 }
#[repr(C)] pub struct acpi_object_package { pub type_: acpi_object_type, pub count:u32, pub elements:*mut acpi_object }
#[repr(C)] pub struct acpi_object_reference { pub type_: acpi_object_type, pub actual_type:acpi_object_type, pub handle:acpi_handle }
#[repr(C)] pub struct acpi_object_processor { pub type_:acpi_object_type, pub proc_id:u32, pub pblk_address:acpi_io_address, pub pblk_length:u32 }
#[repr(C)] pub struct acpi_object_power { pub type_:acpi_object_type, pub system_level:u32, pub resource_order:u32 }

pub type acpi_sci_handler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> u32>;
pub type acpi_event_handler = acpi_sci_handler;
pub type acpi_gpe_handler = Option<unsafe extern "C" fn(acpi_handle,u32,*mut ::core::ffi::c_void)->u32>;
pub type acpi_notify_handler = Option<unsafe extern "C" fn(acpi_handle,u32,*mut ::core::ffi::c_void)>;
pub type acpi_object_handler = Option<unsafe extern "C" fn(acpi_handle,*mut ::core::ffi::c_void)>;
pub type acpi_init_handler = Option<unsafe extern "C" fn(acpi_handle,u32)->acpi_status>;
pub type acpi_table_handler = Option<unsafe extern "C" fn(u32,*mut ::core::ffi::c_void,*mut ::core::ffi::c_void)->acpi_status>;
pub type acpi_interface_handler = Option<unsafe extern "C" fn(acpi_string,u32)->u32>;

pub const ACPI_READ:u32=0; pub const ACPI_WRITE:u32=1; pub const ACPI_IO_MASK:u32=1;
pub const ACPI_INTERRUPT_NOT_HANDLED:u32=0; pub const ACPI_INTERRUPT_HANDLED:u32=1;
pub const ACPI_REENABLE_GPE:u32=0x80; pub const ACPI_REGION_ACTIVATE:u32=0; pub const ACPI_REGION_DEACTIVATE:u32=1;
pub const ACPI_NUM_FIXED_EVENTS:usize=5; pub const ACPI_UUID_LENGTH:usize=16; pub const ACPI_EISAID_STRING_SIZE:usize=8; pub const ACPI_PCICLS_STRING_SIZE:usize=7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
