/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * EFI Test driver Header
 *
 * Copyright(C) 2012-2016 Canonical Ltd.
 *
 */

// Dependency: Linux EFI types and ioctl encoding macros are supplied externally.

#[repr(C, packed)]
pub struct efi_getvariable {
    pub variable_name: *mut efi_char16_t,
    pub vendor_guid: *mut efi_guid_t,
    pub attributes: *mut u32,
    pub data_size: *mut ::core::ffi::c_ulong,
    pub data: *mut ::core::ffi::c_void,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_setvariable {
    pub variable_name: *mut efi_char16_t,
    pub vendor_guid: *mut efi_guid_t,
    pub attributes: u32,
    pub data_size: ::core::ffi::c_ulong,
    pub data: *mut ::core::ffi::c_void,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_getnextvariablename {
    pub variable_name_size: *mut ::core::ffi::c_ulong,
    pub variable_name: *mut efi_char16_t,
    pub vendor_guid: *mut efi_guid_t,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_queryvariableinfo {
    pub attributes: u32,
    pub maximum_variable_storage_size: *mut u64,
    pub remaining_variable_storage_size: *mut u64,
    pub maximum_variable_size: *mut u64,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_gettime {
    pub time: *mut efi_time_t,
    pub capabilities: *mut efi_time_cap_t,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_settime {
    pub time: *mut efi_time_t,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_getwakeuptime {
    pub enabled: *mut efi_bool_t,
    pub pending: *mut efi_bool_t,
    pub time: *mut efi_time_t,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_setwakeuptime {
    pub enabled: efi_bool_t,
    pub time: *mut efi_time_t,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_getnexthighmonotoniccount {
    pub high_count: *mut u32,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_querycapsulecapabilities {
    pub capsule_header_array: *mut *mut efi_capsule_header_t,
    pub capsule_count: ::core::ffi::c_ulong,
    pub maximum_capsule_size: *mut u64,
    pub reset_type: *mut ::core::ffi::c_int,
    pub status: *mut efi_status_t,
}

#[repr(C, packed)]
pub struct efi_resetsystem {
    pub reset_type: ::core::ffi::c_int,
    pub status: efi_status_t,
    pub data_size: ::core::ffi::c_ulong,
    pub data: *mut efi_char16_t,
}

// The ioctl values below delegate to the externally supplied Linux _IO* macros.
macro_rules! EFI_RUNTIME_GET_VARIABLE { () => { _IOWR!('p', 0x01, efi_getvariable) }; }
macro_rules! EFI_RUNTIME_SET_VARIABLE { () => { _IOW!('p', 0x02, efi_setvariable) }; }
macro_rules! EFI_RUNTIME_GET_TIME { () => { _IOR!('p', 0x03, efi_gettime) }; }
macro_rules! EFI_RUNTIME_SET_TIME { () => { _IOW!('p', 0x04, efi_settime) }; }
macro_rules! EFI_RUNTIME_GET_WAKETIME { () => { _IOR!('p', 0x05, efi_getwakeuptime) }; }
macro_rules! EFI_RUNTIME_SET_WAKETIME { () => { _IOW!('p', 0x06, efi_setwakeuptime) }; }
macro_rules! EFI_RUNTIME_GET_NEXTVARIABLENAME { () => { _IOWR!('p', 0x07, efi_getnextvariablename) }; }
macro_rules! EFI_RUNTIME_QUERY_VARIABLEINFO { () => { _IOR!('p', 0x08, efi_queryvariableinfo) }; }
macro_rules! EFI_RUNTIME_GET_NEXTHIGHMONOTONICCOUNT { () => { _IOR!('p', 0x09, efi_getnexthighmonotoniccount) }; }
macro_rules! EFI_RUNTIME_QUERY_CAPSULECAPABILITIES { () => { _IOR!('p', 0x0A, efi_querycapsulecapabilities) }; }
macro_rules! EFI_RUNTIME_RESET_SYSTEM { () => { _IOW!('p', 0x0B, efi_resetsystem) }; }
macro_rules! EFI_RUNTIME_GET_SUPPORTED_MASK { () => { _IOR!('p', 0x0C, ::core::ffi::c_uint) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
