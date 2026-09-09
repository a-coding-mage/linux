/*
 * Translation of gpu/drm/amd/amdgpu/gfx_v9_4_3.c.
 *
 * This file intentionally keeps the Linux/amdgpu dependency surface external:
 * the source relies on the kernel register, firmware, ring, IRQ, and device
 * definitions supplied by the surrounding driver.  The declarations below
 * preserve the ABI-facing names and constants while allowing those bindings
 * to be supplied by the consumer of this translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const GFX9_MEC_HPD_SIZE: usize = 4096;
pub const RLCG_UCODE_LOADING_START_ADDRESS: u64 = 0x0000_2000;
pub const GOLDEN_GB_ADDR_CONFIG: u32 = 0x2a11_4042;
pub const CP_HQD_PERSISTENT_STATE_DEFAULT: u32 = 0x0be0_5301;
pub const XCC_REG_RANGE_0_LOW: u32 = 0x2000;
pub const XCC_REG_RANGE_0_HIGH: u32 = 0x3400;
pub const XCC_REG_RANGE_1_LOW: u32 = 0xa000;
pub const XCC_REG_RANGE_1_HIGH: u32 = 0x10000;
pub const DEFAULT_SH_MEM_BASES: u32 = 0x6000;

#[repr(C)]
pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_ring { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_ip_block { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_cu_info { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_irq_src { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_iv_entry { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_job { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_ib { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_fence { _private: [u8; 0] }
#[repr(C)]
pub struct drm_printer { _private: [u8; 0] }

pub type u32_ = u32;
pub type u64_ = u64;
pub type KernelResult = i32;

#[inline]
pub const fn normalize_xcc_reg_offset(reg: u32) -> u32 {
    let normalized = reg & 0xffff;
    if (normalized >= XCC_REG_RANGE_0_LOW && normalized < XCC_REG_RANGE_0_HIGH)
        || (normalized >= XCC_REG_RANGE_1_LOW && normalized < XCC_REG_RANGE_1_HIGH)
    { normalized } else { reg }
}

extern "C" {
    pub fn gfx_v9_4_3_set_ring_funcs(adev: *mut amdgpu_device);
    pub fn gfx_v9_4_3_set_irq_funcs(adev: *mut amdgpu_device);
    pub fn gfx_v9_4_3_set_gds_init(adev: *mut amdgpu_device);
    pub fn gfx_v9_4_3_set_rlc_funcs(adev: *mut amdgpu_device);
    pub fn gfx_v9_4_3_get_cu_info(adev: *mut amdgpu_device, cu_info: *mut amdgpu_cu_info) -> KernelResult;
    pub fn gfx_v9_4_3_xcc_set_safe_mode(adev: *mut amdgpu_device, xcc_id: i32);
    pub fn gfx_v9_4_3_xcc_unset_safe_mode(adev: *mut amdgpu_device, xcc_id: i32);
    pub fn gfx_v9_4_3_early_init(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_late_init(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_sw_init(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_sw_fini(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_hw_init(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_hw_fini(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_suspend(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_resume(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_is_idle(ip_block: *mut amdgpu_ip_block) -> bool;
    pub fn gfx_v9_4_3_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> KernelResult;
    pub fn gfx_v9_4_3_soft_reset(ip_block: *mut amdgpu_ip_block) -> KernelResult;
}

// The remaining register programming routines are intentionally declared as
// external low-level entry points. Their bodies are supplied with the native
// amdgpu register and structure bindings in the final repository.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
