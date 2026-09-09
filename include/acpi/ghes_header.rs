/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations from acpi/apei.h and acpi/hed.h are supplied externally. */

pub const GHES_EXITING: ::core::ffi::c_uint = 0x0002;

#[repr(C)]
pub union ghes__bindgen_ty_1 {
    pub generic: *mut acpi_hest_generic,
    pub generic_v2: *mut acpi_hest_generic_v2,
}

#[repr(C)]
pub union ghes__bindgen_ty_2 {
    pub list: ::core::mem::ManuallyDrop<list_head>,
    pub timer: ::core::mem::ManuallyDrop<timer_list>,
    pub irq: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct ghes {
    pub generic: ghes__bindgen_ty_1,
    pub estatus: *mut acpi_hest_generic_status,
    pub estatus_length: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_ulong,
    pub list_or_timer_or_irq: ghes__bindgen_ty_2,
    pub dev: *mut device,
    pub elist: list_head,
    pub error_status_vaddr: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct ghes_estatus_node {
    pub llnode: llist_node,
    pub generic: *mut acpi_hest_generic,
    pub ghes: *mut ghes,
}

#[repr(C)]
pub struct ghes_estatus_cache {
    pub estatus_len: u32,
    pub count: atomic_t,
    pub generic: *mut acpi_hest_generic,
    pub time_in: u64,
    pub rcu: rcu_head,
}

pub const GHES_SEV_NO: ::core::ffi::c_uint = 0x0;
pub const GHES_SEV_CORRECTED: ::core::ffi::c_uint = 0x1;
pub const GHES_SEV_RECOVERABLE: ::core::ffi::c_uint = 0x2;
pub const GHES_SEV_PANIC: ::core::ffi::c_uint = 0x3;

#[cfg(feature = "CONFIG_ACPI_APEI_GHES")]
extern "C" {
    pub fn ghes_register_vendor_record_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn ghes_unregister_vendor_record_notifier(nb: *mut notifier_block);
    pub fn devm_ghes_register_vendor_record_notifier(
        dev: *mut device,
        nb: *mut notifier_block,
    ) -> ::core::ffi::c_int;
    pub fn ghes_get_devices() -> *mut list_head;
    pub fn ghes_estatus_pool_region_free(addr: ::core::ffi::c_ulong, size: u32);
}

#[cfg(not(feature = "CONFIG_ACPI_APEI_GHES"))]
#[inline]
pub unsafe fn ghes_get_devices() -> *mut list_head {
    ::core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_ACPI_APEI_GHES"))]
#[inline]
pub unsafe fn ghes_estatus_pool_region_free(_addr: ::core::ffi::c_ulong, _size: u32) {}

extern "C" {
    pub fn ghes_estatus_pool_init(num_ghes: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn acpi_hest_get_version(gdata: *mut acpi_hest_generic_data) -> ::core::ffi::c_int {
    ((*gdata).revision >> 8) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn acpi_hest_get_payload(gdata: *mut acpi_hest_generic_data) -> *mut ::core::ffi::c_void {
    if acpi_hest_get_version(gdata) >= 3 {
        (gdata as *mut acpi_hest_generic_data_v300).add(1) as *mut ::core::ffi::c_void
    } else {
        gdata.add(1) as *mut ::core::ffi::c_void
    }
}

#[inline]
pub unsafe fn acpi_hest_get_error_length(gdata: *mut acpi_hest_generic_data) -> ::core::ffi::c_int {
    (*gdata).error_data_length as ::core::ffi::c_int
}

#[inline]
pub unsafe fn acpi_hest_get_size(gdata: *mut acpi_hest_generic_data) -> usize {
    if acpi_hest_get_version(gdata) >= 3 {
        ::core::mem::size_of::<acpi_hest_generic_data_v300>()
    } else {
        ::core::mem::size_of::<acpi_hest_generic_data>()
    }
}

#[inline]
pub unsafe fn acpi_hest_get_record_size(gdata: *mut acpi_hest_generic_data) -> usize {
    acpi_hest_get_size(gdata).wrapping_add(acpi_hest_get_error_length(gdata) as usize)
}

#[inline]
pub unsafe fn acpi_hest_get_next(gdata: *mut acpi_hest_generic_data) -> *mut ::core::ffi::c_void {
    (gdata as *mut u8).add(acpi_hest_get_record_size(gdata)) as *mut ::core::ffi::c_void
}

#[cfg(feature = "CONFIG_ACPI_APEI_SEA")]
extern "C" {
    pub fn ghes_notify_sea() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_ACPI_APEI_SEA"))]
#[inline]
pub unsafe fn ghes_notify_sea() -> ::core::ffi::c_int {
    -2 /* -ENOENT */
}

extern "C" {
    pub fn ghes_register_report_chain(nb: *mut notifier_block);
    pub fn ghes_unregister_report_chain(nb: *mut notifier_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
