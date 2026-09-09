/* SPDX-License-Identifier: GPL-2.0 */

// `irq_err_count` is a volatile C global; accesses must use volatile
// operations when read or written from Rust.
unsafe extern "C" {
    pub static mut irq_err_count: core::ffi::c_ulong;
}

// C DECLARE_PER_CPU(unsigned long, irq_pmi_count);
// The storage is provided by the platform's per-CPU implementation.
unsafe extern "C" {
    pub static mut irq_pmi_count: core::ffi::c_ulong;
}

// CONFIG_ALPHA_GENERIC selects the platform-specific interrupt count.
#[cfg(CONFIG_ALPHA_GENERIC)]
macro_rules! ACTUAL_NR_IRQS {
    () => { alpha_mv.nr_irqs };
}

#[cfg(not(CONFIG_ALPHA_GENERIC))]
macro_rules! ACTUAL_NR_IRQS {
    () => { NR_IRQS };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
