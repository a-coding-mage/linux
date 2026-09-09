/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/*
 * Rust translation of acpiosxf.h. Types and symbols supplied by the ACPI
 * platform/type headers are intentionally referenced but not defined here.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum acpi_execute_type {
    OSL_GLOBAL_LOCK_HANDLER,
    OSL_NOTIFY_HANDLER,
    OSL_GPE_HANDLER,
    OSL_DEBUGGER_MAIN_THREAD,
    OSL_DEBUGGER_EXEC_THREAD,
    OSL_EC_POLL_HANDLER,
    OSL_EC_BURST_HANDLER,
}

pub const ACPI_NO_UNIT_LIMIT: u32 = u32::MAX;
pub const ACPI_MUTEX_SEM: u32 = 1;
pub const ACPI_SIGNAL_FATAL: u32 = 0;
pub const ACPI_SIGNAL_BREAKPOINT: u32 = 1;

#[repr(C)]
pub struct acpi_signal_fatal_info {
    pub r#type: u32,
    pub code: u32,
    pub argument: u32,
}

// ACPI_MUTEX_TYPE != ACPI_BINARY_SEMAPHORE is a build-time condition from acenv.h.

extern "C" {
    pub fn acpi_os_initialize() -> acpi_status;
    pub fn acpi_os_terminate() -> acpi_status;
    pub fn acpi_os_get_root_pointer() -> acpi_physical_address;
    pub fn acpi_os_predefined_override(init_val: *const acpi_predefined_names, new_val: *mut acpi_string) -> acpi_status;
    pub fn acpi_os_table_override(existing_table: *mut acpi_table_header, new_table: *mut *mut acpi_table_header) -> acpi_status;
    pub fn acpi_os_physical_table_override(existing_table: *mut acpi_table_header, new_address: *mut acpi_physical_address, new_table_length: *mut u32) -> acpi_status;

    pub fn acpi_os_create_lock(out_handle: *mut acpi_spinlock) -> acpi_status;
    pub fn acpi_os_delete_lock(handle: acpi_spinlock);
    pub fn acpi_os_acquire_lock(handle: acpi_spinlock) -> acpi_cpu_flags;
    pub fn acpi_os_release_lock(handle: acpi_spinlock, flags: acpi_cpu_flags);

    pub fn acpi_os_create_semaphore(max_units: u32, initial_units: u32, out_handle: *mut acpi_semaphore) -> acpi_status;
    pub fn acpi_os_delete_semaphore(handle: acpi_semaphore) -> acpi_status;
    pub fn acpi_os_wait_semaphore(handle: acpi_semaphore, units: u32, timeout: u16) -> acpi_status;
    pub fn acpi_os_signal_semaphore(handle: acpi_semaphore, units: u32) -> acpi_status;

    pub fn acpi_os_create_mutex(out_handle: *mut acpi_mutex) -> acpi_status;
    pub fn acpi_os_delete_mutex(handle: acpi_mutex);
    pub fn acpi_os_acquire_mutex(handle: acpi_mutex, timeout: u16) -> acpi_status;
    pub fn acpi_os_release_mutex(handle: acpi_mutex);

    pub fn acpi_os_allocate(size: acpi_size) -> *mut core::ffi::c_void;
    pub fn acpi_os_allocate_zeroed(size: acpi_size) -> *mut core::ffi::c_void;
    pub fn acpi_os_free(memory: *mut core::ffi::c_void);
    pub fn acpi_os_map_memory(where_: acpi_physical_address, length: acpi_size) -> *mut core::ffi::c_void;
    pub fn acpi_os_unmap_memory(logical_address: *mut core::ffi::c_void, size: acpi_size);
    pub fn acpi_os_get_physical_address(logical_address: *mut core::ffi::c_void, physical_address: *mut acpi_physical_address) -> acpi_status;

    pub fn acpi_os_create_cache(cache_name: *mut i8, object_size: u16, max_depth: u16, return_cache: *mut *mut acpi_cache_t) -> acpi_status;
    pub fn acpi_os_delete_cache(cache: *mut acpi_cache_t) -> acpi_status;
    pub fn acpi_os_purge_cache(cache: *mut acpi_cache_t) -> acpi_status;
    pub fn acpi_os_acquire_object(cache: *mut acpi_cache_t) -> *mut core::ffi::c_void;
    pub fn acpi_os_release_object(cache: *mut acpi_cache_t, object: *mut core::ffi::c_void) -> acpi_status;

    pub fn acpi_os_install_interrupt_handler(interrupt_number: u32, service_routine: acpi_osd_handler, context: *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_os_remove_interrupt_handler(interrupt_number: u32, service_routine: acpi_osd_handler) -> acpi_status;
    pub fn acpi_os_get_thread_id() -> acpi_thread_id;
    pub fn acpi_os_execute(r#type: acpi_execute_type, function: acpi_osd_exec_callback, context: *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_os_wait_events_complete();
    pub fn acpi_os_sleep(milliseconds: u64);
    pub fn acpi_os_stall(microseconds: u32);

    pub fn acpi_os_read_port(address: acpi_io_address, value: *mut u32, width: u32) -> acpi_status;
    pub fn acpi_os_write_port(address: acpi_io_address, value: u32, width: u32) -> acpi_status;
    pub fn acpi_os_read_iomem(virt_addr: *mut core::ffi::c_void, value: *mut u64, width: u32) -> i32;
    pub fn acpi_os_read_memory(address: acpi_physical_address, value: *mut u64, width: u32) -> acpi_status;
    pub fn acpi_os_write_memory(address: acpi_physical_address, value: u64, width: u32) -> acpi_status;
    pub fn acpi_os_read_pci_configuration(pci_id: *mut acpi_pci_id, reg: u32, value: *mut u64, width: u32) -> acpi_status;
    pub fn acpi_os_write_pci_configuration(pci_id: *mut acpi_pci_id, reg: u32, value: u64, width: u32) -> acpi_status;

    pub fn acpi_os_readable(pointer: *mut core::ffi::c_void, length: acpi_size) -> u8;
    pub fn acpi_os_writable(pointer: *mut core::ffi::c_void, length: acpi_size) -> u8;
    pub fn acpi_os_get_timer() -> u64;
    pub fn acpi_os_signal(function: u32, info: *mut core::ffi::c_void) -> acpi_status;
    pub fn acpi_os_enter_sleep(sleep_state: u8, rega_value: u32, regb_value: u32) -> acpi_status;
    pub fn acpi_os_printf(format: *const i8, ...);
    pub fn acpi_os_vprintf(format: *const i8, args: va_list);
    pub fn acpi_os_redirect_output(destination: *mut core::ffi::c_void);
    pub fn acpi_os_get_line(buffer: *mut i8, buffer_length: u32, bytes_read: *mut u32) -> acpi_status;
    pub fn acpi_os_initialize_debugger() -> acpi_status;
    pub fn acpi_os_terminate_debugger();
    pub fn acpi_os_wait_command_ready() -> acpi_status;
    pub fn acpi_os_notify_command_complete() -> acpi_status;
    pub fn acpi_os_trace_point(r#type: acpi_trace_event_type, begin: u8, aml: *mut u8, pathname: *mut i8);
    pub fn acpi_os_get_table_by_name(signature: *mut i8, instance: u32, table: *mut *mut acpi_table_header, address: *mut acpi_physical_address) -> acpi_status;
    pub fn acpi_os_get_table_by_index(index: u32, table: *mut *mut acpi_table_header, instance: *mut u32, address: *mut acpi_physical_address) -> acpi_status;
    pub fn acpi_os_get_table_by_address(address: acpi_physical_address, table: *mut *mut acpi_table_header) -> acpi_status;
    pub fn acpi_os_open_directory(pathname: *mut i8, wildcard_spec: *mut i8, requested_file_type: i8) -> *mut core::ffi::c_void;
    pub fn acpi_os_get_next_filename(dir_handle: *mut core::ffi::c_void) -> *mut i8;
    pub fn acpi_os_close_directory(dir_handle: *mut core::ffi::c_void);
}

pub const REQUEST_FILE_ONLY: i32 = 0;
pub const REQUEST_DIR_ONLY: i32 = 1;

// Raw-lock fallbacks when alternate OS prototypes are not supplied.
#[inline] pub unsafe fn acpi_os_create_raw_lock(out_handle: *mut acpi_spinlock) -> acpi_status { acpi_os_create_lock(out_handle) }
#[inline] pub unsafe fn acpi_os_delete_raw_lock(handle: acpi_spinlock) { acpi_os_delete_lock(handle) }
#[inline] pub unsafe fn acpi_os_acquire_raw_lock(handle: acpi_spinlock) -> acpi_cpu_flags { acpi_os_acquire_lock(handle) }
#[inline] pub unsafe fn acpi_os_release_raw_lock(handle: acpi_spinlock, flags: acpi_cpu_flags) { acpi_os_release_lock(handle, flags) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
