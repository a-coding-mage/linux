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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

#[repr(C)]
pub struct atom_clock_dividers {
    pub post_div: u32,
    pub fb_div: u32,
    pub ref_div: u32,
    pub enable_post_div: bool,
    pub enable_dithen: bool,
    pub vco_mode: u32,
    pub real_clock: u32,
    /* added for CI */
    pub post_divider: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct atom_mpll_param {
    pub fb_div: u32,
    pub post_div: u32,
    pub bwcntl: u32,
    pub dll_speed: u32,
    pub vco_mode: u32,
    pub yclk_sel: u32,
    pub qdr: u32,
    pub half_rate: u32,
}

pub const MEM_TYPE_GDDR5: u32 = 0x50;
pub const MEM_TYPE_GDDR4: u32 = 0x40;
pub const MEM_TYPE_GDDR3: u32 = 0x30;
pub const MEM_TYPE_DDR2: u32 = 0x20;
pub const MEM_TYPE_GDDR1: u32 = 0x10;
pub const MEM_TYPE_DDR3: u32 = 0xb0;
pub const MEM_TYPE_MASK: u32 = 0xf0;

#[repr(C)]
pub struct atom_memory_info {
    pub mem_vendor: u8,
    pub mem_type: u8,
}

pub const MAX_AC_TIMING_ENTRIES: usize = 16;

#[repr(C)]
pub struct atom_memory_clock_range_table {
    pub num_entries: u8,
    pub rsv: [u8; 3],
    pub mclk: [u32; MAX_AC_TIMING_ENTRIES],
}

pub const VBIOS_MC_REGISTER_ARRAY_SIZE: usize = 32;
pub const VBIOS_MAX_AC_TIMING_ENTRIES: usize = 20;

#[repr(C)]
pub struct atom_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
pub struct atom_mc_register_address {
    pub s1: u16,
    pub pre_reg_data: u8,
}

#[repr(C)]
pub struct atom_mc_reg_table {
    pub last: u8,
    pub num_entries: u8,
    pub mc_reg_table_entry: [atom_mc_reg_entry; VBIOS_MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [atom_mc_register_address; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

pub const MAX_VOLTAGE_ENTRIES: usize = 32;

#[repr(C)]
pub struct atom_voltage_table_entry {
    pub value: u16,
    pub smio_low: u32,
}

#[repr(C)]
pub struct atom_voltage_table {
    pub count: u32,
    pub mask_low: u32,
    pub phase_delay: u32,
    pub entries: [atom_voltage_table_entry; MAX_VOLTAGE_ENTRIES],
}

extern "C" {
    pub fn amdgpu_atombios_lookup_gpio(adev: *mut amdgpu_device, id: u8) -> amdgpu_gpio_rec;
    pub fn amdgpu_atombios_lookup_i2c_gpio(adev: *mut amdgpu_device, id: u8) -> amdgpu_i2c_bus_rec;
    pub fn amdgpu_atombios_i2c_init(adev: *mut amdgpu_device);
    pub fn amdgpu_atombios_oem_i2c_init(adev: *mut amdgpu_device, i2c_id: u8);
    pub fn amdgpu_atombios_has_dce_engine_info(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_atombios_get_connector_info_from_object_table(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_atombios_get_clock_info(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atombios_get_gfx_info(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atombios_get_vram_width(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atombios_get_asic_ss_info(adev: *mut amdgpu_device, ss: *mut amdgpu_atom_ss, id: i32, clock: u32) -> bool;
    pub fn amdgpu_atombios_get_clock_dividers(adev: *mut amdgpu_device, clock_type: u8, clock: u32, strobe_mode: bool, dividers: *mut atom_clock_dividers) -> i32;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_get_memory_pll_dividers(adev: *mut amdgpu_device, clock: u32, strobe_mode: bool, mpll_param: *mut atom_mpll_param) -> i32;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_set_engine_dram_timings(adev: *mut amdgpu_device, eng_clock: u32, mem_clock: u32) -> i32;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_is_voltage_gpio(adev: *mut amdgpu_device, voltage_type: u8, voltage_mode: u8) -> bool;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_get_voltage_table(adev: *mut amdgpu_device, voltage_type: u8, voltage_mode: u8, voltage_table: *mut atom_voltage_table) -> i32;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_init_mc_reg_table(adev: *mut amdgpu_device, module_index: u8, reg_table: *mut atom_mc_reg_table) -> i32;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_get_max_vddc(adev: *mut amdgpu_device, voltage_type: u8, voltage_id: u16, voltage: *mut u16) -> i32;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_get_leakage_vddc_based_on_leakage_idx(adev: *mut amdgpu_device, voltage: *mut u16, leakage_idx: u16) -> i32;
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_get_default_voltages(adev: *mut amdgpu_device, vddc: *mut u16, vddci: *mut u16, mvdd: *mut u16);
    #[cfg(feature = "CONFIG_DRM_AMDGPU_SI")]
    pub fn amdgpu_atombios_get_svi2_info(adev: *mut amdgpu_device, voltage_type: u8, svd_gpio_id: *mut u8, svc_gpio_id: *mut u8) -> i32;
    pub fn amdgpu_atombios_has_gpu_virtualization_table(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_atombios_scratch_regs_lock(adev: *mut amdgpu_device, lock: bool);
    pub fn amdgpu_atombios_scratch_regs_engine_hung(adev: *mut amdgpu_device, hung: bool);
    pub fn amdgpu_atombios_scratch_regs_set_backlight_level(adev: *mut amdgpu_device, backlight_level: u32);
    pub fn amdgpu_atombios_scratch_need_asic_init(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_atombios_copy_swap(dst: *mut u8, src: *mut u8, num_bytes: u8, to_le: bool);
    pub fn amdgpu_atombios_get_data_table(adev: *mut amdgpu_device, table: u32, size: *mut u16, frev: *mut u8, crev: *mut u8, addr: *mut *mut u8) -> i32;
    pub fn amdgpu_atombios_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_atombios_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atombios_sysfs_init(adev: *mut amdgpu_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
