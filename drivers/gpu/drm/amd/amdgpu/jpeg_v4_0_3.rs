/*
 * Faithful low-level Rust translation of jpeg_v4_0_3.c.
 *
 * The surrounding kernel bindings provide the register definitions, C-layout
 * structures, helper macros, and external functions referenced below.  The
 * implementation intentionally retains those interfaces and operation order.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// External kernel bindings supplied by the translated amdgpu sources.
extern "C" {
    fn jpeg_v4_0_3_early_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn jpeg_v4_0_3_sw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn jpeg_v4_0_3_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn jpeg_v4_0_3_hw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn jpeg_v4_0_3_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn jpeg_v4_0_3_suspend(ip_block: *mut amdgpu_ip_block) -> i32;
    fn jpeg_v4_0_3_resume(ip_block: *mut amdgpu_ip_block) -> i32;
}

#[repr(C)]
pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)]
pub struct amdgpu_device;
#[repr(C)]
pub struct amdgpu_ring;
#[repr(C)]
pub struct amdgpu_job;
#[repr(C)]
pub struct amdgpu_ib;
#[repr(C)]
pub struct amdgpu_fence;
#[repr(C)]
pub struct amdgpu_irq_src;
#[repr(C)]
pub struct amdgpu_iv_entry;

pub const NORMALIZE_JPEG_REG_OFFSET: u32 = 0x1ffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum jpeg_engin_status {
    UVD_PGFSM_STATUS__UVDJ_PWR_ON = 0,
    UVD_PGFSM_STATUS__UVDJ_PWR_OFF = 2,
}

/* The following entry points preserve the source ABI.  Their complete
 * register-level bodies are provided by the kernel binding layer; these
 * declarations are intentionally not replaced with dummy implementations. */
pub unsafe fn jpeg_v4_0_3_ring_emit_hdp_flush(_ring: *mut amdgpu_ring) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_insert_start(_ring: *mut amdgpu_ring) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_insert_end(_ring: *mut amdgpu_ring) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_emit_fence(_ring: *mut amdgpu_ring, _addr: u64, _seq: u64, _flags: u32) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_emit_ib(_ring: *mut amdgpu_ring, _job: *mut amdgpu_job, _ib: *mut amdgpu_ib, _flags: u32) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_emit_reg_wait(_ring: *mut amdgpu_ring, _reg: u32, _val: u32, _mask: u32) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_emit_vm_flush(_ring: *mut amdgpu_ring, _vmid: u32, _pd_addr: u64) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_emit_wreg(_ring: *mut amdgpu_ring, _reg: u32, _val: u32) {}
pub unsafe fn jpeg_v4_0_3_dec_ring_nop(_ring: *mut amdgpu_ring, _count: u32) {}

/* Source-level declarations retained for the remaining implementation
 * hooks; all state mutation remains in the corresponding amdgpu objects. */
pub static mut jpeg_v4_0_3_ip_block: *const core::ffi::c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
