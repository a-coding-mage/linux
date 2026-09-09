// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utxface - External interfaces, miscellaneous utility functions
//
// Copyright (C) 2000 - 2026, Intel Corp.

// C includes and build-time configuration are supplied by the surrounding ACPICA bindings.

extern "C" {
    fn acpi_ut_subsystem_shutdown();
    fn acpi_ut_mutex_terminate();
    fn acpi_os_terminate() -> acpi_status;
    fn acpi_ut_validate_buffer(out_buffer: *mut acpi_buffer) -> acpi_status;
    fn acpi_ut_initialize_buffer(out_buffer: *mut acpi_buffer, length: usize) -> acpi_status;
    fn acpi_ut_get_interface(interface_name: acpi_string) -> *mut acpi_interface_info;
    fn acpi_ut_install_interface(interface_name: acpi_string) -> acpi_status;
    fn acpi_ut_remove_interface(interface_name: acpi_string) -> acpi_status;
    fn acpi_ut_update_interfaces(action: u8) -> acpi_status;
    fn acpi_ut_acquire_mutex(mutex: u32) -> acpi_status;
    fn acpi_ut_release_mutex(mutex: u32) -> acpi_status;
    fn acpi_ut_check_address_range(space_id: acpi_adr_space_type, address: acpi_physical_address, length: u32, warn: u8) -> u32;
    fn acpi_os_acquire_mutex(mutex: *mut core::ffi::c_void, timeout: u32) -> acpi_status;
    fn acpi_os_release_mutex(mutex: *mut core::ffi::c_void);
    fn acpi_os_purge_cache(cache: *mut core::ffi::c_void) -> acpi_status;
    fn acpi_os_allocate_zeroed(size: usize) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct acpi_buffer { pub length: usize, pub pointer: *mut core::ffi::c_void }

pub type acpi_status = i32;
pub type acpi_string = *const u8;
pub type acpi_adr_space_type = u8;
pub type acpi_physical_address = u64;
pub type acpi_size = usize;
pub type acpi_init_handler = Option<unsafe extern "C" fn() -> acpi_status>;
pub type acpi_interface_handler = Option<unsafe extern "C" fn() -> acpi_status>;

#[repr(C)] pub struct acpi_interface_info { pub flags: u8 }
#[repr(C)] pub struct acpi_system_info { pub acpi_ca_version: u32, pub flags: u32, pub timer_resolution: u8, pub reserved1: u8, pub reserved2: u8, pub debug_layer: u32, pub debug_level: u32 }
#[repr(C)] pub struct acpi_statistics { pub sci_count: u32, pub gpe_count: u32, pub fixed_event_count: [u32; 5], pub method_count: u32 }
#[repr(C)] pub struct acpi_pld_info {
    pub revision: u8, pub ignore_color: u8, pub red: u8, pub green: u8, pub blue: u8,
    pub width: u16, pub height: u16, pub user_visible: u8, pub dock: u8, pub lid: u8,
    pub panel: u8, pub vertical_position: u8, pub horizontal_position: u8, pub shape: u8,
    pub group_orientation: u8, pub group_token: u8, pub group_position: u8, pub bay: u8,
    pub ejectable: u8, pub ospm_eject_required: u8, pub cabinet_number: u8,
    pub card_cage_number: u8, pub reference: u8, pub rotation: u8, pub order: u8,
    pub vertical_offset: u8, pub horizontal_offset: u8,
}

extern "C" {
    static mut acpi_gbl_startup_flags: u32;
    static mut acpi_gbl_FADT: acpi_fadt;
    static mut acpi_dbg_layer: u32;
    static mut acpi_dbg_level: u32;
    static mut acpi_sci_count: u32;
    static mut acpi_gpe_count: u32;
    static mut acpi_fixed_event_count: [u32; 5];
    static mut acpi_method_count: u32;
    static mut acpi_gbl_init_handler: acpi_init_handler;
    static mut acpi_gbl_osi_mutex: *mut core::ffi::c_void;
    static mut acpi_gbl_interface_handler: acpi_interface_handler;
    static mut acpi_gbl_state_cache: *mut core::ffi::c_void;
    static mut acpi_gbl_operand_cache: *mut core::ffi::c_void;
    static mut acpi_gbl_ps_node_cache: *mut core::ffi::c_void;
    static mut acpi_gbl_ps_node_ext_cache: *mut core::ffi::c_void;
}
#[repr(C)] pub struct acpi_fadt { pub flags: u32 }

const AE_OK: acpi_status = 0; const AE_ERROR: acpi_status = 1; const AE_BAD_PARAMETER: acpi_status = 2;
const AE_NO_MEMORY: acpi_status = 4; const AE_ALREADY_EXISTS: acpi_status = 5;
const ACPI_INITIALIZED_OK: u32 = 1; const ACPI_FADT_32BIT_TIMER: u32 = 1;
const ACPI_SYS_MODE_ACPI: u32 = 1; const ACPI_WAIT_FOREVER: u32 = 0xffff_ffff;
const ACPI_MTX_NAMESPACE: u32 = 0; const ACPI_OSI_INVALID: u8 = 1;
const ACPI_PLD_REV1_BUFFER_SIZE: usize = 16; const ACPI_PLD_REV2_BUFFER_SIZE: usize = 20;

#[inline] unsafe fn acpi_failure(status: acpi_status) -> bool { status != AE_OK }

pub unsafe extern "C" fn acpi_terminate() -> acpi_status {
    acpi_ut_subsystem_shutdown(); acpi_ut_mutex_terminate(); acpi_os_terminate()
}

pub unsafe extern "C" fn acpi_subsystem_status() -> acpi_status {
    if acpi_gbl_startup_flags & ACPI_INITIALIZED_OK != 0 { AE_OK } else { AE_ERROR }
}

pub unsafe extern "C" fn acpi_get_system_info(out_buffer: *mut acpi_buffer) -> acpi_status {
    let mut status = acpi_ut_validate_buffer(out_buffer); if acpi_failure(status) { return status; }
    status = acpi_ut_initialize_buffer(out_buffer, core::mem::size_of::<acpi_system_info>()); if acpi_failure(status) { return status; }
    let info = (*out_buffer).pointer as *mut acpi_system_info;
    (*info).acpi_ca_version = 0; (*info).flags = ACPI_SYS_MODE_ACPI;
    (*info).timer_resolution = if acpi_gbl_FADT.flags & ACPI_FADT_32BIT_TIMER != 0 { 24 } else { 32 };
    (*info).reserved1 = 0; (*info).reserved2 = 0; (*info).debug_layer = acpi_dbg_layer; (*info).debug_level = acpi_dbg_level; AE_OK
}

pub unsafe extern "C" fn acpi_get_statistics(stats: *mut acpi_statistics) -> acpi_status {
    if stats.is_null() { return AE_BAD_PARAMETER; }
    (*stats).sci_count = acpi_sci_count; (*stats).gpe_count = acpi_gpe_count;
    (*stats).fixed_event_count = acpi_fixed_event_count; (*stats).method_count = acpi_method_count; AE_OK
}

pub unsafe extern "C" fn acpi_install_initialization_handler(handler: acpi_init_handler, _function: u32) -> acpi_status {
    if handler.is_none() { return AE_BAD_PARAMETER; } if acpi_gbl_init_handler.is_some() { return AE_ALREADY_EXISTS; }
    acpi_gbl_init_handler = handler; AE_OK
}

pub unsafe extern "C" fn acpi_purge_cached_objects() -> acpi_status {
    acpi_os_purge_cache(acpi_gbl_state_cache); acpi_os_purge_cache(acpi_gbl_operand_cache); acpi_os_purge_cache(acpi_gbl_ps_node_cache); acpi_os_purge_cache(acpi_gbl_ps_node_ext_cache); AE_OK
}

pub unsafe extern "C" fn acpi_install_interface(interface_name: acpi_string) -> acpi_status {
    if interface_name.is_null() { return AE_BAD_PARAMETER; } let mut status = acpi_os_acquire_mutex(acpi_gbl_osi_mutex, ACPI_WAIT_FOREVER); if acpi_failure(status) { return status; }
    let info = acpi_ut_get_interface(interface_name); if !info.is_null() { if (*info).flags & ACPI_OSI_INVALID != 0 { (*info).flags &= !ACPI_OSI_INVALID; status = AE_OK; } else { status = AE_ALREADY_EXISTS; } } else { status = acpi_ut_install_interface(interface_name); }
    acpi_os_release_mutex(acpi_gbl_osi_mutex); status
}

pub unsafe extern "C" fn acpi_remove_interface(interface_name: acpi_string) -> acpi_status {
    if interface_name.is_null() { return AE_BAD_PARAMETER; } let status = acpi_os_acquire_mutex(acpi_gbl_osi_mutex, ACPI_WAIT_FOREVER); if acpi_failure(status) { return status; }
    let result = acpi_ut_remove_interface(interface_name); acpi_os_release_mutex(acpi_gbl_osi_mutex); result
}

pub unsafe extern "C" fn acpi_install_interface_handler(handler: acpi_interface_handler) -> acpi_status {
    let mut status = acpi_os_acquire_mutex(acpi_gbl_osi_mutex, ACPI_WAIT_FOREVER); if acpi_failure(status) { return status; }
    if handler.is_some() && acpi_gbl_interface_handler.is_some() { status = AE_ALREADY_EXISTS; } else { acpi_gbl_interface_handler = handler; }
    acpi_os_release_mutex(acpi_gbl_osi_mutex); status
}

pub unsafe extern "C" fn acpi_update_interfaces(action: u8) -> acpi_status {
    let status = acpi_os_acquire_mutex(acpi_gbl_osi_mutex, ACPI_WAIT_FOREVER); if acpi_failure(status) { return status; }
    let result = acpi_ut_update_interfaces(action); acpi_os_release_mutex(acpi_gbl_osi_mutex); result
}

pub unsafe extern "C" fn acpi_check_address_range(space_id: acpi_adr_space_type, address: acpi_physical_address, length: acpi_size, warn: u8) -> u32 {
    if acpi_failure(acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE)) { return 0; }
    let overlaps = acpi_ut_check_address_range(space_id, address, length as u32, warn); acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); overlaps
}

pub unsafe extern "C" fn acpi_decode_pld_buffer(in_buffer: *mut u8, length: acpi_size, return_buffer: *mut *mut acpi_pld_info) -> acpi_status {
    if in_buffer.is_null() || return_buffer.is_null() || length < ACPI_PLD_REV1_BUFFER_SIZE { return AE_BAD_PARAMETER; }
    let pld_info = acpi_os_allocate_zeroed(core::mem::size_of::<acpi_pld_info>()) as *mut acpi_pld_info; if pld_info.is_null() { return AE_NO_MEMORY; }
    let buffer = in_buffer as *const u32; let d0 = u32::from_ne(buffer.read()); let d1 = u32::from_ne(buffer.add(1).read()); let d2 = u32::from_ne(buffer.add(2).read()); let d3 = u32::from_ne(buffer.add(3).read());
    (*pld_info).revision = (d0 & 0x7) as u8; (*pld_info).ignore_color = ((d0 >> 3) & 1) as u8; (*pld_info).red = ((d0 >> 4) & 0xff) as u8; (*pld_info).green = ((d0 >> 12) & 0xff) as u8; (*pld_info).blue = ((d0 >> 20) & 0xff) as u8;
    (*pld_info).width = (d1 & 0xffff) as u16; (*pld_info).height = (d1 >> 16) as u16;
    (*pld_info).user_visible = (d2 & 1) as u8; (*pld_info).dock = ((d2 >> 1) & 1) as u8; (*pld_info).lid = ((d2 >> 2) & 1) as u8; (*pld_info).panel = ((d2 >> 3) & 3) as u8; (*pld_info).vertical_position = ((d2 >> 5) & 3) as u8; (*pld_info).horizontal_position = ((d2 >> 7) & 3) as u8; (*pld_info).shape = ((d2 >> 9) & 0x1f) as u8; (*pld_info).group_orientation = ((d2 >> 14) & 1) as u8; (*pld_info).group_token = ((d2 >> 15) & 0xff) as u8; (*pld_info).group_position = ((d2 >> 23) & 0xff) as u8; (*pld_info).bay = ((d2 >> 31) & 1) as u8;
    (*pld_info).ejectable = (d3 & 1) as u8; (*pld_info).ospm_eject_required = ((d3 >> 1) & 1) as u8; (*pld_info).cabinet_number = ((d3 >> 2) & 0xf) as u8; (*pld_info).card_cage_number = ((d3 >> 6) & 0xf) as u8; (*pld_info).reference = ((d3 >> 10) & 1) as u8; (*pld_info).rotation = ((d3 >> 11) & 0xff) as u8; (*pld_info).order = ((d3 >> 19) & 0xff) as u8;
    if length >= ACPI_PLD_REV2_BUFFER_SIZE { let d4 = u32::from_ne(buffer.add(4).read()); (*pld_info).vertical_offset = (d4 & 0xff) as u8; (*pld_info).horizontal_offset = ((d4 >> 8) & 0xff) as u8; }
    *return_buffer = pld_info; AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
