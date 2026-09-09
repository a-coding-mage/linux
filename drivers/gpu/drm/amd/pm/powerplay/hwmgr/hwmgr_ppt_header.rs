/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding C headers: hardwaremanager.h,
// smumgr.h, and atom-types.h.

#[repr(C)]
pub struct phm_ppt_v1_clock_voltage_dependency_record {
    pub clk: u32,
    pub vddInd: u8,
    pub vddciInd: u8,
    pub mvddInd: u8,
    pub vdd_offset: u16,
    pub vddc: u16,
    pub vddgfx: u16,
    pub vddci: u16,
    pub mvdd: u16,
    pub phases: u8,
    pub cks_enable: u8,
    pub cks_voffset: u8,
    pub sclk_offset: u32,
}

#[repr(C)]
pub struct phm_ppt_v1_clock_voltage_dependency_table {
    pub count: u32, // Number of entries.
    // Dynamically allocate count entries.
    pub entries: [phm_ppt_v1_clock_voltage_dependency_record; 0],
}

/* Multimedia Clock Voltage Dependency records and table */
#[repr(C)]
pub struct phm_ppt_v1_mm_clock_voltage_dependency_record {
    pub dclk: u32, // UVD D-clock
    pub vclk: u32, // UVD V-clock
    pub eclk: u32, // VCE clock
    pub aclk: u32, // ACP clock
    pub samclock: u32, // SAMU clock
    pub vddcInd: u8,
    pub vddgfx_offset: u16,
    pub vddc: u16,
    pub vddgfx: u16,
    pub phases: u8,
}

#[repr(C)]
pub struct phm_ppt_v1_mm_clock_voltage_dependency_table {
    pub count: u32, // Number of entries.
    // Dynamically allocate count entries.
    pub entries: [phm_ppt_v1_mm_clock_voltage_dependency_record; 0],
}

#[repr(C)]
pub struct phm_ppt_v1_voltage_lookup_record {
    pub us_calculated: u16,
    pub us_vdd: u16, // Base voltage
    pub us_cac_low: u16,
    pub us_cac_mid: u16,
    pub us_cac_high: u16,
}

#[repr(C)]
pub struct phm_ppt_v1_voltage_lookup_table {
    pub count: u32,
    // Dynamically allocate count entries.
    pub entries: [phm_ppt_v1_voltage_lookup_record; 0],
}

/* PCIE records and Table */
#[repr(C)]
pub struct phm_ppt_v1_pcie_record {
    pub gen_speed: u8,
    pub lane_width: u8,
    pub usreserved: u16,
    pub reserved: u16,
    pub pcie_sclk: u32,
}

#[repr(C)]
pub struct phm_ppt_v1_pcie_table {
    pub count: u32, // Number of entries.
    // Dynamically allocate count entries.
    pub entries: [phm_ppt_v1_pcie_record; 0],
}

#[repr(C)]
pub struct phm_ppt_v1_gpio_table {
    // SCLK DPM level index to switch to when VRHot is triggered
    pub vrhot_triggered_sclk_dpm_index: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
