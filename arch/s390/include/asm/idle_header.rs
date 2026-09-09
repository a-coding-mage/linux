/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright IBM Corp. 2014
 *
 *  Author: Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

/* Dependencies: linux types, per-CPU definitions, and asm/tod_types. */

#[repr(C)]
pub struct s390_idle_data {
    /* Preserved from CONFIG_NO_HZ_COMMON; enable this field in that build configuration. */
    #[cfg(CONFIG_NO_HZ_COMMON)]
    pub in_idle: bool,
    pub timer_idle_enter: usize,
    pub mt_cycles_enter: [usize; 8],
    pub clock_idle_enter: tod_clock,
    pub clock_idle_exit: tod_clock,
}

/* DECLARE_PER_CPU(struct s390_idle_data, s390_idle); */
extern "C" {
    pub static mut s390_idle: s390_idle_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
