/*
 * Faithful low-level Rust interface translation of amd_powerplay.c.
 * Definitions supplied by the kernel headers remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

pub type size_t = usize;
pub type ssize_t = isize;

#[repr(C)] pub struct amdgpu_device { _opaque: [u8; 0] }
#[repr(C)] pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)] pub struct pp_hwmgr { _opaque: [u8; 0] }
#[repr(C)] pub struct amd_pm_funcs { _opaque: [u8; 0] }
#[repr(C)] pub struct amd_ip_funcs { _opaque: [u8; 0] }
#[repr(C)] pub struct amdgpu_ip_block_version { _opaque: [u8; 0] }

extern "C" {
    static pp_dpm_funcs: amd_pm_funcs;
    fn amd_powerplay_create(adev: *mut amdgpu_device) -> c_int;
    fn amd_powerplay_destroy(adev: *mut amdgpu_device);
    fn pp_early_init(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_sw_init(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_sw_fini(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_hw_init(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_hw_fini(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_late_init(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_late_fini(ip_block: *mut amdgpu_ip_block);
    fn pp_suspend(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_resume(ip_block: *mut amdgpu_ip_block) -> c_int;
    fn pp_is_idle(ip_block: *mut amdgpu_ip_block) -> bool;
    fn pp_set_clockgating_state(ip_block: *mut amdgpu_ip_block, state: c_int) -> c_int;
    fn pp_set_powergating_state(ip_block: *mut amdgpu_ip_block, state: c_int) -> c_int;
    fn pp_dpm_load_fw(handle: *mut c_void) -> c_int;
    fn pp_dpm_fw_loading_complete(handle: *mut c_void) -> c_int;
    fn pp_dpm_force_performance_level(handle: *mut c_void, level: c_int) -> c_int;
    fn pp_dpm_get_performance_level(handle: *mut c_void) -> c_int;
    fn pp_dpm_dispatch_tasks(handle: *mut c_void, task_id: c_int, state: *mut c_void) -> c_int;
    fn pp_dpm_get_current_power_state(handle: *mut c_void) -> c_int;
    fn pp_dpm_set_pp_table(handle: *mut c_void, buf: *const c_char, size: size_t) -> c_int;
    fn pp_dpm_read_sensor(handle: *mut c_void, idx: c_int, value: *mut c_void, size: *mut c_int) -> c_int;
    fn pp_dpm_get_sclk(handle: *mut c_void, low: bool) -> u32;
    fn pp_dpm_get_mclk(handle: *mut c_void, low: bool) -> u32;
    fn pp_dpm_powergate_vce(handle: *mut c_void, gate: bool);
    fn pp_dpm_powergate_uvd(handle: *mut c_void, gate: bool);
    fn pp_dpm_switch_power_profile(handle: *mut c_void, profile: c_int, enable: bool) -> c_int;
}

pub static mut pp_smu_ip_block: *const amd_ip_funcs = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
