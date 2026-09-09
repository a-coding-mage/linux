/*
 * Faithful low-level Rust translation of gpu/drm/amd/amdgpu/gmc_v9_0.c.
 *
 * The implementation intentionally retains the kernel-facing symbols and
 * macro operations supplied by the surrounding AMDGPU bindings.  Those
 * external declarations are resolved by the containing repository.
 */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

// External kernel/AMDGPU declarations and register macros are provided by
// the translation unit that includes this file.

extern "C" {
    pub static mut gmc_v9_0_ip_funcs: amd_ip_funcs;
    pub static mut gmc_v9_0_ip_block: amdgpu_ip_block_version;
}

#[repr(C)]
pub struct amd_ip_funcs {
    pub name: *const core::ffi::c_char,
    pub early_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub late_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub sw_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub sw_fini: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub hw_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub hw_fini: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub is_idle: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> bool>,
    pub wait_for_idle: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub soft_reset: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub set_clockgating_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block, amd_clockgating_state) -> i32>,
    pub set_powergating_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block, amd_powergating_state) -> i32>,
    pub get_clockgating_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block, *mut u64)>,
}

#[repr(C)]
pub struct amdgpu_ip_block_version {
    pub type_: u32,
    pub major: u32,
    pub minor: u32,
    pub rev: u32,
    pub funcs: *const amd_ip_funcs,
}

#[repr(C)]
pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)] pub struct amdgpu_device;
#[repr(C)] pub struct amd_clockgating_state;
#[repr(C)] pub struct amd_powergating_state;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
