/* SPDX-License-Identifier: GPL-2.0 */
/* cpudata.h: Per-cpu parameters.
 *
 * Copyright (C) 2004 Keith M Wesolowski (wesolows@foobazco.org)
 *
 * Based on include/asm/cpudata.h and Linux 2.4 smp.h
 * both (C) David S. Miller.
 */

/* Translated from the C header; the Linux percpu dependency is supplied externally. */

#[repr(C)]
pub struct CpuinfoSparc {
    pub udelay_val: ::core::ffi::c_ulong,
    pub clock_tick: ::core::ffi::c_ulong,
    pub counter: ::core::ffi::c_uint,
    #[cfg(CONFIG_SMP)]
    pub irq_resched_count: ::core::ffi::c_uint,
    #[cfg(CONFIG_SMP)]
    pub irq_call_count: ::core::ffi::c_uint,
    pub prom_node: ::core::ffi::c_int,
    pub mid: ::core::ffi::c_int,
    pub next: ::core::ffi::c_int,
}

pub type cpuinfo_sparc = CpuinfoSparc;

extern "C" {
    pub static mut __cpu_data: CpuinfoSparc;
}

/* C macro: cpu_data(__cpu) expands to per_cpu(__cpu_data, (__cpu)). */
#[macro_export]
macro_rules! cpu_data {
    ($cpu:expr) => {
        per_cpu!(__cpu_data, ($cpu))
    };
}

/* C macro: local_cpu_data() expands to (*this_cpu_ptr(&__cpu_data)). */
#[macro_export]
macro_rules! local_cpu_data {
    () => {
        *this_cpu_ptr(&raw mut $crate::__cpu_data)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
