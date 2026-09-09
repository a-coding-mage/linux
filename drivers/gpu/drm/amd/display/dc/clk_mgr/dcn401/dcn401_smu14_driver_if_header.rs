// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.
//
// This is a stripped-down version of the smu13_driver_if.h file for the relevant DAL interfaces.

pub const SMU14_DRIVER_IF_VERSION: u32 = 0x1;

// Only Clks that have DPM descriptors are listed here
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PPCLK_e {
    PPCLK_GFXCLK = 0,
    PPCLK_SOCCLK,
    PPCLK_UCLK,
    PPCLK_FCLK,
    PPCLK_DCLK_0,
    PPCLK_VCLK_0,
    PPCLK_DISPCLK,
    PPCLK_DPPCLK,
    PPCLK_DPREFCLK,
    PPCLK_DCFCLK,
    PPCLK_DTBCLK,
    PPCLK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WatermarkRowGeneric_t {
    pub WmSetting: u8,
    pub Flags: u8,
    pub Padding: [u8; 2],
}

pub const NUM_WM_RANGES: usize = 4;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WATERMARKS_FLAGS_e {
    WATERMARKS_CLOCK_RANGE = 0,
    WATERMARKS_DUMMY_PSTATE,
    WATERMARKS_MALL,
    WATERMARKS_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Watermarks_t {
    // Watermarks
    pub WatermarkRow: [WatermarkRowGeneric_t; NUM_WM_RANGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WatermarksExternal_t {
    pub Watermarks: Watermarks_t,
    pub Spare: [u32; 16],
    pub MmHubPadding: [u32; 8], // SMU internal use
}

// Table types
pub const TABLE_PMFW_PPTABLE: u32 = 0;
pub const TABLE_COMBO_PPTABLE: u32 = 1;
pub const TABLE_WATERMARKS: u32 = 2;
pub const TABLE_AVFS_PSM_DEBUG: u32 = 3;
pub const TABLE_PMSTATUSLOG: u32 = 4;
pub const TABLE_SMU_METRICS: u32 = 5;
pub const TABLE_DRIVER_SMU_CONFIG: u32 = 6;
pub const TABLE_ACTIVITY_MONITOR_COEFF: u32 = 7;
pub const TABLE_OVERDRIVE: u32 = 8;
pub const TABLE_I2C_COMMANDS: u32 = 9;
pub const TABLE_DRIVER_INFO: u32 = 10;
pub const TABLE_ECCINFO: u32 = 11;
pub const TABLE_COUNT: u32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
