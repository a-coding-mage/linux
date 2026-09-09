/* SPDX-License-Identifier: MIT */
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

#[repr(i32)]
pub enum amdgpu_reg_state {
    AMDGPU_REG_STATE_TYPE_INVALID = 0,
    AMDGPU_REG_STATE_TYPE_XGMI = 1,
    AMDGPU_REG_STATE_TYPE_WAFL = 2,
    AMDGPU_REG_STATE_TYPE_PCIE = 3,
    AMDGPU_REG_STATE_TYPE_USR = 4,
    AMDGPU_REG_STATE_TYPE_USR_1 = 5,
}

#[repr(i32)]
pub enum amdgpu_sysfs_reg_offset {
    AMDGPU_SYS_REG_STATE_XGMI = 0x0000,
    AMDGPU_SYS_REG_STATE_WAFL = 0x1000,
    AMDGPU_SYS_REG_STATE_PCIE = 0x2000,
    AMDGPU_SYS_REG_STATE_USR = 0x3000,
    AMDGPU_SYS_REG_STATE_USR_1 = 0x4000,
    AMDGPU_SYS_REG_STATE_END = 0x5000,
}

#[repr(C)]
pub struct amdgpu_reg_state_header {
    pub structure_size: u16,
    pub format_revision: u8,
    pub content_revision: u8,
    pub state_type: u8,
    pub num_instances: u8,
    pub pad: u16,
}

#[repr(i32)]
pub enum amdgpu_reg_inst_state {
    AMDGPU_INST_S_OK,
    AMDGPU_INST_S_EDISABLED,
    AMDGPU_INST_S_EACCESS,
}

#[repr(C)]
pub struct amdgpu_smn_reg_data {
    pub addr: u64,
    pub value: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct amdgpu_reg_inst_header {
    pub instance: u16,
    pub state: u16,
    pub num_smn_regs: u16,
    pub pad: u16,
}

#[repr(C)]
pub struct amdgpu_regs_xgmi_v1_0 {
    pub inst_header: amdgpu_reg_inst_header,
    pub smn_reg_values: [amdgpu_smn_reg_data; 0],
}

#[repr(C)]
pub struct amdgpu_reg_state_xgmi_v1_0 {
    /* common_header.state_type must be AMDGPU_REG_STATE_TYPE_XGMI */
    pub common_header: amdgpu_reg_state_header,
    pub xgmi_state_regs: [amdgpu_regs_xgmi_v1_0; 0],
}

#[repr(C)]
pub struct amdgpu_regs_wafl_v1_0 {
    pub inst_header: amdgpu_reg_inst_header,
    pub smn_reg_values: [amdgpu_smn_reg_data; 0],
}

#[repr(C)]
pub struct amdgpu_reg_state_wafl_v1_0 {
    /* common_header.state_type must be AMDGPU_REG_STATE_TYPE_WAFL */
    pub common_header: amdgpu_reg_state_header,
    pub wafl_state_regs: [amdgpu_regs_wafl_v1_0; 0],
}

#[repr(C)]
pub struct amdgpu_regs_pcie_v1_0 {
    pub inst_header: amdgpu_reg_inst_header,
    pub device_status: u16,
    pub link_status: u16,
    pub sub_bus_number_latency: u32,
    pub pcie_corr_err_status: u32,
    pub pcie_uncorr_err_status: u32,
    pub smn_reg_values: [amdgpu_smn_reg_data; 0],
}

#[repr(C)]
pub struct amdgpu_reg_state_pcie_v1_0 {
    /* common_header.state_type must be AMDGPU_REG_STATE_TYPE_PCIE */
    pub common_header: amdgpu_reg_state_header,
    pub pci_state_regs: [amdgpu_regs_pcie_v1_0; 0],
}

#[repr(C)]
pub struct amdgpu_regs_usr_v1_0 {
    pub inst_header: amdgpu_reg_inst_header,
    pub smn_reg_values: [amdgpu_smn_reg_data; 0],
}

#[repr(C)]
pub struct amdgpu_reg_state_usr_v1_0 {
    /* common_header.state_type must be AMDGPU_REG_STATE_TYPE_USR */
    pub common_header: amdgpu_reg_state_header,
    pub usr_state_regs: [amdgpu_regs_usr_v1_0; 0],
}

#[inline]
pub fn amdgpu_reginst_size(num_inst: u16, inst_size: usize, num_regs: u16) -> usize {
    (num_inst as usize).wrapping_mul(
        inst_size.wrapping_add((num_regs as usize).wrapping_mul(core::mem::size_of::<amdgpu_smn_reg_data>())),
    )
}

#[macro_export]
macro_rules! amdgpu_asic_get_reg_state_supported {
    ($adev:expr) => {
        if unsafe { !(*$adev).asic_funcs.is_null() && !(*(*$adev).asic_funcs).get_reg_state.is_none() } { 1 } else { 0 }
    };
}

#[macro_export]
macro_rules! amdgpu_asic_get_reg_state {
    ($adev:expr, $state:expr, $buf:expr, $size:expr) => {
        if unsafe { !(*(*$adev).asic_funcs).get_reg_state.is_none() } {
            unsafe { (*(*$adev).asic_funcs).get_reg_state.unwrap()($adev, $state, $buf, $size) }
        } else { 0 }
    };
}

extern "C" {
    pub fn amdgpu_reg_state_sysfs_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_reg_state_sysfs_fini(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
