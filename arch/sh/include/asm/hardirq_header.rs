/* SPDX-License-Identifier: GPL-2.0 */

// External declaration corresponding to: extern void ack_bad_irq(unsigned int irq);
unsafe extern "C" {
    pub fn ack_bad_irq(irq: ::core::ffi::c_uint);
}

// C macro: #define ack_bad_irq ack_bad_irq
// Build-time architecture marker: ARCH_WANTS_NMI_IRQSTAT

// Dependency supplied by asm-generic/hardirq.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
