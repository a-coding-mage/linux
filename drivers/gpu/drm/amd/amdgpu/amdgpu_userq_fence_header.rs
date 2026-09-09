// SPDX-License-Identifier: MIT
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel/amdgpu translation.

#[repr(C)]
pub struct amdgpu_userq_fence {
    pub base: dma_fence,
    /*
     * This lock is necessary to synchronize the
     * userqueue dma fence operations.
     */
    pub lock: spinlock_t,
    pub link: list_head,
    pub fence_drv_array_count: core::ffi::c_ulong,
    pub fence_drv: *mut amdgpu_userq_fence_driver,
    pub fence_drv_array: *mut *mut amdgpu_userq_fence_driver,
}

#[repr(C)]
pub struct amdgpu_userq_fence_driver {
    pub refcount: kref,
    pub va: u64,
    pub gpu_addr: u64,
    pub cpu_addr: *mut u64,
    pub context: u64,
    /*
     * This lock is necesaary to synchronize the access
     * to the fences list by the fence driver.
     */
    pub fence_list_lock: spinlock_t,
    pub fences: list_head,
    pub adev: *mut amdgpu_device,
    pub timeline_name: [core::ffi::c_char; TASK_COMM_LEN],
}

extern "C" {
    pub fn amdgpu_userq_fence_driver_get(
        fence_drv: *mut amdgpu_userq_fence_driver,
    );
    pub fn amdgpu_userq_fence_driver_put(
        fence_drv: *mut amdgpu_userq_fence_driver,
    );
    pub fn amdgpu_userq_fence_driver_alloc(
        adev: *mut amdgpu_device,
        fence_drv_req: *mut *mut amdgpu_userq_fence_driver,
    ) -> core::ffi::c_int;
    pub fn amdgpu_userq_fence_driver_free(userq: *mut amdgpu_usermode_queue);
    pub fn amdgpu_userq_fence_driver_process(
        fence_drv: *mut amdgpu_userq_fence_driver,
    ) -> core::ffi::c_int;
    pub fn amdgpu_userq_fence_driver_force_completion(
        userq: *mut amdgpu_usermode_queue,
    );
    pub fn amdgpu_userq_fence_driver_destroy(ref_: *mut kref);
    pub fn amdgpu_userq_signal_ioctl(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        filp: *mut drm_file,
    ) -> core::ffi::c_int;
    pub fn amdgpu_userq_wait_ioctl(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        filp: *mut drm_file,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
