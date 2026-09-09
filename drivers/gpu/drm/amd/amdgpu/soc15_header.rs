/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit:
// nbio_v6_1.h, nbio_v7_0.h, nbio_v7_4.h, amdgpu_reg_state.h

extern "C" {
    pub static vega10_common_ip_block: amdgpu_ip_block_version;
}

pub const SOC15_FLUSH_GPU_TLB_NUM_WREG: u32 = 6;
pub const SOC15_FLUSH_GPU_TLB_NUM_REG_WAIT: u32 = 3;

#[repr(C)]
pub struct soc15_reg_golden {
    pub hwip: u32,
    pub instance: u32,
    pub segment: u32,
    pub reg: u32,
    pub and_mask: u32,
    pub or_mask: u32,
}

#[repr(C)]
pub struct soc15_reg_rlcg {
    pub hwip: u32,
    pub instance: u32,
    pub segment: u32,
    pub reg: u32,
}

#[repr(C)]
pub struct soc15_reg {
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
}

#[repr(C)]
pub struct soc15_reg_entry {
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub reg_value: u32,
    pub se_num: u32,
    pub instance: u32,
}

#[repr(C)]
pub struct soc15_allowed_register_entry {
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub grbm_indexed: bool,
}

#[repr(C)]
pub struct soc15_ras_field_entry {
    pub name: *const core::ffi::c_char,
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub sec_count_mask: u32,
    pub sec_count_shift: u32,
    pub ded_count_mask: u32,
    pub ded_count_shift: u32,
}

// C preprocessor forms preserved as Rust macro equivalents. The caller
// supplies the already-expanded hardware and register constants.
#[macro_export]
macro_rules! SOC15_REG_ENTRY {
    ($ip_hwip:expr, $inst:expr, $reg_base_idx:expr, $reg:expr) => {
        ($ip_hwip, $inst, $reg_base_idx, $reg)
    };
}

#[macro_export]
macro_rules! SOC15_REG_ENTRY_STR {
    ($ip_hwip:expr, $inst:expr, $reg_base_idx:expr, $reg:expr, $reg_name:expr) => {
        ($ip_hwip, $inst, $reg_base_idx, $reg, $reg_name)
    };
}

#[macro_export]
macro_rules! SOC15_REG_ENTRY_OFFSET {
    ($adev:expr, $entry:expr) => {
        $adev.reg_offset[$entry.hwip as usize][$entry.inst as usize]
            [$entry.seg as usize] + $entry.reg_offset
    };
}

// Over ride the instance id.
#[macro_export]
macro_rules! SOC15_REG_ENTRY_OFFSET_INST {
    ($adev:expr, $entry:expr, $inst:expr) => {
        $adev.reg_offset[$entry.hwip as usize][$inst as usize]
            [$entry.seg as usize] + $entry.reg_offset
    };
}

#[macro_export]
macro_rules! SOC15_REG_GOLDEN_VALUE {
    ($ip_hwip:expr, $inst:expr, $reg_base_idx:expr, $reg:expr, $and_mask:expr, $or_mask:expr) => {
        ($ip_hwip, $inst, $reg_base_idx, $reg, $and_mask, $or_mask)
    };
}

#[macro_export]
macro_rules! SOC15_REG_FIELD {
    ($mask:expr, $shift:expr) => { ($mask, $shift) };
}

#[macro_export]
macro_rules! SOC15_REG_FIELD_VAL {
    ($val:expr, $mask:expr, $shift:expr) => { (($val & $mask) >> $shift) };
}

#[macro_export]
macro_rules! SOC15_RAS_REG_FIELD_VAL {
    ($val:expr, $entry:expr, $field_mask:ident, $field_shift:ident) => {
        SOC15_REG_FIELD_VAL!($val, $entry.$field_mask, $entry.$field_shift)
    };
}

extern "C" {
    pub fn soc15_grbm_select(adev: *mut amdgpu_device, me: u32, pipe: u32,
                             queue: u32, vmid: u32, xcc_id: core::ffi::c_int);
    pub fn soc15_set_virt_ops(adev: *mut amdgpu_device);
    pub fn soc15_program_register_sequence(
        adev: *mut amdgpu_device,
        registers: *const soc15_reg_golden,
        array_size: u32,
    );

    pub fn vega10_reg_base_init(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn vega20_reg_base_init(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn arct_reg_base_init(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn aldebaran_reg_base_init(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn aqua_vanjaram_init_soc_config(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn aqua_vanjaram_get_reg_state(
        adev: *mut amdgpu_device,
        reg_state: amdgpu_reg_state,
        buf: *mut core::ffi::c_void,
        max_size: usize,
    ) -> isize;

    pub fn vega10_doorbell_index_init(adev: *mut amdgpu_device);
    pub fn vega20_doorbell_index_init(adev: *mut amdgpu_device);
    pub fn aqua_vanjaram_doorbell_index_init(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
