/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2012 ARM Limited
 */

// C header guard __SPC_H_ omitted in Rust.

// `__init` and `__iomem` are source-language annotations; the latter is
// represented by a raw pointer to opaque memory.
unsafe extern "C" {
    pub fn ve_spc_init(
        base: *mut core::ffi::c_void,
        a15_clusid: u32,
        irq: i32,
    ) -> i32;
    pub fn ve_spc_global_wakeup_irq(set: bool);
    pub fn ve_spc_cpu_wakeup_irq(cluster: u32, cpu: u32, set: bool);
    pub fn ve_spc_set_resume_addr(cluster: u32, cpu: u32, addr: u32);
    pub fn ve_spc_powerdown(cluster: u32, enable: bool);
    pub fn ve_spc_cpu_in_wfi(cpu: u32, cluster: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
