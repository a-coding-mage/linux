/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Power Management Service Unit (PMSU) support for Armada 370/XP platforms.
 *
 * Copyright (C) 2012 Marvell
 */

// Declarations translated from the C header. `phys_addr_t` is supplied by an
// external dependency, corresponding to the C type of the same name.

unsafe extern "C" {
    pub fn armada_xp_boot_cpu(cpu_id: u32, phys_addr: *mut core::ffi::c_void) -> i32;

    pub fn mvebu_setup_boot_addr_wa(
        crypto_eng_target: u32,
        crypto_eng_attribute: u32,
        resume_addr_reg: phys_addr_t,
    ) -> i32;

    pub fn mvebu_v7_pmsu_idle_exit();
    pub fn armada_370_xp_cpu_resume();

    pub fn armada_370_xp_pmsu_idle_enter(deepidle: usize) -> i32;
    pub fn armada_38x_do_cpu_suspend(deepidle: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
