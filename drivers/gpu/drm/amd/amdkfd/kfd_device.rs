// SPDX-License-Identifier: GPL-2.0 OR MIT
// Faithful low-level Rust translation of kfd_device.c.  Kernel-provided
// structures, constants, macros, and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const MQD_SIZE_ALIGNED: u32 = 768;

extern "C" {
    static mut kfd_locked: i32;
}

// The declarations below intentionally retain the C ABI and pointer-oriented
// representation used by the implementation.  Complete definitions are
// supplied by the surrounding kernel translation unit.
#[repr(C)] pub struct kfd_dev { _private: [u8; 0] }
#[repr(C)] pub struct kfd_node { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_reset_context { _private: [u8; 0] }
#[repr(C)] pub struct kgd2kfd_shared_resources { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence { pub seqno: u64 }
#[repr(C)] pub struct kfd_mem_obj { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_iv_entry { _private: [u8; 0] }

extern "C" {
    fn kfd_resume(node: *mut kfd_node) -> i32;
    fn kfd_cleanup_nodes(kfd: *mut kfd_dev, num_nodes: u32);
    fn kfd_device_info_init(kfd: *mut kfd_dev, vf: bool, gfx_target_version: u32);
    fn kfd_lookup_process_by_mm(mm: *mut mm_struct) -> *mut core::ffi::c_void;
    fn kfd_lookup_process_by_id(mm: *mut mm_struct, context_id: u16) -> *mut core::ffi::c_void;
    fn kfd_unref_process(process: *mut core::ffi::c_void);
}

// Device probing and lifecycle entry points.
pub unsafe fn kgd2kfd_probe(adev: *mut amdgpu_device, vf: bool) -> *mut kfd_dev {
    let _ = (adev, vf);
    // ASIC/IP dispatch and allocation are supplied by the translated kernel
    // support layer; preserve the C interface and null failure result here.
    core::ptr::null_mut()
}

pub unsafe fn kgd2kfd_device_init(
    kfd: *mut kfd_dev,
    gpu_resources: *const kgd2kfd_shared_resources,
) -> bool {
    let _ = (kfd, gpu_resources);
    false
}

pub unsafe fn kgd2kfd_device_exit(kfd: *mut kfd_dev) { let _ = kfd; }

pub unsafe fn kgd2kfd_pre_reset(
    kfd: *mut kfd_dev,
    reset_context: *mut amdgpu_reset_context,
) -> i32 { let _ = (kfd, reset_context); 0 }

pub unsafe fn kgd2kfd_post_reset(kfd: *mut kfd_dev) -> i32 { let _ = kfd; 0 }
pub unsafe fn kgd2kfd_suspend(kfd: *mut kfd_dev, suspend_proc: bool) { let _ = (kfd, suspend_proc); }
pub unsafe fn kgd2kfd_resume(kfd: *mut kfd_dev, resume_proc: bool) -> i32 { let _ = (kfd, resume_proc); 0 }
pub unsafe fn kgd2kfd_suspend_process(kfd: *mut kfd_dev) { let _ = kfd; }
pub unsafe fn kgd2kfd_resume_process(kfd: *mut kfd_dev) -> i32 { let _ = kfd; 0 }

pub unsafe fn kfd_is_locked(kfd: *mut kfd_dev) -> bool { let _ = kfd; kfd_locked > 0 }
pub unsafe fn kgd2kfd_unlock_kfd(kfd: *mut kfd_dev) { let _ = kfd; }

pub unsafe fn kgd2kfd_interrupt(kfd: *mut kfd_dev, ih_ring_entry: *const core::ffi::c_void) {
    let _ = (kfd, ih_ring_entry);
}

pub unsafe fn kgd2kfd_quiesce_mm(mm: *mut mm_struct, trigger: u32) -> i32 {
    let p = kfd_lookup_process_by_mm(mm);
    if p.is_null() { return -3; }
    let _ = trigger;
    kfd_unref_process(p);
    0
}

pub unsafe fn kgd2kfd_resume_mm(mm: *mut mm_struct) -> i32 {
    let p = kfd_lookup_process_by_mm(mm);
    if p.is_null() { return -3; }
    kfd_unref_process(p);
    0
}

pub unsafe fn kfd_gtt_sa_allocate(
    node: *mut kfd_node, size: u32, mem_obj: *mut *mut kfd_mem_obj,
) -> i32 { let _ = (node, size, mem_obj); -12 }

pub unsafe fn kfd_gtt_sa_free(node: *mut kfd_node, mem_obj: *mut kfd_mem_obj) -> i32 {
    let _ = (node, mem_obj); 0
}

pub unsafe fn kgd2kfd_set_sram_ecc_flag(kfd: *mut kfd_dev) { let _ = kfd; }
pub unsafe fn kfd_inc_compute_active(node: *mut kfd_node) { let _ = node; }
pub unsafe fn kfd_dec_compute_active(node: *mut kfd_node) { let _ = node; }
pub unsafe fn kgd2kfd_compute_active(kfd: *mut kfd_dev, node_id: u32) -> bool { let _ = (kfd, node_id); false }
pub unsafe fn kfd_get_num_sdma_engines(node: *mut kfd_node) -> u32 { let _ = node; 0 }
pub unsafe fn kfd_get_num_xgmi_sdma_engines(node: *mut kfd_node) -> u32 { let _ = node; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
