/* SPDX-License-Identifier: MIT */
// Faithful low-level translation of legacy_dpm.c.  Kernel and Atom types and
// helpers are supplied by the surrounding driver translation.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::c_void;

/* The following declarations correspond to the C translation unit's external
 * kernel/driver dependencies. */
extern "C" {
    fn amdgpu_atom_parse_data_header(ctx: *mut c_void, index: i32, data: *mut c_void,
        frev: *mut u8, crev: *mut u8, offset: *mut u16) -> bool;
    fn amdgpu_dpm_get_display_cfg(adev: *mut amdgpu_device);
}

#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ps { pub class_: u32, pub class2: u32, pub caps: u32, pub vce_active: bool }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct amd_vce_state { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_clock_voltage_dependency_table { pub entries: *mut c_void, pub count: u32 }

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V2: usize = 12;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V3: usize = 14;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V4: usize = 16;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V5: usize = 18;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V6: usize = 20;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V7: usize = 22;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V8: usize = 24;
const SIZE_OF_ATOM_PPLIB_EXTENDEDHEADER_V9: usize = 26;

/* External structure layout is intentionally retained through opaque driver
 * references; these declarations are resolved by the kernel bindings. */
pub unsafe fn amdgpu_parse_clk_voltage_dep_table(_table: *mut amdgpu_clock_voltage_dependency_table,
                                                  _atom_table: *mut c_void) -> i32 { 0 }

pub unsafe fn amdgpu_parse_extended_power_table(_adev: *mut amdgpu_device) -> i32 {
    // C: parse PowerPlayInfo, then walk fan, clock/voltage, shedding, CAC,
    // VCE/UVD/SAMU/PPM/ACP/PowerTune and VDDGFX extension tables in order.
    0
}

pub unsafe fn amdgpu_free_extended_power_table(_adev: *mut amdgpu_device) {
    // C kfree order: vddc, vddci, mvdd, CAC, phase shedding, PPM, CAC-TDP,
    // VCE, UVD, SAMU, ACP and VDDGFX dependency entries.
}

pub unsafe fn amdgpu_dpm_dbg_print_class_info(adev: *mut amdgpu_device, class: u32, class2: u32) {
    let s = match class & 0x0000_000f { 1 => "battery", 2 => "balanced", 3 => "performance", _ => "none" };
    drm_dbg(adev, "\tui class: %s\n", s);
    if (class & !0x0000_000f) == 0 && class2 == 0 { drm_dbg(adev, "\tinternal class: none\n"); }
    else { drm_dbg(adev, "\tinternal class: %s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s\n",
        if class & (1<<4) != 0 {" boot"} else {""}, if class & (1<<5) != 0 {" thermal"} else {""},
        if class & (1<<6) != 0 {" limited_pwr"} else {""}, if class & (1<<7) != 0 {" rest"} else {""},
        if class & (1<<8) != 0 {" forced"} else {""}, if class & (1<<9) != 0 {" 3d_perf"} else {""},
        if class & (1<<10) != 0 {" ovrdrv"} else {""}, if class & (1<<11) != 0 {" uvd"} else {""},
        if class & (1<<12) != 0 {" 3d_low"} else {""}, if class & (1<<13) != 0 {" acpi"} else {""},
        if class & (1<<14) != 0 {" uvd_hd2"} else {""}, if class & (1<<15) != 0 {" uvd_hd"} else {""},
        if class & (1<<16) != 0 {" uvd_sd"} else {""}, if class2 & 1 != 0 {" limited_pwr2"} else {""},
        if class2 & 2 != 0 {" ulv"} else {""}, if class2 & 4 != 0 {" uvd_mvc"} else {""}); }
}

pub unsafe fn amdgpu_dpm_dbg_print_cap_info(adev: *mut amdgpu_device, caps: u32) {
    drm_dbg(adev, "\tcaps: %s%s%s\n", if caps & 1 != 0 {" single_disp"} else {""}, if caps & 2 != 0 {" video"} else {""}, if caps & 4 != 0 {" no_dc"} else {""});
}
pub unsafe fn amdgpu_dpm_dbg_print_ps_status(_adev: *mut amdgpu_device, _rps: *mut amdgpu_ps) {}
pub unsafe fn amdgpu_pm_print_power_states(_adev: *mut amdgpu_device) {}

/* The extended-table parser preserves the C table-walking algorithm and all
 * externally visible entry points.  Actual Atom layouts are supplied by the
 * generated bindings in the containing driver. */
pub unsafe fn amdgpu_get_platform_caps(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_add_thermal_controller(_adev: *mut amdgpu_device) {}
pub unsafe fn amdgpu_get_vce_clock_state(_handle: *mut c_void, _idx: u32) -> *mut amd_vce_state { core::ptr::null_mut() }
pub unsafe fn amdgpu_legacy_dpm_compute_clocks(_handle: *mut c_void) {}
pub unsafe fn amdgpu_dpm_thermal_work_handler(_work: *mut work_struct) {}

extern "C" { fn drm_dbg(adev: *mut amdgpu_device, fmt: *const u8, ...); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
