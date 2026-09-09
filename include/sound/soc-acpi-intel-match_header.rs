/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2017, Intel Corporation
 */

/*
 * These declarations correspond to the C header's external dependencies:
 * <linux/module.h>, <linux/stddef.h>, and <linux/acpi.h>.
 */

/*
 * These tables are not constants, some fields can be used for
 * pdata or machine ops.
 */
extern "C" {
    pub static mut snd_soc_acpi_intel_broadwell_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_baytrail_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_cherrytrail_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_skl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_kbl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_bxt_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_glk_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_cnl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_cfl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_cml_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_icl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_tgl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_ehl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_jsl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_adl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_rpl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_mtl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_lnl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_arl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_ptl_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_nvl_machines: [snd_soc_acpi_mach; 0];

    pub static mut snd_soc_acpi_intel_cnl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_cfl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_cml_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_icl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_tgl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_adl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_rpl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_mtl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_lnl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_arl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_ptl_sdw_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_intel_nvl_sdw_machines: [snd_soc_acpi_mach; 0];

    /* Generic table used for HDA codec-based platforms, possibly with
     * additional ACPI-enumerated codecs. */
    pub static mut snd_soc_acpi_intel_hda_machines: [snd_soc_acpi_mach; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
