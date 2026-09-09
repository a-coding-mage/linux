/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/cpu.h
 *
 *  Copyright (C) 2004-2005 ARM Ltd.
 */

/*
 * Dependencies supplied by the surrounding kernel translation:
 * linux/percpu.h, linux/cpu.h
 */

#[repr(C)]
pub struct cpuinfo_arm {
    pub cpuid: u32,
    #[cfg(feature = "CONFIG_SMP")]
    pub loops_per_jiffy: ::core::ffi::c_uint,
}

/* DECLARE_PER_CPU(struct cpuinfo_arm, cpu_data); */
extern "C" {
    pub static mut cpu_data: cpuinfo_arm;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
