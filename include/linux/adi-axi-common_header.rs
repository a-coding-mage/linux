/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Analog Devices AXI common registers & definitions
 *
 * Copyright 2019 Analog Devices Inc.
 *
 * https://wiki.analog.com/resources/fpga/docs/axi_ip
 * https://wiki.analog.com/resources/fpga/docs/hdl/regmap
 */

// Dependency supplied by the surrounding translation unit: u32.

pub const ADI_AXI_REG_VERSION: u32 = 0x0000;
pub const ADI_AXI_REG_FPGA_INFO: u32 = 0x001C;

#[inline]
pub const fn ADI_AXI_PCORE_VER(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 16) | (minor << 8) | patch
}

#[inline]
pub const fn ADI_AXI_PCORE_VER_MAJOR(version: u32) -> u32 {
    (version >> 16) & 0xff
}

#[inline]
pub const fn ADI_AXI_PCORE_VER_MINOR(version: u32) -> u32 {
    (version >> 8) & 0xff
}

#[inline]
pub const fn ADI_AXI_PCORE_VER_PATCH(version: u32) -> u32 {
    version & 0xff
}

/**
 * adi_axi_pcore_ver_gteq() - check if a version is satisfied
 * @version: the full version read from the hardware
 * @major: the major version to compare against
 * @minor: the minor version to compare against
 *
 * ADI AXI IP Cores use semantic versioning, so this can be used to check for
 * feature availability.
 *
 * Return: true if the version is greater than or equal to the specified
 *         major and minor version, false otherwise.
 */
#[inline]
pub const fn adi_axi_pcore_ver_gteq(version: u32, major: u32, minor: u32) -> bool {
    ADI_AXI_PCORE_VER_MAJOR(version) > major
        || (ADI_AXI_PCORE_VER_MAJOR(version) == major
            && ADI_AXI_PCORE_VER_MINOR(version) >= minor)
}

#[inline]
pub const fn ADI_AXI_INFO_FPGA_TECH(info: u32) -> u32 {
    (info >> 24) & 0xff
}

#[inline]
pub const fn ADI_AXI_INFO_FPGA_FAMILY(info: u32) -> u32 {
    (info >> 16) & 0xff
}

#[inline]
pub const fn ADI_AXI_INFO_FPGA_SPEED_GRADE(info: u32) -> u32 {
    (info >> 8) & 0xff
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adi_axi_fpga_technology {
    ADI_AXI_FPGA_TECH_UNKNOWN = 0,
    ADI_AXI_FPGA_TECH_SERIES7,
    ADI_AXI_FPGA_TECH_ULTRASCALE,
    ADI_AXI_FPGA_TECH_ULTRASCALE_PLUS,
    ADI_AXI_FPGA_TECH_VERSAL,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adi_axi_fpga_family {
    ADI_AXI_FPGA_FAMILY_UNKNOWN = 0,
    ADI_AXI_FPGA_FAMILY_ARTIX,
    ADI_AXI_FPGA_FAMILY_KINTEX,
    ADI_AXI_FPGA_FAMILY_VIRTEX,
    ADI_AXI_FPGA_FAMILY_ZYNQ,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adi_axi_fpga_speed_grade {
    ADI_AXI_FPGA_SPEED_UNKNOWN = 0,
    ADI_AXI_FPGA_SPEED_1 = 10,
    ADI_AXI_FPGA_SPEED_1L = 11,
    ADI_AXI_FPGA_SPEED_1H = 12,
    ADI_AXI_FPGA_SPEED_1HV = 13,
    ADI_AXI_FPGA_SPEED_1LV = 14,
    ADI_AXI_FPGA_SPEED_2 = 20,
    ADI_AXI_FPGA_SPEED_2L = 21,
    ADI_AXI_FPGA_SPEED_2LV = 22,
    ADI_AXI_FPGA_SPEED_2MP = 23,
    ADI_AXI_FPGA_SPEED_3 = 30,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
