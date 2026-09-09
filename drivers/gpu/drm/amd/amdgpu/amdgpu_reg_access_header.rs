/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit.
use crate::{amd_hw_ip_block_type, spinlock_t};

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

pub type amdgpu_rreg_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u32) -> u32>;
pub type amdgpu_wreg_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32)>;
pub type amdgpu_rreg_ext_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u64) -> u32>;
pub type amdgpu_wreg_ext_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u64, u32)>;
pub type amdgpu_rreg64_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u32) -> u64>;
pub type amdgpu_wreg64_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u64)>;
pub type amdgpu_rreg64_ext_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u64) -> u64>;
pub type amdgpu_wreg64_ext_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u64, u64)>;

pub type amdgpu_block_rreg_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32) -> u32>;
pub type amdgpu_block_wreg_t = Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32)>;
pub type amdgpu_reg_get_smn_base64_t = Option<unsafe extern "C" fn(*mut amdgpu_device, amd_hw_ip_block_type, i32) -> u64>;

#[repr(C)]
pub struct amdgpu_reg_ind {
    pub lock: spinlock_t,
    pub rreg: amdgpu_rreg_t,
    pub wreg: amdgpu_wreg_t,
}

#[repr(C)]
pub struct amdgpu_reg_ind_blk {
    pub lock: spinlock_t,
    pub rreg: amdgpu_block_rreg_t,
    pub wreg: amdgpu_block_wreg_t,
}

#[repr(C)]
pub struct amdgpu_reg_pcie_ind {
    pub lock: spinlock_t,
    pub rreg: amdgpu_rreg_t,
    pub wreg: amdgpu_wreg_t,
    pub rreg_ext: amdgpu_rreg_ext_t,
    pub wreg_ext: amdgpu_wreg_ext_t,
    pub rreg64: amdgpu_rreg64_t,
    pub wreg64: amdgpu_wreg64_t,
    pub rreg64_ext: amdgpu_rreg64_ext_t,
    pub wreg64_ext: amdgpu_wreg64_ext_t,
    pub port_rreg: amdgpu_rreg_t,
    pub port_wreg: amdgpu_wreg_t,
}

#[repr(C)]
pub struct amdgpu_reg_smn_ext {
    pub get_smn_base: amdgpu_reg_get_smn_base64_t,
}

#[repr(C)]
pub struct amdgpu_reg_access {
    pub smc: amdgpu_reg_ind,
    pub uvd_ctx: amdgpu_reg_ind,
    pub didt: amdgpu_reg_ind,
    pub gc_cac: amdgpu_reg_ind,
    pub se_cac: amdgpu_reg_ind,
    pub audio_endpt: amdgpu_reg_ind_blk,
    pub pcie: amdgpu_reg_pcie_ind,
    pub smn: amdgpu_reg_smn_ext,
}

/* ASIC specific register table accessible by UMD */
#[repr(C)]
pub struct amdgpu_allowed_register_entry {
    pub reg_offset: u32,
    pub grbm_indexed: bool,
}

extern "C" {
    pub fn amdgpu_reg_access_init(adev: *mut amdgpu_device);
    pub fn amdgpu_reg_smc_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_reg_smc_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_reg_uvd_ctx_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_reg_uvd_ctx_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_reg_didt_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_reg_didt_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_reg_gc_cac_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_reg_gc_cac_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_reg_se_cac_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_reg_se_cac_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_reg_audio_endpt_rd32(adev: *mut amdgpu_device, block: u32, reg: u32) -> u32;
    pub fn amdgpu_reg_audio_endpt_wr32(adev: *mut amdgpu_device, block: u32, reg: u32, v: u32);
    pub fn amdgpu_reg_pcie_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_reg_pcie_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_reg_pcie_ext_rd32(adev: *mut amdgpu_device, reg: u64) -> u32;
    pub fn amdgpu_reg_pcie_ext_wr32(adev: *mut amdgpu_device, reg: u64, v: u32);
    pub fn amdgpu_reg_pcie_rd64(adev: *mut amdgpu_device, reg: u32) -> u64;
    pub fn amdgpu_reg_pcie_wr64(adev: *mut amdgpu_device, reg: u32, v: u64);
    pub fn amdgpu_reg_pcie_ext_rd64(adev: *mut amdgpu_device, reg: u64) -> u64;
    pub fn amdgpu_reg_pcie_ext_wr64(adev: *mut amdgpu_device, reg: u64, v: u64);
    pub fn amdgpu_reg_pciep_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_reg_pciep_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_reg_get_smn_base64(adev: *mut amdgpu_device, block: amd_hw_ip_block_type, die_inst: i32) -> u64;
    pub fn amdgpu_reg_smn_v1_0_get_base(adev: *mut amdgpu_device, block: amd_hw_ip_block_type, die_inst: i32) -> u64;
    pub fn amdgpu_device_rreg(adev: *mut amdgpu_device, reg: u32, acc_flags: u32) -> u32;
    pub fn amdgpu_device_xcc_rreg(adev: *mut amdgpu_device, reg: u32, acc_flags: u32, xcc_id: u32) -> u32;
    pub fn amdgpu_device_wreg(adev: *mut amdgpu_device, reg: u32, v: u32, acc_flags: u32);
    pub fn amdgpu_device_xcc_wreg(adev: *mut amdgpu_device, reg: u32, v: u32, acc_flags: u32, xcc_id: u32);
    pub fn amdgpu_mm_wreg_mmio_rlc(adev: *mut amdgpu_device, reg: u32, v: u32, xcc_id: u32);
    pub fn amdgpu_mm_wreg8(adev: *mut amdgpu_device, offset: u32, value: u8);
    pub fn amdgpu_mm_rreg8(adev: *mut amdgpu_device, offset: u32) -> u8;
    pub fn amdgpu_device_indirect_rreg(adev: *mut amdgpu_device, reg_addr: u32) -> u32;
    pub fn amdgpu_device_indirect_rreg_ext(adev: *mut amdgpu_device, reg_addr: u64) -> u32;
    pub fn amdgpu_device_indirect_rreg64(adev: *mut amdgpu_device, reg_addr: u32) -> u64;
    pub fn amdgpu_device_indirect_rreg64_ext(adev: *mut amdgpu_device, reg_addr: u64) -> u64;
    pub fn amdgpu_device_indirect_wreg(adev: *mut amdgpu_device, reg_addr: u32, reg_data: u32);
    pub fn amdgpu_device_indirect_wreg_ext(adev: *mut amdgpu_device, reg_addr: u64, reg_data: u32);
    pub fn amdgpu_device_indirect_wreg64(adev: *mut amdgpu_device, reg_addr: u32, reg_data: u64);
    pub fn amdgpu_device_indirect_wreg64_ext(adev: *mut amdgpu_device, reg_addr: u64, reg_data: u64);
    pub fn amdgpu_device_pcie_port_rreg(adev: *mut amdgpu_device, reg: u32) -> u32;
    pub fn amdgpu_device_pcie_port_wreg(adev: *mut amdgpu_device, reg: u32, v: u32);
    pub fn amdgpu_device_wait_on_rreg(adev: *mut amdgpu_device, inst: u32, reg_addr: u32, reg_name: *mut i8, expected_value: u32, mask: u32) -> u32;
    pub fn amdgpu_read_indexed_register(adev: *mut amdgpu_device, se_num: u32, sh_num: u32, reg_offset: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
