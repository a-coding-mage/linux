/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// Dependencies supplied by the corresponding OpenRISC architecture headers:
// asm/spr.h and asm/spr_defs.h

#[repr(C)]
pub struct cache_desc {
    pub size: u32,
    pub sets: u32,
    pub block_size: u32,
    pub ways: u32,
}

#[repr(C)]
pub struct cpuinfo_or1k {
    pub clock_frequency: u32,

    pub icache: cache_desc,
    pub dcache: cache_desc,

    pub coreid: u16,
}

extern "C" {
    pub static mut cpuinfo_or1k: [cpuinfo_or1k; NR_CPUS];
    pub fn setup_cpuinfo();

    /*
     * Check if the cache component exists.
     */
    pub fn cpu_cache_is_present(cache_type: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
