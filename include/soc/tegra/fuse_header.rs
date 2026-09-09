/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2012-2023, NVIDIA CORPORATION.  All rights reserved.
 */

// Translated from soc/tegra/fuse.h. Linux type and configuration dependencies
// are intentionally left as external Rust declarations/conditions.

pub const TEGRA20: u32 = 0x20;
pub const TEGRA30: u32 = 0x30;
pub const TEGRA114: u32 = 0x35;
pub const TEGRA124: u32 = 0x40;
pub const TEGRA132: u32 = 0x13;
pub const TEGRA210: u32 = 0x21;
pub const TEGRA186: u32 = 0x18;
pub const TEGRA194: u32 = 0x19;
pub const TEGRA234: u32 = 0x23;
pub const TEGRA241: u32 = 0x24;
pub const TEGRA264: u32 = 0x26;

pub const TEGRA_FUSE_SKU_CALIB_0: u32 = 0xf0;
pub const TEGRA30_FUSE_SATA_CALIB: u32 = 0x124;
pub const TEGRA_FUSE_USB_CALIB_EXT_0: u32 = 0x250;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tegra_revision {
    TEGRA_REVISION_UNKNOWN = 0,
    TEGRA_REVISION_A01,
    TEGRA_REVISION_A02,
    TEGRA_REVISION_A03,
    TEGRA_REVISION_A03p,
    TEGRA_REVISION_A04,
    TEGRA_REVISION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tegra_platform {
    TEGRA_PLATFORM_SILICON = 0,
    TEGRA_PLATFORM_QT,
    TEGRA_PLATFORM_SYSTEM_FPGA,
    TEGRA_PLATFORM_UNIT_FPGA,
    TEGRA_PLATFORM_ASIM_QT,
    TEGRA_PLATFORM_ASIM_LINSIM,
    TEGRA_PLATFORM_DSIM_ASIM_LINSIM,
    TEGRA_PLATFORM_VERIFICATION_SIMULATION,
    TEGRA_PLATFORM_VDK,
    TEGRA_PLATFORM_VSP,
    TEGRA_PLATFORM_MAX,
}

#[repr(C)]
pub struct tegra_sku_info {
    pub sku_id: i32,
    pub cpu_process_id: i32,
    pub cpu_speedo_id: i32,
    pub cpu_speedo_value: i32,
    pub cpu_iddq_value: i32,
    pub soc_process_id: i32,
    pub soc_speedo_id: i32,
    pub soc_speedo_value: i32,
    pub gpu_process_id: i32,
    pub gpu_speedo_id: i32,
    pub gpu_speedo_value: i32,
    pub revision: tegra_revision,
    pub platform: tegra_platform,
}

#[cfg(feature = "CONFIG_ARCH_TEGRA")]
extern "C" {
    pub static mut tegra_sku_info: tegra_sku_info;
    pub fn tegra_read_straps() -> u32;
    pub fn tegra_read_ram_code() -> u32;
    pub fn tegra_fuse_readl(offset: usize, value: *mut u32) -> i32;
    pub fn tegra_read_chipid() -> u32;
    pub fn tegra_get_chip_id() -> u8;
    pub fn tegra_get_platform() -> u8;
    pub fn tegra_is_silicon() -> bool;
    pub fn tegra194_miscreg_mask_serror() -> i32;
}

#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub static mut tegra_sku_info: tegra_sku_info = tegra_sku_info {
    sku_id: 0, cpu_process_id: 0, cpu_speedo_id: 0, cpu_speedo_value: 0,
    cpu_iddq_value: 0, soc_process_id: 0, soc_speedo_id: 0, soc_speedo_value: 0,
    gpu_process_id: 0, gpu_speedo_id: 0, gpu_speedo_value: 0,
    revision: tegra_revision::TEGRA_REVISION_UNKNOWN,
    platform: tegra_platform::TEGRA_PLATFORM_SILICON,
};

#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra_read_straps() -> u32 { 0 }
#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra_read_ram_code() -> u32 { 0 }
#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra_fuse_readl(_offset: usize, _value: *mut u32) -> i32 { -19 }
#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra_read_chipid() -> u32 { 0 }
#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra_get_chip_id() -> u8 { 0 }
#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra_get_platform() -> u8 { 0 }
#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra_is_silicon() -> bool { false }
#[cfg(not(feature = "CONFIG_ARCH_TEGRA"))]
pub unsafe fn tegra194_miscreg_mask_serror() -> i32 { 0 }

#[repr(C)]
pub struct device;

extern "C" {
    pub fn tegra_soc_device_register() -> *mut device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
