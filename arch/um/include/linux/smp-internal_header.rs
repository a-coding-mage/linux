/* SPDX-License-Identifier: GPL-2.0 */

// #if IS_ENABLED(CONFIG_SMP)

unsafe extern "C" {
    pub fn prefill_possible_map();
}

// #else /* !CONFIG_SMP */

// static inline void prefill_possible_map(void) { }
#[inline]
pub unsafe fn prefill_possible_map_disabled() {}

// #endif /* CONFIG_SMP */

// extern char cpu_irqstacks[NR_CPUS][THREAD_SIZE] __aligned(THREAD_SIZE);
unsafe extern "C" {
    pub static mut cpu_irqstacks: [[core::ffi::c_char; THREAD_SIZE]; NR_CPUS];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
