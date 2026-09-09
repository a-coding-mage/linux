/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

#[repr(C)]
pub struct pp_atomctrl_voltage_table;
#[repr(C)]
pub struct pp_hwmgr;
#[repr(C)]
pub struct phm_ppt_v1_voltage_lookup_table;
#[repr(C)]
pub struct Watermarks_t;
#[repr(C)]
pub struct pp_wm_sets_with_clock_ranges_soc15;
#[repr(C)]
pub struct dm_pp_wm_sets_with_clock_ranges_soc15;
#[repr(C)]
pub struct phm_ppt_v1_clock_voltage_dependency_table;
#[repr(C)]
pub struct amdgpu_device;
#[repr(C)]
pub struct amdgpu_irq_src;
#[repr(C)]
pub struct amdgpu_iv_entry;

extern "C" {
    pub fn convert_to_vid(vddc: u16) -> u8;
    pub fn convert_to_vddc(vid: u8) -> u16;
}

#[repr(C)]
pub struct watermark_row_generic_t {
    pub MinClock: u16,
    pub MaxClock: u16,
    pub MinUclk: u16,
    pub MaxUclk: u16,
    pub WmSetting: u8,
    pub Padding: [u8; 3],
}

#[repr(C)]
pub struct watermarks {
    pub WatermarkRow: [[watermark_row_generic_t; 4]; 2],
    pub padding: [u32; 7],
}

extern "C" {
    pub fn phm_copy_clock_limits_array(hwmgr: *mut pp_hwmgr, pptable_info_array: *mut *mut u32, pptable_array: *const u32, power_saving_clock_count: u32) -> i32;
    pub fn phm_copy_overdrive_settings_limits_array(hwmgr: *mut pp_hwmgr, pptable_info_array: *mut *mut u32, pptable_array: *const u32, od_setting_count: u32) -> i32;
    pub fn phm_wait_for_register_unequal(hwmgr: *mut pp_hwmgr, index: u32, value: u32, mask: u32) -> i32;
    pub fn phm_wait_for_indirect_register_unequal(hwmgr: *mut pp_hwmgr, indirect_port: u32, index: u32, value: u32, mask: u32) -> i32;
    pub fn phm_cf_want_uvd_power_gating(hwmgr: *mut pp_hwmgr) -> bool;
    pub fn phm_cf_want_vce_power_gating(hwmgr: *mut pp_hwmgr) -> bool;
    pub fn phm_cf_want_microcode_fan_ctrl(hwmgr: *mut pp_hwmgr) -> bool;
    pub fn phm_trim_voltage_table(vol_table: *mut pp_atomctrl_voltage_table) -> i32;
    pub fn phm_get_svi2_mvdd_voltage_table(vol_table: *mut pp_atomctrl_voltage_table, dep_table: *mut phm_ppt_v1_clock_voltage_dependency_table) -> i32;
    pub fn phm_get_svi2_vddci_voltage_table(vol_table: *mut pp_atomctrl_voltage_table, dep_table: *mut phm_ppt_v1_clock_voltage_dependency_table) -> i32;
    pub fn phm_get_svi2_vdd_voltage_table(vol_table: *mut pp_atomctrl_voltage_table, lookup_table: *mut phm_ppt_v1_voltage_lookup_table) -> i32;
    pub fn phm_trim_voltage_table_to_fit_state_table(max_vol_steps: u32, vol_table: *mut pp_atomctrl_voltage_table);
    pub fn phm_reset_single_dpm_table(table: *mut core::ffi::c_void, count: u32, max: i32) -> i32;
    pub fn phm_setup_pcie_table_entry(table: *mut core::ffi::c_void, index: u32, pcie_gen: u32, pcie_lanes: u32);
    pub fn phm_get_dpm_level_enable_mask_value(table: *mut core::ffi::c_void) -> i32;
    pub fn phm_get_voltage_id(voltage_table: *mut pp_atomctrl_voltage_table, voltage: u32) -> u8;
    pub fn phm_get_voltage_index(lookup_table: *mut phm_ppt_v1_voltage_lookup_table, voltage: u16) -> u8;
    pub fn phm_find_closest_vddci(vddci_table: *mut pp_atomctrl_voltage_table, vddci: u16) -> u16;
    pub fn phm_find_boot_level(table: *mut core::ffi::c_void, value: u32, boot_level: *mut u32) -> i32;
    pub fn phm_get_sclk_for_voltage_evv(hwmgr: *mut pp_hwmgr, lookup_table: *mut phm_ppt_v1_voltage_lookup_table, virtual_voltage_id: u16, sclk: *mut i32) -> i32;
    pub fn phm_get_lowest_enabled_level(hwmgr: *mut pp_hwmgr, mask: u32) -> u32;
    pub fn phm_get_voltage_evv_on_sclk(hwmgr: *mut pp_hwmgr, voltage_type: u8, sclk: u32, id: u16, voltage: *mut u16) -> i32;
    pub fn phm_set_field_to_u32(offset: u32, original_data: u32, field: u32, size: u32) -> u32;
    pub fn phm_wait_on_register(hwmgr: *mut pp_hwmgr, index: u32, value: u32, mask: u32) -> i32;
    pub fn phm_wait_on_indirect_register(hwmgr: *mut pp_hwmgr, indirect_port: u32, index: u32, value: u32, mask: u32) -> i32;
    pub fn phm_irq_process(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32;
    pub fn smu9_register_irq_handlers(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu_atom_get_data_table(dev: *mut core::ffi::c_void, table: u32, size: *mut u16, frev: *mut u8, crev: *mut u8) -> *mut core::ffi::c_void;
    pub fn smu_get_voltage_dependency_table_ppt_v1(allowed_dep_table: *const phm_ppt_v1_clock_voltage_dependency_table, dep_table: *mut phm_ppt_v1_clock_voltage_dependency_table) -> i32;
    pub fn smu_set_watermarks_for_clocks_ranges(wt_table: *mut core::ffi::c_void, wm_with_clock_ranges: *mut dm_pp_wm_sets_with_clock_ranges_soc15) -> i32;
}

/* The following C macros use token pasting (reg##field), which has no stable
 * direct Rust equivalent without the register definitions. Their exact
 * source-level forms are preserved here as dependency-facing intent:
 * PHM_FIELD_SHIFT, PHM_FIELD_MASK, PHM_SET_FIELD, PHM_GET_FIELD,
 * PHM_READ_FIELD, PHM_READ_INDIRECT_FIELD, PHM_READ_VFPF_INDIRECT_FIELD,
 * PHM_WRITE_FIELD, PHM_WRITE_INDIRECT_FIELD, PHM_WRITE_VFPF_INDIRECT_FIELD,
 * PHM_WAIT_INDIRECT_REGISTER_GIVEN_INDEX, PHM_WAIT_INDIRECT_REGISTER,
 * PHM_WAIT_INDIRECT_FIELD, PHM_WAIT_INDIRECT_REGISTER_UNEQUAL_GIVEN_INDEX,
 * PHM_WAIT_INDIRECT_REGISTER_UNEQUAL, PHM_WAIT_INDIRECT_FIELD_UNEQUAL,
 * PHM_WAIT_VFPF_INDIRECT_REGISTER_UNEQUAL_GIVEN_INDEX,
 * PHM_WAIT_VFPF_INDIRECT_REGISTER_UNEQUAL, PHM_WAIT_VFPF_INDIRECT_FIELD_UNEQUAL,
 * PHM_WAIT_VFPF_INDIRECT_REGISTER_GIVEN_INDEX, PHM_WAIT_VFPF_INDIRECT_REGISTER,
 * PHM_WAIT_VFPF_INDIRECT_FIELD, PHM_WAIT_REGISTER_UNEQUAL_GIVEN_INDEX,
 * PHM_WAIT_REGISTER_UNEQUAL, and PHM_WAIT_FIELD_UNEQUAL. */

/* Helper function to make sysfs_emit_at() happy. */
#[inline]
pub unsafe fn phm_get_sysfs_buf(buf: *mut *mut i8, offset: *mut i32) {
    if (*buf).is_null() || offset.is_null() { return; }
    *offset = offset_in_page(*buf);
    *buf = (*buf).offset(-(*offset as isize));
}

extern "C" { fn offset_in_page(buf: *mut i8) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
