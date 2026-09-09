/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the C header. Linux initialization and interrupt declarations
// are supplied by the surrounding translation unit/dependencies.

extern "C" {
    // C: void __init crime_init(void);
    pub fn crime_init();

    // C: irqreturn_t crime_memerr_intr(int irq, void *dev_id);
    pub fn crime_memerr_intr(irq: core::ffi::c_int, dev_id: *mut core::ffi::c_void) -> irqreturn_t;

    // C: irqreturn_t crime_cpuerr_intr(int irq, void *dev_id);
    pub fn crime_cpuerr_intr(irq: core::ffi::c_int, dev_id: *mut core::ffi::c_void) -> irqreturn_t;

    // C: void __init ip32_be_init(void);
    pub fn ip32_be_init();

    // C: void ip32_prepare_poweroff(void);
    pub fn ip32_prepare_poweroff();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
