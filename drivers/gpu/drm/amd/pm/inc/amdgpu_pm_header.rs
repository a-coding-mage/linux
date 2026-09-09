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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

#[repr(C)]
pub struct cg_flag_name {
    pub flag: u64,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_device_attr_flags {
    ATTR_FLAG_BASIC = 1 << 0,
    ATTR_FLAG_ONEVF = 1 << 16,
}

pub const ATTR_FLAG_TYPE_MASK: u32 = 0x0000ffff;
pub const ATTR_FLAG_MODE_MASK: u32 = 0xffff0000;
pub const ATTR_FLAG_MASK_ALL: u32 = 0xffffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_device_attr_states {
    ATTR_STATE_UNSUPPORTED = 0,
    ATTR_STATE_SUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_device_attr_id {
    device_attr_id__unknown = -1,
    device_attr_id__power_dpm_state = 0,
    device_attr_id__power_dpm_force_performance_level,
    device_attr_id__pp_num_states,
    device_attr_id__pp_cur_state,
    device_attr_id__pp_force_state,
    device_attr_id__pp_table,
    device_attr_id__pp_dpm_sclk,
    device_attr_id__pp_dpm_mclk,
    device_attr_id__pp_dpm_socclk,
    device_attr_id__pp_dpm_fclk,
    device_attr_id__pp_dpm_vclk,
    device_attr_id__pp_dpm_vclk1,
    device_attr_id__pp_dpm_dclk,
    device_attr_id__pp_dpm_dclk1,
    device_attr_id__pp_dpm_dcefclk,
    device_attr_id__pp_dpm_pcie,
    device_attr_id__pp_sclk_od,
    device_attr_id__pp_mclk_od,
    device_attr_id__pp_power_profile_mode,
    device_attr_id__pp_od_clk_voltage,
    device_attr_id__gpu_busy_percent,
    device_attr_id__mem_busy_percent,
    device_attr_id__vcn_busy_percent,
    device_attr_id__pcie_bw,
    device_attr_id__pp_features,
    device_attr_id__unique_id,
    device_attr_id__thermal_throttling_logging,
    device_attr_id__apu_thermal_cap,
    device_attr_id__gpu_metrics,
    device_attr_id__smartshift_apu_power,
    device_attr_id__smartshift_dgpu_power,
    device_attr_id__smartshift_bias,
    device_attr_id__pm_metrics,
    device_attr_id__count,
}

#[repr(C)]
pub struct amdgpu_device_attr {
    pub dev_attr: device_attribute,
    pub attr_id: amdgpu_device_attr_id,
    pub flags: amdgpu_device_attr_flags,
    pub attr_update: Option<unsafe extern "C" fn(
        *mut amdgpu_device,
        *mut amdgpu_device_attr,
        u32,
        *mut amdgpu_device_attr_states,
    ) -> i32>,
}

#[repr(C)]
pub struct amdgpu_device_attr_entry {
    pub entry: list_head,
    pub attr: *mut amdgpu_device_attr,
}

// C macro equivalent; `container_of` is supplied by the surrounding bindings.
#[macro_export]
macro_rules! to_amdgpu_device_attr {
    ($dev_attr:expr) => { container_of!($dev_attr, amdgpu_device_attr, dev_attr) };
}

// The following attribute-construction macros preserve the C macro interface;
// __ATTR, S_IRUGO, S_IWUSR, and the generated callbacks are external symbols.
#[macro_export]
macro_rules! __AMDGPU_DEVICE_ATTR {
    ($name:ident, $mode:expr, $show:expr, $store:expr, $flags:expr $(, $extra:expr)* $(,)?) => {
        { dev_attr: __ATTR!($name, $mode, $show, $store),
          attr_id: device_attr_id__unknown,
          flags: $flags $(, $extra)* }
    };
}

#[macro_export]
macro_rules! AMDGPU_DEVICE_ATTR {
    ($name:ident, $mode:expr, $flags:expr $(, $extra:expr)* $(,)?) => {
        __AMDGPU_DEVICE_ATTR!($name, $mode, amdgpu_get_$name, amdgpu_set_$name, $flags $(, $extra)*)
    };
}

#[macro_export]
macro_rules! AMDGPU_DEVICE_ATTR_RW {
    ($name:ident, $flags:expr $(, $extra:expr)* $(,)?) => {
        AMDGPU_DEVICE_ATTR!($name, S_IRUGO | S_IWUSR, $flags $(, $extra)*)
    };
}

#[macro_export]
macro_rules! AMDGPU_DEVICE_ATTR_RO {
    ($name:ident, $flags:expr $(, $extra:expr)* $(,)?) => {
        __AMDGPU_DEVICE_ATTR!($name, S_IRUGO, amdgpu_get_$name, NULL, $flags $(, $extra)*)
    };
}

extern "C" {
    pub fn amdgpu_pm_sysfs_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_pm_virt_sysfs_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_pm_sysfs_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_pm_virt_sysfs_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_debugfs_pm_init(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
