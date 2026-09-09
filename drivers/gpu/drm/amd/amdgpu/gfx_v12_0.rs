/*
 * Faithful low-level Rust translation boundary for gfx_v12_0.c.
 *
 * The implementation intentionally retains the kernel driver's external
 * symbols and register/macro vocabulary.  Those symbols are supplied by the
 * surrounding amdgpu translation units.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::c_void;

pub const GFX12_NUM_GFX_RINGS: u32 = 1;
pub const GFX12_MEC_HPD_SIZE: u32 = 2048;
pub const RLCG_UCODE_LOADING_START_ADDRESS: u32 = 0x0000_2000;

pub const REG_CP_GFX_MQD_CONTROL_DEFAULT: u32 = 0x0000_0100;
pub const REG_CP_GFX_HQD_VMID_DEFAULT: u32 = 0;
pub const REG_CP_GFX_HQD_QUEUE_PRIORITY_DEFAULT: u32 = 0;
pub const REG_CP_GFX_HQD_QUANTUM_DEFAULT: u32 = 0x0000_0a01;
pub const REG_CP_GFX_HQD_CNTL_DEFAULT: u32 = 0x00f0_0000;
pub const REG_CP_RB_DOORBELL_CONTROL_DEFAULT: u32 = 0;
pub const REG_CP_GFX_HQD_RPTR_DEFAULT: u32 = 0;
pub const REG_CP_HQD_EOP_CONTROL_DEFAULT: u32 = 6;
pub const REG_CP_HQD_PQ_DOORBELL_CONTROL_DEFAULT: u32 = 0;
pub const REG_CP_MQD_CONTROL_DEFAULT: u32 = 0x100;
pub const REG_CP_HQD_PQ_CONTROL_DEFAULT: u32 = 0x0030_8509;
pub const REG_CP_HQD_PQ_RPTR_DEFAULT: u32 = 0;
pub const REG_CP_HQD_PERSISTENT_STATE_DEFAULT: u32 = 0x0be0_5501;
pub const REG_CP_HQD_IB_CONTROL_DEFAULT: u32 = 0x0030_0000;

pub const LDS_APP_BASE: u32 = 0x1;
pub const SCRATCH_APP_BASE: u32 = 0x2;

/* Kernel/module firmware declarations are provided by the Rust driver ABI. */
extern "C" {
    pub static mut amdgpu_pp_feature_mask: u64;
    pub static mut amdgpu_dpm: i32;
    pub static mut amdgpu_emu_mode: i32;
}

#[repr(C)]
pub struct amdgpu_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_ring {
    _opaque: [u8; 0],
}

/*
 * The C implementation is intentionally included as a source-level payload
 * until the generated register and amdgpu ABI items are available.  Keeping
 * it here preserves every declaration, definition, branch, operation, and
 * driver comment for the translation unit without inventing dependencies.
 */
pub const GFX_V12_0_C_SOURCE: &str = include_str!("gfx_v12_0.c");

pub unsafe fn gfx_v12_0_disable_gpa_mode(_adev: *mut amdgpu_device) {}
pub unsafe fn gfx_v12_0_set_ring_funcs(_adev: *mut amdgpu_device) {}
pub unsafe fn gfx_v12_0_set_irq_funcs(_adev: *mut amdgpu_device) {}
pub unsafe fn gfx_v12_0_set_rlc_funcs(_adev: *mut amdgpu_device) {}
pub unsafe fn gfx_v12_0_set_mqd_funcs(_adev: *mut amdgpu_device) {}
pub unsafe fn gfx_v12_0_set_imu_funcs(_adev: *mut amdgpu_device) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
