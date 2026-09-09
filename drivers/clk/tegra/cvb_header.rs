/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Utility functions for parsing Tegra CVB voltage tables
 */

// Dependency equivalent: <linux/types.h>

pub const MAX_DVFS_FREQS: usize = 40;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive Copy, Clone]
pub struct rail_alignment {
    pub offset_uv: i32,
    pub step_uv: i32,
}

#[repr(C)]
#[derive Copy, Clone]
pub struct cvb_coefficients {
    pub c0: i32,
    pub c1: i32,
    pub c2: i32,
}

#[repr(C)]
#[derive Copy, Clone]
pub struct cvb_table_freq_entry {
    pub freq: usize,
    pub coefficients: cvb_coefficients,
}

#[repr(C)]
#[derive Copy, Clone]
pub struct cvb_cpu_dfll_data {
    pub tune0_low: u32,
    pub tune0_high: u32,
    pub tune1: u32,
    pub tune_high_min_millivolts: u32,
}

#[repr(C)]
pub struct cvb_table {
    pub speedo_id: i32,
    pub process_id: i32,

    pub min_millivolts: i32,
    pub max_millivolts: i32,

    pub speedo_scale: i32,
    pub voltage_scale: i32,
    pub entries: [cvb_table_freq_entry; MAX_DVFS_FREQS],
    pub cpu_dfll_data: cvb_cpu_dfll_data,
}

unsafe extern "C" {
    pub fn tegra_cvb_add_opp_table(
        dev: *mut device,
        cvb_tables: *const cvb_table,
        count: usize,
        align: *mut rail_alignment,
        process_id: i32,
        speedo_id: i32,
        speedo_value: i32,
        max_freq: usize,
    ) -> *const cvb_table;

    pub fn tegra_cvb_remove_opp_table(
        dev: *mut device,
        table: *const cvb_table,
        max_freq: usize,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
