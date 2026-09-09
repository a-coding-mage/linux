/*
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

// Dependency declarations are supplied by amdgpu.h and related kernel bindings.

pub const AMDGPU_RESET_MAX_HANDLERS: usize = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_RESET_FLAGS {
    AMDGPU_NEED_FULL_RESET = 0,
    AMDGPU_SKIP_HW_RESET = 1,
    AMDGPU_SKIP_COREDUMP = 2,
    AMDGPU_HOST_FLR = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_RESET_SRCS {
    AMDGPU_RESET_SRC_UNKNOWN,
    AMDGPU_RESET_SRC_JOB,
    AMDGPU_RESET_SRC_RAS,
    AMDGPU_RESET_SRC_MES,
    AMDGPU_RESET_SRC_HWS,
    AMDGPU_RESET_SRC_USER,
    AMDGPU_RESET_SRC_USERQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_reset_method {
    AMD_RESET_METHOD_NONE = -1,
    AMD_RESET_METHOD_LEGACY = 0,
    AMD_RESET_METHOD_MODE0,
    AMD_RESET_METHOD_MODE1,
    AMD_RESET_METHOD_MODE2,
    AMD_RESET_METHOD_LINK,
    AMD_RESET_METHOD_BACO,
    AMD_RESET_METHOD_PCI,
    AMD_RESET_METHOD_ON_INIT,
}

#[repr(C)]
pub struct amdgpu_reset_context {
    pub method: amd_reset_method,
    pub reset_req_dev: *mut amdgpu_device,
    pub job: *mut amdgpu_job,
    pub hive: *mut amdgpu_hive_info,
    pub reset_device_list: *mut list_head,
    pub flags: core::ffi::c_ulong,
    pub src: AMDGPU_RESET_SRCS,
}

#[repr(C)]
pub struct amdgpu_reset_control {
    pub handle: *mut core::ffi::c_void,
    pub reset_work: work_struct,
    pub reset_lock: mutex,
    pub reset_handlers: *mut [*mut amdgpu_reset_handler; AMDGPU_RESET_MAX_HANDLERS],
    pub in_reset: atomic_t,
    pub active_reset: amd_reset_method,
    pub get_reset_handler: Option<unsafe extern "C" fn(
        *mut amdgpu_reset_control,
        *mut amdgpu_reset_context,
    ) -> *mut amdgpu_reset_handler>,
    pub async_reset: Option<unsafe extern "C" fn(*mut work_struct)>,
}

#[repr(C)]
pub struct amdgpu_reset_handler {
    pub reset_method: amd_reset_method,
    pub prepare_env: Option<unsafe extern "C" fn(*mut amdgpu_reset_control, *mut amdgpu_reset_context) -> i32>,
    pub prepare_hwcontext: Option<unsafe extern "C" fn(*mut amdgpu_reset_control, *mut amdgpu_reset_context) -> i32>,
    pub perform_reset: Option<unsafe extern "C" fn(*mut amdgpu_reset_control, *mut amdgpu_reset_context) -> i32>,
    pub restore_hwcontext: Option<unsafe extern "C" fn(*mut amdgpu_reset_control, *mut amdgpu_reset_context) -> i32>,
    pub restore_env: Option<unsafe extern "C" fn(*mut amdgpu_reset_control, *mut amdgpu_reset_context) -> i32>,
    pub do_reset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
}

#[repr(C)]
pub enum amdgpu_reset_domain_type { SINGLE_DEVICE, XGMI_HIVE }

#[repr(C)]
pub struct amdgpu_reset_domain {
    pub refcount: kref,
    pub wq: *mut workqueue_struct,
    pub type_: amdgpu_reset_domain_type,
    pub sem: rw_semaphore,
    pub in_gpu_reset: atomic_t,
    pub reset_res: atomic_t,
}

extern "C" {
    pub fn amdgpu_reset_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_reset_fini(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_reset_prepare_hwcontext(adev: *mut amdgpu_device, reset_context: *mut amdgpu_reset_context) -> i32;
    pub fn amdgpu_reset_perform_reset(adev: *mut amdgpu_device, reset_context: *mut amdgpu_reset_context) -> i32;
    pub fn amdgpu_reset_prepare_env(adev: *mut amdgpu_device, reset_context: *mut amdgpu_reset_context) -> i32;
    pub fn amdgpu_reset_restore_env(adev: *mut amdgpu_device, reset_context: *mut amdgpu_reset_context) -> i32;
    pub fn amdgpu_reset_create_reset_domain(type_: amdgpu_reset_domain_type, wq_name: *mut core::ffi::c_char) -> *mut amdgpu_reset_domain;
    pub fn amdgpu_reset_destroy_reset_domain(ref_: *mut kref);
    pub fn amdgpu_device_lock_reset_domain(reset_domain: *mut amdgpu_reset_domain);
    pub fn amdgpu_device_unlock_reset_domain(reset_domain: *mut amdgpu_reset_domain);
    pub fn amdgpu_reset_get_desc(rst_ctxt: *mut amdgpu_reset_context, buf: *mut core::ffi::c_char, len: usize);
    pub static mut xgmi_reset_on_init_handler: amdgpu_reset_handler;
    pub fn amdgpu_reset_do_xgmi_reset_on_init(reset_context: *mut amdgpu_reset_context) -> i32;
    pub fn amdgpu_reset_in_recovery(adev: *mut amdgpu_device) -> bool;
}

pub unsafe fn amdgpu_reset_set_dpc_status(adev: *mut amdgpu_device, status: bool) {
    (*adev).pcie_reset_ctx.occurs_dpc = status;
    (*adev).no_hw_access = status;
}

pub unsafe fn amdgpu_reset_in_dpc(adev: *mut amdgpu_device) -> bool {
    (*adev).pcie_reset_ctx.occurs_dpc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
