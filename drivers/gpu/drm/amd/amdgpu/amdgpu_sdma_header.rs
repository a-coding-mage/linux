/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by the surrounding translation unit: amdgpu_ras.h.

pub const AMDGPU_MAX_SDMA_INSTANCES: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdgpu_sdma_irq {
    AMDGPU_SDMA_IRQ_INSTANCE0 = 0,
    AMDGPU_SDMA_IRQ_INSTANCE1,
    AMDGPU_SDMA_IRQ_INSTANCE2,
    AMDGPU_SDMA_IRQ_INSTANCE3,
    AMDGPU_SDMA_IRQ_INSTANCE4,
    AMDGPU_SDMA_IRQ_INSTANCE5,
    AMDGPU_SDMA_IRQ_INSTANCE6,
    AMDGPU_SDMA_IRQ_INSTANCE7,
    AMDGPU_SDMA_IRQ_INSTANCE8,
    AMDGPU_SDMA_IRQ_INSTANCE9,
    AMDGPU_SDMA_IRQ_INSTANCE10,
    AMDGPU_SDMA_IRQ_INSTANCE11,
    AMDGPU_SDMA_IRQ_INSTANCE12,
    AMDGPU_SDMA_IRQ_INSTANCE13,
    AMDGPU_SDMA_IRQ_INSTANCE14,
    AMDGPU_SDMA_IRQ_INSTANCE15,
    AMDGPU_SDMA_IRQ_LAST,
}

#[inline]
pub fn NUM_SDMA(x: u32) -> u32 { unsafe { hweight32(x) } }

extern "C" { fn hweight32(x: u32) -> u32; }

#[repr(C)]
pub struct amdgpu_sdma_csa_info { pub size: u32, pub alignment: u32 }

#[repr(C)]
pub struct amdgpu_sdma_funcs {
    pub stop_kernel_queue: Option<unsafe extern "C" fn(*mut amdgpu_ring) -> i32>,
    pub start_kernel_queue: Option<unsafe extern "C" fn(*mut amdgpu_ring) -> i32>,
    pub soft_reset_kernel_queue: Option<unsafe extern "C" fn(*mut amdgpu_device, u32) -> i32>,
}

#[repr(C)]
pub union amdgpu_sdma_instance_ids { pub aid_id: u32, pub xcc_id: u32 }

#[repr(C)]
pub struct amdgpu_sdma_instance {
    pub fw: *const firmware,
    pub fw_version: u32,
    pub feature_version: u32,
    pub ring: amdgpu_ring,
    pub page: amdgpu_ring,
    pub burst_nop: bool,
    pub ids: amdgpu_sdma_instance_ids,
    pub sdma_fw_obj: *mut amdgpu_bo,
    pub sdma_fw_gpu_addr: u64,
    pub sdma_fw_ptr: *mut u32,
    pub engine_reset_mutex: mutex,
    pub gfx_guilty: bool,
    pub page_guilty: bool,
    pub funcs: *const amdgpu_sdma_funcs,
}

#[repr(C)]
pub struct amdgpu_sdma_ras { pub ras_block: amdgpu_ras_block_object }

#[repr(C)]
pub struct amdgpu_sdma {
    pub instance: [amdgpu_sdma_instance; AMDGPU_MAX_SDMA_INSTANCES],
    pub trap_irq: amdgpu_irq_src,
    pub illegal_inst_irq: amdgpu_irq_src,
    pub fence_irq: amdgpu_irq_src,
    pub ecc_irq: amdgpu_irq_src,
    pub vm_hole_irq: amdgpu_irq_src,
    pub doorbell_invalid_irq: amdgpu_irq_src,
    pub pool_timeout_irq: amdgpu_irq_src,
    pub srbm_write_irq: amdgpu_irq_src,
    pub ctxt_empty_irq: amdgpu_irq_src,
    pub num_instances: i32,
    pub sdma_mask: u32,
    pub num_inst_per_aid_or_xcc: amdgpu_sdma_num_inst,
    pub srbm_soft_reset: u32,
    pub has_page_queue: bool,
    pub ras_if: *mut ras_common_if,
    pub ras: *mut amdgpu_sdma_ras,
    pub ip_dump: *mut u32,
    pub supported_reset: u32,
    pub reset_callback_list: list_head,
    pub no_user_submission: bool,
    pub disable_uq: bool,
    pub get_csa_info: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_sdma_csa_info)>,
}

#[repr(C)]
pub union amdgpu_sdma_num_inst { pub num_inst_per_aid: i32, pub num_inst_per_xcc: i32 }

/* Provided by hw blocks that can move/clear data. e.g., gfx or sdma. */
#[repr(C)]
pub struct amdgpu_buffer_funcs {
    pub copy_max_bytes: u32,
    pub copy_num_dw: usize,
    pub emit_copy_buffer: Option<unsafe extern "C" fn(*mut amdgpu_ib, u64, u64, u32, u32)>,
    pub fill_max_bytes: u32,
    pub fill_num_dw: usize,
    pub emit_fill_buffer: Option<unsafe extern "C" fn(*mut amdgpu_ib, u32, u64, u32)>,
}

#[macro_export]
macro_rules! amdgpu_emit_copy_buffer {
    ($adev:expr, $ib:expr, $s:expr, $d:expr, $b:expr, $t:expr) => {
        ((*$adev).mman.buffer_funcs.emit_copy_buffer.unwrap())($ib, $s, $d, $b, $t)
    };
}

#[macro_export]
macro_rules! amdgpu_emit_fill_buffer {
    ($adev:expr, $ib:expr, $s:expr, $d:expr, $b:expr) => {
        ((*$adev).mman.buffer_funcs.emit_fill_buffer.unwrap())($ib, $s, $d, $b)
    };
}

extern "C" {
    pub fn amdgpu_sdma_reset_engine(adev: *mut amdgpu_device, instance_id: u32, caller_handles_kernel_queues: bool) -> i32;
    pub fn amdgpu_sdma_get_instance_from_ring(ring: *mut amdgpu_ring) -> *mut amdgpu_sdma_instance;
    pub fn amdgpu_sdma_get_index_from_ring(ring: *mut amdgpu_ring, index: *mut u32) -> i32;
    pub fn amdgpu_sdma_get_csa_mc_addr(ring: *mut amdgpu_ring, vmid: usize) -> u64;
    pub fn amdgpu_sdma_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> i32;
    pub fn amdgpu_sdma_process_ras_data_cb(adev: *mut amdgpu_device, err_data: *mut core::ffi::c_void, entry: *mut amdgpu_iv_entry) -> i32;
    pub fn amdgpu_sdma_process_ecc_irq(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32;
    pub fn amdgpu_sdma_init_microcode(adev: *mut amdgpu_device, instance: u32, duplicate: bool) -> i32;
    pub fn amdgpu_sdma_destroy_inst_ctx(adev: *mut amdgpu_device, duplicate: bool);
    pub fn amdgpu_sdma_ras_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_debugfs_sdma_sched_mask_init(adev: *mut amdgpu_device);
    pub fn amdgpu_sdma_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_sdma_sysfs_reset_mask_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_sdma_is_shared_inv_eng(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> bool;
    pub fn amdgpu_sdma_get_shared_ring(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> *mut amdgpu_ring;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
