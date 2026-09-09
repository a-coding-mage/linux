/* SPDX-License-Identifier: MIT
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding kernel/Rust translation.

#[repr(C)]
pub struct amdgpu_vram_mgr {
    pub manager: ttm_resource_manager,
    pub mm: gpu_buddy,
    /* protects access to buffer objects */
    pub lock: mutex,
    pub reservations_pending: list_head,
    pub reserved_pages: list_head,
    pub vis_usage: atomic64_t,
    pub default_page_size: u64,
    pub allocated_vres_list: list_head,
    pub cg_region: *mut dmem_cgroup_region,
}

#[repr(C)]
pub struct amdgpu_vres_task {
    pub pid: pid_t,
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
}

#[repr(C)]
pub struct amdgpu_vram_block_info {
    pub start: u64,
    pub size: u64,
    pub task: amdgpu_vres_task,
}

#[repr(C)]
pub struct amdgpu_vram_mgr_resource {
    pub base: ttm_resource,
    pub blocks: list_head,
    pub flags: core::ffi::c_ulong,
    pub vres_node: list_head,
    pub task: amdgpu_vres_task,
}

#[inline]
pub unsafe fn amdgpu_vram_mgr_block_start(block: *mut gpu_buddy_block) -> u64 {
    gpu_buddy_block_offset(block)
}

#[inline]
pub unsafe fn amdgpu_vram_mgr_block_size(block: *mut gpu_buddy_block) -> u64 {
    (PAGE_SIZE as u64) << gpu_buddy_block_order(block)
}

#[inline]
pub unsafe fn amdgpu_vram_mgr_is_cleared(block: *mut gpu_buddy_block) -> bool {
    gpu_buddy_block_is_clear(block)
}

#[inline]
pub unsafe fn to_amdgpu_vram_mgr_resource(
    res: *mut ttm_resource,
) -> *mut amdgpu_vram_mgr_resource {
    container_of!(res, amdgpu_vram_mgr_resource, base)
}

#[inline]
pub unsafe fn amdgpu_vram_mgr_set_cleared(res: *mut ttm_resource) {
    let ares: *mut amdgpu_vram_mgr_resource = to_amdgpu_vram_mgr_resource(res);

    WARN_ON!((*ares).flags & GPU_BUDDY_CLEARED != 0);
    (*ares).flags |= GPU_BUDDY_CLEARED;
}

unsafe extern "C" {
    pub fn amdgpu_vram_mgr_query_address_block_info(
        mgr: *mut amdgpu_vram_mgr,
        address: u64,
        info: *mut amdgpu_vram_block_info,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
