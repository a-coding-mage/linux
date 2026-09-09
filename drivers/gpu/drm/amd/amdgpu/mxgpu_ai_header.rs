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
 *
 */

pub const AI_MAILBOX_POLL_ACK_TIMEDOUT: i32 = 500;
pub const AI_MAILBOX_POLL_MSG_TIMEDOUT: i32 = 6000;
pub const AI_MAILBOX_POLL_FLR_TIMEDOUT: i32 = 10000;
pub const AI_MAILBOX_POLL_MSG_REP_MAX: i32 = 11;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum idh_request {
    IDH_REQ_GPU_INIT_ACCESS = 1,
    IDH_REL_GPU_INIT_ACCESS,
    IDH_REQ_GPU_FINI_ACCESS,
    IDH_REL_GPU_FINI_ACCESS,
    IDH_REQ_GPU_RESET_ACCESS,
    IDH_REQ_GPU_INIT_DATA,

    IDH_LOG_VF_ERROR = 200,
    IDH_READY_TO_RESET = 201,
    IDH_RAS_POISON = 202,
    IDH_REQ_RAS_BAD_PAGES = 205,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum idh_event {
    IDH_CLR_MSG_BUF = 0,
    IDH_READY_TO_ACCESS_GPU,
    IDH_FLR_NOTIFICATION,
    IDH_FLR_NOTIFICATION_CMPL,
    IDH_SUCCESS,
    IDH_FAIL,
    IDH_QUERY_ALIVE,
    IDH_REQ_GPU_INIT_DATA_READY,
    IDH_RAS_POISON_READY,
    IDH_PF_SOFT_FLR_NOTIFICATION,
    IDH_RAS_ERROR_DETECTED,
    IDH_RAS_BAD_PAGES_READY = 15,
    IDH_RAS_BAD_PAGES_NOTIFICATION = 16,
    IDH_UNRECOV_ERR_NOTIFICATION = 17,
    IDH_TEXT_MESSAGE = 255,
}

// External type and function declarations supplied by other translation units.
extern "C" {
    pub static xgpu_ai_virt_ops: amdgpu_virt_ops;

    pub fn xgpu_ai_mailbox_set_irq_funcs(adev: *mut amdgpu_device);
    pub fn xgpu_ai_mailbox_add_irq_id(adev: *mut amdgpu_device) -> i32;
    pub fn xgpu_ai_mailbox_get_irq(adev: *mut amdgpu_device) -> i32;
    pub fn xgpu_ai_mailbox_put_irq(adev: *mut amdgpu_device);
}

// SOC15_REG_OFFSET, NBIO, and mmBIF_BX_PF0_MAILBOX_CONTROL are supplied by
// the surrounding translation and are intentionally preserved as macro calls.
#[macro_export]
macro_rules! AI_MAIBOX_CONTROL_TRN_OFFSET_BYTE {
    () => { (SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_PF0_MAILBOX_CONTROL) * 4) };
}

#[macro_export]
macro_rules! AI_MAIBOX_CONTROL_RCV_OFFSET_BYTE {
    () => { (SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_PF0_MAILBOX_CONTROL) * 4 + 1) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
