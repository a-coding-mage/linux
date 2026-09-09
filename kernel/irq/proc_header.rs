/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C conditional:
 * #if defined(CONFIG_PROC_FS) && defined(CONFIG_GENERIC_IRQ_SHOW)
 * The corresponding Rust configuration is represented by features with the
 * same names.
 */

#[repr(C)]
pub struct irq_chip;

#[cfg(all(feature = "CONFIG_PROC_FS", feature = "CONFIG_GENERIC_IRQ_SHOW"))]
unsafe extern "C" {
    pub fn irq_proc_calc_prec();
    pub fn irq_proc_update_chip(chip: *const irq_chip);
}

#[cfg(not(all(feature = "CONFIG_PROC_FS", feature = "CONFIG_GENERIC_IRQ_SHOW")))]
#[inline]
pub fn irq_proc_calc_prec() {}

#[cfg(not(all(feature = "CONFIG_PROC_FS", feature = "CONFIG_GENERIC_IRQ_SHOW")))]
#[inline]
pub fn irq_proc_update_chip(_chip: *const irq_chip) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
