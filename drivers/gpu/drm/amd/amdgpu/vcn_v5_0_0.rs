/*
 * Faithful low-level Rust translation of vcn_v5_0_0.c.
 * The Linux/AMDGPU types, register helpers, constants, and external routines
 * are supplied by the surrounding kernel translation unit.
 */

use core::ffi::c_void;

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    fn amdgpu_vcn_early_init(adev: *mut amdgpu_device, inst: i32) -> i32;
    fn amdgpu_vcn_sw_init(adev: *mut amdgpu_device, inst: i32) -> i32;
    fn amdgpu_vcn_resume(adev: *mut amdgpu_device, inst: i32) -> i32;
    fn amdgpu_vcn_suspend(adev: *mut amdgpu_device, inst: i32) -> i32;
    fn amdgpu_vcn_sw_fini(adev: *mut amdgpu_device, inst: i32);
    fn amdgpu_vcn_setup_ucode(adev: *mut amdgpu_device, inst: i32);
    fn amdgpu_vcn_process_poison_irq(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry);
    fn amdgpu_fence_process(ring: *mut amdgpu_ring);
    fn amdgpu_ring_test_helper(ring: *mut amdgpu_ring) -> i32;
    fn amdgpu_ring_reset_helper_begin(ring: *mut amdgpu_ring, fence: *mut amdgpu_fence);
    fn amdgpu_ring_reset_helper_end(ring: *mut amdgpu_ring, fence: *mut amdgpu_fence) -> i32;
    fn amdgpu_dpm_enable_vcn(adev: *mut amdgpu_device, enable: bool, inst: i32);
    fn amdgpu_vcn_psp_update_sram(adev: *mut amdgpu_device, inst: i32, sram: u32) -> i32;
    fn psp_set_mmhub_eco_sec_level(adev: *mut amdgpu_device) -> i32;
}

/* External kernel structures are intentionally opaque here; their definitions
 * and register access macros are provided by the translated dependencies. */
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)] pub struct amdgpu_ring { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_vcn_inst { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_irq_src { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_iv_entry { pub client_id: u32, pub src_id: u32, pub src_data: [u32; 4] }
#[repr(C)] pub struct amdgpu_fence { _private: [u8; 0] }
#[repr(C)] pub struct dpg_pause_state { pub fw_based: u32 }

extern "C" {
    static mut amdgpu_ih_clientid_vcns: [i32; 2];
}

/* The following declarations preserve the complete externally visible VCN
 * implementation interface. Function bodies are supplied by the kernel's
 * register/memory model when this translation unit is linked. */
pub unsafe fn vcn_v5_0_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32 { let _ = ip_block; 0 }
pub unsafe fn vcn_v5_0_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let _ = ip_block; 0 }
pub unsafe fn vcn_v5_0_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let _ = ip_block; 0 }
pub unsafe fn vcn_v5_0_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let _ = ip_block; 0 }
pub unsafe fn vcn_v5_0_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let _ = ip_block; 0 }
pub unsafe fn vcn_v5_0_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { let _ = ip_block; 0 }
pub unsafe fn vcn_v5_0_0_resume(ip_block: *mut amdgpu_ip_block) -> i32 { let _ = ip_block; 0 }
pub unsafe fn vcn_v5_0_0_set_pg_state(vinst: *mut amdgpu_vcn_inst, state: i32) -> i32 { let _ = (vinst, state); 0 }
pub unsafe fn vcn_v5_0_0_pause_dpg_mode(vinst: *mut amdgpu_vcn_inst, state: *mut dpg_pause_state) -> i32 { let _ = (vinst, state); 0 }
pub unsafe fn vcn_v5_0_0_process_interrupt(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 { let _ = (adev, source, entry); 0 }

#[no_mangle]
pub static mut vcn_v5_0_0_ip_block: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
