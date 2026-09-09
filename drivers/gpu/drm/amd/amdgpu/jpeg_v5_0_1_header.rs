/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

#[repr(C)]
pub struct amdgpu_ip_block_version {
    _private: [u8; 0],
}

pub extern "C" {
    pub static jpeg_v5_0_1_ip_block: amdgpu_ip_block_version;
}

pub const regUVD_JRBC0_UVD_JRBC_SCRATCH0_INTERNAL_OFFSET: u32 = 0x4094;
pub const regUVD_JRBC_EXTERNAL_MCM_ADDR_INTERNAL_OFFSET: u32 = 0x1bffe;

pub const regUVD_JRBC0_UVD_JRBC_RB_WPTR: u32 = 0x0640;
pub const regUVD_JRBC0_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 1;
pub const regUVD_JRBC0_UVD_JRBC_STATUS: u32 = 0x0649;
pub const regUVD_JRBC0_UVD_JRBC_STATUS_BASE_IDX: u32 = 1;
pub const regUVD_JRBC0_UVD_JRBC_RB_RPTR: u32 = 0x064a;
pub const regUVD_JRBC0_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 1;

pub const regUVD_JRBC1_UVD_JRBC_RB_WPTR: u32 = 0x0000;
pub const regUVD_JRBC1_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC1_UVD_JRBC_STATUS: u32 = 0x0009;
pub const regUVD_JRBC1_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC1_UVD_JRBC_RB_RPTR: u32 = 0x000a;
pub const regUVD_JRBC1_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC2_UVD_JRBC_RB_WPTR: u32 = 0x0040;
pub const regUVD_JRBC2_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC2_UVD_JRBC_STATUS: u32 = 0x0049;
pub const regUVD_JRBC2_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC2_UVD_JRBC_RB_RPTR: u32 = 0x004a;
pub const regUVD_JRBC2_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC3_UVD_JRBC_RB_WPTR: u32 = 0x0080;
pub const regUVD_JRBC3_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC3_UVD_JRBC_STATUS: u32 = 0x0089;
pub const regUVD_JRBC3_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC3_UVD_JRBC_RB_RPTR: u32 = 0x008a;
pub const regUVD_JRBC3_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC4_UVD_JRBC_RB_WPTR: u32 = 0x00c0;
pub const regUVD_JRBC4_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC4_UVD_JRBC_STATUS: u32 = 0x00c9;
pub const regUVD_JRBC4_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC4_UVD_JRBC_RB_RPTR: u32 = 0x00ca;
pub const regUVD_JRBC4_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC5_UVD_JRBC_RB_WPTR: u32 = 0x0100;
pub const regUVD_JRBC5_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC5_UVD_JRBC_STATUS: u32 = 0x0109;
pub const regUVD_JRBC5_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC5_UVD_JRBC_RB_RPTR: u32 = 0x010a;
pub const regUVD_JRBC5_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC6_UVD_JRBC_RB_WPTR: u32 = 0x0140;
pub const regUVD_JRBC6_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC6_UVD_JRBC_STATUS: u32 = 0x0149;
pub const regUVD_JRBC6_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC6_UVD_JRBC_RB_RPTR: u32 = 0x014a;
pub const regUVD_JRBC6_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC7_UVD_JRBC_RB_WPTR: u32 = 0x0180;
pub const regUVD_JRBC7_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC7_UVD_JRBC_STATUS: u32 = 0x0189;
pub const regUVD_JRBC7_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC7_UVD_JRBC_RB_RPTR: u32 = 0x018a;
pub const regUVD_JRBC7_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC8_UVD_JRBC_RB_WPTR: u32 = 0x01c0;
pub const regUVD_JRBC8_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 0;
pub const regUVD_JRBC8_UVD_JRBC_STATUS: u32 = 0x01c9;
pub const regUVD_JRBC8_UVD_JRBC_STATUS_BASE_IDX: u32 = 0;
pub const regUVD_JRBC8_UVD_JRBC_RB_RPTR: u32 = 0x01ca;
pub const regUVD_JRBC8_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 0;

pub const regUVD_JRBC9_UVD_JRBC_RB_WPTR: u32 = 0x0440;
pub const regUVD_JRBC9_UVD_JRBC_RB_WPTR_BASE_IDX: u32 = 1;
pub const regUVD_JRBC9_UVD_JRBC_STATUS: u32 = 0x0449;
pub const regUVD_JRBC9_UVD_JRBC_STATUS_BASE_IDX: u32 = 1;
pub const regUVD_JRBC9_UVD_JRBC_RB_RPTR: u32 = 0x044a;
pub const regUVD_JRBC9_UVD_JRBC_RB_RPTR_BASE_IDX: u32 = 1;

pub const regUVD_JMI0_JPEG_LMI_DROP: u32 = 0x0663;
pub const regUVD_JMI0_JPEG_LMI_DROP_BASE_IDX: u32 = 1;
pub const regUVD_JMI0_UVD_JMI_CLIENT_STALL: u32 = 0x067a;
pub const regUVD_JMI0_UVD_JMI_CLIENT_STALL_BASE_IDX: u32 = 1;
pub const regUVD_JMI0_UVD_JMI_CLIENT_CLEAN_STATUS: u32 = 0x067b;
pub const regUVD_JMI0_UVD_JMI_CLIENT_CLEAN_STATUS_BASE_IDX: u32 = 1;
pub const regJPEG_CORE_RST_CTRL: u32 = 0x072e;
pub const regJPEG_CORE_RST_CTRL_BASE_IDX: u32 = 1;

pub const regVCN_RRMT_CNTL: u32 = 0x0940;
pub const regVCN_RRMT_CNTL_BASE_IDX: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdgpu_jpeg_v5_0_1_sub_block {
    AMDGPU_JPEG_V5_0_1_JPEG0 = 0,
    AMDGPU_JPEG_V5_0_1_JPEG1,
    AMDGPU_JPEG_V5_0_1_MAX_SUB_BLOCK,
}

#[repr(C)]
pub struct amdgpu_irq_src {
    _private: [u8; 0],
}
#[repr(C)]
pub struct amdgpu_iv_entry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn jpeg_v5_0_1_process_interrupt(
        adev: *mut amdgpu_device,
        source: *mut amdgpu_irq_src,
        entry: *mut amdgpu_iv_entry,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
