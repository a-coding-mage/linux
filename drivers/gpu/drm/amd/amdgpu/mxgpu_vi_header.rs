/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

pub const VI_MAILBOX_TIMEDOUT: i32 = 12000;
pub const VI_MAILBOX_RESET_TIME: i32 = 12;

/* VI mailbox messages request */
#[repr(i32)]
pub enum idh_request {
    IDH_REQ_GPU_INIT_ACCESS = 1,
    IDH_REL_GPU_INIT_ACCESS,
    IDH_REQ_GPU_FINI_ACCESS,
    IDH_REL_GPU_FINI_ACCESS,
    IDH_REQ_GPU_RESET_ACCESS,

    IDH_LOG_VF_ERROR = 200,
}

/* VI mailbox messages data */
#[repr(i32)]
pub enum idh_event {
    IDH_CLR_MSG_BUF = 0,
    IDH_READY_TO_ACCESS_GPU,
    IDH_FLR_NOTIFICATION,
    IDH_FLR_NOTIFICATION_CMPL,

    IDH_TEXT_MESSAGE = 255,
}

extern "C" {
    pub static xgpu_vi_virt_ops: amdgpu_virt_ops;

    pub fn xgpu_vi_init_golden_registers(adev: *mut amdgpu_device);
    pub fn xgpu_vi_mailbox_set_irq_funcs(adev: *mut amdgpu_device);
    pub fn xgpu_vi_mailbox_add_irq_id(adev: *mut amdgpu_device) -> i32;
    pub fn xgpu_vi_mailbox_get_irq(adev: *mut amdgpu_device) -> i32;
    pub fn xgpu_vi_mailbox_put_irq(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
