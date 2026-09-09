/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding C/Rust translation units.

pub const AMDGPU_MAX_VPE_INSTANCES: u32 = 2;

#[repr(C)]
pub struct vpe_funcs {
    pub get_reg_offset: Option<unsafe extern "C" fn(*mut amdgpu_vpe, u32, u32) -> u32>,
    pub set_regs: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
    pub irq_init: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
    pub init_microcode: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
    pub load_microcode: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
    pub ring_init: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
    pub ring_start: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
    pub ring_stop: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
    pub ring_fini: Option<unsafe extern "C" fn(*mut amdgpu_vpe) -> i32>,
}

#[repr(C)]
pub struct vpe_regs {
    pub queue0_rb_rptr_lo: u32,
    pub queue0_rb_rptr_hi: u32,
    pub queue0_rb_wptr_lo: u32,
    pub queue0_rb_wptr_hi: u32,
    pub queue0_preempt: u32,
    pub dpm_enable: u32,
    pub dpm_pratio: u32,
    pub dpm_request_interval: u32,
    pub dpm_decision_threshold: u32,
    pub dpm_busy_clamp_threshold: u32,
    pub dpm_idle_clamp_threshold: u32,
    pub dpm_request_lv: u32,
    pub context_indicator: u32,
}

#[repr(C)]
pub struct amdgpu_vpe {
    pub ring: amdgpu_ring,
    pub trap_irq: amdgpu_irq_src,
    pub funcs: *const vpe_funcs,
    pub regs: vpe_regs,
    pub fw: *const firmware,
    pub fw_version: u32,
    pub feature_version: u32,
    pub cmdbuf_obj: *mut amdgpu_bo,
    pub cmdbuf_gpu_addr: u64,
    pub cmdbuf_cpu_addr: *mut u32,
    pub idle_work: delayed_work,
    pub context_started: bool,
    pub num_instances: u32,
    pub collaborate_mode: bool,
    pub supported_reset: u32,
}

extern "C" {
    pub fn amdgpu_vpe_psp_update_sram(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_vpe_init_microcode(vpe: *mut amdgpu_vpe) -> i32;
    pub fn amdgpu_vpe_ring_init(vpe: *mut amdgpu_vpe) -> i32;
    pub fn amdgpu_vpe_ring_fini(vpe: *mut amdgpu_vpe) -> i32;
    pub fn amdgpu_vpe_configure_dpm(vpe: *mut amdgpu_vpe) -> i32;
    pub fn amdgpu_vpe_sysfs_reset_mask_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_vpe_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> i32;
}

#[inline]
pub unsafe fn vpe_ring_init(vpe: *mut amdgpu_vpe) -> i32 {
    if !(*(*vpe).funcs).ring_init.is_none() { ((*(*vpe).funcs).ring_init.unwrap())(vpe) } else { 0 }
}
#[inline]
pub unsafe fn vpe_ring_start(vpe: *mut amdgpu_vpe) -> i32 {
    if !(*(*vpe).funcs).ring_start.is_none() { ((*(*vpe).funcs).ring_start.unwrap())(vpe) } else { 0 }
}
#[inline]
pub unsafe fn vpe_ring_stop(vpe: *mut amdgpu_vpe) -> i32 {
    if !(*(*vpe).funcs).ring_stop.is_none() { ((*(*vpe).funcs).ring_stop.unwrap())(vpe) } else { 0 }
}
#[inline]
pub unsafe fn vpe_ring_fini(vpe: *mut amdgpu_vpe) -> i32 {
    if !(*(*vpe).funcs).ring_fini.is_none() { ((*(*vpe).funcs).ring_fini.unwrap())(vpe) } else { 0 }
}
#[inline]
pub unsafe fn vpe_get_reg_offset(vpe: *mut amdgpu_vpe, inst: u32, offset: u32) -> u32 {
    if !(*(*vpe).funcs).get_reg_offset.is_none() { ((*(*vpe).funcs).get_reg_offset.unwrap())(vpe, inst, offset) } else { 0 }
}
#[inline]
pub unsafe fn vpe_set_regs(vpe: *mut amdgpu_vpe) -> i32 { if !(*(*vpe).funcs).set_regs.is_none() { ((*(*vpe).funcs).set_regs.unwrap())(vpe) } else { 0 } }
#[inline]
pub unsafe fn vpe_irq_init(vpe: *mut amdgpu_vpe) -> i32 { if !(*(*vpe).funcs).irq_init.is_none() { ((*(*vpe).funcs).irq_init.unwrap())(vpe) } else { 0 } }
#[inline]
pub unsafe fn vpe_init_microcode(vpe: *mut amdgpu_vpe) -> i32 { if !(*(*vpe).funcs).init_microcode.is_none() { ((*(*vpe).funcs).init_microcode.unwrap())(vpe) } else { 0 } }
#[inline]
pub unsafe fn vpe_load_microcode(vpe: *mut amdgpu_vpe) -> i32 { if !(*(*vpe).funcs).load_microcode.is_none() { ((*(*vpe).funcs).load_microcode.unwrap())(vpe) } else { 0 } }

extern "C" {
    pub static vpe_v6_1_ip_block: amdgpu_ip_block_version;
    pub static vpe_v2_0_ip_block: amdgpu_ip_block_version;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
