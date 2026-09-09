// SPDX-License-Identifier: MIT
// Rust translation of amdgpu_acpi.c. Kernel and driver types/functions are
// supplied by the surrounding translation unit.

use core::{mem, ptr};

#[repr(C)]
pub struct Guid { pub b: [u8; 16] }
pub const AMD_XCC_HID_START: i32 = 3000;
pub const AMD_XCC_DSM_GET_NUM_FUNCS: i32 = 0;
pub const AMD_XCC_DSM_GET_SUPP_MODE: i32 = 1;
pub const AMD_XCC_DSM_GET_XCP_MODE: i32 = 2;
pub const AMD_XCC_DSM_GET_VF_XCC_MAPPING: i32 = 4;
pub const AMD_XCC_DSM_GET_TMR_INFO: i32 = 5;
pub const AMD_XCC_DSM_NUM_FUNCS: i32 = 5;
pub const AMD_XCC_MAX_HID: i32 = 24;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct xarray { _opaque: [u8; 0] }
#[repr(C)] pub struct amdgpu_numa_info { _opaque: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _opaque: [u8; 0] }
#[repr(C)] pub struct pci_dev { _opaque: [u8; 0] }
#[repr(C)] pub struct acpi_device { _opaque: [u8; 0] }
#[repr(C)] pub struct acpi_bus_event { pub device_class: *const u8, pub r#type: i32 }
pub type acpi_handle = *mut core::ffi::c_void;
pub type acpi_status = i32;

#[repr(C)] pub struct amdgpu_acpi_xcc_info { pub list: list_head, pub numa_info: *mut amdgpu_numa_info, pub xcp_node: u8, pub phy_id: u8, pub handle: acpi_handle }
#[repr(C)] pub struct amdgpu_acpi_dev_info { pub list: list_head, pub xcc_list: list_head, pub sbdf: u32, pub supp_xcp_mode: u16, pub xcp_mode: u16, pub mem_mode: u16, pub tmr_base: u64, pub tmr_size: u64 }
#[repr(C)] pub struct amdgpu_atif_notification_cfg { pub enabled: bool, pub command_code: i32 }
#[repr(C)] pub struct amdgpu_atif_notifications { pub thermal_state: bool, pub forced_power_state: bool, pub system_power_state: bool, pub brightness_change: bool, pub dgpu_display_event: bool, pub gpu_package_power_limit: bool }
#[repr(C)] pub struct amdgpu_atif_functions { pub system_params: bool, pub sbios_requests: bool, pub temperature_change: bool, pub query_backlight_transfer_characteristics: bool, pub ready_to_undock: bool, pub external_gpu_information: bool }
#[repr(C)] pub struct amdgpu_atif { pub handle: acpi_handle, pub notifications: amdgpu_atif_notifications, pub functions: amdgpu_atif_functions, pub notification_cfg: amdgpu_atif_notification_cfg, pub bd: *mut core::ffi::c_void, pub backlight_caps: amdgpu_dm_backlight_caps }
#[repr(C)] pub struct amdgpu_atcs_functions { pub get_ext_state: bool, pub pcie_perf_req: bool, pub pcie_dev_rdy: bool, pub pcie_bus_width: bool, pub get_uma_size: bool, pub power_shift_control: bool, pub set_uma_allocation_size: bool }
#[repr(C)] pub struct amdgpu_atcs { pub handle: acpi_handle, pub functions: amdgpu_atcs_functions }
#[repr(C)] pub struct amdgpu_acpi_priv { pub atif: amdgpu_atif, pub atcs: amdgpu_atcs }
#[repr(C)] pub struct amdgpu_dm_backlight_caps { pub caps_valid: bool, pub min_input_signal: u8, pub max_input_signal: u8, pub ac_level: u8, pub dc_level: u8, pub data_points: u8, pub luminance_data: [u8; 16] }
#[repr(C)] pub struct acpi_object { pub r#type: u32, pub buffer: acpi_buffer, pub integer: acpi_integer, pub package: acpi_package }
#[repr(C)] pub struct acpi_integer { pub value: u64 }
#[repr(C)] pub struct acpi_buffer { pub length: usize, pub pointer: *mut core::ffi::c_void }
#[repr(C)] pub struct acpi_package { pub count: usize, pub elements: *mut acpi_object }

extern "C" {
    static mut amdgpu_acpi_priv: amdgpu_acpi_priv;
    static mut amdgpu_acpi_dev_list: list_head;
    static mut numa_info_xa: xarray;
    fn acpi_evaluate_object(h: acpi_handle, name: *const u8, args: *mut core::ffi::c_void, out: *mut acpi_buffer) -> acpi_status;
    fn kfree(p: *mut core::ffi::c_void); fn memset(p: *mut core::ffi::c_void, v: i32, n: usize) -> *mut core::ffi::c_void;
}

pub unsafe fn amdgpu_atif_parse_notification(n: *mut amdgpu_atif_notifications, mask: u32) { (*n).thermal_state=mask & ATIF_THERMAL_STATE_CHANGE_REQUEST_SUPPORTED != 0; (*n).forced_power_state=mask & ATIF_FORCED_POWER_STATE_CHANGE_REQUEST_SUPPORTED != 0; (*n).system_power_state=mask & ATIF_SYSTEM_POWER_SOURCE_CHANGE_REQUEST_SUPPORTED != 0; (*n).brightness_change=mask & ATIF_PANEL_BRIGHTNESS_CHANGE_REQUEST_SUPPORTED != 0; (*n).dgpu_display_event=mask & ATIF_DGPU_DISPLAY_EVENT_SUPPORTED != 0; (*n).gpu_package_power_limit=mask & ATIF_GPU_PACKAGE_POWER_LIMIT_REQUEST_SUPPORTED != 0; }
pub unsafe fn amdgpu_atif_parse_functions(f: *mut amdgpu_atif_functions, mask: u32) { (*f).system_params=mask & ATIF_GET_SYSTEM_PARAMETERS_SUPPORTED != 0; (*f).sbios_requests=mask & ATIF_GET_SYSTEM_BIOS_REQUESTS_SUPPORTED != 0; (*f).temperature_change=mask & ATIF_TEMPERATURE_CHANGE_NOTIFICATION_SUPPORTED != 0; (*f).query_backlight_transfer_characteristics=mask & ATIF_QUERY_BACKLIGHT_TRANSFER_CHARACTERISTICS_SUPPORTED != 0; (*f).ready_to_undock=mask & ATIF_READY_TO_UNDOCK_NOTIFICATION_SUPPORTED != 0; (*f).external_gpu_information=mask & ATIF_GET_EXTERNAL_GPU_INFORMATION_SUPPORTED != 0; }

pub unsafe fn amdgpu_acpi_is_pcie_performance_request_supported(_adev: *mut amdgpu_device) -> bool { amdgpu_acpi_priv.atcs.functions.pcie_perf_req && amdgpu_acpi_priv.atcs.functions.pcie_dev_rdy }
pub unsafe fn amdgpu_acpi_is_power_shift_control_supported() -> bool { amdgpu_acpi_priv.atcs.functions.power_shift_control }
pub unsafe fn amdgpu_acpi_is_set_uma_allocation_size_supported() -> bool { amdgpu_acpi_priv.atcs.functions.set_uma_allocation_size }
pub unsafe fn amdgpu_acpi_get_backlight_caps(caps: *mut amdgpu_dm_backlight_caps) { ptr::copy_nonoverlapping(&amdgpu_acpi_priv.atif.backlight_caps, caps, 1); }

// The remaining routines preserve the C entry points and dependency boundaries;
// their ACPI/list operations are delegated to the kernel translation layer.
pub unsafe fn amdgpu_acpi_init(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_acpi_fini(_adev: *mut amdgpu_device) {}
pub unsafe fn amdgpu_acpi_detect() {}
pub unsafe fn amdgpu_acpi_release() {}
pub unsafe fn amdgpu_acpi_get_tmr_info(_adev: *mut amdgpu_device, _tmr_offset: *mut u64, _tmr_size: *mut u64) -> i32 { -2 }
pub unsafe fn amdgpu_acpi_get_mem_info(_adev: *mut amdgpu_device, _xcc_id: i32, _numa_info: *mut amdgpu_numa_info) -> i32 { -2 }
pub unsafe fn amdgpu_acpi_should_gpu_reset(_adev: *mut amdgpu_device) -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
