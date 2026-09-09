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

// Dependency declarations supplied by other translated headers.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amd_hw_ip_block_type {
    GC_HWIP = 1,
    HDP_HWIP,
    SDMA0_HWIP,
    SDMA1_HWIP,
    SDMA2_HWIP,
    SDMA3_HWIP,
    SDMA4_HWIP,
    SDMA5_HWIP,
    SDMA6_HWIP,
    SDMA7_HWIP,
    LSDMA_HWIP,
    MMHUB_HWIP,
    ATHUB_HWIP,
    NBIO_HWIP,
    MP0_HWIP,
    MP1_HWIP,
    UVD_HWIP,
    VCN_HWIP = UVD_HWIP,
    JPEG_HWIP = VCN_HWIP,
    VCN1_HWIP,
    VCE_HWIP,
    VPE_HWIP,
    DF_HWIP,
    DCE_HWIP,
    OSSSYS_HWIP,
    SMUIO_HWIP,
    PWR_HWIP,
    NBIF_HWIP,
    THM_HWIP,
    CLK_HWIP,
    UMC_HWIP,
    RSMU_HWIP,
    XGMI_HWIP,
    DCI_HWIP,
    PCIE_HWIP,
    ISP_HWIP,
    ATU_HWIP,
    AIGC_HWIP,
    UMSCH_HWIP,
    MAX_HWIP,
}

pub const HWIP_MAX_INSTANCE: usize = 48;
pub const HW_ID_MAX: u32 = 300;

#[inline]
pub const fn IP_VERSION_FULL(mj: u32, mn: u32, rv: u32, var: u32, srev: u32) -> u32 {
    (mj << 24) | (mn << 16) | (rv << 8) | (var << 4) | srev
}

#[inline]
pub const fn IP_VERSION(mj: u32, mn: u32, rv: u32) -> u32 {
    IP_VERSION_FULL(mj, mn, rv, 0, 0)
}

#[inline]
pub const fn IP_VERSION_MAJ(ver: u32) -> u32 { ver >> 24 }
#[inline]
pub const fn IP_VERSION_MIN(ver: u32) -> u32 { (ver >> 16) & 0xFF }
#[inline]
pub const fn IP_VERSION_REV(ver: u32) -> u32 { (ver >> 8) & 0xFF }
#[inline]
pub const fn IP_VERSION_VARIANT(ver: u32) -> u32 { (ver >> 4) & 0xF }
#[inline]
pub const fn IP_VERSION_SUBREV(ver: u32) -> u32 { ver & 0xF }
#[inline]
pub const fn IP_VERSION_MAJ_MIN_REV(ver: u32) -> u32 { ver >> 8 }

#[repr(C)]
pub struct amdgpu_device;
#[repr(C)]
pub struct amdgpu_ring;
#[repr(C)]
pub struct amdgpu_fence;
pub enum amd_ip_block_type {}
pub enum amd_ip_funcs {}
pub enum amd_clockgating_state {}
pub enum amd_powergating_state {}

pub const AMDGPU_MAX_IP_NUM: usize = AMD_IP_BLOCK_TYPE_NUM;

#[repr(C)]
pub struct amdgpu_ip_map_info {
    pub dev_inst: [[u32; HWIP_MAX_INSTANCE]; MAX_HWIP as usize],
    pub logical_to_dev_inst: Option<unsafe extern "C" fn(*mut amdgpu_device, amd_hw_ip_block_type, i8) -> i8>,
    pub logical_to_dev_mask: Option<unsafe extern "C" fn(*mut amdgpu_device, amd_hw_ip_block_type, u32) -> u32>,
}

#[repr(C)]
pub struct amdgpu_ip_block_status {
    pub valid: bool,
    pub sw: bool,
    pub hw: bool,
    pub late_initialized: bool,
    pub hang: bool,
}

#[repr(C)]
pub struct amdgpu_ip_block_version {
    pub type_: amd_ip_block_type,
    pub major: u32,
    pub minor: u32,
    pub rev: u32,
    pub funcs: *const amd_ip_funcs,
}

#[repr(C)]
pub struct amdgpu_ip_block {
    pub status: amdgpu_ip_block_status,
    pub version: *const amdgpu_ip_block_version,
    pub adev: *mut amdgpu_device,
}

unsafe extern "C" {
    pub fn amdgpu_ip_map_init(adev: *mut amdgpu_device);
    pub fn amdgpu_ip_block_suspend(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn amdgpu_ip_block_resume(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn amdgpu_device_ip_get_ip_block(adev: *mut amdgpu_device, type_: amd_ip_block_type) -> *mut amdgpu_ip_block;
    pub fn amdgpu_device_ip_block_version_cmp(adev: *mut amdgpu_device, type_: amd_ip_block_type, major: u32, minor: u32) -> i32;
    pub fn amdgpu_device_ip_block_add(adev: *mut amdgpu_device, ip_block_version: *const amdgpu_ip_block_version) -> i32;
    pub fn amdgpu_device_ip_set_clockgating_state(adev: *mut amdgpu_device, block_type: amd_ip_block_type, state: amd_clockgating_state) -> i32;
    pub fn amdgpu_device_ip_set_powergating_state(adev: *mut amdgpu_device, block_type: amd_ip_block_type, state: amd_powergating_state) -> i32;
    pub fn amdgpu_device_ip_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64);
    pub fn amdgpu_device_ip_wait_for_idle(adev: *mut amdgpu_device, block_type: amd_ip_block_type) -> i32;
    pub fn amdgpu_device_ip_is_valid(adev: *mut amdgpu_device, block_type: amd_ip_block_type) -> bool;
    pub fn amdgpu_device_ip_soft_reset(guilty_ring: *mut amdgpu_ring, guilty_fence: *mut amdgpu_fence) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
