/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2013 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

pub const CPS_ENTRY_PATCH_INSNS: usize = 6;

#[repr(C)]
pub struct vpe_boot_config {
    pub pc: ::core::ffi::c_ulong,
    pub sp: ::core::ffi::c_ulong,
    pub gp: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct core_boot_config {
    pub vpe_mask: atomic_t,
    pub vpe_config: *mut vpe_boot_config,
}

#[repr(C)]
pub struct cluster_boot_config {
    pub core_power: *mut ::core::ffi::c_ulong,
    pub cpumask: cpumask,
    pub core_config: *mut core_boot_config,
}

unsafe extern "C" {
    pub static mut mips_cps_cluster_bootcfg: *mut cluster_boot_config;

    pub fn mips_cps_core_boot(cca: ::core::ffi::c_int, gcr_base: *mut ::core::ffi::c_void);
    pub fn mips_cps_core_init();

    pub fn mips_cps_boot_vpes(cfg: *mut core_boot_config, vpe: ::core::ffi::c_uint);

    pub fn mips_cps_pm_save();
    pub fn mips_cps_pm_restore();

    pub fn excep_tlbfill();
    pub fn excep_xtlbfill();
    pub fn excep_cache();
    pub fn excep_genex();
    pub fn excep_intex();
    pub fn excep_ejtag();
}

/* CONFIG_MIPS_CPS is a build-time C configuration condition. */
#[cfg(CONFIG_MIPS_CPS)]
unsafe extern "C" {
    pub fn mips_cps_smp_in_use() -> bool;
}

/* !CONFIG_MIPS_CPS */
#[cfg(not(CONFIG_MIPS_CPS))]
#[inline]
pub fn mips_cps_smp_in_use() -> bool {
    false
}

/* The __ASSEMBLER__ branch declares .extern mips_cps_bootcfg and has no Rust item. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
