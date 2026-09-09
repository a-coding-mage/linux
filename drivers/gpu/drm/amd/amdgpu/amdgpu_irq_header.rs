/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel/driver translation.

pub const AMDGPU_MAX_IRQ_SRC_ID: usize = 0x100;
pub const AMDGPU_MAX_IRQ_CLIENT_ID: usize = 0x100;

pub const AMDGPU_IRQ_CLIENTID_LEGACY: u32 = 0;
pub const AMDGPU_IRQ_CLIENTID_MAX: usize = SOC15_IH_CLIENTID_MAX as usize;

pub const AMDGPU_IRQ_SRC_DATA_MAX_SIZE_DW: usize = 4;

#[repr(C)]
pub struct amdgpu_device;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_interrupt_state {
    AMDGPU_IRQ_STATE_DISABLE,
    AMDGPU_IRQ_STATE_ENABLE,
}

#[repr(C)]
pub struct amdgpu_iv_entry {
    pub ih: *mut amdgpu_ih_ring,
    pub client_id: ::core::ffi::c_uint,
    pub src_id: ::core::ffi::c_uint,
    pub ring_id: ::core::ffi::c_uint,
    pub vmid: ::core::ffi::c_uint,
    pub vmid_src: ::core::ffi::c_uint,
    pub timestamp: u64,
    pub timestamp_src: ::core::ffi::c_uint,
    pub pasid: ::core::ffi::c_uint,
    pub node_id: ::core::ffi::c_uint,
    pub src_data: [::core::ffi::c_uint; AMDGPU_IRQ_SRC_DATA_MAX_SIZE_DW],
    pub iv_entry: *const u32,
}

#[repr(C)]
pub struct amdgpu_irq_src {
    pub num_types: ::core::ffi::c_uint,
    pub enabled_types: *mut atomic_t,
    pub funcs: *const amdgpu_irq_src_funcs,
}

#[repr(C)]
pub struct amdgpu_irq_client {
    pub sources: *mut *mut amdgpu_irq_src,
}

// Provided by interrupt generating IP blocks.
#[repr(C)]
pub struct amdgpu_irq_src_funcs {
    pub set: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        source: *mut amdgpu_irq_src,
        type_: ::core::ffi::c_uint,
        state: amdgpu_interrupt_state,
    ) -> ::core::ffi::c_int>,
    pub process: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        source: *mut amdgpu_irq_src,
        entry: *mut amdgpu_iv_entry,
    ) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct amdgpu_irq {
    pub installed: bool,
    pub irq: ::core::ffi::c_uint,
    pub lock: spinlock_t,
    // Interrupt sources.
    pub client: [amdgpu_irq_client; AMDGPU_IRQ_CLIENTID_MAX],
    // Status, etc.
    pub msi_enabled: bool, // MSI enabled.
    // Interrupt rings.
    pub ih: amdgpu_ih_ring,
    pub ih1: amdgpu_ih_ring,
    pub ih2: amdgpu_ih_ring,
    pub ih_soft: amdgpu_ih_ring,
    pub ih_funcs: *const amdgpu_ih_funcs,
    pub ih1_work: work_struct,
    pub ih2_work: work_struct,
    pub ih_soft_work: work_struct,
    pub self_irq: amdgpu_irq_src,
    // Generic IRQ data.
    pub domain: *mut irq_domain, // GPU IRQ controller domain.
    pub virq: [::core::ffi::c_uint; AMDGPU_MAX_IRQ_SRC_ID],
    pub srbm_soft_reset: u32,
    pub retry_cam_doorbell_index: u32,
    pub retry_cam_enabled: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum interrupt_node_id_per_aid {
    AID0_NODEID = 0,
    XCD0_NODEID = 1,
    XCD1_NODEID = 2,
    AID1_NODEID = 4,
    XCD2_NODEID = 5,
    XCD3_NODEID = 6,
    AID2_NODEID = 8,
    XCD4_NODEID = 9,
    XCD5_NODEID = 10,
    AID3_NODEID = 12,
    XCD6_NODEID = 13,
    XCD7_NODEID = 14,
    NODEID_MAX,
}

pub unsafe extern "C" {
    pub static mut node_id_to_phys_map: [::core::ffi::c_int; NODEID_MAX as usize];
    pub fn amdgpu_irq_disable_all(adev: *mut amdgpu_device);
    pub fn amdgpu_irq_init(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_irq_fini_sw(adev: *mut amdgpu_device);
    pub fn amdgpu_irq_fini_hw(adev: *mut amdgpu_device);
    pub fn amdgpu_irq_add_id(adev: *mut amdgpu_device, client_id: ::core::ffi::c_uint,
        src_id: ::core::ffi::c_uint, source: *mut amdgpu_irq_src) -> ::core::ffi::c_int;
    pub fn amdgpu_irq_dispatch(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring);
    pub fn amdgpu_irq_delegate(adev: *mut amdgpu_device, entry: *mut amdgpu_iv_entry,
        num_dw: ::core::ffi::c_uint);
    pub fn amdgpu_irq_update(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src,
        type_: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn amdgpu_irq_get(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src,
        type_: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn amdgpu_irq_put(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src,
        type_: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn amdgpu_irq_enabled(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src,
        type_: ::core::ffi::c_uint) -> bool;
    pub fn amdgpu_irq_gpu_reset_resume_helper(adev: *mut amdgpu_device);
    pub fn amdgpu_irq_add_domain(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_irq_remove_domain(adev: *mut amdgpu_device);
    pub fn amdgpu_irq_create_mapping(adev: *mut amdgpu_device,
        src_id: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn amdgpu_restore_msix(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
