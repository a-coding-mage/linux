/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
 *
 * Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
 */

/* C dependency: #include <sound/soc-acpi.h> */

pub const FLAG_AMD_SOF: u32 = 1u32 << 1;
pub const FLAG_AMD_SOF_ONLY_DMIC: u32 = 1u32 << 2;
pub const FLAG_AMD_LEGACY: u32 = 1u32 << 3;
pub const FLAG_AMD_LEGACY_ONLY_DMIC: u32 = 1u32 << 4;

pub const ACP_PCI_DEV_ID: u32 = 0x15E2;

unsafe extern "C" {
    pub static mut snd_soc_acpi_amd_sof_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_rmb_sof_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_vangogh_sof_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp63_sof_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp63_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp63_sof_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp70_sof_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp70_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp70_sof_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp7x_sof_machines: [snd_soc_acpi_mach; 0];
}

#[repr(C)]
pub struct config_entry {
    pub flags: u32,
    pub device: u16,
    pub dmi_table: *const dmi_system_id,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
