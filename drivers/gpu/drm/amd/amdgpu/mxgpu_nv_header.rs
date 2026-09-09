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

pub const NV_MAILBOX_POLL_ACK_TIMEDOUT: u32 = 500;
pub const NV_MAILBOX_POLL_MSG_TIMEDOUT: u32 = 15000;
pub const NV_MAILBOX_POLL_FLR_TIMEDOUT: u32 = 10000;
pub const NV_MAILBOX_POLL_MSG_REP_MAX: u32 = 11;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    IDH_REQ_RAS_ERROR_COUNT = 203,
    IDH_REQ_RAS_CPER_DUMP = 204,
    IDH_REQ_RAS_BAD_PAGES = 205,
    IDH_REQ_RAS_CHK_CRITI = 206,
    IDH_REQ_RAS_REMOTE_CMD = 207,
    IDH_REQ_PTL_UPDATE = 208,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    IDH_RAS_ERROR_COUNT_READY = 11,
    IDH_RAS_CPER_DUMP_READY = 14,
    IDH_RAS_BAD_PAGES_READY = 15,
    IDH_RAS_BAD_PAGES_NOTIFICATION = 16,
    IDH_UNRECOV_ERR_NOTIFICATION = 17,
    IDH_REQ_RAS_CHK_CRITI_READY = 18,
    IDH_REQ_RAS_REMOTE_CMD_READY = 19,
    IDH_PTL_UPDATE_READY = 20,

    IDH_TEXT_MESSAGE = 255,
}

extern "C" {
    pub static xgpu_nv_virt_ops: amdgpu_virt_ops;

    pub fn xgpu_nv_mailbox_set_irq_funcs(adev: *mut amdgpu_device);
    pub fn xgpu_nv_mailbox_add_irq_id(adev: *mut amdgpu_device) -> i32;
    pub fn xgpu_nv_mailbox_get_irq(adev: *mut amdgpu_device) -> i32;
    pub fn xgpu_nv_mailbox_put_irq(adev: *mut amdgpu_device);
}

pub const mmMAILBOX_CONTROL: u32 = 0xE5E;

pub const NV_MAIBOX_CONTROL_TRN_OFFSET_BYTE: u32 = mmMAILBOX_CONTROL * 4;
pub const NV_MAIBOX_CONTROL_RCV_OFFSET_BYTE: u32 = NV_MAIBOX_CONTROL_TRN_OFFSET_BYTE + 1;

pub const mmMAILBOX_MSGBUF_TRN_DW0: u32 = 0xE56;
pub const mmMAILBOX_MSGBUF_TRN_DW1: u32 = 0xE57;
pub const mmMAILBOX_MSGBUF_TRN_DW2: u32 = 0xE58;
pub const mmMAILBOX_MSGBUF_TRN_DW3: u32 = 0xE59;

pub const mmMAILBOX_MSGBUF_RCV_DW0: u32 = 0xE5A;
pub const mmMAILBOX_MSGBUF_RCV_DW1: u32 = 0xE5B;
pub const mmMAILBOX_MSGBUF_RCV_DW2: u32 = 0xE5C;
pub const mmMAILBOX_MSGBUF_RCV_DW3: u32 = 0xE5D;

pub const mmMAILBOX_INT_CNTL: u32 = 0xE5F;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
