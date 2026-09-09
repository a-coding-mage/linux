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

// C includes: <linux/ktime.h>, <linux/types.h>, <linux/xarray.h>, and
// "amdgpu_ring.h" supply the external types referenced below.

use core::ffi::c_void;

// Forward declarations supplied by other translation units.
pub struct drm_device;
pub struct drm_file;
pub struct amdgpu_fpriv;
pub struct amdgpu_ctx_mgr;
pub struct amdgpu_device;
pub struct drm_sched_entity;
pub struct dma_fence;
pub struct kref;
pub struct spinlock_t;
pub struct xarray;
pub struct atomic64_t;

// Build-time constant supplied by the AMDGPU headers.
pub const AMDGPU_HW_IP_NUM: usize = 0; // TODO: supplied by amdgpu headers
pub const AMDGPU_MAX_ENTITY_NUM: usize = 4;

#[repr(C)]
pub struct amdgpu_ctx_entity {
    pub hw_ip: u32,
    pub sequence: u64,
    pub entity: drm_sched_entity,
    pub fences: [*mut dma_fence; 0], // C flexible array member
}

#[repr(C)]
pub struct amdgpu_ctx {
    pub refcount: kref,
    pub ring_lock: spinlock_t,
    pub reset_counter: u32,
    pub reset_counter_query: u32,
    pub init_priority: i32,
    pub override_priority: i32,
    pub stable_pstate: u32,
    pub preamble_presented: bool,
    pub generation: u64,
    pub ras_counter_ce: usize,
    pub ras_counter_ue: usize,
    pub mgr: *mut amdgpu_ctx_mgr,
    pub entities: [[*mut amdgpu_ctx_entity; AMDGPU_MAX_ENTITY_NUM]; AMDGPU_HW_IP_NUM],
}

#[repr(C)]
pub struct amdgpu_ctx_mgr {
    pub adev: *mut amdgpu_device,
    pub ctx_handles: xarray,
    pub time_spend: [atomic64_t; AMDGPU_HW_IP_NUM],
}

extern "C" {
    pub static amdgpu_ctx_num_entities: [u32; AMDGPU_HW_IP_NUM];

    pub fn amdgpu_ctx_get(fpriv: *mut amdgpu_fpriv, id: u32) -> *mut amdgpu_ctx;
    pub fn amdgpu_ctx_fini(kref: *mut kref);

    pub fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref));

    pub fn amdgpu_ctx_get_entity(
        ctx: *mut amdgpu_ctx,
        hw_ip: u32,
        instance: u32,
        ring: u32,
        entity: *mut *mut drm_sched_entity,
    ) -> i32;
    pub fn amdgpu_ctx_add_fence(
        ctx: *mut amdgpu_ctx,
        entity: *mut drm_sched_entity,
        fence: *mut dma_fence,
    ) -> u64;
    pub fn amdgpu_ctx_get_fence(
        ctx: *mut amdgpu_ctx,
        entity: *mut drm_sched_entity,
        seq: u64,
    ) -> *mut dma_fence;
    pub fn amdgpu_ctx_priority_is_valid(ctx_prio: i32) -> bool;
    pub fn amdgpu_ctx_priority_override(ctx: *mut amdgpu_ctx, ctx_prio: i32);

    pub fn amdgpu_ctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_ctx_wait_prev_fence(
        ctx: *mut amdgpu_ctx,
        entity: *mut drm_sched_entity,
    ) -> i32;
    pub fn amdgpu_ctx_mgr_init(mgr: *mut amdgpu_ctx_mgr, adev: *mut amdgpu_device);
    pub fn amdgpu_ctx_mgr_entity_flush(mgr: *mut amdgpu_ctx_mgr, timeout: isize) -> isize;
    pub fn amdgpu_ctx_mgr_fini(mgr: *mut amdgpu_ctx_mgr);
    pub fn amdgpu_ctx_mgr_usage(mgr: *mut amdgpu_ctx_mgr, usage: *mut ktime_t);
}

pub type ktime_t = i64;

#[inline]
pub unsafe fn amdgpu_ctx_put(ctx: *mut amdgpu_ctx) {
    if !ctx.is_null() {
        kref_put(&mut (*ctx).refcount, amdgpu_ctx_fini);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
