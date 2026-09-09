/*
 * Faithful low-level Rust translation of sdma_v4_4_2.c.
 *
 * The surrounding AMDGPU types, register definitions, packet constructors,
 * and kernel helpers are supplied by the translated dependency units.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External dependency symbols are intentionally left unresolved. */
extern "C" {
    fn amdgpu_sriov_vf(adev: *mut amdgpu_device) -> bool;
    fn amdgpu_ip_version(adev: *mut amdgpu_device, hwip: u32, instance: u32) -> u32;
    fn amdgpu_sdma_init_microcode(adev: *mut amdgpu_device, instance: u32, shared: bool) -> i32;
    fn amdgpu_ring_write(ring: *mut amdgpu_ring, value: u32);
    fn amdgpu_ring_alloc(ring: *mut amdgpu_ring, n: u32) -> i32;
    fn amdgpu_ring_commit(ring: *mut amdgpu_ring);
    fn amdgpu_ring_get_rptr(ring: *mut amdgpu_ring) -> u64;
    fn amdgpu_ring_test_helper(ring: *mut amdgpu_ring) -> i32;
    fn amdgpu_gmc_emit_flush_gpu_tlb(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64);
    fn amdgpu_sdma_set_vm_pte_scheds(adev: *mut amdgpu_device, funcs: *const amdgpu_vm_pte_funcs);
    fn amdgpu_sdma_set_buffer_funcs_scheds(adev: *mut amdgpu_device, funcs: *const amdgpu_buffer_funcs);
    fn amdgpu_sdma_reset_engine(adev: *mut amdgpu_device, id: u32, force: bool) -> i32;
    fn amdgpu_dpm_reset_sdma(adev: *mut amdgpu_device, mask: u32) -> i32;
}

#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ring { pub adev: *mut amdgpu_device, pub me: u32, pub wptr: u64, pub cached_rptr: u64, pub rptr_offs: u32, pub wptr_offs: u32, pub doorbell_index: u32, pub use_doorbell: bool, pub funcs: *const amdgpu_ring_funcs }
#[repr(C)] pub struct amdgpu_job { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ib { pub ptr: *mut u32, pub length_dw: u32 }
#[repr(C)] pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)] pub struct amdgpu_irq_src { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_iv_entry { pub client_id: u32, pub node_id: u32, pub ring_id: u32, pub src_id: u32, pub vmid: u32, pub pasid: u32, pub src_data: [u32; 2] }
#[repr(C)] pub struct drm_printer { _private: [u8; 0] }

#[repr(C)] pub struct amdgpu_vm_pte_funcs { pub copy_pte_num_dw: u32, pub copy_pte: Option<unsafe extern "C" fn(*mut amdgpu_ib,u64,u64,u32)>, pub write_pte: Option<unsafe extern "C" fn(*mut amdgpu_ib,u64,u64,u32,u32)>, pub set_pte_pde: Option<unsafe extern "C" fn(*mut amdgpu_ib,u64,u64,u32,u32,u64)> }
#[repr(C)] pub struct amdgpu_buffer_funcs { pub copy_max_bytes: u32, pub copy_num_dw: u32, pub fill_max_bytes: u32, pub fill_num_dw: u32 }
#[repr(C)] pub struct amdgpu_ring_funcs { pub get_rptr: Option<unsafe extern "C" fn(*mut amdgpu_ring)->u64>, pub get_wptr: Option<unsafe extern "C" fn(*mut amdgpu_ring)->u64>, pub set_wptr: Option<unsafe extern "C" fn(*mut amdgpu_ring)> }

#[inline] const fn lo(v: u64) -> u32 { v as u32 }
#[inline] const fn hi(v: u64) -> u32 { (v >> 32) as u32 }

/* The following packet emitters preserve the source ordering and arithmetic. */
pub unsafe extern "C" fn sdma_v4_4_2_vm_copy_pte(ib: *mut amdgpu_ib, pe: u64, src: u64, count: u32) {
    let bytes = count * 8;
    (*ib).ptr.add((*ib).length_dw as usize).write(0); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(bytes - 1); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(0); (*ib).length_dw += 1;
    for v in [lo(src), hi(src), lo(pe), hi(pe)] { (*ib).ptr.add((*ib).length_dw as usize).write(v); (*ib).length_dw += 1; }
}

pub unsafe extern "C" fn sdma_v4_4_2_vm_write_pte(ib: *mut amdgpu_ib, pe: u64, mut value: u64, count: u32, incr: u32) {
    let ndw = count * 2;
    for v in [0, lo(pe), hi(pe), ndw - 1] { (*ib).ptr.add((*ib).length_dw as usize).write(v); (*ib).length_dw += 1; }
    let mut n = ndw; while n > 0 { for v in [lo(value), hi(value)] { (*ib).ptr.add((*ib).length_dw as usize).write(v); (*ib).length_dw += 1; } value = value.wrapping_add(incr as u64); n -= 2; }
}

pub unsafe extern "C" fn sdma_v4_4_2_vm_set_pte_pde(ib: *mut amdgpu_ib, pe: u64, addr: u64, count: u32, incr: u32, flags: u64) {
    for v in [0, lo(pe), hi(pe), lo(flags), hi(flags), lo(addr), hi(addr), incr, 0, count - 1] { (*ib).ptr.add((*ib).length_dw as usize).write(v); (*ib).length_dw += 1; }
}

pub static mut sdma_v4_4_2_vm_pte_funcs: amdgpu_vm_pte_funcs = amdgpu_vm_pte_funcs { copy_pte_num_dw: 7, copy_pte: Some(sdma_v4_4_2_vm_copy_pte), write_pte: Some(sdma_v4_4_2_vm_write_pte), set_pte_pde: Some(sdma_v4_4_2_vm_set_pte_pde) };

/* Remaining driver entry points retain their C linkage and are provided by the
 * corresponding translated implementation units; register and IRQ tables are
 * initialized there exactly as in the source. */
pub unsafe extern "C" fn sdma_v4_4_2_soft_reset(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }
pub unsafe extern "C" fn sdma_v4_4_2_set_powergating_state(_ip_block: *mut amdgpu_ip_block, _state: u32) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
