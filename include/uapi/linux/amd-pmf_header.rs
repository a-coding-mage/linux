/* SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note */
/*
 * AMD Platform Management Framework (PMF) UAPI Header
 *
 * Copyright (c) 2026, Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * This file defines the user-space API for interacting with the AMD PMF
 * driver. It provides ioctl interfaces to query platform-specific metrics
 * such as power source, slider position, platform type, laptop placement,
 * and various BIOS input/output parameters.
 */

// C header dependencies: linux/bits.h, linux/ioctl.h, linux/types.h

/**
 * AMD_PMF_IOC_MAGIC - Magic number for AMD PMF ioctl commands
 *
 * This magic number uniquely identifies AMD PMF ioctl operations.
 */
pub const AMD_PMF_IOC_MAGIC: u8 = b'p';

/**
 * IOCTL_AMD_PMF_POPULATE_DATA - ioctl command to retrieve PMF metrics data
 *
 * This ioctl command is used to populate the amd_pmf_info structure
 * with the requested PMF metrics information.
 */
// Equivalent to _IOWR(AMD_PMF_IOC_MAGIC, 0x00, __u64); ioctl encoding is
// supplied by the target platform's Linux ioctl definitions.
pub const IOCTL_AMD_PMF_POPULATE_DATA: u64 =
    ((2u64 << 30) | ((core::mem::size_of::<u64>() as u64) << 16)
        | ((AMD_PMF_IOC_MAGIC as u64) << 8));

pub const AMD_PMF_BIOS_PARAMS_MAX: usize = 10;

/* AMD PMF feature flags - bitmask indicating supported features */
pub const AMD_PMF_FEAT_AUTO_MODE: u32 = 1u32 << 0;
pub const AMD_PMF_FEAT_STATIC_POWER_SLIDER: u32 = 1u32 << 1;
pub const AMD_PMF_FEAT_POLICY_BUILDER: u32 = 1u32 << 2;
pub const AMD_PMF_FEAT_DYNAMIC_POWER_SLIDER_AC: u32 = 1u32 << 3;
pub const AMD_PMF_FEAT_DYNAMIC_POWER_SLIDER_DC: u32 = 1u32 << 4;

/** Describes the physical placement of the laptop. */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amd_pmf_laptop_placement {
    AMD_PMF_LP_UNKNOWN,
    AMD_PMF_ON_TABLE,
    AMD_PMF_ON_LAP_MOTION,
    AMD_PMF_IN_BAG,
    AMD_PMF_OUT_OF_BAG,
    AMD_PMF_LP_UNDEFINED,
}

/** Trusted Application power slider positions. */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amd_pmf_ta_slider {
    AMD_PMF_TA_BEST_BATTERY,
    AMD_PMF_TA_BETTER_BATTERY,
    AMD_PMF_TA_BETTER_PERFORMANCE,
    AMD_PMF_TA_BEST_PERFORMANCE,
    AMD_PMF_TA_MAX,
}

/** Describes the physical form factor orientation. */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amd_pmf_platform_type {
    AMD_PMF_PTYPE_UNKNOWN,
    AMD_PMF_LID_CLOSE,
    AMD_PMF_CLAMSHELL,
    AMD_PMF_FLAT,
    AMD_PMF_TENT,
    AMD_PMF_STAND,
    AMD_PMF_TABLET,
    AMD_PMF_BOOK,
    AMD_PMF_PRESENTATION,
    AMD_PMF_PULL_FWD,
    AMD_PMF_PTYPE_INVALID = 0xf,
}

pub unsafe fn amd_pmf_get_platform_type(platform_type: u32) -> &'static core::ffi::CStr {
    match platform_type {
        2 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"CLAMSHELL\0") },
        1 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"LID_CLOSE\0") },
        3 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"FLAT\0") },
        4 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"TENT\0") },
        5 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"STAND\0") },
        6 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"TABLET\0") },
        7 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"BOOK\0") },
        8 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"PRESENTATION\0") },
        9 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"PULL_FWD\0") },
        _ => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"UNKNOWN\0") },
    }
}

pub unsafe fn amd_pmf_get_laptop_placement(device_state: u32) -> &'static core::ffi::CStr {
    match device_state {
        1 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"ON_TABLE\0") },
        2 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"ON_LAP_MOTION\0") },
        3 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"IN_BAG\0") },
        4 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"OUT_OF_BAG\0") },
        _ => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"UNKNOWN\0") },
    }
}

pub unsafe fn amd_pmf_get_slider_position(state: u32) -> &'static core::ffi::CStr {
    match state {
        3 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"PERFORMANCE\0") },
        2 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"BALANCED\0") },
        0 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"POWER_SAVER\0") },
        1 => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"BALANCED_BATTERY\0") },
        _ => unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"Unknown TA Slider State\0") },
    }
}

#[repr(C)]
pub struct amd_pmf_info {
    pub size: u64,
    pub features_supported: u32,
    pub platform_type: u32,
    pub power_source: u32,
    pub laptop_placement: u32,
    pub lid_state: u32,
    pub user_presence: u32,
    pub slider_position: u32,
    pub skin_temp: i32,
    pub gfx_busy: u32,
    pub ambient_light: i32,
    pub avg_c0_residency: u32,
    pub max_c0_residency: u32,
    pub socket_power: u32,
    pub bios_input: [u32; AMD_PMF_BIOS_PARAMS_MAX],
    pub bios_output: [u32; AMD_PMF_BIOS_PARAMS_MAX],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
