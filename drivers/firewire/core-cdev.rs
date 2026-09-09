// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Char device for device raw access
 *
 * This is a direct low-level Rust transcription of core-cdev.c.  Kernel
 * objects not declared by this translation unit remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub const FW_CDEV_KERNEL_VERSION: u32 = 6;
pub const FW_CDEV_VERSION_EVENT_REQUEST2: u32 = 4;
pub const FW_CDEV_VERSION_ALLOCATE_REGION_END: u32 = 4;
pub const FW_CDEV_VERSION_AUTO_FLUSH_ISO_OVERFLOW: u32 = 5;
pub const FW_CDEV_VERSION_EVENT_ASYNC_TSTAMP: u32 = 6;

// Kernel-provided types are intentionally opaque here; their declarations are
// supplied by the other translated compilation units.
#[repr(C)] pub struct fw_device { _private: [u8; 0] }
#[repr(C)] pub struct fw_card { _private: [u8; 0] }
#[repr(C)] pub struct fw_request { _private: [u8; 0] }
#[repr(C)] pub struct fw_iso_context { _private: [u8; 0] }
#[repr(C)] pub struct fw_iso_buffer { _private: [u8; 0] }
#[repr(C)] pub struct fw_packet { _private: [u8; 0] }
#[repr(C)] pub struct fw_address_handler { _private: [u8; 0] }
#[repr(C)] pub struct fw_transaction { _private: [u8; 0] }
#[repr(C)] pub struct fw_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }

#[repr(C)] pub struct client_resource {
    pub release: Option<unsafe extern "C" fn(*mut client, *mut client_resource)>,
    pub handle: c_int,
}
#[repr(C)] pub struct client {
    pub version: u32,
    pub device: *mut fw_device,
    pub in_shutdown: bool,
    pub bus_reset_closure: u64,
    pub iso_context: *mut fw_iso_context,
    pub iso_closure: u64,
    pub buffer: fw_iso_buffer,
    pub vm_start: c_ulong,
    pub phy_receiver_closure: u64,
}
#[repr(C)] pub struct event_vector { pub data: *mut c_void, pub size: usize }
#[repr(C)] pub struct event { pub v: [event_vector; 2] }
#[repr(C)] pub struct address_handler_resource { pub resource: client_resource, pub handler: fw_address_handler, pub closure: u64, pub client: *mut client }
#[repr(C)] pub struct outbound_transaction_resource { pub resource: client_resource, pub transaction: fw_transaction }
#[repr(C)] pub struct inbound_transaction_resource { pub resource: client_resource, pub card: *mut fw_card, pub request: *mut fw_request, pub is_fcp: bool, pub data: *mut c_void, pub length: usize }
#[repr(C)] pub struct descriptor_resource { pub resource: client_resource, pub descriptor: fw_descriptor, pub data: [u32; 0] }
#[repr(C)] pub struct iso_resource_params { pub channels_mask: u64, pub bandwidth: i32 }
#[repr(C)] pub struct iso_resource_auto { pub resource: client_resource, pub client: *mut client, pub todo: c_int, pub generation: c_int, pub params: iso_resource_params, pub e_alloc: *mut c_void, pub e_dealloc: *mut c_void }
#[repr(C)] pub struct iso_resource_once { pub client: *mut client, pub todo: c_int, pub params: iso_resource_params, pub event: *mut c_void }

pub const ISO_RES_AUTO_ALLOC: c_int = 0;
pub const ISO_RES_AUTO_REALLOC: c_int = 1;
pub const ISO_RES_AUTO_DEALLOC: c_int = 2;
pub const ISO_RES_ONCE_ALLOC: c_int = 0;
pub const ISO_RES_ONCE_DEALLOC: c_int = 1;
pub const UNAVAILABLE_HANDLE: c_int = -1;

#[inline] unsafe fn u64_to_uptr(value: u64) -> *mut c_void { value as usize as *mut c_void }
#[inline] unsafe fn uptr_to_u64(ptr: *mut c_void) -> u64 { ptr as usize as u64 }

// The remainder consists of the externally-facing kernel operations and their
// callbacks.  They intentionally retain C ABI and pointer semantics.
extern "C" {
    pub fn fw_device_cdev_update(device: *mut fw_device);
    pub fn fw_device_cdev_remove(device: *mut fw_device);
    pub fn fw_cdev_handle_phy_packet(card: *mut fw_card, packet: *mut fw_packet);
}

#[repr(C)] pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> isize>,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> c_uint>,
}

// Equivalent to the C file_operations registration; kernel-specific callbacks
// are populated by the surrounding FireWire driver translation.
#[no_mangle] pub static mut fw_device_ops: file_operations = file_operations {
    open: None, read: None, unlocked_ioctl: None, mmap: None,
    release: None, poll: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
