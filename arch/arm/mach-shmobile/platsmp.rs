// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for R-Mobile / SH-Mobile
 *
 * Copyright (C) 2010  Magnus Damm
 * Copyright (C) 2011  Paul Mundt
 *
 * Based on vexpress, Copyright (C) 2002 ARM Ltd, All Rights Reserved
 */

// External declarations supplied by the Linux kernel and related sources.
unsafe extern "C" {
    static mut shmobile_smp_fn: [usize; 0];
    static mut shmobile_smp_arg: [usize; 0];
    static mut shmobile_smp_mpidr: [usize; 0];

    fn flush_cache_all();
    fn cpu_logical_map(cpu: u32) -> usize;
}

pub unsafe fn shmobile_smp_hook(cpu: u32, fn_: usize, arg: usize) {
    *shmobile_smp_fn.as_mut_ptr().add(cpu as usize) = 0;
    flush_cache_all();

    *shmobile_smp_mpidr.as_mut_ptr().add(cpu as usize) = cpu_logical_map(cpu);
    *shmobile_smp_fn.as_mut_ptr().add(cpu as usize) = fn_;
    *shmobile_smp_arg.as_mut_ptr().add(cpu as usize) = arg;
    flush_cache_all();
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub fn shmobile_smp_cpu_can_disable(_cpu: u32) -> bool {
    true /* Hotplug of any CPU is supported */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
